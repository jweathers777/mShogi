use super::*;
use mshogi_core::chess::{ClaimPolicy, Position};

fn board(fen: &str) -> ChessPosition {
    ChessPosition::from_position(&Position::from_fen(fen).unwrap())
}

/// Deterministic generator for test walks.
pub(crate) struct TestRng(pub u64);
impl TestRng {
    pub(crate) fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}

#[test]
fn perft_counts_equal_published_values() {
    // Chess Programming Wiki perft results.
    for (fen, depth, nodes) in [
        (
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            4,
            197_281u64,
        ),
        (
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            3,
            97_862,
        ),
        ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 5, 674_624),
        (
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            4,
            422_333,
        ),
        (
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            3,
            62_379,
        ),
        (
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            3,
            89_890,
        ),
    ] {
        assert_eq!(board(fen).perft(depth), nodes, "{fen}");
    }
}

#[test]
#[ignore = "about one minute in a debug build; run in release for the deep counts"]
fn deep_perft_counts_equal_published_values() {
    for (fen, depth, nodes) in [
        (
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            6,
            119_060_324u64,
        ),
        (
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
            5,
            193_690_690,
        ),
        ("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", 7, 178_633_661),
        (
            "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
            5,
            15_833_292,
        ),
        (
            "rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8",
            5,
            89_941_194,
        ),
        (
            "r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10",
            5,
            164_075_551,
        ),
    ] {
        assert_eq!(board(fen).perft(depth), nodes, "{fen}");
    }
}

fn same_board(fast: &ChessPosition, native: &Position) {
    for sq in 0..64 {
        let expected = native.piece_at(square(sq)).map(|piece| {
            (
                usize::from(piece.color == Color::Black),
                kind_index(piece.kind),
            )
        });
        assert_eq!(
            fast.kind_at(sq),
            expected,
            "square {sq} in {}",
            native.to_fen()
        );
    }
    assert_eq!(fast.side_to_move(), native.side_to_move());
    let rights = native.castling_rights();
    assert_eq!(
        fast.castling,
        u8::from(rights.kingside(Color::White))
            | u8::from(rights.queenside(Color::White)) << 1
            | u8::from(rights.kingside(Color::Black)) << 2
            | u8::from(rights.queenside(Color::Black)) << 3
    );
    assert_eq!(
        fast.en_passant,
        native
            .en_passant_target()
            .map_or(NO_SQUARE, |sq| index(sq) as u8)
    );
    assert_eq!(u32::from(fast.halfmove), native.halfmove_clock());
    assert_eq!(fast.in_check(), native.in_check());
    // Incremental state equals a rebuild from the native position.
    let rebuilt = ChessPosition::from_position(native);
    assert_eq!(fast.hash, rebuilt.hash);
    assert_eq!(fast.hash, fast.computed_hash());
    assert_eq!(fast.material, rebuilt.material);
    assert_eq!(fast.placement, rebuilt.placement);
    assert_eq!(fast.phase, rebuilt.phase);
    assert_eq!(fast.pieces, rebuilt.pieces);
    assert_eq!(fast.occupied, rebuilt.occupied);
    assert_eq!(fast.checkers, rebuilt.checkers);
}

/// Walk random legal games in both implementations and compare legal moves,
/// resulting positions, pseudo-legality answers and exact take-back.
#[test]
fn random_walks_agree_with_native_rules() {
    let mut rng = TestRng(0x5EED_1234_ABCD_0001);
    let starts = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1",
        "r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1",
        "8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1",
        "4k3/1P4P1/8/3pP3/8/8/p6p/R3K2R w KQ d6 0 2",
    ];
    let mut positions = 0;
    for game in 0..60 {
        let mut native = Position::from_fen(starts[game % starts.len()]).unwrap();
        let mut fast = ChessPosition::from_position(&native);
        for _ in 0..120 {
            same_board(&fast, &native);
            let mut expected = native.legal_actions();
            let legal = fast.legal_moves();
            let mut actual: Vec<_> = legal.iter().map(|&m| fast.action(m)).collect();
            let key = |a: &Action| (index(a.from), index(a.to), a.promotion.map(|p| p as u8));
            expected.sort_by_key(key);
            actual.sort_by_key(key);
            assert_eq!(actual, expected, "{}", native.to_fen());
            // Pseudo-legality of arbitrary encodings equals membership in the
            // generated pseudo-legal set.
            let mut pseudo = Vec::new();
            fast.generate(Stage::Tactical, &mut pseudo);
            let tactical = pseudo.len();
            fast.generate(Stage::Quiet, &mut pseudo);
            for (i, &m) in pseudo.iter().enumerate() {
                assert_eq!(
                    fast.is_tactical(m),
                    i < tactical,
                    "{m:?} {}",
                    native.to_fen()
                );
                assert!(fast.is_pseudo_legal(m), "{m:?} {}", native.to_fen());
                assert_eq!(pseudo.iter().filter(|&&other| other == m).count(), 1);
            }
            // The check without a change of the position agrees with make.
            for &m in &pseudo[tactical..] {
                if fast.kind_at(m.from()).is_some_and(|(_, kind)| kind != PAWN) {
                    let legal_by_make = fast.make(m);
                    if legal_by_make {
                        fast.unmake();
                    }
                    assert_eq!(
                        fast.quiet_piece_move_is_legal(m),
                        legal_by_make,
                        "{m:?} {}",
                        native.to_fen()
                    );
                }
            }
            for _ in 0..300 {
                let m = Move((rng.next() & 0xffff) as u16);
                assert_eq!(
                    fast.is_pseudo_legal(m),
                    pseudo.contains(&m),
                    "{m:?} {}",
                    native.to_fen()
                );
            }
            if legal.is_empty() {
                break;
            }
            // Make and take back each legal move: exact restoration.
            let before = (
                fast.hash,
                fast.pieces,
                fast.material,
                fast.placement,
                fast.phase,
            );
            for &m in &legal {
                assert!(fast.make(m));
                fast.unmake();
                assert_eq!(
                    before,
                    (
                        fast.hash,
                        fast.pieces,
                        fast.material,
                        fast.placement,
                        fast.phase
                    )
                );
            }
            let m = legal[(rng.next() % legal.len() as u64) as usize];
            native.apply(fast.action(m)).unwrap();
            assert!(fast.make(m));
            positions += 1;
            if native.halfmove_clock() >= 100 {
                break;
            }
        }
    }
    assert!(positions > 3_000, "{positions}");
}

#[test]
fn exchange_evaluation_on_known_cases() {
    let find = |p: &ChessPosition, text: &str| {
        let action: Action = text.parse().unwrap();
        p.find(action).unwrap()
    };
    // Rook takes a pawn that a pawn defends: loses rook for pawn.
    let p = board("4k3/8/3p4/4p3/8/8/4R3/4K3 w - - 0 1");
    let m = find(&p, "e2e5");
    assert!(p.exchange_at_least(m, -400));
    assert!(!p.exchange_at_least(m, -399));
    // Undefended pawn: wins a pawn.
    let p = board("4k3/8/8/4p3/8/8/4R3/4K3 w - - 0 1");
    let m = find(&p, "e2e5");
    assert!(p.exchange_at_least(m, 100));
    assert!(!p.exchange_at_least(m, 101));
    // X-ray: doubled rooks against a pawn defended one time by a rook.
    // RxP RxR RxR: 100 - 500 + 500 = 100.
    let p = board("4r1k1/8/8/4p3/8/8/4R3/4R1K1 w - - 0 1");
    let m = find(&p, "e2e5");
    assert!(p.exchange_at_least(m, 100));
    assert!(!p.exchange_at_least(m, 101));
    // The king cannot recapture a defended piece: NxP, and KxN is not possible
    // because the rook defends the knight.
    let p = board("8/8/8/3k4/4p3/8/3N4/4R1K1 w - - 0 1");
    let m = find(&p, "d2e4");
    assert!(p.exchange_at_least(m, 100));
}

#[test]
fn repetition_and_move_count_rules_on_the_search_path() {
    let mut p = board("4k3/8/8/8/8/8/8/R3K3 w - - 0 1");
    let play = |p: &mut ChessPosition, text: &str| {
        let m = p.find(text.parse().unwrap()).unwrap();
        assert!(p.make(m));
    };
    assert_eq!(p.rule_score(), None);
    for text in ["a1a2", "e8e7", "a2a1", "e7e8"] {
        assert_eq!(p.rule_score(), None);
        play(&mut p, text);
    }
    // The start position of the search occurs again on the path.
    assert_eq!(p.rule_score(), Some(0));
    for _ in 0..4 {
        p.unmake();
    }
    assert_eq!(p.rule_score(), None);

    // A position that occurred one time before the root is not yet a draw when
    // it occurs the second time, but it is at the third occurrence.
    let mut game =
        AdjudicatedGame::from_fen("4k3/8/8/8/8/8/8/R3K3 w - - 0 1", ClaimPolicy::ExplicitOnly)
            .unwrap();
    for text in ["a1a2", "e8e7", "a2a1", "e7e8"] {
        game.apply(text.parse().unwrap()).unwrap();
    }
    let mut p = ChessPosition::from_game(&game);
    assert_eq!(p.rule_score(), None);
    for text in ["a1a2", "e8e7", "a2a1"] {
        play(&mut p, text);
        assert_eq!(p.rule_score(), None, "{text}");
    }
    play(&mut p, "e7e8");
    assert_eq!(p.rule_score(), Some(0));

    // At 99 halfmoves the side to move can claim with an intended quiet move.
    assert!(board("4k3/8/8/8/8/8/8/R3K3 w - - 99 80").draw_claimable());
    assert!(!board("4k3/8/8/8/8/8/8/R3K3 w - - 98 80").draw_claimable());
    // The only legal move is a capture: no claim.
    assert!(!board("k7/P1K5/8/8/8/8/8/8 b - - 99 80").draw_claimable());
    // Hundred halfmoves without progress; mate has precedence.
    assert_eq!(
        board("4k3/8/8/8/8/8/8/R3K3 w - - 100 80").rule_score(),
        Some(0)
    );
    assert_eq!(
        board("R3k3/8/4K3/8/8/8/8/8 b - - 100 80").rule_score(),
        None
    );
    // Dead material.
    assert_eq!(
        board("4k3/8/8/8/8/8/8/2B1K3 w - - 0 1").rule_score(),
        Some(0)
    );
    assert_eq!(board("4k3/8/8/8/8/8/8/1NB1K3 w - - 0 1").rule_score(), None);
}

#[test]
fn null_move_restores_the_position() {
    let mut p = board("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2");
    let before = (p.hash, p.side, p.en_passant, p.castling);
    assert!(p.null_move_safe());
    p.make_null();
    assert_ne!(p.hash, before.0);
    assert_eq!(p.hash, p.computed_hash());
    p.unmake_null();
    assert_eq!(before, (p.hash, p.side, p.en_passant, p.castling));
    assert!(!board("4k3/8/8/8/8/8/4P3/4K3 w - - 0 1").null_move_safe());
}
