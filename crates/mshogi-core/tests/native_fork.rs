//! Analysis forks retain admitted history while separating command authority.
macro_rules! check {
    ($module:ident, $root:expr) => {{
        use mshogi_core::$module::*;
        let mut original = $root;
        let mut last = None;
        for _ in 0..8 {
            if original.conclusion().is_some() {
                break;
            }
            let action = original.legal_actions()[0].clone();
            last = Some(
                original
                    .submit(GameRequest {
                        expected: original.context(),
                        actor: original.position().side_to_move(),
                        command: GameCommand::Play { action },
                    })
                    .unwrap(),
            );
        }
        let record = GameRecord::from_game(&original).unwrap();
        let mut replay = record.to_game().unwrap();
        let mut fork = original.fork().unwrap();
        assert_eq!(GameRecord::from_game(&fork).unwrap(), record);
        assert_ne!(fork.context(), original.context());
        assert_eq!(fork.replay_work(), original.replay_work());
        let mut token = last.unwrap();
        assert!(fork.undo(&mut token).is_err());
        assert_eq!(GameRecord::from_game(&fork).unwrap(), record);
        for _ in 0..8 {
            assert_eq!(fork.conclusion(), replay.conclusion());
            if fork.conclusion().is_some() {
                break;
            }
            assert_eq!(fork.legal_actions(), replay.legal_actions());
            let action = fork.legal_actions()[0].clone();
            for game in [&mut fork, &mut replay] {
                let mut undo = game
                    .submit(GameRequest {
                        expected: game.context(),
                        actor: game.position().side_to_move(),
                        command: GameCommand::Play {
                            action: action.clone(),
                        },
                    })
                    .unwrap();
                let mut redo = game.undo(&mut undo).unwrap();
                game.redo(&mut redo).unwrap();
            }
            assert_eq!(
                GameRecord::from_game(&fork).unwrap(),
                GameRecord::from_game(&replay).unwrap()
            );
        }
        assert_eq!(GameRecord::from_game(&original).unwrap(), record);
        original.undo(&mut token).unwrap();
    }};
}
#[test]
fn chu_fork_matches_replay_and_keeps_tokens_apart() {
    check!(chu, Game::standard());
}
#[test]
fn tenjiku_forks_match_replay_and_keep_tokens_apart() {
    for profile in mshogi_core::tenjiku::Profile::ALL {
        check!(tenjiku, Game::standard(profile));
    }
}
