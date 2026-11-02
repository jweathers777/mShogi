//! Reproducible bounded real-subprocess Tori self-play, not strength measurement.
use mshogi_core::{native::CanonicalRecord, tori as t};
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
            game: CanonicalRecord::tori(t::GameRecord::from_game(game).unwrap()),
            limits: WorkLimits {
                depth: 1,
                nodes: 256,
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
            panic!("bounded Tori child exceeded independent20-second watchdog");
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
        let ExecutionResult::Tori { result } = &response.observation.result else {
            panic!("expected native Tori search")
        };
        result.validate(&game).unwrap();
        let ToriResult::Choice { action, telemetry } = result else {
            panic!("representative depth1 must complete: {result:?}")
        };
        assert_eq!(telemetry.completed_depth, 1);
        assert!(telemetry.score.is_some());
        assert!(telemetry.nodes <= 256);
        assert_eq!(telemetry.perspective, game.position().side_to_move());
        assert!(game.legal_actions().contains(action));
        actions.push(*action);
        game.submit(t::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: t::GameCommand::Play { action: *action },
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
        let ExecutionResult::Tori {
            result:
                ToriResult::Finished {
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
    let record = CanonicalRecord::tori(native.clone()).to_json().unwrap();
    let CanonicalRecord::Tori(restored) = CanonicalRecord::from_json(&record).unwrap() else {
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

#[test]
fn actual_standard_selfplay_is_deterministic_bounded_and_unscored_at_work_stop() {
    let first = bounded(t::Game::standard(), 8);
    let second = bounded(t::Game::standard(), 8);
    assert_eq!(first, second);
    assert_eq!(first.actions.len(), 8);
    let CanonicalRecord::Tori(record) = CanonicalRecord::from_json(&first.record).unwrap() else {
        panic!()
    };
    assert_eq!(record.known_board_actions(), 8);
    assert_eq!(record.event_count(), 9);
    assert!(record.conclusion().is_none());
    println!(
        "TORI-SELFPLAY-STANDARD {}",
        serde_json::json!({"command":"mshogi internal-search","limits":{"depth":1,"nodes":256},"maximum_plies":8,"record":first.record,"observations":first.observations})
    );
}

fn mate_root() -> t::Game {
    // TORI-O-MATE's independently reviewed terminal diagram, one predecessor
    // earlier. Ea4c-4b checks4a; Pt4d protects4b; native Gote3a/5a/3b/5b
    // blockers leave no king escape or defense. No evaluator expectation oracle.
    let pieces = [
        ("7g", t::Side::Sente, t::Face::Phoenix),
        ("4a", t::Side::Gote, t::Face::Phoenix),
        ("4d", t::Side::Sente, t::Face::Pheasant),
        ("3a", t::Side::Gote, t::Face::Swallow),
        ("5a", t::Side::Gote, t::Face::Swallow),
        ("3b", t::Side::Gote, t::Face::Swallow),
        ("5b", t::Side::Gote, t::Face::Swallow),
        ("4c", t::Side::Sente, t::Face::Eagle),
    ];
    t::Game::from_root(
        t::Position::from_diagram(
            &pieces.map(|(s, side, face)| (s.parse().unwrap(), t::Piece { side, face })),
            [t::Hand::empty(); 2],
            t::Side::Sente,
            t::RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap()
}
#[test]
fn actual_complete_custom_game_reaches_independently_expected_native_checkmate() {
    let first = bounded(mate_root(), 4);
    let second = bounded(mate_root(), 4);
    assert_eq!(first, second);
    assert_eq!(
        first.actions,
        [t::Action::Move {
            from: "4c".parse().unwrap(),
            to: "4b".parse().unwrap(),
            result_face: t::Face::Eagle
        }]
    );
    let CanonicalRecord::Tori(record) = CanonicalRecord::from_json(&first.record).unwrap() else {
        panic!()
    };
    assert_eq!(
        record.conclusion(),
        Some(t::Conclusion::Checkmate {
            winner: t::Side::Sente
        })
    );
    assert_eq!(record.known_board_actions(), 1);
    assert_eq!(record.event_count(), 1);
    assert!(record.to_game().unwrap().snapshot().incomplete.is_none());
    println!(
        "TORI-SELFPLAY-COMPLETE {}",
        serde_json::json!({"command":"mshogi internal-search","limits":{"depth":1,"nodes":256},"maximum_plies":4,"record":first.record,"observations":first.observations})
    );
}
