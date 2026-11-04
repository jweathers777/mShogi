use super::*;
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
    // promotion, repeated boundary or bare-royal result intervenes at depth2.
    for profile in Profile::ALL {
        let game = diagram(profile);
        let before = GameRecord::from_game(&game).unwrap();
        let authority = game.context();
        assert_eq!(
            game.perft(0, 1).unwrap(),
            PerftReport {
                depth: 0,
                leaves: 1,
                visited: 1
            }
        );
        assert_eq!(
            game.perft(1, 5).unwrap(),
            PerftReport {
                depth: 1,
                leaves: 4,
                visited: 5
            }
        );
        assert_eq!(
            game.perft(2, 21).unwrap(),
            PerftReport {
                depth: 2,
                leaves: 16,
                visited: 21
            }
        );
        let divide = game.divide(2, 21).unwrap();
        assert_eq!(divide.total.leaves, 16);
        assert_eq!(divide.total.visited, 21);
        assert_eq!(divide.entries.len(), 4);
        assert!(divide.entries.iter().all(|entry| entry.leaves == 4));
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
        game.perft(2, 20),
        Err(PerftError::WorkLimit { .. })
    ));
    assert_eq!(
        game.perft(MAX_PERFT_DEPTH + 1, 100),
        Err(PerftError::Limits)
    );
    assert_eq!(game.perft(1, 0), Err(PerftError::Limits));
    assert_eq!(game.divide(0, 10), Err(PerftError::Limits));
    assert!(matches!(
        game.perft_with_stop(2, 100, &mut || true),
        Err(PerftError::Stopped { .. })
    ));
    let mut calls = 0;
    assert!(matches!(
        game.divide_with_stop(2, 100, &mut || {
            calls += 1;
            calls > 20
        }),
        Err(PerftError::Stopped { .. })
    ));
    assert_eq!(GameRecord::from_game(&game).unwrap(), before);
    assert_eq!(game.context(), context);
}
