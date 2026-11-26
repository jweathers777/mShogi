//! Bounded command, reply and retirement evidence for UCI, USI and UCCI.
use crate::{
    common::{elapsed_ns, observed_ns},
    process::{Engine, Output, WriteFailure, escaped, valid_protocol_byte},
    protocol::{Failure, Traffic, option_value},
    *,
};
use std::{
    sync::mpsc::RecvTimeoutError,
    time::{Duration, Instant},
};

pub(crate) fn handshake_header(line: &str) -> bool {
    [
        "id ",
        "option ",
        "Pikafish ",
        "Fairy-Stockfish ",
        "Stockfish ",
    ]
    .iter()
    .any(|lead| line.starts_with(lead))
}
pub(crate) const HANDSHAKE_LINES: usize = 256;
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Dialect {
    Uci,
    Usi,
    Ucci,
}
impl Dialect {
    pub fn tokens(self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::Uci => ("uci", "uciok", "ucinewgame"),
            Self::Usi => ("usi", "usiok", "usinewgame"),
            Self::Ucci => ("ucci", "ucciok", "setoption newgame"),
        }
    }
}

pub(crate) fn supports_option<'a>(
    lines: impl IntoIterator<Item = &'a str>,
    dialect: Dialect,
    option: &UciOption,
) -> bool {
    let prefix = if dialect == Dialect::Ucci {
        format!("option {} type ", option.name)
    } else {
        format!("option name {} type ", option.name)
    };
    lines
        .into_iter()
        .find(|line| line.starts_with(&prefix))
        .is_some_and(|line| option_value(line, &option.value))
}

pub(crate) trait Journal {
    type Entry: serde::Serialize;
    fn entry(&mut self, event: Traffic) -> Self::Entry;
    fn retain(&mut self, entry: Self::Entry);
    fn dropped(&mut self, _bytes: usize) {}
}
impl Journal for Vec<Traffic> {
    type Entry = Traffic;
    fn entry(&mut self, event: Traffic) -> Traffic {
        event
    }
    fn retain(&mut self, entry: Traffic) {
        self.push(entry);
    }
}
impl Journal for Transcript {
    type Entry = TranscriptEntry;
    fn entry(&mut self, event: Traffic) -> TranscriptEntry {
        self.observed_entries += 1;
        TranscriptEntry {
            sequence: self.observed_entries,
            engine: event.engine,
            at_ns: event.at_ns,
            direction: if event.written.is_some() {
                TransportDirection::Command
            } else {
                TransportDirection::Response
            },
            game_version: event.version as u64,
            phase: match event.phase.as_str() {
                "newgame" => "new_game".into(),
                "newgame_ready" => "readiness".into(),
                _ => event.phase,
            },
            text: event.text,
            truncated: false,
            write_completed: event.written,
        }
    }
    fn retain(&mut self, entry: TranscriptEntry) {
        self.entries.push(entry);
    }
    fn dropped(&mut self, bytes: usize) {
        self.truncated = true;
        self.dropped_bytes = self.dropped_bytes.saturating_add(bytes as u64);
    }
}

pub(crate) struct Transport<J: Journal> {
    pub journal: J,
    pub complete: bool,
    pub start: Instant,
    pub generation: u64,
    pub version: usize,
    pub observed: u64,
    pub phase_budget: (u64, u64),
    pub watchdogs: Watchdogs,
    io: IoLimits,
    bytes_left: usize,
    deadline: Option<Instant>,
}
impl<J: Journal> Transport<J> {
    pub fn new(watchdogs: Watchdogs, io: IoLimits, game_ms: Option<u64>, journal: J) -> Self {
        let start = Instant::now();
        Self {
            journal,
            complete: true,
            start,
            generation: 1,
            version: 0,
            observed: 0,
            phase_budget: (0, 0),
            watchdogs,
            io,
            bytes_left: io.transcript_bytes,
            deadline: game_ms.map(|ms| start + Duration::from_millis(ms)),
        }
    }
    pub fn failure(
        &self,
        engine: usize,
        kind: FailureKind,
        phase: &str,
        detail: impl Into<String>,
    ) -> Failure {
        Failure {
            kind,
            engine,
            generation: self.generation,
            phase: phase.into(),
            detail: detail.into().chars().take(1024).collect(),
        }
    }
    fn log(
        &mut self,
        engine: usize,
        phase: &str,
        text: String,
        written: Option<bool>,
        at_ns: u64,
    ) -> Result<(), Failure> {
        let length = text.len();
        let entry = self.journal.entry(Traffic {
            engine,
            generation: self.generation,
            version: self.version,
            at_ns,
            direction: if written.is_some() {
                "command"
            } else {
                "response"
            }
            .into(),
            phase: phase.into(),
            text,
            written,
        });
        let cost = serde_json::to_vec(&entry).expect("finite event").len() + 1;
        if cost > self.bytes_left {
            self.bytes_left = 0;
            self.complete = false;
            self.journal.dropped(length);
            return Err(self.failure(
                engine,
                FailureKind::ResourceLimit,
                phase,
                "transcript limit reached",
            ));
        }
        self.bytes_left -= cost;
        self.journal.retain(entry);
        Ok(())
    }
    fn overall_timeout(&self, engine: usize) -> Failure {
        self.failure(
            engine,
            FailureKind::Watchdog,
            "experiment_limit",
            "overall game watchdog expired",
        )
    }
    pub fn write_at(
        &mut self,
        engine: &mut Engine,
        phase: &str,
        line: &str,
        timeout: Duration,
        at: u64,
    ) -> (bool, Result<(), Failure>) {
        self.observed = elapsed_ns(self.start);
        self.phase_budget = (at, timeout.as_millis() as u64);
        if line.len() + 1 > self.io.command_bytes {
            return (
                false,
                Err(self.failure(
                    engine.index,
                    FailureKind::ResourceLimit,
                    phase,
                    "command exceeds byte cap",
                )),
            );
        }
        let deadline = self.deadline.filter(|_| phase != "cleanup");
        let timeout = deadline.map_or(timeout, |d| {
            timeout.min(d.saturating_duration_since(Instant::now()))
        });
        if timeout.is_zero() && deadline.is_some() {
            return (false, Err(self.overall_timeout(engine.index)));
        }
        let result = engine.write(format!("{line}\n").into_bytes(), timeout);
        self.observed = elapsed_ns(self.start);
        let retained = self.log(engine.index, phase, line.into(), Some(result.is_ok()), at);
        if deadline.is_some_and(|d| Instant::now() >= d) {
            return (result.is_ok(), Err(self.overall_timeout(engine.index)));
        }
        if let Err(e) = result {
            return (false, Err(self.write_failure(engine, phase, e)));
        }
        (true, retained)
    }
    pub fn write_failure(&self, engine: &mut Engine, phase: &str, error: WriteFailure) -> Failure {
        let timed_out = error.timed_out();
        if !timed_out && phase != "cleanup" {
            engine.settle_pipe();
        } else {
            let _ = engine.observe_exit();
        }
        let kind = if engine.exit.as_ref().is_some_and(|s| !s.success) {
            FailureKind::UnexpectedExit
        } else if timed_out {
            FailureKind::Watchdog
        } else {
            FailureKind::MissingReply
        };
        self.failure(engine.index, kind, phase, error)
    }
    pub fn send(&mut self, engine: &mut Engine, phase: &str, text: &str) -> Result<(), Failure> {
        let (_, result) = self.write_at(
            engine,
            phase,
            text,
            Duration::from_millis(self.watchdogs.write_ms),
            elapsed_ns(self.start),
        );
        result
    }
    pub fn line(
        &mut self,
        engine: &mut Engine,
        phase: &str,
        deadline: Instant,
        mut peer: Option<&mut Engine>,
    ) -> Result<Option<String>, Failure> {
        let overall_first = self.deadline.is_some_and(|d| d < deadline);
        let deadline = self.deadline.map_or(deadline, |d| d.min(deadline));
        loop {
            let now = Instant::now();
            self.observed = observed_ns(self.start, now);
            if overall_first && now >= deadline {
                return Err(self.overall_timeout(engine.index));
            }
            if now >= deadline {
                return Ok(None);
            }
            if let Some(peer) = peer.as_deref_mut() {
                match peer.observe_exit() {
                    Ok(None) => (),
                    Ok(Some(_)) => {
                        return Err(self.failure(
                            peer.index,
                            FailureKind::UnexpectedExit,
                            phase,
                            "idle engine exited",
                        ));
                    }
                    Err(e) => {
                        return Err(self.failure(
                            peer.index,
                            FailureKind::UnexpectedExit,
                            phase,
                            e,
                        ));
                    }
                }
            }
            let output = engine.receive(
                deadline
                    .saturating_duration_since(Instant::now())
                    .min(Duration::from_millis(2)),
            );
            let received = Instant::now();
            self.observed = observed_ns(self.start, received);
            let retained = match &output {
                Ok(Output::Line(bytes)) => {
                    let text = escaped(bytes);
                    engine.last_output = Some(text.clone());
                    let phase = if received >= deadline {
                        "retired"
                    } else {
                        phase
                    };
                    self.log(engine.index, phase, text, None, self.observed)
                }
                Ok(Output::Oversized) => {
                    self.complete = false;
                    Ok(())
                }
                _ => Ok(()),
            };
            if overall_first && received >= deadline {
                return Err(self.overall_timeout(engine.index));
            }
            if received >= deadline {
                return Ok(None);
            }
            retained?;
            match output {
                Ok(Output::Line(bytes)) => {
                    if bytes.iter().any(|&b| !valid_protocol_byte(b)) {
                        return Err(self.failure(
                            engine.index,
                            FailureKind::MalformedReply,
                            phase,
                            "non-ASCII/control output",
                        ));
                    }
                    let line = String::from_utf8(bytes).expect("checked ASCII");
                    if !line.trim().is_empty() {
                        return Ok(Some(line));
                    }
                }
                Ok(Output::Oversized) => {
                    self.complete = false;
                    return Err(self.failure(
                        engine.index,
                        FailureKind::ResourceLimit,
                        phase,
                        "oversized protocol line",
                    ));
                }
                Ok(Output::Eof) | Err(RecvTimeoutError::Disconnected) => {
                    let kind = engine.settle_pipe();
                    return Err(self.failure(
                        engine.index,
                        kind,
                        phase,
                        "EOF before required reply",
                    ));
                }
                Ok(Output::Error(e)) => {
                    return Err(self.failure(engine.index, FailureKind::UnexpectedExit, phase, e));
                }
                Err(RecvTimeoutError::Timeout) => (),
            }
        }
    }
    fn expect(
        &mut self,
        engine: &mut Engine,
        phase: &str,
        token: &str,
        millis: u64,
        mut peer: Option<&mut Engine>,
    ) -> Result<Vec<String>, Failure> {
        self.phase_budget = (elapsed_ns(self.start), millis);
        let deadline =
            self.start + Duration::from_nanos(self.phase_budget.0) + Duration::from_millis(millis);
        let mut lines = Vec::new();
        let mut count = 0;
        loop {
            let line = self
                .line(engine, phase, deadline, peer.as_deref_mut())?
                .ok_or_else(|| {
                    self.failure(
                        engine.index,
                        FailureKind::Watchdog,
                        phase,
                        format!("no {token}"),
                    )
                })?;
            let line = line.trim();
            if line == token {
                return Ok(lines);
            }
            count += 1;
            if count > HANDSHAKE_LINES {
                return Err(self.failure(
                    engine.index,
                    FailureKind::ResourceLimit,
                    phase,
                    "too many protocol lines",
                ));
            }
            if line.starts_with("bestmove") {
                return Err(self.failure(
                    engine.index,
                    FailureKind::LateReply,
                    phase,
                    "unsolicited bestmove",
                ));
            }
            if phase == "handshake" && handshake_header(line) {
                lines.push(line.to_owned());
            } else if !line.starts_with("info ") {
                return Err(self.failure(
                    engine.index,
                    FailureKind::MalformedReply,
                    phase,
                    "unexpected reply",
                ));
            }
        }
    }
    pub fn prepare(
        &mut self,
        engine: &mut Engine,
        dialect: Dialect,
        options: &[UciOption],
        claim_policy: Option<&str>,
        mut peer: Option<&mut Engine>,
    ) -> Result<(), Failure> {
        let (hello, ok, newgame) = dialect.tokens();
        self.send(engine, "handshake", hello)?;
        let lines = self.expect(
            engine,
            "handshake",
            ok,
            self.watchdogs.handshake_ms,
            peer.as_deref_mut(),
        )?;
        let mut bytes = 0;
        for id in lines.iter().filter(|line| line.starts_with("id ")) {
            let keep = id.len().min(4096 - bytes);
            if engine.identity.len() >= 32 || keep == 0 {
                engine.identity_truncated = true;
                break;
            }
            engine.identity.push(id[..keep].into());
            bytes += keep;
            engine.identity_truncated |= keep != id.len();
        }
        let claim = claim_policy
            .filter(|_| {
                engine
                    .identity
                    .iter()
                    .any(|s| s.starts_with("id name mShogi "))
            })
            .map(|value| UciOption {
                name: "ClaimPolicy".into(),
                value: value.into(),
            });
        for option in options.iter().chain(claim.iter()) {
            if !supports_option(lines.iter().map(String::as_str), dialect, option) {
                return Err(self.failure(
                    engine.index,
                    FailureKind::Configuration,
                    "options",
                    format!("option or value not advertised: {}", option.name),
                ));
            }
            let command = if dialect == Dialect::Ucci {
                format!("setoption {} {}", option.name, option.value)
            } else {
                format!("setoption name {} value {}", option.name, option.value)
            };
            self.send(engine, "options", &command)?;
        }
        self.send(engine, "readiness", "isready")?;
        self.expect(
            engine,
            "readiness",
            "readyok",
            self.watchdogs.ready_ms,
            peer.as_deref_mut(),
        )?;
        self.send(engine, "newgame", newgame)?;
        if dialect != Dialect::Usi {
            self.send(engine, "newgame_ready", "isready")?;
            self.expect(
                engine,
                "newgame_ready",
                "readyok",
                self.watchdogs.ready_ms,
                peer,
            )?;
        }
        Ok(())
    }
    pub fn retire(&mut self, engine: &mut Engine) -> (EngineObservation, Option<Failure>) {
        let budget = Duration::from_millis(self.watchdogs.cleanup_ms);
        let deadline = Instant::now() + budget;
        // Reserve half the watchdog for forced termination and the pipe joins.
        // Small receive intervals also limit oversleep on coalesced OS timers.
        let drain = deadline - budget / 2;
        let pending = engine.unsettled_pipe;
        if pending {
            let grace = (budget / 4).min(Duration::from_millis(100));
            engine.pre_cleanup_status = engine.observe_exit_for(grace);
        }
        let mut failure = None;
        let mut quit_written = false;
        for command in ["stop", "quit"] {
            let left = drain.saturating_duration_since(Instant::now());
            if left.is_zero() {
                break;
            }
            let (completed, result) = self.write_at(
                engine,
                "cleanup",
                command,
                left.min(budget / 4)
                    .min(Duration::from_millis(self.watchdogs.write_ms)),
                elapsed_ns(self.start),
            );
            if command == "quit" {
                quit_written = completed;
            }
            if let Err(e) = result {
                failure.get_or_insert(e);
            }
        }
        let mut reached_eof = false;
        loop {
            let left = drain.saturating_duration_since(Instant::now());
            if left.is_zero() {
                self.complete = false;
                failure.get_or_insert_with(|| {
                    self.failure(
                        engine.index,
                        FailureKind::ResourceLimit,
                        "retired",
                        "retirement capture did not establish EOF within deadline",
                    )
                });
                break;
            }
            // When quit failed, a live child can keep stdout open indefinitely.
            // Keep queued evidence, then spend the remaining budget on reaping.
            let wait = if quit_written {
                left.min(Duration::from_millis(5))
            } else {
                Duration::ZERO
            };
            match engine.receive(wait) {
                Ok(Output::Line(bytes)) => {
                    if let Err(e) = self.log(
                        engine.index,
                        "retired",
                        escaped(&bytes),
                        None,
                        elapsed_ns(self.start),
                    ) {
                        failure.get_or_insert(e);
                    }
                    if bytes.iter().any(|&b| !valid_protocol_byte(b)) {
                        failure.get_or_insert_with(|| {
                            self.failure(
                                engine.index,
                                FailureKind::MalformedReply,
                                "retired",
                                "non-ASCII/control output",
                            )
                        });
                    }
                    if bytes.trim_ascii_start().starts_with(b"bestmove") {
                        failure.get_or_insert_with(|| {
                            self.failure(
                                engine.index,
                                FailureKind::LateReply,
                                "retired",
                                "reply after generation retired",
                            )
                        });
                    }
                }
                Ok(Output::Eof) | Err(RecvTimeoutError::Disconnected) => {
                    reached_eof = true;
                    break;
                }
                Ok(Output::Oversized) => {
                    self.complete = false;
                    failure.get_or_insert_with(|| {
                        self.failure(
                            engine.index,
                            FailureKind::ResourceLimit,
                            "retired",
                            "oversized retired output",
                        )
                    });
                }
                Ok(Output::Error(e)) => {
                    failure.get_or_insert_with(|| {
                        self.failure(engine.index, FailureKind::UnexpectedExit, "retired", e)
                    });
                }
                Err(RecvTimeoutError::Timeout) if !quit_written => {
                    self.complete = false;
                    break;
                }
                Err(RecvTimeoutError::Timeout) => (),
            }
        }
        let mut observation =
            engine.cleanup_until(deadline, !pending && quit_written && reached_eof);
        if !observation.cleanup_failures.is_empty()
            || observation.exit.as_ref().is_none_or(|e| !e.success)
        {
            failure.get_or_insert_with(|| {
                self.failure(
                    engine.index,
                    FailureKind::UnexpectedExit,
                    "cleanup",
                    "child failed bounded cleanup",
                )
            });
        }
        observation.retirement_failure = failure.clone();
        (observation, failure)
    }
}
