//! Development fixtures: actual native games, never a substitute rules engine.
use mshogi_core::tori as t;
pub fn contract_game() -> t::Game {
    use t::{Face::*, Side::*};
    let pieces = [
        ("4g", Sente, Phoenix),
        ("4a", Gote, Phoenix),
        ("7c", Sente, Falcon),
        ("7f", Sente, Eagle),
        ("6g", Sente, Crane),
        ("2g", Sente, Pheasant),
        ("7g", Sente, LeftQuail),
        ("1g", Sente, RightQuail),
        ("2d", Sente, Swallow),
        ("1f", Sente, Goose),
        ("7b", Gote, Swallow),
    ];
    let board = pieces
        .into_iter()
        .map(|(sq, side, face)| (sq.parse().unwrap(), t::Piece { side, face }))
        .collect::<Vec<_>>();
    t::Game::from_root(
        t::Position::from_diagram(
            &board,
            [
                t::Hand::from_counts(&[(RightQuail, 1), (Swallow, 2), (Crane, 1)]).unwrap(),
                t::Hand::from_counts(&[(LeftQuail, 1), (Swallow, 1), (Crane, 1), (Pheasant, 1)])
                    .unwrap(),
            ],
            Sente,
            t::RootMetadata::diagram(31),
        )
        .unwrap(),
    )
    .unwrap()
}
