//! WXF2018 Article7.1-7.5 display adapter; canonical ICCS stays unchanged.
use mshogi_core::xiangqi::{Action, PieceKind, Position, Side};
pub(super) fn notation(position: &Position, action: Action) -> String {
    let piece = position.piece(action.from).expect("admitted source");
    let file = |f: u8| {
        if piece.side == Side::Red {
            9 - f
        } else {
            f + 1
        }
    };
    let mut peers: Vec<_> = position
        .pieces()
        .filter(|(at, p)| {
            p.side == piece.side && p.kind == piece.kind && at.file() == action.from.file()
        })
        .map(|(at, _)| at)
        .collect();
    peers.sort_by_key(|at| {
        if piece.side == Side::Red {
            9 - at.rank()
        } else {
            at.rank()
        }
    });
    // Article7.5 explicitly illustrates three soldiers as ordinal+file.
    // Owner-approved MSH123 display completion extends this to4/5 and to
    // simultaneous multi-file stacks whose +/-P spelling could collide.
    let paired_soldier_files = (0..9)
        .filter(|file| {
            position
                .pieces()
                .filter(|(at, p)| {
                    p.side == piece.side && p.kind == PieceKind::Soldier && at.file() == *file
                })
                .count()
                >= 2
        })
        .count();
    let ordinal = piece.kind == PieceKind::Soldier
        && (peers.len() >= 3 || (peers.len() == 2 && paired_soldier_files > 1));
    let prefix = if ordinal {
        format!(
            "{}{}",
            peers.iter().position(|p| *p == action.from).unwrap() + 1,
            file(action.from.file())
        )
    } else if peers.len() == 2 {
        format!(
            "{}{}",
            if peers[0] == action.from { '+' } else { '-' },
            letter(piece.kind)
        )
    } else {
        format!("{}{}", letter(piece.kind), file(action.from.file()))
    };
    let direction = if action.from.rank() == action.to.rank() {
        '='
    } else if (i16::from(action.to.rank()) - i16::from(action.from.rank()))
        * if piece.side == Side::Red { 1 } else { -1 }
        > 0
    {
        '+'
    } else {
        '-'
    };
    let value = if direction == '='
        || matches!(
            piece.kind,
            PieceKind::Advisor | PieceKind::Elephant | PieceKind::Horse
        ) {
        file(action.to.file())
    } else {
        action.from.rank().abs_diff(action.to.rank())
    };
    format!("{prefix}{direction}{value}")
}
fn letter(kind: PieceKind) -> char {
    match kind {
        PieceKind::General => 'K',
        PieceKind::Advisor => 'A',
        PieceKind::Elephant => 'E',
        PieceKind::Horse => 'H',
        PieceKind::Rook => 'R',
        PieceKind::Cannon => 'C',
        PieceKind::Soldier => 'P',
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn article7_initial_examples_both_movers() {
        let p = Position::initial();
        for (a, s) in [("h2e2", "C2=5"), ("b0c2", "H8+7"), ("g0e2", "E3+5")] {
            let a: Action = a.parse().unwrap();
            assert!(p.legal_actions().unwrap().contains(&a));
            assert_eq!(notation(&p, a), s);
        }
        let p = Position::from_fen(
            "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR b - - 0 1",
        )
        .unwrap();
        for (a, s) in [("b9c7", "H2+3"), ("h7e7", "C8=5")] {
            assert_eq!(notation(&p, a.parse().unwrap()), s);
        }
    }
}

#[cfg(test)]
mod fixture_tests {
    use super::*;
    fn position(side: Side, extra: &[(&str, Side, PieceKind)]) -> Position {
        let mut board = [None; 90];
        for (text, owner, kind) in [
            (
                if side == Side::Red { "e0" } else { "d0" },
                Side::Red,
                PieceKind::General,
            ),
            (
                if side == Side::Red { "f9" } else { "e9" },
                Side::Black,
                PieceKind::General,
            ),
        ]
        .into_iter()
        .chain(extra.iter().copied())
        {
            let point: mshogi_core::xiangqi::Intersection = text.parse().unwrap();
            board[point.index()] = Some(mshogi_core::xiangqi::Piece::new(owner, kind));
        }
        Position::from_diagram(
            &board
                .into_iter()
                .enumerate()
                .filter_map(|(i, p)| {
                    p.map(|p| {
                        (
                            mshogi_core::xiangqi::Intersection::new((i % 9) as u8, (i / 9) as u8)
                                .unwrap(),
                            p,
                        )
                    })
                })
                .collect::<Vec<_>>(),
            side,
            mshogi_core::xiangqi::Counters::unknown(),
        )
        .unwrap()
    }
    fn expect(p: &Position, action: &str, text: &str) {
        let a: Action = action.parse().unwrap();
        assert!(p.legal_actions().unwrap().contains(&a), "{action}");
        assert_eq!(notation(p, a), text);
    }
    #[test]
    fn article7_front_rear_and_distance_both_sides() {
        let p = position(
            Side::Red,
            &[
                ("e3", Side::Red, PieceKind::Cannon),
                ("e6", Side::Red, PieceKind::Cannon),
            ],
        );
        expect(&p, "e6d6", "+C=6");
        expect(&p, "e3d3", "-C=6");
        let p = position(
            Side::Black,
            &[
                ("e3", Side::Black, PieceKind::Cannon),
                ("e6", Side::Black, PieceKind::Cannon),
            ],
        );
        expect(&p, "e3d3", "+C=4");
        expect(&p, "e6d6", "-C=4");
        let p = position(Side::Red, &[("e4", Side::Red, PieceKind::Rook)]);
        expect(&p, "e4e1", "R5-3");
        let p = position(Side::Black, &[("e6", Side::Black, PieceKind::Rook)]);
        expect(&p, "e6e8", "R5-2");
    }
    // Source figureB, English PDF24–25 (printed16–17), independently transcribed.
    #[test]
    fn article7_figure_b_three_soldiers() {
        let p = position(
            Side::Red,
            &[
                ("d8", Side::Red, PieceKind::Soldier),
                ("d6", Side::Red, PieceKind::Soldier),
                ("d5", Side::Red, PieceKind::Soldier),
                ("d0", Side::Red, PieceKind::Advisor),
                ("c0", Side::Red, PieceKind::Elephant),
                ("d9", Side::Black, PieceKind::Advisor),
                ("e8", Side::Black, PieceKind::Advisor),
                ("c9", Side::Black, PieceKind::Elephant),
                ("e7", Side::Black, PieceKind::Elephant),
                ("a2", Side::Black, PieceKind::Rook),
            ],
        );
        expect(&p, "d5e5", "36=5");
        expect(&p, "d6e6", "26=5");
        expect(&p, "d8e8", "16=5");
    }
    #[test]
    fn approved_four_five_and_multiple_file_extension() {
        let p = position(
            Side::Red,
            &[
                ("d8", Side::Red, PieceKind::Soldier),
                ("d7", Side::Red, PieceKind::Soldier),
                ("d6", Side::Red, PieceKind::Soldier),
                ("d5", Side::Red, PieceKind::Soldier),
            ],
        );
        expect(&p, "d5e5", "46=5");
        let p = position(
            Side::Red,
            &[
                ("d9", Side::Red, PieceKind::Soldier),
                ("d8", Side::Red, PieceKind::Soldier),
                ("d7", Side::Red, PieceKind::Soldier),
                ("d6", Side::Red, PieceKind::Soldier),
                ("d5", Side::Red, PieceKind::Soldier),
            ],
        );
        expect(&p, "d5e5", "56=5");
        let p = position(
            Side::Black,
            &[
                ("f0", Side::Black, PieceKind::Soldier),
                ("f1", Side::Black, PieceKind::Soldier),
                ("f2", Side::Black, PieceKind::Soldier),
                ("f3", Side::Black, PieceKind::Soldier),
                ("f4", Side::Black, PieceKind::Soldier),
            ],
        );
        expect(&p, "f4e4", "56=5");
        let p = position(
            Side::Red,
            &[
                ("c5", Side::Red, PieceKind::Soldier),
                ("c6", Side::Red, PieceKind::Soldier),
                ("g5", Side::Red, PieceKind::Soldier),
                ("g6", Side::Red, PieceKind::Soldier),
            ],
        );
        expect(&p, "c6d6", "17=6");
        expect(&p, "c5d5", "27=6");
        expect(&p, "g6f6", "13=4");
        expect(&p, "g5f5", "23=4");
    }
}
