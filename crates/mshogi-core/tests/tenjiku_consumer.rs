//! An external consumer of the public native handoff, without shared/product dispatch.
use mshogi_core::tenjiku::{
    Action, Base, EffectPhase, Game, GameCommand, GameError, GameRecord, GameRequest, Piece,
    Position, Profile, RecordError, RootMetadata, Side, Square,
};
use serde::Deserialize;
use serde_json::value::RawValue;

fn square(file: u8, rank: u8) -> Square {
    Square::new(file, rank).unwrap()
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConsumerPayload {
    profile: Profile,
    record: Box<RawValue>,
}

fn consume(text: &str) -> Result<Game, String> {
    let payload: ConsumerPayload = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let native = GameRecord::from_json(payload.record.get()).map_err(|e| e.to_string())?;
    if payload.profile != native.profile() {
        return Err("outer/native profile conflict".into());
    }
    native.to_game().map_err(|e| e.to_string())
}

fn game(profile: Profile) -> Game {
    let entries = [
        (16, 16, Side::Sente, Base::K),
        (15, 14, Side::Sente, Base::P),
        (8, 10, Side::Sente, Base::FiD),
        (1, 1, Side::Gote, Base::K),
        (2, 3, Side::Gote, Base::P),
        (9, 8, Side::Gote, Base::FiD),
        (8, 7, Side::Gote, Base::P),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (f, r, side, base))| {
        (
            square(f, r),
            Piece::new(id as u16, side, base, false).unwrap(),
        )
    })
    .collect::<Vec<_>>();
    Game::from_root(
        Position::from_diagram(
            profile,
            &entries,
            Side::Sente,
            RootMetadata::imported("public-native-consumer".into(), 120).unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}

#[test]
fn public_complete_effects_history_replay_inspection_and_fresh_authority() {
    for profile in Profile::ALL {
        let mut game = game(profile);
        let root = GameRecord::from_game(&game).unwrap();
        let root_context = game.context();
        let action = Action::single(square(8, 10), square(8, 8));
        assert!(game.legal_actions().contains(&action));
        let inspected = game.local_position().inspection(square(8, 10));
        assert!(inspected.iter().any(|c| c.action == action));
        assert_eq!(GameRecord::from_game(&game).unwrap(), root);
        let mut undo = game
            .submit(GameRequest {
                expected: root_context.clone(),
                actor: Side::Sente,
                command: GameCommand::Play {
                    action: action.clone(),
                },
            })
            .unwrap();
        let effects = game.events()[0].effects.as_ref().unwrap();
        assert_eq!(effects.action, action);
        assert!(effects.mover_after.is_none());
        assert_eq!(game.position().piece(square(9, 8)).unwrap().base, Base::FiD);
        if profile == Profile::AdamsModern {
            assert!(
                matches!(&effects.phases[0], EffectPhase::Burn {victims} if victims.len()==1 && victims[0].square==square(8,7))
            );
            assert!(matches!(effects.phases[1], EffectPhase::MoverDies { .. }));
            assert!(game.position().piece(square(8, 7)).is_none());
        } else {
            assert_eq!(effects.phases.len(), 1);
            assert!(matches!(effects.phases[0], EffectPhase::MoverDies { .. }));
            assert!(game.position().piece(square(8, 7)).is_some());
        }
        let accepted = GameRecord::from_game(&game).unwrap();
        let envelope = format!(
            "{{\"profile\":{},\"record\":{}}}",
            serde_json::to_string(&profile).unwrap(),
            accepted.as_json()
        );
        let mut restored = consume(&envelope).unwrap();
        assert!(game.context().same_semantics(&restored.context()));
        assert_ne!(game.context(), restored.context());
        assert_eq!(restored.profile(), profile);
        assert_eq!(
            restored.position().root().source(),
            Some("public-native-consumer")
        );
        assert_eq!(restored.position().accepted_plies(), 1);
        assert_eq!(restored.position().root().declared_completed_plies(), 120);
        let prefix = restored.record_prefix(0).unwrap();
        assert_eq!(prefix, root);
        assert_eq!(
            restored.event_prefix(0).unwrap().state(),
            root.to_game().unwrap().position().state()
        );
        let before_selection = restored.context();
        restored.select_prefix(0).unwrap();
        assert_eq!(GameRecord::from_game(&restored).unwrap(), accepted);
        assert_eq!(
            restored
                .submit(GameRequest {
                    expected: before_selection,
                    actor: Side::Gote,
                    command: GameCommand::Resign,
                })
                .unwrap_err(),
            GameError::StaleContext
        );
        let mut redo = game.undo(&mut undo).unwrap();
        assert_eq!(GameRecord::from_game(&game).unwrap(), root);
        assert_ne!(game.context(), root_context);
        game.redo(&mut redo).unwrap();
        assert_eq!(GameRecord::from_game(&game).unwrap(), accepted);
        assert_eq!(
            accepted.to_game_with_stop(&mut || true).unwrap_err(),
            RecordError::Stopped
        );
        assert_eq!(GameRecord::from_game(&game).unwrap(), accepted);
    }
}

#[test]
fn public_consumer_rejects_profile_conflicts_forged_effects_and_duplicate_fields() {
    for profile in Profile::ALL {
        let mut game = game(profile);
        let record = GameRecord::from_game(&game).unwrap();
        let other = if profile == Profile::AdamsModern {
            Profile::MullerCompleted
        } else {
            Profile::AdamsModern
        };
        let mismatch = format!(
            "{{\"profile\":{},\"record\":{}}}",
            serde_json::to_string(&other).unwrap(),
            record.as_json()
        );
        assert!(consume(&mismatch).is_err());
        let duplicate = record.as_json().replacen(
            "\"promoted\":false",
            "\"promoted\":false,\"promoted\":false",
            1,
        );
        assert!(GameRecord::from_json(&duplicate).is_err());
        let forged = record.as_json().replacen(
            "\"conclusion\":null",
            "\"conclusion\":{\"kind\":\"agreement\"}",
            1,
        );
        assert!(GameRecord::from_json(&forged).is_err());
        let profile_wire = serde_json::to_string(&profile).unwrap();
        let duplicate_outer = format!(
            "{{\"profile\":{profile_wire},\"profile\":{profile_wire},\"record\":{}}}",
            record.as_json()
        );
        assert!(consume(&duplicate_outer).is_err());
        assert_eq!(GameRecord::from_game(&game).unwrap(), record);
        game.submit(GameRequest {
            expected: game.context(),
            actor: Side::Sente,
            command: GameCommand::Play {
                action: Action::single(square(8, 10), square(8, 8)),
            },
        })
        .unwrap();
        let accepted = GameRecord::from_game(&game).unwrap();
        let mut forged: serde_json::Value = serde_json::from_str(accepted.as_json()).unwrap();
        if profile == Profile::AdamsModern {
            forged["events"][0]["effects"]["phases"][0]["victims"][0]["square"] =
                serde_json::json!([8, 6]);
        } else {
            forged["events"][0]["effects"]["phases"][0]["square"] = serde_json::json!([8, 9]);
        }
        assert_eq!(
            GameRecord::from_json(&forged.to_string()).unwrap_err(),
            RecordError::Derived
        );
        assert_eq!(GameRecord::from_game(&game).unwrap(), accepted);
    }
}
