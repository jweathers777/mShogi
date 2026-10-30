//! R2 current-format boundary tests. Expected native transcripts are supplied
//! independently of the document decoder; these are not packaged GUI evidence.
use super::*;
use crate::*;
use mshogi_core::{native::NativeSide, shogi as s, xiangqi as x};
use serde_json::{Value, json, value::RawValue};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

fn path(label: &str) -> PathBuf {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    std::env::temp_dir().join(format!(
        "msh117-document-{}-{}-{label}.mshogi",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ))
}
fn controller() -> DesktopController {
    let mut controller = DesktopController::new("/unused/msh117-engine".into()).unwrap();
    controller.shared.manual_engine = true;
    controller
}
fn settle(controller: &mut DesktopController) {
    let deadline = Instant::now() + Duration::from_secs(30);
    while controller.shared_snapshot().capacity.status == CapacityPhase::Checking {
        assert!(
            Instant::now() < deadline,
            "document admission did not complete"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn send(controller: &mut DesktopController, operation: SessionOperation) {
    settle(controller);
    let snapshot = controller.shared_snapshot();
    let (_, receipt) = controller
        .shared_command_with_receipt(SharedDesktopCommand {
            expected: snapshot.token,
            game: snapshot.game,
            operation,
        })
        .unwrap();
    settle(controller);
    receipt.completion().unwrap().unwrap();
}
fn wait(controller: &mut DesktopController, id: u64) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = controller.shared_snapshot();
        if snapshot.io.id == id {
            match snapshot.io.phase {
                IoPhase::Complete => return Ok(()),
                IoPhase::Failed => return Err(snapshot.io.message.unwrap_or_default()),
                _ => {}
            }
        }
        assert!(
            Instant::now() < deadline,
            "document file operation did not complete"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn saved(controller: &mut DesktopController) -> Vec<u8> {
    let file = path("save");
    let id = controller.queue_shared_save(file.clone()).unwrap();
    wait(controller, id).unwrap();
    let bytes = fs::read(&file).unwrap();
    fs::remove_file(file).unwrap();
    bytes
}
fn open(controller: &mut DesktopController, bytes: &[u8]) -> Result<(), String> {
    let file = path("open");
    fs::write(&file, bytes).unwrap();
    let id = controller
        .queue_shared_open(file.clone(), controller.shared_open_token())
        .unwrap();
    let result = wait(controller, id);
    fs::remove_file(file).unwrap();
    result
}
fn envelope(profile: RulesProfile) -> String {
    String::from_utf8(
        encode(
            profile,
            &json!({"native_document":"opaque in outer parser"}),
        )
        .unwrap(),
    )
    .unwrap()
}

#[test]
fn current_outer_identity_is_explicit_for_every_native_game() {
    for profile in [
        RulesProfile::ChessMatchV1,
        RulesProfile::ShogiCsa36V1,
        RulesProfile::XiangqiWxf2018V1,
    ] {
        let text = envelope(profile);
        let parsed = DocumentEnvelope::parse(text.as_bytes()).unwrap();
        assert_eq!(parsed.format, "mshogi.desktop.document.v1");
        assert_eq!(parsed.envelope_version, 1);
        assert_eq!(parsed.native_schema_version, 1);
        assert_eq!(parsed.game_kind, profile.game());
        assert_eq!(parsed.rules_profile, profile);
        assert_eq!(parsed.profile_edition, profile.edition());
        assert!(parsed.payload.get().contains("opaque in outer parser"));
    }
}

#[test]
fn every_outer_field_is_required_and_duplicate_fields_reject() {
    let text = envelope(RulesProfile::ShogiCsa36V1);
    let value: Value = serde_json::from_str(&text).unwrap();
    for key in [
        "envelope_version",
        "format",
        "game_kind",
        "rules_profile",
        "profile_edition",
        "native_schema_version",
        "payload",
    ] {
        let mut absent = value.clone();
        absent.as_object_mut().unwrap().remove(key);
        assert!(
            DocumentEnvelope::parse(&serde_json::to_vec(&absent).unwrap()).is_err(),
            "missing {key} accepted"
        );
        let duplicate = format!("{{\"{key}\":{},{}", value[key], &text[1..]);
        assert!(
            DocumentEnvelope::parse(duplicate.as_bytes()).is_err(),
            "duplicate {key} accepted"
        );
    }
}

#[test]
fn unsupported_conflicting_malformed_and_old_outer_formats_reject() {
    let original: Value = serde_json::from_str(&envelope(RulesProfile::ShogiCsa36V1)).unwrap();
    for (key, bad) in [
        ("envelope_version", json!(0)),
        ("envelope_version", json!(2)),
        ("format", json!("mshogi.desktop.archive.v2")),
        ("game_kind", json!("chess")),
        ("game_kind", json!("tori")),
        ("rules_profile", json!("mshogi.xiangqi.wxf2018.v1")),
        ("rules_profile", json!("unapproved.profile")),
        ("profile_edition", json!("unapproved-edition")),
        ("native_schema_version", json!(0)),
        ("native_schema_version", json!(2)),
        ("native_schema_version", json!("1")),
        ("unexpected", json!(true)),
    ] {
        let mut value = original.clone();
        value[key] = bad;
        assert!(
            DocumentEnvelope::parse(&serde_json::to_vec(&value).unwrap()).is_err(),
            "accepted altered {key}"
        );
    }
    for version in [1, 2, 3] {
        let old = json!({"schema_version": version, "format": format!("mshogi.desktop.archive.v{version}"), "profile": format!("mshogi.desktop.archive.v{version}"), "payload": {}});
        assert!(DocumentEnvelope::parse(&serde_json::to_vec(&old).unwrap()).is_err());
    }
    assert!(DocumentEnvelope::parse(b"{").is_err());
    assert!(
        DocumentEnvelope::parse(&vec![b' '; MAX_ARCHIVE_BYTES + 1])
            .err()
            .unwrap()
            .contains("capacity")
    );
}

#[test]
fn raw_payload_bytes_reach_the_native_boundary_without_map_normalization() {
    let mut parsed =
        DocumentEnvelope::parse(envelope(RulesProfile::ShogiCsa36V1).as_bytes()).unwrap();
    let raw = "{ \"events\" : [], \"events\" : [1] }";
    parsed.payload = RawValue::from_string(raw.into()).unwrap();
    let bytes = serde_json::to_vec(&parsed).unwrap();
    let parsed = DocumentEnvelope::parse(&bytes).unwrap();
    assert_eq!(parsed.payload.get(), raw);
}

#[test]
fn real_native_transcripts_roundtrip_current_documents_with_fresh_stopped_runtime() {
    let mut shogi = s::Game::new();
    for action in ["7g7f", "3c3d"] {
        shogi
            .submit(s::GameRequest {
                expected: shogi.context(),
                actor: shogi.position().side_to_move(),
                command: s::GameCommand::Play(action.parse().unwrap()),
            })
            .unwrap();
    }
    assert_eq!(shogi.position().side_to_move(), s::Side::Sente);
    assert_eq!(shogi.events().len(), 2);
    let mut xiangqi = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    for action in ["a3a4", "a6a5"] {
        xiangqi
            .submit(x::GameRequest {
                expected: xiangqi.context(),
                actor: xiangqi.position().side_to_move(),
                command: x::GameCommand::Play {
                    action: action.parse().unwrap(),
                },
            })
            .unwrap();
    }
    assert_eq!(xiangqi.position().side_to_move(), x::Side::Red);
    assert!(xiangqi.conclusion().is_none());
    let fixtures = [
        (
            CanonicalRecord::shogi(s::GameRecord::from_game(&shogi)),
            NativeSide::Shogi(s::Side::Sente),
        ),
        (
            CanonicalRecord::xiangqi(x::GameRecord::from_game(&xiangqi).unwrap()),
            NativeSide::Xiangqi(x::Side::Red),
        ),
    ];
    for (expected, human) in fixtures {
        let mut source = controller();
        send(
            &mut source,
            SessionOperation::Replace {
                record: expected.clone(),
                human,
            },
        );
        let original = source.shared_snapshot();
        let bytes = saved(&mut source);
        let document = DocumentEnvelope::parse(&bytes).unwrap();
        assert_eq!(document.game_kind, expected.game_kind());
        assert_eq!(document.rules_profile, expected.profile());
        let mut restored = controller();
        open(&mut restored, &bytes).unwrap();
        let reopened = restored.shared_snapshot();
        assert_eq!(reopened.context, original.context);
        assert_ne!(reopened.token, original.token);
        assert!(reopened.paused);
        assert!(reopened.execution_purpose.is_none());
        assert!(!reopened.document.dirty);
        let reopened_document = DocumentEnvelope::parse(&saved(&mut restored)).unwrap();
        #[derive(serde::Deserialize)]
        struct NativePayload {
            current: Box<RawValue>,
        }
        let native: NativePayload = serde_json::from_str(reopened_document.payload.get()).unwrap();
        assert_eq!(
            CanonicalRecord::from_json(native.current.get()).unwrap(),
            expected
        );
    }
    // Chess's full MatchRecord remains distinct from the execution GameRecord.
    let mut source = controller();
    let (_, receipt) = source
        .command_with_receipt(DesktopCommand::Resign {
            version: source.session.version,
        })
        .unwrap();
    settle(&mut source);
    receipt.completion().unwrap().unwrap();
    assert!(source.session.chess_mut().game.conclusion().is_some());
    assert!(
        source
            .session
            .chess_mut()
            .game
            .record()
            .events()
            .any(|(request, _)| matches!(
                request.command,
                mshogi_core::chess::MatchCommand::Resign { .. }
            ) && request.actor == mshogi_core::chess::Color::White)
    );
    assert_eq!(
        source
            .session
            .chess_mut()
            .game
            .game()
            .record()
            .events()
            .len(),
        0
    );
    let before = source.session.chess_mut().game.record().to_json();
    let bytes = saved(&mut source);
    assert_eq!(
        DocumentEnvelope::parse(&bytes).unwrap().game_kind,
        GameKind::Chess
    );
    let mut restored = controller();
    open(&mut restored, &bytes).unwrap();
    assert_eq!(restored.session.chess_mut().game.record().to_json(), before);
    assert!(restored.session.chess_mut().play_paused);
    assert!(restored.session.chess_mut().pending.is_none());
}

#[test]
fn duplicate_native_schema_fields_reject_atomically_after_raw_outer_dispatch() {
    for (record, human) in [
        (
            CanonicalRecord::shogi(s::GameRecord::from_game(&s::Game::new())),
            NativeSide::Shogi(s::Side::Sente),
        ),
        (
            CanonicalRecord::xiangqi(
                x::GameRecord::from_game(&x::Game::standard(x::ClaimPolicy::Explicit).unwrap())
                    .unwrap(),
            ),
            NativeSide::Xiangqi(x::Side::Red),
        ),
    ] {
        let mut current = controller();
        send(&mut current, SessionOperation::Replace { record, human });
        let bytes = saved(&mut current);
        let before = current.shared_snapshot();
        let mut document = DocumentEnvelope::parse(&bytes).unwrap();
        let text = document.payload.get();
        let needle = "\"schema_version\":1";
        assert!(
            text.contains(needle),
            "native payload schema absent: {text}"
        );
        let hostile = text.replacen(needle, "\"schema_version\":1,\"schema_version\":1", 1);
        document.payload = RawValue::from_string(hostile).unwrap();
        let hostile = serde_json::to_vec(&document).unwrap();
        // Outer parsing deliberately retains the duplicate; native parsing rejects it.
        assert!(DocumentEnvelope::parse(&hostile).is_ok());
        assert!(open(&mut current, &hostile).is_err());
        let after = current.shared_snapshot();
        assert_eq!(after.token.document, before.token.document);
        assert_eq!(after.token.version, before.token.version);
        assert_eq!(after.token.selection, before.token.selection);
        assert_eq!(after.context, before.context);
        assert_eq!(after.document.revision, before.document.revision);
        assert_eq!(after.document.dirty, before.document.dirty);
    }
}

#[test]
fn hostile_identity_diagnostics_are_bounded_and_utf8_safe() {
    let original: Value = serde_json::from_str(&envelope(RulesProfile::ShogiCsa36V1)).unwrap();
    for key in ["game_kind", "rules_profile", "unexpected"] {
        let mut value = original.clone();
        if key == "unexpected" {
            value["棋".repeat(10_000)] = json!(true);
        } else {
            value[key] = json!("棋".repeat(10_000));
        }
        let error = DocumentEnvelope::parse(&serde_json::to_vec(&value).unwrap())
            .err()
            .unwrap();
        assert!(error.len() <= mshogi_engine::MAX_STDERR_BYTES);
        assert!(error.starts_with("Invalid current desktop document:"));
    }
}

#[test]
fn native_payload_failure_diagnostics_stay_bounded_and_open_is_atomic() {
    let mut current = controller();
    let bytes = saved(&mut current);
    let before = current.shared_snapshot();
    let mut document = DocumentEnvelope::parse(&bytes).unwrap();
    let mut payload: Value = serde_json::from_str(document.payload.get()).unwrap();
    payload["棋".repeat(10_000)] = json!(true);
    document.payload = serde_json::value::to_raw_value(&payload).unwrap();
    let error = open(&mut current, &serde_json::to_vec(&document).unwrap()).unwrap_err();
    assert!(error.len() <= mshogi_engine::MAX_STDERR_BYTES);
    assert!(error.starts_with("Invalid desktop archive:"));
    let after = current.shared_snapshot();
    assert_eq!(after.context, before.context);
    assert_eq!(after.token.document, before.token.document);
    assert_eq!(after.token.version, before.token.version);
    assert_eq!(after.document.revision, before.document.revision);
    assert_eq!(after.document.dirty, before.document.dirty);
}
