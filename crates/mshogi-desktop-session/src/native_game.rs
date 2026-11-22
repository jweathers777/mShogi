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
    /// Measured aggregate replay allowance of one document.
    const MAX_DOCUMENT_WORK: u64;
    /// Native rules: an engine Stop is admitted after a conclusion.
    const STOP_AFTER_CONCLUSION: bool;
    /// Recorded saved-format compatibility: a saved control must equal its
    /// canonical form. False keeps older files with defaulted fields loadable.
    const STRICT_CONTROL_FIELDS: bool;
    /// Replay-work charge of a saved browse selection, in prefix reconstructions.
    /// Chu keeps its recorded 2 so that saved Chu documents near the work limit
    /// still load; Tenjiku charges validation plus runtime construction and replay.
    const SELECTION_WORK_FACTOR: u64;
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
    /// Immutable rules profile of a record.
    fn profile(record: &Self::Record) -> RulesProfile;
    /// Immutable rules profile of a game.
    fn game_profile(game: &Self::Game) -> RulesProfile;
    /// Deterministic work to reconstruct a record.
    fn reconstruction_work(record: &Self::Record) -> u64;
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
    ) -> Result<Self::Undo, NativeFailure>;
    /// Reverse one submitted command.
    fn undo(game: &mut Self::Game, undo: Self::Undo) -> Result<(), NativeFailure>;
    /// Native replay work of a game so far.
    fn replay_work(game: &Self::Game) -> u64;
    /// Events of a game.
    fn events(game: &Self::Game) -> usize;
    /// Event index where the last command began: one command can produce
    /// several events, and history changes only at command boundaries.
    fn last_command_start(game: &Self::Game) -> Option<usize> {
        Self::events(game).checked_sub(1)
    }
    /// Validate saved evidence against the game at its journal position; the
    /// native work it used.
    fn validate_evidence(
        evidence: &Self::Evidence,
        game: &Self::Game,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<u64, String>;
    /// Side to move.
    fn side_to_move(game: &Self::Game) -> Self::Side;
    /// The game has a native conclusion.
    fn concluded(game: &Self::Game) -> bool;
    /// An engine Stop is already recorded in the history snapshot.
    fn stop_recorded(game: &Self::Game) -> bool;

    /// Resign command.
    fn resign() -> Self::Command;
    /// Engine-failure Stop command, if the game records one.
    fn engine_failure_stop() -> Option<Self::Command>;

    /// This game's engine result, if the execution result is one.
    fn engine_result(result: &ExecutionResult) -> Option<&Self::EngineResult>;
    /// The complete command, with its actor, that an engine result proposes.
    fn engine_command(
        result: &Self::EngineResult,
        game: &Self::Game,
    ) -> Option<(Self::Side, Self::Command)>;
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
     selection_work: $selection:expr,
     profile: |$r:ident| $profile:expr, game_profile: |$g:ident| $game_profile:expr) => {
        /// Marker of the native game behind the generic document and session.
        pub struct $game;
        impl NativeGame for $game {
            const NAME: &'static str = stringify!($result_variant);
            const KIND: GameKind = GameKind::$result_variant;
            const SCOPE: ResourceScope = ResourceScope::$scope;
            const MAX_DOCUMENT_WORK: u64 = $work;
            const STOP_AFTER_CONCLUSION: bool = $stop;
            const STRICT_CONTROL_FIELDS: bool = $strict;
            const SELECTION_WORK_FACTOR: u64 = $selection;
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
            type Settings = crate::game_document::GameSettings<Self::Side>;
            type Evidence = NoEvidence;
            fn validate_evidence(
                evidence: &NoEvidence,
                _game: &Self::Game,
                _stop: &mut impl FnMut() -> bool,
            ) -> Result<u64, String> {
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
            fn profile($r: &Self::Record) -> RulesProfile {
                $profile
            }
            fn game_profile($g: &Self::Game) -> RulesProfile {
                $game_profile
            }
            fn reconstruction_work(record: &Self::Record) -> u64 {
                record.reconstruction_work()
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
            fn undo(game: &mut Self::Game, mut undo: Self::Undo) -> Result<(), NativeFailure> {
                game.undo(&mut undo)
                    .map(|_| ())
                    .map_err($glue::native_failure)
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
            fn resign() -> Self::Command {
                mshogi_core::$core::GameCommand::Resign
            }
            fn engine_failure_stop() -> Option<Self::Command> {
                Some(mshogi_core::$core::GameCommand::Stop {
                    reason: mshogi_core::$core::StopReason::EngineFailure,
                })
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
    selection_work: 2,
    profile: |_r| RulesProfile::ChuJcsa2019V1, game_profile: |_g| RulesProfile::ChuJcsa2019V1);
large_shogi_game!(Tenjiku, tenjiku, tenjiku_glue, TenjikuResult, Tenjiku, Tenjiku,
    // Measured: see `tenjiku_document::MAX_TENJIKU_DOCUMENT_WORK`.
    work: 160_000_000, stop_after_conclusion: true, strict: true,
    selection_work: 3,
    profile: |r| RulesProfile::from(r.profile()), game_profile: |g| RulesProfile::from(g.profile()));

/// Tori has no native replay-work count; the desktop models it.
pub mod tori_work {
    use mshogi_core::tori as t;
    /// A native root admission or serialization costs work even without events.
    pub const ROOT_WORK: u64 = 128;
    /// Modelled work of one native board action, in units of about one
    /// microsecond of release replay (macOS arm64). Since MSH-208 (Tori
    /// Repetition Rule) a full-length record replays in about 120-140 ms,
    /// about 137 us per ply at 1,022 plies, and the cost per ply rises with
    /// length. 192 is about 1.4 times the measured cost at full length.
    pub const PLAY_WORK: u64 = 192;
    /// Upper bound of one record's modelled replay work (all events are plays).
    pub const MAX_RECORD_WORK: u64 = ROOT_WORK + PLAY_WORK * t::MAX_GAME_EVENTS as u64;
    /// Modelled work of a history of `events` events, `plays` of them board actions.
    pub fn of(events: usize, plays: usize) -> u64 {
        ROOT_WORK + PLAY_WORK * plays as u64 + events.saturating_sub(plays) as u64
    }
}

/// Marker of the native Tori game behind the generic document and session.
pub struct Tori;
impl NativeGame for Tori {
    const NAME: &'static str = "Tori";
    const KIND: GameKind = GameKind::Tori;
    const SCOPE: ResourceScope = ResourceScope::Tori;
    const MAX_DOCUMENT_WORK: u64 = TORI_DOCUMENT_WORK;
    const STOP_AFTER_CONCLUSION: bool = false;
    const STRICT_CONTROL_FIELDS: bool = true;
    const SELECTION_WORK_FACTOR: u64 = 3;
    const MAX_RECORD_BYTES: u64 = mshogi_core::tori::MAX_GAME_RECORD_BYTES as u64;
    const MAX_EVENTS: u64 = mshogi_core::tori::MAX_GAME_EVENTS as u64;
    const MAX_REPLAY_WORK: u64 = tori_work::MAX_RECORD_WORK;
    const SIDES: [Self::Side; 2] = [
        mshogi_core::tori::Side::Sente,
        mshogi_core::tori::Side::Gote,
    ];

    type Side = mshogi_core::tori::Side;
    type Command = mshogi_core::tori::GameCommand;
    type Action = mshogi_core::tori::Action;
    type Game = mshogi_core::tori::Game;
    type Undo = mshogi_core::tori::GameUndo;
    type Record = mshogi_core::tori::GameRecord;
    type EngineResult = mshogi_engine::ToriResult;
    type Settings = crate::game_document::GameSettings<Self::Side>;
    type Evidence = NoEvidence;
    fn validate_evidence(
        evidence: &NoEvidence,
        _game: &Self::Game,
        _stop: &mut impl FnMut() -> bool,
    ) -> Result<u64, String> {
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
    fn profile(_record: &Self::Record) -> RulesProfile {
        RulesProfile::ToriModernV1
    }
    fn game_profile(_game: &Self::Game) -> RulesProfile {
        RulesProfile::ToriModernV1
    }
    fn reconstruction_work(record: &Self::Record) -> u64 {
        tori_work::of(record.event_count(), record.known_board_actions())
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
    ) -> Result<Self::Undo, NativeFailure> {
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
    }
    fn undo(game: &mut Self::Game, mut undo: Self::Undo) -> Result<(), NativeFailure> {
        game.undo(&mut undo)
            .map(|_| ())
            .map_err(tori_glue::native_failure)
    }
    fn replay_work(game: &Self::Game) -> u64 {
        tori_work::of(game.events().len(), game.snapshot().board_ply)
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
    fn engine_failure_stop() -> Option<Self::Command> {
        Some(mshogi_core::tori::GameCommand::Stop {
            reason: mshogi_core::tori::StopReason::EngineFailure,
        })
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
    ) -> Result<u64, String> {
        if stop() {
            return Err("Tori observation validation stopped".into());
        }
        // Validation copies the game without replay and plays the line.
        result.validate(game)?;
        Ok(tori_work::PLAY_WORK * result.telemetry().principal_variation.len() as u64)
    }
}
/// Whole-document validation allowance of Tori, in modelled replay units.
/// Measured (MSH-223, release, macOS arm64, test
/// `measure_full_length_records_against_the_work_limit`): one 1,022-event
/// record is 196,352 units and replays in 158 ms (about 0.8 us per unit). The
/// largest valid document (4,088 retained events, two retired branches and a
/// browse selection) charges 2,551,620 units and validates in 1.2 s. A forged
/// file of full-length records is refused at this allowance after about 3.7 s,
/// well within the file worker's 15 s deadline.
const TORI_DOCUMENT_WORK: u64 = 4_194_304;
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

/// Shogi has no native replay-work count; the desktop models it like Tori.
pub mod shogi_work {
    use mshogi_core::shogi as s;
    /// A native root admission or serialization costs work even without events.
    pub const ROOT_WORK: u64 = 128;
    /// Modelled work of one native command. Measured (MSH-224, release, macOS
    /// arm64, test `measure_full_length_shogi_replay`): a 319-command record
    /// replays in 46 ms, about 144 us per command; 200 is about 1.4 times that,
    /// the same margin as Tori.
    pub const PLAY_WORK: u64 = 200;
    /// Upper bound of one record's modelled replay work.
    pub const MAX_RECORD_WORK: u64 = ROOT_WORK + PLAY_WORK * s::MAX_RECORD_EVENTS as u64;
    /// Modelled work of a history of `events` commands.
    pub fn of(events: usize) -> u64 {
        ROOT_WORK + PLAY_WORK * events as u64
    }
}

/// Marker of the native Shogi game behind the generic document and session.
pub struct Shogi;
impl NativeGame for Shogi {
    const NAME: &'static str = "Shogi";
    const KIND: GameKind = GameKind::Shogi;
    const SCOPE: ResourceScope = ResourceScope::Shogi;
    const MAX_DOCUMENT_WORK: u64 = SHOGI_DOCUMENT_WORK;
    const STOP_AFTER_CONCLUSION: bool = false;
    const STRICT_CONTROL_FIELDS: bool = true;
    const SELECTION_WORK_FACTOR: u64 = 3;
    const MAX_RECORD_BYTES: u64 = mshogi_core::shogi::MAX_RECORD_BYTES as u64;
    const MAX_EVENTS: u64 = mshogi_core::shogi::MAX_RECORD_EVENTS as u64;
    const MAX_REPLAY_WORK: u64 = shogi_work::MAX_RECORD_WORK;
    const SIDES: [Self::Side; 2] = [
        mshogi_core::shogi::Side::Sente,
        mshogi_core::shogi::Side::Gote,
    ];

    type Side = mshogi_core::shogi::Side;
    type Command = mshogi_core::shogi::GameCommand;
    type Action = mshogi_core::shogi::Action;
    type Game = mshogi_core::shogi::Game;
    type Undo = mshogi_core::shogi::GameUndo;
    type Record = mshogi_core::shogi::GameRecord;
    type EngineResult = mshogi_engine::ShogiResult;
    type Settings = crate::shared::archive::ShogiSettings;
    type Evidence = NoEvidence;
    fn validate_evidence(
        evidence: &NoEvidence,
        _game: &Self::Game,
        _stop: &mut impl FnMut() -> bool,
    ) -> Result<u64, String> {
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
    fn profile(_record: &Self::Record) -> RulesProfile {
        RulesProfile::ShogiCsa36V1
    }
    fn game_profile(_game: &Self::Game) -> RulesProfile {
        RulesProfile::ShogiCsa36V1
    }
    fn reconstruction_work(record: &Self::Record) -> u64 {
        shogi_work::of(record.event_count())
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
    ) -> Result<Self::Undo, NativeFailure> {
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
    }
    fn undo(game: &mut Self::Game, undo: Self::Undo) -> Result<(), NativeFailure> {
        game.undo(undo)
            .map_err(|e| NativeFailure::Invalid(format!("{e:?}")))
    }
    fn replay_work(game: &Self::Game) -> u64 {
        shogi_work::of(game.events().len())
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
    fn engine_failure_stop() -> Option<Self::Command> {
        None
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
    ) -> Result<u64, String> {
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
        Ok(shogi_work::PLAY_WORK * line.len() as u64)
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
/// Whole-document validation allowance of Shogi, in modelled replay units.
/// Measured (MSH-224, release, macOS arm64, test
/// `measure_full_length_shogi_replay`): the largest valid document (3,828
/// retained events: full-length records, retired branches and a selection)
/// charges 2,362,756 units and validates in 1.6 s; a forged 16 MB file of
/// full-length records is refused at this allowance after 3.2 s.
const SHOGI_DOCUMENT_WORK: u64 = 4_194_304;

#[cfg(test)]
mod measurements {
    use mshogi_core::shogi as s;
    /// MSH-224 measurement: replay of a full-length Shogi record. Run with
    /// `--ignored --nocapture` in release.
    #[test]
    #[ignore]
    fn measure_full_length_shogi_replay() {
        let mut g = s::Game::new();
        while g.conclusion().is_none() {
            let actions = g.position().legal_actions();
            let next = actions.iter().copied().find(|a| {
                let mut probe = g.clone();
                probe
                    .submit(s::GameRequest {
                        expected: probe.context(),
                        actor: probe.position().side_to_move(),
                        command: s::GameCommand::Play(*a),
                    })
                    .is_ok()
                    && (probe.conclusion().is_none() || probe.events().len() >= 319)
            });
            let Some(a) = next else { break };
            g.submit(s::GameRequest {
                expected: g.context(),
                actor: g.position().side_to_move(),
                command: s::GameCommand::Play(a),
            })
            .unwrap();
        }
        let record = s::GameRecord::from_game(&g);
        let started = std::time::Instant::now();
        for _ in 0..5 {
            record.to_game().unwrap();
        }
        let ms = started.elapsed().as_secs_f64() * 1e3 / 5.0;
        println!(
            "MSH224 shogi events={} replay_ms={ms:.1} us_per_event={:.1}",
            record.event_count(),
            ms * 1e3 / record.event_count() as f64
        );
        // Largest valid document: retired full-length branches up to the
        // retained-event limit, and a browse selection.
        use crate::game_document::{GameControl, RewindKind, ValidatedGameDocument};
        let last = g.events().len() - 1;
        let command = g.events()[last].command;
        let actor = g.events()[last].actor;
        let mut doc =
            ValidatedGameDocument::<super::Shogi>::new(record.clone(), s::Side::Sente).unwrap();
        loop {
            let next = doc.with_control(
                GameControl::Rewind {
                    event_index: last,
                    reason: RewindKind::Undo,
                },
                &mut || false,
            );
            let Ok(rewound) = next else { break };
            let Ok(replayed) =
                rewound.with_control(GameControl::Native { actor, command }, &mut || false)
            else {
                break;
            };
            doc = replayed;
        }
        let doc = doc.with_selection(Some(last), &mut || false).unwrap();
        let started = std::time::Instant::now();
        ValidatedGameDocument::<super::Shogi>::decode(doc.bytes(), &mut || false).unwrap();
        println!(
            "MSH224 shogi largest_valid_ms={:.1} usage={:?}",
            started.elapsed().as_secs_f64() * 1e3,
            doc.usage()
        );
        // Forged: valid full-length retired records up to the byte limit.
        let single = ValidatedGameDocument::<super::Shogi>::new(record, s::Side::Sente).unwrap();
        let per = single.bytes().len() / 2;
        let mut v: serde_json::Value = serde_json::from_slice(single.bytes()).unwrap();
        let current = v["payload"]["current"].clone();
        v["payload"]["retired"] = serde_json::json!(vec![
            serde_json::json!({"control_index": 0, "record": current});
            crate::MAX_ARCHIVE_BYTES / per - 3
        ]);
        let started = std::time::Instant::now();
        let refused = ValidatedGameDocument::<super::Shogi>::decode(
            &serde_json::to_vec(&v).unwrap(),
            &mut || false,
        )
        .unwrap_err();
        println!(
            "MSH224 shogi forged_refusal_ms={:.1} forged={:?}",
            started.elapsed().as_secs_f64() * 1e3,
            refused.resource.map(|r| r.facts()[0].category)
        );
    }
}
