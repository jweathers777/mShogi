//! The Tori search position against the native Tori rules, which are the
//! authority: legal actions (moves and drops), resulting state, check status,
//! hash and move encoding.
use super::position::{Move, ToriPosition};
use crate::alphabeta::mate::{self, MateLimits, MateResult, MateTable};
use crate::alphabeta::{MATE_SCORE, SearchPosition, Stage};
use mshogi_core::tori::{
    Action, Face, Game, GameCommand, GameRequest, Hand, Piece, Position, RootMetadata, Side, Square,
};

struct Random(u64);
impl Random {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}
fn sorted(mut actions: Vec<Action>) -> Vec<String> {
    let mut text: Vec<String> = actions.drain(..).map(|a| format!("{a:?}")).collect();
    text.sort();
    text
}
fn play(game: &mut Game, action: Action) {
    game.submit(GameRequest {
        expected: game.context(),
        actor: game.position().side_to_move(),
        command: GameCommand::Play { action },
    })
    .unwrap();
}
fn diagram(entries: &[(&str, Side, Face)], hands: [Hand; 2], turn: Side) -> Game {
    Game::from_root(
        Position::from_diagram(
            &entries
                .iter()
                .map(|(s, side, face)| {
                    (
                        s.parse().unwrap(),
                        Piece {
                            side: *side,
                            face: *face,
                        },
                    )
                })
                .collect::<Vec<_>>(),
            hands,
            turn,
            RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap()
}
/// Compare one search position with the native position of `game`.
fn compare(position: &mut ToriPosition, game: &Game) {
    let native = game.position();
    let local = game.local_position();
    assert_eq!(position.side_to_move(), native.side_to_move());
    for square in Square::all() {
        assert_eq!(position.piece(square), native.piece(square), "{square}");
    }
    for side in [Side::Sente, Side::Gote] {
        for face in Face::HANDS {
            assert_eq!(
                position.hand_count(side, face),
                native.hand(side).count(face)
            );
        }
        let material = super::evaluate(native);
        assert_eq!(
            position.material(side),
            match side {
                Side::Sente => material.sente,
                Side::Gote => material.gote,
            }
        );
    }
    assert_eq!(position.hash(), position.hash_from_scratch());
    assert_eq!(
        position.in_check(),
        local.is_in_check(native.side_to_move())
    );
    // Legal actions without the history rule: the position-local rules.
    let legal = position.legal_moves();
    let actions: Vec<Action> = legal.iter().map(|&m| position.action(m)).collect();
    assert_eq!(sorted(actions.clone()), sorted(local.legal_actions()));
    for (&m, &action) in legal.iter().zip(&actions) {
        assert_eq!(position.find(action), Some(m));
    }
    // The two stages give each pseudo-legal move one time, and the table
    // decoding accepts exactly those moves.
    let mut pseudo = Vec::new();
    position.generate(Stage::Tactical, &mut pseudo);
    let tactical = pseudo.len();
    position.generate(Stage::Quiet, &mut pseudo);
    for (index, &m) in pseudo.iter().enumerate() {
        assert_eq!(position.is_tactical(m), index < tactical);
        assert!(!pseudo[..index].contains(&m));
    }
    let mut decoded = 0;
    for bits in 0..=4097u32 {
        if let Some(m) = position.decode(bits) {
            assert_eq!(ToriPosition::encode(m), bits);
            assert!(pseudo.contains(&m), "{m:?}");
            decoded += 1;
        }
    }
    assert_eq!(decoded, pseudo.len());
}

#[test]
fn random_games_agree_with_the_native_rules() {
    let mut positions = 0;
    let mut drops = 0;
    let mut captures = 0;
    let mut promotions = 0;
    for seed in 1..=48u64 {
        let mut random = Random(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        let mut game = Game::standard();
        let mut position = ToriPosition::from_game(&game);
        for _ in 0..140 {
            compare(&mut position, &game);
            positions += 1;
            // A copy made from the game has the same hash as the played copy.
            if positions % 16 == 1 {
                assert_eq!(ToriPosition::from_game(&game).hash(), position.hash());
            }
            if game.conclusion().is_some() {
                assert!(game.legal_actions().is_empty());
                break;
            }
            let legal = game.legal_actions();
            // Prefer drops and captures in part of the games, so that hands,
            // the Swallow rules and promotions get much use.
            let wanted: Vec<Action> = legal
                .iter()
                .copied()
                .filter(|action| match action {
                    Action::Drop { .. } => seed.is_multiple_of(3),
                    Action::Move { to, .. } => {
                        seed.is_multiple_of(2) && game.position().piece(*to).is_some()
                    }
                })
                .collect();
            let pool = if wanted.is_empty() || random.next().is_multiple_of(4) {
                &legal
            } else {
                &wanted
            };
            let action = pool[(random.next() % pool.len() as u64) as usize];
            match action {
                Action::Drop { .. } => drops += 1,
                Action::Move {
                    from,
                    to,
                    result_face,
                } => {
                    captures += usize::from(game.position().piece(to).is_some());
                    promotions +=
                        usize::from(game.position().piece(from).unwrap().face != result_face);
                }
            }
            let m = position.find(action).expect("native action exists");
            let before = position.hash();
            assert!(position.make(m));
            position.unmake();
            assert_eq!(position.hash(), before);
            assert!(position.make(m));
            play(&mut game, action);
        }
    }
    assert!(positions > 2_000, "{positions}");
    assert!(
        drops > 200 && captures > 200 && promotions > 40,
        "{drops} {captures} {promotions}"
    );
}

/// The check flag after every legal move agrees with the native rules. The
/// search position derives it from the moved piece and the rays that its
/// departure opens (MSH-200), so each child is checked, not only the path.
/// The check generator's moves: pseudo-legal and without repeats.
fn generated_checks(position: &ToriPosition) -> Vec<Move> {
    let mut pseudo = Vec::new();
    position.generate(Stage::Tactical, &mut pseudo);
    position.generate(Stage::Quiet, &mut pseudo);
    let mut candidates = Vec::new();
    position.generate_checks(&mut candidates);
    for (index, m) in candidates.iter().enumerate() {
        assert!(pseudo.contains(m), "{:?}", position.action(*m));
        assert!(!candidates[..index].contains(m));
    }
    candidates
}

#[test]
fn checks_check_generation_and_evasions_agree_with_the_native_rules() {
    let mut children = 0;
    let mut checks = 0;
    let mut ray_pieces = 0;
    let mut evasions = 0;
    for seed in 1..=48u64 {
        let mut random = Random(seed.wrapping_mul(0xD1B5_4A32_D192_ED03) | 1);
        let mut game = Game::standard();
        let mut position = ToriPosition::from_game(&game);
        for _ in 0..120 {
            if game.conclusion().is_some() {
                break;
            }
            let mut local = game.local_position().clone();
            let mover = local.position().side_to_move();
            let candidates = generated_checks(&position);
            let legal = position.legal_moves();
            for &m in &legal {
                let action = position.action(m);
                assert!(position.make(m));
                let (_, mut undo) = local.apply(action).unwrap();
                assert_eq!(
                    position.in_check(),
                    local.is_in_check(mover.opponent()),
                    "{action:?}"
                );
                children += 1;
                if position.in_check() {
                    checks += 1;
                    assert!(candidates.contains(&m), "missing check {action:?}");
                }
                local.undo(&mut undo).unwrap();
                position.unmake();
            }
            // The search generator (in check: drops only between the
            // Phoenix and a single ray piece) keeps every legal move.
            let mut search = Vec::new();
            position.generate_search(Stage::Tactical, &mut search);
            position.generate_search(Stage::Quiet, &mut search);
            search.retain(|&m| {
                let kept = position.make(m);
                if kept {
                    position.unmake();
                }
                kept
            });
            assert_eq!(
                sorted(search.iter().map(|&m| position.action(m)).collect()),
                sorted(legal.iter().map(|&m| position.action(m)).collect())
            );
            evasions += usize::from(position.in_check());
            ray_pieces += Square::all()
                .filter(|&square| {
                    matches!(
                        game.position().piece(square),
                        Some(Piece {
                            face: Face::Eagle | Face::LeftQuail | Face::RightQuail,
                            ..
                        })
                    )
                })
                .count();
            // Prefer captures and drops, so that Eagles, Quails and drops
            // are frequent.
            let legal = game.legal_actions();
            let wanted: Vec<Action> = legal
                .iter()
                .copied()
                .filter(|action| match action {
                    Action::Drop { .. } => true,
                    Action::Move { to, .. } => game.position().piece(*to).is_some(),
                })
                .collect();
            let pool = if wanted.is_empty() || random.next().is_multiple_of(3) {
                &legal
            } else {
                &wanted
            };
            let action = pool[(random.next() % pool.len() as u64) as usize];
            assert!(position.make(position.find(action).unwrap()));
            play(&mut game, action);
        }
    }
    assert!(
        children > 50_000 && checks > 1_000 && ray_pieces > 1_000 && evasions > 100,
        "{children} {checks} {ray_pieces} {evasions}"
    );
}

#[test]
fn perft_agrees_with_the_native_perft() {
    let game = Game::standard();
    let mut position = ToriPosition::from_game(&game);
    for depth in 1..=3u8 {
        let native = game
            .local_position()
            .perft(depth, 1_000_000)
            .unwrap()
            .leaves;
        assert_eq!(position.perft(u32::from(depth)), native, "depth {depth}");
    }
    // A position with drops for both sides, reached by fixed exchanges.
    let mut game = Game::standard();
    let mut random = Random(165);
    for _ in 0..30 {
        let legal = game.legal_actions();
        let captures: Vec<_> = legal
            .iter()
            .copied()
            .filter(
                |a| matches!(a, Action::Move { to, .. } if game.position().piece(*to).is_some()),
            )
            .collect();
        let pool = if captures.is_empty() {
            &legal
        } else {
            &captures
        };
        let action = pool[(random.next() % pool.len() as u64) as usize];
        play(&mut game, action);
        if game.conclusion().is_some() {
            panic!("fixed walk must not end the game");
        }
    }
    assert!(
        Face::HANDS
            .into_iter()
            .any(|f| game.position().hand(Side::Sente).count(f) > 0)
    );
    let mut position = ToriPosition::from_game(&game);
    for depth in 1..=2u8 {
        let native = game
            .local_position()
            .perft(depth, 1_000_000)
            .unwrap()
            .leaves;
        assert_eq!(position.perft(u32::from(depth)), native, "depth {depth}");
    }
}

#[test]
fn swallow_drop_rules_agree_with_the_native_rules() {
    let drop = |to: &str| Action::Drop {
        face: Face::Swallow,
        to: to.parse().unwrap(),
    };
    let swallow = Hand::from_counts(&[(Face::Swallow, 1)]).unwrap();
    // The Gote Phoenix on 4a has no flight square, and the Pheasant on 4d
    // protects 4b: a Swallow drop on 4b is checkmate, so it is not legal.
    let boxed = [
        ("7g", Side::Sente, Face::Phoenix),
        ("4a", Side::Gote, Face::Phoenix),
        ("4d", Side::Sente, Face::Pheasant),
        ("3a", Side::Gote, Face::Swallow),
        ("5a", Side::Gote, Face::Swallow),
        ("3b", Side::Gote, Face::Swallow),
        ("5b", Side::Gote, Face::Swallow),
    ];
    let game = diagram(&boxed, [swallow, Hand::empty()], Side::Sente);
    let mut position = ToriPosition::from_game(&game);
    compare(&mut position, &game);
    assert!(position.find(drop("4b")).is_some(), "pseudo-legal");
    let m = position.find(drop("4b")).unwrap();
    assert!(!position.make(m));
    assert_eq!(position.hash(), position.hash_from_scratch());
    // Without the Pheasant the Phoenix can capture the Swallow: the drop is legal.
    let open: Vec<_> = boxed
        .iter()
        .copied()
        .filter(|entry| entry.2 != Face::Pheasant)
        .collect();
    let game = diagram(&open, [swallow, Hand::empty()], Side::Sente);
    let mut position = ToriPosition::from_game(&game);
    compare(&mut position, &game);
    let m = position.find(drop("4b")).unwrap();
    assert!(position.make(m));
    // Two Swallows of one side on a file, and the last rank.
    let game = diagram(
        &[
            ("7g", Side::Sente, Face::Phoenix),
            ("1a", Side::Gote, Face::Phoenix),
            ("4e", Side::Sente, Face::Swallow),
            ("4c", Side::Sente, Face::Swallow),
            ("5d", Side::Sente, Face::Swallow),
            ("3d", Side::Gote, Face::Swallow),
            ("3e", Side::Gote, Face::Swallow),
        ],
        [swallow, swallow],
        Side::Sente,
    );
    let mut position = ToriPosition::from_game(&game);
    compare(&mut position, &game);
    assert!(
        position.find(drop("4d")).is_none(),
        "third Swallow on a file"
    );
    assert!(position.find(drop("5a")).is_none(), "last rank");
    assert!(position.find(drop("5b")).is_some());
    assert!(
        position.find(drop("3c")).is_some(),
        "the other side's Swallows do not count"
    );
}

#[test]
fn the_root_excludes_departures_that_the_native_repetition_rule_forbids() {
    // Both sides move a Phoenix out and back until the start position occurs
    // for the third time. The native rule then forbids the earlier departures.
    let mut game = diagram(
        &[
            ("4g", Side::Sente, Face::Phoenix),
            ("4a", Side::Gote, Face::Phoenix),
            ("1g", Side::Sente, Face::Crane),
            ("7a", Side::Gote, Face::Crane),
        ],
        [Hand::empty(); 2],
        Side::Sente,
    );
    let step = |from: &str, to: &str| Action::Move {
        from: from.parse().unwrap(),
        to: to.parse().unwrap(),
        result_face: Face::Phoenix,
    };
    // Seven plies: the start key occurred at 0 and 4, so Gote's 5a-4a would
    // make its third occurrence (T06, MSH-208).
    let cycle = [("4g", "5g"), ("4a", "5a"), ("5g", "4g"), ("5a", "4a")];
    for &(from, to) in cycle.iter().cycle().take(7) {
        play(&mut game, step(from, to));
    }
    let repetition = game.repetition().expect("third occurrence").clone();
    assert_eq!(repetition.forbidden, vec![step("5a", "4a")]);
    let position = ToriPosition::from_game(&game);
    let mut moves: Vec<Move> = Vec::new();
    position.generate(Stage::Tactical, &mut moves);
    position.generate(Stage::Quiet, &mut moves);
    let actions: Vec<Action> = moves.iter().map(|&m| position.action(m)).collect();
    assert!(!actions.contains(&step("5a", "4a")));
    assert!(actions.contains(&step("5a", "5b")));
    for action in game.legal_actions() {
        assert!(actions.contains(&action));
    }
    // A new position has no repetition score.
    let mut position = position;
    let m = position.find(step("5a", "5b")).unwrap();
    assert!(position.make(m));
    assert_eq!(position.rule_score(), None);
    position.unmake();
    // Inside the search, Sente's return to the root key (seen at plies 3 and
    // 7 of the game) is its third occurrence: illegal, so Gote wins.
    let mut path = Vec::new();
    for (from, to) in [("5a", "5b"), ("4g", "3g"), ("5b", "5a"), ("3g", "4g")] {
        let m = position.find(step(from, to)).unwrap();
        assert!(position.make(m));
        path.push(m);
    }
    assert_eq!(position.rule_score(), Some(MATE_SCORE - 4));
    for _ in &path {
        position.unmake();
    }
    // A position with no earlier history needs a return inside the search.
    let fresh = Game::from_root(
        Position::from_diagram(
            &[
                ("4g", Side::Sente, Face::Phoenix),
                ("4a", Side::Gote, Face::Phoenix),
                ("1g", Side::Sente, Face::Crane),
                ("7a", Side::Gote, Face::Crane),
            ]
            .iter()
            .map(|(s, side, face)| {
                (
                    s.parse().unwrap(),
                    Piece {
                        side: *side,
                        face: *face,
                    },
                )
            })
            .collect::<Vec<_>>(),
            [Hand::empty(); 2],
            Side::Sente,
            RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap();
    let mut position = ToriPosition::from_game(&fresh);
    // First return: a level cycle. Second return: the third occurrence.
    for (index, (from, to)) in cycle.iter().copied().cycle().take(8).enumerate() {
        let m = position.find(step(from, to)).unwrap();
        assert!(position.make(m));
        let expected = match index {
            3 => Some(0),
            7 => Some(MATE_SCORE - 8),
            _ if index > 3 => Some(0),
            _ => None,
        };
        assert_eq!(position.rule_score(), expected, "ply {index}");
    }
}

#[test]
fn history_keys_survive_stop_events_between_actions() {
    // A Stop command adds an event without a board action. The search copy
    // must see each earlier position one time, as without the Stop events.
    let step = |from: &str, to: &str| Action::Move {
        from: from.parse().unwrap(),
        to: to.parse().unwrap(),
        result_face: Face::Phoenix,
    };
    let cycle = [("4g", "5g"), ("4a", "5a"), ("5g", "4g"), ("5a", "4a")];
    let build = |stops: bool| {
        let mut game = diagram(
            &[
                ("4g", Side::Sente, Face::Phoenix),
                ("4a", Side::Gote, Face::Phoenix),
                ("1g", Side::Sente, Face::Crane),
                ("7a", Side::Gote, Face::Crane),
            ],
            [Hand::empty(); 2],
            Side::Sente,
        );
        // Six actions: the start position occurs two times, and the root is
        // the second occurrence of the position after 4g-5g and 4a-5a.
        for (index, &(from, to)) in cycle.iter().cycle().take(6).enumerate() {
            play(&mut game, step(from, to));
            if stops && (index == 0 || index == 4) {
                game.submit(GameRequest {
                    expected: game.context(),
                    actor: game.position().side_to_move(),
                    command: GameCommand::Stop {
                        reason: mshogi_core::tori::StopReason::Cancelled,
                    },
                })
                .unwrap();
            }
        }
        game
    };
    let mut scores = Vec::new();
    for stops in [false, true] {
        let game = build(stops);
        assert_eq!(game.events().len(), if stops { 8 } else { 6 });
        let mut position = ToriPosition::from_game(&game);
        let mut seen = Vec::new();
        for &(from, to) in &cycle[2..] {
            let m = position.find(step(from, to)).unwrap();
            assert!(position.make(m));
            seen.push(position.rule_score());
        }
        scores.push(seen);
    }
    // The position after 5g-4g occurred one time before: no score. The start
    // position after 5a-4a occurred two times before the root: its third
    // occurrence is illegal (T06, MSH-208), a win for the side to move.
    assert_eq!(scores[0], vec![None, Some(MATE_SCORE - 2)]);
    assert_eq!(scores[1], scores[0]);
}

#[test]
fn gote_swallow_drop_rules_agree_with_the_native_rules() {
    // The Sente drop-mate position of the test above, turned for Gote.
    let drop = |to: &str| Action::Drop {
        face: Face::Swallow,
        to: to.parse().unwrap(),
    };
    let swallow = Hand::from_counts(&[(Face::Swallow, 1)]).unwrap();
    let boxed = [
        ("1a", Side::Gote, Face::Phoenix),
        ("4g", Side::Sente, Face::Phoenix),
        ("4d", Side::Gote, Face::Pheasant),
        ("5g", Side::Sente, Face::Swallow),
        ("3g", Side::Sente, Face::Swallow),
        ("5f", Side::Sente, Face::Swallow),
        ("3f", Side::Sente, Face::Swallow),
    ];
    let game = diagram(&boxed, [Hand::empty(), swallow], Side::Gote);
    let mut position = ToriPosition::from_game(&game);
    compare(&mut position, &game);
    let m = position.find(drop("4f")).expect("pseudo-legal");
    assert!(!position.make(m), "Swallow drop mate");
    assert!(position.find(drop("2g")).is_none(), "last rank for Gote");
    assert!(position.find(drop("2f")).is_some());
}

#[test]
fn static_exchange_uses_board_and_hand_values() {
    use Face::*;
    use Side::{Gote, Sente};
    // Each diagram adds the Sente Phoenix on 7g; Gote's Phoenix is on 1a
    // unless the diagram places it.
    let see = |entries: &[(&str, Side, Face)], hand: Hand, action: Action| {
        let mut all = vec![("7g", Sente, Phoenix)];
        if !entries.iter().any(|e| e.1 == Gote && e.2 == Phoenix) {
            all.push(("1a", Gote, Phoenix));
        }
        all.extend_from_slice(entries);
        let game = diagram(&all, [hand, Hand::empty()], Sente);
        let position = ToriPosition::from_game(&game);
        position.see(position.find(action).unwrap())
    };
    let take = |face| Action::Move {
        from: "4e".parse().unwrap(),
        to: "4d".parse().unwrap(),
        result_face: face,
    };
    let none = Hand::empty();
    let (swallow, target) = (("4e", Sente, Swallow), ("4d", Gote, Swallow));
    // An undefended Swallow: board value plus hand value.
    assert_eq!(see(&[swallow, target], none, take(Swallow)), 200);
    // Defended by a Crane: an equal trade.
    let crane = ("4c", Gote, Crane);
    assert_eq!(see(&[swallow, target, crane], none, take(Swallow)), 0);
    // A Falcon that takes the defended Swallow is lost (board and hand).
    let falcon = ("4e", Sente, Falcon);
    assert_eq!(
        see(&[falcon, target, crane], none, take(Falcon)),
        200 - 1400
    );
    // X-ray: the Left Quail behind the Swallow defends the target square,
    // so Gote does not recapture.
    let quail = ("4f", Sente, LeftQuail);
    assert_eq!(
        see(&[swallow, target, crane, quail], none, take(Swallow)),
        200
    );
    // The Phoenix recaptures an undefended piece, but not a defended one.
    let phoenix = ("4c", Gote, Phoenix);
    assert_eq!(see(&[swallow, target, phoenix], none, take(Swallow)), 0);
    let guard = ("3e", Sente, Crane);
    assert_eq!(
        see(&[swallow, target, phoenix, guard], none, take(Swallow)),
        200
    );
    // A Pheasant drop attacked by a Swallow loses the Pheasant for nothing.
    let pheasant = Hand::from_counts(&[(Pheasant, 1)]).unwrap();
    let drop = Action::Drop {
        face: Pheasant,
        to: "4d".parse().unwrap(),
    };
    assert_eq!(see(&[("4c", Gote, Swallow)], pheasant, drop), -600);
    assert_eq!(see(&[], pheasant, drop), 0);
}

#[test]
fn mate_search_finds_a_drop_mate_and_respects_the_swallow_rule() {
    use Face::*;
    use Side::{Gote, Sente};
    let boxed = [
        ("7g", Sente, Phoenix),
        ("4a", Gote, Phoenix),
        ("4d", Sente, Pheasant),
        ("3a", Gote, Swallow),
        ("5a", Gote, Swallow),
        ("3b", Gote, Swallow),
        ("5b", Gote, Swallow),
    ];
    let search = |hand: Hand| {
        let game = diagram(&boxed, [hand, Hand::empty()], Sente);
        let mut position = ToriPosition::from_game(&game);
        let before = position.hash();
        let mut table = MateTable::new(16);
        let limits = MateLimits {
            nodes: 100_000,
            deadline: None,
        };
        let (result, _) = mate::search(&mut position, &mut table, limits, &mut || false);
        assert_eq!(position.hash(), before);
        (game, position, result)
    };
    // A Crane drop on 4b, protected by the Pheasant, is checkmate.
    let (game, mut position, result) = search(Hand::from_counts(&[(Crane, 1)]).unwrap());
    let MateResult::Mate(line) = result else {
        panic!("no mate found");
    };
    let mut native = game.local_position().clone();
    for &m in &line {
        native.apply(position.action(m)).unwrap();
        assert!(position.make(m));
    }
    let defender = native.position().side_to_move();
    assert!(native.legal_actions().is_empty() && native.is_in_check(defender));
    // The same drop with a Swallow is illegal: no mate.
    let (_, _, result) = search(Hand::from_counts(&[(Swallow, 1)]).unwrap());
    assert!(!matches!(result, MateResult::Mate(_)));
}

#[test]
fn a_falcon_that_promotes_checks_through_the_square_it_leaves() {
    use Face::*;
    use Side::{Gote, Sente};
    // The Falcon steps from 4c into the zone and becomes an Eagle, whose
    // backward ray passes 4c to the Phoenix on 4e.
    let game = diagram(
        &[
            ("7g", Sente, Phoenix),
            ("4c", Sente, Falcon),
            ("4e", Gote, Phoenix),
        ],
        [Hand::empty(), Hand::empty()],
        Sente,
    );
    let mut position = ToriPosition::from_game(&game);
    let m = position
        .find(Action::Move {
            from: "4c".parse().unwrap(),
            to: "4b".parse().unwrap(),
            result_face: Eagle,
        })
        .unwrap();
    assert!(position.make(m) && position.in_check());
    position.unmake();
    assert!(generated_checks(&position).contains(&m));
}
