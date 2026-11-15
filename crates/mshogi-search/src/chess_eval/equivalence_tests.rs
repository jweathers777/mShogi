//! Differential checks against the frozen pre-optimization weight evaluator.
use super::{reference_weights as reference, *};
use crate::{alphabeta::SearchPosition, chess_bitboard::tests::TestRng};
use mshogi_core::chess::{Action, AdjudicatedGame, ClaimPolicy, Color, Position};

fn compare(fast: &ChessPosition, native: &Position, extra_weights: bool) {
    // Reconstruct the reference from the native board so incremental errors in
    // the candidate do not also enter the expected result.
    let independent = ChessPosition::from_position(native);
    let expected = reference::Prepared::new(&independent);
    let actual = weights::Prepared::new(fast);
    let fen = native.to_fen();
    assert_eq!(fast.hash(), independent.hash(), "hash: {fen}");
    assert_eq!(fast.side_to_move(), native.side_to_move(), "side: {fen}");
    let count = if extra_weights { 4 } else { 1 };
    for values in [
        weights::DEFAULT.0,
        weights::INITIAL.0,
        weights::LOWER,
        weights::UPPER,
    ]
    .into_iter()
    .take(count)
    {
        let old_weights = reference::Weights(values);
        let new_weights = weights::Weights(values);
        let old = expected.decompose(&old_weights);
        let new = actual.decompose(&new_weights);
        assert_eq!(new.white, old.white, "every White term: {fen}");
        assert_eq!(new.black, old.black, "every Black term: {fen}");
        assert_eq!(new.middle_phase, old.middle_phase, "phase: {fen}");
        assert_eq!(new.endgame_scale, old.endgame_scale, "scale: {fen}");
        assert_eq!(new.specialized, old.specialized, "specialized score: {fen}");
        assert_eq!(
            new.white_score(),
            old.white_score(),
            "final White score: {fen}"
        );
        assert_eq!(
            actual.score(&new_weights),
            expected.score(&old_weights),
            "prepared score: {fen}"
        );
    }
    let white = expected.score(&reference::DEFAULT);
    let side = if native.side_to_move() == Color::White {
        white
    } else {
        -white
    };
    assert_eq!(
        evaluate(fast),
        side,
        "production side-relative score: {fen}"
    );
}

// Vertical reflection plus color exchange retains castling geometry. Preserve
// the clock fields and transform raw en-passant targets as well.
fn reflect(native: &Position) -> Position {
    let fen = native.to_fen();
    let f: Vec<_> = fen.split_whitespace().collect();
    let board = f[0]
        .split('/')
        .rev()
        .map(|rank| {
            rank.chars()
                .map(|c| {
                    if c.is_ascii_uppercase() {
                        c.to_ascii_lowercase()
                    } else {
                        c.to_ascii_uppercase()
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("/");
    let rights: String = [('k', 'K'), ('q', 'Q'), ('K', 'k'), ('Q', 'q')]
        .into_iter()
        .filter_map(|(old, new)| f[2].contains(old).then_some(new))
        .collect();
    let rights = if rights.is_empty() { "-" } else { &rights };
    let ep = if f[3] == "-" {
        "-".to_owned()
    } else {
        let bytes = f[3].as_bytes();
        format!("{}{}", bytes[0] as char, (b'9' - bytes[1] + b'0') as char)
    };
    Position::from_fen(&format!(
        "{board} {} {rights} {ep} {} {}",
        if f[1] == "w" { "b" } else { "w" },
        f[4],
        f[5]
    ))
    .unwrap()
}

#[test]
fn frozen_evaluator_matches_twenty_thousand_native_walk_positions_and_undos() {
    assert_eq!(
        weights::DEFAULT.0,
        reference::DEFAULT.0,
        "cost-only change keeps fitted weights"
    );
    let mut rng = TestRng(0x177_2026_1115);
    let mut checked = 0;
    let mut colors = [0; 2];
    while checked < 20_000 {
        let mut game = AdjudicatedGame::new(ClaimPolicy::AutoClaim);
        let mut fast = ChessPosition::from_game(&game);
        let mut played = 0;
        for _ in 0..160 {
            compare(&fast, game.position(), checked % 31 == 0);
            colors[usize::from(game.position().side_to_move() == Color::Black)] += 1;
            checked += 1;
            if checked % 16 == 0 {
                let mirrored = reflect(game.position());
                compare(&ChessPosition::from_position(&mirrored), &mirrored, true);
            }
            if checked >= 20_000 || game.outcome().is_some() || game.policy_claim().is_some() {
                break;
            }
            let legal = game.position().legal_actions();
            if legal.is_empty() {
                break;
            }
            let action = legal[(rng.next() % legal.len() as u64) as usize];
            let m = fast
                .find(action)
                .expect("native legal move exists in fast position");
            assert!(fast.make(m));
            game.apply(action).unwrap();
            played += 1;
        }
        // Restore the actual incremental position all the way to the root.
        for _ in 0..played {
            fast.unmake();
            game.undo().expect("native undo");
            compare(&fast, game.position(), false);
        }
    }
    assert!(checked >= 20_000);
    assert!(
        colors.iter().all(|&n| n >= 9_000),
        "both colors covered: {colors:?}"
    );
}

#[test]
fn frozen_evaluator_matches_special_moves_reflections_and_undo() {
    for (fen, action) in [
        ("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", "e1g1"),
        ("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", "e1c1"),
        ("4k3/8/8/3pP3/8/8/8/4K3 w - d6 0 1", "e5d6"),
        ("4k3/P7/8/8/8/8/8/4K3 w - - 0 1", "a7a8q"),
        ("4k3/P7/8/8/8/8/8/4K3 w - - 0 1", "a7a8r"),
        ("4k3/P7/8/8/8/8/8/4K3 w - - 0 1", "a7a8b"),
        ("4k3/P7/8/8/8/8/8/4K3 w - - 0 1", "a7a8n"),
        ("1r2k3/P7/8/8/8/8/8/4K3 w - - 0 1", "a7b8q"),
    ] {
        let original = Position::from_fen(fen).unwrap();
        for reflected in [false, true] {
            let mut native = if reflected {
                reflect(&original)
            } else {
                original.clone()
            };
            let text: String = action
                .chars()
                .map(|c| {
                    if reflected && c.is_ascii_digit() {
                        char::from(b'9' - c as u8 + b'0')
                    } else {
                        c
                    }
                })
                .collect();
            let action: Action = text.parse().unwrap();
            assert!(
                native.legal_actions().contains(&action),
                "fixture {fen}: {text}"
            );
            let mut fast = ChessPosition::from_position(&native);
            compare(&fast, &native, true);
            let m = fast.find(action).unwrap();
            assert!(fast.make(m));
            let undo = native.apply(action).unwrap();
            compare(&fast, &native, true);
            fast.unmake();
            native.undo(&undo).unwrap();
            compare(&fast, &native, true);
        }
    }
}

#[test]
fn rejected_pinned_moves_restore_every_evaluation_term() {
    for (fen, action) in [
        ("k3r3/8/8/8/8/8/4R3/4K3 w - - 0 1", "e2d2"),
        ("4k3/4r3/8/8/8/8/8/K3R3 b - - 0 1", "e7d7"),
    ] {
        let native = Position::from_fen(fen).unwrap();
        let mut fast = ChessPosition::from_position(&native);
        let action: Action = action.parse().unwrap();
        assert!(!native.legal_actions().contains(&action));
        let pseudo = fast.find(action).expect("pinned pseudo-legal rook move");
        assert!(!fast.make(pseudo));
        compare(&fast, &native, true);
    }
}
