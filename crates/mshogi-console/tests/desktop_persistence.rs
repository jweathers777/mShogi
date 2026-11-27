//! Real files and bundled-engine processes through the public desktop controller.
mod support;
use mshogi_core::chess::Color;
use mshogi_core::game::{CanonicalRecord, TaggedSide};
use mshogi_desktop_session::{
    AnalysisOperation, AnalysisPhase, DesktopController, EnginePhase, IoPhase, SharedSnapshot,
};
use mshogi_desktop_session::{PlayMode, SessionOperation, SharedOpenToken, game_document};
use std::{fs, path::PathBuf};
use support::desktop::*;

fn controller() -> DesktopController {
    DesktopController::new(PathBuf::from(env!("CARGO_BIN_EXE_mshogi"))).unwrap()
}
fn saved(controller: &mut DesktopController, path: PathBuf) -> SharedSnapshot {
    let id = controller.queue_shared_save(path).unwrap();
    let snapshot = until(controller, 30, |s| {
        s.io.id == id && matches!(s.io.phase, IoPhase::Complete | IoPhase::Failed)
    });
    assert_eq!(snapshot.io.phase, IoPhase::Complete, "{:?}", snapshot.io);
    snapshot
}

#[test]
fn minimal_document_reopens_paused_and_historical_analysis_uses_only_its_prefix() {
    let directory = support::cli::Dir::new();
    let path = directory.0.join("game.mshogi");
    let second_path = directory.0.join("reopened.mshogi");
    let mut controller = controller();
    let initial = controller.shared_snapshot();
    controller
        .settled(initial.token, SessionOperation::play("e2e4"))
        .unwrap();
    let played = until(&mut controller, 30, |s| {
        history(s).len() == 2 && s.phase == EnginePhase::Idle
    });
    controller
        .settled(
            played.token,
            SessionOperation::Analyze {
                operation: AnalysisOperation {
                    depth: 1,
                    nodes: 1000,
                },
            },
        )
        .unwrap();
    let analyzed = until(&mut controller, 30, |s| {
        chess(s).analysis.status == AnalysisPhase::Complete
    });
    assert_eq!(analyzed.observations.len(), 2);
    controller
        .settled(
            controller.shared_token(),
            SessionOperation::Preferences {
                settings: game_document::GameSettings {
                    play_mode: PlayMode::HumanEngine,
                    orientation: TaggedSide::Chess(Color::Black),
                    analysis_defaults: AnalysisOperation {
                        depth: 3,
                        nodes: 12_345,
                    },
                },
            },
        )
        .unwrap();
    let before = saved(&mut controller, path.clone());
    assert!(!before.document.dirty);
    let bytes = fs::read(&path).unwrap();
    let archive: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(archive.as_object().unwrap().len(), 3);
    assert_eq!(archive["format"], "mshogi.desktop.document.v2");
    assert_eq!(archive["game_kind"], "chess");
    assert!(archive.get("profile").is_none());
    assert!(archive.get("schema_version").is_none());
    let payload = &archive["payload"];
    assert!(payload.get("profile").is_none());
    assert!(payload.get("schema_version").is_none());
    assert_eq!(
        payload
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        ["human", "record", "settings"]
    );
    assert!(payload["record"]["commands"].to_string().contains("e2e4"));
    assert_eq!(payload["record"]["commands"].as_array().unwrap().len(), 2);
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
    let reopened = until(&mut controller, 30, |s| {
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
    assert!(reopened.observations.is_empty());
    let again = saved(&mut controller, second_path.clone());
    let archive_again: serde_json::Value =
        serde_json::from_slice(&fs::read(second_path).unwrap()).unwrap();
    assert_eq!(archive_again["payload"], *payload);
    // Event zero is the known initial position, with neither played move nor
    // future repetition information. Analysis must remain read-only there.
    controller
        .settled(again.token, SessionOperation::Inspect { event_index: 0 })
        .unwrap();
    let viewed = until(&mut controller, 30, |s| s.historical && s.event_index == 0);
    assert_eq!(selected_fen(&viewed), fen(&initial));
    assert_eq!(history(&viewed), history(&before));
    controller
        .settled(
            viewed.token,
            SessionOperation::Analyze {
                operation: AnalysisOperation {
                    depth: 1,
                    nodes: 1000,
                },
            },
        )
        .unwrap();
    let historical = until(&mut controller, 30, |s| {
        chess(s).analysis.status == AnalysisPhase::Complete
    });
    let context = chess(&historical).analysis.context.as_ref().unwrap();
    let mshogi_core::game::CanonicalRecord::Chess(record) = &context.game else {
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
    let directory = support::cli::Dir::new();
    let path = directory.0.join("valid.mshogi");
    let corrupt = directory.0.join("corrupt.mshogi");
    let mut controller = controller();
    let initial = saved(&mut controller, path.clone());
    let bytes = fs::read(&path).unwrap();
    let changed = controller
        .settled(
            controller.shared_token(),
            SessionOperation::Preferences {
                settings: game_document::GameSettings {
                    play_mode: PlayMode::HumanEngine,
                    orientation: TaggedSide::Chess(Color::Black),
                    analysis_defaults: AnalysisOperation {
                        depth: 2,
                        nodes: 2000,
                    },
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
    let failed = until(&mut controller, 30, |s| {
        s.io.id == id && s.io.phase == IoPhase::Failed
    });
    assert!(failed.document.dirty);
    assert_eq!(fs::read(&path).unwrap(), bytes);
    assert_eq!(failed.document.path, initial.document.path);
    fs::write(&corrupt, b"{broken archive").unwrap();
    let id = controller
        .queue_product_open(corrupt, controller.shared_open_token())
        .unwrap();
    let rejected = until(&mut controller, 30, |s| {
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
    let directory = support::cli::Dir::new();
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
    let reopened = until(&mut controller, 30, |s| {
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
    let earlier = until(&mut controller, 30, |s| s.historical);
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

fn orientation(snapshot: &SharedSnapshot) -> Color {
    let TaggedSide::Chess(orientation) = snapshot.document.settings.orientation else {
        panic!("Chess settings")
    };
    orientation
}

fn selected_fen(snapshot: &SharedSnapshot) -> String {
    let CanonicalRecord::Chess(record) = &snapshot.context else {
        panic!("Chess context")
    };
    record.replay_history().unwrap().game.position().to_fen()
}
