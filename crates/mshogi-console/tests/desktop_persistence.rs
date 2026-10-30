//! Real files and bundled-engine processes through the public desktop controller.
use mshogi_core::chess::Color;
use mshogi_desktop_session::{
    AnalysisOperation, AnalysisPhase, DesktopCommand, DesktopController, EnginePhase, IoPhase,
    Snapshot,
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
fn until(controller: &mut DesktopController, ready: impl Fn(&Snapshot) -> bool) -> Snapshot {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = controller.snapshot();
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
fn saved(controller: &mut DesktopController, path: PathBuf) -> Snapshot {
    let id = controller.queue_save(path).unwrap();
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
    let initial = controller.snapshot();
    controller
        .settled(DesktopCommand::Play {
            version: initial.version,
            action: "e2e4".parse().unwrap(),
        })
        .unwrap();
    let played = until(&mut controller, |s| {
        s.history.len() == 2 && s.engine.status == EnginePhase::Idle
    });
    controller
        .settled(DesktopCommand::Analyze {
            version: played.version,
            operation: AnalysisOperation::Moves {
                depth: 1,
                nodes: 1000,
            },
        })
        .unwrap();
    let analyzed = until(&mut controller, |s| {
        s.analysis.status == AnalysisPhase::Complete
    });
    assert_eq!(analyzed.observations.len(), 2);
    controller
        .settled(DesktopCommand::Preferences {
            orientation: Color::Black,
            analysis_defaults: AnalysisOperation::Moves {
                depth: 3,
                nodes: 12_345,
            },
        })
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
        .settled(DesktopCommand::NewGame {
            version: before.version,
            document_revision: before.document.revision,
            human: Color::Black,
        })
        .unwrap();
    let token = controller.open_token();
    let id = controller.queue_open(path, token).unwrap();
    let reopened = until(&mut controller, |s| {
        s.io.id == id && s.io.phase == IoPhase::Complete
    });
    assert_eq!(reopened.history, before.history);
    assert_eq!(reopened.fen, before.fen);
    assert!(reopened.version > before.version && reopened.revision > before.revision);
    assert!(reopened.play_paused && !reopened.document.dirty);
    assert_eq!(reopened.engine.status, EnginePhase::Paused);
    assert!(reopened.analysis.context.is_none());
    assert_eq!(reopened.document.settings.orientation, Color::Black);
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
        .settled(DesktopCommand::Inspect {
            version: again.version,
            event_index: 0,
        })
        .unwrap();
    let viewed = until(&mut controller, |s| {
        s.view.historical && s.view.event_index == 0
    });
    assert_eq!(viewed.view.fen, initial.fen);
    assert_eq!(viewed.history, before.history);
    controller
        .settled(DesktopCommand::Analyze {
            version: viewed.version,
            operation: AnalysisOperation::Moves {
                depth: 1,
                nodes: 1000,
            },
        })
        .unwrap();
    let historical = until(&mut controller, |s| {
        s.analysis.status == AnalysisPhase::Complete
    });
    let context = historical.analysis.context.as_ref().unwrap();
    let mshogi_core::native::CanonicalRecord::Chess(record) = &context.game else {
        panic!("chess analysis context");
    };
    assert!(record.events().is_empty());
    assert_eq!(
        record.replay_history().unwrap().game.position().to_fen(),
        initial.fen
    );
    assert_eq!(historical.history, before.history);
    let live = controller
        .settled(DesktopCommand::ReturnLive {
            version: historical.version,
        })
        .unwrap();
    assert!(!live.view.historical && live.play_paused);
    assert_eq!(live.fen, before.fen);
    assert!(live.analysis.result.is_none());
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
        .settled(DesktopCommand::Preferences {
            orientation: Color::Black,
            analysis_defaults: AnalysisOperation::Moves {
                depth: 2,
                nodes: 2000,
            },
        })
        .unwrap();
    assert!(changed.document.dirty);
    assert!(
        controller
            .settled(DesktopCommand::NewGame {
                version: changed.version,
                document_revision: initial.document.revision,
                human: Color::Black,
            })
            .is_err(),
        "same game version cannot authorize discarding newer settings"
    );
    let id = controller
        .queue_save(directory.0.join("missing").join("game.mshogi"))
        .unwrap();
    let failed = until(&mut controller, |s| {
        s.io.id == id && s.io.phase == IoPhase::Failed
    });
    assert!(failed.document.dirty);
    assert_eq!(fs::read(&path).unwrap(), bytes);
    assert_eq!(failed.document.path, initial.document.path);
    fs::write(&corrupt, b"{broken archive").unwrap();
    let id = controller
        .queue_open(corrupt, controller.open_token())
        .unwrap();
    let rejected = until(&mut controller, |s| {
        s.io.id == id && s.io.phase == IoPhase::Failed
    });
    assert_eq!(rejected.version, failed.version);
    assert_eq!(rejected.fen, failed.fen);
    assert_eq!(rejected.history, failed.history);
    assert_eq!(rejected.document.revision, failed.document.revision);
    assert_eq!(rejected.document.settings.orientation, Color::Black);
    assert!(rejected.document.dirty);
    shutdown(&mut controller);
}

#[test]
fn reopening_retains_unscored_resignation_but_an_earlier_same_ply_view_does_not() {
    let directory = Directory::new();
    let path = directory.0.join("resigned.mshogi");
    let mut controller = controller();
    let initial = controller.snapshot();
    let resigned = controller
        .settled(DesktopCommand::Resign {
            version: initial.version,
        })
        .unwrap();
    assert!(matches!(
        resigned.conclusion,
        Some(mshogi_core::chess::MatchConclusion::Unresolved { .. })
    ));
    assert!(resigned.history.is_empty());
    let before = saved(&mut controller, path.clone());
    let id = controller
        .queue_open(path, controller.open_token())
        .unwrap();
    let reopened = until(&mut controller, |s| {
        s.io.id == id && s.io.phase == IoPhase::Complete
    });
    assert_eq!(reopened.conclusion, before.conclusion);
    let previous_event = reopened.view.event_count - 1;
    controller
        .settled(DesktopCommand::Inspect {
            version: reopened.version,
            event_index: previous_event,
        })
        .unwrap();
    let earlier = until(&mut controller, |s| s.view.historical);
    assert_eq!(earlier.view.ply, 0);
    assert!(
        earlier.view.conclusion.is_none(),
        "future resignation cannot leak into a same-ply prefix"
    );
    assert_eq!(
        earlier.conclusion, reopened.conclusion,
        "live conclusion remains authoritative"
    );
    shutdown(&mut controller);
}

// Match the host command promise: completion means native admission settled.
trait SettledCommand {
    fn settled(&mut self, command: DesktopCommand) -> Result<Snapshot, String>;
}
impl SettledCommand for DesktopController {
    fn settled(&mut self, command: DesktopCommand) -> Result<Snapshot, String> {
        let (mut snapshot, receipt) = self.command_with_receipt(command)?;
        let deadline = Instant::now() + Duration::from_secs(20);
        loop {
            if let Some(result) = receipt.completion() {
                result?;
                return Ok(snapshot);
            }
            assert!(
                Instant::now() < deadline,
                "native capacity admission deadline"
            );
            thread::sleep(Duration::from_millis(1));
            snapshot = self.snapshot();
        }
    }
}
