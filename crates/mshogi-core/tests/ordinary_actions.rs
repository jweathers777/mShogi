//! Independently expected ordinary transitions, exact undo and atomic failures.

use mshogi_core::{
    CHESS_POSITION_PROFILE_ID,
    chess::{Action, ActionError, Position, Promotion, Square, UndoError},
};
use serde_json::Value;

const INITIAL: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const AFTER_E4: &str = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1";
const CASTLES: &str = "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1";

// Test-data adapter only; no production text/machine protocol is established.
fn action(text: &str) -> Action {
    let bytes = text.as_bytes();
    assert!(matches!(bytes.len(), 4 | 5));
    Action {
        from: Square::new(bytes[0] - b'a', bytes[1] - b'1').unwrap(),
        to: Square::new(bytes[2] - b'a', bytes[3] - b'1').unwrap(),
        promotion: bytes.get(4).map(|choice| match choice {
            b'q' => Promotion::Queen,
            b'r' => Promotion::Rook,
            b'b' => Promotion::Bishop,
            b'n' => Promotion::Knight,
            _ => panic!("unsupported fixture promotion"),
        }),
    }
}

#[test]
fn independently_expected_transitions_restore_every_saved_field_in_reverse() {
    let data: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/chess/ordinary-transitions.json"
    ))
    .unwrap();
    assert_eq!(data["format"], "mshogi.chess.ordinary-transitions.v1");
    assert_eq!(data["schema_version"], 1);
    assert_eq!(data["rules_profile"], CHESS_POSITION_PROFILE_ID);
    let mut ids = std::collections::HashSet::new();
    let mut step_count = 0;
    for case in data["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        assert!(ids.insert(id), "duplicate case {id}");
        assert_eq!(case["rules_profile"], CHESS_POSITION_PROFILE_ID, "{id}");
        assert!(
            !case["provenance"]["rationale"].as_str().unwrap().is_empty(),
            "{id}"
        );
        let mut position = Position::from_fen(case["before_fen"].as_str().unwrap()).unwrap();
        let mut records = Vec::new();
        for step in case["steps"].as_array().unwrap() {
            let uci = step["action"].as_str().unwrap();
            let expected = step["after_fen"].as_str().unwrap();
            let before = position.clone();
            let undo = position
                .apply_ordinary(action(uci))
                .unwrap_or_else(|error| panic!("{id} {uci} from {}: {error}", before.to_fen()));
            assert_eq!(
                position.to_fen(),
                expected,
                "{id} {uci} from {}",
                before.to_fen()
            );
            assert_eq!(
                position,
                Position::from_fen(expected).unwrap(),
                "{id} {uci}: full saved tuple"
            );
            records.push((before, undo));
            step_count += 1;
        }
        while let Some((before, undo)) = records.pop() {
            position
                .undo(&undo)
                .unwrap_or_else(|error| panic!("{id}: {error}"));
            assert_eq!(position, before, "{id}: exact reverse-step restoration");
            assert_eq!(
                position.to_fen(),
                before.to_fen(),
                "{id}: serialized restoration"
            );
        }
        assert_eq!(
            position.to_fen(),
            case["before_fen"].as_str().unwrap(),
            "{id}: full sequence reversal"
        );
    }
    assert_eq!((ids.len(), step_count), (49, 74));
}

fn assert_rejected(fen: &str, text: &str, expected: ActionError) {
    let mut position = Position::from_fen(fen).unwrap();
    let before = position.clone();
    let error = position.apply_ordinary(action(text)).unwrap_err();
    assert_eq!(error, expected, "{text} from {fen}");
    assert!(!error.to_string().is_empty());
    assert_eq!(position, before, "{text}: rejected action changed state");
    assert_eq!(
        position.to_fen(),
        fen,
        "{text}: serialization changed on failure"
    );
}

#[test]
fn rejected_ordinary_preconditions_are_atomic() {
    use ActionError::*;
    for (text, expected) in [
        ("a3a4", EmptySource),
        ("a7a6", WrongSide),
        ("e2e2", SameSquare),
        ("a1a2", FriendlyDestination),
        ("e2e8", KingCapture),
        ("a1a3", IllegalGeometry),
        ("f1b5", IllegalGeometry),
        ("d1d3", IllegalGeometry),
        ("g1g3", IllegalGeometry),
        ("e1e3", IllegalGeometry),
        ("e2f3", IllegalGeometry),
        ("e2e5", IllegalGeometry),
        ("e2e3q", UnexpectedPromotion),
        ("g1f3q", UnexpectedPromotion),
    ] {
        assert_rejected(INITIAL, text, expected);
    }
    for (fen, text) in [
        ("7k/8/8/8/4P3/8/8/K7 w - - 0 1", "e4e3"), // Backwards pawn.
        ("7k/8/8/8/4P3/8/8/K7 w - - 0 1", "e4e6"), // Non-home double push.
        ("7k/8/8/8/8/4p3/4P3/K7 w - - 0 1", "e2e4"), // Blocked middle.
        ("7k/8/8/8/4p3/8/4P3/K7 w - - 0 1", "e2e4"), // Occupied destination.
        ("7k/8/8/8/8/4p3/4P3/K7 w - - 0 1", "e2e3"), // No straight pawn capture.
        ("7k/8/8/8/4K3/8/8/8 w - - 0 1", "e4g4"),  // Not standard castling.
        ("7k/8/8/8/3p4/8/8/K7 b - - 0 1", "d4d5"), // Black backwards.
        ("7k/8/8/8/3p4/8/8/K7 b - - 0 1", "d4d2"), // Black non-home double.
    ] {
        assert_rejected(fen, text, IllegalGeometry);
    }
    for (fen, text) in [
        ("1r5k/P7/8/8/8/8/8/7K w - - 0 1", "a7a8"),
        ("1r5k/P7/8/8/8/8/8/7K w - - 0 1", "a7b8"),
        ("7k/8/8/8/8/8/p7/1R5K b - - 0 1", "a2a1"),
        ("7k/8/8/8/8/8/p7/1R5K b - - 0 1", "a2b1"),
    ] {
        assert_rejected(fen, text, MissingPromotion);
    }
}

#[test]
fn castling_and_en_passant_are_explicitly_unsupported_without_effects() {
    use ActionError::*;
    for text in ["e1g1", "e1c1"] {
        assert_rejected(CASTLES, text, UnsupportedCastling);
    }
    let black_castles = CASTLES.replace(" w ", " b ");
    for text in ["e8g8", "e8c8"] {
        assert_rejected(&black_castles, text, UnsupportedCastling);
    }
    assert_rejected(
        "7k/8/8/3pP3/8/8/8/K7 w - d6 0 2",
        "e5d6",
        UnsupportedEnPassant,
    );
    assert_rejected(
        "7k/8/8/8/3Pp3/8/8/K7 b - d3 0 2",
        "e4d3",
        UnsupportedEnPassant,
    );
    // Without the right, a diagonal-to-empty pawn action is simply invalid.
    assert_rejected("7k/8/8/3pP3/8/8/8/K7 w - - 1 2", "e5d6", IllegalGeometry);
}

#[test]
fn unsafe_results_are_rejected_after_resolving_capture_and_movement() {
    for (fen, text) in [
        ("4r2k/8/8/8/8/8/4R3/4K3 w - - 0 1", "e2f2"), // Exposes own king.
        ("4r2k/8/8/8/8/8/4n3/4K3 w - - 0 1", "e1e2"), // Capture uncovers rook.
        ("7k/8/8/8/8/p7/8/K7 w - - 0 1", "a1b2"),     // Moves into pawn attack.
        ("8/8/8/8/8/2k5/8/K7 w - - 0 1", "a1b2"),     // Adjacent kings.
        ("4r2k/8/8/8/8/8/3B4/4K3 w - - 0 1", "d2c3"), // Fails to evade check.
        ("4k3/4n3/8/2K5/8/8/8/4R3 w - - 0 1", "c5d5"), // Pinned enemy still attacks.
    ] {
        let mut position = Position::from_fen(fen).unwrap();
        let before = position.clone();
        let error = position.apply_ordinary(action(text)).unwrap_err();
        assert!(
            matches!(error, ActionError::IllegalPosition(_)),
            "{text} from {fen}: {error}"
        );
        assert_eq!(position, before, "{text}");
    }
}

#[test]
fn counter_overflow_cannot_publish_any_piece_right_or_ep_effect() {
    use ActionError::CounterOverflow;
    assert_rejected(
        &INITIAL.replace(" 0 1", " 4294967295 1"),
        "g1f3",
        CounterOverflow,
    );
    let black = INITIAL
        .replace(" w ", " b ")
        .replace(" 0 1", " 0 4294967295");
    assert_rejected(&black, "g8f6", CounterOverflow);
    assert_rejected(&black, "a7a6", CounterOverflow);
    let with_ep = AFTER_E4.replace(" 0 1", " 0 4294967295");
    assert_rejected(&with_ep, "g8f6", CounterOverflow);
    let rook_capture = CASTLES
        .replace(" w ", " b ")
        .replace(" 0 1", " 12 4294967295");
    assert_rejected(&rook_capture, "a8a1", CounterOverflow);
}

#[test]
fn undo_requires_the_complete_post_state_and_rejects_wrong_order_atomically() {
    let mut position = Position::from_fen(INITIAL).unwrap();
    let first = position.apply_ordinary(action("e2e4")).unwrap();
    let after_first = position.clone();
    let second = position.apply_ordinary(action("e7e5")).unwrap();
    let after_second = position.clone();
    assert_eq!(position.undo(&first), Err(UndoError::PositionMismatch));
    assert_eq!(position, after_second);
    position.undo(&second).unwrap();
    assert_eq!(position, after_first);
    position.undo(&first).unwrap();
    assert_eq!(position.to_fen(), INITIAL);
    assert_eq!(position.undo(&first), Err(UndoError::PositionMismatch));
    assert_eq!(position.to_fen(), INITIAL);

    for other in [
        AFTER_E4.replace(" e3 ", " - "),
        AFTER_E4.replace(" 0 1", " 0 2"),
        AFTER_E4.replace("KQkq", "KQk"),
        AFTER_E4.replace("/8/PPPP1PPP/RNBQKBNR", "/5N2/PPPP1PPP/RNBQKB1R"),
    ] {
        let mut mismatched = Position::from_fen(&other).unwrap();
        let before = mismatched.clone();
        assert_eq!(mismatched.undo(&first), Err(UndoError::PositionMismatch));
        assert_eq!(mismatched, before);
    }
    let mut quiet = Position::from_fen(INITIAL).unwrap();
    let quiet_record = quiet.apply_ordinary(action("g1f3")).unwrap();
    for other in [
        quiet.to_fen().replace(" b ", " w "),
        quiet.to_fen().replace(" 1 1", " 2 1"),
    ] {
        let mut mismatched = Position::from_fen(&other).unwrap();
        let before = mismatched.clone();
        assert_eq!(
            mismatched.undo(&quiet_record),
            Err(UndoError::PositionMismatch)
        );
        assert_eq!(mismatched, before);
    }
    // Equality is local saved-state identity, not a nonce or proof of game origin.
    let mut identical_value = Position::from_fen(AFTER_E4).unwrap();
    identical_value.undo(&first).unwrap();
    assert_eq!(identical_value.to_fen(), INITIAL);
}
