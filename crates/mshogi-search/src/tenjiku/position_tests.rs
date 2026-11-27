//! Independent profile expectations and differential private/projections.
use super::position::TenjikuPosition;
use crate::alphabeta::{MATE_SCORE, SearchPosition, Stage};
use mshogi_core::perft::perft;
use mshogi_core::tenjiku::*;
use serde_json::Value;
use std::collections::BTreeSet;
fn fixtures() -> Vec<Value> {
    serde_json::from_str::<Value>(include_str!("../../../../docs/m4/tenjiku/fixtures.json"))
        .unwrap()["fixtures"]
        .as_array()
        .unwrap()
        .clone()
}
macro_rules! parse {
    ($v:expr) => {
        serde_json::from_value(($v).clone()).unwrap()
    };
}

fn root(v: &Value, profile: Profile) -> Position {
    let entries = v["board"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            let id = p["id"].as_str().unwrap().parse().unwrap();
            let mut piece = Piece::new(
                id,
                parse!(&p["side"]),
                parse!(&p["original"]),
                p["promoted"].as_bool().unwrap(),
            )
            .unwrap();
            if let Some(rights) = v["promotion_rights"].as_array() {
                for r in rights {
                    if r["piece_id"] == p["id"] {
                        piece.right = match r["state"].as_str().unwrap() {
                            "unseen" => PromotionRight::Unseen,
                            "first_declined_next_move" => PromotionRight::FirstDeclinedNextMove,
                            "restriction_consumed" => PromotionRight::Consumed,
                            s => panic!("{s}"),
                        };
                    }
                }
            }
            (parse!(&p["at"]), piece)
        })
        .collect::<Vec<_>>();
    Position::from_diagram(
        profile,
        &entries,
        parse!(&v["turn"]),
        RootMetadata::diagram(0),
    )
    .unwrap()
}
fn face_piece(id: u16, side: Side, face: Face) -> Piece {
    if let Some(base) = Base::ALL.into_iter().find(|b| b.face() == face) {
        Piece::new(id, side, base, false).unwrap()
    } else {
        let base = Base::ALL
            .into_iter()
            .find(|b| b.promotion() == Some(face))
            .unwrap();
        Piece::new(id, side, base, true).unwrap()
    }
}
fn actions(p: &mut TenjikuPosition) -> BTreeSet<Action> {
    p.legal_moves()
        .into_iter()
        .map(|m| p.to_action(m))
        .collect()
}
fn state(p: &TenjikuPosition, n: &Position) {
    assert_eq!(p.profile(), n.profile());
    assert_eq!(p.side_to_move(), n.side_to_move());
    for at in Square::all() {
        assert_eq!(p.piece(at), n.piece(at), "{at}");
    }
    for side in [Side::Sente, Side::Gote] {
        assert_eq!(
            p.material(side),
            n.pieces()
                .filter(|(_, p)| p.side == side)
                .map(|(_, p)| super::piece_value(p.face()))
                .sum::<i32>()
        );
    }
    assert_eq!(p.hash(), p.hash_from_scratch());
}
fn compare(p: &mut TenjikuPosition, n: &LocalPosition, label: &str) {
    assert_eq!(
        actions(p),
        n.legal_actions().into_iter().collect(),
        "{label}"
    );
    state(p, n.position());
}
fn probe(p: &mut TenjikuPosition, n: &mut LocalPosition, a: &Action, label: &str) -> bool {
    let before = p.clone();
    let expected = n.simulate(a);
    let m = p.find(a);
    let accepted = m.is_some_and(|m| p.make(m));
    assert_eq!(accepted, expected.is_ok(), "{label}: {a:?}");
    if let Ok((next, _)) = expected {
        state(p, next.position());
        p.unmake();
        state(p, n.position());
        assert_eq!(p.hash(), before.hash());
        assert!(p.make(m.unwrap()));
        *n = next;
    } else {
        state(p, n.position());
        assert_eq!(p.hash(), before.hash());
    }
    accepted
}
#[test]
fn all_44_faces_match_independent_complete_geometry_for_both_profiles_and_sides() {
    let fs = fixtures();
    let mut count = 0;
    for profile in Profile::ALL {
        for f in fs
            .iter()
            .filter(|f| f["given"]["query"] == "empty_center_local_geometry")
        {
            let g = &f["given"];
            let at: Square = parse!(&g["actor"]);
            let side: Side = parse!(&g["side"]);
            let piece = face_piece(17, side, parse!(&g["face"]));
            let pos =
                Position::from_diagram(profile, &[(at, piece)], side, RootMetadata::diagram(0))
                    .unwrap();
            let mut p = TenjikuPosition::from_position(&pos);
            let actual: BTreeSet<_> = actions(&mut p)
                .into_iter()
                .map(|mut a| {
                    a.promote = false;
                    a
                })
                .collect();
            let expected: BTreeSet<Action> =
                parse!(&f["expected"][profile.id()]["complete_geometry_actions"]);
            assert_eq!(actual, expected, "{} {profile:?}", f["id"]);
            // A cached move can name any square, including one never emitted by raw generation.
            for to in Square::all() {
                let direct = Action::single(at, to);
                assert_eq!(
                    p.find(&direct).is_some(),
                    expected.contains(&direct),
                    "{} {profile:?} {to}",
                    f["id"]
                );
            }
            count += 1;
        }
    }
    assert_eq!(count, 176);
}
#[test]
fn independent_concrete_routes_and_sequences_match_full_effects() {
    let fs = fixtures();
    let mut roots = 0;
    let mut steps = 0;
    for profile in Profile::ALL {
        for f in &fs {
            let g = &f["given"];
            if !g["root"].is_object() {
                continue;
            }
            let pos = root(&g["root"], profile);
            let mut n = LocalPosition::admit(pos.clone()).unwrap();
            let mut p = TenjikuPosition::from_position(&pos);
            let id = f["id"].as_str().unwrap();
            compare(&mut p, &n, id);
            roots += 1;
            if g["route"].is_array() {
                let a = Action {
                    kind: if g["kind"] == "igui" {
                        ActionKind::Igui
                    } else {
                        ActionKind::Travel
                    },
                    from: parse!(&g["actor"]),
                    route: parse!(&g["route"]),
                    promote: g["promote"].as_bool().unwrap_or(false),
                    target: g.get("target").map(|v| parse!(v)),
                };
                let accepted = probe(&mut p, &mut n, &a, id);
                if let Some(expected) = f["expected"][profile.id()]["accepted"].as_bool() {
                    assert_eq!(accepted, expected, "{id}");
                }
                steps += 1;
            }
            if let Some(commands) = g["commands"].as_array() {
                for command in commands {
                    let c = &command["command"];
                    if c["kind"] != "play" {
                        continue;
                    }
                    let a: Action = parse!(&c["action"]);
                    if !probe(&mut p, &mut n, &a, id) {
                        break;
                    }
                    steps += 1;
                }
            }
        }
    }
    assert!(roots >= 200, "{roots}");
    assert!(steps >= 200, "{steps}");
}
#[test]
fn independent_remote_perft_and_initial_counts() {
    for profile in Profile::ALL {
        let pos = Position::from_diagram(
            profile,
            &[
                (
                    Square::new(1, 16).unwrap(),
                    Piece::new(1, Side::Sente, Base::K, false).unwrap(),
                ),
                (
                    Square::new(16, 1).unwrap(),
                    Piece::new(2, Side::Gote, Base::K, false).unwrap(),
                ),
            ],
            Side::Sente,
            RootMetadata::diagram(0),
        )
        .unwrap();
        let n = LocalPosition::admit(pos).unwrap();
        let mut p = TenjikuPosition::from_local(&n);
        assert_eq!(p.perft(1), 3);
        assert_eq!(p.perft(2), 9);
        assert_eq!(perft(&n, 2, 1_000_000, || false).unwrap().leaves, 9);
        let n = LocalPosition::standard(profile);
        let mut p = TenjikuPosition::from_local(&n);
        compare(&mut p, &n, "standard");
        assert_eq!(p.perft(1), n.legal_actions().len() as u64);
    }
}
#[test]
fn randomized_projection_hash_and_complete_undo() {
    let fs = fixtures();
    let mut seed = 0x183f1deu64;
    let mut visited = 0;
    let mut removals = 0;
    let mut compounds = 0;
    let mut flips = 0;
    for profile in Profile::ALL {
        for start in 0..6 {
            let mut n = if start == 0 {
                LocalPosition::standard(profile)
            } else {
                let f = fs
                    .iter()
                    .filter(|f| f["given"]["root"].is_object() && f["given"]["route"].is_array())
                    .nth(start * 5)
                    .unwrap();
                LocalPosition::admit(root(&f["given"]["root"], profile)).unwrap()
            };
            let mut p = TenjikuPosition::from_local(&n);
            let initial = p.clone();
            let mut prior = Vec::new();
            for _ in 0..60 {
                compare(&mut p, &n, "random");
                let moves = p.legal_moves();
                if moves.is_empty() {
                    break;
                }
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                let m = moves[seed as usize % moves.len()];
                let a = p.to_action(m);
                let (next, e) = n.simulate(&a).unwrap();
                removals += e.captures().count();
                compounds += usize::from(a.route.len() > 1);
                flips += usize::from(
                    e.phases
                        .iter()
                        .any(|p| matches!(p, EffectPhase::Promotion { .. })),
                );
                prior.push(n);
                assert!(p.make(m));
                n = next;
                state(&p, n.position());
                visited += 1;
            }
            while let Some(n) = prior.pop() {
                p.unmake();
                state(&p, n.position());
            }
            assert_eq!(p.hash(), initial.hash());
        }
    }
    assert!(visited > 300, "{visited}");
    assert!(removals > 15, "{removals}");
    assert!(compounds > 15, "{compounds}");
    assert!(flips > 5, "{flips}");
}

#[test]
fn burn_phase_expectations_and_bare_royal_results_are_preserved() {
    let fs = fixtures();
    for profile in Profile::ALL {
        for f in fs
            .iter()
            .filter(|f| f["id"].as_str().unwrap().starts_with("TEN-BURN-"))
        {
            let g = &f["given"];
            let pos = root(&g["root"], profile);
            let actor = pos.side_to_move();
            let from: Square = parse!(&g["actor"]);
            let mover = pos.piece(from).unwrap();
            let a = Action {
                kind: ActionKind::Travel,
                from,
                route: parse!(&g["route"]),
                promote: g["promote"].as_bool().unwrap_or(false),
                target: None,
            };
            let mut p = TenjikuPosition::from_position(&pos);
            let m = p.find(&a).unwrap();
            assert!(p.make(m));
            let expected = &f["expected"][profile.id()];
            if let Some(phases) = expected["effects"].as_array() {
                let removed: BTreeSet<_> = phases
                    .iter()
                    .filter(|phase| phase["phase"] == "capture" || phase["phase"] == "burn")
                    .flat_map(|phase| {
                        phase["victims"].as_array().unwrap().iter().map(|v| {
                            let s: Square = parse!(v);
                            pos.piece(s).unwrap().id
                        })
                    })
                    .collect();
                let actual: BTreeSet<_> = pos
                    .pieces()
                    .filter(|(_, piece)| {
                        piece.side != actor
                            && !Square::all()
                                .any(|s| p.piece(s).is_some_and(|after| after.id == piece.id))
                    })
                    .map(|(_, p)| p.id)
                    .collect();
                assert_eq!(actual, removed, "{} {profile:?}", f["id"]);
                let dies = phases.iter().any(|phase| phase["phase"] == "mover_dies");
                assert_eq!(
                    Square::all().any(|s| p.piece(s).is_some_and(|p| p.id == mover.id)),
                    !dies
                );
            }
            if let Some(alive) = expected["mover_survives"].as_bool() {
                assert_eq!(
                    Square::all().any(|s| p.piece(s).is_some_and(|p| p.id == mover.id)),
                    alive
                );
            }
            if let Some(face) = expected["promoted_face"].as_str() {
                assert_eq!(p.piece(a.to().unwrap()).unwrap().face().code(), face);
            }
            if expected["enemy_pawn_survives"] == true {
                for (s, pawn) in pos
                    .pieces()
                    .filter(|(_, p)| p.side != actor && p.base == Base::P)
                {
                    assert_eq!(p.piece(s), Some(pawn));
                }
            }
            if expected["both_royal_sets_empty"] == true {
                assert_eq!(p.rule_score(), Some(0));
            }
            if expected["opponent_royals_remaining"] == 0 {
                assert_eq!(p.rule_score(), Some(-MATE_SCORE + 1));
            }
            p.unmake();
            state(&p, &pos);
        }
        for f in fs
            .iter()
            .filter(|f| f["id"].as_str().unwrap().starts_with("TEN-BARE-CONCRETE"))
        {
            let mut game = Game::from_root(root(&f["given"]["root"], profile)).unwrap();
            let mut p = TenjikuPosition::from_game(&game);
            for command in f["given"]["commands"].as_array().unwrap() {
                let action: Action = parse!(&command["command"]["action"]);
                let m = p.find(&action).unwrap();
                assert!(p.make(m));
                game.submit(GameRequest {
                    expected: game.context(),
                    actor: game.position().side_to_move(),
                    command: GameCommand::Play { action },
                })
                .unwrap();
                state(&p, game.position());
                if let Some(c) = game.conclusion() {
                    let expected = c.winner().map_or(0, |winner| {
                        if winner == p.side_to_move() {
                            MATE_SCORE - game.events().len() as i32
                        } else {
                            -MATE_SCORE + game.events().len() as i32
                        }
                    });
                    assert_eq!(p.rule_score(), Some(expected), "{} {profile:?}", f["id"]);
                }
            }
        }
    }
}
#[test]
fn full_identity_rights_profile_and_history_are_distinct() {
    let fs = fixtures();
    for profile in Profile::ALL {
        let f = fs
            .iter()
            .find(|f| f["id"] == "TEN-REFUSAL-CONCRETE-EXPIRES")
            .unwrap();
        let pos = root(&f["given"]["root"], profile);
        let baseline = TenjikuPosition::from_position(&pos);
        let mut pieces: Vec<_> = pos.pieces().collect();
        pieces[0].1.id = 60000;
        let other = Position::from_diagram(
            profile,
            &pieces,
            pos.side_to_move(),
            RootMetadata::diagram(0),
        )
        .unwrap();
        assert_ne!(
            baseline.hash(),
            TenjikuPosition::from_position(&other).hash()
        );
        assert_eq!(
            RepetitionKey::from_position(&pos),
            RepetitionKey::from_position(&other)
        );
        let target = pieces.iter_mut().find(|(_, p)| p.base == Base::G).unwrap();
        target.1.right = PromotionRight::FirstDeclinedNextMove;
        let rights = Position::from_diagram(
            profile,
            &pieces,
            pos.side_to_move(),
            RootMetadata::diagram(0),
        )
        .unwrap();
        assert_ne!(
            TenjikuPosition::from_position(&other).hash(),
            TenjikuPosition::from_position(&rights).hash()
        );
        assert_eq!(
            RepetitionKey::from_position(&other),
            RepetitionKey::from_position(&rights)
        );
        let mut game = Game::from_root(pos).unwrap();
        for command in f["given"]["commands"].as_array().unwrap() {
            let c: GameCommand = parse!(&command["command"]);
            game.submit(GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: c,
            })
            .unwrap();
            let p = TenjikuPosition::from_game(&game);
            state(&p, game.position());
        }
        let p = TenjikuPosition::from_game(&game);
        let mut restricted = p.clone();
        let allowed: Vec<_> = game.legal_actions().into_iter().rev().step_by(2).collect();
        restricted.restrict_root(&allowed);
        assert_eq!(actions(&mut restricted), allowed.into_iter().collect());
    }
    assert_ne!(
        TenjikuPosition::from_position(&Position::standard(Profile::AdamsModern)).hash(),
        TenjikuPosition::from_position(&Position::standard(Profile::MullerCompleted)).hash()
    );
}
#[test]
fn search_representatives_preserve_exact_successors() {
    let fs = fixtures();
    let mut removed = 0;
    for profile in Profile::ALL {
        for f in fs
            .iter()
            .filter(|f| f["given"]["root"].is_object() && f["given"]["route"].is_array())
        {
            let rules_position = LocalPosition::admit(root(&f["given"]["root"], profile)).unwrap();
            let mut p = TenjikuPosition::from_local(&rules_position);
            let admitted = rules_position.legal_actions();
            for restrict in [false, true] {
                if restrict {
                    p.restrict_root(
                        &admitted
                            .iter()
                            .rev()
                            .step_by(2)
                            .cloned()
                            .collect::<Vec<_>>(),
                    );
                }
                for stage in [Stage::Tactical, Stage::Quiet] {
                    let before = p.hash();
                    let mut full = Vec::new();
                    p.generate(stage, &mut full);
                    let mut selected = Vec::new();
                    p.generate_search(stage, &mut selected);
                    let mut repeated = Vec::new();
                    p.generate_search(stage, &mut repeated);
                    assert_eq!(selected, repeated);
                    let effect = |m| {
                        let a = p.to_action(m);
                        assert!(admitted.contains(&a));
                        let (next, _) = rules_position.simulate(&a).unwrap();
                        format!("{:?}", next.position().state())
                    };
                    let expected: BTreeSet<_> = full.iter().copied().map(effect).collect();
                    let actual: BTreeSet<_> = selected.iter().copied().map(effect).collect();
                    assert_eq!(expected, actual);
                    assert_eq!(actual.len(), selected.len());
                    removed += full.len() - selected.len();
                    assert_eq!(before, p.hash());
                    state(&p, rules_position.position());
                }
            }
        }
    }
    assert!(removed > 100);
}

fn diagram(profile: Profile, entries: &[(u8, u8, Side, Base, bool)], turn: Side) -> Position {
    Position::from_diagram(
        profile,
        &entries
            .iter()
            .enumerate()
            .map(|(id, &(f, r, s, b, p))| {
                (
                    Square::new(f, r).unwrap(),
                    Piece::new(id as u16, s, b, p).unwrap(),
                )
            })
            .collect::<Vec<_>>(),
        turn,
        RootMetadata::diagram(0),
    )
    .unwrap()
}
#[test]
fn quiet_discovery_respects_profile_and_root_admission() {
    for profile in Profile::ALL {
        let root = diagram(
            profile,
            &[
                (16, 16, Side::Sente, Base::K, false),
                (8, 6, Side::Sente, Base::R, false),
                (6, 6, Side::Sente, Base::P, false),
                (4, 6, Side::Gote, Base::K, false),
                (15, 2, Side::Gote, Base::P, false),
            ],
            Side::Sente,
        );
        let mut pos = TenjikuPosition::from_position(&root);
        let discovery = Action::single(Square::new(6, 6).unwrap(), Square::new(6, 5).unwrap());
        let m = pos.find(&discovery).unwrap();
        assert!(!pos.is_tactical(m));
        let mut checks = Vec::new();
        pos.generate_checks(&mut checks);
        assert!(checks.contains(&m));
        assert!(pos.make(m));
        assert!(pos.in_check());
        pos.unmake();
        let allowed: Vec<_> = LocalPosition::admit(root)
            .unwrap()
            .legal_actions()
            .into_iter()
            .filter(|action| action != &discovery)
            .collect();
        pos.restrict_root(&allowed);
        checks.clear();
        pos.generate_checks(&mut checks);
        assert!(!checks.contains(&m));
    }
}

#[test]
fn free_eagle_diagonal_turn_threat_and_exchange_survive_straight_ray_blocker() {
    for profile in Profile::ALL {
        let pos = diagram(
            profile,
            &[
                (16, 16, Side::Sente, Base::K, false),
                (6, 6, Side::Sente, Base::FEg, false),
                (5, 6, Side::Sente, Base::P, false),
                (4, 6, Side::Gote, Base::K, false),
                (15, 2, Side::Gote, Base::P, false),
            ],
            Side::Gote,
        );
        let p = TenjikuPosition::from_position(&pos);
        assert!(
            p.in_check(),
            "orthogonal distance 2 royal reachable around blocker"
        );
        let attack = diagram(
            profile,
            &[
                (16, 16, Side::Sente, Base::K, false),
                (6, 6, Side::Sente, Base::FEg, false),
                (5, 6, Side::Sente, Base::P, false),
                (4, 6, Side::Gote, Base::K, false),
                (15, 2, Side::Gote, Base::P, false),
            ],
            Side::Sente,
        );
        let p = TenjikuPosition::from_position(&attack);
        assert_eq!(p.rule_score(), Some(MATE_SCORE - 1));
        let route = if profile == Profile::MullerCompleted {
            vec![Square::new(5, 5).unwrap(), Square::new(4, 6).unwrap()]
        } else {
            vec![Square::new(4, 6).unwrap()]
        };
        let a = Action {
            kind: ActionKind::Travel,
            from: Square::new(6, 6).unwrap(),
            route,
            promote: false,
            target: None,
        };
        assert!(
            LocalPosition::admit(attack)
                .unwrap()
                .legal_actions()
                .contains(&a)
        );
        let pos = diagram(
            profile,
            &[
                (16, 16, Side::Sente, Base::K, false),
                (6, 6, Side::Sente, Base::FEg, false),
                (5, 6, Side::Sente, Base::P, false),
                (4, 6, Side::Sente, Base::P, false),
                (1, 1, Side::Gote, Base::K, false),
                (4, 8, Side::Gote, Base::R, false),
                (15, 2, Side::Gote, Base::P, false),
            ],
            Side::Gote,
        );
        let p = TenjikuPosition::from_position(&pos);
        let a = Action::single(Square::new(4, 8).unwrap(), Square::new(4, 6).unwrap());
        let m = p.find(&a).unwrap();
        assert!(
            !p.exchange_at_least(m, 0),
            "Eagle recaptures around blocker"
        );
    }
}
#[test]
fn exchange_accounts_for_off_target_demon_burn_and_mover_suicide() {
    for profile in Profile::ALL {
        let pos = diagram(
            profile,
            &[
                (1, 16, Side::Sente, Base::K, false),
                (15, 14, Side::Sente, Base::P, false),
                (8, 10, Side::Sente, Base::R, false),
                (16, 1, Side::Gote, Base::K, false),
                (8, 8, Side::Gote, Base::P, false),
                (8, 5, Side::Gote, Base::FiD, false),
            ],
            Side::Sente,
        );
        let p = TenjikuPosition::from_position(&pos);
        let a = Action::single(Square::new(8, 10).unwrap(), Square::new(8, 8).unwrap());
        let m = p.find(&a).unwrap();
        assert!(
            !p.exchange_at_least(m, 0),
            "Demon removes rook without landing on it"
        );
        let n = LocalPosition::admit(pos).unwrap().simulate(&a).unwrap().0;
        let burn = Action::single(Square::new(8, 5).unwrap(), Square::new(8, 7).unwrap());
        let (after, e) = n.simulate(&burn).unwrap();
        assert!(after.position().piece(Square::new(8, 8).unwrap()).is_none());
        assert!(
            e.phases
                .iter()
                .any(|p| matches!(p, EffectPhase::Burn { .. }))
        );
        let f = fixtures()
            .into_iter()
            .find(|f| f["id"] == "TEN-BURN-DYING")
            .unwrap();
        let pos = root(&f["given"]["root"], profile);
        let p = TenjikuPosition::from_position(&pos);
        let a = Action::single(Square::new(8, 10).unwrap(), Square::new(8, 8).unwrap());
        let m = p.find(&a).unwrap();
        assert_eq!(
            p.tactical_gain(m),
            if profile == Profile::AdamsModern {
                -2400
            } else {
                -2500
            }
        );
        assert!(!p.exchange_at_least(m, 0));
        assert_eq!(p.hash(), p.hash_from_scratch());
    }
}
#[test]
#[ignore = "manual release representation benchmark; run without other builds"]
fn benchmark_and_private_generation() {
    let fs = fixtures();
    for profile in Profile::ALL {
        for (name, rules_position) in [
            ("initial", LocalPosition::standard(profile)),
            (
                "area-burn",
                LocalPosition::admit(root(
                    &fs.iter().find(|f| f["id"] == "TEN-BURN-SURVIVE").unwrap()["given"]["root"],
                    profile,
                ))
                .unwrap(),
            ),
        ] {
            let fast = TenjikuPosition::from_local(&rules_position);
            let iterations = 300;
            let start = std::time::Instant::now();
            let mut rules_count = 0;
            for _ in 0..iterations {
                rules_count += std::hint::black_box(rules_position.legal_actions()).len();
            }
            let us = start.elapsed().as_micros();
            let start = std::time::Instant::now();
            let mut fast_count = 0;
            for _ in 0..iterations {
                let mut v = Vec::new();
                fast.generate(Stage::Tactical, &mut v);
                fast.generate(Stage::Quiet, &mut v);
                fast_count += std::hint::black_box(v).len();
            }
            let fast_us = start.elapsed().as_micros();
            assert_eq!(rules_count, fast_count);
            eprintln!(
                "{profile:?} {name}: rounds={iterations},actions={rules_count},us={us},fast_us={fast_us},speedup={:.2}",
                us as f64 / fast_us as f64
            );
        }
    }
}
#[test]
fn complete_compound_perft_two_agrees_for_both_profiles() {
    for profile in Profile::ALL {
        for (base, promoted) in [
            (Base::LHk, false),
            (Base::FiD, false),
            (Base::VGn, false),
            (Base::ChS, true),
        ] {
            let pos = diagram(
                profile,
                &[
                    (1, 16, Side::Sente, Base::K, false),
                    (8, 8, Side::Sente, base, promoted),
                    (16, 1, Side::Gote, Base::K, false),
                    (15, 3, Side::Gote, Base::P, false),
                ],
                Side::Sente,
            );
            let n = LocalPosition::admit(pos).unwrap();
            let mut p = TenjikuPosition::from_local(&n);
            assert_eq!(
                p.perft(2),
                perft(&n, 2, 1_000_000, || false).unwrap().leaves,
                "{profile:?} {base:?}"
            );
            state(&p, n.position());
        }
    }
}
#[test]
fn private_repeat_approximation_never_changes_root_admission() {
    let fs = fixtures();
    for profile in Profile::ALL {
        for id in ["TEN-REPETITION-CONCRETE", "TEN-REPETITION-RIGHTS-CONCRETE"] {
            let f = fs.iter().find(|f| f["id"] == id).unwrap();
            let mut game = Game::from_root(root(&f["given"]["root"], profile)).unwrap();
            let commands = f["given"]["commands"].as_array().unwrap();
            for command in &commands[..commands.len() - 1] {
                game.submit(GameRequest {
                    expected: game.context(),
                    actor: game.position().side_to_move(),
                    command: parse!(&command["command"]),
                })
                .unwrap();
            }
            let action: Action = parse!(&commands.last().unwrap()["command"]["action"]);
            assert!(!game.legal_actions().contains(&action));
            let mut private = TenjikuPosition::from_game(&game);
            let before = private.hash();
            assert!(private.make(private.find(&action).unwrap()));
            assert_eq!(
                private.rule_score(),
                Some(0),
                "private repeated state is only a neutral search estimate"
            );
            private.unmake();
            assert_eq!(private.hash(), before);
            private.restrict_root(&game.legal_actions());
            assert!(private.find(&action).is_none());
            assert!(!actions(&mut private).contains(&action));
        }
    }
}
#[test]
#[ignore = "manual release constructor benchmark; run without other builds"]
fn benchmark_known_history_constructor() {
    for profile in Profile::ALL {
        let mut game = Game::from_root(diagram(
            profile,
            &[
                (16, 16, Side::Sente, Base::K, false),
                (15, 14, Side::Sente, Base::P, false),
                (1, 1, Side::Gote, Base::K, false),
                (2, 3, Side::Gote, Base::P, false),
            ],
            Side::Sente,
        ))
        .unwrap();
        let mut seed = 0x183c0ffeeu64;
        let mut refusal = None;
        for _ in 0..1020 {
            let owner = game.position().side_to_move();
            let actions: Vec<_> = game
                .legal_actions()
                .into_iter()
                .filter(|a| {
                    game.position().piece(a.from).unwrap().base == Base::K
                        && a.to().is_some_and(|s| {
                            if owner == Side::Sente {
                                s.file() >= 10 && s.rank() >= 10
                            } else {
                                s.file() <= 7 && s.rank() <= 7
                            }
                        })
                })
                .collect();
            if actions.is_empty() {
                refusal = Some("no unused king route".to_string());
                break;
            }
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let a = actions[seed as usize % actions.len()].clone();
            if let Err(e) = game.submit(GameRequest {
                expected: game.context(),
                actor: owner,
                command: GameCommand::Play { action: a },
            }) {
                refusal = Some(format!("{e:?}"));
                break;
            }
        }
        assert!(game.events().len() >= 128);
        let start = std::time::Instant::now();
        let repeats = 100;
        for _ in 0..repeats {
            let p = std::hint::black_box(TenjikuPosition::from_game(&game));
            assert_eq!(p.side_to_move(), game.position().side_to_move());
        }
        let micros = start.elapsed().as_micros();
        let p = TenjikuPosition::from_game(&game);
        state(&p, game.position());
        eprintln!(
            "{profile:?} history: accepted_events={},refusal={refusal:?},rounds={repeats},constructor_total_us={micros},mean_us={:.2}",
            game.events().len(),
            micros as f64 / repeats as f64
        );
    }
}
