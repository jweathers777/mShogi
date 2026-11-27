//! Differential tests use the independent authoritative Shogi rules.
use super::position::{ShogiPosition, VALUE, face_index};
use crate::alphabeta::mate::{self, MateLimits, MateResult, MateTable};
use crate::alphabeta::{MATE_SCORE, SearchPosition, Stage};
use mshogi_core::perft::perft;
use mshogi_core::shogi::{
    Action, Game, GameCommand, GameRequest, PieceKind, Position, Side, Square,
};

fn sorted(actions: impl IntoIterator<Item = Action>) -> Vec<String> {
    let mut s: Vec<_> = actions.into_iter().map(|a| a.to_string()).collect();
    s.sort();
    s
}
fn compare(p: &mut ShogiPosition, rules_position: &Position) {
    assert_eq!(p.side_to_move(), rules_position.side_to_move());
    for sq in Square::all() {
        assert_eq!(p.piece(sq), rules_position.piece(sq), "{sq}");
    }
    for side in [Side::Sente, Side::Gote] {
        for kind in PieceKind::HAND_KINDS {
            assert_eq!(
                p.hand_count(side, kind),
                rules_position.hand(side).count(kind)
            );
        }
    }
    assert_eq!(p.hash(), p.hash_from_scratch());
    assert_eq!(
        p.hash(),
        ShogiPosition::from_position(rules_position).hash()
    );
    assert_eq!(
        p.in_check(),
        rules_position.is_in_check(rules_position.side_to_move())
    );
    // King-square safety feature against attack detection.
    let mut attacked = 0;
    p.features(&mut |index, value| {
        if index == super::eval::KING_ADJACENT_ATTACKED {
            attacked += value;
        }
    });
    let mut expected = 0;
    for (sign, side) in [
        (1, rules_position.side_to_move()),
        (-1, rules_position.side_to_move().opponent()),
    ] {
        let king = Square::all()
            .find(|&sq| {
                rules_position
                    .piece(sq)
                    .is_some_and(|q| q.side == side && q.kind == PieceKind::King)
            })
            .unwrap();
        for sq in Square::all() {
            let near = sq != king
                && (sq.index() / 9).abs_diff(king.index() / 9) <= 1
                && (sq.index() % 9).abs_diff(king.index() % 9) <= 1;
            if near && rules_position.is_square_attacked(sq, side.opponent()) {
                expected += sign;
            }
        }
    }
    assert_eq!(attacked, expected, "{}", rules_position.to_sfen());
    // The incremental search evaluation equals the full feature sum.
    assert_eq!(p.evaluate(), super::eval::score(p, &super::eval::DEFAULT));
    let legal = p.legal_moves();
    assert_eq!(
        sorted(legal.iter().map(|&m| p.action(m))),
        sorted(rules_position.legal_actions()),
        "{}",
        rules_position.to_sfen()
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
    compare_checks_and_evasions(p, &legal, &pseudo);
}
/// The check generator returns pseudo-legal moves without repeats and
/// every legal checking move; in check, the search generator (drops only
/// between the king and a single slider) keeps exactly the legal moves.
fn compare_checks_and_evasions(
    p: &mut ShogiPosition,
    legal: &[super::position::Move],
    pseudo: &[super::position::Move],
) -> (usize, bool) {
    let mut candidates = Vec::new();
    p.generate_checks(&mut candidates);
    for (i, m) in candidates.iter().enumerate() {
        assert!(pseudo.contains(m), "{:?}", p.action(*m));
        assert!(!candidates[..i].contains(m));
    }
    let mut checking = 0;
    for &m in legal {
        assert!(p.make(m));
        let check = p.in_check();
        p.unmake();
        if check {
            checking += 1;
            assert!(candidates.contains(&m), "missing check {}", p.action(m));
        }
    }
    let mut search = Vec::new();
    p.generate_search(Stage::Tactical, &mut search);
    p.generate_search(Stage::Quiet, &mut search);
    let mut kept = vec![];
    for m in search {
        if p.make(m) {
            p.unmake();
            kept.push(p.action(m).to_string());
        }
    }
    kept.sort();
    assert_eq!(kept, sorted(legal.iter().map(|&m| p.action(m))));
    (checking, p.in_check())
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
        let mut rules_position = Position::initial();
        let mut p = ShogiPosition::from_position(&rules_position);
        for _ in 0..140 {
            compare(&mut p, &rules_position);
            positions += 1;
            let legal = rules_position.legal_actions();
            if legal.is_empty() {
                break;
            }
            let wanted: Vec<_> = legal
                .iter()
                .copied()
                .filter(|a| match a {
                    Action::Drop { .. } => seed.is_multiple_of(3),
                    Action::Move { to, promote, .. } => {
                        *promote || seed.is_multiple_of(2) && rules_position.piece(*to).is_some()
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
                    captures += usize::from(rules_position.piece(to).is_some());
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
            rules_position.apply(action).unwrap();
        }
        compare(&mut p, &rules_position);
    }
    assert!(positions > 2000, "{positions}");
    assert!(
        drops > 100 && captures > 100 && promotions > 30,
        "{drops} {captures} {promotions}"
    );
}
#[test]
fn initial_and_drop_promotion_perft_agree() {
    let rules_position = Position::initial();
    let mut p = ShogiPosition::from_position(&rules_position);
    // Established standard start counts, also independently checked with generation.
    for (depth, count) in [(1, 30), (2, 900), (3, 25470)] {
        assert_eq!(p.perft(depth), count);
        assert_eq!(
            perft(&rules_position, depth, 1_000_000, || false)
                .unwrap()
                .leaves,
            count
        );
    }
    for sfen in [
        "4k4/9/4P4/9/9/9/9/9/4K4 b Np 1",
        "3lkl3/3p1p3/4G4/9/9/9/9/9/K8 b P 1",
        "4k4/9/3+B5/9/9/9/5+r3/9/4K4 w S 1",
    ] {
        let rules_position = Position::from_sfen(sfen).unwrap();
        let mut p = ShogiPosition::from_position(&rules_position);
        compare(&mut p, &rules_position);
        assert_eq!(
            p.perft(2),
            perft(&rules_position, 2, 1_000_000, || false)
                .unwrap()
                .leaves,
            "{sfen}"
        );
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
        let rules_position = Position::from_sfen(sfen).unwrap();
        let mut p = ShogiPosition::from_position(&rules_position);
        compare(&mut p, &rules_position);
        let actions = sorted(p.legal_moves().iter().map(|&m| p.action(m)));
        assert!(!actions.contains(&illegal.to_string()));
        assert!(actions.contains(&legal.to_string()), "{sfen}: {legal}");
    }
    let rules_position = Position::from_sfen("4k4/3P5/4S4/9/9/9/9/9/K8 b - 1").unwrap();
    let mut p = ShogiPosition::from_position(&rules_position);
    compare(&mut p, &rules_position);
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
    let rules_position = Position::from_sfen("4k4/9/9/9/9/9/9/9/4K4 b - 1").unwrap();
    let mut p = ShogiPosition::from_position(&rules_position);
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
    let rules_position = Position::from_sfen("4k4/9/5R3/9/9/9/9/9/K8 b - 1").unwrap();
    let mut p = ShogiPosition::from_position(&rules_position);
    for a in ["4c5c", "5a4a", "5c4c", "4a5a"] {
        play_position(&mut p, a);
    }
    assert_eq!(p.rule_score(), Some(-MATE_SCORE + 4));
    for _ in 0..4 {
        p.unmake();
    }
    assert_eq!(
        p.hash(),
        ShogiPosition::from_position(&rules_position).hash()
    );
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
    let rules_position = Position::from_sfen("4k4/9/9/9/9/9/9/9/4K4 b - 321").unwrap();
    assert_eq!(
        ShogiPosition::from_position(&rules_position).rule_score(),
        Some(0)
    );
    let rules_position = Position::from_sfen("3lkl3/3pGp3/4G4/9/9/9/9/9/K8 w - 321").unwrap();
    assert!(rules_position.legal_actions().is_empty());
    assert_eq!(
        ShogiPosition::from_position(&rules_position).rule_score(),
        Some(-MATE_SCORE)
    );
}

#[test]
fn table_decode_accepts_exactly_generated_moves() {
    for sfen in [
        "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1",
        "4k4/9/3+P5/4S4/9/9/9/9/4K4 b RBNLP 1",
    ] {
        let rules_position = Position::from_sfen(sfen).unwrap();
        let p = ShogiPosition::from_position(&rules_position);
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
    let mut rules_position = Position::from_sfen("8k/9/9/9/9/9/3r5/9/4K4 w - 1").unwrap();
    let mut p = ShogiPosition::from_position(&rules_position);
    for action in ["6g5g", "5i6i", "5g6g", "6i5i"] {
        rules_position.apply(action.parse().unwrap()).unwrap();
        play_position(&mut p, action);
        compare(&mut p, &rules_position);
    }
    assert_eq!(p.side_to_move(), Side::Gote);
    assert_eq!(p.rule_score(), Some(-MATE_SCORE + 4));
}

#[test]
fn a_capture_drop_cycle_restores_the_position_but_falls_outside_the_search_window() {
    // Three Golds circulate between board and hands. The two captures and
    // two drops restore the exact board, hands and turn after six plies.
    let mut rules_position = Position::from_sfen("8k/9/9/9/4Gg3/9/9/9/K8 b g 1").unwrap();
    let initial = rules_position.repetition_identity();
    let mut p = ShogiPosition::from_position(&rules_position);
    let initial_hash = p.hash();
    let cycle = ["5e4e", "G*5e", "G*5d", "5e5d", "4e5e", "5d4e"];
    for action in cycle {
        rules_position.apply(action.parse().unwrap()).unwrap();
        play_position(&mut p, action);
        compare(&mut p, &rules_position);
        assert_eq!(p.rule_score(), None);
    }
    assert_eq!(rules_position.repetition_identity(), initial);
    assert_eq!(p.hash(), initial_hash);
    for _ in cycle {
        p.unmake();
    }
    assert_eq!(p.hash(), initial_hash);
    assert_eq!(p.rule_score(), None);
}

#[test]
fn incremental_placement_matches_recalculation() {
    let mut rules_position = Position::initial();
    let mut fast = ShogiPosition::from_position(&rules_position);
    let mut rng = 0x187_6a09_e667_f3bc;
    for _ in 0..240 {
        let mut totals = [0i32; 2];
        for (i, side) in [Side::Sente, Side::Gote].into_iter().enumerate() {
            let mut king = None;
            for sq in Square::all() {
                if rules_position
                    .piece(sq)
                    .is_some_and(|p| p.side == side && p.kind == PieceKind::King)
                {
                    king = Some(sq.index());
                }
            }
            for sq in Square::all() {
                if let Some(piece) = rules_position.piece(sq).filter(|p| p.side == side) {
                    totals[i] += VALUE[face_index(piece.kind, piece.promoted)];
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
                totals[i] += i32::from(rules_position.hand(side).count(kind))
                    * (VALUE[face_index(kind, false)] + 10);
            }
        }
        let side = usize::from(rules_position.side_to_move() == Side::Gote);
        // INITIAL weights reproduce the earlier evaluation exactly.
        assert_eq!(
            super::eval::score(&fast, &super::eval::INITIAL),
            totals[side] - totals[side ^ 1] + 10
        );
        let actions = rules_position.legal_actions();
        if actions.is_empty() {
            break;
        }
        let action = actions[next(&mut rng) as usize % actions.len()];
        let m = fast.find(action).unwrap();
        assert!(fast.make(m));
        rules_position.apply(action).unwrap();
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
        let mut rules_position = item["sfen"]
            .as_str()
            .map(|s| Position::from_sfen(s).unwrap())
            .unwrap_or_else(Position::initial);
        for action in item["moves"].as_array().unwrap() {
            rules_position
                .apply(action.as_str().unwrap().parse().unwrap())
                .unwrap();
        }
        roots.push(ShogiPosition::from_position(&rules_position));
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

#[test]
fn check_generation_and_evasions_cover_drops_discoveries_and_promotions() {
    for sfen in [
        // Discovered checks by a lance, a rook and a bishop behind own pieces.
        "4k4/9/9/4S4/4L4/9/9/9/K8 b G 1",
        "4k4/9/4N4/9/4R4/9/9/9/K8 b - 1",
        "8k/9/6S2/9/4B4/9/9/9/K8 b P 1",
        // Promotion checks and checking drops of every hand piece.
        "4k4/9/9/3P1S3/9/9/9/9/K8 b RBGSNLP 1",
        "lnsgk2nl/1r4gb1/p1ppppspp/9/1p5P1/9/PPPPPPP1P/1BG4R1/LNS1KGSNL b P 1",
        // Single sliding check (interposition drops), double and adjacent checks.
        "4k4/9/9/9/4r4/9/9/9/4K4 b GSN 1",
        "4k4/9/9/9/9/9/9/9/K3r4 b GSNLP 1",
        "4k4/9/9/9/9/9/9/3g5/4K4 b GSN 1",
        "8k/9/1b5r1/9/9/9/9/9/7K1 b GSNP 1",
    ] {
        let rules_position = Position::from_sfen(sfen).unwrap();
        let mut p = ShogiPosition::from_position(&rules_position);
        compare(&mut p, &rules_position);
    }
}

#[test]
fn static_exchange_uses_board_and_hand_values() {
    let see = |sfen: &str, action: &str| {
        let p = ShogiPosition::from_position(&Position::from_sfen(sfen).unwrap());
        p.see(p.find(action.parse().unwrap()).unwrap())
    };
    // Pawn takes a Pawn defended by a Gold: equal trade, and the promoted
    // Pawn (board 500, hand 100) is lost again.
    assert_eq!(see("4k4/4g4/4p4/4P4/9/9/9/9/4K4 b - 1", "5d5c+"), 0);
    assert_eq!(see("4k4/4g4/4p4/4P4/9/9/9/9/4K4 b - 1", "5d5c"), 0);
    // An undefended Pawn: board value plus hand value.
    assert_eq!(see("4k4/9/4p4/4P4/9/9/9/9/4K4 b - 1", "5d5c"), 200);
    // Rook takes a King-defended Pawn and promotes: the Dragon (board 1000,
    // hand Rook 800) is lost for Pawn and promotion gain.
    assert_eq!(
        see("4k4/4p4/9/9/9/4R4/9/9/K8 b - 1", "5f5b+"),
        200 + 200 - 1800
    );
    assert_eq!(see("4k4/3s5/4p4/9/9/4R4/9/9/K8 b - 1", "5f5c"), 200 - 1600);
    // A Gold drop attacked by a Pawn and defended by a Silver loses the
    // Gold for a Pawn; an undefended drop square is safe.
    assert_eq!(see("4k4/9/4p4/9/4S4/9/9/9/K8 b G 1", "G*5d"), 200 - 1000);
    assert_eq!(see("4k4/9/9/9/9/9/9/9/K8 b G 1", "G*5e"), 0);
    // The King cannot recapture on a defended square.
    assert_eq!(see("4k4/9/9/9/4p4/4P4/4R4/9/K8 b - 1", "5f5e"), 200);
    // X-ray: the Rook behind the Pawn defends the target square.
    assert_eq!(
        see("4k4/4r4/9/4p4/4P4/4R4/9/9/K8 b - 1", "5e5d"),
        // Gote does not recapture: the Rook behind would then win its Rook.
        200
    );
}

fn mate_search(p: &mut ShogiPosition, nodes: u64) -> MateResult<super::position::Move> {
    let mut table = MateTable::new(16);
    let limits = MateLimits {
        nodes,
        deadline: None,
    };
    let before = p.hash();
    let (result, _) = mate::search(p, &mut table, limits, &mut || false);
    assert_eq!(p.hash(), before);
    result
}

#[test]
fn mate_search_finds_known_tsume_and_respects_pawn_drop_mate() {
    let suite: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tools/strength/data/shogi-tactics-v1.json"
    ))
    .unwrap();
    for row in suite["positions"].as_array().unwrap() {
        let rules_position = Position::from_sfen(row["sfen"].as_str().unwrap()).unwrap();
        let mut p = ShogiPosition::from_position(&rules_position);
        let MateResult::Mate(line) = mate_search(&mut p, 100_000) else {
            panic!("no mate found in {}", row["id"]);
        };
        let first = p.action(line[0]).to_string();
        let expected: Vec<_> = row["expected_moves"].as_array().unwrap().iter().collect();
        assert!(
            expected.iter().any(|e| e.as_str() == Some(&first)),
            "{} {first}",
            row["id"]
        );
        // The line is legal under rules and ends in mate.
        let mut rules_position = rules_position.clone();
        for &m in &line {
            rules_position.apply(p.action(m)).unwrap();
            assert!(p.make(m));
        }
        assert!(
            rules_position.legal_actions().is_empty()
                && rules_position.is_in_check(rules_position.side_to_move())
        );
    }
    // Pawn drop to 1b would mate at once and is illegal; no other mate.
    let sfen = "8k/9/6NG1/9/9/9/9/9/K8 b P 1";
    let rules_position = Position::from_sfen(sfen).unwrap();
    assert!(!rules_position.is_legal("P*1b".parse().unwrap()));
    let mut p = ShogiPosition::from_position(&rules_position);
    match mate_search(&mut p, 200_000) {
        MateResult::Mate(line) => {
            assert_ne!(p.action(line[0]).to_string(), "P*1b");
            let mut rules_position = rules_position.clone();
            for &m in &line {
                rules_position.apply(p.action(m)).unwrap();
            }
            assert!(rules_position.legal_actions().is_empty());
        }
        MateResult::NoMate | MateResult::Unknown => {}
    }
}

#[test]
fn mate_search_rejects_a_perpetual_check_cycle() {
    // A lone Rook can check forever but never mate.
    let rules_position = Position::from_sfen("4k4/9/5R3/9/9/9/9/9/K8 b - 1").unwrap();
    let mut p = ShogiPosition::from_position(&rules_position);
    assert_eq!(mate_search(&mut p, 500_000), MateResult::NoMate);
    // The same after a recorded cycle: the next repetition is a rule result.
    for a in ["4c5c", "5a4a", "5c4c", "4a5a"] {
        play_position(&mut p, a);
    }
    assert!(!matches!(mate_search(&mut p, 500_000), MateResult::Mate(_)));
}

#[test]
fn mate_search_progresses_when_the_table_drops_results() {
    // A tiny table shared by all positions fills with solved entries and must
    // drop unsolved work; the search must still finish each proof.
    let suite: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tools/strength/data/shogi-tsume-v1.json"
    ))
    .unwrap();
    let mut table = MateTable::new(4);
    let rows = suite["positions"].as_array().unwrap();
    for row in rows
        .iter()
        .filter(|r| r["mate_plies"].as_u64() <= Some(5))
        .take(40)
    {
        let rules_position = Position::from_sfen(row["sfen"].as_str().unwrap()).unwrap();
        let mut p = ShogiPosition::from_position(&rules_position);
        let limits = MateLimits {
            nodes: 200_000,
            deadline: None,
        };
        let (result, nodes) = mate::search(&mut p, &mut table, limits, &mut || false);
        assert!(nodes < 200_000, "{} used the whole budget", row["id"]);
        assert!(!matches!(result, MateResult::NoMate), "{}", row["id"]);
    }
}
