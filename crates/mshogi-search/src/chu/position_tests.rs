//! Independent profile fixtures plus native local differential verification.
use super::position::ChuPosition;
use crate::alphabeta::{MATE_SCORE, SearchPosition, Stage};
use mshogi_core::chu::{
    Action, Game, GameCommand, GameRequest, LocalPosition, Piece, Position, RootMetadata, Side,
    Square,
};
use serde_json::Value;
use std::collections::BTreeSet;
fn fixtures() -> Vec<Value> {
    serde_json::from_str::<Value>(include_str!("../../../../docs/m4/chu/fixtures.json")).unwrap()["fixtures"].as_array().unwrap().clone()
}
fn sq(v: &Value) -> Square {
    serde_json::from_value(v.clone()).unwrap()
}
fn root(v: &Value) -> LocalPosition {
    if v == "standard_start" {
        return LocalPosition::standard();
    }
    let pieces: Vec<_> = v["board"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            (
                sq(&p["at"]),
                Piece::from_code(
                    serde_json::from_value(p["side"].clone()).unwrap(),
                    p["piece"].as_str().unwrap(),
                )
                .unwrap(),
            )
        })
        .collect();
    LocalPosition::admit(
        Position::from_diagram(
            &pieces,
            serde_json::from_value(v["turn"].clone()).unwrap(),
            RootMetadata::diagram(v["declared_board_plies"].as_u64().unwrap()),
        )
        .unwrap(),
    )
}
fn action(v: &Value) -> Action {
    Action {
        from: sq(&v["from"]),
        route: v["route"].as_array().unwrap().iter().map(sq).collect(),
        promote: v["promote"].as_bool().unwrap(),
    }
}
fn actions(p: &mut ChuPosition) -> BTreeSet<Action> {
    p.legal_moves()
        .into_iter()
        .map(|m| p.to_native(m))
        .collect()
}
fn compare_state(p: &ChuPosition, native: &LocalPosition) {
    assert_eq!(p.side_to_move(), native.position().side_to_move());
    for at in Square::all() {
        assert_eq!(p.piece(at), native.position().piece(at), "{at}");
    }
    assert_eq!(p.protection(), *native.protection());
    assert_eq!(p.hash(), p.hash_from_scratch());
    let fresh = ChuPosition::from_local(native);
    assert_eq!(p.hash(), fresh.hash());
    assert_eq!(p.evaluate(), fresh.evaluate());
    assert_eq!(p.in_check(), fresh.in_check());
    let e = super::evaluate(native.position());
    assert_eq!(p.material(Side::Sente), e.sente);
    assert_eq!(p.material(Side::Gote), e.gote);
}
fn compare(p: &mut ChuPosition, native: &LocalPosition) {
    compare_state(p, native);
    let legal = native.legal_actions();
    assert_eq!(actions(p), legal.into_iter().collect());
    let mut pseudo = Vec::new();
    p.generate(Stage::Tactical, &mut pseudo);
    let split = pseudo.len();
    p.generate(Stage::Quiet, &mut pseudo);
    let mut seen = BTreeSet::new();
    for (i, &m) in pseudo.iter().enumerate() {
        assert!(seen.insert(m), "duplicate {m:?}");
        assert_eq!(p.is_tactical(m), i < split);
        assert_eq!(p.find(&p.to_native(m)), Some(m));
        assert_eq!(p.decode(ChuPosition::encode(m)), Some(m));
    }
    assert!(p.decode(0).is_none());
    assert!(p.decode(u32::MAX).is_none());
}
fn probe(p: &mut ChuPosition, native: &mut LocalPosition, a: &Action, expected: bool, id: &str) {
    let before_hash = p.hash();
    let native_before = native.clone();
    let native_result = native.apply(a);
    assert_eq!(native_result.is_ok(), expected, "native {id}: {a:?}");
    let accepted = p.find(a).is_some_and(|m| p.make(m));
    assert_eq!(accepted, expected, "search {id}: {a:?}");
    if !accepted {
        assert_eq!(p.hash(), before_hash);
        assert_eq!(*native, native_before);
    }
    compare(p, native);
}
#[test]
fn all_independent_local_profile_fixtures_agree() {
    let mut count = 0;
    let mut lion = 0;
    for f in fixtures() {
        if !f["given"]["root"].is_object() && f["given"]["root"] != "standard_start" {
            continue;
        }
        if f["clauses"]
            .as_array()
            .unwrap()
            .iter()
            .any(|c| !matches!(c.as_str(), Some("C01" | "C02" | "C03" | "C04" | "C05")))
        {
            continue;
        }
        let id = f["id"].as_str().unwrap();
        let mut native = root(&f["given"]["root"]);
        let mut p = ChuPosition::from_local(&native);
        compare(&mut p, &native);
        count += 1;
        lion += usize::from(id.starts_with("CHU-LION"));
        if f["given"]["actor"].is_array() {
            let actor = sq(&f["given"]["actor"]);
            let chosen: BTreeSet<_> = actions(&mut p)
                .into_iter()
                .filter(|a| a.from == actor)
                .collect();
            if let Some(expected) = f["expected"]["empty_board_nonorigin_landing_count"].as_u64() {
                let landings: BTreeSet<_> = chosen
                    .iter()
                    .filter_map(|a| a.to())
                    .filter(|&to| to != actor)
                    .collect();
                assert_eq!(landings.len() as u64, expected, "{id}");
            }
            if let Some(expected) = f["expected"]["actor_complete_actions"].as_array() {
                assert_eq!(chosen, expected.iter().map(action).collect(), "{id}");
                assert_eq!(
                    chosen.len() as u64,
                    f["expected"]["actor_action_count"].as_u64().unwrap()
                );
            }
        }

        for (field, expected_field) in [
            ("legal_probe", "legal_probe_admitted"),
            ("illegal_probe", "illegal_probe_admitted"),
        ] {
            if f["given"][field].is_object() {
                let mut n = native.clone();
                let mut q = p.clone();
                probe(
                    &mut q,
                    &mut n,
                    &action(&f["given"][field]),
                    f["expected"][expected_field].as_bool().unwrap(),
                    id,
                );
            }
        }
        if f["given"]["action"].is_object()
            && let Some(expected) = f["expected"]["admitted"].as_bool()
        {
            let a = action(&f["given"]["action"]);
            probe(&mut p, &mut native, &a, expected, id);
            if expected && let Some(piece) = f["expected"]["result_piece"].as_str() {
                assert_eq!(p.piece(a.to().unwrap()).unwrap().code(), piece, "{id}");
            }
        }
        if let Some(commands) = f["given"]["commands"].as_array() {
            for (i, command) in commands.iter().enumerate() {
                if command["kind"] != "move" {
                    break;
                }
                let expected = if f["expected"]["all_admitted"] == true {
                    Some(true)
                } else {
                    f["expected"][if i == 0 {
                        "first_admitted"
                    } else {
                        "second_admitted"
                    }]
                    .as_bool()
                };
                let Some(expected) = expected else {
                    break;
                };
                probe(&mut p, &mut native, &action(command), expected, id);
                if i == 0 {
                    for field in [
                        "protected_lions_after_first",
                        "transient_protection_after_first",
                    ] {
                        if f["expected"][field].is_array() {
                            let expected: BTreeSet<_> = f["expected"][field]
                                .as_array()
                                .unwrap()
                                .iter()
                                .map(sq)
                                .collect();
                            assert_eq!(
                                p.protection().squares.into_iter().collect::<BTreeSet<_>>(),
                                expected,
                                "{id}"
                            );
                        }
                    }
                }
                if !expected {
                    break;
                }
            }
        }
    }
    assert!(count > 150, "{count}");
    assert!(lion >= 15, "{lion}");
}
fn native_perft(native: &LocalPosition, depth: u32) -> u64 {
    if depth == 0 {
        return 1;
    }
    native
        .legal_actions()
        .iter()
        .map(|a| {
            let mut next = native.clone();
            next.apply(a).unwrap();
            native_perft(&next, depth - 1)
        })
        .sum()
}
#[test]
fn independent_remote_perft_and_native_compound_perft_agree() {
    let fs = fixtures();
    for id in [
        "CHU-PERFT-REMOTE",
        "CHU-MOVE-Ln",
        "CHU-MOVE-SE",
        "CHU-MOVE-HF",
        "CHU-LION-NONLION-Ki",
    ] {
        let f = fs.iter().find(|f| f["id"] == id).unwrap();
        let native = root(&f["given"]["root"]);
        let mut p = ChuPosition::from_local(&native);
        for depth in 1..=2 {
            let n = native_perft(&native, depth);
            assert_eq!(p.perft(depth), n, "{id} depth{depth}");
            if id == "CHU-PERFT-REMOTE" {
                assert_eq!(n, if depth == 1 { 7 } else { 42 });
            }
        }
    }
    let native = LocalPosition::standard();
    let mut p = ChuPosition::from_local(&native);
    assert_eq!(p.perft(2), native_perft(&native, 2));
}
fn next(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}
#[test]
fn random_native_actions_positions_protection_hashes_and_undo_agree() {
    let (mut positions, mut captures, mut compounds, mut promotions) = (0, 0, 0, 0);
    for seed in 1..=20u64 {
        let mut rng = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
        let mut native = LocalPosition::standard();
        let mut p = ChuPosition::from_local(&native);
        for _ in 0..120 {
            compare(&mut p, &native);
            positions += 1;
            if [Side::Sente, Side::Gote].iter().any(|&s| {
                !native
                    .position()
                    .pieces()
                    .any(|(_, p)| p.side == s && p.is_royal())
            }) {
                break;
            }
            let legal = native.legal_actions();
            if legal.is_empty() {
                break;
            }
            let desired: Vec<_> = legal
                .iter()
                .filter(|a| {
                    a.promote
                        || a.route.iter().any(|&to| {
                            to != a.from
                                && native.position().piece(to).is_some_and(|piece| {
                                    piece.side != native.position().side_to_move()
                                })
                        })
                })
                .collect();
            let a = if !desired.is_empty()
                && seed.is_multiple_of(2)
                && !next(&mut rng).is_multiple_of(4)
            {
                desired[(next(&mut rng) % desired.len() as u64) as usize].clone()
            } else {
                legal[(next(&mut rng) % legal.len() as u64) as usize].clone()
            };
            compounds += usize::from(a.route.len() == 2);
            promotions += usize::from(a.promote);
            let m = p.find(&a).unwrap();
            let hash = p.hash();
            let evaluation = p.evaluate();
            let protection = p.protection();
            assert!(p.make(m));
            let (effects, _) = native.apply(&a).unwrap();
            captures += effects.captures.len();
            compare_state(&p, &native);
            p.unmake();
            assert_eq!(p.hash(), hash);
            assert_eq!(p.evaluate(), evaluation);
            assert_eq!(p.protection(), protection);
            assert!(p.make(m));
        }
    }
    assert!(positions > 1500, "{positions}");
    assert!(
        captures > 100 && compounds > 30 && promotions > 20,
        "captures{captures} compounds{compounds} promotions{promotions}"
    );
}
#[test]
fn two_royal_capture_and_threat_are_complete_action_based() {
    let sq = |f, r| Square::new(f, r).unwrap();
    let local = |entries: &[(u8, u8, Side, &str)], turn| {
        LocalPosition::admit(
            Position::from_diagram(
                &entries
                    .iter()
                    .map(|&(f, r, s, c)| (sq(f, r), Piece::from_code(s, c).unwrap()))
                    .collect::<Vec<_>>(),
                turn,
                RootMetadata::diagram(0),
            )
            .unwrap(),
        )
    };
    let native = local(
        &[
            (1, 12, Side::Sente, "K"),
            (6, 6, Side::Sente, "Ln"),
            (6, 5, Side::Gote, "K"),
            (5, 5, Side::Gote, "DE+"),
        ],
        Side::Gote,
    );
    let p = ChuPosition::from_local(&native);
    assert!(p.in_check(), "one Lion action captures both royals");
    let mut native = local(
        &[
            (1, 12, Side::Sente, "K"),
            (6, 6, Side::Sente, "Ln"),
            (6, 5, Side::Gote, "K"),
            (5, 5, Side::Gote, "DE+"),
        ],
        Side::Sente,
    );
    let mut p = ChuPosition::from_local(&native);
    let a = Action {
        from: sq(6, 6),
        route: vec![sq(6, 5), sq(5, 5)],
        promote: false,
    };
    probe(&mut p, &mut native, &a, true, "double royal capture");
    assert_eq!(p.rule_score(), Some(-MATE_SCORE + 1));
    let native = local(
        &[
            (1, 12, Side::Sente, "K"),
            (3, 6, Side::Sente, "R"),
            (9, 6, Side::Sente, "R"),
            (3, 1, Side::Gote, "K"),
            (9, 1, Side::Gote, "DE+"),
        ],
        Side::Gote,
    );
    assert!(
        !ChuPosition::from_local(&native).in_check(),
        "separate actions do not capture both"
    );
}
#[test]
fn native_root_restriction_excludes_complete_routes_and_null_is_disabled() {
    let game = Game::standard();
    let legal = game.legal_actions();
    let selected = legal[0].clone();
    let mut p = ChuPosition::from_game(&game);
    p.restrict_root(std::slice::from_ref(&selected));
    assert_eq!(actions(&mut p), BTreeSet::from([selected.clone()]));
    assert!(!p.null_move_safe());
    let m = p.find(&selected).unwrap();
    assert!(p.make(m));
    assert!(p.legal_moves().len() > 1);
    p.unmake();
    assert_eq!(actions(&mut p), BTreeSet::from([selected]));
}
#[test]
#[ignore = "bounded representation comparison; run explicitly in release mode"]
fn benchmark_against_native_local_generation() {
    let fs = fixtures();
    let f = fs.iter().find(|f| f["id"] == "CHU-LION-C2-3").unwrap();
    let mut tactical = root(&f["given"]["root"]);
    tactical.apply(&action(&f["given"]["commands"][0])).unwrap();
    for (name, native) in [
        ("initial", LocalPosition::standard()),
        ("lion-window", tactical),
    ] {
        let mut p = ChuPosition::from_local(&native);
        let rounds = 2_000;
        let start = std::time::Instant::now();
        let mut native_count = 0;
        for _ in 0..rounds {
            native_count += std::hint::black_box(native.legal_actions()).len();
        }
        let native_time = start.elapsed();
        let start = std::time::Instant::now();
        let mut fast_count = 0;
        for _ in 0..rounds {
            fast_count += std::hint::black_box(p.legal_moves()).len();
        }
        let fast_time = start.elapsed();
        assert_eq!(fast_count, native_count);
        eprintln!(
            "Chu {name} legal generation: rounds={rounds}, actions={native_count}, native_us={}, fast_us={}, speedup={:.2}",
            native_time.as_micros(),
            fast_time.as_micros(),
            native_time.as_secs_f64() / fast_time.as_secs_f64()
        );
    }
}

#[test]
fn native_history_reconstruction_and_must_vary_root_filter() {
    let fs = fixtures();
    for id in ["CHU-REP-QUIET", "CHU-REP-PASSES"] {
        let f = fs.iter().find(|f| f["id"] == id).unwrap();
        let mut game = Game::from_local_root(root(&f["given"]["root"])).unwrap();
        let accepted = f["expected"]["accepted_board_plies"].as_u64().unwrap() as usize;
        let commands = f["given"]["commands"].as_array().unwrap();
        for command in commands.iter().take(accepted) {
            game.submit(GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: GameCommand::Play {
                    action: action(command),
                },
            })
            .unwrap();
        }
        let mut p = ChuPosition::from_game(&game);
        compare(&mut p, game.local_position());
        let forbidden = action(&commands[accepted]);
        let local_move = p.find(&forbidden).unwrap();
        assert!(p.make(local_move));
        p.unmake();
        let allowed = game.legal_actions();
        assert!(!allowed.contains(&forbidden));
        p.restrict_root(&allowed);
        assert!(p.find(&forbidden).is_none());
        assert_eq!(actions(&mut p), allowed.into_iter().collect());
    }
}
#[test]
fn lion_window_changes_hash_and_rejected_capture_preserves_every_field() {
    let fs = fixtures();
    let f = fs.iter().find(|f| f["id"] == "CHU-LION-NONLION-R").unwrap();
    let mut native = root(&f["given"]["root"]);
    let mut p = ChuPosition::from_local(&native);
    let first = action(&f["given"]["commands"][0]);
    let second = action(&f["given"]["commands"][1]);
    probe(&mut p, &mut native, &first, true, "create protection");
    let fresh = ChuPosition::from_position(native.position());
    assert_ne!(fresh.hash(), p.hash());
    let (hash, eval, protection) = (p.hash(), p.evaluate(), p.protection());
    assert!(!p.make(p.find(&second).unwrap()));
    assert_eq!(p.hash(), hash);
    assert_eq!(p.evaluate(), eval);
    assert_eq!(p.protection(), protection);
    compare(&mut p, &native);
}

#[test]
fn narrow_material_fixtures_and_prince_recapture_keep_native_timing() {
    let fs = fixtures();
    for f in fs
        .iter()
        .filter(|f| f["id"].as_str().unwrap().starts_with("CHU-MATERIAL"))
    {
        if !f["given"]["root"].is_object() {
            continue;
        }
        let id = f["id"].as_str().unwrap();
        let native = root(&f["given"]["root"]);
        let p = ChuPosition::from_local(&native);
        if f["expected"].get("material_winner").is_some() {
            let expected = f["expected"]["material_winner"].as_str().map(|s| {
                if (s == "sente") == (p.side_to_move() == Side::Sente) {
                    MATE_SCORE
                } else {
                    -MATE_SCORE
                }
            });
            assert_eq!(p.rule_score(), expected, "{id}");
        }
    }
    for id in [
        "CHU-MATERIAL-RECAPTURE",
        "CHU-MATERIAL-COUNTERWIN",
        "CHU-MATERIAL-DECLINE-COUNTERCAPTURE",
    ] {
        let f = fs.iter().find(|f| f["id"] == id).unwrap();
        let mut native = root(&f["given"]["root"]);
        let mut p = ChuPosition::from_local(&native);
        assert_eq!(p.rule_score(), None, "{id} root");
        let commands = if let Some(commands) = f["given"]["commands"].as_array() {
            commands.clone()
        } else {
            vec![f["given"]["action"].clone()]
        };
        for command in commands {
            probe(&mut p, &mut native, &action(&command), true, id);
            if id == "CHU-MATERIAL-RECAPTURE" {
                assert_eq!(
                    p.rule_score(),
                    None,
                    "Prince may recapture, then agreement remains optional"
                );
            }
        }
        if id == "CHU-MATERIAL-COUNTERWIN" {
            assert_eq!(p.rule_score(), Some(-MATE_SCORE + 1));
        }
        if id == "CHU-MATERIAL-DECLINE-COUNTERCAPTURE" {
            assert_eq!(p.rule_score(), Some(MATE_SCORE - 1));
        }
    }
}
