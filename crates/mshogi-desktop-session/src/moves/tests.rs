use super::*;
fn selected(projection: MovesProjection, index: usize) -> MovesProjection {
    Cache::default().get((1, 1), index, || projection)
}
fn play_shogi(game: &mut s::Game, text: &str) {
    game.submit(s::GameRequest {
        expected: game.context(),
        actor: game.position().side_to_move(),
        command: s::GameCommand::Play(text.parse().unwrap()),
    })
    .unwrap();
}
fn play_xiangqi(game: &mut x::Game, text: &str) {
    game.submit(x::GameRequest {
        expected: game.context(),
        actor: game.position().side_to_move(),
        command: x::GameCommand::Play {
            action: text.parse().unwrap(),
        },
    })
    .unwrap();
}
#[test]
fn native_chess_pairs_root_counter_geometry_and_nonmove_boundaries() {
    let mut game = Game::from_fen(
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 42",
        ClaimPolicy::ExplicitOnly,
    )
    .unwrap();
    game.apply("e7e5".parse().unwrap()).unwrap();
    game.apply("e2e4".parse().unwrap()).unwrap();
    let state = DesktopSession::from_record(&game.record(), Color::White).unwrap();
    let p = chess_projection(&state);
    assert_eq!(
        p.entries
            .iter()
            .map(|m| (m.number, m.column, m.event_index, m.notation.as_str()))
            .collect::<Vec<_>>(),
        vec![(42, 1, 1, "e5"), (43, 0, 2, "e4")]
    );
    assert!(p.root.numbering.contains("42") && p.root.numbering.contains("unknown"));
    assert!(selected(p.clone(), 0).last_action.is_none());
    assert_eq!(
        selected(p.clone(), 1).last_action.unwrap().steps,
        vec!["e5"]
    );
    // Runtime match Start is a non-move; it retains the preceding actual cue.
    assert_eq!(selected(p, 3).last_action.unwrap().event_index, 2);
    let mut game = Game::from_fen(
        "r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1",
        ClaimPolicy::ExplicitOnly,
    )
    .unwrap();
    game.apply("e1g1".parse().unwrap()).unwrap();
    let state = DesktopSession::from_record(&game.record(), Color::White).unwrap();
    let cue = selected(chess_projection(&state), 1).last_action.unwrap();
    assert_eq!(cue.action, "e1g1");
    assert_eq!(cue.effects.len(), 1);
    assert_eq!((&*cue.effects[0].from, &*cue.effects[0].to), ("h1", "f1"));
}
#[test]
fn native_shogi_declared_counter_drop_and_terminal_are_not_moves() {
    let root = s::GameRoot::import(
        s::Position::from_sfen("8k/9/9/9/9/9/9/9/K8 w p 37").unwrap(),
        s::RootCategory::ReducedMaterialDiagram,
        s::CounterSource::Supplied,
    )
    .unwrap();
    let mut game = s::Game::from_root(root);
    play_shogi(&mut game, "P*5e");
    game.submit(s::GameRequest {
        expected: game.context(),
        actor: s::Side::Sente,
        command: s::GameCommand::Resign,
    })
    .unwrap();
    let p = shogi_projection(&game);
    assert!(p.root.numbering.contains("37") && p.root.numbering.contains("unknown"));
    assert_eq!(p.entries.len(), 1);
    assert_eq!(
        (
            p.entries[0].number,
            p.entries[0].column,
            p.entries[0].event_index
        ),
        (1, 1, 1)
    );
    let cue = selected(p, 2).last_action.unwrap();
    assert!(matches!(cue.origin, ChoiceOrigin::Hand { .. }));
    assert_eq!(cue.steps, vec!["5e"]);
}
#[test]
fn xiangqi_action_warning_exact_prefix_and_command_ranges() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../tests/fixtures/xiangqi/history-cycles.json"
    ))
    .unwrap();
    let f = fixtures["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == "F022-CC")
        .unwrap();
    let mut game = x::Game::from_root(
        x::PositionRoot::from_fen(f["root_fen"].as_str().unwrap()).unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap();
    for action in f["moves"].as_array().unwrap() {
        play_xiangqi(&mut game, action.as_str().unwrap());
    }
    let before = x::GameRecord::from_game(&game).unwrap();
    let p = xiangqi_projection(&game);
    assert_eq!(p.entries.len(), 8);
    let last = &p.entries[7];
    assert_eq!(
        (last.event_index, last.command_start, last.command_end),
        (8, 7, 9)
    );
    assert!(game.event_prefix(8).unwrap().warnings.is_empty());
    assert_eq!(game.event_prefix(9).unwrap().warnings.len(), 1);
    assert!(game.committed_prefix(8).is_err());
    assert!(game.committed_prefix(9).is_ok());
    let a = selected(p.clone(), 8).last_action.unwrap();
    let b = selected(p, 9).last_action.unwrap();
    assert_eq!(a.action, b.action);
    assert_eq!((a.event_index, b.event_index), (8, 8));
    assert_eq!(
        before,
        x::GameRecord::from_game(&game).unwrap(),
        "projection is read-only"
    );
}
#[test]
fn polling_cache_selected_prefix_and_monotonic_undo_identity() {
    let game = Game::new(ClaimPolicy::ExplicitOnly);
    let state = DesktopSession::from_record(&game.record(), Color::White).unwrap();
    let p = chess_projection(&state);
    let mut cache = Cache::default();
    let calls = std::cell::Cell::new(0);
    for selected in [0, 1, 0, 1] {
        cache.get((1, 2), selected, || {
            calls.set(calls.get() + 1);
            p.clone()
        });
    }
    assert_eq!(calls.get(), 1);
    cache.get((1, 3), 0, || {
        calls.set(calls.get() + 1);
        p.clone()
    });
    cache.get((2, 3), 0, || {
        calls.set(calls.get() + 1);
        p.clone()
    });
    assert_eq!(calls.get(), 3);
}
