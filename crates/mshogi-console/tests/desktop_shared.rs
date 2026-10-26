//! Actual shared desktop controller through the bundled executable's internal entry point.
use mshogi_core::{chess, native::*, shogi as s};
use mshogi_desktop_session::*;
use std::time::{Duration, Instant};
fn command(c: &mut DesktopController, operation: SessionOperation) -> SharedSnapshot {
    let snapshot = c.shared_snapshot();
    let (_, receipt) = c
        .shared_command_with_receipt(SharedDesktopCommand {
            expected: snapshot.token,
            game: snapshot.game,
            operation,
        })
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let snapshot = c.shared_snapshot();
        if let Some(result) = receipt.completion() {
            result.unwrap();
            return snapshot;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn wait(c: &mut DesktopController, done: impl Fn(&SharedSnapshot) -> bool) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        let snapshot = c.shared_snapshot();
        if done(&snapshot) {
            return snapshot;
        }
        assert!(Instant::now() < deadline, "{snapshot:?}");
        std::thread::sleep(Duration::from_millis(2));
    }
}
fn analyze(c: &mut DesktopController) -> SharedSnapshot {
    command(
        c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves {
                depth: 1,
                nodes: 1000,
            },
        },
    );
    wait(c, |s| {
        s.analysis.is_some()
            || s.chess
                .as_ref()
                .is_some_and(|s| s.analysis.result.is_some())
    })
}
fn root(fen: &str) -> s::Game {
    s::Game::from_root(
        s::GameRoot::import(
            s::Position::from_sfen(fen).unwrap(),
            s::RootCategory::ReducedMaterialDiagram,
            s::CounterSource::Supplied,
        )
        .unwrap(),
    )
}
#[test]
fn real_chess_shogi_chess_execution_uses_one_supervisor_and_fresh_ids() {
    let mut c = DesktopController::new(env!("CARGO_BIN_EXE_mshogi").into()).unwrap();
    let chess = analyze(&mut c);
    let old = chess.chess.unwrap().analysis.context.unwrap();
    command(
        &mut c,
        SessionOperation::Replace {
            record: CanonicalRecord::shogi(s::GameRecord::from_game(&s::Game::new())),
            human: NativeSide::Shogi(s::Side::Gote),
        },
    );
    command(&mut c, SessionOperation::Resume {});
    let played = wait(&mut c, |s| s.event_index == 1);
    assert_eq!(played.events[0].label, "Sente: 1g1f");
    assert_eq!(played.turn.label, "Gote");
    assert!(played.observations[0].response.context.request_id > old.request_id);
    let before = played.context;
    let observed = analyze(&mut c);
    assert_eq!(observed.context, before);
    assert!(observed.analysis.is_some());
    let last = observed.analysis.unwrap().response;
    command(
        &mut c,
        SessionOperation::Replace {
            record: CanonicalRecord::chess(
                chess::AdjudicatedGame::new(chess::ClaimPolicy::ExplicitOnly).record(),
            ),
            human: NativeSide::Chess(chess::Color::White),
        },
    );
    assert!(!c.accept_shared(last.clone()));
    let chess = analyze(&mut c);
    let fresh = chess.chess.unwrap().analysis.context.unwrap();
    assert!(fresh.request_id > last.context.request_id);
    assert!(fresh.process_generation > last.context.process_generation);
    assert!(!c.accept_shared(last));
    c.begin_shutdown();
}
#[test]
fn actual_engine_declaration_is_a_terminal_command_and_analysis_cannot_apply_it() {
    let mut c = DesktopController::new(env!("CARGO_BIN_EXE_mshogi").into()).unwrap();
    let game = root("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1");
    command(
        &mut c,
        SessionOperation::Replace {
            record: CanonicalRecord::shogi(s::GameRecord::from_game(&game)),
            human: NativeSide::Shogi(s::Side::Gote),
        },
    );
    let observed = analyze(&mut c);
    assert_eq!(observed.event_index, 0);
    assert!(!observed.status.concluded);
    assert!(observed.analysis.is_some());
    command(&mut c, SessionOperation::Resume {});
    let ended = wait(&mut c, |s| s.status.concluded);
    assert_eq!(ended.event_index, 1);
    assert_eq!(ended.events[0].board_ply, 0);
    assert!(ended.status.label.contains("entering-king declaration"));
    command(&mut c, SessionOperation::Inspect { event_index: 0 });
    let prefix = analyze(&mut c);
    let response = &prefix.analysis.as_ref().unwrap().response;
    assert_eq!(response.context.game.event_count(), 0);
    assert_eq!(prefix.event_index, 0);
    assert!(!prefix.status.concluded);
    command(&mut c, SessionOperation::ReturnLive {});
    assert!(c.shared_snapshot().status.concluded);
    c.begin_shutdown();
}
