use super::super::{Base, GameRecord, Piece, Position, Profile, RootMetadata, Side, Square};
use super::*;
use crate::perft::{PerftError, PerftReport, divide, perft};
fn square(file: u8, rank: u8) -> Square {
    Square::new(file, rank).unwrap()
}
fn diagram(profile: Profile) -> Game {
    let pieces = [
        (
            square(16, 16),
            Piece::new(0, Side::Sente, Base::K, false).unwrap(),
        ),
        (
            square(16, 14),
            Piece::new(1, Side::Sente, Base::P, false).unwrap(),
        ),
        (
            square(1, 1),
            Piece::new(2, Side::Gote, Base::K, false).unwrap(),
        ),
        (
            square(1, 3),
            Piece::new(3, Side::Gote, Base::P, false).unwrap(),
        ),
    ];
    let position =
        Position::from_diagram(profile, &pieces, Side::Sente, RootMetadata::diagram(0)).unwrap();
    Game::from_root(position).unwrap()
}
#[test]
fn independent_full_game_corner_king_and_remote_pawn_counts_and_divide() {
    // Each owner has three corner King moves and one remote Pawn move.
    // Every first choice leaves exactly those four opponent choices; no effect,
    // promotion, repeated boundary or bare-royal result intervenes at depth 2.
    for profile in Profile::ALL {
        let game = diagram(profile);
        let before = GameRecord::from_game(&game).unwrap();
        let authority = game.context();
        assert_eq!(
            perft(&game, 0, 1, || false).unwrap(),
            PerftReport {
                depth: 0,
                leaves: 1,
                visited: 1
            }
        );
        assert_eq!(
            perft(&game, 1, 5, || false).unwrap(),
            PerftReport {
                depth: 1,
                leaves: 4,
                visited: 5
            }
        );
        assert_eq!(
            perft(&game, 2, 21, || false).unwrap(),
            PerftReport {
                depth: 2,
                leaves: 16,
                visited: 21
            }
        );
        let report = divide(&game, 2, 21, || false).unwrap();
        assert_eq!(report.total.leaves, 16);
        assert_eq!(report.total.visited, 21);
        assert_eq!(report.entries.len(), 4);
        assert!(report.entries.iter().all(|entry| entry.leaves == 4));
        assert_eq!(GameRecord::from_game(&game).unwrap(), before);
        assert_eq!(game.context(), authority);
    }
}
#[test]
fn limits_budget_and_stop_never_return_partial_exact_or_mutate_original() {
    let game = diagram(Profile::AdamsModern);
    let before = GameRecord::from_game(&game).unwrap();
    let context = game.context();
    assert!(matches!(
        perft(&game, 2, 20, || false),
        Err(PerftError::WorkLimit { .. })
    ));
    assert_eq!(
        perft(&game, Game::MAX_DEPTH + 1, 100, || false),
        Err(PerftError::DepthLimit { max: 3 })
    );
    for limit in [0, STANDARD_VISIT_LIMIT + 1] {
        assert_eq!(
            perft(&game, 1, limit, || false),
            Err(PerftError::VisitLimit {
                max: STANDARD_VISIT_LIMIT
            })
        );
    }
    assert_eq!(
        divide(&game, 0, 10, || false),
        Err(PerftError::ZeroDivideDepth)
    );
    assert!(matches!(
        perft(&game, 2, 100, || true),
        Err(PerftError::Stopped { .. })
    ));
    let mut calls = 0;
    assert!(matches!(
        divide(&game, 2, 100, || {
            calls += 1;
            calls > 20
        }),
        Err(PerftError::Stopped { .. })
    ));
    assert_eq!(GameRecord::from_game(&game).unwrap(), before);
    assert_eq!(game.context(), context);
}
