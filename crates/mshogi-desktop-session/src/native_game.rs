//! Per-game boundary of the generic desktop document and session (MSH-217).
//!
//! Each game supplies its native types and the native calls that the generic
//! code makes; the rules stay in `mshogi_core`. A constant here records a real
//! difference between games, never a copy that drifted.
use crate::ResourceScope;
use mshogi_core::native::{CanonicalRecord, GameKind, NativeSide, RulesProfile};
use mshogi_engine::ExecutionResult;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::fmt::Debug;

/// A refused native record operation.
#[derive(Debug)]
pub enum RecordFailure {
    /// The caller stopped the operation.
    Stopped,
    /// A native resource limit; the text is the native diagnostic.
    Capacity(String),
    /// Any other refusal; the text is the native diagnostic.
    Invalid(String),
}

/// The native resource of a game capacity refusal.
#[derive(Debug)]
pub enum NativeResource {
    /// Canonical record bytes.
    EncodedBytes,
    /// Native events.
    Events,
    /// Native replay work.
    ReplayWork,
    /// Board counters or runtime identities: no one specific limit.
    Counter,
}

/// A refused native game command or query.
#[derive(Debug)]
pub enum NativeFailure {
    /// The caller stopped the operation.
    Stopped,
    /// A native resource limit and the native diagnostic.
    Capacity(NativeResource, String),
    /// Any other refusal; the text is the native diagnostic.
    Invalid(String),
}

/// Saved document settings of a game: at least the native orientation and
/// the participant mode.
pub trait DocumentSettings:
    Clone + Eq + Debug + Serialize + DeserializeOwned + Send + Sync + 'static
{
    /// Native participant.
    type Side;
    /// Settings of a new document.
    fn new(orientation: Self::Side, play_mode: crate::PlayMode) -> Self;
    /// Persisted ownership of the two native participants.
    fn play_mode(&self) -> crate::PlayMode;
    /// Keep the document's participant mode.
    fn set_play_mode(&mut self, play_mode: crate::PlayMode);
    /// Check game-specific settings fields.
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
    /// These settings with `limits` as the remembered analysis limits, for a
    /// game whose settings remember them.
    fn with_analysis_defaults(&self, _limits: mshogi_engine::WorkLimits) -> Option<Self> {
        None
    }
}

/// Evidence type of a game that saves none.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum NoEvidence {}

/// A game that the generic document and session serve.
pub trait NativeGame: Sized + Send + Sync + 'static {
    /// Display name in diagnostics.
    const NAME: &'static str;
    /// Desktop document game kind.
    const KIND: GameKind;
    /// Scope of this game's resource refusals.
    const SCOPE: ResourceScope;
    /// Both native participants, Sente first.
    const SIDES: [Self::Side; 2];

    /// Native participant.
    type Side: Copy + Eq + Debug + Serialize + DeserializeOwned + Send + Sync;
    /// Complete native command.
    type Command: Clone + Eq + Debug + Serialize + DeserializeOwned + Send + Sync;
    /// Complete native board action.
    type Action: Clone + Debug;
    /// Native game with full history.
    type Game;
    /// Canonical native record.
    type Record: Clone + PartialEq + Debug + Send + Sync;
    /// This game's engine result.
    type EngineResult;
    /// Saved document settings.
    type Settings: DocumentSettings<Side = Self::Side>;
    /// Completed non-engine work that the document keeps beside its
    /// observations (Xiangqi exact-endgame reports); `NoEvidence` otherwise.
    type Evidence: Clone + PartialEq + Debug + Serialize + DeserializeOwned + Send + Sync;

    /// Wrap a native participant.
    fn native_side(side: Self::Side) -> NativeSide;
    /// Wrap a native record.
    fn canonical(record: Self::Record) -> CanonicalRecord;
    /// Unwrap this game's native record.
    fn native(record: &CanonicalRecord) -> Option<&Self::Record>;
    /// Parse the native payload and keep the game produced by validation.
    fn parse_record(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(Self::Record, Self::Game), RecordFailure>;
    /// Immutable rules profile of a record.
    fn profile(record: &Self::Record) -> RulesProfile;
    /// Immutable rules profile of a game.
    fn game_profile(game: &Self::Game) -> RulesProfile;
    /// Events of a record.
    fn event_count(record: &Self::Record) -> usize;
    /// Side to move at the end of a record.
    fn record_side_to_move(record: &Self::Record) -> Self::Side;
    /// Reconstruct a game; the stop is checked during replay.
    fn to_game(
        record: &Self::Record,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self::Game, RecordFailure>;
    /// Canonical record of a game.
    fn from_game(game: &Self::Game) -> Result<Self::Record, RecordFailure>;
    /// Exact event prefix of a game; the stop is checked during replay.
    fn record_prefix(
        game: &Self::Game,
        count: usize,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self::Record, RecordFailure>;
    /// Submit a command in the current context.
    fn submit(
        game: &mut Self::Game,
        actor: Self::Side,
        command: Self::Command,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), NativeFailure>;
    /// Events of a game.
    fn events(game: &Self::Game) -> usize;
    /// Event index where the last command began: one command can produce
    /// several events, and history changes only at command boundaries.
    fn last_command_start(game: &Self::Game) -> Option<usize> {
        Self::events(game).checked_sub(1)
    }
    /// Validate evidence against the game it was taken on.
    fn validate_evidence(
        evidence: &Self::Evidence,
        game: &Self::Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String>;
    /// Side to move.
    fn side_to_move(game: &Self::Game) -> Self::Side;
    /// The game has a native conclusion.
    fn concluded(game: &Self::Game) -> bool;
    /// An engine Stop is already recorded in the history snapshot.
    fn stop_recorded(game: &Self::Game) -> bool;

    /// Resign command.
    fn resign() -> Self::Command;

    /// This game's engine result, if the execution result is one.
    fn engine_result(result: &ExecutionResult) -> Option<&Self::EngineResult>;
    /// The complete command, with its actor, that an engine result proposes.
    fn engine_command(
        result: &Self::EngineResult,
        game: &Self::Game,
    ) -> Option<(Self::Side, Self::Command)>;
    /// Validate a result against its target game.
    fn validate_result(
        result: &Self::EngineResult,
        game: &Self::Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String>;
}

/// The Chu and Tenjiku cores have the same API; one impl serves both.
macro_rules! large_shogi_game {
    ($game:ident, $core:ident, $glue:ident, $engine_result:ident, $result_variant:ident, $scope:ident,
     profile: |$r:ident| $profile:expr, game_profile: |$g:ident| $game_profile:expr) => {
        /// Marker of the native game behind the generic document and session.
        pub struct $game;
        impl NativeGame for $game {
            const NAME: &'static str = stringify!($result_variant);
            const KIND: GameKind = GameKind::$result_variant;
            const SCOPE: ResourceScope = ResourceScope::$scope;
            const SIDES: [Self::Side; 2] = [
                mshogi_core::$core::Side::Sente,
                mshogi_core::$core::Side::Gote,
            ];

            type Side = mshogi_core::$core::Side;
            type Command = mshogi_core::$core::GameCommand;
            type Action = mshogi_core::$core::Action;
            type Game = mshogi_core::$core::Game;
            type Record = mshogi_core::$core::GameRecord;
            type EngineResult = mshogi_engine::$engine_result;
            type Settings = crate::game_document::GameSettings<Self::Side>;
            type Evidence = NoEvidence;
            fn validate_evidence(
                evidence: &NoEvidence,
                _game: &Self::Game,
                _stop: &mut impl FnMut() -> bool,
            ) -> Result<(), String> {
                match *evidence {}
            }

            fn native_side(side: Self::Side) -> NativeSide {
                NativeSide::$result_variant(side)
            }
            fn canonical(record: Self::Record) -> CanonicalRecord {
                CanonicalRecord::$core(record)
            }
            fn native(record: &CanonicalRecord) -> Option<&Self::Record> {
                match record {
                    CanonicalRecord::$result_variant(r) => Some(r),
                    _ => None,
                }
            }
            fn parse_record(
                text: &str,
                stop: &mut impl FnMut() -> bool,
            ) -> Result<(Self::Record, Self::Game), RecordFailure> {
                mshogi_core::$core::GameRecord::from_json_with_game(text, stop)
                    .map_err($glue::record_failure)
            }
            fn profile($r: &Self::Record) -> RulesProfile {
                $profile
            }
            fn game_profile($g: &Self::Game) -> RulesProfile {
                $game_profile
            }
            fn event_count(record: &Self::Record) -> usize {
                record.event_count()
            }
            fn record_side_to_move(record: &Self::Record) -> Self::Side {
                record.side_to_move()
            }
            fn to_game(
                record: &Self::Record,
                stop: &mut impl FnMut() -> bool,
            ) -> Result<Self::Game, RecordFailure> {
                record
                    .to_game_with_stop(stop)
                    .map_err($glue::record_failure)
            }
            fn from_game(game: &Self::Game) -> Result<Self::Record, RecordFailure> {
                mshogi_core::$core::GameRecord::from_game(game).map_err($glue::record_failure)
            }
            fn record_prefix(
                game: &Self::Game,
                count: usize,
                stop: &mut impl FnMut() -> bool,
            ) -> Result<Self::Record, RecordFailure> {
                game.record_prefix_with_stop(count, stop)
                    .map_err($glue::record_failure)
            }
            fn submit(
                game: &mut Self::Game,
                actor: Self::Side,
                command: Self::Command,
                stop: &mut impl FnMut() -> bool,
            ) -> Result<(), NativeFailure> {
                let expected = game.context();
                game.submit_with_stop(
                    mshogi_core::$core::GameRequest {
                        expected,
                        actor,
                        command,
                    },
                    stop,
                )
                .map_err($glue::native_failure)
                .map(|_| ())
            }
            fn events(game: &Self::Game) -> usize {
                game.events().len()
            }
            fn side_to_move(game: &Self::Game) -> Self::Side {
                game.position().side_to_move()
            }
            fn concluded(game: &Self::Game) -> bool {
                game.conclusion().is_some()
            }
            fn stop_recorded(game: &Self::Game) -> bool {
                game.snapshot().incomplete.is_some()
            }
            fn resign() -> Self::Command {
                mshogi_core::$core::GameCommand::Resign
            }
            fn engine_result(result: &ExecutionResult) -> Option<&Self::EngineResult> {
                match result {
                    ExecutionResult::$result_variant { result } => Some(result),
                    _ => None,
                }
            }
            fn engine_command(
                result: &Self::EngineResult,
                game: &Self::Game,
            ) -> Option<(Self::Side, Self::Command)> {
                match result {
                    mshogi_engine::$engine_result::Choice { action, .. } => Some((
                        game.position().side_to_move(),
                        mshogi_core::$core::GameCommand::Play {
                            action: action.clone(),
                        },
                    )),
                    _ => None,
                }
            }
            fn validate_result(
                result: &Self::EngineResult,
                game: &Self::Game,
                stop: &mut impl FnMut() -> bool,
            ) -> Result<(), String> {
                result.validate_with_work(game, stop).map(|_| ())
            }
        }
        mod $glue {
            use super::*;
            use mshogi_core::$core as c;
            pub(super) fn record_failure(e: c::RecordError) -> RecordFailure {
                match e {
                    c::RecordError::Stopped => RecordFailure::Stopped,
                    c::RecordError::Capacity => RecordFailure::Capacity(e.to_string()),
                    e => RecordFailure::Invalid(e.to_string()),
                }
            }
            pub(super) fn native_failure(e: c::GameError) -> NativeFailure {
                match e {
                    c::GameError::Stopped => NativeFailure::Stopped,
                    c::GameError::Capacity(r) => NativeFailure::Capacity(
                        match r {
                            c::Resource::EncodedBytes => NativeResource::EncodedBytes,
                            c::Resource::Events => NativeResource::Events,
                            c::Resource::ReplayWork => NativeResource::ReplayWork,
                            c::Resource::Counter => NativeResource::Counter,
                        },
                        e.to_string(),
                    ),
                    e => NativeFailure::Invalid(e.to_string()),
                }
            }
        }
    };
}

large_shogi_game!(Chu, chu, chu_glue, ChuResult, Chu, Chu,
    profile: |_r| RulesProfile::ChuJcsa2019V1, game_profile: |_g| RulesProfile::ChuJcsa2019V1);
large_shogi_game!(Tenjiku, tenjiku, tenjiku_glue, TenjikuResult, Tenjiku, Tenjiku,
    profile: |r| RulesProfile::from(r.profile()), game_profile: |g| RulesProfile::from(g.profile()));

/// Marker of the native Tori game behind the generic document and session.
pub struct Tori;
impl NativeGame for Tori {
    const NAME: &'static str = "Tori";
    const KIND: GameKind = GameKind::Tori;
    const SCOPE: ResourceScope = ResourceScope::Tori;
    const SIDES: [Self::Side; 2] = [
        mshogi_core::tori::Side::Sente,
        mshogi_core::tori::Side::Gote,
    ];

    type Side = mshogi_core::tori::Side;
    type Command = mshogi_core::tori::GameCommand;
    type Action = mshogi_core::tori::Action;
    type Game = mshogi_core::tori::Game;
    type Record = mshogi_core::tori::GameRecord;
    type EngineResult = mshogi_engine::ToriResult;
    type Settings = crate::game_document::GameSettings<Self::Side>;
    type Evidence = NoEvidence;
    fn validate_evidence(
        evidence: &NoEvidence,
        _game: &Self::Game,
        _stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        match *evidence {}
    }

    fn native_side(side: Self::Side) -> NativeSide {
        NativeSide::Tori(side)
    }
    fn canonical(record: Self::Record) -> CanonicalRecord {
        CanonicalRecord::tori(record)
    }
    fn native(record: &CanonicalRecord) -> Option<&Self::Record> {
        match record {
            CanonicalRecord::Tori(r) => Some(r),
            _ => None,
        }
    }
    fn parse_record(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(Self::Record, Self::Game), RecordFailure> {
        mshogi_core::tori::GameRecord::from_json_with_game(text, stop)
            .map_err(tori_glue::record_failure)
    }
    fn profile(_record: &Self::Record) -> RulesProfile {
        RulesProfile::ToriModernV1
    }
    fn game_profile(_game: &Self::Game) -> RulesProfile {
        RulesProfile::ToriModernV1
    }
    fn event_count(record: &Self::Record) -> usize {
        record.event_count()
    }
    fn record_side_to_move(record: &Self::Record) -> Self::Side {
        record.side_to_move()
    }
    fn to_game(
        record: &Self::Record,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self::Game, RecordFailure> {
        record
            .to_game_with_stop(stop)
            .map_err(tori_glue::record_failure)
    }
    fn from_game(game: &Self::Game) -> Result<Self::Record, RecordFailure> {
        mshogi_core::tori::GameRecord::from_game(game).map_err(tori_glue::record_failure)
    }
    fn record_prefix(
        game: &Self::Game,
        count: usize,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self::Record, RecordFailure> {
        // A Tori prefix copies the cached history; it does not replay.
        if stop() {
            return Err(RecordFailure::Stopped);
        }
        game.record_prefix(count).map_err(tori_glue::record_failure)
    }
    fn submit(
        game: &mut Self::Game,
        actor: Self::Side,
        command: Self::Command,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), NativeFailure> {
        if stop() {
            return Err(NativeFailure::Stopped);
        }
        let expected = game.context();
        game.submit(mshogi_core::tori::GameRequest {
            expected,
            actor,
            command,
        })
        .map_err(tori_glue::native_failure)
        .map(|_| ())
    }
    fn events(game: &Self::Game) -> usize {
        game.events().len()
    }
    fn side_to_move(game: &Self::Game) -> Self::Side {
        game.position().side_to_move()
    }
    fn concluded(game: &Self::Game) -> bool {
        game.conclusion().is_some()
    }
    fn stop_recorded(game: &Self::Game) -> bool {
        game.snapshot().incomplete.is_some()
    }
    fn resign() -> Self::Command {
        mshogi_core::tori::GameCommand::Resign
    }
    fn engine_result(result: &ExecutionResult) -> Option<&Self::EngineResult> {
        match result {
            ExecutionResult::Tori { result } => Some(result),
            _ => None,
        }
    }
    fn engine_command(
        result: &Self::EngineResult,
        game: &Self::Game,
    ) -> Option<(Self::Side, Self::Command)> {
        match result {
            mshogi_engine::ToriResult::Choice { action, .. } => Some((
                game.position().side_to_move(),
                mshogi_core::tori::GameCommand::Play { action: *action },
            )),
            _ => None,
        }
    }
    fn validate_result(
        result: &Self::EngineResult,
        game: &Self::Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        if stop() {
            return Err("Tori observation validation stopped".into());
        }
        // Validation copies the game without replay and plays the line.
        result.validate(game)?;
        Ok(())
    }
}
mod tori_glue {
    use super::*;
    use mshogi_core::tori as t;
    pub(super) fn record_failure(e: t::RecordError) -> RecordFailure {
        match e {
            t::RecordError::Stopped => RecordFailure::Stopped,
            t::RecordError::Capacity => RecordFailure::Capacity(e.to_string()),
            e => RecordFailure::Invalid(e.to_string()),
        }
    }
    pub(super) fn native_failure(e: t::GameError) -> NativeFailure {
        match e {
            t::GameError::Capacity(r) => NativeFailure::Capacity(
                match r {
                    t::Resource::EncodedBytes => NativeResource::EncodedBytes,
                    t::Resource::Events => NativeResource::Events,
                    t::Resource::Counter => NativeResource::Counter,
                },
                e.to_string(),
            ),
            e => NativeFailure::Invalid(e.to_string()),
        }
    }
}

/// Marker of the native Shogi game behind the generic document and session.
pub struct Shogi;
impl NativeGame for Shogi {
    const NAME: &'static str = "Shogi";
    const KIND: GameKind = GameKind::Shogi;
    const SCOPE: ResourceScope = ResourceScope::Shogi;
    const SIDES: [Self::Side; 2] = [
        mshogi_core::shogi::Side::Sente,
        mshogi_core::shogi::Side::Gote,
    ];

    type Side = mshogi_core::shogi::Side;
    type Command = mshogi_core::shogi::GameCommand;
    type Action = mshogi_core::shogi::Action;
    type Game = mshogi_core::shogi::Game;
    type Record = mshogi_core::shogi::GameRecord;
    type EngineResult = mshogi_engine::ShogiResult;
    type Settings = crate::shared::archive::ShogiSettings;
    type Evidence = NoEvidence;
    fn validate_evidence(
        evidence: &NoEvidence,
        _game: &Self::Game,
        _stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        match *evidence {}
    }

    fn native_side(side: Self::Side) -> NativeSide {
        NativeSide::Shogi(side)
    }
    fn canonical(record: Self::Record) -> CanonicalRecord {
        CanonicalRecord::shogi(record)
    }
    fn native(record: &CanonicalRecord) -> Option<&Self::Record> {
        match record {
            CanonicalRecord::Shogi(r) => Some(r),
            _ => None,
        }
    }
    fn parse_record(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(Self::Record, Self::Game), RecordFailure> {
        if stop() {
            return Err(RecordFailure::Stopped);
        }
        let parsed =
            mshogi_core::shogi::GameRecord::from_json_with_game(text).map_err(|e| match e {
                mshogi_core::shogi::RecordError::Capacity => RecordFailure::Capacity(e.to_string()),
                e => RecordFailure::Invalid(e.to_string()),
            })?;
        if stop() {
            return Err(RecordFailure::Stopped);
        }
        Ok(parsed)
    }
    fn profile(_record: &Self::Record) -> RulesProfile {
        RulesProfile::ShogiCsa36V1
    }
    fn game_profile(_game: &Self::Game) -> RulesProfile {
        RulesProfile::ShogiCsa36V1
    }
    fn event_count(record: &Self::Record) -> usize {
        record.event_count()
    }
    fn record_side_to_move(record: &Self::Record) -> Self::Side {
        record.side_to_move()
    }
    fn to_game(
        record: &Self::Record,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self::Game, RecordFailure> {
        // A Shogi record has at most 321 events; replay checks the stop before and after.
        if stop() {
            return Err(RecordFailure::Stopped);
        }
        let game = record.to_game().map_err(|e| match e {
            mshogi_core::shogi::RecordError::Capacity => RecordFailure::Capacity(e.to_string()),
            e => RecordFailure::Invalid(e.to_string()),
        })?;
        if stop() {
            return Err(RecordFailure::Stopped);
        }
        Ok(game)
    }
    fn from_game(game: &Self::Game) -> Result<Self::Record, RecordFailure> {
        Ok(mshogi_core::shogi::GameRecord::from_game(game))
    }
    fn record_prefix(
        game: &Self::Game,
        count: usize,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self::Record, RecordFailure> {
        if stop() {
            return Err(RecordFailure::Stopped);
        }
        let prefix = game
            .prefix(count)
            .map_err(|e| RecordFailure::Invalid(format!("{e:?}")))?;
        Ok(mshogi_core::shogi::GameRecord::from_game(&prefix))
    }
    fn submit(
        game: &mut Self::Game,
        actor: Self::Side,
        command: Self::Command,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), NativeFailure> {
        if stop() {
            return Err(NativeFailure::Stopped);
        }
        let expected = game.context();
        game.submit(mshogi_core::shogi::GameRequest {
            expected,
            actor,
            command,
        })
        .map_err(|e| NativeFailure::Invalid(format!("Native shogi command rejected: {e:?}")))
        .map(|_| ())
    }
    fn events(game: &Self::Game) -> usize {
        game.events().len()
    }
    fn side_to_move(game: &Self::Game) -> Self::Side {
        game.position().side_to_move()
    }
    fn concluded(game: &Self::Game) -> bool {
        game.conclusion().is_some()
    }
    fn stop_recorded(_game: &Self::Game) -> bool {
        false
    }
    fn resign() -> Self::Command {
        mshogi_core::shogi::GameCommand::Resign
    }
    fn engine_result(result: &ExecutionResult) -> Option<&Self::EngineResult> {
        match result {
            ExecutionResult::Shogi { result } => Some(result),
            _ => None,
        }
    }
    fn engine_command(
        result: &Self::EngineResult,
        game: &Self::Game,
    ) -> Option<(Self::Side, Self::Command)> {
        match result {
            // An engine declares only for the side to move.
            mshogi_engine::ShogiResult::Choice { choice, .. } => Some(shogi_choice(choice, game))
                .filter(|(actor, _)| *actor == game.position().side_to_move()),
            _ => None,
        }
    }
    fn validate_result(
        result: &Self::EngineResult,
        game: &Self::Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        // The result identity is checked with the observation; replay its line
        // on a private copy of the game.
        let line = match result {
            mshogi_engine::ShogiResult::Choice { telemetry, .. }
            | mshogi_engine::ShogiResult::Finished { telemetry, .. }
            | mshogi_engine::ShogiResult::Incomplete { telemetry } => {
                &telemetry.principal_variation
            }
        };
        if let mshogi_engine::ShogiResult::Choice {
            choice: mshogi_engine::ShogiChoice::Declare { actor },
            ..
        } = result
            && *actor != game.position().side_to_move()
        {
            return Err("Shogi engine declaration is not for the side to move".into());
        }
        let mut probe = game.clone();
        for choice in line {
            let (actor, command) = shogi_choice(choice, &probe);
            Self::submit(&mut probe, actor, command, stop).map_err(|e| match e {
                NativeFailure::Stopped => "Shogi observation validation stopped".to_string(),
                NativeFailure::Capacity(_, d) | NativeFailure::Invalid(d) => d,
            })?;
        }
        Ok(())
    }
}
/// The actor and command of an engine choice in `game`.
fn shogi_choice(
    choice: &mshogi_engine::ShogiChoice,
    game: &mshogi_core::shogi::Game,
) -> (mshogi_core::shogi::Side, mshogi_core::shogi::GameCommand) {
    match choice {
        mshogi_engine::ShogiChoice::Move { action } => (
            game.position().side_to_move(),
            mshogi_core::shogi::GameCommand::Play(*action),
        ),
        mshogi_engine::ShogiChoice::Declare { actor } => {
            (*actor, mshogi_core::shogi::GameCommand::DeclareEnteringKing)
        }
    }
}

/// Marker of the native Xiangqi game behind the generic document and session.
pub struct Xiangqi;
impl NativeGame for Xiangqi {
    const NAME: &'static str = "Xiangqi";
    const KIND: GameKind = GameKind::Xiangqi;
    const SCOPE: ResourceScope = ResourceScope::Xiangqi;
    const SIDES: [Self::Side; 2] = [
        mshogi_core::xiangqi::Side::Red,
        mshogi_core::xiangqi::Side::Black,
    ];

    type Side = mshogi_core::xiangqi::Side;
    type Command = mshogi_core::xiangqi::GameCommand;
    type Action = mshogi_core::xiangqi::Action;
    type Game = mshogi_core::xiangqi::Game;
    type Record = mshogi_core::xiangqi::GameRecord;
    type EngineResult = mshogi_engine::XiangqiResult;
    type Settings = crate::shared::xiangqi::XiangqiSettings;
    type Evidence = crate::shared::xiangqi::XiangqiExactEvidence;

    fn native_side(side: Self::Side) -> NativeSide {
        NativeSide::Xiangqi(side)
    }
    fn canonical(record: Self::Record) -> CanonicalRecord {
        CanonicalRecord::xiangqi(record)
    }
    fn native(record: &CanonicalRecord) -> Option<&Self::Record> {
        match record {
            CanonicalRecord::Xiangqi(r) => Some(r),
            _ => None,
        }
    }
    fn parse_record(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(Self::Record, Self::Game), RecordFailure> {
        if stop() {
            return Err(RecordFailure::Stopped);
        }
        let parsed =
            mshogi_core::xiangqi::GameRecord::from_json_with_game(text).map_err(|e| match e {
                mshogi_core::xiangqi::RecordError::Capacity => {
                    RecordFailure::Capacity(e.to_string())
                }
                e => RecordFailure::Invalid(e.to_string()),
            })?;
        if stop() {
            return Err(RecordFailure::Stopped);
        }
        Ok(parsed)
    }
    fn profile(_record: &Self::Record) -> RulesProfile {
        RulesProfile::XiangqiWxf2018V1
    }
    fn game_profile(_game: &Self::Game) -> RulesProfile {
        RulesProfile::XiangqiWxf2018V1
    }
    fn event_count(record: &Self::Record) -> usize {
        record.event_count()
    }
    fn record_side_to_move(record: &Self::Record) -> Self::Side {
        record.side_to_move()
    }
    fn to_game(
        record: &Self::Record,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self::Game, RecordFailure> {
        // A Xiangqi record replays in milliseconds; check the stop around it.
        if stop() {
            return Err(RecordFailure::Stopped);
        }
        let game = record.to_game().map_err(|e| match e {
            mshogi_core::xiangqi::RecordError::Capacity => RecordFailure::Capacity(e.to_string()),
            e => RecordFailure::Invalid(e.to_string()),
        })?;
        if stop() {
            return Err(RecordFailure::Stopped);
        }
        Ok(game)
    }
    fn from_game(game: &Self::Game) -> Result<Self::Record, RecordFailure> {
        mshogi_core::xiangqi::GameRecord::from_game(game).map_err(|e| match e {
            mshogi_core::xiangqi::RecordError::Capacity => RecordFailure::Capacity(e.to_string()),
            e => RecordFailure::Invalid(e.to_string()),
        })
    }
    fn record_prefix(
        game: &Self::Game,
        count: usize,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self::Record, RecordFailure> {
        // History stops only at command boundaries (MSH-226, John).
        if stop() {
            return Err(RecordFailure::Stopped);
        }
        let prefix = game
            .committed_prefix(count)
            .map_err(|e| RecordFailure::Invalid(e.to_string()))?;
        Self::from_game(&prefix)
    }
    fn submit(
        game: &mut Self::Game,
        actor: Self::Side,
        command: Self::Command,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), NativeFailure> {
        if stop() {
            return Err(NativeFailure::Stopped);
        }
        let expected = game.context();
        game.submit(mshogi_core::xiangqi::GameRequest {
            expected,
            actor,
            command,
        })
        .map(|s| s.undo)
        .map_err(|e| NativeFailure::Invalid(e.to_string()))
        .map(|_| ())
    }
    fn events(game: &Self::Game) -> usize {
        game.events().len()
    }
    fn last_command_start(game: &Self::Game) -> Option<usize> {
        game.command_event_ranges().last().map(|(start, _)| start)
    }
    fn side_to_move(game: &Self::Game) -> Self::Side {
        game.position().side_to_move()
    }
    fn concluded(game: &Self::Game) -> bool {
        // Pending adjudication admits no further command either.
        game.conclusion().is_some() || game.pending_adjudication().is_some()
    }
    fn stop_recorded(_game: &Self::Game) -> bool {
        false
    }
    fn resign() -> Self::Command {
        mshogi_core::xiangqi::GameCommand::Resign
    }
    fn engine_result(result: &ExecutionResult) -> Option<&Self::EngineResult> {
        match result {
            ExecutionResult::Xiangqi { result } => Some(result),
            _ => None,
        }
    }
    fn engine_command(
        result: &Self::EngineResult,
        game: &Self::Game,
    ) -> Option<(Self::Side, Self::Command)> {
        match result {
            mshogi_engine::XiangqiResult::Choice { choice, .. } => {
                Some(xiangqi_choice(*choice, game.position().side_to_move()))
            }
            _ => None,
        }
    }
    fn validate_result(
        result: &Self::EngineResult,
        game: &Self::Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        let line = match result {
            mshogi_engine::XiangqiResult::Choice { telemetry, .. }
            | mshogi_engine::XiangqiResult::Finished { telemetry, .. }
            | mshogi_engine::XiangqiResult::Incomplete { telemetry }
            | mshogi_engine::XiangqiResult::Unresolved { telemetry, .. } => {
                &telemetry.principal_variation
            }
        };
        if line.is_empty() {
            return Ok(());
        }
        let mut probe = game.fork().map_err(|e| e.to_string())?;
        for choice in line {
            let (actor, command) = xiangqi_choice(*choice, probe.position().side_to_move());
            Self::submit(&mut probe, actor, command, stop).map_err(|e| match e {
                NativeFailure::Stopped => "Xiangqi observation validation stopped".to_string(),
                NativeFailure::Capacity(_, d) | NativeFailure::Invalid(d) => d,
            })?;
        }
        Ok(())
    }
    fn validate_evidence(
        evidence: &Self::Evidence,
        game: &Self::Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        evidence.validate_against(game, stop)
    }
}
/// The actor and command of an engine choice.
fn xiangqi_choice(
    choice: mshogi_engine::XiangqiChoice,
    turn: mshogi_core::xiangqi::Side,
) -> (
    mshogi_core::xiangqi::Side,
    mshogi_core::xiangqi::GameCommand,
) {
    match choice {
        mshogi_engine::XiangqiChoice::Move { action } => {
            (turn, mshogi_core::xiangqi::GameCommand::Play { action })
        }
        mshogi_engine::XiangqiChoice::Claim { actor } => {
            (actor, mshogi_core::xiangqi::GameCommand::ClaimNaturalCount)
        }
    }
}
