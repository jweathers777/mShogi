//! Game root and commands for the shared compact record.
use super::game::GameRoot;
use super::*;
use crate::record::compact::{self, Compact, compact_types};
pub use compact::RecordError;
/// Record schema of the saved game.
pub const GAME_RECORD_SCHEMA_VERSION: u32 = 3;
/// Complete validated game record.
pub type GameRecord = compact::CompactRecord<HistorySnapshot>;
/// New game events following an exact retained base.
pub type GameDelta = compact::CompactDelta<HistorySnapshot>;
impl Compact for HistorySnapshot {
    compact_types!();
    type Root = GameRoot;
    const PROFILE: Option<&'static str> = Some(PROFILE_ID);
    fn edition(_: &Self::Root) -> &'static str {
        PROFILE_EDITION
    }
    fn root(game: &Game) -> Self::Root {
        game.game_root()
    }
    fn install(root: Self::Root, _stop: &mut impl FnMut() -> bool) -> Result<Game, RecordError> {
        Game::from_record_root(root).map_err(|e| Self::error(e, RecordError::Root))
    }
    fn error(error: GameError, fallback: RecordError) -> RecordError {
        match error {
            GameError::Capacity(_) => RecordError::Capacity,
            GameError::Stopped => RecordError::Stopped,
            _ => fallback,
        }
    }
}
impl GameCommand {
    /// Strict command syntax. The game owns admission.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        compact::command(text)
    }
}
impl Action {
    /// Strict complete action syntax.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        compact::command(text)
    }
}
#[cfg(test)]
mod compact_bounds {
    use super::*;
    use crate::record::MAX_COMPACT_ROOT_BYTES;

    #[test]
    fn maximal_wire_fields_fit_compact_bounds() {
        // Overfilled board/protection bound every admitted game root.
        let mut wire = compact::Wire::new::<HistorySnapshot>(
            HistorySnapshot::root(&Game::standard()),
            Vec::<GameEvent>::new(),
        );
        wire.root.state.root = RootMetadata::imported("\0".repeat(1024), u64::MAX).unwrap();
        wire.root.protection.owner = Some(Side::Sente);
        wire.root.protection.squares = Square::all().collect();
        for base in Base::ALL {
            wire.root.state.board = vec![
                Some(Piece {
                    side: Side::Sente,
                    base,
                    promoted: false
                });
                144
            ];
            assert!(serde_json::to_vec(&wire).unwrap().len() <= MAX_COMPACT_ROOT_BYTES);
        }
    }
}

#[cfg(test)]
mod incremental_tests {
    use super::*;
    use crate::game::incremental::Session;

    #[test]
    fn suffix_admission_matches_replay_and_refusal_is_atomic() {
        let mut game = Game::standard();
        let mut previous = GameRecord::from_game(&game).unwrap();
        let mut session = Session::<GameRecord>::new();
        assert!(session.advance(&previous.suffix(0).unwrap()).is_err());
        session.reset(&previous).unwrap();
        let mut seed = 213u64;
        for _ in 0..20 {
            for _ in 0..2 {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                let actions = game.legal_actions();
                let action = actions[seed as usize % actions.len()].clone();
                game.submit(GameRequest {
                    expected: game.context(),
                    actor: game.position().side_to_move(),
                    command: GameCommand::Play { action },
                })
                .unwrap();
            }
            let current = GameRecord::from_game(&game).unwrap();
            assert!(current.has_prefix(&previous));
            assert!(!previous.has_prefix(&current));
            let delta = current.suffix(previous.event_count()).unwrap();
            let text = delta.to_json().unwrap();
            let delta = GameDelta::from_json(&text).unwrap();
            let context = session.game().unwrap().context();
            for field in ["board_ply", "index"] {
                let mut forged = serde_json::to_value(&delta).unwrap();
                forged["events"][1][field] = serde_json::json!(999999);
                let bad: GameDelta = serde_json::from_value(forged).unwrap();
                assert!(session.advance(&bad).is_err());
                assert_eq!(session.game().unwrap().context(), context);
                assert_eq!(
                    GameRecord::from_game(session.game().unwrap()).unwrap(),
                    previous
                );
            }
            let game = session.advance(&delta).unwrap();
            assert_eq!(GameRecord::from_game(game).unwrap(), current);
            assert_eq!(
                GameRecord::from_game(&current.to_game().unwrap()).unwrap(),
                current
            );
            previous = current;
        }
        let mut value =
            serde_json::to_value(previous.suffix(previous.event_count()).unwrap()).unwrap();
        value.as_object_mut().unwrap().remove("conclusion");
        assert!(GameDelta::from_json(&value.to_string()).is_err());
        let text = previous
            .suffix(previous.event_count())
            .unwrap()
            .to_json()
            .unwrap();
        assert!(
            GameDelta::from_json(&text.replacen(
                "\"start_event\":",
                "\"start_event\":0,\"start_event\":",
                1
            ))
            .is_err()
        );
        assert!(
            !GameRecord::from_game(&Game::standard())
                .unwrap()
                .has_prefix(&previous)
        );
        session.clear();
        assert!(
            session
                .advance(&previous.suffix(previous.event_count()).unwrap())
                .is_err()
        );
    }
}
