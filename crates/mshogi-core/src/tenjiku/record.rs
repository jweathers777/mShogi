//! Game root and commands for the shared compact record.
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
    type Root = StructuralState;
    const PROFILE: Option<&'static str> = None;
    fn edition(root: &Self::Root) -> &'static str {
        root.profile().edition()
    }
    fn root(game: &Game) -> Self::Root {
        game.game_root().clone()
    }
    fn install(root: Self::Root, stop: &mut impl FnMut() -> bool) -> Result<Game, RecordError> {
        let position = Position::install(root).map_err(|_| RecordError::Root)?;
        Game::from_root_with_stop(position, stop).map_err(|e| Self::error(e, RecordError::Root))
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
impl GameRecord {
    /// Immutable profile selected at the root.
    pub fn profile(&self) -> Profile {
        self.root.profile()
    }
}
#[cfg(test)]
#[path = "record_tests.rs"]
mod tests;

#[cfg(test)]
mod compact_bounds {
    use super::*;
    use crate::record::MAX_COMPACT_ROOT_BYTES;
    #[test]
    fn maximal_wire_fields_fit_compact_bounds() {
        for profile in Profile::ALL {
            let game = Game::standard(profile);
            let mut root = game.game_root().clone();
            root.root = RootMetadata::imported("\0".repeat(1024), u64::MAX).unwrap();
            // Deliberately overfill material/identities to bound every valid root.
            for base in Base::ALL {
                root.board = vec![
                    Some(Piece {
                        id: u16::MAX,
                        side: Side::Sente,
                        base,
                        promoted: false,
                        right: PromotionRight::FirstDeclinedNextMove
                    });
                    256
                ];
                let wire =
                    compact::Wire::new::<HistorySnapshot>(root.clone(), Vec::<GameEvent>::new());
                assert!(serde_json::to_vec(&wire).unwrap().len() <= MAX_COMPACT_ROOT_BYTES);
            }
        }
    }
}
