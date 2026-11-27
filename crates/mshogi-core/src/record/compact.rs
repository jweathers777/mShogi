//! Compact game records for games with one event per accepted command.
use super::{RecordedCommand, reject_duplicates, serialize_json};
use crate::game::{
    history::{AppendGame, GameHistory, GameUndo, Request, Snapshot},
    incremental::IncrementalRecord,
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::value::RawValue;
use std::fmt::{self, Debug};

/// Refusal of a compact record or a retained suffix.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RecordError {
    /// An index, root or command exceeds its representation.
    Capacity,
    /// Invalid or duplicate JSON field.
    Json,
    /// Unsupported profile, edition or schema.
    Identity,
    /// Invalid game root.
    Root,
    /// Refused game command at this index.
    Command(usize),
    /// Internal game replay invariant at this command.
    Invariant(usize),
    /// Missing prefix or incomplete command boundary.
    Prefix,
    /// Declared facts differ from game admission.
    Derived,
    /// Caller stopped reconstruction.
    Stopped,
}
impl fmt::Display for RecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "game record refused: {self:?}")
    }
}
impl std::error::Error for RecordError {}

/// Rules-specific root and event data used by the shared codec.
pub trait Compact:
    Snapshot<
        Side: Copy + Debug + Eq + Serialize + DeserializeOwned,
        Event: Eq + Serialize + DeserializeOwned,
        Command: Clone + Serialize + DeserializeOwned,
    > + Eq
{
    type Root: Clone + Debug + Eq + Serialize + DeserializeOwned;
    type Conclusion: Copy + Debug + Eq + Serialize + DeserializeOwned;
    const SCHEMA: u32;
    const PROFILE: Option<&'static str>;
    fn edition(root: &Self::Root) -> &'static str;
    fn root(game: &GameHistory<Self>) -> Self::Root;
    fn install(
        root: Self::Root,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<GameHistory<Self>, RecordError>;
    fn command(event: &Self::Event) -> (Self::Side, &Self::Command);
    fn side(game: &GameHistory<Self>) -> Self::Side;
    fn conclusion(game: &GameHistory<Self>) -> Option<Self::Conclusion>;
    fn error(error: Self::Error, fallback: RecordError) -> RecordError;
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct Wire<R, E> {
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<String>,
    edition: String,
    schema_version: u32,
    pub(crate) root: R,
    events: E,
}
type ParsedWire<S> = Wire<
    <S as Compact>::Root,
    Vec<RecordedCommand<<S as Snapshot>::Side, <S as Snapshot>::Command>>,
>;
impl<R, E> Wire<R, E> {
    pub(crate) fn new<S: Compact<Root = R>>(root: R, events: E) -> Self {
        Self {
            profile: S::PROFILE.map(str::to_owned),
            edition: S::edition(&root).into(),
            schema_version: S::SCHEMA,
            root,
            events,
        }
    }
}

fn encode_root<S: Compact>(root: S::Root, events: &[S::Event]) -> Result<String, RecordError> {
    let commands: Vec<_> = events
        .iter()
        .map(|event| {
            let (actor, command) = S::command(event);
            RecordedCommand { actor, command }
        })
        .collect();
    serde_json::to_string(&Wire::new::<S>(root, commands)).map_err(|_| RecordError::Json)
}
pub(crate) fn encode<S: Compact>(game: &GameHistory<S>) -> Result<String, RecordError> {
    encode_root::<S>(S::root(game), game.events())
}
/// Canonical bytes and the validated history used to identify exact suffixes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompactRecord<S: Compact> {
    json: String,
    pub(crate) root: S::Root,
    ledger: Vec<S::Event>,
    board_plies: usize,
    side: S::Side,
    conclusion: Option<S::Conclusion>,
}
impl<S: Compact> CompactRecord<S> {
    /// Capture the complete committed game.
    pub fn from_game(game: &GameHistory<S>) -> Result<Self, RecordError> {
        let root = S::root(game);
        let json = encode_root::<S>(root.clone(), game.events())?;
        Ok(Self {
            json,
            root,
            ledger: game.events().to_vec(),
            board_plies: game.snapshot().board_ply(),
            side: S::side(game),
            conclusion: S::conclusion(game),
        })
    }
    /// Exact validated root and ledger prefix.
    pub fn has_prefix(&self, earlier: &Self) -> bool {
        self.root == earlier.root && self.ledger.starts_with(&earlier.ledger)
    }
    /// Copy the events after a transport-authenticated base.
    pub fn suffix(&self, start_event: usize) -> Result<CompactDelta<S>, RecordError> {
        let events = self
            .ledger
            .get(start_event..)
            .ok_or(RecordError::Prefix)?
            .to_vec();
        Ok(CompactDelta(DeltaFields {
            start_event,
            events,
            conclusion: self.conclusion,
        }))
    }
    /// Canonical saved bytes.
    pub fn to_json(&self) -> &str {
        &self.json
    }
    /// Borrow canonical saved bytes.
    pub fn as_json(&self) -> &str {
        &self.json
    }
    /// Committed event count.
    pub fn event_count(&self) -> usize {
        self.ledger.len()
    }
    /// Actual board actions since this root.
    pub fn known_board_actions(&self) -> usize {
        self.board_plies
    }
    /// Game turn at this prefix.
    pub fn side_to_move(&self) -> S::Side {
        self.side
    }
    /// Game conclusion at this prefix.
    pub fn conclusion(&self) -> Option<S::Conclusion> {
        self.conclusion
    }
    /// Strict admission and reconstruction of every command.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        Self::from_json_with_stop(text, &mut || false)
    }
    /// Admit a record with cooperative cancellation.
    pub fn from_json_with_stop(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self, RecordError> {
        Self::from_json_with_game(text, stop).map(|(r, _)| r)
    }
    /// Return the admitted record and its game from the same replay.
    pub fn from_json_with_game(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(Self, GameHistory<S>), RecordError> {
        let (game, _) = Self::replay(text, false, stop)?;
        let record = Self::from_game(&game)?;
        check(stop)?;
        require_equal(text, &record.json)?;
        check(stop)?;
        Ok((record, game))
    }
    /// Reconstruct a game with fresh authority.
    pub fn to_game(&self) -> Result<GameHistory<S>, RecordError> {
        self.to_game_with_stop(&mut || false)
    }
    /// Reconstruct an admitted game with cooperative cancellation.
    pub fn to_game_with_stop(
        &self,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<GameHistory<S>, RecordError> {
        Self::replay(&self.json, false, stop).map(|(g, _)| g)
    }
    /// Reconstruct LIFO reversal handles for the complete game.
    pub fn to_game_with_undo(&self) -> Result<(GameHistory<S>, Vec<GameUndo>), RecordError> {
        Self::replay(&self.json, true, &mut || false)
    }
    pub(crate) fn replay(
        text: &str,
        retain_undo: bool,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(GameHistory<S>, Vec<GameUndo>), RecordError> {
        check(stop)?;
        let wire: ParsedWire<S> = parse(text)?;
        check(stop)?;
        if wire.profile.as_deref() != S::PROFILE
            || wire.edition != S::edition(&wire.root)
            || wire.schema_version != S::SCHEMA
        {
            return Err(RecordError::Identity);
        }
        let mut game = S::install(wire.root, stop)?;
        check(stop)?;
        let mut undo = Vec::with_capacity(if retain_undo { wire.events.len() } else { 0 });
        for (index, event) in wire.events.into_iter().enumerate() {
            check(stop)?;
            let token = game
                .admit(
                    Request {
                        expected: game.context(),
                        actor: event.actor,
                        command: event.command,
                    },
                    stop,
                )
                .map_err(|e| S::error(e, RecordError::Command(index)))?;
            if retain_undo {
                undo.push(token);
            }
        }
        check(stop)?;
        Ok((game, undo))
    }
}
impl<S: Compact> Serialize for CompactRecord<S> {
    fn serialize<T: serde::Serializer>(&self, serializer: T) -> Result<T::Ok, T::Error> {
        serialize_json(&self.json, serializer)
    }
}
impl<'de, S: Compact> Deserialize<'de> for CompactRecord<S> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(d)?;
        Self::from_json(raw.get()).map_err(serde::de::Error::custom)
    }
}

fn check(stop: &mut impl FnMut() -> bool) -> Result<(), RecordError> {
    if stop() {
        Err(RecordError::Stopped)
    } else {
        Ok(())
    }
}
pub(crate) fn parse<T: DeserializeOwned>(text: &str) -> Result<T, RecordError> {
    reject_duplicates(text).map_err(|_| RecordError::Json)?;
    serde_json::from_str(text).map_err(|_| RecordError::Json)
}
pub(crate) fn require_equal(text: &str, expected: &str) -> Result<(), RecordError> {
    let value =
        |s: &str| serde_json::from_str::<serde_json::Value>(s).map_err(|_| RecordError::Json);
    if value(text)? == value(expected)? {
        Ok(())
    } else {
        Err(RecordError::Derived)
    }
}
pub(crate) fn command<T: DeserializeOwned + Serialize>(text: &str) -> Result<T, RecordError> {
    if text.len() > 4096 {
        return Err(RecordError::Capacity);
    }
    let command = parse(text)?;
    let canonical = serde_json::to_string(&command).map_err(|_| RecordError::Json)?;
    require_equal(text, &canonical).map_err(|_| RecordError::Json)?;
    Ok(command)
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct DeltaFields<E, C> {
    start_event: usize,
    events: Vec<E>,
    conclusion: Option<C>,
}
/// Strict game events for an already authenticated session base.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(transparent, bound = "")]
pub struct CompactDelta<S: Compact>(DeltaFields<S::Event, S::Conclusion>);
impl<S: Compact> CompactDelta<S> {
    /// Number of events at the retained base.
    pub fn start_event(&self) -> usize {
        self.0.start_event
    }
    /// Number of newly supplied events.
    pub fn event_count(&self) -> usize {
        self.0.events.len()
    }
    /// Decode syntax and require all redundant fields before game admission.
    pub fn from_json(text: &str) -> Result<Self, RecordError> {
        let delta = Self(parse(text)?);
        delta
            .0
            .start_event
            .checked_add(delta.event_count())
            .ok_or(RecordError::Capacity)?;
        require_equal(text, &delta.to_json()?)?;
        Ok(delta)
    }
    /// Encode the transport suffix.
    pub fn to_json(&self) -> Result<String, RecordError> {
        serde_json::to_string(self).map_err(|_| RecordError::Json)
    }
}
impl<'de, S: Compact> Deserialize<'de> for CompactDelta<S> {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(d)?;
        Self::from_json(raw.get()).map_err(serde::de::Error::custom)
    }
}
impl<S: Compact> IncrementalRecord for CompactRecord<S> {
    type Game = GameHistory<S>;
    type Delta = CompactDelta<S>;
    type Error = RecordError;
    fn reconstruct(&self) -> Result<Self::Game, RecordError> {
        self.to_game()
    }
    fn append(game: &mut Self::Game, delta: &Self::Delta) -> Result<(), RecordError> {
        if delta.start_event() != game.events().len() {
            return Err(RecordError::Prefix);
        }
        game.append_transaction(|game| {
            for event in &delta.0.events {
                let index = game.events().len();
                let (actor, command) = S::command(event);
                S::step(game, actor, command.clone(), &mut || false)
                    .map_err(|e| S::error(e, RecordError::Command(index)))?;
                if game.events().last() != Some(event) {
                    return Err(RecordError::Derived);
                }
            }
            if S::conclusion(game) != delta.0.conclusion {
                return Err(RecordError::Derived);
            }
            Ok(())
        })
        .map(drop)
    }
}

// All three histories expose the same event and snapshot contract.
macro_rules! compact_types {
    () => {
        type Conclusion = Conclusion;
        const SCHEMA: u32 = GAME_RECORD_SCHEMA_VERSION;
        fn command(event: &GameEvent) -> (Side, &GameCommand) {
            (event.actor, &event.command)
        }
        fn side(game: &Game) -> Side {
            game.position().side_to_move()
        }
        fn conclusion(game: &Game) -> Option<Conclusion> {
            game.conclusion()
        }
    };
}
pub(crate) use compact_types;
