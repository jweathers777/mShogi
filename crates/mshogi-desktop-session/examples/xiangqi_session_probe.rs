//! Development-only native consumer probe. This is not product desktop UI evidence.
use mshogi_core::{native::CanonicalRecord, xiangqi as x};
use mshogi_desktop_session::{
    AnalysisOperation, DesktopController, EnginePhase, GameKind, NativeSide, SessionOperation,
    SharedDesktopCommand, SharedSnapshot,
};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};
fn send(c: &mut DesktopController, op: SessionOperation) -> SharedSnapshot {
    let game = c.shared_snapshot().game;
    c.shared_command(SharedDesktopCommand {
        expected: c.shared_token(),
        game,
        operation: op,
    })
    .unwrap()
}
fn wait(c: &mut DesktopController) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let s = c.shared_snapshot();
        if s.phase != EnginePhase::Thinking {
            return s;
        }
        assert!(Instant::now() < deadline, "actual child deadline");
        std::thread::sleep(Duration::from_millis(5));
    }
}
fn main() {
    let path = PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("bundled native engine executable path"),
    );
    let mut c = DesktopController::new(path).unwrap();
    let game = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    let record = CanonicalRecord::xiangqi(x::GameRecord::from_game(&game).unwrap());
    send(
        &mut c,
        SessionOperation::Replace {
            record,
            human: NativeSide::Xiangqi(x::Side::Black),
        },
    );
    send(&mut c, SessionOperation::Resume {});
    let played = wait(&mut c);
    assert_eq!(played.game, GameKind::Xiangqi);
    assert_eq!(played.board_ply, 1);
    assert_eq!(played.observations.len(), 1);
    assert_eq!(played.turn.label, "Black");
    assert_eq!(played.play_limits.depth, 1);
    assert_eq!(played.play_limits.nodes, 128);
    let live = played.context;
    send(
        &mut c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves {
                depth: 1,
                nodes: 128,
            },
        },
    );
    let analyzed = wait(&mut c);
    assert_eq!(analyzed.context, live);
    assert!(analyzed.analysis.is_some());
    send(
        &mut c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves {
                depth: 3,
                nodes: 512,
            },
        },
    );
    let cancelled = send(&mut c, SessionOperation::Cancel {});
    assert_eq!(cancelled.phase, EnginePhase::Cancelled);
    assert_eq!(cancelled.context, live);
    let history = send(&mut c, SessionOperation::Inspect { event_index: 0 });
    assert!(history.historical);
    assert_eq!(history.board_ply, 0);
    send(
        &mut c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves {
                depth: 1,
                nodes: 128,
            },
        },
    );
    let historical = wait(&mut c);
    assert_eq!(historical.board_ply, 0);
    assert!(historical.analysis.is_some());
    let returned = send(&mut c, SessionOperation::ReturnLive {});
    assert_eq!(returned.context, live);
    assert!(returned.paused);
    assert!(returned.analysis.is_none());
    c.begin_shutdown();
    let deadline = Instant::now() + Duration::from_secs(10);
    while !c.shutdown_acknowledged() {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(5));
    }
    println!(
        "PASS: actual supervised Xiangqi engine turn, read-only live/history analysis, cancellation, Return to live, fresh identities and acknowledged process cleanup; native contract only, not GUI evidence."
    );
}
