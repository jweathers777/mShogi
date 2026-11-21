//! Per-game boundary of the generic desktop document and session (MSH-217).
//!
//! Each game supplies its native types and the native calls that the generic
//! code makes; the rules stay in `mshogi_core`. A constant here records a real
//! difference between games, never a copy that drifted.
use crate::ResourceScope;
use mshogi_core::native::{CanonicalRecord, GameKind, RulesProfile};
use mshogi_engine::ExecutionResult;
use serde::{Serialize, de::DeserializeOwned};
use std::fmt::Debug;

/// A refused native record operation.
pub enum RecordFailure {
    /// The caller stopped the operation.
    Stopped,
    /// A native resource limit; the text is the native diagnostic.
    Capacity(String),
    /// Any other refusal; the text is the native diagnostic.
    Invalid(String),
}

/// The native resource of a game capacity refusal.
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
pub enum NativeFailure {
    /// The caller stopped the operation.
    Stopped,
    /// A native resource limit and the native diagnostic.
    Capacity(NativeResource, String),
    /// Any other refusal; the text is the native diagnostic.
    Invalid(String),
}

/// A game that the generic document and session serve.
pub trait NativeGame: Sized + Send + Sync + 'static {
    /// Display name in diagnostics.
    const NAME: &'static str;
    /// Desktop document game kind.
    const KIND: GameKind;
    /// Scope of this game's resource refusals.
    const SCOPE: ResourceScope;
    /// Measured aggregate replay allowance of one document.
    const MAX_DOCUMENT_WORK: u64;
    /// Native rules: an engine Stop is admitted after a conclusion.
    const STOP_AFTER_CONCLUSION: bool;
    /// Recorded saved-format compatibility: a saved control must equal its
    /// canonical form. False keeps older files with defaulted fields loadable.
    const STRICT_CONTROL_FIELDS: bool;
    /// Native per-record canonical byte limit, reported in capacity refusals.
    const MAX_RECORD_BYTES: u64;
    /// Native per-record event limit.
    const MAX_EVENTS: u64;
    /// Native per-record replay work limit.
    const MAX_REPLAY_WORK: u64;
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
    /// Native reversal handle of one submitted command.
    type Undo;
    /// Canonical native record.
    type Record: Clone + PartialEq + Debug + Send + Sync;
    /// This game's engine result.
    type EngineResult;

    /// Wrap a native record.
    fn canonical(record: Self::Record) -> CanonicalRecord;
    /// Unwrap this game's native record.
    fn native(record: &CanonicalRecord) -> Option<&Self::Record>;
    /// Immutable rules profile of a record.
    fn profile(record: &Self::Record) -> RulesProfile;
    /// Deterministic work to reconstruct a record.
    fn reconstruction_work(record: &Self::Record) -> u64;
    /// Events of a record.
    fn event_count(record: &Self::Record) -> usize;
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
    ) -> Result<Self::Undo, NativeFailure>;
    /// Reverse one submitted command.
    fn undo(game: &mut Self::Game, undo: &mut Self::Undo) -> Result<(), NativeFailure>;
    /// Native replay work of a game so far.
    fn replay_work(game: &Self::Game) -> u64;
    /// Events of a game.
    fn events(game: &Self::Game) -> usize;
    /// Side to move.
    fn side_to_move(game: &Self::Game) -> Self::Side;
    /// The game has a native conclusion.
    fn concluded(game: &Self::Game) -> bool;
    /// An engine Stop is already recorded in the history snapshot.
    fn stop_recorded(game: &Self::Game) -> bool;

    /// Play command of an action.
    fn play(action: Self::Action) -> Self::Command;
    /// Resign command.
    fn resign() -> Self::Command;
    /// Engine-failure Stop command.
    fn engine_failure_stop() -> Self::Command;

    /// This game's engine result, if the execution result is one.
    fn engine_result(result: &ExecutionResult) -> Option<&Self::EngineResult>;
    /// The engine's proposed action of a result, if it proposes one.
    fn engine_choice(result: &Self::EngineResult) -> Option<&Self::Action>;
    /// Validate a result against its target game; the native work it used.
    fn validate_result(
        result: &Self::EngineResult,
        game: &Self::Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<u64, String>;
}

/// The Chu and Tenjiku cores have the same API; one impl serves both.
macro_rules! large_shogi_game {
    ($game:ident, $core:ident, $glue:ident, $engine_result:ident, $result_variant:ident, $scope:ident,
     work: $work:expr, stop_after_conclusion: $stop:expr, strict: $strict:expr,
     profile: |$r:ident| $profile:expr) => {
        /// Marker of the native game behind the generic document and session.
        pub struct $game;
        impl NativeGame for $game {
            const NAME: &'static str = stringify!($result_variant);
            const KIND: GameKind = GameKind::$result_variant;
            const SCOPE: ResourceScope = ResourceScope::$scope;
            const MAX_DOCUMENT_WORK: u64 = $work;
            const STOP_AFTER_CONCLUSION: bool = $stop;
            const STRICT_CONTROL_FIELDS: bool = $strict;
            const MAX_RECORD_BYTES: u64 = mshogi_core::$core::MAX_GAME_RECORD_BYTES as u64;
            const MAX_EVENTS: u64 = mshogi_core::$core::MAX_GAME_EVENTS as u64;
            const MAX_REPLAY_WORK: u64 = mshogi_core::$core::MAX_REPLAY_WORK;
            const SIDES: [Self::Side; 2] = [
                mshogi_core::$core::Side::Sente,
                mshogi_core::$core::Side::Gote,
            ];

            type Side = mshogi_core::$core::Side;
            type Command = mshogi_core::$core::GameCommand;
            type Action = mshogi_core::$core::Action;
            type Game = mshogi_core::$core::Game;
            type Undo = mshogi_core::$core::GameUndo;
            type Record = mshogi_core::$core::GameRecord;
            type EngineResult = mshogi_engine::$engine_result;

            fn canonical(record: Self::Record) -> CanonicalRecord {
                CanonicalRecord::$core(record)
            }
            fn native(record: &CanonicalRecord) -> Option<&Self::Record> {
                match record {
                    CanonicalRecord::$result_variant(r) => Some(r),
                    _ => None,
                }
            }
            fn profile($r: &Self::Record) -> RulesProfile {
                $profile
            }
            fn reconstruction_work(record: &Self::Record) -> u64 {
                record.reconstruction_work()
            }
            fn event_count(record: &Self::Record) -> usize {
                record.event_count()
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
            ) -> Result<Self::Undo, NativeFailure> {
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
            }
            fn undo(game: &mut Self::Game, undo: &mut Self::Undo) -> Result<(), NativeFailure> {
                game.undo(undo).map(|_| ()).map_err($glue::native_failure)
            }
            fn replay_work(game: &Self::Game) -> u64 {
                game.replay_work()
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
            fn play(action: Self::Action) -> Self::Command {
                mshogi_core::$core::GameCommand::Play { action }
            }
            fn resign() -> Self::Command {
                mshogi_core::$core::GameCommand::Resign
            }
            fn engine_failure_stop() -> Self::Command {
                mshogi_core::$core::GameCommand::Stop {
                    reason: mshogi_core::$core::StopReason::EngineFailure,
                }
            }
            fn engine_result(result: &ExecutionResult) -> Option<&Self::EngineResult> {
                match result {
                    ExecutionResult::$result_variant { result } => Some(result),
                    _ => None,
                }
            }
            fn engine_choice(result: &Self::EngineResult) -> Option<&Self::Action> {
                match result {
                    mshogi_engine::$engine_result::Choice { action, .. } => Some(action),
                    _ => None,
                }
            }
            fn validate_result(
                result: &Self::EngineResult,
                game: &Self::Game,
                stop: &mut impl FnMut() -> bool,
            ) -> Result<u64, String> {
                result.validate_with_work(game, stop)
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
    work: 12_000_000, stop_after_conclusion: false, strict: false,
    profile: |_r| RulesProfile::ChuJcsa2019V1);
large_shogi_game!(Tenjiku, tenjiku, tenjiku_glue, TenjikuResult, Tenjiku, Tenjiku,
    // Measured: see `tenjiku_document::MAX_TENJIKU_DOCUMENT_WORK`.
    work: 160_000_000, stop_after_conclusion: true, strict: true,
    profile: |r| RulesProfile::from(r.profile()));
