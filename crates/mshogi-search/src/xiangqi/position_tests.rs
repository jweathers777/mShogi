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
