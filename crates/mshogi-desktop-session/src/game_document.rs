//! The saved game document of every game on the generic session (MSH-217,
//! reduced in MSH-228). It holds the game, not an audit trail: the current
//! native record (start position and every command), the human's side and
//! the settings. Opening a document parses the record, which replays and
//! checks every command in the rules core.
use crate::native_game::{
    DocumentSettings, NativeFailure, NativeGame, NativeResource, RecordFailure,
};
use crate::{document, *};
use mshogi_core::native::RulesProfile;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::marker::PhantomData;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
/// Saved settings of a game without game-specific fields.
pub struct GameSettings<S> {
    /// Persisted ownership of the two native participants.
    #[serde(default)]
    pub play_mode: crate::PlayMode,
    /// Native side displayed at the bottom.
    pub orientation: S,
}
impl<S> DocumentSettings for GameSettings<S>
where
    S: Clone + Eq + fmt::Debug + Serialize + serde::de::DeserializeOwned + Send + Sync + 'static,
{
    type Side = S;
    fn new(orientation: S, play_mode: crate::PlayMode) -> Self {
        Self {
            play_mode,
            orientation,
        }
    }
    fn play_mode(&self) -> crate::PlayMode {
        self.play_mode
    }
    fn set_play_mode(&mut self, play_mode: crate::PlayMode) {
        self.play_mode = play_mode;
    }
}
/// The settings of game `G`.
pub type SettingsOf<G> = <G as NativeGame>::Settings;

#[derive(Serialize)]
#[serde(bound(serialize = ""))]
struct Payload<G: NativeGame> {
    record: G::Record,
    human: G::Side,
    settings: SettingsOf<G>,
}
impl<G: NativeGame> Clone for Payload<G> {
    fn clone(&self) -> Self {
        Self {
            record: self.record.clone(),
            human: self.human,
            settings: self.settings.clone(),
        }
    }
}

#[derive(Clone, Debug)]
/// A refused document operation, separate from a game outcome.
pub struct GameDocumentError {
    /// Bounded technical diagnostic.
    pub diagnostic: String,
    /// Capacity facts when a limit refused the operation.
    pub resource: Option<ResourceFailure>,
    /// The caller stopped the operation.
    pub stopped: bool,
}
impl fmt::Display for GameDocumentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.diagnostic)
    }
}
impl std::error::Error for GameDocumentError {}
impl From<OperationFailure> for GameDocumentError {
    fn from(e: OperationFailure) -> Self {
        Self {
            diagnostic: e.diagnostic,
            resource: e.resource,
            stopped: false,
        }
    }
}
impl From<GameDocumentError> for OperationFailure {
    fn from(e: GameDocumentError) -> Self {
        Self {
            diagnostic: e.diagnostic,
            resource: e.resource,
        }
    }
}

/// Error helpers of game `G`: diagnostics name the game and resources its scope.
pub(crate) struct Errors<G>(PhantomData<G>);
impl<G: NativeGame> Errors<G> {
    pub(crate) fn invalid(e: impl fmt::Display) -> GameDocumentError {
        GameDocumentError {
            diagnostic: document::diagnostic(format!("Invalid {} document: {e}", G::NAME)),
            resource: None,
            stopped: false,
        }
    }
    fn stopped() -> GameDocumentError {
        GameDocumentError {
            diagnostic: format!("{} document operation stopped before acceptance.", G::NAME),
            resource: None,
            stopped: true,
        }
    }
    fn check(stopped: &mut impl FnMut() -> bool) -> Result<(), GameDocumentError> {
        if stopped() {
            Err(Self::stopped())
        } else {
            Ok(())
        }
    }
    fn capacity(category: ResourceCategory, diagnostic: String) -> GameDocumentError {
        let mut fact = ResourceFact::unknown().with_scope(G::SCOPE);
        fact.category = category;
        OperationFailure::resource(ResourceFailure::new(fact, diagnostic)).into()
    }
    fn file_size(bytes: usize) -> GameDocumentError {
        OperationFailure::resource(ResourceFailure::new(
            ResourceFact::measured(
                ResourceCategory::EncodedDocumentBytes,
                bytes as u64,
                MAX_ARCHIVE_BYTES as u64,
            )
            .with_scope(G::SCOPE)
            .with_unit(ResourceUnit::Bytes),
            "Application limit reached: the document is larger than 16 MiB.",
        ))
        .into()
    }
    fn record(error: RecordFailure) -> GameDocumentError {
        match error {
            RecordFailure::Stopped => Self::stopped(),
            RecordFailure::Capacity(diagnostic) => {
                Self::capacity(ResourceCategory::Unknown, diagnostic)
            }
            RecordFailure::Invalid(diagnostic) => Self::invalid(diagnostic),
        }
    }
    pub(crate) fn native(error: NativeFailure) -> GameDocumentError {
        let (resource, diagnostic) = match error {
            NativeFailure::Stopped => return Self::stopped(),
            NativeFailure::Invalid(diagnostic) => return Self::invalid(diagnostic),
            NativeFailure::Capacity(resource, diagnostic) => (resource, diagnostic),
        };
        let category = match resource {
            NativeResource::EncodedBytes => ResourceCategory::CanonicalRecordBytes,
            NativeResource::Counter => ResourceCategory::Unknown,
        };
        Self::capacity(category, diagnostic)
    }
}

/// An accepted, validated game document.
pub struct ValidatedGameDocument<G: NativeGame> {
    payload: Payload<G>,
    bytes: Vec<u8>,
}
impl<G: NativeGame> Clone for ValidatedGameDocument<G> {
    fn clone(&self) -> Self {
        Self {
            payload: self.payload.clone(),
            bytes: self.bytes.clone(),
        }
    }
}
impl<G: NativeGame> fmt::Debug for ValidatedGameDocument<G> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ValidatedGameDocument")
            .field("game", &G::NAME)
            .field("bytes", &self.bytes.len())
            .finish_non_exhaustive()
    }
}
impl<G: NativeGame> ValidatedGameDocument<G> {
    /// A document for a native record; the human plays `human`.
    pub fn new(record: G::Record, human: G::Side) -> Result<Self, GameDocumentError> {
        Self::new_with_mode(record, human, crate::PlayMode::default(), &mut || false)
    }
    pub(crate) fn new_with_mode(
        record: G::Record,
        human: G::Side,
        play_mode: crate::PlayMode,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, GameDocumentError> {
        Errors::<G>::check(stopped)?;
        Self::accept(Payload {
            record,
            human,
            settings: G::Settings::new(human, play_mode),
        })
    }
    /// Read a saved document: strict fields, then the native record parse,
    /// which replays and checks every command.
    pub fn decode(
        bytes: &[u8],
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, GameDocumentError> {
        Self::decode_with_game(bytes, stopped).map(|(document, _)| document)
    }
    /// Keep the native game already reconstructed while checking the file.
    pub(crate) fn decode_with_game(
        bytes: &[u8],
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<(Self, G::Game), GameDocumentError> {
        type E<G> = Errors<G>;
        E::<G>::check(stopped)?;
        if bytes.len() > MAX_ARCHIVE_BYTES {
            return Err(E::<G>::file_size(bytes.len()));
        }
        document::reject_duplicates(bytes).map_err(E::<G>::invalid)?;
        let e = document::DocumentEnvelope::parse(bytes).map_err(E::<G>::invalid)?;
        if e.game_kind != G::KIND {
            return Err(E::<G>::invalid("wrong desktop game"));
        }
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields, bound(deserialize = ""))]
        struct Wire<G: NativeGame> {
            record: Box<serde_json::value::RawValue>,
            human: G::Side,
            settings: SettingsOf<G>,
        }
        let w: Wire<G> = serde_json::from_str(e.payload.get()).map_err(E::<G>::invalid)?;
        let result = G::parse_record(w.record.get(), stopped);
        E::<G>::check(stopped)?;
        let (record, game) = result.map_err(E::<G>::record)?;
        let document = Self::accept(Payload {
            record,
            human: w.human,
            settings: w.settings,
        })?;
        Ok((document, game))
    }
    fn accept(payload: Payload<G>) -> Result<Self, GameDocumentError> {
        type E<G> = Errors<G>;
        payload.settings.validate().map_err(E::<G>::invalid)?;
        let bytes = document::encode(G::KIND, &payload).map_err(E::<G>::invalid)?;
        if bytes.len() > MAX_ARCHIVE_BYTES {
            return Err(E::<G>::file_size(bytes.len()));
        }
        Ok(Self { payload, bytes })
    }
    /// The saved bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// The native record: start position and every command.
    pub fn record(&self) -> &G::Record {
        &self.payload.record
    }
    /// The immutable rules profile.
    pub fn profile(&self) -> RulesProfile {
        G::profile(self.record())
    }
    /// The side the local human plays.
    pub fn human(&self) -> G::Side {
        self.payload.human
    }
    /// The saved settings.
    pub fn settings(&self) -> &SettingsOf<G> {
        &self.payload.settings
    }
    /// This document with one more command, admitted by the rules core.
    pub fn with_command(
        &self,
        actor: G::Side,
        command: G::Command,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, GameDocumentError> {
        type E<G> = Errors<G>;
        let mut game = G::to_game(self.record(), stopped).map_err(E::<G>::record)?;
        G::submit(&mut game, actor, command, stopped).map_err(E::<G>::native)?;
        self.with_game(&game)
    }
    /// This document cut back to the first `event_index` events (Undo, Take back).
    pub fn with_rewind(
        &self,
        event_index: usize,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<Self, GameDocumentError> {
        type E<G> = Errors<G>;
        let game = G::to_game(self.record(), stopped).map_err(E::<G>::record)?;
        if event_index >= G::events(&game) {
            return Err(E::<G>::invalid("rewind must remove an event"));
        }
        let prefix = G::record_prefix(&game, event_index, stopped).map_err(E::<G>::record)?;
        let mut p = self.payload.clone();
        p.record = prefix;
        Self::accept(p)
    }
    /// This document with new settings; the play mode stays the document's.
    pub fn with_settings(&self, settings: SettingsOf<G>) -> Result<Self, GameDocumentError> {
        if settings.play_mode() != self.payload.settings.play_mode() {
            return Err(Errors::<G>::invalid(
                "participant mode can only be selected for a new game",
            ));
        }
        let mut p = self.payload.clone();
        p.settings = settings;
        Self::accept(p)
    }
    /// This document holding the record of `game`.
    pub(crate) fn with_game(&self, game: &G::Game) -> Result<Self, GameDocumentError> {
        let mut p = self.payload.clone();
        p.record = G::from_game(game).map_err(Errors::<G>::record)?;
        Self::accept(p)
    }
}

#[cfg(test)]
#[path = "game_document_tests.rs"]
mod tests;
