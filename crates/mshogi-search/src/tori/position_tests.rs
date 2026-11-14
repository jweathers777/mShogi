//! The Tori search position against the native Tori rules, which are the
//! authority: legal actions (moves and drops), resulting state, check status,
//! hash and move encoding.
use super::position::{Move, ToriPosition};
use crate::alphabeta::{SearchPosition, Stage};
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
    for _ in 0..2 {
        for (from, to) in [("4g", "5g"), ("4a", "5a"), ("5g", "4g"), ("5a", "4a")] {
            play(&mut game, step(from, to));
        }
    }
    let obligation = game.obligation().expect("third occurrence").clone();
    assert_eq!(obligation.forbidden_departures, vec![step("4g", "5g")]);
    let position = ToriPosition::from_game(&game);
    let mut moves: Vec<Move> = Vec::new();
    position.generate(Stage::Tactical, &mut moves);
    position.generate(Stage::Quiet, &mut moves);
    let actions: Vec<Action> = moves.iter().map(|&m| position.action(m)).collect();
    assert!(!actions.contains(&step("4g", "5g")));
    assert!(actions.contains(&step("4g", "3g")));
    for action in game.legal_actions() {
        assert!(actions.contains(&action));
    }
    // The position counts the two earlier occurrences from the game.
    let mut position = position;
    let m = position.find(step("4g", "3g")).unwrap();
    assert!(position.make(m));
    assert_eq!(position.rule_score(), None);
}
