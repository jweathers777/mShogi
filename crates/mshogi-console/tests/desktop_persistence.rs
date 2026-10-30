//! Real files and bundled-engine processes through the public desktop controller.
use mshogi_core::chess::Color;
use mshogi_core::native::{CanonicalRecord, NativeSide};
use mshogi_desktop_session::{
    AnalysisOperation, AnalysisPhase, DesktopController, EnginePhase, IoPhase, SharedSnapshot,
};
use mshogi_desktop_session::{
    ChessPresentation, NativeDesktopSettings, SessionOperation, SessionToken, SharedDesktopCommand,
    SharedOpenToken,
};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::{Duration, Instant},
};

static NEXT_DIRECTORY: AtomicU64 = AtomicU64::new(0);
struct Directory(PathBuf);
impl Directory {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "mshogi-desktop-persistence-{}-{}",
            std::process::id(),
            NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}
fn controller() -> DesktopController {
    DesktopController::new(PathBuf::from(env!("CARGO_BIN_EXE_mshogi"))).unwrap()
}
fn until(
    controller: &mut DesktopController,
    ready: impl Fn(&SharedSnapshot) -> bool,
) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = controller.shared_snapshot();
        if ready(&snapshot) {
            return snapshot;
        }
        assert!(
            Instant::now() < deadline,
            "completion deadline: {snapshot:?}"
        );
        thread::sleep(Duration::from_millis(1));
    }
}
fn saved(controller: &mut DesktopController, path: PathBuf) -> SharedSnapshot {
    let id = controller.queue_shared_save(path).unwrap();
    let snapshot = until(controller, |s| {
        s.io.id == id && matches!(s.io.phase, IoPhase::Complete | IoPhase::Failed)
    });
    assert_eq!(snapshot.io.phase, IoPhase::Complete, "{:?}", snapshot.io);
    snapshot
}
fn shutdown(controller: &mut DesktopController) {
    controller.begin_shutdown();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !controller.shutdown_acknowledged() {
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn complete_archive_reopens_paused_and_historical_analysis_uses_only_its_prefix() {
    let directory = Directory::new();
    let path = directory.0.join("game.mshogi");
    let second_path = directory.0.join("reopened.mshogi");
    let mut controller = controller();
    let initial = controller.shared_snapshot();
    controller
        .settled(
            initial.token,
            SessionOperation::ChessPlay {
                action: "e2e4".parse().unwrap(),
            },
        )
        .unwrap();
    let played = until(&mut controller, |s| {
        history(s).len() == 2 && s.phase == EnginePhase::Idle
    });
    controller
        .settled(
            played.token,
            SessionOperation::Analyze {
                operation: AnalysisOperation::Moves {
                    depth: 1,
                    nodes: 1000,
                },
            },
        )
        .unwrap();
    let analyzed = until(&mut controller, |s| {
        chess(s).analysis.status == AnalysisPhase::Complete
    });
    assert_eq!(analyzed.observations.len(), 2);
    controller
        .settled(
            controller.shared_token(),
            SessionOperation::ChessPreferences {
                orientation: Color::Black,
                analysis_defaults: AnalysisOperation::Moves {
                    depth: 3,
                    nodes: 12_345,
                },
            },
        )
        .unwrap();
    let before = saved(&mut controller, path.clone());
    assert!(!before.document.dirty);
    let bytes = fs::read(&path).unwrap();
    let archive: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(archive["envelope_version"], 1);
    assert_eq!(archive["format"], "mshogi.desktop.document.v1");
    assert_eq!(archive["game_kind"], "chess");
    assert_eq!(
        archive["rules_profile"],
        mshogi_core::CHESS_MATCH_PROFILE_ID
    );
    assert_eq!(
        archive["profile_edition"],
        mshogi_core::native::CHESS_PROFILE_EDITION
    );
    assert_eq!(archive["native_schema_version"], 1);
    assert!(archive.get("profile").is_none());
    assert!(archive.get("schema_version").is_none());
    let payload = &archive["payload"];
    assert!(payload.get("profile").is_none());
    assert!(payload.get("schema_version").is_none());
    assert!(
        payload["match_record_json"]
            .as_str()
            .unwrap()
            .contains("e2e4")
    );
    assert!(payload["controls"].as_array().unwrap().len() >= 3);
    assert_eq!(payload["observations"].as_array().unwrap().len(), 2);
    controller
        .fresh(
            SharedOpenToken {
                document: before.token.document,
                version: before.token.version,
                document_revision: before.document.revision,
            },
            Color::Black,
        )
        .unwrap();
    let token = controller.shared_open_token();
    let id = controller.queue_product_open(path, token).unwrap();
    let reopened = until(&mut controller, |s| {
        s.io.id == id && s.io.phase == IoPhase::Complete
    });
    assert_eq!(history(&reopened), history(&before));
    assert_eq!(fen(&reopened), fen(&before));
    assert!(
        reopened.token.version > before.token.version
            && reopened.token.revision > before.token.revision
    );
    assert!(reopened.paused && !reopened.document.dirty);
    assert_eq!(reopened.phase, EnginePhase::Paused);
    assert!(chess(&reopened).analysis.context.is_none());
    assert_eq!(orientation(&reopened), Color::Black);
    assert!(reopened.observations.iter().all(|o| o.historical));
    let again = saved(&mut controller, second_path.clone());
    let archive_again: serde_json::Value =
        serde_json::from_slice(&fs::read(second_path).unwrap()).unwrap();
    assert_eq!(
        archive_again["payload"]["match_record_json"],
        payload["match_record_json"]
    );
    assert_eq!(
        archive_again["payload"]["game_record_json"],
        payload["game_record_json"]
    );
    // Event zero is the known initial position, with neither played move nor
    // future repetition information. Analysis must remain read-only there.
    controller
        .settled(again.token, SessionOperation::Inspect { event_index: 0 })
        .unwrap();
    let viewed = until(&mut controller, |s| s.historical && s.event_index == 0);
    assert_eq!(selected_fen(&viewed), fen(&initial));
    assert_eq!(history(&viewed), history(&before));
    controller
        .settled(
            viewed.token,
            SessionOperation::Analyze {
                operation: AnalysisOperation::Moves {
                    depth: 1,
                    nodes: 1000,
                },
            },
        )
        .unwrap();
    let historical = until(&mut controller, |s| {
        chess(s).analysis.status == AnalysisPhase::Complete
    });
    let context = chess(&historical).analysis.context.as_ref().unwrap();
    let mshogi_core::native::CanonicalRecord::Chess(record) = &context.game else {
        panic!("chess analysis context");
    };
    assert!(record.events().is_empty());
    assert_eq!(
        record.replay_history().unwrap().game.position().to_fen(),
        fen(&initial)
    );
    assert_eq!(history(&historical), history(&before));
    let live = controller
        .settled(historical.token, SessionOperation::ReturnLive {})
        .unwrap();
    assert!(!live.historical && live.paused);
    assert_eq!(fen(&live), fen(&before));
    assert!(chess(&live).analysis.result.is_none());
    shutdown(&mut controller);
}

#[test]
fn failed_save_and_corrupt_open_preserve_the_valid_file_and_live_document() {
    let directory = Directory::new();
    let path = directory.0.join("valid.mshogi");
    let corrupt = directory.0.join("corrupt.mshogi");
    let mut controller = controller();
    let initial = saved(&mut controller, path.clone());
    let bytes = fs::read(&path).unwrap();
    let changed = controller
        .settled(
            controller.shared_token(),
            SessionOperation::ChessPreferences {
                orientation: Color::Black,
                analysis_defaults: AnalysisOperation::Moves {
                    depth: 2,
                    nodes: 2000,
                },
            },
        )
        .unwrap();
    assert!(changed.document.dirty);
    assert!(
        controller
            .fresh(
                SharedOpenToken {
                    document: changed.token.document,
                    version: changed.token.version,
                    document_revision: initial.document.revision
                },
                Color::Black
            )
            .is_err(),
        "same game version cannot authorize discarding newer settings"
    );
    let id = controller
        .queue_shared_save(directory.0.join("missing").join("game.mshogi"))
        .unwrap();
    let failed = until(&mut controller, |s| {
        s.io.id == id && s.io.phase == IoPhase::Failed
    });
    assert!(failed.document.dirty);
    assert_eq!(fs::read(&path).unwrap(), bytes);
    assert_eq!(failed.document.path, initial.document.path);
    fs::write(&corrupt, b"{broken archive").unwrap();
    let id = controller
        .queue_product_open(corrupt, controller.shared_open_token())
        .unwrap();
    let rejected = until(&mut controller, |s| {
        s.io.id == id && s.io.phase == IoPhase::Failed
    });
    assert_eq!(rejected.token.version, failed.token.version);
    assert_eq!(fen(&rejected), fen(&failed));
    assert_eq!(history(&rejected), history(&failed));
    assert_eq!(rejected.document.revision, failed.document.revision);
    assert_eq!(orientation(&rejected), Color::Black);
    assert!(rejected.document.dirty);
    shutdown(&mut controller);
}

#[test]
fn reopening_retains_unscored_resignation_but_an_earlier_same_ply_view_does_not() {
    let directory = Directory::new();
    let path = directory.0.join("resigned.mshogi");
    let mut controller = controller();
    let initial = controller.shared_snapshot();
    let resigned = controller
        .settled(initial.token, SessionOperation::Resign {})
        .unwrap();
    assert!(matches!(
        chess(&resigned).live_conclusion,
        Some(mshogi_core::chess::MatchConclusion::Unresolved { .. })
    ));
    assert!(history(&resigned).is_empty());
    let before = saved(&mut controller, path.clone());
    let id = controller
        .queue_product_open(path, controller.shared_open_token())
        .unwrap();
    let reopened = until(&mut controller, |s| {
        s.io.id == id && s.io.phase == IoPhase::Complete
    });
    assert_eq!(
        chess(&reopened).live_conclusion,
        chess(&before).live_conclusion
    );
    let previous_event = reopened.event_index - 1;
    controller
        .settled(
            reopened.token,
            SessionOperation::Inspect {
                event_index: previous_event,
            },
        )
        .unwrap();
    let earlier = until(&mut controller, |s| s.historical);
    assert_eq!(earlier.board_ply, 0);
    assert!(
        chess(&earlier).selected_conclusion.is_none(),
        "future resignation cannot leak into a same-ply prefix"
    );
    assert_eq!(
        chess(&earlier).live_conclusion,
        chess(&reopened).live_conclusion,
        "live conclusion remains authoritative"
    );
    shutdown(&mut controller);
}

fn chess(snapshot: &SharedSnapshot) -> &ChessPresentation {
    snapshot.chess.as_ref().expect("native Chess capabilities")
}
fn history(snapshot: &SharedSnapshot) -> Vec<String> {
    live_game(snapshot)
        .history()
        .actions()
        .map(|a| a.to_string())
        .collect()
}
fn fen(snapshot: &SharedSnapshot) -> String {
    live_game(snapshot).position().to_fen()
}
fn orientation(snapshot: &SharedSnapshot) -> Color {
    let NativeDesktopSettings::Chess { settings } = &snapshot.document.settings else {
        panic!("Chess settings")
    };
    settings.orientation
}
trait SettledCommand {
    fn settled(
        &mut self,
        expected: SessionToken,
        operation: SessionOperation,
    ) -> Result<SharedSnapshot, String>;
    fn fresh(&mut self, expected: SharedOpenToken, human: Color) -> Result<SharedSnapshot, String>;
}
fn complete(
    controller: &mut DesktopController,
    mut snapshot: SharedSnapshot,
    receipt: mshogi_desktop_session::AdmissionReceipt,
) -> Result<SharedSnapshot, String> {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if let Some(result) = receipt.completion() {
            result?;
            return Ok(snapshot);
        }
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
        snapshot = controller.shared_snapshot();
    }
}
impl SettledCommand for DesktopController {
    fn settled(
        &mut self,
        expected: SessionToken,
        operation: SessionOperation,
    ) -> Result<SharedSnapshot, String> {
        let (snapshot, receipt) = self.shared_command_with_receipt(SharedDesktopCommand {
            expected,
            game: mshogi_desktop_session::GameKind::Chess,
            operation,
        })?;
        complete(self, snapshot, receipt)
    }
    fn fresh(&mut self, expected: SharedOpenToken, human: Color) -> Result<SharedSnapshot, String> {
        let (snapshot, receipt) = self.shared_new_game(
            expected,
            mshogi_desktop_session::GameKind::Chess,
            NativeSide::Chess(human),
        )?;
        complete(self, snapshot, receipt)
    }
}

fn live_game(snapshot: &SharedSnapshot) -> mshogi_core::chess::AdjudicatedGame {
    snapshot
        .chess_match
        .as_ref()
        .unwrap()
        .replay()
        .unwrap()
        .game()
        .clone()
}

fn selected_fen(snapshot: &SharedSnapshot) -> String {
    let CanonicalRecord::Chess(record) = &snapshot.context else {
        panic!("Chess context")
    };
    record.replay_history().unwrap().game.position().to_fen()
}
