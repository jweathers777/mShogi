//! Game root and commands for the shared compact record.
use super::*;
use crate::record::compact::{self, Compact, compact_types};
pub use compact::RecordError;
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Placed {
    square: Square,
    piece: Piece,
}
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WirePosition {
    root_kind: RootKind,
    source: Option<String>,
    earlier_history_unknown: bool,
    historical_reachability_proven: bool,
    declared_completed_plies: u64,
    accepted_plies: u64,
    declared_total_plies: u64,
    side_to_move: Side,
    board: Vec<Placed>,
    hands: [[u8; 6]; 2],
}
impl WirePosition {
    fn from_state(s: &StructuralState) -> Self {
        Self {
            root_kind: s.root.kind,
            source: s.root.source.clone(),
            earlier_history_unknown: s.root.earlier_history_unknown(),
            historical_reachability_proven: s.root.historical_reachability_proven(),
            declared_completed_plies: s.root.declared_completed_plies,
            accepted_plies: s.accepted_plies,
            declared_total_plies: s.root.declared_completed_plies + s.accepted_plies,
            side_to_move: s.side,
            board: Square::all()
                .filter_map(|square| s.board[square.index()].map(|piece| Placed { square, piece }))
                .collect(),
            hands: [s.hands[0].0, s.hands[1].0],
        }
    }
    fn root(&self) -> Result<Position, RecordError> {
        if self.accepted_plies != 0 {
            return Err(RecordError::Root);
        }
        let root = match self.root_kind {
            RootKind::Standard => Position::standard(),
            kind => {
                let meta = match (kind, &self.source) {
                    (RootKind::Diagram, None) => {
                        RootMetadata::diagram(self.declared_completed_plies)
                    }
                    (RootKind::Imported, Some(source)) => {
                        RootMetadata::imported(source.clone(), self.declared_completed_plies)
                            .map_err(|_| RecordError::Root)?
                    }
                    _ => return Err(RecordError::Root),
                };
                let hands = [0, 1].map(|i| {
                    Hand::from_counts(
                        &Face::HANDS
                            .into_iter()
                            .zip(self.hands[i])
                            .collect::<Vec<_>>(),
                    )
                    .map_err(|_| RecordError::Root)
                });
                let [s, g] = hands;
                Position::from_diagram(
                    &self
                        .board
                        .iter()
                        .map(|p| (p.square, p.piece))
                        .collect::<Vec<_>>(),
                    [s?, g?],
                    self.side_to_move,
                    meta,
                )
                .map_err(|_| RecordError::Root)?
            }
        };
        if *self != Self::from_state(root.state()) {
            return Err(RecordError::Root);
        }
        Ok(root)
    }
}
/// Record schema of the saved game.
pub const GAME_RECORD_SCHEMA_VERSION: u32 = 4;
/// Complete validated game record.
pub type GameRecord = compact::CompactRecord<HistorySnapshot>;
/// New game events following an exact retained base.
pub type GameDelta = compact::CompactDelta<HistorySnapshot>;
impl Compact for HistorySnapshot {
    compact_types!();
    type Root = WirePosition;
    const PROFILE: Option<&'static str> = Some(PROFILE_ID);
    fn edition(_: &Self::Root) -> &'static str {
        PROFILE_EDITION
    }
    fn root(game: &Game) -> Self::Root {
        WirePosition::from_state(game.root_snapshot().state())
    }
    fn install(root: Self::Root, _stop: &mut impl FnMut() -> bool) -> Result<Game, RecordError> {
        Game::from_root(root.root()?).map_err(|e| Self::error(e, RecordError::Root))
    }
    fn error(error: GameError, fallback: RecordError) -> RecordError {
        match error {
            GameError::Capacity(_) => RecordError::Capacity,
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
#[cfg(test)]
mod compact_bounds {
    use super::*;
    use crate::record::MAX_COMPACT_ROOT_BYTES;

    #[test]
    fn maximal_wire_fields_fit_compact_bounds() {
        // Deliberately overfill material to bound every structurally valid root.
        let mut wire = compact::Wire::new::<HistorySnapshot>(
            HistorySnapshot::root(&Game::standard()),
            Vec::<GameEvent>::new(),
        );
        wire.root.root_kind = RootKind::Imported;
        wire.root.source = Some("\0".repeat(1024));
        wire.root.declared_completed_plies = u64::MAX;
        wire.root.declared_total_plies = u64::MAX;
        wire.root.hands = [[u8::MAX; 6]; 2];
        for face in Face::ALL {
            wire.root.board = Square::all()
                .map(|square| Placed {
                    square,
                    piece: Piece {
                        side: Side::Sente,
                        face,
                    },
                })
                .collect();
            assert!(serde_json::to_vec(&wire).unwrap().len() <= MAX_COMPACT_ROOT_BYTES);
        }
    }
}
