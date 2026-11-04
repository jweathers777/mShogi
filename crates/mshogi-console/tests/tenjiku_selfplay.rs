//! Reproducible bounded real-subprocess Tenjiku self-play, not strength measurement.
use mshogi_core::{native::CanonicalRecord, tenjiku as t};
use mshogi_engine::*;
use std::{
    io::{Read, Write},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

fn request(game: &t::Game, id: u64) -> ExecutionRequest {
    ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: id,
            session_version: id,
            process_generation: id,
            game: CanonicalRecord::tenjiku(t::GameRecord::from_game(game).unwrap()),
            limits: WorkLimits {
                depth: 1,
                nodes: 1024,
            },
        },
    }
}
fn actual(request: &ExecutionRequest) -> ExecutionResponse {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mshogi"))
        .arg("internal-search")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    writeln!(
        child.stdin.take().unwrap(),
        "{}",
        serde_json::to_string(request).unwrap()
    )
    .unwrap();
    // Drain both pipes while the process runs (especially Windows' small pipe
    // buffers); retain only supported bounded transport/diagnostic payloads.
    let stdout = child.stdout.take().unwrap();
    let stdout = thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout
            .take((MAX_RESPONSE_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .unwrap();
        bytes
    });
    let stderr = child.stderr.take().unwrap();
    let stderr = thread::spawn(move || {
        let mut bytes = Vec::new();
        stderr
            .take((MAX_STDERR_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .unwrap();
        bytes
    });
    let deadline = Instant::now() + Duration::from_secs(20);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() >= deadline {
            child.kill().unwrap();
            let _ = child.wait();
            panic!("bounded Tenjiku child exceeded independent20-second watchdog");
        }
        thread::sleep(Duration::from_millis(1));
    }
    let status = child.wait().unwrap();
    let stdout = stdout.join().unwrap();
    let stderr = stderr.join().unwrap();
    assert!(status.success(), "{}", String::from_utf8_lossy(&stderr));
    assert!(stderr.is_empty());
    assert!(stdout.len() <= MAX_RESPONSE_BYTES);
    let response: ExecutionResponse = serde_json::from_slice(&stdout).unwrap();
    assert!(response.matches(request));
    response
}

#[derive(Debug, Eq, PartialEq)]
struct Trace {
    record: String,
    actions: Vec<t::Action>,
    observations: Vec<ExecutionObservation>,
}
fn bounded(mut game: t::Game, maximum_plies: usize) -> Trace {
    let mut actions = vec![];
    let mut observations = vec![];
    for id in 1..=maximum_plies as u64 {
        if game.conclusion().is_some() {
            break;
        }
        let request = request(&game, id);
        let response = actual(&request);
        let ExecutionResult::Tenjiku { result } = &response.observation.result else {
            panic!("expected native Tenjiku search")
        };
        result.validate(&game).unwrap();
        let TenjikuResult::Choice { action, telemetry } = result else {
            panic!("representative depth1 must complete: {result:?}")
        };
        assert_eq!(telemetry.completed_depth, 1);
        assert!(telemetry.score.is_some());
        assert!(telemetry.nodes <= 1024);
        assert_eq!(telemetry.perspective, game.position().side_to_move());
        assert!(game.legal_actions().contains(action));
        actions.push(action.clone());
        game.submit(t::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: t::GameCommand::Play {
                action: action.clone(),
            },
        })
        .unwrap();
        observations.push(response.observation);
    }
    if game.conclusion().is_none() {
        game.submit(t::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: t::GameCommand::Stop {
                reason: t::StopReason::WorkLimit,
            },
        })
        .unwrap();
        assert!(game.conclusion().is_none());
        assert_eq!(game.snapshot().incomplete, Some(t::StopReason::WorkLimit));
    } else {
        // The next actual process sees the exact native conclusion, not a
        // synthetic zero-move/heuristic result or an already retired request.
        let response = actual(&request(&game, maximum_plies as u64 + 1));
        let ExecutionResult::Tenjiku {
            result:
                TenjikuResult::Finished {
                    conclusion,
                    telemetry,
                },
        } = &response.observation.result
        else {
            panic!("expected native terminal root")
        };
        assert_eq!(Some(*conclusion), game.conclusion());
        assert!(telemetry.principal_variation.is_empty());
        observations.push(response.observation);
    }
    let native = t::GameRecord::from_game(&game).unwrap();
    let record = CanonicalRecord::tenjiku(native.clone()).to_json().unwrap();
    let CanonicalRecord::Tenjiku(restored) = CanonicalRecord::from_json(&record).unwrap() else {
        panic!()
    };
    assert_eq!(*restored, native);
    assert_eq!(
        t::GameRecord::from_game(&restored.to_game().unwrap()).unwrap(),
        native
    );
    Trace {
        record,
        actions,
        observations,
    }
}

// Debug exercises actual subprocess/replay behavior on an independent corner
// diagram. Mandatory release execution on all three platforms exercises the
// full 156-piece standard root; neither path is skipped or called the other.
fn smoke_root(profile: t::Profile) -> t::Game {
    if !cfg!(debug_assertions) {
        return t::Game::standard(profile);
    }
    let pieces = [
        (16, 16, t::Side::Sente, t::Base::K),
        (16, 14, t::Side::Sente, t::Base::P),
        (1, 1, t::Side::Gote, t::Base::K),
        (1, 3, t::Side::Gote, t::Base::P),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (f, r, side, base))| {
        (
            square(f, r),
            t::Piece::new(id as u16, side, base, false).unwrap(),
        )
    })
    .collect::<Vec<_>>();
    t::Game::from_root(
        t::Position::from_diagram(
            profile,
            &pieces,
            t::Side::Sente,
            t::RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap()
}
#[test]
fn actual_selfplay_is_deterministic_bounded_and_unscored_at_work_stop() {
    for profile in t::Profile::ALL {
        let first = bounded(smoke_root(profile), 4);
        let second = bounded(smoke_root(profile), 4);
        assert_eq!(first, second);
        assert_eq!(first.actions.len(), 4);
        let CanonicalRecord::Tenjiku(record) = CanonicalRecord::from_json(&first.record).unwrap()
        else {
            panic!()
        };
        assert_eq!(record.profile(), profile);
        assert_eq!(
            record
                .to_game()
                .unwrap()
                .native_root()
                .board()
                .iter()
                .flatten()
                .count(),
            if cfg!(debug_assertions) { 4 } else { 156 }
        );
        assert_eq!(
            record.to_game().unwrap().position().root().kind(),
            if cfg!(debug_assertions) {
                t::RootKind::Diagram
            } else {
                t::RootKind::Standard
            }
        );
        assert_eq!(record.known_board_actions(), 4);
        assert_eq!(record.event_count(), 5);
        assert!(record.conclusion().is_none());
        println!(
            "TENJIKU-SELFPLAY-SMOKE {}",
            serde_json::json!({"profile":profile.id(),"root":if cfg!(debug_assertions){"independent_corner_diagram"}else{"standard"},"command":"mshogi internal-search","limits":{"depth":1,"nodes":1024},"maximum_plies":4,"record":first.record,"observations":first.observations})
        );
    }
}
fn square(f: u8, r: u8) -> t::Square {
    t::Square::new(f, r).unwrap()
}
fn royal_root(profile: t::Profile) -> t::Game {
    // Source-derived Lion double capture: the two opponent royals are adjacent
    // consecutive landings. Capturing both ends the actual game in one action;
    // capturing just one cannot establish RoyalCapture while the other survives.
    let pieces = [
        (16, 16, t::Side::Sente, t::Base::K, false),
        (16, 14, t::Side::Sente, t::Base::P, false),
        (8, 8, t::Side::Sente, t::Base::Ln, false),
        (8, 7, t::Side::Gote, t::Base::K, false),
        (8, 6, t::Side::Gote, t::Base::DE, true),
        (1, 3, t::Side::Gote, t::Base::P, false),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (f, r, side, base, promoted))| {
        (
            square(f, r),
            t::Piece::new(id as u16, side, base, promoted).unwrap(),
        )
    })
    .collect::<Vec<_>>();
    t::Game::from_root(
        t::Position::from_diagram(
            profile,
            &pieces,
            t::Side::Sente,
            t::RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap()
}
#[test]
fn actual_complete_custom_game_captures_both_independently_expected_native_royals() {
    for profile in t::Profile::ALL {
        let first = bounded(royal_root(profile), 4);
        let second = bounded(royal_root(profile), 4);
        assert_eq!(first, second);
        assert_eq!(
            first.actions,
            vec![t::Action {
                kind: t::ActionKind::Travel,
                from: square(8, 8),
                route: vec![square(8, 7), square(8, 6)],
                promote: false,
                target: None
            }]
        );
        let CanonicalRecord::Tenjiku(record) = CanonicalRecord::from_json(&first.record).unwrap()
        else {
            panic!()
        };
        assert_eq!(record.profile(), profile);
        assert_eq!(
            record.conclusion(),
            Some(t::Conclusion::RoyalCapture {
                winner: t::Side::Sente
            })
        );
        assert_eq!(record.known_board_actions(), 1);
        assert_eq!(record.event_count(), 1);
        assert!(record.to_game().unwrap().snapshot().incomplete.is_none());
        assert_eq!(first.observations.len(), 2);
        println!(
            "TENJIKU-SELFPLAY-COMPLETE {}",
            serde_json::json!({"profile":profile.id(),"command":"mshogi internal-search","limits":{"depth":1,"nodes":1024},"maximum_plies":4,"record":first.record,"observations":first.observations})
        );
    }
}
