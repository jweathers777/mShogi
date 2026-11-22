//! The minimal saved document (MSH-228): game, sides and settings only.
use super::*;
use crate::native_game::Tori;
use mshogi_core::tori as t;
use serde_json::{Value, json};

type Doc = ValidatedGameDocument<Tori>;

fn standard() -> Doc {
    Doc::new(
        t::GameRecord::from_game(&t::Game::standard()).unwrap(),
        t::Side::Sente,
    )
    .unwrap()
}
fn first_move(doc: &Doc) -> t::GameCommand {
    let game = doc.record().to_game().unwrap();
    t::GameCommand::Play {
        action: game.legal_actions()[0],
    }
}
fn value(doc: &Doc) -> Value {
    serde_json::from_slice(doc.bytes()).unwrap()
}
fn decode(v: &Value) -> Result<Doc, GameDocumentError> {
    Doc::decode(&serde_json::to_vec(v).unwrap(), &mut || false)
}

#[test]
fn the_payload_holds_only_the_game_sides_and_settings() {
    let v = value(&standard());
    let mut keys: Vec<_> = v["payload"].as_object().unwrap().keys().cloned().collect();
    keys.sort();
    assert_eq!(keys, ["human", "record", "settings"]);
}

#[test]
fn commands_rewind_and_settings_round_trip_through_saved_bytes() {
    let doc = standard();
    let moved = doc
        .with_command(t::Side::Sente, first_move(&doc), &mut || false)
        .unwrap();
    assert_eq!(moved.record().event_count(), 1);
    let reopened = Doc::decode(moved.bytes(), &mut || false).unwrap();
    assert_eq!(reopened.record(), moved.record());
    assert_eq!(reopened.bytes(), moved.bytes());
    // Undo and Take back cut the command list; nothing else is kept.
    let cut = moved.with_rewind(0, &mut || false).unwrap();
    assert_eq!(cut.record(), doc.record());
    assert!(cut.with_rewind(0, &mut || false).is_err());
    let flipped = moved
        .with_settings(GameSettings {
            play_mode: crate::PlayMode::default(),
            orientation: t::Side::Gote,
        })
        .unwrap();
    assert_eq!(
        Doc::decode(flipped.bytes(), &mut || false)
            .unwrap()
            .settings()
            .orientation,
        t::Side::Gote
    );
}

#[test]
fn the_rules_core_refuses_an_illegal_command_and_a_play_mode_change() {
    let doc = standard();
    assert!(
        doc.with_command(t::Side::Gote, first_move(&doc), &mut || false)
            .is_err()
    );
    assert!(
        doc.with_settings(GameSettings {
            play_mode: crate::PlayMode::TwoHumans,
            orientation: t::Side::Sente,
        })
        .is_err()
    );
}

#[test]
fn opening_refuses_wrong_shape_game_profile_and_records() {
    let good = value(&standard());
    assert!(decode(&good).is_ok());
    let mut extra = good.clone();
    extra["payload"]["controls"] = json!([]);
    assert!(decode(&extra).is_err(), "unknown field");
    let mut missing = good.clone();
    missing["payload"].as_object_mut().unwrap().remove("human");
    assert!(decode(&missing).is_err(), "missing field");
    let mut kind = good.clone();
    kind["game_kind"] = json!("chu");
    assert!(decode(&kind).is_err(), "wrong game");
    let mut record = good.clone();
    record["payload"]["record"]["payload"]["events"] =
        json!([{"index": 0, "actor": "gote", "command": {"kind": "resign"}}]);
    assert!(decode(&record).is_err(), "record that does not replay");
    let oversize = vec![b' '; MAX_ARCHIVE_BYTES + 1];
    let error = Doc::decode(&oversize, &mut || false).unwrap_err();
    assert_eq!(
        error.resource.unwrap().facts()[0].category,
        ResourceCategory::EncodedDocumentBytes
    );
}

#[test]
fn opening_checks_the_stop() {
    let doc = standard();
    let error = Doc::decode(doc.bytes(), &mut || true).unwrap_err();
    assert!(error.stopped);
}
