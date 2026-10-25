//! MSH-13 action-boundary review evidence (see docs/review/m1-action-boundary.md).
//!
//! Part 1 exercises the real production chess boundaries directly. Part 2 is an
//! explicitly synthetic, test-only fixture: invented rules on a 16x16 board
//! that challenge the action/effect/undo contract beyond chess. It is not an
//! implementation of Chu shogi, Tenjiku shogi or any real game.

use std::collections::BTreeSet;

use mshogi_core::chess::{Action, ActionError, Position, Promotion, Square};
use serde_json::Value;

fn sq(name: &str) -> Square {
    let b = name.as_bytes();
    Square::new(b[0] - b'a', b[1] - b'1').unwrap()
}

fn act(from: &str, to: &str, promotion: Option<Promotion>) -> Action {
    Action {
        from: sq(from),
        to: sq(to),
        promotion,
    }
}

// ---------------------------------------------------------------- Part 1

#[test]
fn production_square_is_intentionally_limited_to_the_chess_board() {
    assert!(Square::new(7, 7).is_some());
    for (file, rank) in [(8, 0), (0, 8), (15, 15), (255, 0)] {
        assert_eq!(Square::new(file, rank), None, "({file}, {rank})");
    }
}

#[test]
fn chess_action_identity_needs_no_route_field() {
    // Corpus sanity check, not a proof: the rules argument in the review is
    // that the piece is fixed by `from`, each orthodox piece reaches a
    // destination one way, castling and en passant are unique shapes and the
    // promotion choice is explicit. Here no legal set repeats a triple.
    let corpus: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/chess/corpus.json")).unwrap();
    let mut positions = 0;
    for case in corpus["cases"].as_array().unwrap() {
        if case["operation"] != "position" {
            continue;
        }
        let position = Position::from_fen(case["input"]["fen"].as_str().unwrap()).unwrap();
        let legal = position.legal_actions();
        let keys: BTreeSet<(u8, u8, u8, u8, Option<u8>)> = legal
            .iter()
            .map(|a| {
                let promotion = a.promotion.map(|p| p as u8);
                (
                    a.from.file(),
                    a.from.rank(),
                    a.to.file(),
                    a.to.rank(),
                    promotion,
                )
            })
            .collect();
        assert_eq!(keys.len(), legal.len(), "{}", case["id"]);
        positions += 1;
    }
    assert_eq!(positions, 35);
}

/// Apply a real multi-effect action and check the hand-inspected result,
/// exact undo, and that the caller supplied only the player's choice.
fn check_effects(before_fen: &str, action: Action, after_fen: &str) {
    let mut position = Position::from_fen(before_fen).unwrap();
    let before = position.clone();
    let record = position.apply(action).unwrap();
    assert_eq!(position.to_fen(), after_fen);
    position.undo(&record).unwrap();
    assert_eq!(position, before);
}

#[test]
fn production_multi_effect_actions_derive_every_effect_from_the_board() {
    // Castling: king and rook move, both White rights are lost, halfmove +1.
    check_effects(
        "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 5 9",
        act("e1", "g1", None),
        "r3k2r/8/8/8/8/8/8/R4RK1 b kq - 6 9",
    );
    // En passant: the victim is removed from d5, not from the destination d6.
    check_effects(
        "7k/8/8/3pP3/8/8/8/K7 w - d6 0 2",
        act("e5", "d6", None),
        "7k/8/3P4/8/8/8/8/K7 b - - 0 2",
    );
    // Capturing promotion on a home rook: capture, replacement and the loss of
    // Black's queenside right all follow from one complete choice.
    check_effects(
        "r3k3/1P6/8/8/8/8/8/4K3 w q - 0 1",
        act("b7", "a8", Some(Promotion::Knight)),
        "N3k3/8/8/8/8/8/8/4K3 b - - 0 1",
    );
}

#[test]
fn production_rejection_is_atomic_and_needs_a_complete_choice() {
    for (fen, action, expected) in [
        // Castling through an attacked square.
        (
            "r3k2r/8/8/8/2b5/8/8/R3K2R w KQkq - 0 1",
            act("e1", "g1", None),
            ActionError::CastlingKingAttacked,
        ),
        // A last-rank pawn move without its promotion choice is incomplete.
        (
            "r3k3/1P6/8/8/8/8/8/4K3 w q - 0 1",
            act("b7", "a8", None),
            ActionError::MissingPromotion,
        ),
    ] {
        let mut position = Position::from_fen(fen).unwrap();
        let before = position.clone();
        assert_eq!(position.apply(action).unwrap_err(), expected);
        assert_eq!(position, before, "{fen}");
    }
    // En passant exposing the own king fails after all effects, atomically.
    let mut position = Position::from_fen("7k/8/8/r4pPK/8/8/8/8 w - f6 0 2").unwrap();
    let before = position.clone();
    assert!(matches!(
        position.apply(act("g5", "f6", None)),
        Err(ActionError::IllegalPosition(_))
    ));
    assert_eq!(position, before);
}

// ---------------------------------------------------------------- Part 2
//
// Synthetic rules (invented for this fixture only): a "strider" moves one or
// two king steps; each step onto an enemy captures it; a step onto an own
// piece is invalid; the second step must be adjacent to the first and may
// return to the origin. Everything else on the 16x16 board is inert.

#[path = "support/synthetic_strider.rs"]
mod synthetic;

use synthetic::{Board, CompoundAction, Effect, Kind, Piece, Rejection, Selection, Sq};

const STRIDER: Piece = Piece {
    side: 0,
    kind: Kind::Strider,
};
const ENEMY: Piece = Piece {
    side: 1,
    kind: Kind::Stone,
};
const OWN: Piece = Piece {
    side: 0,
    kind: Kind::Stone,
};

fn s(file: u8, rank: u8) -> Sq {
    Sq::new(file, rank).unwrap()
}

fn board_with(pieces: &[(Sq, Piece)]) -> Board {
    let mut board = Board::empty();
    for &(at, piece) in pieces {
        board.put(at, piece);
    }
    board
}

#[test]
fn synthetic_squares_beyond_the_chess_range_are_addressable() {
    for (file, rank) in [(16, 0), (0, 16), (255, 255)] {
        assert_eq!(Sq::new(file, rank), None);
    }
    assert_eq!(s(15, 15), Sq(255));
    let mut board = board_with(&[(s(14, 14), STRIDER), (s(15, 15), ENEMY)]);
    let (effects, _) = board
        .apply(&CompoundAction {
            from: s(14, 14),
            route: vec![s(15, 15)],
        })
        .unwrap();
    assert_eq!(
        effects,
        [
            Effect::Capture {
                at: s(15, 15),
                piece: ENEMY
            },
            Effect::Step { to: s(15, 15) }
        ]
    );
    assert_eq!(board.at(s(15, 15)), Some(STRIDER));
}

#[test]
fn equal_endpoints_with_distinct_routes_are_distinct_actions_with_distinct_effects() {
    let origin = s(8, 8);
    let start = board_with(&[(origin, STRIDER), (s(9, 9), ENEMY)]);
    let capture_and_return = CompoundAction {
        from: origin,
        route: vec![s(9, 9), origin],
    };
    let step_out_and_return = CompoundAction {
        from: origin,
        route: vec![s(7, 8), origin],
    };
    // Same start and end square, different complete actions and identities.
    assert_ne!(capture_and_return, step_out_and_return);
    assert_ne!(
        synthetic::encode(&capture_and_return),
        synthetic::encode(&step_out_and_return)
    );

    let mut a = start.clone();
    let (effects, _) = a.apply(&capture_and_return).unwrap();
    assert_eq!(
        effects,
        [
            Effect::Capture {
                at: s(9, 9),
                piece: ENEMY
            },
            Effect::Step { to: s(9, 9) },
            Effect::Step { to: origin }
        ]
    );
    assert_eq!((a.at(origin), a.at(s(9, 9))), (Some(STRIDER), None));

    let mut b = start.clone();
    let (effects, _) = b.apply(&step_out_and_return).unwrap();
    assert_eq!(
        effects,
        [Effect::Step { to: s(7, 8) }, Effect::Step { to: origin }]
    );
    assert_eq!((b.at(origin), b.at(s(9, 9))), (Some(STRIDER), Some(ENEMY)));
    assert_ne!(a, b, "the routes have different rule-derived results");
}

#[test]
fn multiple_rule_derived_effects_reverse_completely() {
    let start = board_with(&[(s(3, 3), STRIDER), (s(4, 3), ENEMY), (s(5, 3), ENEMY)]);
    let mut board = start.clone();
    let action = CompoundAction {
        from: s(3, 3),
        route: vec![s(4, 3), s(5, 3)],
    };
    let (effects, undo) = board.apply(&action).unwrap();
    // Ordered internal effects, in rule order along the route.
    assert_eq!(
        effects,
        [
            Effect::Capture {
                at: s(4, 3),
                piece: ENEMY
            },
            Effect::Step { to: s(4, 3) },
            Effect::Capture {
                at: s(5, 3),
                piece: ENEMY
            },
            Effect::Step { to: s(5, 3) }
        ]
    );
    assert_eq!(board.at(s(5, 3)), Some(STRIDER));
    assert_eq!((board.at(s(3, 3)), board.at(s(4, 3))), (None, None));
    assert_eq!(board.side, 1);
    board.undo(&undo).unwrap();
    assert_eq!(board, start);
    assert!(board.undo(&undo).is_err(), "repeated undo is rejected");
    assert_eq!(board, start);
}

#[test]
fn an_invalid_later_step_rejects_the_whole_action_atomically() {
    let start = board_with(&[(s(8, 8), STRIDER), (s(9, 9), ENEMY), (s(7, 7), OWN)]);
    for (route, expected) in [
        // The first step would capture, but the second is not adjacent.
        (vec![s(9, 9), s(11, 11)], Rejection::BadRoute),
        // The second step lands on an own piece.
        (vec![s(8, 7), s(7, 7)], Rejection::OwnPieceInRoute),
        (vec![], Rejection::BadRoute),
        (vec![s(9, 9), s(10, 10), s(11, 11)], Rejection::BadRoute),
    ] {
        let mut board = start.clone();
        let result = board.apply(&CompoundAction {
            from: s(8, 8),
            route,
        });
        assert_eq!(result.err(), Some(expected));
        assert_eq!(board, start);
    }
}

// Illustrates the API shape: `Selection` holds no board reference, so the
// non-committal property is guaranteed by the type design, not by this test.
#[test]
fn partial_selection_is_not_a_committed_turn() {
    let mut board = board_with(&[(s(8, 8), STRIDER), (s(9, 9), ENEMY)]);
    let before = board.clone();
    let mut selection = Selection {
        from: Some(s(8, 8)),
        ..Selection::default()
    };
    selection.route.push(s(9, 9));
    // Choosing the first step changes nothing; abandoning it changes nothing.
    assert_eq!(board, before);
    drop(selection);
    assert_eq!(board, before);
    // Only a complete committed action reaches the checked boundary.
    let mut selection = Selection {
        from: Some(s(8, 8)),
        ..Selection::default()
    };
    selection.route.extend([s(9, 9), s(8, 8)]);
    let action = selection.commit().unwrap();
    board.apply(&action).unwrap();
    assert_ne!(board, before);
}

#[test]
fn serialized_identity_round_trips_and_carries_no_effects() {
    let action = CompoundAction {
        from: s(15, 14),
        route: vec![s(15, 15), s(15, 14)],
    };
    let text = synthetic::encode(&action);
    assert_eq!(text, "239>255>239");
    assert_eq!(synthetic::decode(&text), Some(action));
    // Effect annotations are not part of the identity and are not accepted.
    assert_eq!(synthetic::decode("239>255>239 capture=255"), None);
    assert_eq!(synthetic::decode("239>256"), None);
}
