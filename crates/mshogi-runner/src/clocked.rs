//! Shared persistent-process, controller-clocked protocol host for external
//! game tests (USI Shogi, UCI Xiangqi). Game rules, admission and reports stay
//! in each game's module; this module only moves protocol lines and measures
//! the reply time against the mover's clock.
use crate::{
    process::{Engine, Output, escaped},
    protocol::option_value,
    protocol::{Failure, Traffic},
    *,
};
use std::{
    sync::mpsc::RecvTimeoutError,
    time::{Duration, Instant},
};

/// Protocol words of one dialect.
#[derive(Clone, Copy)]
pub(crate) struct Dialect {
    /// Handshake command, for example `usi`.
    pub hello: &'static str,
    /// Handshake completion, for example `usiok`.
    pub ok: &'static str,
    /// New-game command, for example `usinewgame`.
    pub newgame: &'static str,
    /// UCCI option syntax (`option NAME type ...`, `setoption NAME VALUE`)
    /// instead of the UCI/USI `name`/`value` keywords.
    pub ucci_options: bool,
}

pub(crate) struct Host<'a> {
    /// Dialect of engine 0 and engine 1.
    pub dialects: [Dialect; 2],
    pub engines: &'a [EngineConfig; 2],
    pub watchdogs: Watchdogs,
    pub io: IoLimits,
    pub start: Instant,
    pub version: usize,
    pub traffic: Vec<Traffic>,
    pub bytes: usize,
    pub complete: bool,
}

impl Host<'_> {
    pub(crate) fn failure(
        &self,
        engine: usize,
        kind: FailureKind,
        phase: &str,
        detail: impl Into<String>,
    ) -> Failure {
        Failure {
            kind,
            engine,
            generation: 1,
            phase: phase.into(),
            detail: detail.into().chars().take(1024).collect(),
        }
    }
    pub(crate) fn log(
        &mut self,
        engine: usize,
        direction: &str,
        phase: &str,
        text: String,
        written: Option<bool>,
    ) {
        let event = Traffic {
            engine,
            generation: 1,
            version: self.version,
            at_ns: crate::common::elapsed_ns(self.start),
            direction: direction.into(),
            phase: phase.into(),
            text,
            written,
        };
        let size = serde_json::to_vec(&event).map_or(usize::MAX, |b| b.len());
        if self.bytes.saturating_add(size) > self.io.transcript_bytes {
            self.complete = false;
            return;
        }
        self.bytes += size;
        self.traffic.push(event);
    }
    pub(crate) fn send(
        &mut self,
        engine: &mut Engine,
        phase: &str,
        text: &str,
    ) -> Result<(), Failure> {
        if text.len() + 1 > self.io.command_bytes {
            return Err(self.failure(
                engine.index,
                FailureKind::ResourceLimit,
                phase,
                "command exceeds limit",
            ));
        }
        let result = engine.write(
            format!("{text}\n").into_bytes(),
            Duration::from_millis(self.watchdogs.write_ms),
        );
        self.log(
            engine.index,
            "command",
            phase,
            text.into(),
            Some(result.is_ok()),
        );
        result.map_err(|e| {
            self.failure(
                engine.index,
                FailureKind::UnexpectedExit,
                phase,
                String::from(e),
            )
        })
    }
    /// Next non-empty line before the deadline; `None` at the deadline.
    pub(crate) fn line(
        &mut self,
        engine: &mut Engine,
        phase: &str,
        deadline: Instant,
    ) -> Result<Option<String>, Failure> {
        loop {
            let now = Instant::now();
            if now >= deadline {
                return Ok(None);
            }
            match engine.receive((deadline - now).min(Duration::from_millis(5))) {
                Ok(Output::Line(bytes)) => {
                    if bytes.is_empty() {
                        continue;
                    }
                    if !bytes.is_ascii()
                        || bytes
                            .iter()
                            .any(|b| b.is_ascii_control() && !b.is_ascii_whitespace())
                    {
                        self.log(engine.index, "response", phase, escaped(&bytes), None);
                        return Err(self.failure(
                            engine.index,
                            FailureKind::MalformedReply,
                            phase,
                            "non-ASCII/control output",
                        ));
                    }
                    let text = String::from_utf8(bytes).unwrap();
                    if !text.starts_with("info") {
                        self.log(
                            engine.index,
                            "response",
                            phase,
                            escaped(text.as_bytes()),
                            None,
                        );
                    }
                    return Ok(Some(text));
                }
                Ok(Output::Oversized) => {
                    return Err(self.failure(
                        engine.index,
                        FailureKind::ResourceLimit,
                        phase,
                        "oversized protocol line",
                    ));
                }
                Ok(Output::Eof) => {
                    return Err(self.failure(
                        engine.index,
                        FailureKind::UnexpectedExit,
                        phase,
                        "EOF before required reply",
                    ));
                }
                Ok(Output::Error(e)) => {
                    return Err(self.failure(engine.index, FailureKind::UnexpectedExit, phase, e));
                }
                Err(RecvTimeoutError::Disconnected) => {
                    return Err(self.failure(
                        engine.index,
                        FailureKind::MissingReply,
                        phase,
                        "stdout reader closed",
                    ));
                }
                Err(RecvTimeoutError::Timeout) => {}
            }
        }
    }
    pub(crate) fn expect(
        &mut self,
        engine: &mut Engine,
        phase: &str,
        token: &str,
        millis: u64,
    ) -> Result<Vec<String>, Failure> {
        let deadline = Instant::now() + Duration::from_millis(millis);
        let mut lines = Vec::new();
        loop {
            let Some(line) = self.line(engine, phase, deadline)? else {
                return Err(self.failure(
                    engine.index,
                    FailureKind::Watchdog,
                    phase,
                    format!("no {token}"),
                ));
            };
            if line == token {
                return Ok(lines);
            }
            if lines.len() >= 512 {
                return Err(self.failure(
                    engine.index,
                    FailureKind::ResourceLimit,
                    phase,
                    "too many lines",
                ));
            }
            lines.push(line);
        }
    }
    pub(crate) fn prepare(&mut self, engine: &mut Engine) -> Result<Vec<String>, Failure> {
        let index = engine.index;
        let dialect = self.dialects[index];
        self.send(engine, "handshake", dialect.hello)?;
        let lines = self.expect(engine, "handshake", dialect.ok, self.watchdogs.handshake_ms)?;
        for option in &self.engines[index].options {
            let prefix = if dialect.ucci_options {
                format!("option {} type ", option.name)
            } else {
                format!("option name {} type ", option.name)
            };
            let advertised = lines.iter().find(|l| l.starts_with(&prefix)).cloned();
            match advertised {
                Some(line) if option_value(&line, &option.value) => {}
                Some(_) => {
                    return Err(self.failure(
                        index,
                        FailureKind::Configuration,
                        "options",
                        format!("value not supported: {}", option.name),
                    ));
                }
                None => {
                    return Err(self.failure(
                        index,
                        FailureKind::Configuration,
                        "options",
                        format!("option not advertised: {}", option.name),
                    ));
                }
            }
            self.send(
                engine,
                "options",
                &if dialect.ucci_options {
                    format!("setoption {} {}", option.name, option.value)
                } else {
                    format!("setoption name {} value {}", option.name, option.value)
                },
            )?;
        }
        self.send(engine, "readiness", "isready")?;
        self.expect(engine, "readiness", "readyok", self.watchdogs.ready_ms)?;
        self.send(engine, "newgame", dialect.newgame)?;
        Ok(lines.into_iter().filter(|l| l.starts_with("id ")).collect())
    }
}

pub(crate) enum Outcome {
    Accepted(String, u64, u64),
    Exhausted(u64),
    /// The overall game watchdog, not the mover's clock, ended the wait.
    Watchdog,
}

impl Host<'_> {
    /// Send the position and clock commands, then wait for `bestmove` until
    /// the mover's remaining time plus grace or the overall deadline.
    pub(crate) fn search(
        &mut self,
        engine: &mut Engine,
        position: &str,
        go: &str,
        remaining_ms: u64,
        grace_ms: u64,
        deadline: Instant,
    ) -> Result<Outcome, Failure> {
        let index = engine.index;
        self.send(engine, "position", position)?;
        self.send(engine, "search", go)?;
        let started = Instant::now();
        let limit = remaining_ms.saturating_add(grace_ms);
        let clock_deadline = started + Duration::from_millis(limit);
        let reply_deadline = clock_deadline.min(deadline);
        let watchdog_capped = deadline < clock_deadline;
        let mut info = 0u64;
        loop {
            match self.line(engine, "search", reply_deadline)? {
                None => {
                    if watchdog_capped {
                        return Ok(Outcome::Watchdog);
                    }
                    let elapsed = started.elapsed().as_millis() as u64;
                    return Ok(Outcome::Exhausted(elapsed.max(limit + 1)));
                }
                Some(line) if line.starts_with("bestmove") => {
                    let elapsed = started.elapsed().as_micros().div_ceil(1000) as u64;
                    if elapsed > limit {
                        return Ok(Outcome::Exhausted(elapsed));
                    }
                    return Ok(Outcome::Accepted(line, elapsed, info));
                }
                Some(line) if line.starts_with("info") => info += 1,
                Some(_) => {
                    return Err(self.failure(
                        index,
                        FailureKind::MalformedReply,
                        "search",
                        "unexpected search output",
                    ));
                }
            }
        }
    }
}
