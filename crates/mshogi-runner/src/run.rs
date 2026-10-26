//! Serialized controller: protocol failures cannot fabricate chess outcomes.
use crate::{
    process::{Engine, Output, escaped},
    report::summaries,
    *,
};
use mshogi_core::chess::*;
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::Read,
    sync::mpsc::RecvTimeoutError,
    time::{Duration, Instant},
};

pub(crate) fn identity(path: &str) -> FileIdentity {
    let mut result = FileIdentity {
        path: path.into(),
        canonical_path: None,
        bytes: None,
        sha256: None,
        error: None,
    };
    let read = || -> Result<(String, u64, String), String> {
        let canonical = std::fs::canonicalize(path).map_err(|e| e.to_string())?;
        let metadata = std::fs::metadata(&canonical).map_err(|e| e.to_string())?;
        if !metadata.is_file() {
            return Err("identity path is not a regular file".into());
        }
        let mut file = File::open(&canonical).map_err(|e| e.to_string())?;
        let metadata = file.metadata().map_err(|e| e.to_string())?;
        if !metadata.is_file() {
            return Err("opened identity is not a regular file".into());
        }
        let size = metadata.len();
        if size > 2_147_483_648 {
            return Err("identity file exceeds 2 GiB hashing bound".into());
        }
        let mut digest = Sha256::new();
        let mut bytes = [0u8; 32768];
        let mut total = 0u64;
        loop {
            let n = file.read(&mut bytes).map_err(|e| e.to_string())?;
            if n == 0 {
                break;
            }
            total += n as u64;
            if total > 2_147_483_648 {
                return Err("identity file grew beyond hashing bound".into());
            }
            digest.update(&bytes[..n]);
        }
        if total != size {
            return Err("identity file size changed during hashing".into());
        }
        Ok((
            canonical.to_string_lossy().into_owned(),
            size,
            format!("{:x}", digest.finalize()),
        ))
    };
    match read() {
        Ok((canonical, size, sha)) => {
            result.canonical_path = Some(canonical);
            result.bytes = Some(size);
            result.sha256 = Some(sha)
        }
        Err(e) => result.error = Some(e),
    }
    result
}
struct Log {
    transcript: Transcript,
    bytes: usize,
    cap: usize,
}
impl Log {
    fn new(cap: usize) -> Self {
        Self {
            transcript: Transcript::new(),
            bytes: 0,
            cap,
        }
    }
    fn record(&mut self, mut entry: TranscriptEntry) {
        self.transcript.observed_entries = self.transcript.observed_entries.saturating_add(1);
        entry.sequence = self.transcript.observed_entries;
        let cost = serde_json::to_vec(&entry).expect("finite event").len() + 1;
        if self.bytes + cost <= self.cap {
            self.bytes += cost;
            self.transcript.entries.push(entry)
        } else {
            self.transcript.truncated = true;
            self.transcript.dropped_bytes = self
                .transcript
                .dropped_bytes
                .saturating_add(entry.text.len() as u64);
        }
    }
}
type Ending = (GameStop, Option<RunnerFailure>, Vec<RunnerFailure>);
enum Await {
    Reply(String, u64),
    Failure(RunnerFailure),
    Clock {
        deadline_ns: u64,
        observed_at_ns: u64,
        actor: Color,
    },
}
pub(crate) fn position_command(history: &Game) -> String {
    let mut position = format!("position fen {}", history.initial_position().to_fen());
    if history.ply_count() != 0 {
        position.push_str(" moves");
        for action in history.actions() {
            position.push(' ');
            position.push_str(&action.to_string());
        }
    }
    position
}
pub(crate) fn config_digest(config: &ExperimentConfig) -> String {
    format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(config).expect("finite configuration"))
    )
}
pub(crate) fn go_command(
    config: &ExperimentConfig,
    remaining: Option<[u64; 2]>,
    active: Option<Color>,
    last_at: u64,
    dispatch: u64,
) -> String {
    let mut go = String::from("go");
    if let Some(n) = config.work.depth {
        go.push_str(&format!(" depth {n}"));
    }
    if let Some(n) = config.work.nodes {
        go.push_str(&format!(" nodes {n}"));
    }
    if let Some(n) = config.work.movetime_ms {
        go.push_str(&format!(" movetime {n}"));
    }
    if let (Some(mut balances), TimeControl::Increment { increment_ns, .. }) =
        (remaining, config.time_control)
    {
        if let Some(active) = active {
            let i = usize::from(active == Color::Black);
            balances[i] = balances[i].saturating_sub(dispatch.saturating_sub(last_at));
        }
        go.push_str(&format!(
            " wtime {} btime {} winc {} binc {}",
            balances[0] / 1_000_000,
            balances[1] / 1_000_000,
            increment_ns / 1_000_000,
            increment_ns / 1_000_000
        ));
    }
    go
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Expiry {
    Clock(u64, Color),
    Watchdog(u64),
}
fn earliest_expiry(now: u64, clock: Option<(u64, Color)>, watchdog: u64) -> Option<Expiry> {
    let clock = clock.filter(|(at, _)| *at <= now);
    if let Some((at, side)) = clock
        && (now < watchdog || at <= watchdog)
    {
        return Some(Expiry::Clock(at, side));
    }
    if now >= watchdog {
        return Some(Expiry::Watchdog(watchdog));
    }
    None
}
struct GameRun<'a> {
    config: &'a ExperimentConfig,
    game: ChessMatch,
    engines: [Option<Engine>; 2],
    origin: Instant,
    last_match_ns: u64,
    log: Log,
}
impl<'a> GameRun<'a> {
    fn now(&self) -> u64 {
        u64::try_from(self.origin.elapsed().as_nanos())
            .expect("experiment duration below 584 years")
    }
    fn failure(
        &self,
        kind: FailureKind,
        index: usize,
        phase: &str,
        detail: impl Into<String>,
        watchdog_ms: Option<u64>,
    ) -> RunnerFailure {
        let engine = self.engines[index].as_ref();
        RunnerFailure {
            kind,
            engine: index,
            phase: phase.into(),
            at_ns: self.now(),
            watchdog_ms,
            deadline_ns: None,
            detail: detail.into(),
            exit: engine.and_then(|e| e.exit.clone()),
            last_output: engine.and_then(|e| e.last_output.clone()),
        }
    }
    fn clock_deadline(&self) -> Option<(u64, Color)> {
        let active = self.game.active()?;
        let remaining = self.game.remaining_ns()?;
        self.last_match_ns
            .checked_add(remaining[usize::from(active == Color::Black)])
            .map(|n| (n, active))
    }
    fn submit(&mut self, at_ns: u64, actor: Color, command: MatchCommand) -> MatchReply {
        let reply = self.game.submit(MatchRequest {
            at_ns,
            version: self.game.version(),
            actor,
            command,
        });
        self.last_match_ns = at_ns;
        reply
    }
    fn clock_if_due(&mut self) -> bool {
        self.clock_if_due_at(self.now())
    }
    fn clock_if_due_at(&mut self, observed_at_ns: u64) -> bool {
        if let Some((deadline_ns, actor)) = self.clock_deadline()
            && observed_at_ns >= deadline_ns
        {
            // The deadline selects precedence; the event records when the
            // controller actually detected it, including a delayed wakeup.
            self.submit(
                observed_at_ns,
                actor,
                MatchCommand::Observe { witness: None },
            );
            return true;
        }
        false
    }
    fn send(&mut self, index: usize, line: &str, phase: &str) -> Result<(), Box<RunnerFailure>> {
        self.send_with_timeout(index, line, phase, self.config.watchdogs.write_ms)
    }
    fn send_with_timeout(
        &mut self,
        index: usize,
        line: &str,
        phase: &str,
        timeout_ms: u64,
    ) -> Result<(), Box<RunnerFailure>> {
        self.send_at(index, line, phase, timeout_ms, self.now())
    }
    fn send_at(
        &mut self,
        index: usize,
        line: &str,
        phase: &str,
        timeout_ms: u64,
        at: u64,
    ) -> Result<(), Box<RunnerFailure>> {
        if line.len() + 1 > self.config.io.command_bytes {
            return Err(Box::new(self.failure(
                FailureKind::InvalidResponse,
                index,
                phase,
                "generated command exceeds configured byte cap",
                None,
            )));
        }
        let mut duration = Duration::from_millis(timeout_ms);
        if let Some((at, _)) = self.clock_deadline() {
            duration = duration.min(Duration::from_nanos(at.saturating_sub(self.now())));
        }
        let mut bytes = line.as_bytes().to_vec();
        bytes.push(b'\n');
        let result = self.engines[index]
            .as_mut()
            .expect("spawned engine")
            .write(bytes, duration);
        self.log.record(TranscriptEntry {
            sequence: 0,
            engine: index,
            direction: TransportDirection::Command,
            at_ns: at,
            game_version: self.game.version(),
            phase: phase.into(),
            text: line.into(),
            truncated: false,
            write_completed: Some(result.is_ok()),
        });
        if let Err(detail) = result {
            let _ = self.engines[index].as_mut().unwrap().observe_exit();
            let kind = if self.engines[index]
                .as_ref()
                .unwrap()
                .exit
                .as_ref()
                .is_some_and(|s| !s.success)
            {
                FailureKind::UnexpectedExit
            } else if detail.contains("timed out") || detail.contains("did not complete") {
                FailureKind::ResponseTimeout
            } else {
                FailureKind::MissingResponse
            };
            let mut failure = self.failure(kind, index, phase, detail, Some(timeout_ms));
            if kind == FailureKind::ResponseTimeout {
                failure.deadline_ns = at.checked_add(timeout_ms * 1_000_000);
            }
            return Err(Box::new(failure));
        }
        Ok(())
    }
    fn expired_await(
        &self,
        index: usize,
        phase: &str,
        watchdog_ms: u64,
        expected: &str,
        deadline_ns: u64,
        observed_at: u64,
    ) -> Option<Await> {
        match earliest_expiry(observed_at, self.clock_deadline(), deadline_ns)? {
            Expiry::Clock(deadline_ns, actor) => Some(Await::Clock {
                deadline_ns,
                observed_at_ns: observed_at,
                actor,
            }),
            Expiry::Watchdog(at) => {
                let mut failure = self.failure(
                    FailureKind::ResponseTimeout,
                    index,
                    phase,
                    format!("required {expected} reply absent at watchdog"),
                    Some(watchdog_ms),
                );
                failure.deadline_ns = Some(at);
                failure.at_ns = observed_at;
                Some(Await::Failure(failure))
            }
        }
    }
    fn await_reply(
        &mut self,
        index: usize,
        phase: &str,
        watchdog_ms: u64,
        expected: &str,
    ) -> Await {
        let deadline = Instant::now() + Duration::from_millis(watchdog_ms);
        let deadline_ns = self.now() + watchdog_ms * 1_000_000;
        loop {
            if let Some(event) =
                self.expired_await(index, phase, watchdog_ms, expected, deadline_ns, self.now())
            {
                return event;
            }
            // A non-active opponent may die while this engine is searching.
            for other in 0..2 {
                if other != index && self.engines[other].is_some() {
                    let observed = self.engines[other].as_mut().unwrap().observe_exit();
                    match observed {
                        Ok(Some(_)) => {
                            return Await::Failure(self.failure(
                                FailureKind::UnexpectedExit,
                                other,
                                phase,
                                "idle engine exited",
                                None,
                            ));
                        }
                        Err(e) => {
                            return Await::Failure(self.failure(
                                FailureKind::UnexpectedExit,
                                other,
                                phase,
                                e,
                                None,
                            ));
                        }
                        Ok(None) => {}
                    }
                }
            }
            let remaining = deadline
                .saturating_duration_since(Instant::now())
                .min(Duration::from_millis(2));
            let output = self.engines[index].as_ref().unwrap().receive(remaining);
            let observed_at = self.now();
            if let Some(event) = self.expired_await(
                index,
                phase,
                watchdog_ms,
                expected,
                deadline_ns,
                observed_at,
            ) {
                return event;
            }
            match output {
                Err(RecvTimeoutError::Timeout) => continue,
                Err(RecvTimeoutError::Disconnected) | Ok(Output::Eof) => {
                    self.engines[index].as_mut().unwrap().exit_after_eof();
                    let kind = if self.engines[index]
                        .as_ref()
                        .unwrap()
                        .exit
                        .as_ref()
                        .is_some_and(|s| !s.success)
                    {
                        FailureKind::UnexpectedExit
                    } else {
                        FailureKind::MissingResponse
                    };
                    let mut failure = self.failure(
                        kind,
                        index,
                        phase,
                        format!("EOF before required {expected}"),
                        None,
                    );
                    // Metadata/status acquisition must never move an already
                    // observed EOF past a later chess-clock deadline.
                    failure.at_ns = observed_at;
                    return Await::Failure(failure);
                }
                Ok(Output::Error(e)) => {
                    let mut f = self.failure(
                        FailureKind::MissingResponse,
                        index,
                        phase,
                        format!("stdout read error: {e}"),
                        None,
                    );
                    f.at_ns = observed_at;
                    return Await::Failure(f);
                }
                Ok(Output::Oversized) => {
                    let mut f = self.failure(
                        FailureKind::InvalidResponse,
                        index,
                        phase,
                        "stdout line exceeds configured cap",
                        None,
                    );
                    f.at_ns = observed_at;
                    return Await::Failure(f);
                }
                Ok(Output::Line(bytes)) => {
                    let at = observed_at;
                    // Arrival is serialized controller receipt, not a guessed
                    // kernel-pipe write time or a helper thread's scheduling.
                    if let Some(event) = self.expired_await(
                        index,
                        phase,
                        watchdog_ms,
                        expected,
                        deadline_ns,
                        observed_at,
                    ) {
                        return event;
                    }
                    let text = escaped(&bytes);
                    self.engines[index].as_mut().unwrap().last_output = Some(text.clone());
                    self.log.record(TranscriptEntry {
                        sequence: 0,
                        engine: index,
                        direction: TransportDirection::Response,
                        at_ns: at,
                        game_version: self.game.version(),
                        phase: phase.into(),
                        text,
                        truncated: false,
                        write_completed: None,
                    });
                    if !bytes.is_ascii()
                        || bytes
                            .iter()
                            .any(|b| b.is_ascii_control() && !b.is_ascii_whitespace())
                    {
                        let mut f = self.failure(
                            FailureKind::InvalidResponse,
                            index,
                            phase,
                            "stdout must be ASCII UCI text",
                            None,
                        );
                        f.at_ns = observed_at;
                        return Await::Failure(f);
                    }
                    let line = String::from_utf8(bytes).expect("ASCII");
                    let line = line.trim().to_owned();
                    if line.starts_with("id ") {
                        let engine = self.engines[index].as_mut().unwrap();
                        let remaining =
                            4096usize.saturating_sub(engine.identity.iter().map(String::len).sum());
                        if engine.identity.len() < 32 && remaining != 0 {
                            engine
                                .identity
                                .push(line[..line.len().min(remaining)].to_owned());
                            engine.identity_truncated |= line.len() > remaining;
                        } else {
                            engine.identity_truncated = true;
                        }
                    }
                    if line == expected
                        || (expected == "bestmove"
                            && line.split_ascii_whitespace().next() == Some("bestmove"))
                    {
                        return Await::Reply(line, observed_at);
                    }
                }
            }
        }
    }
    fn failed(&mut self, failure: impl Into<Box<RunnerFailure>>) -> Ending {
        let mut failure = *failure.into();
        // A fixed clock deadline is established before a later observed process
        // failure; equal timestamps choose clock expiry first.
        if let Some((expiry, actor)) = self.clock_deadline()
            && expiry <= failure.deadline_ns.unwrap_or(failure.at_ns)
        {
            self.submit(
                failure.at_ns,
                actor,
                MatchCommand::Observe { witness: None },
            );
        }
        if let Some(engine) = self.engines[failure.engine].as_mut()
            && failure.exit.is_none()
        {
            failure.exit = engine.observe_exit().ok().flatten();
        }
        if self.game.conclusion().is_some() {
            (GameStop::ChessConclusion, None, vec![failure])
        } else {
            (GameStop::Failure, Some(failure), Vec::new())
        }
    }
    fn handle_await(&mut self, awaited: Await) -> Result<(String, u64), Box<Ending>> {
        match awaited {
            Await::Reply(s, at) => Ok((s, at)),
            Await::Failure(f) => Err(Box::new(self.failed(f))),
            Await::Clock {
                deadline_ns,
                observed_at_ns,
                actor,
            } => {
                debug_assert!(observed_at_ns >= deadline_ns);
                self.submit(
                    observed_at_ns,
                    actor,
                    MatchCommand::Observe { witness: None },
                );
                Err(Box::new((GameStop::ChessConclusion, None, Vec::new())))
            }
        }
    }
    fn play(&mut self, colors: [usize; 2]) -> Ending {
        if self.game.conclusion().is_some() {
            return (GameStop::ChessConclusion, None, Vec::new());
        }
        for index in 0..2 {
            match Engine::spawn(index, &self.config.engines[index], self.config.io) {
                Ok(engine) => self.engines[index] = Some(engine),
                Err(e) => {
                    return self.failed(self.failure(
                        FailureKind::ProcessStartFailure,
                        index,
                        "spawn",
                        e,
                        None,
                    ));
                }
            }
        }
        for index in 0..2 {
            if let Err(f) = self.send(index, "uci", "handshake") {
                return self.failed(f);
            }
            let awaited = self.await_reply(
                index,
                "handshake",
                self.config.watchdogs.handshake_ms,
                "uciok",
            );
            if let Err(end) = self.handle_await(awaited) {
                return *end;
            }
            // UCI provides no setoption acknowledgement. Record requested
            // settings and subsequent readiness without inventing acceptance.
            for option in &self.config.engines[index].options.clone() {
                if let Err(f) = self.send(
                    index,
                    &format!("setoption name {} value {}", option.name, option.value),
                    "options",
                ) {
                    return self.failed(f);
                }
            }
            // The own adapter's declared option is protocol-specific. External
            // engines retain their own search preferences; controller claims
            // remain authoritative regardless of those preferences.
            let name = self.engines[index]
                .as_ref()
                .unwrap()
                .identity
                .iter()
                .find_map(|s| s.strip_prefix("id name "));
            if name.is_some_and(|n| n.starts_with("mShogi ")) {
                let policy = if self.config.claim_policy == ClaimPolicy::AutoClaim {
                    "auto"
                } else {
                    "explicit"
                };
                if let Err(f) = self.send(
                    index,
                    &format!("setoption name ClaimPolicy value {policy}"),
                    "options",
                ) {
                    return self.failed(f);
                }
            }
            if let Err(f) = self.send(index, "ucinewgame", "new_game") {
                return self.failed(f);
            }
            if let Err(f) = self.send(index, "isready", "readiness") {
                return self.failed(f);
            }
            let awaited = self.await_reply(
                index,
                "readiness",
                self.config.watchdogs.ready_ms,
                "readyok",
            );
            if let Err(end) = self.handle_await(awaited) {
                return *end;
            }
        }
        let side = self.game.game().position().side_to_move();
        self.submit(self.now(), side, MatchCommand::Start);
        loop {
            if self.game.conclusion().is_some() {
                return (GameStop::ChessConclusion, None, Vec::new());
            }
            if self.clock_if_due() {
                continue;
            }
            if let Some(claim) = self.game.game().policy_claim() {
                self.submit(
                    self.now(),
                    claim.actor,
                    MatchCommand::Claim {
                        reason: claim.reason,
                        intended: claim.intended,
                    },
                );
                continue;
            }
            if self.game.game().history().ply_count() >= self.config.max_plies as usize {
                return (GameStop::PlyLimit, None, Vec::new());
            }
            let side = self.game.game().position().side_to_move();
            let index = colors[usize::from(side == Color::Black)];
            let position = position_command(self.game.game().history());
            if let Err(f) = self.send(index, &position, "position") {
                return self.failed(f);
            }
            if self.clock_if_due() {
                continue;
            }
            if self.game.remaining_ns().is_some() {
                self.submit(self.now(), side, MatchCommand::Observe { witness: None });
                if self.game.conclusion().is_some() {
                    continue;
                }
            }
            let dispatch = self.now();
            let go = go_command(
                self.config,
                self.game.remaining_ns(),
                self.game.active(),
                self.last_match_ns,
                dispatch,
            );
            if let Err(f) = self.send_at(index, &go, "go", self.config.watchdogs.write_ms, dispatch)
            {
                return self.failed(f);
            }
            let awaited = self.await_reply(
                index,
                "bestmove",
                self.config.watchdogs.response_ms,
                "bestmove",
            );
            let (line, received_at) = match self.handle_await(awaited) {
                Ok(reply) => reply,
                Err(end) => return *end,
            };
            let tokens: Vec<_> = line.split_ascii_whitespace().collect();
            let action = match tokens.as_slice() {
                ["bestmove", a] | ["bestmove", a, "ponder", _] => a.parse::<Action>(),
                _ => Err(ActionParseError),
            };
            let action = match action {
                Ok(a) => a,
                Err(e) => {
                    let mut failure = self.failure(
                        FailureKind::InvalidMove,
                        index,
                        "bestmove",
                        format!("raw {line:?}: {e}"),
                        None,
                    );
                    failure.at_ns = received_at;
                    return self.failed(failure);
                }
            };
            let reply = self.submit(received_at, side, MatchCommand::Move(action));
            if let Some(rejection) = reply.rejection {
                let mut failure = self.failure(
                    FailureKind::InvalidMove,
                    index,
                    "bestmove",
                    format!("raw {line:?}: {}", rejection.detail),
                    None,
                );
                failure.at_ns = received_at;
                return self.failed(failure);
            }
        }
    }
    fn cleanup(&mut self) -> Vec<EngineObservation> {
        let mut observations = Vec::new();
        for index in 0..2 {
            if self.engines[index].is_some() {
                // Log the actual attempted stop/quit commands, including write
                // failures, even when a game or process has already concluded.
                let started = Instant::now();
                let budget = self.config.watchdogs.cleanup_ms;
                let write_budget = (budget / 4).max(1);
                if self.game.conclusion().is_none() {
                    let _ = self.send_with_timeout(index, "stop", "cleanup", write_budget);
                }
                let left = budget.saturating_sub(started.elapsed().as_millis() as u64);
                if left != 0 {
                    let _ =
                        self.send_with_timeout(index, "quit", "cleanup", write_budget.min(left));
                }
                let left = budget.saturating_sub(started.elapsed().as_millis() as u64);
                observations.push(self.engines[index].as_mut().unwrap().cleanup(left));
            }
        }
        observations
    }
}
/// Run bounded paired UCI games, retaining authoritative replay and failure data.
/// Configuration errors fail before launching children; game/process failures
/// produce successful report values whose games are explicitly unscored.
pub fn run_experiment(config: &ExperimentConfig) -> Result<ExperimentReport, RunnerError> {
    config.validate()?;
    let engine_identities = std::array::from_fn(|i| EngineIdentity {
        executable: identity(&config.engines[i].path),
        model: config.engines[i].model_path.as_ref().map(|p| identity(p)),
        model_identity: config.engines[i].model_identity.clone(),
    });
    let deterministic_rerun_eligible = config.rerun_determinism_eligible()
        && engine_identities.iter().all(|e| {
            e.executable.sha256.is_some() && e.model.as_ref().is_none_or(|m| m.sha256.is_some())
        });
    let mut games = Vec::new();
    for (opening_index, opening) in config.openings.iter().enumerate() {
        for pairing in 0..if config.paired { 2 } else { 1 } {
            let game = ChessMatch::new(config.opening_game(opening)?, config.time_control)
                .map_err(|e| RunnerError(e.to_string()))?;
            let mut run = GameRun {
                config,
                game,
                engines: [None, None],
                origin: Instant::now(),
                last_match_ns: 0,
                log: Log::new(config.io.transcript_bytes),
            };
            let engines_by_color = if pairing == 0 { [0, 1] } else { [1, 0] };
            let (stop, failure, diagnostic_failures) = run.play(engines_by_color);
            let engines = run.cleanup();
            games.push(GameReport {
                opening_index,
                opening_id: opening.id.clone(),
                pairing,
                engines_by_color,
                stop,
                conclusion: run.game.conclusion(),
                failure,
                diagnostic_failures,
                match_record_json: run.game.record().to_json(),
                transcript: run.log.transcript,
                engines,
            });
        }
    }
    let summaries = summaries(config, &games);
    Ok(ExperimentReport {
        schema_version: 1,
        config: config.clone(),
        config_sha256: config_digest(config),
        engine_identities,
        runner_version: env!("CARGO_PKG_VERSION").into(),
        os: std::env::consts::OS.into(),
        arch: std::env::consts::ARCH.into(),
        deterministic_rerun_eligible,
        games,
        summaries,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn config() -> ExperimentConfig {
        let engine = EngineConfig {
            path: "unused".into(),
            args: vec![],
            options: vec![],
            model_path: None,
            model_identity: None,
            deterministic: false,
        };
        ExperimentConfig {
            schema_version: 1,
            id: "clock test".into(),
            engines: [engine.clone(), engine],
            openings: vec![],
            paired: false,
            claim_policy: ClaimPolicy::AutoClaim,
            time_control: TimeControl::Increment {
                initial_ns: [50_000_000, 50_000_000],
                increment_ns: 2_000_000,
            },
            work: WorkLimit {
                depth: Some(2),
                nodes: Some(1000),
                movetime_ms: None,
            },
            max_plies: 512,
            watchdogs: Watchdogs::default(),
            io: IoLimits::default(),
        }
    }
    #[test]
    fn go_projects_elapsed_transport_without_mutating_or_double_charging_clock() {
        let c = config();
        let balances = [50_000_000, 50_000_000];
        assert_eq!(
            go_command(&c, Some(balances), Some(Color::White), 0, 49_000_000),
            "go depth 2 nodes 1000 wtime 1 btime 50 winc 2 binc 2"
        );
        assert_eq!(balances, [50_000_000, 50_000_000]);
        assert_eq!(
            go_command(
                &c,
                Some([1_000_000, 50_000_000]),
                Some(Color::White),
                49_000_000,
                49_000_000
            ),
            "go depth 2 nodes 1000 wtime 1 btime 50 winc 2 binc 2"
        );
        assert_eq!(
            go_command(&c, Some(balances), Some(Color::Black), 0, 50_000_000),
            "go depth 2 nodes 1000 wtime 50 btime 0 winc 2 binc 2"
        );
    }
    #[test]
    fn earliest_deadline_wins_even_if_controller_wakes_after_both() {
        assert_eq!(
            earliest_expiry(60, Some((50, Color::White)), 40),
            Some(Expiry::Watchdog(40))
        );
        assert_eq!(
            earliest_expiry(60, Some((40, Color::Black)), 50),
            Some(Expiry::Clock(40, Color::Black))
        );
        assert_eq!(
            earliest_expiry(60, Some((40, Color::White)), 40),
            Some(Expiry::Clock(40, Color::White))
        );
        assert_eq!(earliest_expiry(30, Some((40, Color::White)), 50), None);
    }
    #[test]
    fn fixed_reply_receipt_precedes_later_processing_and_status_collection() {
        let c = config();
        let g = AdjudicatedGame::from_fen("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", ClaimPolicy::AutoClaim)
            .unwrap();
        let game = ChessMatch::new(g, c.time_control).unwrap();
        let mut run = GameRun {
            config: &c,
            game,
            engines: [None, None],
            origin: Instant::now(),
            last_match_ns: 0,
            log: Log::new(1024),
        };
        run.submit(0, Color::White, MatchCommand::Start);
        let (line, receipt) = run
            .handle_await(Await::Reply("bestmove g6g7".into(), 49_000_000))
            .unwrap();
        // A later parse/log/status operation may occur after 50ms; the already
        // established 49ms command arrival remains the authoritative event.
        let parsed = line
            .split_ascii_whitespace()
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let reply = run.submit(receipt, Color::White, MatchCommand::Move(parsed));
        assert_eq!(
            reply.conclusion,
            Some(MatchConclusion::Chess(ChessOutcome::Checkmate {
                winner: Color::White
            }))
        );
        assert_eq!(reply.remaining_ns, Some([1_000_000, 50_000_000]));
        let eof = RunnerFailure {
            kind: FailureKind::MissingResponse,
            engine: 0,
            phase: "bestmove".into(),
            at_ns: 20,
            watchdog_ms: None,
            deadline_ns: None,
            detail: "EOF; exit metadata obtained at 32".into(),
            exit: None,
            last_output: None,
        };
        assert!(25 > eof.deadline_ns.unwrap_or(eof.at_ns));
        let timeout = RunnerFailure {
            kind: FailureKind::ResponseTimeout,
            at_ns: 60,
            deadline_ns: Some(40),
            ..eof
        };
        assert!(50 > timeout.deadline_ns.unwrap_or(timeout.at_ns));
    }
    #[test]
    fn delayed_clock_detection_records_observation_time_in_all_paths() {
        let c = config();
        for path in ["await", "poll", "failure"] {
            let g =
                AdjudicatedGame::from_fen("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", ClaimPolicy::AutoClaim)
                    .unwrap();
            let game = ChessMatch::new(g, c.time_control).unwrap();
            let mut run = GameRun {
                config: &c,
                game,
                engines: [None, None],
                origin: Instant::now(),
                last_match_ns: 0,
                log: Log::new(1024),
            };
            run.submit(0, Color::White, MatchCommand::Start);
            let deadline = 50_000_000;
            let observed = 80_000_000;
            match path {
                "await" => {
                    let event = run
                        .expired_await(0, "bestmove", 100, "bestmove", 100_000_000, observed)
                        .unwrap();
                    assert!(
                        matches!(event, Await::Clock { deadline_ns, observed_at_ns, actor: Color::White } if deadline_ns == deadline && observed_at_ns == observed)
                    );
                    assert!(run.handle_await(event).is_err());
                }
                "poll" => assert!(run.clock_if_due_at(observed)),
                "failure" => {
                    let failure = RunnerFailure {
                        kind: FailureKind::ResponseTimeout,
                        engine: 0,
                        phase: "bestmove".into(),
                        at_ns: observed,
                        watchdog_ms: Some(70),
                        deadline_ns: Some(70_000_000),
                        detail: "detected after both deadlines".into(),
                        exit: None,
                        last_output: None,
                    };
                    let (stop, primary, diagnostics) = run.failed(failure);
                    assert_eq!(stop, GameStop::ChessConclusion);
                    assert!(primary.is_none());
                    assert_eq!(diagnostics[0].at_ns, observed);
                }
                _ => unreachable!(),
            }
            let raw: serde_json::Value =
                serde_json::from_str(&run.game.record().to_json()).unwrap();
            assert_eq!(raw["events"][1]["request"]["at_ns"], observed, "{path}");
            assert_ne!(raw["events"][1]["request"]["at_ns"], deadline, "{path}");
            assert_eq!(run.game.remaining_ns(), Some([0, 50_000_000]));
            assert_eq!(
                run.game.conclusion(),
                Some(MatchConclusion::Draw {
                    trigger: Some(TerminationTrigger::FlagFall)
                })
            );
            assert_eq!(run.game.record().replay().unwrap(), run.game);
        }
    }
}
