//! Native differential checks and independently specified Xiangqi fixtures.
use super::position::XiangqiPosition;
use crate::alphabeta::{MATE_SCORE, SearchPosition, Stage};
use mshogi_core::xiangqi::{
    Action, ClaimPolicy, Game, GameCommand, GameRequest, Intersection, Position, PositionRoot, Side,
};

fn sorted(actions: impl IntoIterator<Item = Action>) -> Vec<String> {
    let mut result: Vec<_> = actions.into_iter().map(|a| a.to_string()).collect();
    result.sort();
    result
}
fn compare(p: &mut XiangqiPosition, native: &Position) {
    assert_eq!(p.side_to_move(), native.side_to_move());
    for at in Intersection::all() {
        assert_eq!(p.piece(at), native.piece(at), "{at}");
    }
    for side in [Side::Red, Side::Black] {
        let material: i32 = native
            .pieces()
            .filter(|(_, p)| p.side == side)
            .map(|(_, p)| super::piece_value(p.kind))
            .sum();
        assert_eq!(p.material(side), material);
    }
    let fresh = XiangqiPosition::from_position(native);
    assert_eq!(p.hash(), p.hash_from_scratch());
    assert_eq!(p.hash(), fresh.hash());
    assert_eq!(p.evaluate(), fresh.evaluate());
    assert_eq!(p.in_check(), native.is_in_check(native.side_to_move()));
    let legal = p.legal_moves();
    assert_eq!(
        sorted(legal.iter().map(|&m| p.action(m))),
        sorted(native.legal_actions().unwrap()),
        "{}",
        native.to_fen().unwrap()
    );
    let mut pseudo = Vec::new();
    p.generate(Stage::Tactical, &mut pseudo);
    let split = pseudo.len();
    p.generate(Stage::Quiet, &mut pseudo);
    assert_eq!(
        sorted(pseudo.iter().map(|&m| p.action(m))),
        sorted(native.pseudo_legal_actions().unwrap())
    );
    for (i, &m) in pseudo.iter().enumerate() {
        assert_eq!(p.is_tactical(m), i < split);
        assert!(!pseudo[..i].contains(&m));
        assert_eq!(p.decode(XiangqiPosition::encode(m)), Some(m));
        assert_eq!(p.find(p.action(m)), Some(m));
    }
}
fn next(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}
#[test]
fn random_legal_actions_results_hashes_and_restoration_agree() {
    let (mut positions, mut captures) = (0, 0);
    for seed in 1..=24u64 {
        let mut rng = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
        let mut native = Position::initial();
        let mut p = XiangqiPosition::from_position(&native);
        for _ in 0..140 {
            compare(&mut p, &native);
            positions += 1;
            let legal = native.legal_actions().unwrap();
            if legal.is_empty() {
                break;
            }
            let wanted: Vec<_> = legal
                .iter()
                .copied()
                .filter(|a| native.piece(a.to).is_some())
                .collect();
            let pool =
                if wanted.is_empty() || !seed.is_multiple_of(2) || next(&mut rng).is_multiple_of(4)
                {
                    &legal
                } else {
                    &wanted
                };
            let action = pool[(next(&mut rng) % pool.len() as u64) as usize];
            captures += usize::from(native.piece(action.to).is_some());
            let m = p.find(action).unwrap();
            let before = p.hash();
            let score = p.evaluate();
            assert!(p.make(m));
            p.unmake();
            assert_eq!(p.hash(), before);
            assert_eq!(p.evaluate(), score);
            compare(&mut p, &native);
            assert!(p.make(m));
            native.apply(action).unwrap();
        }
        compare(&mut p, &native);
    }
    assert!(positions > 2000, "{positions}");
    assert!(captures > 200, "{captures}");
}
#[test]
fn existing_independent_board_fixtures_and_perft_agree() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/fixtures/xiangqi/board.json"
    ))
    .unwrap();
    let mut checked = 0;
    for fixture in fixtures["fixtures"].as_array().unwrap() {
        if fixture["domain"] != "live_root" {
            continue;
        }
        let native = Position::from_fen(fixture["fen"].as_str().unwrap()).unwrap();
        let mut p = XiangqiPosition::from_position(&native);
        compare(&mut p, &native);
        let actual = sorted(p.legal_moves().iter().map(|&m| p.action(m)));
        let strings = |value: &serde_json::Value| {
            let mut v: Vec<String> = value
                .as_array()
                .unwrap()
                .iter()
                .map(|a| a.as_str().unwrap().to_owned())
                .collect();
            v.sort();
            v
        };
        if fixture["legal_moves"].is_array() {
            assert_eq!(
                actual,
                strings(&fixture["legal_moves"]),
                "{}",
                fixture["id"]
            );
        }
        if let Some(froms) = fixture["moves_from"].as_object() {
            for (from, expected) in froms {
                assert_eq!(
                    actual
                        .iter()
                        .filter(|a| a.starts_with(from))
                        .cloned()
                        .collect::<Vec<_>>(),
                    strings(expected),
                    "{}: {from}",
                    fixture["id"]
                );
            }
        }
        for (field, expected) in [("must_include", true), ("must_exclude", false)] {
            if fixture[field].is_array() {
                for a in strings(&fixture[field]) {
                    assert_eq!(actual.contains(&a), expected, "{}: {a}", fixture["id"]);
                }
            }
        }
        if let Some(check) = fixture["in_check"].as_bool() {
            assert_eq!(p.in_check(), check);
        }
        if let Some(perft) = fixture["perft"].as_object() {
            for (depth, count) in perft {
                let depth: u8 = depth.parse().unwrap();
                let expected = count.as_u64().unwrap();
                assert_eq!(
                    p.perft(u32::from(depth)),
                    expected,
                    "{} depth{depth}",
                    fixture["id"]
                );
                assert_eq!(native.perft(depth).unwrap(), expected);
            }
        }
        checked += 1;
    }
    assert!(checked >= 30, "{checked}");
    let native = Position::initial();
    let mut p = XiangqiPosition::from_position(&native);
    for depth in 1..=3 {
        assert_eq!(p.perft(depth.into()), native.perft(depth).unwrap());
    }
}
fn play(p: &mut XiangqiPosition, native: &mut Position, text: &str) {
    let action: Action = text.parse().unwrap();
    let m = p.find(action).unwrap();
    native.apply(action).unwrap();
    assert!(p.make(m));
    compare(p, native);
}
#[test]
fn continuous_checker_loses_for_both_colors_and_null_move_is_a_barrier() {
    for (fen, cycle, side) in [
        (
            "4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1",
            ["e9f9", "f8e8", "f9e9", "e8f8"],
            Side::Red,
        ),
        (
            "5k3/9/9/9/9/9/9/9/3K5/4r4 b - - 0 1",
            ["e0d0", "d1e1", "d0e0", "e1d1"],
            Side::Black,
        ),
    ] {
        let mut native = Position::from_fen(fen).unwrap();
        let mut p = XiangqiPosition::from_position(&native);
        for action in cycle {
            play(&mut p, &mut native, action);
        }
        assert_eq!(p.side_to_move(), side);
        assert_eq!(p.rule_score(), Some(-MATE_SCORE + 4));
        let (hash, eval) = (p.hash(), p.evaluate());
        p.make_null();
        assert_eq!(p.rule_score(), None);
        p.unmake_null();
        assert_eq!(p.hash(), hash);
        assert_eq!(p.evaluate(), eval);
        assert_eq!(p.rule_score(), Some(-MATE_SCORE + 4));
        for _ in cycle {
            p.unmake();
        }
        assert_eq!(p.rule_score(), None);
    }
}
#[test]
fn ordinary_cycle_draws_and_history_events_reconstruct() {
    let mut game = Game::standard(ClaimPolicy::Explicit).unwrap();
    let mut p = XiangqiPosition::from_game(&game);
    let cycle = ["b0c2", "b9c7", "c2b0", "c7b9"];
    for text in cycle {
        let action: Action = text.parse().unwrap();
        let m = p.find(action).unwrap();
        assert!(p.make(m));
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play { action },
        })
        .unwrap();
    }
    assert_eq!(p.rule_score(), Some(0));
    let reconstructed = XiangqiPosition::from_game(&game);
    assert_eq!(reconstructed.hash(), p.hash());
    assert_eq!(reconstructed.rule_score(), None);
    let mut reconstructed = reconstructed;
    for text in cycle {
        let m = reconstructed.find(text.parse().unwrap()).unwrap();
        assert!(reconstructed.make(m));
    }
    assert_eq!(reconstructed.rule_score(), Some(0));
    // History includes native automatic warning events after checking cycles.
    let mut game = Game::from_root(
        PositionRoot::from_fen("4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1").unwrap(),
        ClaimPolicy::Explicit,
    )
    .unwrap();
    for text in ["e9f9", "f8e8", "f9e9", "e8f8"].iter().cycle().take(11) {
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play {
                action: text.parse().unwrap(),
            },
        })
        .unwrap();
    }
    assert!(game.events().len() > 11);
    let mut p = XiangqiPosition::from_game(&game);
    compare(&mut p, game.position());
    let m = p.find("e8f8".parse().unwrap()).unwrap();
    assert!(p.make(m));
    assert_eq!(p.rule_score(), Some(-MATE_SCORE + 1));
}
#[test]
fn stalemate_is_a_loss_before_bare_general_draw() {
    for fen in [
        "5k3/4R4/9/9/9/9/9/9/9/3K5 b - - 0 1",
        "3k5/9/9/9/9/9/9/9/4r4/5K3 w - - 0 1",
    ] {
        let native = Position::from_fen(fen).unwrap();
        assert!(!native.is_in_check(native.side_to_move()));
        assert!(native.legal_actions().unwrap().is_empty());
        assert_eq!(
            XiangqiPosition::from_position(&native).rule_score(),
            Some(-MATE_SCORE)
        );
    }
    let native = Position::from_fen("5k3/9/9/9/9/9/9/9/9/3K5 w - - 0 1").unwrap();
    assert_eq!(
        XiangqiPosition::from_position(&native).rule_score(),
        Some(0)
    );
}
#[test]
fn table_decode_accepts_exactly_generated_moves() {
    let p = XiangqiPosition::from_position(&Position::initial());
    let mut pseudo = Vec::new();
    p.generate(Stage::Tactical, &mut pseudo);
    p.generate(Stage::Quiet, &mut pseudo);
    let mut decoded = 0;
    for bits in 0..=65537 {
        if let Some(m) = p.decode(bits) {
            assert_eq!(XiangqiPosition::encode(m), bits);
            assert!(pseudo.contains(&m));
            decoded += 1;
        }
    }
    assert_eq!(decoded, pseudo.len());
}
#[test]
fn reverse_attack_scan_agrees_with_every_piece_target_on_random_games() {
    let mut checked = 0;
    for seed in 1..=40u64 {
        let mut rng = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
        let mut native = Position::initial();
        let mut p = XiangqiPosition::from_position(&native);
        for _ in 0..160 {
            for target in 0..90 {
                for by in 0..2 {
                    assert_eq!(
                        p.attacked_fast(target, by),
                        p.attacked_by_targets(target, by),
                        "seed {seed} target {target} by {by}"
                    );
                    checked += 1;
                }
            }
            let legal = native.legal_actions().unwrap();
            if legal.is_empty() {
                break;
            }
            let action = legal[(next(&mut rng) % legal.len() as u64) as usize];
            assert!(p.make(p.find(action).unwrap()));
            native.apply(action).unwrap();
        }
    }
    assert!(checked > 500_000, "{checked}");
}
#[test]
fn a_one_sided_chase_cycle_loses_for_the_chaser_and_mutual_or_protected_cycles_draw() {
    // Red Chariot chases the unprotected Black Horse between b7 and c9:
    // c3b3 attacks b7 along the b-file, b3c3 attacks c9 along the c-file. The
    // Horse's own moves attack nothing. Every Red move chases: Red loses.
    let fen = "4k4/9/1n7/9/9/9/2R6/9/9/3K5 w - - 0 1";
    let mut native = Position::from_fen(fen).unwrap();
    let mut p = XiangqiPosition::from_position(&native);
    for action in ["c3b3", "b7c9", "b3c3", "c9b7"] {
        play(&mut p, &mut native, action);
    }
    assert_eq!(p.side_to_move(), Side::Red);
    assert_eq!(p.rule_score(), Some(-MATE_SCORE + 4));
    // The same cycle from Black's side to move: Red, not the mover, loses.
    p.unmake();
    native = Position::from_fen(fen).unwrap();
    let mut q = XiangqiPosition::from_position(&native);
    for action in ["c3b3", "b7c9", "b3c3", "c9b7", "c3b3"] {
        play(&mut q, &mut native, action);
    }
    assert_eq!(q.side_to_move(), Side::Black);
    assert_eq!(q.rule_score(), Some(MATE_SCORE - 5));
    // Protected Horse: a Black Chariot on a7 guards b7 along rank 7 and one
    // on a9 guards c9 along rank 9. The Red moves are not chases: draw.
    let protected = "r3k4/9/rn7/9/9/9/2R6/9/9/3K5 w - - 0 1";
    let mut native = Position::from_fen(protected).unwrap();
    let mut r = XiangqiPosition::from_position(&native);
    for action in ["c3b3", "b7c9", "b3c3", "c9b7"] {
        play(&mut r, &mut native, action);
    }
    assert_eq!(r.rule_score(), Some(0));
}
#[test]
fn static_exchange_follows_changing_cannon_screens() {
    // a-file: Black Chariot a9, Black Horse a5, Red Chariot a1, Red Cannon a0.
    // Cannon takes first (screen a1): Horse; if the Chariot recaptures the
    // Cannon, the Red Chariot takes it, so Black stops: +Horse. Chariot takes
    // first: +Horse, Chariot recaptures, and the Cannon has lost its screen:
    // Horse - Chariot.
    let p = XiangqiPosition::from_position(
        &Position::from_fen("r3k4/9/9/9/n8/9/9/9/R8/C2K5 w - - 0 1").unwrap(),
    );
    let cannon = p.find("a0a5".parse().unwrap()).unwrap();
    let chariot = p.find("a1a5".parse().unwrap()).unwrap();
    // Values are the compiled material weights.
    use super::eval::{DEFAULT, MATERIAL};
    let (horse, rook, soldier) = (
        DEFAULT[MATERIAL + 2],
        DEFAULT[MATERIAL + 3],
        DEFAULT[MATERIAL + 5],
    );
    assert_eq!(p.see(cannon), horse);
    assert_eq!(p.see(chariot), horse - rook);
    // An undefended Horse: +Horse. A Soldier on e8 defended by the Black
    // Advisor and General, taken by a Chariot from e1: Soldier - Chariot.
    let p = XiangqiPosition::from_position(
        &Position::from_fen("3ak4/4p4/9/9/n8/9/9/9/9/R2K5 w - - 0 1").unwrap(),
    );
    assert_eq!(p.see(p.find("a0a5".parse().unwrap()).unwrap()), horse);
    let p = XiangqiPosition::from_position(
        &Position::from_fen("3ak4/4p4/9/9/9/9/9/9/4R4/3K5 w - - 0 1").unwrap(),
    );
    assert_eq!(
        p.see(p.find("e1e8".parse().unwrap()).unwrap()),
        soldier - rook
    );
}
#[test]
fn check_generator_contains_every_legal_checking_move_on_random_games() {
    use crate::alphabeta::SearchPosition;
    let mut checks = 0;
    for seed in 1..=40u64 {
        let mut rng = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
        let mut native = Position::initial();
        let mut p = XiangqiPosition::from_position(&native);
        for _ in 0..160 {
            let mut generated = Vec::new();
            p.generate_checks(&mut generated);
            for action in native.legal_actions().unwrap() {
                let mut after = native.clone();
                after.apply(action).unwrap();
                if after.is_in_check(after.side_to_move()) {
                    checks += 1;
                    let m = p.find(action).unwrap();
                    assert!(generated.contains(&m), "seed {seed} missing check {action}");
                }
            }
            let legal = native.legal_actions().unwrap();
            if legal.is_empty() {
                break;
            }
            let action = legal[(next(&mut rng) % legal.len() as u64) as usize];
            assert!(p.make(p.find(action).unwrap()));
            native.apply(action).unwrap();
        }
    }
    assert!(checks > 1_000, "{checks}");
}
#[test]
fn reverse_least_attacker_agrees_with_a_value_ordered_target_scan() {
    // On random games, the SEE of every capture equals a reference swap
    // computed from full target scans (independent attacker search).
    use crate::alphabeta::{SearchPosition, Stage};
    let mut compared = 0;
    for seed in 1..=30u64 {
        let mut rng = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
        let mut native = Position::initial();
        let mut p = XiangqiPosition::from_position(&native);
        for _ in 0..140 {
            let mut captures = Vec::new();
            p.generate(Stage::Tactical, &mut captures);
            for m in captures {
                assert_eq!(p.see(m), p.see_by_target_scan(m), "seed {seed}");
                compared += 1;
            }
            let legal = native.legal_actions().unwrap();
            if legal.is_empty() {
                break;
            }
            let action = legal[(next(&mut rng) % legal.len() as u64) as usize];
            assert!(p.make(p.find(action).unwrap()));
            native.apply(action).unwrap();
        }
    }
    assert!(compared > 2_000, "{compared}");
}
#[test]
fn lazy_search_chase_flags_equal_flags_computed_while_replaying_history() {
    let mut compared = 0;
    for seed in 1..=20u64 {
        let mut rng = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
        let mut game = Game::standard(ClaimPolicy::Explicit).unwrap();
        let mut lazy = XiangqiPosition::from_game(&game);
        for _ in 0..120 {
            if game.conclusion().is_some() || game.pending_adjudication().is_some() {
                break;
            }
            let legal = game.position().legal_actions().unwrap();
            if legal.is_empty() {
                break;
            }
            let action = legal[(next(&mut rng) % legal.len() as u64) as usize];
            game.submit(GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: GameCommand::Play { action },
            })
            .unwrap();
            assert!(lazy.make(lazy.find(action).unwrap()));
        }
        let eager = XiangqiPosition::from_game(&game);
        let flags = lazy.chase_flags(1);
        assert_eq!(flags, eager.chases[1..].to_vec(), "seed {seed}");
        compared += flags.len();
    }
    assert!(compared > 1_000, "{compared}");
}
#[test]
fn a_chase_cycle_that_starts_in_game_history_uses_stored_and_lazy_flags() {
    // The chase cycle of the test above. Six moves are native game history
    // (flags stored during replay): the start position occurs twice there.
    // Two search moves (lazy flags) give its third occurrence, which a cycle
    // reaching into history needs.
    let root = PositionRoot::from_fen("4k4/9/1n7/9/9/9/2R6/9/9/3K5 w - - 0 1").unwrap();
    let mut game = Game::from_root(root, ClaimPolicy::Explicit).unwrap();
    for text in ["c3b3", "b7c9", "b3c3", "c9b7", "c3b3", "b7c9"] {
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play {
                action: text.parse().unwrap(),
            },
        })
        .unwrap();
    }
    let mut p = XiangqiPosition::from_game(&game);
    for text in ["b3c3", "c9b7"] {
        let m = p.find(text.parse().unwrap()).unwrap();
        assert!(p.make(m));
    }
    // Red to move at ply 2 from the root; every Red move chased: Red loses.
    assert_eq!(p.rule_score(), Some(-MATE_SCORE + 2));
}
#[test]
fn evaluation_equals_the_feature_sum_with_compiled_and_initial_weights() {
    use super::eval::{DEFAULT, score};
    let mut checked = 0;
    for seed in 1..=20u64 {
        let mut rng = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
        let mut native = Position::initial();
        let mut p = XiangqiPosition::from_position(&native);
        for _ in 0..150 {
            if !p.in_check() {
                assert_eq!(p.evaluate(), score(&p, &DEFAULT), "seed {seed}");
                // A fresh position from the native board has the same value.
                let fresh = XiangqiPosition::from_position(&native);
                assert_eq!(fresh.evaluate(), p.evaluate());
                checked += 1;
            }
            let legal = native.legal_actions().unwrap();
            if legal.is_empty() {
                break;
            }
            let action = legal[(next(&mut rng) % legal.len() as u64) as usize];
            assert!(p.make(p.find(action).unwrap()));
            native.apply(action).unwrap();
        }
    }
    assert!(checked > 1_500, "{checked}");
}
#[test]
fn initial_weights_equal_the_earlier_hand_set_formula_on_random_games() {
    // The predecessor's evaluation, copied as written before MSH-196:
    // material plus placement formulas and a tempo of 10 for the mover.
    use super::eval::{INITIAL, score};
    fn old(native: &Position) -> i32 {
        let mut total = [0i32; 2];
        for (at, piece) in native.pieces() {
            let side = usize::from(piece.side == Side::Black);
            let index = at.index();
            let rank = if side == 0 { index / 9 } else { 9 - index / 9 } as i32;
            let central = 4 - (index % 9).abs_diff(4) as i32;
            let (value, placement) = match piece.kind {
                mshogi_core::xiangqi::PieceKind::General => (0, 0),
                mshogi_core::xiangqi::PieceKind::Advisor => (200, 0),
                mshogi_core::xiangqi::PieceKind::Elephant => (200, 0),
                mshogi_core::xiangqi::PieceKind::Horse => (400, 6 * central + 4 * rank.min(6)),
                mshogi_core::xiangqi::PieceKind::Rook => (900, 3 * central + 2 * rank),
                mshogi_core::xiangqi::PieceKind::Cannon => (450, 4 * central + 2 * rank.min(6)),
                mshogi_core::xiangqi::PieceKind::Soldier => (
                    100,
                    20 * (rank - 3).max(0) + if rank >= 5 { 8 * central } else { 0 },
                ),
            };
            total[side] += value + placement;
        }
        let me = usize::from(native.side_to_move() == Side::Black);
        total[me] - total[me ^ 1] + 10
    }
    let mut checked = 0;
    for seed in 1..=20u64 {
        let mut rng = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
        let mut native = Position::initial();
        let mut p = XiangqiPosition::from_position(&native);
        for _ in 0..150 {
            assert_eq!(score(&p, &INITIAL), old(&native), "seed {seed}");
            checked += 1;
            let legal = native.legal_actions().unwrap();
            if legal.is_empty() {
                break;
            }
            let action = legal[(next(&mut rng) % legal.len() as u64) as usize];
            assert!(p.make(p.find(action).unwrap()));
            native.apply(action).unwrap();
        }
    }
    assert!(checked > 2_000, "{checked}");
}
#[test]
fn colour_mirrored_positions_have_equal_evaluations_and_features() {
    // Mirror a position top-bottom and swap colours: the side to move sees
    // the same position, so every feature and the evaluation must agree.
    use super::eval::{COUNT, INITIAL, score};
    fn mirror(fen: &str) -> String {
        let fields: Vec<&str> = fen.split(' ').collect();
        let board: Vec<String> = fields[0]
            .split('/')
            .rev()
            .map(|row| {
                row.chars()
                    .map(|c| {
                        if c.is_ascii_uppercase() {
                            c.to_ascii_lowercase()
                        } else {
                            c.to_ascii_uppercase()
                        }
                    })
                    .collect()
            })
            .collect();
        let side = if fields[1] == "w" { "b" } else { "w" };
        format!("{} {} - - 0 1", board.join("/"), side)
    }
    let mut checked = 0;
    for seed in 1..=10u64 {
        let mut rng = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
        let mut native = Position::initial();
        for _ in 0..120 {
            let fen = native.to_fen().unwrap();
            let a = XiangqiPosition::from_position(&Position::from_fen(&fen).unwrap());
            let b = XiangqiPosition::from_position(&Position::from_fen(&mirror(&fen)).unwrap());
            let (mut fa, mut fb) = ([0i32; COUNT], [0i32; COUNT]);
            a.features(&mut |i, v| fa[i] += v);
            b.features(&mut |i, v| fb[i] += v);
            assert_eq!(fa, fb, "{fen}");
            assert_eq!(a.evaluate(), b.evaluate());
            assert_eq!(score(&a, &INITIAL), score(&b, &INITIAL));
            checked += 1;
            let legal = native.legal_actions().unwrap();
            if legal.is_empty() {
                break;
            }
            let action = legal[(next(&mut rng) % legal.len() as u64) as usize];
            native.apply(action).unwrap();
        }
    }
    assert!(checked > 800, "{checked}");
}
#[test]
fn line_tables_equal_the_ray_walk_for_every_square_on_random_games() {
    let mut checked = 0;
    for seed in 1..=30u64 {
        let mut rng = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
        let mut native = Position::initial();
        let mut p = XiangqiPosition::from_position(&native);
        for _ in 0..150 {
            for from in 0..90 {
                for (kind, capture) in [(4, true), (4, false), (5, true), (5, false)] {
                    assert_eq!(
                        p.slider_targets(kind, from, capture),
                        p.slider_targets_by_walk(kind, from, capture),
                        "seed {seed} from {from} kind {kind} capture {capture}"
                    );
                    checked += 1;
                }
            }
            let legal = native.legal_actions().unwrap();
            if legal.is_empty() {
                break;
            }
            let action = legal[(next(&mut rng) % legal.len() as u64) as usize];
            assert!(p.make(p.find(action).unwrap()));
            native.apply(action).unwrap();
        }
    }
    assert!(checked > 1_000_000, "{checked}");
}
#[test]
fn filtered_legality_and_check_flags_equal_native_for_every_move_on_random_games() {
    let mut checked = 0;
    for seed in 1..=30u64 {
        let mut rng = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
        let mut native = Position::initial();
        let mut p = XiangqiPosition::from_position(&native);
        for _ in 0..150 {
            let legal = native.legal_actions().unwrap();
            let mut moves = Vec::new();
            p.generate(Stage::Tactical, &mut moves);
            p.generate(Stage::Quiet, &mut moves);
            for m in moves {
                let action = p.action(m);
                let ok = p.make(m);
                assert_eq!(ok, legal.contains(&action), "seed {seed} {action}");
                if ok {
                    let mut after = native.clone();
                    after.apply(action).unwrap();
                    assert_eq!(
                        p.in_check(),
                        after.is_in_check(after.side_to_move()),
                        "seed {seed} {action}"
                    );
                    p.unmake();
                    checked += 1;
                }
            }
            if legal.is_empty() {
                break;
            }
            let action = legal[(next(&mut rng) % legal.len() as u64) as usize];
            assert!(p.make(p.find(action).unwrap()));
            native.apply(action).unwrap();
        }
    }
    assert!(checked > 100_000, "{checked}");
}
