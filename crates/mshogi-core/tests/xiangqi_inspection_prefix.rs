//! MSH-98 native read-only inspection and atomic historical replay boundaries.
use mshogi_core::xiangqi::{
    ClaimPolicy, Conclusion, DrawReason, Game, GameCommand, GameError, GameRecord, GameRequest,
    Position, PositionRoot, Side,
};
use serde_json::Value;
use std::collections::BTreeSet;

fn names(position: &Position, side: Side) -> BTreeSet<String> {
    position
        .inspection_actions(side)
        .unwrap()
        .into_iter()
        .map(|action| action.to_string())
        .collect()
}
fn record(game: &Game) -> String {
    GameRecord::from_game(game).unwrap().to_json().to_owned()
}
fn submit(game: &mut Game, actor: Side, command: GameCommand) {
    game.submit(GameRequest {
        expected: game.context(),
        actor,
        command,
    })
    .unwrap();
}

#[test]
fn inspection_reuses_frozen_independent_board_facts_for_both_native_owners() {
    let fixtures: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/xiangqi/board.json")).unwrap();
    for case in fixtures["fixtures"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        let position = Position::from_fen(case["fen"].as_str().unwrap()).unwrap();
        let before = position.clone();
        if case["domain"] == "structural_analysis" {
            for side in [Side::Red, Side::Black] {
                assert!(position.inspection_actions(side).is_err(), "{id}");
            }
            continue;
        }
        let choices = names(&position, position.side_to_move());
        if let Some(expected) = case["legal_moves"].as_array() {
            let expected: BTreeSet<_> = expected
                .iter()
                .map(|name| name.as_str().unwrap().to_owned())
                .collect();
            assert_eq!(choices, expected, "{id}: independent complete set");
        }
        if let Some(origins) = case["moves_from"].as_object() {
            for (origin, expected) in origins {
                let expected: BTreeSet<_> = expected
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|name| name.as_str().unwrap().to_owned())
                    .collect();
                let selected: BTreeSet<_> = choices
                    .iter()
                    .filter(|name| name.starts_with(origin))
                    .cloned()
                    .collect();
                assert_eq!(selected, expected, "{id}: independent origin {origin}");
            }
        }
        for field in ["must_include", "must_exclude"] {
            if let Some(expected) = case[field].as_array() {
                for action in expected {
                    assert_eq!(
                        choices.contains(action.as_str().unwrap()),
                        field == "must_include",
                        "{id}: {action}"
                    );
                }
            }
        }
        // Inspection may be queried repeatedly for both participants without
        // rewriting the actual side, counters, board or repetition identity.
        names(&position, position.side_to_move().opponent());
        assert_eq!(position, before, "{id}: inspection changed native position");
    }
}

#[test]
fn checked_actual_player_does_not_prevent_read_only_opponent_inspection() {
    // XQ-B35: the e8 rook checks Red. A hypothetical Black action cannot
    // expose its e9 General to the Red e0 General. Thus the rook stays on e,
    // and Black's General may move only to d9/f9. General capture is excluded.
    let position = Position::from_fen("4k4/4r4/9/9/9/9/9/9/9/4K4 w - - 0 1").unwrap();
    assert!(position.is_in_check(Side::Red));
    let before = position.clone();
    let expected: BTreeSet<_> = [
        "e8e1", "e8e2", "e8e3", "e8e4", "e8e5", "e8e6", "e8e7", "e9d9", "e9f9",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    assert_eq!(names(&position, Side::Black), expected);
    assert_eq!(position, before);
    let mut actual = position;
    assert!(actual.apply("e8e7".parse().unwrap()).is_err());
    assert_eq!(
        actual, before,
        "inspection cannot admit an opponent command"
    );
}

#[test]
fn committed_history_preserves_prefix_context_without_future_warnings_or_commands() {
    // Frozen F022-CC: the third root occurrence after eight board actions
    // emits Red's perpetual-check warning. The warning is part of action8,
    // never an independently resumable partially committed action.
    let root = PositionRoot::from_fen("4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 47 23").unwrap();
    let mut game = Game::from_root(root.clone(), ClaimPolicy::Explicit).unwrap();
    let mut committed = vec![(0, record(&game))];
    // A rejected claim is audited with two same-ply events, still one command.
    submit(&mut game, Side::Black, GameCommand::ClaimNaturalCount);
    assert_eq!(game.events().len(), 2);
    committed.push((game.events().len(), record(&game)));
    for (index, action) in ["e9f9", "f8e8", "f9e9", "e8f8"]
        .into_iter()
        .cycle()
        .take(8)
        .enumerate()
    {
        let actor = game.position().side_to_move();
        submit(
            &mut game,
            actor,
            GameCommand::Play {
                action: action.parse().unwrap(),
            },
        );
        if index < 7 {
            assert!(game.warnings().is_empty());
        }
        committed.push((game.events().len(), record(&game)));
    }
    assert_eq!(game.warnings().len(), 1);
    assert_eq!(game.events().len(), 11, "2 claim + 8 actions + warning");
    let before_resign_ply = game.event_prefix(game.events().len()).unwrap().board_ply;
    // Black is not the mover; terminal command semantics remain native.
    submit(&mut game, Side::Black, GameCommand::Resign);
    assert_eq!(game.events().len(), 13);
    assert_eq!(game.event_prefix(13).unwrap().board_ply, before_resign_ply);
    committed.push((13, record(&game)));
    let source_context = game.context();
    let source_record = record(&game);
    for count in 0..=game.events().len() + 1 {
        match committed.iter().find(|(end, _)| *end == count) {
            Some((_, expected)) => {
                let mut prefix = game.committed_prefix(count).unwrap();
                assert_eq!(prefix.root(), &root);
                assert_eq!(prefix.claim_policy(), ClaimPolicy::Explicit);
                assert_eq!(record(&prefix), *expected, "native complete prefix {count}");
                assert_ne!(prefix.context(), source_context, "fresh runtime identity");
                assert_eq!(prefix.events(), &game.events()[..count]);
                if count < 11 {
                    assert!(prefix.warnings().is_empty());
                }
                let stale = prefix.submit(GameRequest {
                    expected: source_context.clone(),
                    actor: Side::Red,
                    command: GameCommand::Resign,
                });
                assert_eq!(stale.unwrap_err(), GameError::StaleContext);
            }
            None => assert_eq!(
                game.committed_prefix(count).unwrap_err(),
                GameError::NotCommandBoundary
            ),
        }
    }
    // All partial event prefixes remain available for honest read-only viewing.
    assert_eq!(game.event_prefix(10).unwrap().board_ply, 8);
    assert!(game.event_prefix(10).unwrap().warnings.is_empty());
    assert_eq!(game.event_prefix(12).unwrap().conclusion, None);
    assert_eq!(game.context(), source_context);
    assert_eq!(record(&game), source_record);
}

#[test]
fn root_automatic_conclusion_is_not_downgraded_to_a_resumable_ongoing_root() {
    let game = Game::from_root(
        PositionRoot::from_fen("5k3/9/9/9/9/9/9/9/9/3K5 w - - 9 7").unwrap(),
        ClaimPolicy::AutoValidClaims,
    )
    .unwrap();
    assert_eq!(game.events().len(), 1);
    assert_eq!(
        game.conclusion(),
        Some(Conclusion::Draw {
            reason: DrawReason::NeitherCanWin
        })
    );
    assert_eq!(game.event_prefix(0).unwrap().conclusion, None);
    assert_eq!(
        game.committed_prefix(0).unwrap_err(),
        GameError::NotCommandBoundary
    );
    let prefix = game.committed_prefix(1).unwrap();
    assert_eq!(record(&prefix), record(&game));
    assert_eq!(prefix.claim_policy(), ClaimPolicy::AutoValidClaims);
    assert_ne!(prefix.context(), game.context());
}
