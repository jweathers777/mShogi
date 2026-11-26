//! Per-game boundary of the generic desktop document and session (MSH-217).
//!
//! Each game supplies its native types and the native calls that the generic
//! code makes; the rules stay in `mshogi_core`. A constant here records a real
//! difference between games, never a copy that drifted.
use crate::ResourceScope;
use crate::analysis::AnalysisOperation;
pub use crate::native_chess::Chess;
use mshogi_core::native::{CanonicalRecord, GameKind, NativeSide, RulesProfile};
use mshogi_engine::{EngineGame, ExecutionResult, WorkLimits};
use serde::{Serialize, de::DeserializeOwned};
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
    /// Encoded native root bytes.
    EncodedBytes,
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
    /// Work limits of an engine turn.
    const PLAY_LIMITS: WorkLimits;
    /// Largest accepted search depth.
    const MAX_DEPTH: u16;
    /// Largest accepted visit budget.
    const MAX_NODES: u64;
    /// Analysis uses the fixed move-analysis budgets instead of the bounds above.
    const ANALYSIS_BUDGETS: bool = false;
    /// Analysis limits of a new document.
    const ANALYSIS_DEFAULTS: AnalysisOperation = AnalysisOperation {
        depth: Self::PLAY_LIMITS.depth,
        nodes: Self::PLAY_LIMITS.nodes,
    };

    /// Native participant.
    type Side: Copy + Eq + Debug + Serialize + DeserializeOwned + Send + Sync;
    /// Complete native command.
    type Command: Clone + Eq + Debug + Serialize + DeserializeOwned + Send + Sync;
    /// Complete native board action.
    type Action: Clone + Debug;
    /// Native game with full history.
    type Game;
    /// Canonical native record.
    type Record: Clone + PartialEq + Debug + Serialize + Send + Sync;
    /// This game's engine result.
    type EngineResult;

    /// Wrap a native participant.
    fn native_side(side: Self::Side) -> NativeSide;
    /// The native participant of a game-qualified side of this game.
    fn side_of(side: NativeSide) -> Option<Self::Side> {
        Self::SIDES
            .into_iter()
            .find(|s| Self::native_side(*s) == side)
    }
    /// Record of a new standard game under `profile`.
    fn new_record(profile: RulesProfile) -> Result<Self::Record, String>;
    /// The native record of a canonical record of this game.
    fn record_of(record: CanonicalRecord) -> Option<Self::Record>;
    /// Exact canonical context sent to the engine.
    fn context(game: &Self::Game) -> Result<CanonicalRecord, RecordFailure>;
    /// Side to move in an admitted engine context, without replay.
    fn context_side(record: &CanonicalRecord) -> Self::Side;
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
    /// Side to move.
    fn side_to_move(game: &Self::Game) -> Self::Side;
    /// The game has a native conclusion.
    fn concluded(game: &Self::Game) -> bool;

    /// Resign command.
    fn resign() -> Self::Command;

    /// Execution failures can be carried by a native result family.
    fn failure(result: &ExecutionResult) -> Option<&mshogi_engine::EngineFailure> {
        match result {
            ExecutionResult::Failure { failure } => Some(failure),
            _ => None,
        }
    }
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
        _context: &mshogi_engine::ExecutionContext,
        game: &Self::Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String>;
}

// Five games use the same native record for persistence and engine requests.
// Keep that wiring in one place; Chess supplies a separate match record.
macro_rules! canonical_record {
    ($variant:ident) => {
        /// Engine context of a game. It can differ from the durable match record.
        fn context(game: &Self::Game) -> Result<CanonicalRecord, RecordFailure> {
            Self::from_game(game).map(|record| CanonicalRecord::$variant(Box::new(record)))
        }
        fn record_of(record: CanonicalRecord) -> Option<Self::Record> {
            match record {
                CanonicalRecord::$variant(record) => Some(*record),
                _ => None,
            }
        }
        /// Side to move in an engine context, without replay.
        fn context_side(record: &CanonicalRecord) -> Self::Side {
            let CanonicalRecord::$variant(record) = record else {
                panic!("validated native context")
            };
            Self::record_side_to_move(record)
        }
    };
}

// The five games on the shared search core use one engine adapter.
macro_rules! engine_glue {
    ($engine:ident) => {
        const PLAY_LIMITS: WorkLimits = <mshogi_engine::game::$engine as EngineGame>::PLAY_LIMITS;
        const MAX_DEPTH: u16 = <mshogi_engine::game::$engine as EngineGame>::MAX_DEPTH;
        const MAX_NODES: u64 = <mshogi_engine::game::$engine as EngineGame>::MAX_NODES;
        type EngineResult = mshogi_engine::EngineResult<mshogi_engine::game::$engine>;
        fn engine_result(result: &ExecutionResult) -> Option<&Self::EngineResult> {
            <mshogi_engine::game::$engine as EngineGame>::result(result)
        }
        fn engine_command(
            result: &Self::EngineResult,
            game: &Self::Game,
        ) -> Option<(Self::Side, Self::Command)> {
            use mshogi_engine::game::$engine as E;
            match result {
                mshogi_engine::EngineResult::Choice { choice, .. } => {
                    Some(E::command(choice, E::side_to_move(game)))
                }
                _ => None,
            }
        }
        fn validate_result(
            result: &Self::EngineResult,
            _context: &mshogi_engine::ExecutionContext,
            game: &Self::Game,
            stop: &mut impl FnMut() -> bool,
        ) -> Result<(), String> {
            result.validate_with_stop(game, stop)
        }
    };
}

/// The Chu and Tenjiku cores have the same API; one impl serves both.
macro_rules! large_shogi_game {
    ($game:ident, $core:ident, $glue:ident, $result_variant:ident, $scope:ident,
     profile: |$r:ident| $profile:expr, game_profile: |$g:ident| $game_profile:expr,
     new_game: |$p:ident| $new_game:expr) => {
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

            fn native_side(side: Self::Side) -> NativeSide {
                NativeSide::$result_variant(side)
            }
            canonical_record!($result_variant);
            fn new_record($p: RulesProfile) -> Result<Self::Record, String> {
                let game = $new_game;
                mshogi_core::$core::GameRecord::from_game(&game).map_err(|e| e.to_string())
            }
            fn parse_record(
                text: &str,
                stop: &mut impl FnMut() -> bool,
            ) -> Result<(Self::Record, Self::Game), RecordFailure> {
                mshogi_core::$core::GameRecord::from_json_with_game(text, stop)
                    .map_err(RecordFailure::from)
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
                record.to_game_with_stop(stop).map_err(RecordFailure::from)
            }
            fn from_game(game: &Self::Game) -> Result<Self::Record, RecordFailure> {
                mshogi_core::$core::GameRecord::from_game(game).map_err(RecordFailure::from)
            }
            fn record_prefix(
                game: &Self::Game,
                count: usize,
                stop: &mut impl FnMut() -> bool,
            ) -> Result<Self::Record, RecordFailure> {
                game.record_prefix_with_stop(count, stop)
                    .map_err(RecordFailure::from)
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
            fn resign() -> Self::Command {
                mshogi_core::$core::GameCommand::Resign
            }
            engine_glue!($game);
        }
        mod $glue {
            use super::*;
            use mshogi_core::$core as c;
            pub(super) fn native_failure(e: c::GameError) -> NativeFailure {
                match e {
                    c::GameError::Stopped => NativeFailure::Stopped,
                    c::GameError::Capacity(r) => NativeFailure::Capacity(
                        match r {
                            c::Resource::EncodedBytes => NativeResource::EncodedBytes,
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

large_shogi_game!(Chu, chu, chu_glue, Chu, Chu,
    profile: |_r| RulesProfile::ChuJcsa2019V1, game_profile: |_g| RulesProfile::ChuJcsa2019V1,
    new_game: |_p| mshogi_core::chu::Game::standard());
large_shogi_game!(Tenjiku, tenjiku, tenjiku_glue, Tenjiku, Tenjiku,
profile: |r| RulesProfile::from(r.profile()), game_profile: |g| RulesProfile::from(g.profile()),
new_game: |p| mshogi_core::tenjiku::Game::standard(
    p.tenjiku().ok_or("Tenjiku requires an explicit supported rules profile.")?,
));

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

    fn native_side(side: Self::Side) -> NativeSide {
        NativeSide::Tori(side)
    }
    canonical_record!(Tori);
    fn new_record(_profile: RulesProfile) -> Result<Self::Record, String> {
        mshogi_core::tori::GameRecord::from_game(&mshogi_core::tori::Game::standard())
            .map_err(|e| e.to_string())
    }
    fn parse_record(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(Self::Record, Self::Game), RecordFailure> {
        mshogi_core::tori::GameRecord::from_json_with_game(text, stop).map_err(RecordFailure::from)
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
        record.to_game_with_stop(stop).map_err(RecordFailure::from)
    }
    fn from_game(game: &Self::Game) -> Result<Self::Record, RecordFailure> {
        mshogi_core::tori::GameRecord::from_game(game).map_err(RecordFailure::from)
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
        game.record_prefix(count).map_err(RecordFailure::from)
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
    fn resign() -> Self::Command {
        mshogi_core::tori::GameCommand::Resign
    }
    engine_glue!(Tori);
}
mod tori_glue {
    use super::*;
    use mshogi_core::tori as t;
    pub(super) fn native_failure(e: t::GameError) -> NativeFailure {
        match e {
            t::GameError::Capacity(r) => NativeFailure::Capacity(
                match r {
                    t::Resource::EncodedBytes => NativeResource::EncodedBytes,
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
    // Analysis stays quick by default; play uses the full engine budget.
    const ANALYSIS_DEFAULTS: AnalysisOperation = AnalysisOperation {
        depth: 1,
        nodes: 1000,
    };

    fn native_side(side: Self::Side) -> NativeSide {
        NativeSide::Shogi(side)
    }
    canonical_record!(Shogi);
    fn new_record(_profile: RulesProfile) -> Result<Self::Record, String> {
        Ok(mshogi_core::shogi::GameRecord::from_game(
            &mshogi_core::shogi::Game::new(),
        ))
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
    fn resign() -> Self::Command {
        mshogi_core::shogi::GameCommand::Resign
    }
    engine_glue!(Shogi);
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

    fn native_side(side: Self::Side) -> NativeSide {
        NativeSide::Xiangqi(side)
    }
    canonical_record!(Xiangqi);
    fn new_record(_profile: RulesProfile) -> Result<Self::Record, String> {
        use mshogi_core::xiangqi as x;
        let game = x::Game::standard(x::ClaimPolicy::Explicit).map_err(|e| e.to_string())?;
        x::GameRecord::from_game(&game).map_err(|e| e.to_string())
    }
    fn parse_record(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(Self::Record, Self::Game), RecordFailure> {
        if stop() {
            return Err(RecordFailure::Stopped);
        }
        let parsed = mshogi_core::xiangqi::GameRecord::from_json_with_game(text)
            .map_err(|e| RecordFailure::Invalid(e.to_string()))?;
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
        let game = record
            .to_game()
            .map_err(|e| RecordFailure::Invalid(e.to_string()))?;
        if stop() {
            return Err(RecordFailure::Stopped);
        }
        Ok(game)
    }
    fn from_game(game: &Self::Game) -> Result<Self::Record, RecordFailure> {
        mshogi_core::xiangqi::GameRecord::from_game(game)
            .map_err(|e| RecordFailure::Invalid(e.to_string()))
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
    fn resign() -> Self::Command {
        mshogi_core::xiangqi::GameCommand::Resign
    }
    engine_glue!(Xiangqi);
}

/// Work limits of an analysis request, checked against the game's bounds.
pub(crate) fn analysis_limits<G: NativeGame>(
    operation: AnalysisOperation,
) -> Result<WorkLimits, String> {
    let limits = WorkLimits {
        time_ms: None,
        depth: operation.depth,
        nodes: operation.nodes,
    };
    if G::ANALYSIS_BUDGETS {
        operation.validate()?;
    } else {
        validate_limits::<G>(limits)?;
    }
    Ok(limits)
}

/// Check engine work limits against the game's bounds.
pub(crate) fn validate_limits<G: NativeGame>(limits: WorkLimits) -> Result<(), String> {
    if limits.depth == 0
        || limits.depth > G::MAX_DEPTH
        || limits.nodes == 0
        || limits.nodes > G::MAX_NODES
    {
        return Err(format!(
            "Unsupported {} work limits. Use a positive bounded native depth and visit limit.",
            G::NAME
        ));
    }
    Ok(())
}

impl From<mshogi_core::native::RecordError> for RecordFailure {
    fn from(e: mshogi_core::native::RecordError) -> Self {
        use mshogi_core::native::RecordError;
        match e {
            RecordError::Stopped => Self::Stopped,
            RecordError::Capacity => Self::Capacity(e.to_string()),
            e => Self::Invalid(e.to_string()),
        }
    }
}
