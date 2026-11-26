//! Native canonical records. Native replay derives all event results.
use super::game::CommandEntry;
use super::{
    ClaimPolicy, Conclusion, Game, GameError, GameRequest, NoWinAssessment, PositionRoot, Side,
};
use crate::native::history::AppendGame;
use crate::native::incremental::IncrementalRecord;
use crate::record::RecordedCommand;
pub use crate::record::compact::RecordError;
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;
/// Native record schema; profile, governing edition and transport remain distinct.
pub const GAME_RECORD_SCHEMA_VERSION: u32 = 2;
/// Native game record validated against every authoritative command/event prefix.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameRecord {
    json: String,
    // Derived once by the authoritative constructor/replay, never trusted wire data.
    root: PositionRoot,
    policy: ClaimPolicy,
    commands: Vec<CommandEntry>,
    events: Vec<super::GameEvent>,
    known_board_actions: usize,
    side_to_move: Side,
    conclusion: Option<Conclusion>,
    assessment: NoWinAssessment,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct WireRecord {
    schema_version: u32,
    claim_policy: ClaimPolicy,
    root: Box<RawValue>,
    commands: Vec<RecordedCommand<Side, super::GameCommand>>,
}
impl WireRecord {
    fn from_game(game: &Game) -> Self {
        Self {
            schema_version: GAME_RECORD_SCHEMA_VERSION,
            claim_policy: game.policy,
            root: RawValue::from_string(game.root.to_json()).expect("native root JSON"),
            commands: game
                .commands
                .iter()
                .map(|entry| RecordedCommand {
                    actor: entry.actor,
                    command: entry.command,
                })
                .collect(),
        }
    }
}
impl GameRecord {
    /// Export the complete currently accepted native commands/events, including audits.
    pub fn from_game(game: &Game) -> Result<Self, RecordError> {
        Ok(Self {
            json: serde_json::to_string(&WireRecord::from_game(game))
                .map_err(|_| RecordError::Json)?,
            root: game.root.clone(),
            policy: game.policy,
            commands: game.commands.clone(),
            events: game.events().to_vec(),
            known_board_actions: game
                .events()
                .iter()
                .filter(|event| matches!(event.event, super::EventKind::BoardAction { .. }))
                .count(),
            side_to_move: game.position().side_to_move(),
            conclusion: game.conclusion(),
            assessment: game.no_win_assessment(),
        })
    }
    /// Exact complete event count, including same-ply warnings and claim audits.
    pub const fn event_count(&self) -> usize {
        self.events.len()
    }
    /// Known board actions; an imported declared counter never adds missing history.
    pub const fn known_board_actions(&self) -> usize {
        self.known_board_actions
    }
    /// Native current turn, not a winner or an execution capability.
    pub const fn side_to_move(&self) -> Side {
        self.side_to_move
    }
    /// Already replay-validated native conclusion; no replay here.
    pub const fn conclusion(&self) -> Option<Conclusion> {
        self.conclusion
    }
    /// Already replay-validated sound fast no-win assessment.
    pub fn no_win_assessment(&self) -> &NoWinAssessment {
        &self.assessment
    }
    /// Stable compact JSON, independent of runtime request identities.
    pub fn to_json(&self) -> &str {
        &self.json
    }
    /// Strict direct decoding precedes every normalization. Raw native roots retain
    /// duplicate fields for their own parser; no map is used ahead of that boundary.
    pub fn from_json(json: &str) -> Result<Self, RecordError> {
        Self::from_json_with_game(json).map(|(record, _)| record)
    }
    /// Parse and validate once, retaining the reconstructed game for the caller.
    pub fn from_json_with_game(json: &str) -> Result<(Self, Game), RecordError> {
        let game = Self::replay(json)?;
        Ok((Self::from_game(&game)?, game))
    }
    fn replay(json: &str) -> Result<Game, RecordError> {
        Self::replay_with_undo(json, false).map(|(game, _)| game)
    }
    fn replay_with_undo(
        json: &str,
        retain_undo: bool,
    ) -> Result<(Game, Vec<super::GameUndo>), RecordError> {
        let wire: WireRecord = crate::record::compact::parse(json)?;
        if wire.schema_version != GAME_RECORD_SCHEMA_VERSION {
            return Err(RecordError::Identity);
        }

        let root = PositionRoot::from_json(wire.root.get()).map_err(|_| RecordError::Root)?;
        let mut game = Game::from_root(root, wire.claim_policy).map_err(|_| RecordError::Root)?;
        let mut undo = Vec::with_capacity(if retain_undo { wire.commands.len() } else { 0 });
        for (i, entry) in wire.commands.iter().enumerate() {
            let submission = game
                .submit(GameRequest {
                    expected: game.context(),
                    actor: entry.actor,
                    command: entry.command,
                })
                .map_err(|error| match error {
                    GameError::Invariant => RecordError::Invariant(i),
                    _ => RecordError::Command(i),
                })?;
            if retain_undo {
                undo.push(submission.undo);
            }
        }
        // Strict typed and native parsers above have already rejected duplicates.
        // This final presence/equality check additionally rejects *missing* nullable
        // fields instead of letting Serde's Option default erase their absence.
        // It allows harmless JSON whitespace and object-key ordering differences.
        let actual: serde_json::Value =
            serde_json::from_str(json).map_err(|_| RecordError::Json)?;
        let expected =
            serde_json::to_value(WireRecord::from_game(&game)).map_err(|_| RecordError::Json)?;
        if actual != expected {
            return Err(RecordError::Derived);
        }
        Ok((game, undo))
    }
    /// Authoritative bounded replay with fresh runtime identity and LIFO command
    /// undo handles. Consumers must not reconstruct commands from derived events.
    /// Root automatic conclusions have no command handle.
    pub fn to_game_with_undo(&self) -> Result<(Game, Vec<super::GameUndo>), RecordError> {
        Self::replay_with_undo(&self.json, true)
    }
    /// Reconstruct with fresh runtime identity; observations are never pending work.
    pub fn to_game(&self) -> Result<Game, RecordError> {
        Self::replay(&self.json)
    }
}

impl Serialize for GameRecord {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        crate::record::serialize_json(&self.json, serializer)
    }
}

/// All commands and derived events after a complete Xiangqi command boundary.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GameDelta {
    start_event: usize,
    commands: Vec<CommandEntry>,
    events: Vec<super::GameEvent>,
}
impl<'de> Deserialize<'de> for GameDelta {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Fields {
            start_event: usize,
            commands: Vec<CommandEntry>,
            events: Vec<super::GameEvent>,
        }
        let raw = Box::<RawValue>::deserialize(d)?;
        let fields: Fields =
            crate::record::compact::parse(raw.get()).map_err(serde::de::Error::custom)?;
        let delta = Self {
            start_event: fields.start_event,
            commands: fields.commands,
            events: fields.events,
        };
        let json = serde_json::to_string(&delta).map_err(serde::de::Error::custom)?;
        crate::record::compact::require_equal(raw.get(), &json)
            .map_err(serde::de::Error::custom)?;
        Ok(delta)
    }
}
impl GameRecord {
    /// Compare the exact root, claim policy and committed command history.
    pub fn has_prefix(&self, earlier: &Self) -> bool {
        self.root == earlier.root
            && self.policy == earlier.policy
            && self.commands.starts_with(&earlier.commands)
    }
    /// Copy only complete command groups. Automatic root events cannot be divided.
    pub fn suffix(&self, start_event: usize) -> Result<GameDelta, RecordError> {
        let first = if start_event == self.events.len() {
            self.commands.len()
        } else if start_event == 0 && !self.commands.is_empty() {
            0
        } else {
            self.commands
                .binary_search_by_key(&start_event, |c| c.event_end)
                .map_err(|_| RecordError::Prefix)?
                + 1
        };
        Ok(GameDelta {
            start_event,
            commands: self.commands[first..].to_vec(),
            events: self.events[start_event..].to_vec(),
        })
    }
}
impl IncrementalRecord for GameRecord {
    type Game = Game;
    type Delta = GameDelta;
    type Error = RecordError;
    fn reconstruct(&self) -> Result<Game, RecordError> {
        self.to_game()
    }
    fn append(game: &mut Game, delta: &GameDelta) -> Result<(), RecordError> {
        if delta.start_event != game.events().len() {
            return Err(RecordError::Prefix);
        }
        game.append_transaction(|game| {
            for entry in &delta.commands {
                let index = game.commands.len();
                game.apply_request(GameRequest {
                    expected: game.context(),
                    actor: entry.actor,
                    command: entry.command,
                })
                .map_err(|e| {
                    if e == GameError::Invariant {
                        RecordError::Invariant(index)
                    } else {
                        RecordError::Command(index)
                    }
                })?;
                if game.commands.last() != Some(entry) {
                    return Err(RecordError::Derived);
                }
            }
            if game.events()[delta.start_event..] != delta.events {
                return Err(RecordError::Derived);
            }
            Ok(())
        })
        .map(drop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::native::incremental::Session;
    use crate::xiangqi::GameCommand;

    #[test]
    fn suffix_keeps_multi_event_commands_atomic() {
        let mut game = Game::standard(ClaimPolicy::Explicit).unwrap();
        let base = GameRecord::from_game(&game).unwrap();
        for command in [GameCommand::ClaimNaturalCount, GameCommand::Resign] {
            game.submit(GameRequest {
                expected: game.context(),
                actor: Side::Red,
                command,
            })
            .unwrap();
        }
        let full = GameRecord::from_game(&game).unwrap();
        assert!(full.has_prefix(&base));
        assert_eq!(full.event_count(), 3);
        assert_eq!(full.suffix(2), Err(RecordError::Prefix));
        let delta = full.suffix(0).unwrap();
        let mut session = Session::<GameRecord>::new();
        session.reset(&base).unwrap();
        let context = session.game().unwrap().context();
        let mut forged = delta.clone();
        forged.events.pop();
        assert!(session.advance(&forged).is_err());
        assert_eq!(session.game().unwrap().context(), context);
        assert_eq!(
            GameRecord::from_game(session.game().unwrap()).unwrap(),
            base
        );
        assert_eq!(
            GameRecord::from_game(session.advance(&delta).unwrap()).unwrap(),
            full
        );
        assert!(full.has_prefix(&full));
        assert!(!base.has_prefix(&full));
        let different_policy = Game::standard(ClaimPolicy::AutoValidClaims).unwrap();
        assert!(!full.has_prefix(&GameRecord::from_game(&different_policy).unwrap()));
    }
}
