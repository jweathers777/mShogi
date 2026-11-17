//! Differential tests use the independent authoritative native Shogi rules.
use super::position::ShogiPosition;
use crate::alphabeta::{MATE_SCORE, SearchPosition, Stage};
use mshogi_core::shogi::{
    Action, Game, GameCommand, GameRequest, PieceKind, Position, Side, Square,
};

fn sorted(actions: impl IntoIterator<Item = Action>) -> Vec<String> {
    let mut s: Vec<_> = actions.into_iter().map(|a| a.to_string()).collect();
    s.sort();
    s
}
fn compare(p: &mut ShogiPosition, native: &Position) {
    assert_eq!(p.side_to_move(), native.side_to_move());
    for sq in Square::all() {
        assert_eq!(p.piece(sq), native.piece(sq), "{sq}");
    }
    for side in [Side::Sente, Side::Gote] {
        for kind in PieceKind::HAND_KINDS {
            assert_eq!(p.hand_count(side, kind), native.hand(side).count(kind));
        }
        let e = super::evaluate(native);
        assert_eq!(
            p.material(side),
            if side == Side::Sente { e.sente } else { e.gote }
        );
    }
    assert_eq!(p.hash(), p.hash_from_scratch());
    assert_eq!(p.hash(), ShogiPosition::from_position(native).hash());
    assert_eq!(p.in_check(), native.is_in_check(native.side_to_move()));
    let legal = p.legal_moves();
    assert_eq!(
        sorted(legal.iter().map(|&m| p.action(m))),
        sorted(native.legal_actions()),
        "{}",
        native.to_sfen()
    );
    let mut pseudo = Vec::new();
    p.generate(Stage::Tactical, &mut pseudo);
    let split = pseudo.len();
    p.generate(Stage::Quiet, &mut pseudo);
    for (i, &m) in pseudo.iter().enumerate() {
        assert_eq!(p.is_tactical(m), i < split);
        assert!(!pseudo[..i].contains(&m));
        assert_eq!(p.decode(ShogiPosition::encode(m)), Some(m));
        assert_eq!(p.find(p.action(m)), Some(m));
    }
    assert!(p.decode(0).is_none());
    assert!(p.decode(u32::MAX).is_none());
}
fn next(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}
#[test]
fn random_legal_actions_results_hashes_and_restoration_agree() {
    let (mut positions, mut drops, mut captures, mut promotions) = (0, 0, 0, 0);
    for seed in 1..=24u64 {
        let mut rng = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
        let mut native = Position::initial();
        let mut p = ShogiPosition::from_position(&native);
        for _ in 0..140 {
            compare(&mut p, &native);
            positions += 1;
            let legal = native.legal_actions();
            if legal.is_empty() {
                break;
            }
            let wanted: Vec<_> = legal
                .iter()
                .copied()
                .filter(|a| match a {
                    Action::Drop { .. } => seed.is_multiple_of(3),
                    Action::Move { to, promote, .. } => {
                        *promote || seed.is_multiple_of(2) && native.piece(*to).is_some()
                    }
                })
                .collect();
            let pool = if wanted.is_empty() || next(&mut rng).is_multiple_of(4) {
                &legal
            } else {
                &wanted
            };
            let action = pool[(next(&mut rng) % pool.len() as u64) as usize];
            match action {
                Action::Drop { .. } => drops += 1,
                Action::Move { to, promote, .. } => {
                    captures += usize::from(native.piece(to).is_some());
                    promotions += usize::from(promote);
                }
            }
            let m = p.find(action).unwrap();
            let before = p.hash();
            let score = p.evaluate();
            assert!(p.make(m));
            p.unmake();
            assert_eq!(p.hash(), before);
            assert_eq!(p.evaluate(), score);
            assert!(p.make(m));
            native.apply(action).unwrap();
        }
        compare(&mut p, &native);
    }
    assert!(positions > 2000, "{positions}");
    assert!(
        drops > 100 && captures > 100 && promotions > 30,
        "{drops} {captures} {promotions}"
    );
}
#[test]
fn initial_and_drop_promotion_perft_agree() {
    let native = Position::initial();
    let mut p = ShogiPosition::from_position(&native);
    // Established standard start counts, also independently checked with native generation.
    for (depth, count) in [(1, 30), (2, 900), (3, 25470)] {
        assert_eq!(p.perft(depth), count);
        assert_eq!(native.perft(depth).unwrap(), count);
    }
    for sfen in [
        "4k4/9/4P4/9/9/9/9/9/4K4 b Np 1",
        "3lkl3/3p1p3/4G4/9/9/9/9/9/K8 b P 1",
        "4k4/9/3+B5/9/9/9/5+r3/9/4K4 w S 1",
    ] {
        let native = Position::from_sfen(sfen).unwrap();
        let mut p = ShogiPosition::from_position(&native);
        compare(&mut p, &native);
        assert_eq!(p.perft(2), native.perft(2).unwrap(), "{sfen}");
    }
}
#[test]
fn pawn_drop_mate_dead_ranks_nifu_and_promotion_choices() {
    for (sfen, illegal, legal) in [
        ("3lkl3/3p1p3/4G4/9/9/9/9/9/K8 b P 1", "P*5b", "P*6d"),
        ("3lkl3/3p1p3/9/9/9/9/9/9/K8 b P 1", "P*1a", "P*5b"),
        ("4k4/9/9/9/4P4/9/9/9/K8 b PNL 1", "P*5d", "P*4d"),
        ("4k4/9/9/9/4P4/9/9/9/K8 b PNL 1", "N*4b", "N*4c"),
        ("4k4/9/9/9/4P4/9/9/9/K8 b PNL 1", "L*4a", "L*4b"),
        ("8k/9/9/9/9/9/4g4/3P1P3/3LKL3 w p 1", "P*5h", "P*6f"),
    ] {
        let native = Position::from_sfen(sfen).unwrap();
        let mut p = ShogiPosition::from_position(&native);
        compare(&mut p, &native);
        let actions = sorted(p.legal_moves().iter().map(|&m| p.action(m)));
        assert!(!actions.contains(&illegal.to_string()));
        assert!(actions.contains(&legal.to_string()), "{sfen}: {legal}");
    }
    let native = Position::from_sfen("4k4/3P5/4S4/9/9/9/9/9/K8 b - 1").unwrap();
    let mut p = ShogiPosition::from_position(&native);
    compare(&mut p, &native);
    let actions = sorted(p.legal_moves().iter().map(|&m| p.action(m)));
    for a in ["6b6a+", "5c5b", "5c5b+"] {
        assert!(actions.contains(&a.into()));
    }
    assert!(!actions.contains(&"6b6a".into()));
}
fn play_position(p: &mut ShogiPosition, action: &str) {
    let m = p.find(action.parse().unwrap()).unwrap();
    assert!(p.make(m), "{action}");
}
#[test]
fn path_repetition_and_perpetual_check_have_distinct_scores_and_null_barriers() {
    let native = Position::from_sfen("4k4/9/9/9/9/9/9/9/4K4 b - 1").unwrap();
    let mut p = ShogiPosition::from_position(&native);
    for a in ["5i6i", "5a6a", "6i5i", "6a5a"] {
        play_position(&mut p, a);
    }
    assert_eq!(p.rule_score(), Some(0));
    let hash = p.hash();
    p.make_null();
    assert_eq!(p.rule_score(), None);
    p.unmake_null();
    assert_eq!(p.hash(), hash);
    assert_eq!(p.rule_score(), Some(0));
    // Rook follows the opposing king across files, checking on every turn.
    let native = Position::from_sfen("4k4/9/5R3/9/9/9/9/9/K8 b - 1").unwrap();
    let mut p = ShogiPosition::from_position(&native);
    for a in ["4c5c", "5a4a", "5c4c", "4a5a"] {
        play_position(&mut p, a);
    }
    assert_eq!(p.rule_score(), Some(-MATE_SCORE + 4));
    for _ in 0..4 {
        p.unmake();
    }
    assert_eq!(p.hash(), ShogiPosition::from_position(&native).hash());
    assert_eq!(p.rule_score(), None);
}
#[test]
fn game_history_reconstructs_checks_and_fourfold_count() {
    let mut game = Game::new();
    for _ in 0..2 {
        for a in ["5i5h", "5a5b", "5h5i", "5b5a"] {
            let action: Action = a.parse().unwrap();
            game.submit(GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: GameCommand::Play(action),
            })
            .unwrap();
        }
    }
    let mut p = ShogiPosition::from_game(&game);
    assert_eq!(p.rule_score(), None);
    for a in ["5i5h", "5a5b", "5h5i", "5b5a"] {
        play_position(&mut p, a);
    }
    assert_eq!(p.rule_score(), Some(0));
}
#[test]
fn length_draw_does_not_override_no_move_loss() {
    let native = Position::from_sfen("4k4/9/9/9/9/9/9/9/4K4 b - 321").unwrap();
    assert_eq!(ShogiPosition::from_position(&native).rule_score(), Some(0));
    let native = Position::from_sfen("3lkl3/3pGp3/4G4/9/9/9/9/9/K8 w - 321").unwrap();
    assert!(native.legal_actions().is_empty());
    assert_eq!(
        ShogiPosition::from_position(&native).rule_score(),
        Some(-MATE_SCORE)
    );
}

#[test]
fn table_decode_accepts_exactly_generated_moves() {
    for sfen in [
        "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1",
        "4k4/9/3+P5/4S4/9/9/9/9/4K4 b RBNLP 1",
    ] {
        let native = Position::from_sfen(sfen).unwrap();
        let p = ShogiPosition::from_position(&native);
        let mut pseudo = Vec::new();
        p.generate(Stage::Tactical, &mut pseudo);
        p.generate(Stage::Quiet, &mut pseudo);
        let mut decoded = 0;
        for bits in 0..=65537 {
            if let Some(m) = p.decode(bits) {
                assert_eq!(ShogiPosition::encode(m), bits);
                assert!(pseudo.contains(&m));
                decoded += 1;
            }
        }
        assert_eq!(decoded, pseudo.len());
    }
}

#[test]
fn gote_perpetual_checker_loses_on_the_repeated_path() {
    // Rotate the Sente checking cycle through 180 degrees and swap owners.
    let mut native = Position::from_sfen("8k/9/9/9/9/9/3r5/9/4K4 w - 1").unwrap();
    let mut p = ShogiPosition::from_position(&native);
    for action in ["6g5g", "5i6i", "5g6g", "6i5i"] {
        native.apply(action.parse().unwrap()).unwrap();
        play_position(&mut p, action);
        compare(&mut p, &native);
    }
    assert_eq!(p.side_to_move(), Side::Gote);
    assert_eq!(p.rule_score(), Some(-MATE_SCORE + 4));
}

#[test]
fn repetition_cycle_crosses_captures_and_drops() {
    // Three Golds circulate between board and hands. The two captures and
    // two drops restore the exact board, hands and turn after six plies.
    let mut native = Position::from_sfen("8k/9/9/9/4Gg3/9/9/9/K8 b g 1").unwrap();
    let initial = native.repetition_identity();
    let mut p = ShogiPosition::from_position(&native);
    let initial_hash = p.hash();
    let cycle = ["5e4e", "G*5e", "G*5d", "5e5d", "4e5e", "5d4e"];
    for (index, action) in cycle.into_iter().enumerate() {
        native.apply(action.parse().unwrap()).unwrap();
        play_position(&mut p, action);
        compare(&mut p, &native);
        assert_eq!(p.rule_score(), (index == 5).then_some(0));
    }
    assert_eq!(native.repetition_identity(), initial);
    assert_eq!(p.hash(), initial_hash);
    for _ in cycle {
        p.unmake();
    }
    assert_eq!(p.hash(), initial_hash);
    assert_eq!(p.rule_score(), None);
}

#[test]
fn incremental_placement_matches_native_recalculation() {
    let mut native = Position::initial();
    let mut fast = ShogiPosition::from_position(&native);
    let mut rng = 0x187_6a09_e667_f3bc;
    for _ in 0..240 {
        let mut totals = [0i32; 2];
        for (i, side) in [Side::Sente, Side::Gote].into_iter().enumerate() {
            let mut king = None;
            for sq in Square::all() {
                if native
                    .piece(sq)
                    .is_some_and(|p| p.side == side && p.kind == PieceKind::King)
                {
                    king = Some(sq.index());
                }
            }
            for sq in Square::all() {
                if let Some(piece) = native.piece(sq).filter(|p| p.side == side) {
                    totals[i] += super::piece_value(piece.kind, piece.promoted);
                    if piece.kind == PieceKind::Pawn && !piece.promoted {
                        totals[i] += 4 * if i == 0 {
                            8 - sq.index() / 9
                        } else {
                            sq.index() / 9
                        } as i32;
                    }
                    let k = king.unwrap();
                    if sq.index() != k
                        && (sq.index() / 9).abs_diff(k / 9) <= 1
                        && (sq.index() % 9).abs_diff(k % 9) <= 1
                    {
                        totals[i] += 8;
                    }
                }
            }
            for kind in PieceKind::HAND_KINDS {
                totals[i] += i32::from(native.hand(side).count(kind))
                    * (super::piece_value(kind, false) + 10);
            }
        }
        let side = usize::from(native.side_to_move() == Side::Gote);
        assert_eq!(fast.evaluate(), totals[side] - totals[side ^ 1] + 10);
        let actions = native.legal_actions();
        if actions.is_empty() {
            break;
        }
        let action = actions[next(&mut rng) as usize % actions.len()];
        let m = fast.find(action).unwrap();
        assert!(fast.make(m));
        native.apply(action).unwrap();
    }
}

/// Fixed work profile, deliberately ignored during ordinary tests. The same
/// test body is run against frozen and candidate positions; no search or games.
#[test]
#[ignore]
fn profile_position_hot_path() {
    let catalog: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tools/strength/data/shogi-bench-v1.json"
    ))
    .unwrap();
    let mut roots = Vec::new();
    for item in catalog["positions"].as_array().unwrap() {
        let mut native = item["sfen"]
            .as_str()
            .map(|s| Position::from_sfen(s).unwrap())
            .unwrap_or_else(Position::initial);
        for action in item["moves"].as_array().unwrap() {
            native
                .apply(action.as_str().unwrap().parse().unwrap())
                .unwrap();
        }
        roots.push(ShogiPosition::from_position(&native));
    }
    // Warm only immutable tables, then keep the work count fixed.
    std::hint::black_box(roots[0].legal_moves());
    let started = std::time::Instant::now();
    let mut moves = 0u64;
    let mut checksum = 0i64;
    for _ in 0..2000 {
        for p in &mut roots {
            for m in p.legal_moves() {
                assert!(p.make(m));
                checksum += i64::from(std::hint::black_box(p.evaluate()));
                std::hint::black_box(p.hash());
                p.unmake();
                moves += 1;
            }
        }
    }
    println!(
        "position_profile roots={} iterations=2000 moves={moves} checksum={checksum} seconds={:.6}",
        roots.len(),
        started.elapsed().as_secs_f64()
    );
}
