use super::*;
use mshogi_engine::{BridgeOptions, EngineBridge, ExecutionRequest, PROTOCOL_VERSION};
use std::{
    thread,
    time::{Duration, Instant},
};

/// Run actual bounded protocol4 subprocesses over native full-history owners.
pub fn run(config: &Config) -> Result<Report, RunnerError> {
    run_with_stop(config, &mut || false)
}
/// Cooperative experiment cancellation; never a scored native game result.
pub fn run_with_stop(
    config: &Config,
    stopped: &mut impl FnMut() -> bool,
) -> Result<Report, RunnerError> {
    config.validate()?;
    let identities: Vec<_> = config
        .players
        .iter()
        .map(|p| EngineIdentity {
            executable: crate::run::identity(&p.executable),
            model: None,
            model_identity: Some("executable-embedded".into()),
        })
        .collect();
    if identities
        .iter()
        .any(|i| i.executable.error.is_some() || i.executable.sha256.is_none())
    {
        return Err(err("cannot fingerprint an actual participant executable"));
    }
    let mut report = Report {
        schema: SCHEMA,
        config: config.clone(),
        config_sha256: digest(&serde_json::to_string(config).map_err(err)?),
        schedule_algorithm: SCHEDULE.into(),
        environment: Environment::observed(),
        identities,
        games: vec![],
    };
    // Reserve room for final roots/diagnostics before any work; observations are
    // charged by their actual encoding. These are experiment, not native limits.
    let mut charged = serde_json::to_vec(&report).map_err(err)?.len();
    for pairing in config.schedule() {
        charged = charged.saturating_add(
            serde_json::to_vec(
                &native::NativeState::from_root(&config.openings[pairing.opening].root)
                    .map_err(err)?
                    .root()
                    .map_err(err)?,
            )
            .map_err(err)?
            .len()
                + 8192,
        );
    }
    let deadline = Instant::now() + Duration::from_millis(config.wall_budget_ms);
    let mut id = 0u64;
    for pairing in config.schedule() {
        let opening = &config.openings[pairing.opening];
        let mut state = native::NativeState::from_root(&opening.root).map_err(err)?;
        let initial = state.root().map_err(err)?;
        let mut game = Game {
            pairing,
            steps: vec![],
            final_root: initial,
            native_outcome: None,
            score_first_twice: None,
            stop: "step_limit".into(),
            cleanup: vec![],
        };
        let bridge_options = BridgeOptions {
            watchdog: Duration::from_millis(config.watchdog_ms),
            cleanup: Duration::from_millis(config.cleanup_ms),
        };
        let bridges: Vec<_> = config
            .players
            .iter()
            .map(|p| EngineBridge::with_options(p.executable.clone().into(), bridge_options))
            .collect::<Result<_, _>>()
            .map_err(err)?;
        loop {
            if let Some(reason) = state.stop_reason() {
                game.stop = reason;
                break;
            }
            if stopped() {
                game.stop = "cancelled".into();
                break;
            }
            if Instant::now() >= deadline {
                game.stop = "wall_budget".into();
                break;
            }
            if game.steps.len() >= usize::from(config.max_steps) {
                break;
            }
            // Reserve more than one bounded response + request + native final
            // record before submitting; do not search and then discard evidence.
            if charged > MAX_BYTES - 8 * 1024 * 1024 {
                game.stop = "artifact_limit".into();
                break;
            }
            let before = state.root().map_err(err)?;
            let player = if first_side(state.turn()) != game.pairing.swapped {
                0
            } else {
                game.pairing.baseline
            };
            id += 1;
            let request = ExecutionRequest {
                protocol: PROTOCOL_VERSION,
                context: ExecutionContext {
                    purpose: ExecutionPurpose::EngineTurn {},
                    request_id: id,
                    session_version: id,
                    process_generation: id,
                    game: before.record.clone(),
                    limits: config.players[player].work(before.record.profile())?,
                },
            };
            let began = Instant::now();
            bridges[player].submit(request.clone()).map_err(err)?;
            let mut failure = None;
            let observation = loop {
                if stopped() {
                    failure = Some("cancelled".to_owned());
                    break None;
                }
                if Instant::now() >= deadline {
                    failure = Some("wall_budget".to_owned());
                    break None;
                }
                if let Some(response) = bridges[player].poll() {
                    break Some(response.observation);
                }
                thread::sleep(Duration::from_millis(1));
            };
            let elapsed_ns = began.elapsed().as_nanos().min(u128::from(u64::MAX)) as u64;
            if observation.is_none() {
                bridges[player].cancel();
            }
            let mut candidate = native::NativeState::from_root(&before).map_err(err)?;
            let decision = if let Some(o) = &observation {
                if !o.matches(&request.context) {
                    failure = Some("observation:context".into());
                    native::Admission::Failed("observation:context".into())
                } else if !within_limits(o) {
                    failure = Some("observation:limits".into());
                    native::Admission::Failed("observation:limits".into())
                } else {
                    match admit_bounded(&mut candidate, &o.result, request.context.limits) {
                        Ok(a) => a,
                        Err(e) => {
                            failure = Some(e.clone());
                            native::Admission::Failed(format!("admission:{e}"))
                        }
                    }
                }
            } else {
                native::Admission::Failed(failure.clone().unwrap())
            };
            let step = Step {
                player,
                elapsed_ns,
                request: request.context,
                observation,
                failure,
                applied: matches!(decision, native::Admission::Applied),
            };
            let after = candidate.root().map_err(err)?;
            let cost = serde_json::to_vec(&step).map_err(err)?.len()
                + serde_json::to_vec(&after).map_err(err)?.len()
                + 1024;
            if cost > 8 * 1024 * 1024 {
                return Err(err(
                    "bounded protocol/native evidence exceeded reserved transition capacity",
                ));
            }
            charged += cost;
            state = candidate;
            game.steps.push(step);
            if let Some(reason) = state.stop_reason().or_else(|| stop_for(decision)) {
                game.stop = reason;
                break;
            }
        }
        for bridge in &bridges {
            bridge.shutdown();
        }
        let cleanup_deadline = Instant::now() + Duration::from_millis(config.cleanup_ms + 100);
        while bridges.iter().any(|b| !b.shutdown_acknowledged())
            && Instant::now() < cleanup_deadline
        {
            thread::sleep(Duration::from_millis(1));
        }
        for (i, b) in bridges.iter().enumerate() {
            if !b.shutdown_acknowledged() {
                game.cleanup.push(format!(
                    "participant{i} supervisor shutdown unacknowledged; cleanup not proven"
                ));
            }
        }
        game.final_root = state.root().map_err(err)?;
        game.native_outcome = state.outcome();
        game.score_first_twice = state.score_first_twice();
        report.games.push(game);
    }
    report.validate()?;
    if serde_json::to_vec(&report).map_err(err)?.len() > MAX_BYTES {
        return Err(err("aggregate evidence encoding exceeds32MiB"));
    }
    Ok(report)
}
