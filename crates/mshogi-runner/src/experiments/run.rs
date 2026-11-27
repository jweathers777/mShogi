use super::*;
use mshogi_engine::{BridgeOptions, EngineBridge, ExecutionRequest, PROTOCOL_VERSION};
use std::{
    thread,
    time::{Duration, Instant},
};

/// Run actual bounded the internal protocol subprocesses over full-history owners.
pub fn run(config: &Config) -> Result<ValidatedReport, RunnerError> {
    run_with_stop(config, &mut || false)
}
/// Cooperative experiment cancellation; never a scored game result.
pub fn run_with_stop(
    config: &Config,
    stopped: &mut impl FnMut() -> bool,
) -> Result<ValidatedReport, RunnerError> {
    config.validate()?;
    let identities: Vec<_> = config
        .players
        .iter()
        .map(|p| EngineIdentity {
            executable: crate::common::identity(&p.executable),
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
    // Size the report with an accepted opening for every scheduled game. Allow
    // for an unacknowledged shutdown from every supervisor until cleanup finishes.
    for pairing in config.schedule() {
        let root = &config.openings[pairing.opening].root;
        let state = game::MatchState::from_root(root).map_err(err)?;
        report.games.push(Game {
            pairing,
            steps: vec![],
            final_root: state.root().map_err(err)?,
            outcome: state.outcome(),
            score_first_twice: state.score_first_twice(),
            stop: state
                .stop_reason()
                .unwrap_or_else(|| "artifact_limit".into()),
            cleanup: (0..config.players.len()).map(cleanup_diagnostic).collect(),
        });
    }
    let mut charged = crate::common::encoded_len(&report)?;
    if charged > MAX_BYTES {
        return Err(err(
            "scheduled opening evidence exceeds the 128 MiB report budget",
        ));
    }
    let started = Instant::now();
    let wall_budget = Duration::from_millis(config.wall_budget_ms);
    let mut id = 0u64;
    for game in &mut report.games {
        let mut state = game::MatchState::from_root(&game.final_root).map_err(err)?;
        let mut ending_bytes = ending_size(
            &game.final_root,
            &game.outcome,
            game.score_first_twice,
            &game.stop,
        )?;
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
        let mut remaining: Vec<u64> = (0..config.players.len())
            .map(|p| config.player_clock(p).map_or(0, |c| c.base_ms))
            .collect();
        let mut moved = vec![false; config.players.len()];
        loop {
            if let Some(reason) = state.stop_reason() {
                game.stop = reason;
                break;
            }
            if stopped() {
                game.stop = "cancelled".into();
                break;
            }
            if started.elapsed() >= wall_budget {
                game.stop = "wall_budget".into();
                break;
            }
            if game.steps.len() >= config.max_steps {
                game.stop = "step_limit".into();
                break;
            }
            let before = state.root().map_err(err)?;
            let player = game.pairing.player(&state);
            id += 1;
            let request = ExecutionRequest {
                protocol: PROTOCOL_VERSION,
                context: ExecutionContext {
                    purpose: ExecutionPurpose::EngineTurn {},
                    request_id: id,
                    session_version: id,
                    game: before.record.clone(),
                    limits: clock_work(
                        config.players[player].work(before.record.profile())?,
                        config.player_clock(player),
                        remaining[player],
                        !moved[player],
                    ),
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
                if started.elapsed() >= wall_budget {
                    failure = Some("wall_budget".to_owned());
                    break None;
                }
                if config.clock.is_some()
                    && began.elapsed() >= Duration::from_millis(remaining[player])
                {
                    failure = Some("clock_limit".to_owned());
                    break None;
                }
                if let Some(response) = bridges[player].poll() {
                    break Some(response.observation);
                }
                thread::sleep(Duration::from_millis(1));
            };
            let elapsed_ns = crate::common::elapsed_ns(began);
            if observation.is_none() {
                bridges[player].cancel();
            }
            let mut candidate = state.clone();
            let exhausted =
                config.clock.is_some() && elapsed_ns.div_ceil(1_000_000) >= remaining[player];
            let (decision, failure) = decide_step(
                &mut candidate,
                &request.context,
                observation.as_ref(),
                failure.as_deref(),
                exhausted,
            )?;
            let clock = config.player_clock(player).map(|c| {
                clock_receipt(
                    c,
                    remaining[player],
                    elapsed_ns,
                    matches!(decision, game::Admission::Applied),
                )
            });
            if let Some(r) = &clock {
                remaining[player] = r.after_ms;
            }
            moved[player] = true;
            let step = Step {
                clock,
                player,
                elapsed_ns,
                request: request.context,
                observation,
                failure,
                applied: matches!(decision, game::Admission::Applied),
            };
            let after = candidate.root().map_err(err)?;
            let outcome = candidate.outcome();
            let score = candidate.score_first_twice();
            let stop = candidate.stop_reason().or_else(|| stop_for(decision));
            let reserved_stop = stop.as_deref().unwrap_or("artifact_limit");
            let new_ending = ending_size(&after, &outcome, score, reserved_stop)?;
            let next_charge = charged - ending_bytes
                + new_ending
                + crate::common::encoded_len(&encoded_step(&step)?)?
                + usize::from(!game.steps.is_empty());
            if next_charge > MAX_BYTES {
                game.stop = "artifact_limit".into();
                break;
            }
            charged = next_charge;
            ending_bytes = new_ending;
            state = candidate;
            game.steps.push(step);
            game.final_root = after;
            game.outcome = outcome;
            game.score_first_twice = score;
            if let Some(reason) = stop {
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
        let reserved_cleanup = crate::common::encoded_len(&game.cleanup)?;
        game.cleanup = bridges
            .iter()
            .enumerate()
            .filter(|(_, b)| !b.shutdown_acknowledged())
            .map(|(i, _)| cleanup_diagnostic(i))
            .collect();
        charged = charged - ending_bytes - reserved_cleanup
            + ending_size(
                &game.final_root,
                &game.outcome,
                game.score_first_twice,
                &game.stop,
            )?
            + crate::common::encoded_len(&game.cleanup)?;
    }
    report.into_validated()
}

fn cleanup_diagnostic(player: usize) -> String {
    format!("participant {player} supervisor shutdown unacknowledged; cleanup not proven")
}

// Constant tuple punctuation cancels when replacing an ending in the report.
fn ending_size(
    root: &Root,
    outcome: &Option<serde_json::Value>,
    score: Option<u8>,
    stop: &str,
) -> Result<usize, RunnerError> {
    crate::common::encoded_len(&(root, outcome, score, stop))
}
