//! Independent profile fixtures plus local differential verification.
use super::position::ChuPosition;
use crate::alphabeta::{MATE_SCORE, SearchPosition, Stage};
use mshogi_core::chu::{
    Action, Game, GameCommand, GameRequest, LocalPosition, Piece, Position, RootMetadata, Side,
    Square,
};
use mshogi_core::perft::perft;
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
        .map(|m| p.to_action(m))
        .collect()
}
fn compare_state(p: &ChuPosition, rules_position: &LocalPosition) {
    assert_eq!(p.side_to_move(), rules_position.position().side_to_move());
    for at in Square::all() {
        assert_eq!(p.piece(at), rules_position.position().piece(at), "{at}");
    }
    assert_eq!(p.protection(), *rules_position.protection());
    assert_eq!(p.hash(), p.hash_from_scratch());
    let fresh = ChuPosition::from_local(rules_position);
    assert_eq!(p.hash(), fresh.hash());
    assert_eq!(p.evaluate(), fresh.evaluate());
    assert_eq!(p.in_check(), fresh.in_check());
}
fn compare(p: &mut ChuPosition, rules_position: &LocalPosition) {
    compare_state(p, rules_position);
    let legal = rules_position.legal_actions();
    assert_eq!(actions(p), legal.into_iter().collect());
    let mut pseudo = Vec::new();
    p.generate(Stage::Tactical, &mut pseudo);
    let split = pseudo.len();
    p.generate(Stage::Quiet, &mut pseudo);
    let mut seen = BTreeSet::new();
    for (i, &m) in pseudo.iter().enumerate() {
        assert!(seen.insert(m), "duplicate {m:?}");
        assert_eq!(p.is_tactical(m), i < split);
        assert_eq!(p.find(&p.to_action(m)), Some(m));
        assert_eq!(p.decode(ChuPosition::encode(m)), Some(m));
    }
    assert!(p.decode(0).is_none());
    assert!(p.decode(u32::MAX).is_none());
}
fn probe(
    p: &mut ChuPosition,
    rules_position: &mut LocalPosition,
    a: &Action,
    expected: bool,
    id: &str,
) {
    let before_hash = p.hash();
    let before = rules_position.clone();
    let engine_result = rules_position.apply(a);
    assert_eq!(engine_result.is_ok(), expected, "{id}: {a:?}");
    let accepted = p.find(a).is_some_and(|m| p.make(m));
    assert_eq!(accepted, expected, "search {id}: {a:?}");
    if !accepted {
        assert_eq!(p.hash(), before_hash);
        assert_eq!(*rules_position, before);
    }
    compare(p, rules_position);
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
        let mut rules_position = root(&f["given"]["root"]);
        let mut p = ChuPosition::from_local(&rules_position);
        compare(&mut p, &rules_position);
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
                let mut n = rules_position.clone();
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
            probe(&mut p, &mut rules_position, &a, expected, id);
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
                probe(&mut p, &mut rules_position, &action(command), expected, id);
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
#[test]
fn independent_remote_perft_and_compound_perft_agree() {
    let fs = fixtures();
    for id in [
        "CHU-PERFT-REMOTE",
        "CHU-MOVE-Ln",
        "CHU-MOVE-SE",
        "CHU-MOVE-HF",
        "CHU-LION-NONLION-Ki",
    ] {
        let f = fs.iter().find(|f| f["id"] == id).unwrap();
        let rules_position = root(&f["given"]["root"]);
        let mut p = ChuPosition::from_local(&rules_position);
        for depth in 1..=2 {
            let n = perft(&rules_position, depth, 1_000_000, || false)
                .unwrap()
                .leaves;
            assert_eq!(p.perft(depth), n, "{id} depth {depth}");
            if id == "CHU-PERFT-REMOTE" {
                assert_eq!(n, if depth == 1 { 7 } else { 42 });
            }
        }
    }
    let rules_position = LocalPosition::standard();
    let mut p = ChuPosition::from_local(&rules_position);
    assert_eq!(
        p.perft(2),
        perft(&rules_position, 2, 1_000_000, || false)
            .unwrap()
            .leaves
    );
}
fn next(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}
#[test]
fn random_actions_positions_protection_hashes_and_undo_agree() {
    let (mut positions, mut captures, mut compounds, mut promotions) = (0, 0, 0, 0);
    for seed in 1..=20u64 {
        let mut rng = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
        let mut rules_position = LocalPosition::standard();
        let mut p = ChuPosition::from_local(&rules_position);
        for _ in 0..120 {
            compare(&mut p, &rules_position);
            positions += 1;
            if [Side::Sente, Side::Gote].iter().any(|&s| {
                !rules_position
                    .position()
                    .pieces()
                    .any(|(_, p)| p.side == s && p.is_royal())
            }) {
                break;
            }
            let legal = rules_position.legal_actions();
            if legal.is_empty() {
                break;
            }
            let desired: Vec<_> = legal
                .iter()
                .filter(|a| {
                    a.promote
                        || a.route.iter().any(|&to| {
                            to != a.from
                                && rules_position.position().piece(to).is_some_and(|piece| {
                                    piece.side != rules_position.position().side_to_move()
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
            let (effects, _) = rules_position.apply(&a).unwrap();
            captures += effects.captures.len();
            compare_state(&p, &rules_position);
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
        "captures {captures} compounds {compounds} promotions {promotions}"
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
    let rules_position = local(
        &[
            (1, 12, Side::Sente, "K"),
            (6, 6, Side::Sente, "Ln"),
            (6, 5, Side::Gote, "K"),
            (5, 5, Side::Gote, "DE+"),
        ],
        Side::Gote,
    );
    let p = ChuPosition::from_local(&rules_position);
    assert!(p.in_check(), "one Lion action captures both royals");
    let mut rules_position = local(
        &[
            (1, 12, Side::Sente, "K"),
            (6, 6, Side::Sente, "Ln"),
            (6, 5, Side::Gote, "K"),
            (5, 5, Side::Gote, "DE+"),
        ],
        Side::Sente,
    );
    let mut p = ChuPosition::from_local(&rules_position);
    let a = Action {
        from: sq(6, 6),
        route: vec![sq(6, 5), sq(5, 5)],
        promote: false,
    };
    probe(
        &mut p,
        &mut rules_position,
        &a,
        true,
        "double royal capture",
    );
    assert_eq!(p.rule_score(), Some(-MATE_SCORE + 1));
    let rules_position = local(
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
        !ChuPosition::from_local(&rules_position).in_check(),
        "separate actions do not capture both"
    );
}
#[test]
fn root_restriction_excludes_complete_routes_and_null_is_disabled() {
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
fn benchmark_against_local_generation() {
    let fs = fixtures();
    let f = fs.iter().find(|f| f["id"] == "CHU-LION-C2-3").unwrap();
    let mut tactical = root(&f["given"]["root"]);
    tactical.apply(&action(&f["given"]["commands"][0])).unwrap();
    for (name, rules_position) in [
        ("initial", LocalPosition::standard()),
        ("lion-window", tactical),
    ] {
        let mut p = ChuPosition::from_local(&rules_position);
        let rounds = 2_000;
        let start = std::time::Instant::now();
        let mut rules_count = 0;
        for _ in 0..rounds {
            rules_count += std::hint::black_box(rules_position.legal_actions()).len();
        }
        let time = start.elapsed();
        let start = std::time::Instant::now();
        let mut fast_count = 0;
        for _ in 0..rounds {
            fast_count += std::hint::black_box(p.legal_moves()).len();
        }
        let fast_time = start.elapsed();
        assert_eq!(fast_count, rules_count);
        eprintln!(
            "Chu {name} legal generation: rounds={rounds}, actions={rules_count}, us={}, fast_us={}, speedup={:.2}",
            time.as_micros(),
            fast_time.as_micros(),
            time.as_secs_f64() / fast_time.as_secs_f64()
        );
    }
}

#[test]
fn history_reconstruction_and_must_vary_root_filter() {
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
    let mut rules_position = root(&f["given"]["root"]);
    let mut p = ChuPosition::from_local(&rules_position);
    let first = action(&f["given"]["commands"][0]);
    let second = action(&f["given"]["commands"][1]);
    probe(
        &mut p,
        &mut rules_position,
        &first,
        true,
        "create protection",
    );
    let fresh = ChuPosition::from_position(rules_position.position());
    assert_ne!(fresh.hash(), p.hash());
    let (hash, eval, protection) = (p.hash(), p.evaluate(), p.protection());
    assert!(!p.make(p.find(&second).unwrap()));
    assert_eq!(p.hash(), hash);
    assert_eq!(p.evaluate(), eval);
    assert_eq!(p.protection(), protection);
    compare(&mut p, &rules_position);
}

#[test]
fn narrow_material_fixtures_and_prince_recapture_keep_timing() {
    let fs = fixtures();
    for f in fs
        .iter()
        .filter(|f| f["id"].as_str().unwrap().starts_with("CHU-MATERIAL"))
    {
        if !f["given"]["root"].is_object() {
            continue;
        }
        let id = f["id"].as_str().unwrap();
        let rules_position = root(&f["given"]["root"]);
        let p = ChuPosition::from_local(&rules_position);
        if f["expected"].get("material_winner").is_some() {
            let expected = f["expected"]["material_winner"].as_str().map(|s| {
                if (s == "sente") == (p.side_to_move() == Side::Sente) {
                    MATE_SCORE
                } else {
                    -MATE_SCORE
                }
            });
            if expected.is_some() {
                assert_eq!(p.rule_score(), expected, "{id}");
            } else {
                // Independently prove any distance-one win by a complete
                // action, not separate attacks on the royal squares.
                let victim = rules_position.position().side_to_move().opponent();
                let capture_all = rules_position.legal_actions().iter().any(|a| {
                    let mut after = rules_position.clone();
                    after.apply(a).unwrap();
                    !after
                        .position()
                        .pieces()
                        .any(|(_, piece)| piece.side == victim && piece.is_royal())
                });
                assert_eq!(
                    p.rule_score(),
                    capture_all.then_some(MATE_SCORE - 1),
                    "{id}"
                );
            }
        }
    }
    for id in [
        "CHU-MATERIAL-RECAPTURE",
        "CHU-MATERIAL-COUNTERWIN",
        "CHU-MATERIAL-DECLINE-COUNTERCAPTURE",
    ] {
        let f = fs.iter().find(|f| f["id"] == id).unwrap();
        let mut rules_position = root(&f["given"]["root"]);
        let mut p = ChuPosition::from_local(&rules_position);
        assert_eq!(
            p.rule_score(),
            if id == "CHU-MATERIAL-COUNTERWIN" {
                Some(MATE_SCORE - 1)
            } else {
                None
            },
            "{id} root"
        );
        let commands = if let Some(commands) = f["given"]["commands"].as_array() {
            commands.clone()
        } else {
            vec![f["given"]["action"].clone()]
        };
        for command in commands {
            let m = p.find(&action(&command)).unwrap();
            assert!(
                p.exchange_at_least(m, 0),
                "material terminal/equalizer must remain: {id}"
            );
            probe(&mut p, &mut rules_position, &action(&command), true, id);
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

#[test]
fn distance_one_royal_capture_proof_preserves_lion_rights_for_both_sides() {
    use mshogi_core::chu::LionProtection;
    for rotated in [false, true] {
        let transform = |f: u8, r: u8| {
            Square::new(
                if rotated { 13 - f } else { f },
                if rotated { 13 - r } else { r },
            )
            .unwrap()
        };
        let owner = |s: Side| if rotated { s.opponent() } else { s };
        let entries = [
            (1, 12, Side::Sente, "K"),
            (6, 8, Side::Sente, "R"),
            (6, 1, Side::Gote, "K"),
            (8, 8, Side::Gote, "Ln"),
            (8, 9, Side::Gote, "G"),
        ];
        let position = Position::from_diagram(
            &entries
                .iter()
                .map(|&(f, r, s, c)| (transform(f, r), Piece::from_code(owner(s), c).unwrap()))
                .collect::<Vec<_>>(),
            owner(Side::Sente),
            RootMetadata::diagram(0),
        )
        .unwrap();
        let rules_position = LocalPosition::from_parts(
            position,
            LionProtection {
                owner: Some(owner(Side::Gote)),
                squares: vec![transform(8, 8)],
            },
        )
        .unwrap();
        let mut p = ChuPosition::from_local(&rules_position);
        let before = (p.hash(), p.evaluate(), p.protection());
        // The rook may capture the royal on its file, but not the protected
        // Lion on its rank. The proof cannot consume the Lion window.
        for _ in 0..3 {
            assert_eq!(p.rule_score(), Some(MATE_SCORE - 1));
        }
        let forbidden = Action::single(transform(6, 8), transform(8, 8));
        assert!(!p.exchange_at_least(p.find(&forbidden).unwrap(), 0));
        assert!(!p.make(p.find(&forbidden).unwrap()));
        assert_eq!((p.hash(), p.evaluate(), p.protection()), before);
        compare(&mut p, &rules_position);
        let win = Action::single(transform(6, 8), transform(6, 1));
        assert!(p.exchange_at_least(p.find(&win).unwrap(), 100_000));
        assert!(p.make(p.find(&win).unwrap()));
        assert_eq!(p.rule_score(), Some(-MATE_SCORE + 1));
        p.unmake();
        assert_eq!((p.hash(), p.evaluate(), p.protection()), before);
    }
}

#[test]
fn capture_shortcut_requires_one_complete_action_for_both_royals() {
    let make = |entries: &[(u8, u8, Side, &str)]| {
        LocalPosition::admit(
            Position::from_diagram(
                &entries
                    .iter()
                    .map(|&(f, r, s, c)| {
                        (Square::new(f, r).unwrap(), Piece::from_code(s, c).unwrap())
                    })
                    .collect::<Vec<_>>(),
                Side::Sente,
                RootMetadata::diagram(0),
            )
            .unwrap(),
        )
    };
    let compound = make(&[
        (1, 12, Side::Sente, "K"),
        (6, 6, Side::Sente, "Ln"),
        (6, 5, Side::Gote, "K"),
        (5, 5, Side::Gote, "DE+"),
    ]);
    assert_eq!(
        ChuPosition::from_local(&compound).rule_score(),
        Some(MATE_SCORE - 1)
    );
    let compound_search = ChuPosition::from_local(&compound);
    let win = Action {
        from: Square::new(6, 6).unwrap(),
        route: vec![Square::new(6, 5).unwrap(), Square::new(5, 5).unwrap()],
        promote: false,
    };
    assert!(compound.legal_actions().contains(&win));
    assert!(compound_search.exchange_at_least(compound_search.find(&win).unwrap(), 100_000));
    let separate = make(&[
        (1, 12, Side::Sente, "K"),
        (3, 6, Side::Sente, "R"),
        (9, 6, Side::Sente, "R"),
        (3, 1, Side::Gote, "K"),
        (9, 1, Side::Gote, "DE+"),
    ]);
    assert_eq!(ChuPosition::from_local(&separate).rule_score(), None);
}

#[test]
fn exposed_seed12_root_completes_an_iteration_with_the_same_100k_budget() {
    // Frozen failed game-test pair 0012, first game, first request. Preserve
    // the four seeded actions; the original candidate used 100,000 nodes and
    // reported depth 0. This is an exposed regression, not strength evidence.
    let entries = [
        (7, 3, Side::Gote, "G"),
        (6, 2, Side::Gote, "K"),
        (5, 3, Side::Gote, "Si"),
        (11, 5, Side::Sente, "Ph"),
        (10, 5, Side::Gote, "VM"),
        (6, 5, Side::Gote, "Ln"),
        (3, 5, Side::Gote, "SM"),
        (8, 6, Side::Gote, "P"),
        (5, 6, Side::Gote, "P"),
        (2, 6, Side::Sente, "Ki"),
        (11, 7, Side::Gote, "Ki"),
        (8, 7, Side::Sente, "P"),
        (5, 7, Side::Sente, "P"),
        (10, 8, Side::Sente, "SM"),
        (7, 8, Side::Sente, "Ln"),
        (3, 8, Side::Sente, "VM"),
        (2, 8, Side::Gote, "Ph"),
        (8, 10, Side::Sente, "Si"),
        (7, 11, Side::Sente, "K"),
        (6, 10, Side::Sente, "G"),
    ];
    let sq = |f, r| Square::new(f, r).unwrap();
    let position = Position::from_diagram(
        &entries
            .iter()
            .map(|&(f, r, s, c)| (sq(f, r), Piece::from_code(s, c).unwrap()))
            .collect::<Vec<_>>(),
        Side::Sente,
        RootMetadata::diagram(0),
    )
    .unwrap();
    let mut game = Game::from_root(position).unwrap();
    for (from, to) in [
        ((8, 10), (9, 11)),
        ((7, 3), (7, 2)),
        ((6, 10), (6, 11)),
        ((7, 2), (7, 1)),
    ] {
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play {
                action: Action::single(sq(from.0, from.1), sq(to.0, to.1)),
            },
        })
        .unwrap();
    }
    let before = game.context();
    let start = std::time::Instant::now();
    let result = super::search(
        &game,
        super::Limits {
            depth: 64,
            nodes: 100_000,
        },
    )
    .unwrap();
    eprintln!(
        "Chu exposed seed 12: depth={}, nodes={}, elapsed_us={}",
        result.completed_depth,
        result.nodes,
        start.elapsed().as_micros()
    );
    assert!(result.completed_depth >= 1, "{result:?}");
    assert!(result.nodes <= 100_000);
    let Some(super::Decision::Choose(choice)) = result.decision else {
        panic!("no completed choice");
    };
    assert!(game.legal_actions().contains(&choice));
    assert_eq!(game.context(), before);
}

#[test]
fn bounded_exchange_keeps_free_and_recovered_captures_but_rejects_defended_loss() {
    for owner in [Side::Sente, Side::Gote] {
        let other = if owner == Side::Sente {
            Side::Gote
        } else {
            Side::Sente
        };
        let at = |f, r| Square::new(f, r).unwrap();
        let build = |defender: bool, recovery: bool, lion: bool| {
            let mut pieces = vec![
                (at(1, 12), Piece::from_code(owner, "K").unwrap()),
                (at(12, 1), Piece::from_code(other, "K").unwrap()),
                (
                    at(6, 7),
                    Piece::from_code(owner, if lion { "Ln" } else { "R" }).unwrap(),
                ),
                (at(6, 6), Piece::from_code(other, "P").unwrap()),
                (at(10, 10), Piece::from_code(owner, "P").unwrap()),
            ];
            if defender {
                pieces.push((at(3, 6), Piece::from_code(other, "R").unwrap()));
            }
            if recovery {
                pieces.push((at(7, 7), Piece::from_code(owner, "B").unwrap()));
            }
            LocalPosition::admit(
                Position::from_diagram(&pieces, owner, RootMetadata::diagram(0)).unwrap(),
            )
        };
        for (defender, recovery, expected) in [
            (false, false, true),
            (true, false, false),
            (true, true, true),
        ] {
            let rules_position = build(defender, recovery, false);
            let mut p = ChuPosition::from_local(&rules_position);
            let a = Action {
                from: at(6, 7),
                route: vec![at(6, 6)],
                promote: false,
            };
            assert!(rules_position.legal_actions().contains(&a));
            let m = p.find(&a).unwrap();
            let before = (p.hash(), p.evaluate(), actions(&mut p));
            assert_eq!(
                p.exchange_at_least(m, 0),
                expected,
                "{owner:?} defender={defender} recovery={recovery}"
            );
            if !defender {
                let gain = p.tactical_gain(m);
                assert!(p.exchange_at_least(m, gain));
                assert!(!p.exchange_at_least(m, gain + 1));
            }
            assert_eq!(before, (p.hash(), p.evaluate(), actions(&mut p)));
            compare_state(&p, &rules_position);
        }
        // The Lion returns to its source. A rook attacking the victim's square
        // cannot recapture the Lion there after this admitted igui action.
        let rules_position = build(true, false, true);
        let p = ChuPosition::from_local(&rules_position);
        let a = Action {
            from: at(6, 7),
            route: vec![at(6, 6), at(6, 7)],
            promote: false,
        };
        assert!(rules_position.legal_actions().contains(&a));
        let m = p.find(&a).unwrap();
        assert!(p.exchange_at_least(m, p.tactical_gain(m)));
    }
}

#[test]
fn exchange_counts_promotion_and_both_compound_victims() {
    let at = |f, r| Square::new(f, r).unwrap();
    for (mover, source, victims, route, promote) in [
        ("P", at(6, 5), vec![at(6, 4)], vec![at(6, 4)], true),
        (
            "Ln",
            at(6, 7),
            vec![at(6, 6), at(7, 6)],
            vec![at(6, 6), at(7, 6)],
            false,
        ),
    ] {
        let mut pieces = vec![
            (at(1, 12), Piece::from_code(Side::Sente, "K").unwrap()),
            (at(12, 1), Piece::from_code(Side::Gote, "K").unwrap()),
            (at(10, 10), Piece::from_code(Side::Sente, "P").unwrap()),
            (at(10, 2), Piece::from_code(Side::Gote, "P").unwrap()),
            (source, Piece::from_code(Side::Sente, mover).unwrap()),
        ];
        pieces.extend(
            victims
                .iter()
                .map(|&s| (s, Piece::from_code(Side::Gote, "P").unwrap())),
        );
        let rules_position = LocalPosition::admit(
            Position::from_diagram(&pieces, Side::Sente, RootMetadata::diagram(0)).unwrap(),
        );
        let p = ChuPosition::from_local(&rules_position);
        let a = Action {
            from: source,
            route,
            promote,
        };
        assert!(rules_position.legal_actions().contains(&a));
        let mut after = rules_position.clone();
        after.apply(&a).unwrap();
        let balance = |n: &LocalPosition| {
            n.position()
                .pieces()
                .map(|(_, piece)| {
                    super::piece_value(piece.face())
                        * if piece.side == Side::Sente { 1 } else { -1 }
                })
                .sum::<i32>()
        };
        let gain = balance(&after) - balance(&rules_position);
        let m = p.find(&a).unwrap();
        assert_eq!(p.tactical_gain(m), gain);
        assert!(p.exchange_at_least(m, gain));
        assert!(!p.exchange_at_least(m, gain + 1));
        compare_state(&p, &rules_position);
    }
}

#[test]
fn search_routes_preserve_exact_successors_and_root_restrictions() {
    fn check(rules_position: &LocalPosition, restricted: Option<&[Action]>) -> usize {
        let mut p = ChuPosition::from_local(rules_position);
        if let Some(allowed) = restricted {
            p.restrict_root(allowed);
        }
        let before = (p.hash(), p.evaluate(), p.protection());
        let mut reduced = 0;
        for stage in [Stage::Tactical, Stage::Quiet] {
            let mut full = Vec::new();
            p.generate(stage, &mut full);
            let mut selected = Vec::new();
            p.generate_search(stage, &mut selected);
            let mut again = Vec::new();
            p.generate_search(stage, &mut again);
            assert_eq!(selected, again, "stable first representative");
            let effect = |a: &Action| {
                let mut next = rules_position.clone();
                next.apply(a).unwrap();
                // Full state is the independent equality oracle.
                format!(
                    "{:?}|{:?}|{:?}",
                    next.position().side_to_move(),
                    Square::all()
                        .map(|s| next.position().piece(s))
                        .collect::<Vec<_>>(),
                    next.protection()
                )
            };
            let admitted = rules_position.legal_actions();
            let full_effects: BTreeSet<_> = full
                .iter()
                .map(|&m| p.to_action(m))
                .filter(|a| admitted.contains(a))
                .map(|a| effect(&a))
                .collect();
            let chosen_effects: BTreeSet<_> = selected
                .iter()
                .map(|&m| {
                    let a = p.to_action(m);
                    assert!(admitted.contains(&a));
                    if let Some(allowed) = restricted {
                        assert!(allowed.contains(&a));
                    }
                    effect(&a)
                })
                .collect();
            assert_eq!(
                chosen_effects.len(),
                selected.len(),
                "no duplicate successor"
            );
            assert_eq!(full_effects, chosen_effects);
            reduced += full.len().saturating_sub(selected.len());
        }
        assert_eq!(before, (p.hash(), p.evaluate(), p.protection()));
        compare_state(&p, rules_position);
        reduced
    }
    let mut reduced = 0;
    for f in fixtures() {
        if f["given"]["root"].is_object() && !f["id"].as_str().unwrap().starts_with("CHU-MATERIAL")
        {
            let rules_position = root(&f["given"]["root"]);
            reduced += check(&rules_position, None);
            let allowed: Vec<_> = rules_position
                .legal_actions()
                .into_iter()
                .rev()
                .step_by(2)
                .collect();
            reduced += check(&rules_position, Some(&allowed));
        }
    }
    let fs = fixtures();
    let window = fs.iter().find(|f| f["id"] == "CHU-LION-C2-3").unwrap();
    let mut active = root(&window["given"]["root"]);
    active
        .apply(&action(&window["given"]["commands"][0]))
        .unwrap();
    assert!(!active.protection().squares.is_empty());
    reduced += check(&active, None);
    let mut rules_position = LocalPosition::standard();
    let mut seed = 0x1825eeu64;
    for _ in 0..80 {
        reduced += check(&rules_position, None);
        let legal = rules_position.legal_actions();
        if legal.is_empty() {
            break;
        }
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        rules_position
            .apply(&legal[seed as usize % legal.len()])
            .unwrap();
    }
    assert!(
        reduced > 100,
        "compound/pass equivalents exercised: {reduced}"
    );
}

#[test]
fn exchange_recapturing_lion_can_return_away_from_the_original_target() {
    let at = |f, r| Square::new(f, r).unwrap();
    let entries = [
        (1, 12, Side::Sente, "K"),
        (12, 1, Side::Gote, "K"),
        (6, 7, Side::Sente, "R"),
        (7, 6, Side::Sente, "R"),
        (6, 6, Side::Gote, "P"),
        (5, 5, Side::Gote, "Ln"),
    ];
    let mut rules_position = LocalPosition::admit(
        Position::from_diagram(
            &entries
                .iter()
                .map(|&(f, r, s, c)| (at(f, r), Piece::from_code(s, c).unwrap()))
                .collect::<Vec<_>>(),
            Side::Sente,
            RootMetadata::diagram(0),
        )
        .unwrap(),
    );
    let p = ChuPosition::from_local(&rules_position);
    let initial = Action::single(at(6, 7), at(6, 6));
    assert!(!p.exchange_at_least(p.find(&initial).unwrap(), 0));
    rules_position.apply(&initial).unwrap();
    let recapture = Action {
        from: at(5, 5),
        route: vec![at(6, 6), at(5, 5)],
        promote: false,
    };
    assert!(rules_position.legal_actions().contains(&recapture));
    rules_position.apply(&recapture).unwrap();
    assert!(rules_position.position().piece(at(6, 6)).is_none());
    assert_eq!(
        rules_position.position().piece(at(5, 5)).unwrap().face(),
        mshogi_core::chu::Face::Lion
    );
    assert!(
        !rules_position
            .legal_actions()
            .iter()
            .any(|a| a.route.contains(&at(5, 5)))
    );
}

#[test]
fn exposed_seed20_midgame_completes_with_bounded_quiescence() {
    // Exact root and six accepted actions from round4/match100k/failed-request.json.
    // The frozen predecessor reached depth 0 at 100k on this second engine turn.
    // This exposed regression proves resource completion, not playing strength.
    let entries = [
        (6, 2, Side::Gote, "K"),
        (7, 3, Side::Gote, "G"),
        (5, 3, Side::Gote, "Si"),
        (11, 5, Side::Sente, "Ph"),
        (10, 5, Side::Gote, "VM"),
        (6, 5, Side::Gote, "Ln"),
        (3, 5, Side::Gote, "SM"),
        (8, 6, Side::Gote, "P"),
        (5, 6, Side::Gote, "P"),
        (2, 6, Side::Sente, "Ki"),
        (11, 7, Side::Gote, "Ki"),
        (8, 7, Side::Sente, "P"),
        (5, 7, Side::Sente, "P"),
        (10, 8, Side::Sente, "SM"),
        (7, 8, Side::Sente, "Ln"),
        (3, 8, Side::Sente, "VM"),
        (2, 8, Side::Gote, "Ph"),
        (8, 10, Side::Sente, "Si"),
        (6, 10, Side::Sente, "G"),
        (7, 11, Side::Sente, "K"),
    ];
    let sq = |f, r| Square::new(f, r).unwrap();
    let position = Position::from_diagram(
        &entries
            .iter()
            .map(|&(f, r, s, c)| (sq(f, r), Piece::from_code(s, c).unwrap()))
            .collect::<Vec<_>>(),
        Side::Sente,
        RootMetadata::diagram(0),
    )
    .unwrap();
    let mut game = Game::from_root(position).unwrap();
    for (from, to, promote) in [
        ((8, 10), (9, 11), false),
        ((10, 5), (10, 1), false),
        ((6, 10), (6, 11), false),
        ((10, 1), (9, 1), false),
        ((2, 6), (2, 4), true),
        ((11, 7), (11, 9), true),
    ] {
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play {
                action: Action {
                    from: sq(from.0, from.1),
                    route: vec![sq(to.0, to.1)],
                    promote,
                },
            },
        })
        .unwrap();
    }
    assert_eq!(game.events().len(), 6);
    assert_eq!(game.legal_actions().len(), 190);
    let before = mshogi_core::chu::GameRecord::from_game(&game).unwrap();
    let mut private = ChuPosition::from_game(&game);
    assert!(!private.in_check());
    let mut threatened = 0;
    for a in game.legal_actions() {
        assert!(private.make(private.find(&a).unwrap()));
        threatened += usize::from(private.in_check());
        private.unmake();
    }
    assert_eq!(
        threatened, 9,
        "quiet threat responses occur below this unchecked root"
    );
    let result = super::search(
        &game,
        super::Limits {
            depth: 64,
            nodes: 100_000,
        },
    )
    .unwrap();
    assert!(result.completed_depth >= 1, "{result:?}");
    assert!(result.nodes <= 100_000);
    let Some(super::Decision::Choose(choice)) = result.decision else {
        panic!("no completed choice")
    };
    assert!(game.legal_actions().contains(&choice));
    assert_eq!(
        mshogi_core::chu::GameRecord::from_game(&game).unwrap(),
        before
    );
}

#[test]
fn single_royal_threat_matches_captures_for_every_piece_identity() {
    use mshogi_core::chu::Base;
    let target: Square = "6f".parse().unwrap();
    let remote: Square = "12l".parse().unwrap();
    let block: Square = "6g".parse().unwrap();
    let mut positive = 0;
    let mut negative = 0;
    for attacker in [Side::Sente, Side::Gote] {
        for base in Base::ALL {
            for promoted in [false, true] {
                let Some(piece) = Piece::new(attacker, base, promoted) else {
                    continue;
                };
                for origin in ["6e", "6g", "9i", "6j", "7h", "4f"] {
                    let origin: Square = origin.parse().unwrap();
                    for blocked in [false, true] {
                        if blocked && origin == block {
                            continue;
                        }
                        let victim = attacker.opponent();
                        let mut pieces = vec![
                            (target, Piece::new(victim, Base::King, false).unwrap()),
                            (origin, piece),
                        ];
                        if base != Base::King {
                            pieces.push((remote, Piece::new(attacker, Base::King, false).unwrap()));
                        }
                        if blocked {
                            pieces.push((block, Piece::new(victim, Base::Pawn, false).unwrap()));
                        }
                        let rules_position = LocalPosition::admit(
                            Position::from_diagram(&pieces, victim, RootMetadata::diagram(0))
                                .unwrap(),
                        );
                        let expected =
                            rules_position
                                .legal_actions_for(attacker)
                                .iter()
                                .any(|action| {
                                    rules_position
                                        .preview_for(attacker, action)
                                        .unwrap()
                                        .captures
                                        .iter()
                                        .any(|capture| capture.square == target)
                                });
                        assert_eq!(
                            ChuPosition::from_local(&rules_position).in_check(),
                            expected,
                            "{attacker:?} {base:?} promoted={promoted} origin={origin} blocked={blocked}"
                        );
                        positive += usize::from(expected);
                        negative += usize::from(!expected);
                    }
                }
            }
        }
    }
    assert!(positive > 100 && negative > 100);
}
