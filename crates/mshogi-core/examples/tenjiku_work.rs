//! Reproducible native measurements, separate from playing strength and product latency.
//!
//! `cargo run -p mshogi-core --release --example tenjiku_work -- adams 128 play`
//! Modes: play, controls, replay, undo. Rules work is deterministic accounting;
//! timings are observations on this invocation. No outer watchdog is simulated.
use mshogi_core::tenjiku::*;
use serde_json::{Value, json};
use std::{error::Error, mem::size_of, time::Instant};

fn submit(game: &mut Game, actor: Side, command: GameCommand) -> Result<GameUndo, GameError> {
    game.submit(GameRequest {
        expected: game.context(),
        actor,
        command,
    })
}
fn record(game: &Game) -> Result<GameRecord, RecordError> {
    GameRecord::from_game(game)
}
fn micros(start: Instant) -> u128 {
    start.elapsed().as_micros()
}

fn effect_probe(profile: Profile) -> Result<Value, Box<dyn Error>> {
    let sq = |f, r| Square::new(f, r).unwrap();
    let mut generation = vec![];
    for base in [Base::FiD, Base::VGn, Base::Ln, Base::LHk, Base::FEg] {
        let local = LocalPosition::admit(Position::from_diagram(
            profile,
            &[(sq(8, 8), Piece::new(0, Side::Sente, base, false)?)],
            Side::Sente,
            RootMetadata::diagram(0),
        )?)?;
        let start = Instant::now();
        let query = local.geometry_actions_from_metered(sq(8, 8), || false)?;
        let generation_us = micros(start);
        let start = Instant::now();
        let bytes = serde_json::to_vec(&query.value)?.len();
        generation.push(json!({"original":base,"actions":query.value.len(),"generation_us":generation_us,"work_units":query.work_units,"action_bytes":bytes,"serialization_us":micros(start)}));
    }
    // One active burn destroys seven adjacent enemy bystanders; the mover's
    // origin is deliberately outside that batch. This is a workload observation,
    // not a source of expected rule facts for conformance.
    let mut entries = vec![(sq(8, 11), Piece::new(0, Side::Sente, Base::FiD, false)?)];
    for (i, (f, r)) in [(7, 7), (8, 7), (9, 7), (7, 8), (9, 8), (7, 9), (9, 9)]
        .into_iter()
        .enumerate()
    {
        entries.push((
            sq(f, r),
            Piece::new(i as u16 + 1, Side::Gote, Base::P, false)?,
        ));
    }
    let local = LocalPosition::admit(Position::from_diagram(
        profile,
        &entries,
        Side::Sente,
        RootMetadata::diagram(0),
    )?)?;
    let action = Action {
        route: vec![sq(8, 10), sq(8, 9), sq(8, 8)],
        ..Action::single(sq(8, 11), sq(8, 8))
    };
    let start = Instant::now();
    let (_, effects) = local.simulate(&action)?;
    let effect_us = micros(start);
    let start = Instant::now();
    let effect_bytes = serde_json::to_vec(&effects)?.len();
    Ok(
        json!({"empty_center_generation":generation,"active_burn":{"captured_or_burned":effects.captures().count(),"resolution_us":effect_us,"work_units":effects.work_units,"effect_bytes":effect_bytes,"serialization_us":micros(start)}}),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let profile = match args.first().map(String::as_str).unwrap_or("adams") {
        "adams" | "mshogi.tenjiku.adams1999-modern.v1" => Profile::AdamsModern,
        "muller" | "mshogi.tenjiku.muller-completed.v1" => Profile::MullerCompleted,
        _ => return Err("profile must be adams or muller (or the exact ID)".into()),
    };
    let requested = args
        .get(1)
        .map(|s| s.parse::<usize>())
        .transpose()?
        .unwrap_or(32);
    let mode = args.get(2).map(String::as_str).unwrap_or("play");
    if !matches!(mode, "play" | "controls" | "replay" | "undo") || requested > MAX_GAME_EVENTS + 1 {
        return Err("mode play|controls|replay|undo; length <= MAX_GAME_EVENTS+1".into());
    }
    let root_start = Instant::now();
    let mut game = Game::standard(profile);
    let root_us = micros(root_start);
    let root_record = record(&game)?;
    let build_start = Instant::now();
    let mut refusal = None;
    let mut generator_attempts = 0usize;
    let mut seed = 0x87_2026u64;
    for ply in 0..requested {
        if mode == "controls" {
            let (actor, command) = if ply % 2 == 0 {
                (Side::Sente, GameCommand::OfferDraw)
            } else {
                (Side::Gote, GameCommand::DeclineDraw)
            };
            let before = record(&game)?;
            let context = game.context();
            match submit(&mut game, actor, command) {
                Ok(_) => {}
                Err(error) => {
                    let preserved = record(&game)? == before && game.context() == context;
                    if !preserved {
                        return Err("control refusal changed accepted state".into());
                    }
                    refusal = Some(
                        json!({"error":error.to_string(),"accepted_record_and_authority_preserved":preserved}),
                    );
                    break;
                }
            }
        } else {
            // Prefer quiet progress with native effects, keeping the source156-piece
            // standard game material intact. Repetition admission remains Game-owned.
            let mut choices = game.local_position().legal_actions();
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
            choices.sort_by_key(|a| {
                let p = game.position().piece(a.from).expect("native action origin");
                let to = a.to().unwrap();
                let forward = match p.side {
                    Side::Sente => to.rank() < a.from.rank(),
                    Side::Gote => to.rank() > a.from.rank(),
                };
                let mix =
                    (u64::from(a.from.index() as u16) * 257 + u64::from(to.index() as u16)) ^ seed;
                (
                    !forward,
                    a.promote,
                    p.is_royal(),
                    p.base != Base::P,
                    mix,
                    a.clone(),
                )
            });
            let mut accepted = false;
            for action in choices {
                if action.promote {
                    continue;
                }
                generator_attempts += 1;
                let (_, effects) = game.local_position().simulate(&action)?;
                if effects.mover_after.is_none() || effects.captures().next().is_some() {
                    continue;
                }
                match submit(&mut game, effects.actor, GameCommand::Play { action }) {
                    Ok(_) => {
                        accepted = true;
                        break;
                    }
                    Err(GameError::Repetition) => continue,
                    Err(error) => {
                        refusal = Some(json!({"error":error.to_string()}));
                        break;
                    }
                }
            }
            if !accepted {
                if refusal.is_none() {
                    refusal =
                        Some(json!({"error":"no quiet nonpromoting native continuation selected"}));
                }
                break;
            }
        }
    }
    let build_us = micros(build_start);
    let encode_start = Instant::now();
    let native_record = record(&game)?;
    let encode_us = micros(encode_start);
    let before_queries = native_record.clone();
    let before_query_context = game.context();
    let query_start = Instant::now();
    let local_choices = game.local_position().legal_actions_metered(|| false)?;
    let local_query_us = micros(query_start);
    let origins = game.position().pieces().map(|(s, _)| s).collect::<Vec<_>>();
    let mut inspection_rows = vec![];
    let mut inspection_us = 0u128;
    let mut action_serialization_us = 0u128;
    let mut action_bytes = 0usize;
    let mut inspection_work_units = 0u64;
    for origin in origins {
        let start = Instant::now();
        let inspected = game.local_position().inspection_metered(origin, || false)?;
        let inspect_us = micros(start);
        let actions = inspected
            .value
            .iter()
            .map(|c| &c.action)
            .collect::<Vec<_>>();
        let start = Instant::now();
        let bytes = serde_json::to_vec(&actions)?.len();
        let serialize_us = micros(start);
        inspection_us += inspect_us;
        inspection_work_units += inspected.work_units;
        action_serialization_us += serialize_us;
        action_bytes += bytes;
        inspection_rows.push((inspect_us,json!({"origin":origin,"choices":actions.len(),"inspection_us":inspect_us,"work_units":inspected.work_units,"action_json_bytes":bytes,"serialization_us":serialize_us})));
    }
    inspection_rows.sort_by_key(|r| std::cmp::Reverse(r.0));
    let slowest = inspection_rows
        .iter()
        .take(3)
        .map(|r| r.1.clone())
        .collect::<Vec<_>>();
    if record(&game)? != before_queries || game.context() != before_query_context {
        return Err("read-only queries changed accepted record".into());
    }
    let replay_start = Instant::now();
    let ordinary = native_record.to_game()?;
    let replay_us = micros(replay_start);
    let replay_exact = record(&ordinary)? == native_record
        && ordinary.context().same_semantics(&game.context())
        && ordinary.context() != game.context();
    if !replay_exact {
        return Err("ordinary replay failed semantic/fresh-authority comparison".into());
    }
    let mut undo_report = Value::Null;
    if mode == "undo" {
        let start = Instant::now();
        let (mut restored, mut handles) = native_record.to_game_with_undo()?;
        let retain_us = micros(start);
        let count = handles.len();
        let capacity = handles.capacity();
        let start = Instant::now();
        while let Some(mut handle) = handles.pop() {
            restored.undo(&mut handle)?;
        }
        let undo_us = micros(start);
        let root_exact = record(&restored)? == root_record;
        if !root_exact {
            return Err("explicit undo did not restore root".into());
        }
        undo_report = json!({"replay_with_undo_us":retain_us,"handles":count,"vector_capacity":capacity,"inline_handle_bytes":size_of::<GameUndo>(),"inline_vector_allocation_bytes":capacity*size_of::<GameUndo>(),"excludes_game_owned_history_and_allocator_overhead":true,"undo_us":undo_us,"restored_root_exact":root_exact});
    }
    let cancel_start = Instant::now();
    let mut replay_polls = 0u64;
    let replay_stopped = native_record.to_game_with_stop(&mut || {
        replay_polls += 1;
        replay_polls >= 8
    });
    let cancel_replay_us = micros(cancel_start);
    let replay_cancelled = matches!(replay_stopped, Err(RecordError::Stopped));
    if !replay_cancelled {
        return Err("requested replay stop was not observed".into());
    }
    let origin = game
        .position()
        .pieces()
        .next()
        .map(|(s, _)| s)
        .ok_or("no inspection origin")?;
    let cancel_start = Instant::now();
    let mut query_polls = 0u64;
    let query_stopped = game.local_position().inspection_metered(origin, || {
        query_polls += 1;
        query_polls >= 3
    });
    let cancel_query_us = micros(cancel_start);
    if !matches!(query_stopped, Err(ActionError::Stopped)) {
        return Err("requested inspection stop was not observed".into());
    }
    let mut admission_probe = native_record.to_game()?;
    let old_context = admission_probe.context();
    let mut admission_polls = 0u64;
    let start = Instant::now();
    let stopped = admission_probe.submit_with_stop(
        GameRequest {
            expected: old_context.clone(),
            actor: Side::Sente,
            command: GameCommand::Stop {
                reason: StopReason::Cancelled,
            },
        },
        &mut || {
            admission_polls += 1;
            admission_polls >= 2
        },
    );
    let cancel_admission_us = micros(start);
    let cancelled_atomic = matches!(stopped, Err(GameError::Stopped))
        && record(&admission_probe)? == native_record
        && admission_probe.context() == old_context;
    if !cancelled_atomic {
        return Err("cancelled admission changed accepted state".into());
    }
    let usage = game.capacity_usage()?;
    let mut close = native_record.to_game()?;
    submit(
        &mut close,
        Side::Sente,
        GameCommand::Stop {
            reason: StopReason::ResourceLimit,
        },
    )?;
    submit(&mut close, Side::Gote, GameCommand::Resign)?;
    submit(
        &mut close,
        Side::Gote,
        GameCommand::Stop {
            reason: StopReason::Cancelled,
        },
    )?;
    let closed = record(&close)?;
    let closed_replayed = closed.to_game()?;
    let closing_exact = record(&closed_replayed)? == closed;
    if !closing_exact {
        return Err("actual reserved Stop/Resign path failed replay".into());
    }
    let output = json!({
        "isolated_native_workloads":effect_probe(profile)?,
        "profile":profile.id(),"edition":profile.edition(),"mode":mode,"requested":requested,"accepted_events":game.events().len(),"accepted_board_plies":game.snapshot().board_ply,
        "root_us":root_us,"build_us":build_us,"generator_attempts":generator_attempts,"refusal":refusal,
        "record":{"encoded_bytes":native_record.as_json().len(),"encode_us":encode_us,"deterministic_reconstruction_units":native_record.reconstruction_work()},
        "capacity":{"bytes":usage.encoded_bytes,"events":usage.events,"work_units":usage.replay_work,"reserved_bytes":usage.reserved_bytes,"reserved_events":usage.reserved_events,"reserved_work_units":usage.reserved_work,"limits":{"bytes":MAX_GAME_RECORD_BYTES,"events":MAX_GAME_EVENTS,"work_units":MAX_REPLAY_WORK}},
        "local_choices":{"count":local_choices.value.len(),"query_us":local_query_us,"work_units":local_choices.work_units,"history_filtered":false},
        "per_origin_inspection":{"origins":inspection_rows.len(),"total_us":inspection_us,"total_work_units":inspection_work_units,"action_serialization_us":action_serialization_us,"total_action_json_bytes":action_bytes,"slowest_three":slowest},
        "ordinary_replay":{"us":replay_us,"exact_semantics_and_fresh_authority":replay_exact,"returned_undo_handles":0,"retention_policy":"ordinary replay requests no private undo handles; source-defined, not an RSS measurement"},
        "explicit_undo":undo_report,
        "cooperative_cancellation":{"replay_us":cancel_replay_us,"replay_polls":replay_polls,"inspection_us":cancel_query_us,"inspection_polls":query_polls,"admission_us":cancel_admission_us,"admission_polls":admission_polls,"atomic":cancelled_atomic,"no_external_watchdog_or_product_latency_claim":true},
        "actual_closing_path":{"stop_then_resign_then_stop":true,"events":close.events().len(),"encoded_bytes":closed.as_json().len(),"replayed_exactly":closing_exact},
        "timing_unit":"microseconds measured by Instant; deterministic units are not CPU time or strength"
    });
    println!("{output}");
    Ok(())
}
