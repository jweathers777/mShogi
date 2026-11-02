//! An actual external core consumer of the game-owned Chu handoff. This does not
//! add shared desktop/engine dispatch or advertise an engine capability.
use mshogi_core::chu::{
    self, Action, Game, GameCommand, GameError, GameRecord, GameRequest, Piece, Position,
    RootMetadata, Side, Square,
};
use serde::Deserialize;
use serde_json::{json, value::RawValue};
fn square(f: u8, r: u8) -> Square {
    Square::new(f, r).unwrap()
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConsumerPayload {
    profile: String,
    record: Box<RawValue>,
}
fn consume(raw: &str) -> Result<Game, String> {
    let p: ConsumerPayload = serde_json::from_str(raw).map_err(|e| e.to_string())?;
    if p.profile != chu::PROFILE_ID {
        return Err("unsupported profile".into());
    }
    GameRecord::from_json(p.record.get())
        .and_then(|r| r.to_game())
        .map_err(|e| e.to_string())
}
fn game() -> Game {
    let entries = [
        (square(1, 12), Piece::from_code(Side::Sente, "K").unwrap()),
        (square(12, 1), Piece::from_code(Side::Gote, "K").unwrap()),
        (square(6, 6), Piece::from_code(Side::Sente, "Ln").unwrap()),
        (square(6, 5), Piece::from_code(Side::Gote, "P").unwrap()),
        (square(5, 5), Piece::from_code(Side::Gote, "Si").unwrap()),
    ];
    Game::from_root(
        Position::from_diagram(
            &entries,
            Side::Sente,
            RootMetadata::imported("native-consumer-fixture".into(), 120).unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}
#[test]
fn actual_complete_payloads_choices_effects_prefixes_and_fresh_authority() {
    let mut g = game();
    let selected = Action {
        from: square(6, 6),
        route: vec![square(6, 5), square(5, 5)],
        promote: false,
    };
    assert!(g.legal_actions().contains(&selected));
    let choices = g.local_position().inspection(square(6, 6));
    assert!(!choices.is_empty());
    let old = g.context();
    g.submit(GameRequest {
        expected: old.clone(),
        actor: Side::Sente,
        command: GameCommand::Play {
            action: selected.clone(),
        },
    })
    .unwrap();
    let e = g.events()[0].effects.as_ref().unwrap();
    assert_eq!(e.action, selected);
    assert_eq!(
        e.captures.iter().map(|c| c.square).collect::<Vec<_>>(),
        [square(6, 5), square(5, 5)]
    );
    let wire=json!({"profile":chu::PROFILE_ID,"record":serde_json::from_str::<serde_json::Value>(GameRecord::from_game(&g).unwrap().to_json()).unwrap()}).to_string();
    let mut reopened = consume(&wire).unwrap();
    assert!(g.context().same_semantics(&reopened.context()));
    assert_ne!(g.context(), reopened.context());
    assert_eq!(
        reopened.position().root().source(),
        Some("native-consumer-fixture")
    );
    assert_eq!(reopened.position().declared_total_plies(), 121);
    let prefix = reopened.record_prefix(0).unwrap().to_game().unwrap();
    assert!(prefix.events().is_empty());
    assert_eq!(prefix.position().declared_total_plies(), 120);
    assert_eq!(prefix.royal_squares(Side::Sente), [square(1, 12)]);
    let historical = reopened.event_prefix(0).unwrap();
    assert_eq!(historical.state(), prefix.position().state());
    assert_eq!(
        historical.protection(),
        prefix.local_position().protection()
    );
    assert_eq!(
        reopened
            .submit(GameRequest {
                expected: old,
                actor: Side::Sente,
                command: GameCommand::Play { action: selected }
            })
            .unwrap_err(),
        GameError::StaleContext
    );
}
#[test]
fn native_handoff_rejects_duplicates_before_an_intermediate_map_can_erase_them() {
    let g = game();
    let raw = GameRecord::from_game(&g).unwrap();
    let bad = raw.to_json().replacen(
        "\"promoted\":false",
        "\"promoted\":false,\"promoted\":false",
        1,
    );
    assert!(
        consume(&format!(
            "{{\"profile\":\"{}\",\"record\":{bad}}}",
            chu::PROFILE_ID
        ))
        .is_err()
    );
    assert!(
        consume(&format!(
            "{{\"profile\":\"{}\",\"record\":{},\"profile\":\"{}\"}}",
            chu::PROFILE_ID,
            raw.to_json(),
            chu::PROFILE_ID
        ))
        .is_err()
    );
    assert!(
        consume(&format!(
            "{{\"profile\":\"mshogi.chess.match.v1\",\"record\":{}}}",
            raw.to_json()
        ))
        .is_err()
    );
}
