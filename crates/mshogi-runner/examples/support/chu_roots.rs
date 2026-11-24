//! Fixed native Chu development roots, shared by calibration and strength tools.
#![allow(dead_code)]
use mshogi_core::chu::*;

fn diagram(entries: &[(&str, Side, Base, bool)], turn: Side) -> Game {
    Game::from_root(
        Position::from_diagram(
            &entries
                .iter()
                .map(|(s, side, base, promoted)| {
                    (
                        s.parse().unwrap(),
                        Piece::new(*side, *base, *promoted).unwrap(),
                    )
                })
                .collect::<Vec<_>>(),
            turn,
            RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap()
}

/// Equal material and equal royal count, with each Gote placement obtained by
/// rotating its Sente counterpart. Diagram reachability is not claimed.
pub fn compact(family: usize) -> Game {
    use Base::*;
    let mut pieces = vec![
        ("7k", King, false),
        ("7h", Lion, false),
        ("5g", Pawn, false),
        ("8g", Pawn, false),
    ];
    let extra: &[(&str, Base, bool)] = match family {
        0 => &[("10j", Rook, false), ("4k", Bishop, false)],
        1 => &[
            ("10j", DragonKing, false),
            ("4k", DragonHorse, false),
            ("6j", Gold, false),
            ("8j", Silver, false),
        ],
        2 => &[
            ("7j", Elephant, true),
            ("10j", Rook, false),
            ("4k", Bishop, false),
            ("6j", Gold, false),
        ],
        3 => &[
            ("11e", Phoenix, false),
            ("2f", Kirin, false),
            ("10h", SideMover, false),
            ("3h", VerticalMover, false),
            ("6j", Gold, false),
            ("8j", Silver, false),
        ],
        4 => &[
            ("7j", Elephant, false),
            ("10j", Rook, false),
            ("4k", Bishop, false),
            ("6j", Tiger, false),
            ("8j", Leopard, false),
            ("9i", Copper, false),
        ],
        5 => &[
            ("7j", Elephant, true),
            ("10j", DragonKing, false),
            ("4k", DragonHorse, false),
            ("10h", SideMover, false),
            ("3h", VerticalMover, false),
            ("6j", Gold, false),
            ("8j", Silver, false),
            ("9i", GoBetween, false),
        ],
        6 => &[
            ("10j", FreeKing, false),
            ("3j", Lance, false),
            ("10h", ReverseChariot, false),
            ("6j", Tiger, false),
            ("8j", Leopard, false),
            ("9i", Copper, false),
        ],
        7 => &[
            ("7j", Elephant, true),
            ("10j", FreeKing, false),
            ("11e", Phoenix, false),
            ("2f", Kirin, false),
            ("3h", Lance, false),
            ("6j", ReverseChariot, false),
            ("8j", Copper, false),
            ("9i", GoBetween, false),
        ],
        _ => unreachable!(),
    };
    pieces.extend_from_slice(extra);
    let mut entries = Vec::new();
    for (at, base, promoted) in pieces {
        let square: Square = at.parse().unwrap();
        entries.push((square, Piece::new(Side::Sente, base, promoted).unwrap()));
        entries.push((
            square.rotated(),
            Piece::new(Side::Gote, base, promoted).unwrap(),
        ));
    }
    Game::from_root(
        Position::from_diagram(&entries, Side::Sente, RootMetadata::diagram(0)).unwrap(),
    )
    .unwrap()
}

pub fn tactical(index: u64) -> Game {
    use Base::*;
    use Side::*;
    match index {
        // Existing search regression: Lion double capture and native material win.
        0 => diagram(
            &[
                ("12l", Sente, King, false),
                ("1a", Gote, King, false),
                ("6f", Sente, Lion, false),
                ("6e", Gote, Pawn, false),
                ("5e", Gote, Rook, false),
            ],
            Sente,
        ),
        // CHU-PROMOTE-Ki-YES native fixture, with both Golds retained.
        1 => diagram(
            &[
                ("1l", Sente, King, false),
                ("2l", Sente, Gold, false),
                ("12a", Gote, King, false),
                ("11a", Gote, Gold, false),
                ("6e", Sente, Kirin, false),
            ],
            Sente,
        ),
        // CHU-LION-DEFENDED-D2 native fixture.
        2 => diagram(
            &[
                ("12l", Sente, King, false),
                ("2l", Sente, Pawn, false),
                ("12a", Gote, King, false),
                ("11a", Gote, Pawn, false),
                ("6f", Sente, Lion, false),
                ("6d", Gote, Lion, false),
                ("6a", Gote, Rook, false),
            ],
            Sente,
        ),
        // CHU-ROYAL-SURVIVOR native fixture: capture one of two opposing royals.
        3 => diagram(
            &[
                ("12l", Sente, King, false),
                ("2l", Sente, Pawn, false),
                ("12a", Gote, King, false),
                ("11a", Gote, Pawn, false),
                ("8j", Sente, Elephant, true),
                ("12j", Gote, Rook, false),
            ],
            Gote,
        ),
        _ => panic!("unsupported tactical calibration seed"),
    }
}

pub fn quiet_setup(game: &mut Game, seed: u64, plies: u64, rear_only: bool) {
    let mut state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    for _ in 0..plies {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        // Setup changes neither material nor promotion. Every selected move is
        // still admitted by the full native Game, including must-vary rules.
        let choices: Vec<_> = game
            .legal_actions()
            .into_iter()
            .filter(|a| {
                let source = game.position().piece(a.from).unwrap();
                let rear = |s: &Square| match source.side {
                    Side::Sente => s.rank() >= 10,
                    Side::Gote => s.rank() <= 3,
                };
                (!rear_only
                    || (!source.is_royal() && !source.is_lion() && a.route.iter().all(rear)))
                    && !a.promote
                    && a.route
                        .iter()
                        .all(|s| *s == a.from || game.position().piece(*s).is_none())
            })
            .collect();
        assert!(!choices.is_empty());
        let action = choices[state as usize % choices.len()].clone();
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play { action },
        })
        .unwrap();
        assert!(game.conclusion().is_none());
    }
}
