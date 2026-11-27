//! Serialized controller: protocol failures cannot fabricate chess outcomes.
use crate::{
    process::Engine,
    protocol::Failure,
    report::summaries,
    transport::{Dialect, Transport},
    *,
};
use mshogi_core::chess::*;

use std::time::Duration;

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
    crate::common::sha256(serde_json::to_vec(config).expect("finite configuration"))
}
pub(crate) fn go_command(
    config: &ExperimentConfig,
    engine: usize,
    remaining: Option<[u64; 2]>,
    active: Option<Color>,
    last_at: u64,
    dispatch: u64,
) -> String {
    let work = config.work_for(engine).expect("admitted participant");
    let mut go = String::from("go");
    if let Some(n) = work.depth {
        go.push_str(&format!(" depth {n}"));
    }
    if let Some(n) = work.nodes {
        go.push_str(&format!(" nodes {n}"));
    }
    if let Some(n) = work.movetime_ms {
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
    last_match_ns: u64,
    log: Transport<Transcript>,
}
impl<'a> GameRun<'a> {
    fn now(&self) -> u64 {
        crate::common::elapsed_ns(self.log.start)
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
        let mut budget = Duration::from_millis(timeout_ms);
        if let Some((deadline, _)) = self.clock_deadline() {
            budget = budget.min(Duration::from_nanos(deadline.saturating_sub(self.now())));
        }
        self.log.version = self.game.version() as usize;
        let (_, result) = self.log.write_at(
            self.engines[index].as_mut().unwrap(),
            phase,
            line,
            budget,
            at,
        );
        result.map_err(|e| {
            Box::new(self.transport_failure(e, Some(timeout_ms), at, self.log.observed))
        })
    }
    fn transport_failure(
        &self,
        error: Failure,
        timeout: Option<u64>,
        dispatch: u64,
        observed: u64,
    ) -> RunnerFailure {
        let mut failure = self.failure(
            error.kind,
            error.engine,
            &error.phase,
            error.detail,
            timeout,
        );
        failure.at_ns = observed;
        if failure.kind == FailureKind::Watchdog {
            failure.deadline_ns = timeout.and_then(|ms| dispatch.checked_add(ms * 1_000_000));
        }
        failure
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
                    FailureKind::Watchdog,
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
        let dispatch = self.now();
        let watchdog = dispatch + watchdog_ms * 1_000_000;
        let deadline = self
            .clock_deadline()
            .map_or(watchdog, |(n, _)| n.min(watchdog));
        self.log.version = self.game.version() as usize;
        loop {
            let [first, second] = &mut self.engines;
            let (active, peer) = if index == 0 {
                (first, second)
            } else {
                (second, first)
            };
            let result = self.log.line(
                active.as_mut().unwrap(),
                phase,
                self.log.start + Duration::from_nanos(deadline),
                peer.as_mut(),
            );
            let observed = self.log.observed;
            if let Some(event) =
                self.expired_await(index, phase, watchdog_ms, expected, watchdog, observed)
            {
                return event;
            }
            match result {
                Ok(Some(line)) if line.split_ascii_whitespace().next() == Some(expected) => {
                    return Await::Reply(line, observed);
                }
                Ok(Some(line)) if !line.trim_start().starts_with("info ") => {
                    let mut failure = self.failure(
                        FailureKind::MalformedReply,
                        index,
                        phase,
                        "unexpected search output",
                        None,
                    );
                    failure.at_ns = observed;
                    return Await::Failure(failure);
                }
                Err(e) => {
                    return Await::Failure(self.transport_failure(e, None, dispatch, observed));
                }
                _ => (),
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
        if self.engines[failure.engine]
            .as_ref()
            .is_some_and(|e| e.unsettled_pipe)
            && let Some(status) = failure.exit.as_ref()
        {
            failure.kind = if status.success {
                FailureKind::MissingReply
            } else {
                FailureKind::UnexpectedExit
            };
            self.engines[failure.engine]
                .as_mut()
                .unwrap()
                .unsettled_pipe = false;
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
                        FailureKind::ProcessStart,
                        index,
                        "spawn",
                        e,
                        None,
                    ));
                }
            }
        }
        for index in 0..2 {
            let [first, second] = &mut self.engines;
            let (active, peer) = if index == 0 {
                (first, second)
            } else {
                (second, first)
            };
            let policy = if self.config.claim_policy == ClaimPolicy::AutoClaim {
                "auto"
            } else {
                "explicit"
            };
            if let Err(e) = self.log.prepare(
                active.as_mut().unwrap(),
                Dialect::Uci,
                &self.config.engines[index].options,
                Some(policy),
                peer.as_mut(),
            ) {
                let (dispatch, millis) = self.log.phase_budget;
                let f = self.transport_failure(e, Some(millis), dispatch, self.log.observed);
                return self.failed(f);
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
            if self.game.game().history().ply_count() >= self.config.max_plies {
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
                index,
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
                        FailureKind::MalformedReply,
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
                    FailureKind::IllegalAction,
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
    fn cleanup(&mut self, mut failure: Option<&mut RunnerFailure>) -> Vec<EngineObservation> {
        let mut observations = Vec::new();
        self.log.version = self.game.version() as usize;
        for engine in self.engines.iter_mut().flatten() {
            let (observation, _) = self.log.retire(engine);
            if engine.unsettled_pipe
                && let Some(f) = failure.as_deref_mut().filter(|f| f.engine == engine.index)
                && let Some(status) = engine.refine_pipe_failure(&mut f.kind, &mut f.detail)
            {
                f.exit = Some(status);
            }
            observations.push(observation);
        }
        observations
    }
}
/// Run bounded paired UCI games, retaining authoritative replay and failure data.
/// Configuration errors fail before launching children; game/process failures
/// produce successful report values whose games are explicitly unscored.
pub fn run_experiment(config: &ExperimentConfig) -> Result<ExperimentReport, RunnerError> {
    config.validate()?;
    let engine_identities = crate::common::identities(&config.engines);
    let deterministic_rerun_eligible = crate::common::rerun_eligible(config, &engine_identities);
    let mut report = ExperimentReport {
        schema_version: 1,
        config: config.clone(),
        config_sha256: config_digest(config),
        engine_identities,
        runner_version: env!("CARGO_PKG_VERSION").into(),
        os: std::env::consts::OS.into(),
        arch: std::env::consts::ARCH.into(),
        deterministic_rerun_eligible,
        games: vec![],
        summaries: vec![],
    };
    let mut budget = crate::common::ReportBudget::FULL;
    budget.include(&report)?;
    for (opening_index, opening) in config.openings.iter().enumerate() {
        for pairing in 0..if config.paired { 2 } else { 1 } {
            let game = ChessMatch::new(config.opening_game(opening)?, config.time_control)
                .map_err(|e| RunnerError(e.to_string()))?;
            let mut run = GameRun {
                config,
                game,
                engines: [None, None],
                last_match_ns: 0,
                log: Transport::new(config.watchdogs, config.io, None, Transcript::new()),
            };
            run.log.report_budget = budget;
            let engines_by_color = if pairing == 0 { [0, 1] } else { [1, 0] };
            let (stop, mut failure, mut diagnostic_failures) = run.play(engines_by_color);
            let engines = run.cleanup(failure.as_mut().or_else(|| diagnostic_failures.first_mut()));
            run.log.journal.truncated |= !run.log.complete;
            let entry = GameReport {
                opening_index,
                opening_id: opening.id.clone(),
                pairing,
                engines_by_color,
                stop,
                conclusion: run.game.conclusion(),
                failure,
                diagnostic_failures,
                match_record_json: run.game.record().to_json(),
                transcript: run.log.journal,
                engines,
            };
            budget.charge(
                crate::common::encoded_len(&entry)? + usize::from(!report.games.is_empty()),
            )?;
            report.games.push(entry);
        }
    }
    report.summaries = summaries(config, &report.games);
    crate::common::ReportBudget::FULL.check(&report)?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::process::WriteFailure;
    use std::time::Instant;
    // Invoked as a real subprocess of this test executable, never as an engine
    // opponent. Normal test discovery must not terminate its own test process.
    #[test]
    #[ignore = "subprocess fixture, exercised by controlled_early_write_failure"]
    fn controlled_nonzero_exit_child() {
        eprintln!("controlled runner child exits with code 7");
        std::process::exit(7);
    }
    #[test]
    #[ignore = "live subprocess fixture, exercised by controlled cleanup test"]
    fn controlled_live_child() {
        std::thread::sleep(Duration::from_secs(60));
    }
    fn exited_child(nonzero: bool) -> Engine {
        let engine = EngineConfig {
            path: std::env::current_exe()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            args: if nonzero {
                vec![
                    "--ignored".into(),
                    "--exact".into(),
                    "run::tests::controlled_nonzero_exit_child".into(),
                    "--nocapture".into(),
                ]
            } else {
                // A zero-match libtest invocation exits successfully.
                vec!["--exact".into(), "no_such_controlled_child_test".into()]
            },
            options: vec![],
            model_path: None,
            model_identity: None,
            deterministic: true,
        };
        let mut child = Engine::spawn(0, &engine, IoLimits::default()).unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        while child.observe_exit().unwrap().is_none() {
            assert!(Instant::now() < deadline, "controlled child did not exit");
            std::thread::sleep(Duration::from_millis(1));
        }
        assert_eq!(
            child.exit.as_ref().unwrap().code,
            Some(if nonzero { 7 } else { 0 })
        );
        // The test harness prints its own banner. Drain it before observing
        // the closed stdout channel as the missing protocol reply.
        while let Ok(crate::process::Output::Line(_)) = child.receive(Duration::from_secs(1)) {}
        child.defer_exit_observations(2);
        child
    }
    #[test]
    fn controlled_early_write_failure_keeps_nonzero_exit_diagnostics() {
        let c = config();
        let game = ChessMatch::new(
            AdjudicatedGame::from_fen("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", ClaimPolicy::AutoClaim)
                .unwrap(),
            TimeControl::Untimed,
        )
        .unwrap();
        let mut run = GameRun {
            config: &c,
            game,
            engines: [Some(exited_child(true)), None],
            last_match_ns: 0,
            log: Transport::new(c.watchdogs, c.io, None, Transcript::new()),
        };
        let before = run.game.record().clone();
        // Real closed stdin yields a real failed write; two subsequent exit
        // observations are unavailable. Old send_at/failed froze MissingResponse.
        let failure = run.send(0, "uci", "handshake").unwrap_err();
        let (stop, failure, diagnostics) = run.failed(failure);
        let failure = failure.unwrap();
        assert_eq!(stop, GameStop::Failure);
        assert!(diagnostics.is_empty());
        assert_eq!(failure.kind, FailureKind::UnexpectedExit);
        assert_eq!(failure.exit.as_ref().unwrap().code, Some(7));
        assert_eq!(failure.engine, 0);
        assert_eq!(failure.phase, "handshake");
        assert_eq!(run.game.conclusion(), None);
        let observations = run.cleanup(None);
        assert!(
            observations[0]
                .stderr
                .text
                .contains("controlled runner child")
        );
        assert!(observations[0].cleanup_failures.is_empty());
        assert_eq!(run.game.record(), &before);
        assert_eq!(before.replay().unwrap(), run.game);
    }
    fn write_failure(
        run: &mut GameRun<'_>,
        index: usize,
        phase: &str,
        error: WriteFailure,
        timeout_ms: u64,
        dispatch: u64,
        observed: u64,
    ) -> RunnerFailure {
        let error = run
            .log
            .write_failure(run.engines[index].as_mut().unwrap(), phase, error);
        run.transport_failure(error, Some(timeout_ms), dispatch, observed)
    }
    fn controlled_run(c: &ExperimentConfig, child: Engine) -> GameRun<'_> {
        GameRun {
            config: c,
            game: ChessMatch::new(
                AdjudicatedGame::from_fen("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", ClaimPolicy::AutoClaim)
                    .unwrap(),
                c.time_control,
            )
            .unwrap(),
            engines: [Some(child), None],
            last_match_ns: 0,
            log: Transport::new(c.watchdogs, c.io, None, Transcript::new()),
        }
    }
    #[test]
    fn controlled_clean_write_failure_is_missing_response() {
        let c = config();
        let mut run = controlled_run(&c, exited_child(false));
        let failure = run.send(0, "uci", "handshake").unwrap_err();
        let (stop, failure, _) = run.failed(failure);
        let failure = failure.unwrap();
        assert_eq!(stop, GameStop::Failure);
        assert_eq!(failure.kind, FailureKind::MissingReply);
        assert!(failure.exit.as_ref().unwrap().success);
        assert_eq!(failure.exit.as_ref().unwrap().code, Some(0));
        assert_eq!(run.game.conclusion(), None);
        assert!(run.cleanup(None)[0].cleanup_failures.is_empty());
    }
    #[test]
    fn controlled_late_natural_exit_finalizes_before_cleanup_intervention() {
        let c = config();
        for nonzero in [true, false] {
            let mut child = exited_child(nonzero);
            // Unavailable through the 10ms immediate settlement and failed(),
            // then definitive during the EXISTING cleanup grace.
            child.defer_exit_observations(12);
            let mut run = controlled_run(&c, child);
            let failure = run.send(0, "uci", "handshake").unwrap_err();
            let (_, failure, _) = run.failed(failure);
            let mut failure = failure.unwrap();
            assert_eq!(failure.kind, FailureKind::MissingReply);
            assert_eq!(failure.exit, None);
            let receipt = failure.at_ns;
            let observations = run.cleanup(Some(&mut failure));
            let expected = if nonzero {
                FailureKind::UnexpectedExit
            } else {
                FailureKind::MissingReply
            };
            assert_eq!(failure.kind, expected);
            assert_eq!(
                failure.exit.as_ref().unwrap().code,
                Some(if nonzero { 7 } else { 0 })
            );
            assert_eq!(failure.at_ns, receipt);
            assert_eq!(observations[0].exit, failure.exit);
            assert!(observations[0].cleanup_failures.is_empty());
            assert_eq!(run.game.conclusion(), None);
        }
    }
    #[test]
    fn controlled_live_closed_stdin_does_not_treat_supervisor_kill_as_original_exit() {
        let mut c = config();
        c.watchdogs.cleanup_ms = 100;
        let fixture = EngineConfig {
            path: std::env::current_exe()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            args: vec![
                "--ignored".into(),
                "--exact".into(),
                "run::tests::controlled_live_child".into(),
            ],
            options: vec![],
            model_path: None,
            model_identity: None,
            deterministic: true,
        };
        let mut child = Engine::spawn(0, &fixture, c.io).unwrap();
        assert!(child.observe_exit().unwrap().is_none());
        child.close_stdin_for_test();
        let mut run = controlled_run(&c, child);
        let mut peer = exited_child(false);
        peer.index = 1;
        run.engines[1] = Some(peer);
        let failure = run.send(0, "uci", "handshake").unwrap_err();
        let (_, failure, _) = run.failed(failure);
        let mut failure = failure.unwrap();
        let receipt = failure.at_ns;
        let observations = run.cleanup(Some(&mut failure));
        assert_eq!(failure.kind, FailureKind::MissingReply);
        assert_eq!(failure.exit, None);
        assert_eq!(failure.at_ns, receipt);
        assert!(!observations[0].exit.as_ref().unwrap().success);
        assert!(observations[0].cleanup_failures.is_empty());
        assert!(!run.log.complete);
        assert!(
            run.log
                .journal
                .entries
                .iter()
                .any(|e| e.engine == 1 && e.phase == "cleanup" && e.text == "quit")
        );
        assert_eq!(run.game.conclusion(), None);
    }
    #[test]
    fn controlled_late_stdout_eof_status_finalizes_before_intervention() {
        let c = config();
        let mut child = exited_child(true);
        child.defer_exit_observations(12);
        let mut run = controlled_run(&c, child);
        let awaited = run.await_reply(0, "handshake", 2000, "uciok");
        let Await::Failure(failure) = awaited else {
            panic!("expected required-response EOF")
        };
        let (_, failure, _) = run.failed(failure);
        let mut failure = failure.unwrap();
        assert_eq!(failure.kind, FailureKind::MissingReply);
        let receipt = failure.at_ns;
        let observations = run.cleanup(Some(&mut failure));
        assert_eq!(failure.kind, FailureKind::UnexpectedExit);
        assert_eq!(failure.exit.as_ref().unwrap().code, Some(7));
        assert_eq!(failure.at_ns, receipt);
        assert!(observations[0].cleanup_failures.is_empty());
        assert_eq!(run.game.conclusion(), None);
    }
    #[test]
    fn controlled_single_sample_negative_control_reproduces_missing_response() {
        let c = config();
        let mut child = exited_child(true);
        child.single_exit_sample_control();
        let mut run = controlled_run(&c, child);
        // Run on every native CI platform: the original one-sample ordering
        // freezes the wrong category despite this child's actual exit 7.
        let failure = run.send(0, "uci", "handshake").unwrap_err();
        let (_, failure, _) = run.failed(failure);
        let mut failure = failure.unwrap();
        assert_eq!(failure.kind, FailureKind::MissingReply);
        assert_eq!(failure.exit, None);
        let actual = run.cleanup(Some(&mut failure));
        assert_eq!(actual[0].exit.as_ref().unwrap().code, Some(7));
        assert!(actual[0].cleanup_failures.is_empty());
        assert_eq!(failure.kind, FailureKind::UnexpectedExit);
        assert_eq!(failure.exit.as_ref().unwrap().code, Some(7));
        assert_eq!(run.game.conclusion(), None);
    }
    #[test]
    fn controlled_fast_and_slow_status_visibility_within_existing_bound() {
        let c = config();
        for lag in [1, 2, 9, 10] {
            let mut child = exited_child(true);
            child.defer_exit_observations(lag);
            let mut run = controlled_run(&c, child);
            let failure = run.send(0, "uci", "handshake").unwrap_err();
            assert_eq!(failure.kind, FailureKind::UnexpectedExit, "lag={lag}");
            assert_eq!(failure.exit.as_ref().unwrap().code, Some(7));
            assert!(run.cleanup(None)[0].cleanup_failures.is_empty());
        }
    }
    #[test]
    fn controlled_write_failure_receipt_and_clock_precedence_are_preserved() {
        let c = config();
        for lag in [2, 12] {
            for observed_at in [49_000_000, 60_000_000] {
                let mut child = exited_child(true);
                child.defer_exit_observations(lag);
                let mut run = controlled_run(&c, child);
                run.submit(0, Color::White, MatchCommand::Start);
                let failure = write_failure(
                    &mut run,
                    0,
                    "go",
                    WriteFailure::Transport("closed pipe".into()),
                    100,
                    40_000_000,
                    observed_at,
                );
                assert_eq!(failure.at_ns, observed_at);
                let (stop, mut failure, mut diagnostics) = run.failed(failure);
                let observations =
                    run.cleanup(failure.as_mut().or_else(|| diagnostics.first_mut()));
                let finalized = failure.as_ref().or_else(|| diagnostics.first()).unwrap();
                assert_eq!(finalized.kind, FailureKind::UnexpectedExit);
                assert_eq!(finalized.exit.as_ref().unwrap().code, Some(7));
                assert_eq!(finalized.at_ns, observed_at);
                if observed_at == 49_000_000 {
                    assert_eq!(stop, GameStop::Failure);
                    assert!(failure.is_some());
                    assert_eq!(run.game.conclusion(), None);
                } else {
                    assert_eq!(stop, GameStop::ChessConclusion);
                    assert!(failure.is_none());
                    assert_eq!(diagnostics[0].at_ns, observed_at);
                    assert!(run.game.conclusion().is_some());
                }
                assert!(observations[0].cleanup_failures.is_empty());
            }
        }
    }
    #[test]
    fn controlled_timeout_and_cleanup_do_not_settle_or_guess_from_error_text() {
        let c = config();
        for (phase, error, expected) in [
            (
                "options",
                WriteFailure::Timeout("acknowledgement deadline".into()),
                FailureKind::Watchdog,
            ),
            (
                "options",
                WriteFailure::Transport("peer says timed out".into()),
                FailureKind::MissingReply,
            ),
            (
                "cleanup",
                WriteFailure::Transport("closed pipe".into()),
                FailureKind::MissingReply,
            ),
        ] {
            let mut child = exited_child(true);
            child.defer_exit_observations(if phase == "cleanup" { 2 } else { usize::MAX });
            let mut run = controlled_run(&c, child);
            let failure = write_failure(&mut run, 0, phase, error, 10, 40_000_000, 60_000_000);
            assert_eq!(failure.kind, expected);
            assert_eq!(failure.at_ns, 60_000_000);
            assert_eq!(failure.exit, None);
            assert_eq!(
                failure.deadline_ns,
                (expected == FailureKind::Watchdog).then_some(50_000_000)
            );
            run.engines[0].as_mut().unwrap().defer_exit_observations(0);
            assert!(run.cleanup(None)[0].cleanup_failures.is_empty());
        }
    }
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
            engine_work: None,
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
            go_command(&c, 0, Some(balances), Some(Color::White), 0, 49_000_000),
            "go depth 2 nodes 1000 wtime 1 btime 50 winc 2 binc 2"
        );
        assert_eq!(balances, [50_000_000, 50_000_000]);
        assert_eq!(
            go_command(
                &c,
                0,
                Some([1_000_000, 50_000_000]),
                Some(Color::White),
                49_000_000,
                49_000_000
            ),
            "go depth 2 nodes 1000 wtime 1 btime 50 winc 2 binc 2"
        );
        assert_eq!(
            go_command(&c, 0, Some(balances), Some(Color::Black), 0, 50_000_000),
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
            last_match_ns: 0,
            log: Transport::new(c.watchdogs, c.io, None, Transcript::new()),
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
            kind: FailureKind::MissingReply,
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
            kind: FailureKind::Watchdog,
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
                last_match_ns: 0,
                log: Transport::new(c.watchdogs, c.io, None, Transcript::new()),
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
                        kind: FailureKind::Watchdog,
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
