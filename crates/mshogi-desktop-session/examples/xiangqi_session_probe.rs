//! Development-only native consumer probe. This is not product desktop UI evidence.
use mshogi_core::{native::CanonicalRecord, xiangqi as x};
use mshogi_desktop_session::{
    AnalysisOperation, DesktopController, EnginePhase, GameKind, IoPhase, NativeSide,
    SessionOperation, SharedDesktopCommand, SharedSnapshot,
};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};
fn send(c: &mut DesktopController, op: SessionOperation) -> SharedSnapshot {
    let game = c.shared_snapshot().game;
    let (_, receipt) = c
        .shared_command_with_receipt(SharedDesktopCommand {
            expected: c.shared_token(),
            game,
            operation: op,
        })
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = c.shared_snapshot();
        if let Some(result) = receipt.completion() {
            result.unwrap();
            return snapshot;
        }
        assert!(Instant::now() < deadline, "native archive admission");
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn wait(c: &mut DesktopController) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let s = c.shared_snapshot();
        if s.phase != EnginePhase::Thinking
            && s.capacity.status != mshogi_desktop_session::CapacityPhase::Checking
        {
            return s;
        }
        assert!(Instant::now() < deadline, "actual child deadline");
        std::thread::sleep(Duration::from_millis(5));
    }
}
fn file_wait(c: &mut DesktopController, id: u64) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = c.shared_snapshot();
        assert_eq!(snapshot.io.id, id);
        match snapshot.io.phase {
            IoPhase::Complete => return snapshot,
            IoPhase::Failed => panic!("archive operation failed: {:?}", snapshot.io.message),
            _ => {}
        }
        assert!(Instant::now() < deadline, "actual archive deadline");
        std::thread::sleep(Duration::from_millis(5));
    }
}
fn main() {
    let path = PathBuf::from(
        std::env::args()
            .nth(1)
            .expect("bundled native engine executable path"),
    );
    let mut c = DesktopController::new(path.clone()).unwrap();
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
    let saved = send(&mut c, SessionOperation::Inspect { event_index: 0 });
    let file = std::env::temp_dir().join(format!(
        "msh99-probe-{}-{}.mshogi",
        std::process::id(),
        saved.token.revision
    ));
    let saved_selection = (saved.event_index, saved.board_ply, saved.context.clone());
    let observations = saved.observations;
    let old_floor = observations
        .iter()
        .map(|o| {
            o.response
                .context
                .request_id
                .max(o.response.context.session_version)
        })
        .max()
        .unwrap();
    let id = c.queue_shared_save(file.clone()).unwrap();
    assert!(!file_wait(&mut c, id).document.dirty);
    c.begin_shutdown();
    let deadline = Instant::now() + Duration::from_secs(10);
    while !c.shutdown_acknowledged() {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(5));
    }
    // A genuinely fresh controller starts in Chess; native Open must publish the
    // installed Xiangqi family, preserve exact selection, and launch no work.
    let mut c = DesktopController::new(path).unwrap();
    let token = c.shared_open_token();
    let id = c.queue_shared_open(file.clone(), token).unwrap();
    let reopened = file_wait(&mut c, id);
    assert_eq!(reopened.game, GameKind::Xiangqi);
    assert!(reopened.historical && reopened.paused && reopened.execution_purpose.is_none());
    assert_eq!(
        (
            reopened.event_index,
            reopened.board_ply,
            reopened.context.clone()
        ),
        saved_selection
    );
    assert!(reopened.token.version > old_floor);
    assert_eq!(reopened.observations.len(), observations.len());
    for (actual, saved) in reopened.observations.iter().zip(&observations) {
        assert!(actual.historical);
        assert_eq!(actual.response, saved.response);
    }
    let restored = send(&mut c, SessionOperation::ReturnLive {});
    assert_eq!(restored.context, live);
    assert!(restored.paused);
    let resumed = send(&mut c, SessionOperation::Resume {});
    let action = resumed.choices[0].action.parse().unwrap();
    send(
        &mut c,
        SessionOperation::Xiangqi {
            command: x::GameCommand::Play { action },
        },
    );
    let played = wait(&mut c);
    assert_eq!(played.board_ply, 3);
    assert!(
        played
            .observations
            .last()
            .unwrap()
            .response
            .context
            .request_id
            > old_floor
    );
    c.begin_shutdown();
    let deadline = Instant::now() + Duration::from_secs(10);
    while !c.shutdown_acknowledged() {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(5));
    }
    std::fs::remove_file(file).unwrap();
    println!(
        "PASS: actual supervised Xiangqi play/live-history analysis/cancel, Save, fresh-controller Open with historical selection and stopped work, Return to live, explicit Resume and continued play, fresh identities and acknowledged cleanup; native/process/archive evidence only, not GUI."
    );
}
