//! The five games on the shared search core. Each impl holds only the types,
//! calls and constants of its game; the shared wiring is in the macros below.
use crate::{
    EngineGame, EngineResult, EngineTelemetry, ExecutionResult, NoPending, StopReason, WorkLimits,
    adapter::Decision,
};
use mshogi_core::{
    chu, native::CanonicalRecord, native::GameKind, native::NativeSide, shogi, tenjiku, tori,
    xiangqi,
};
use mshogi_search::alphabeta::{Searcher, timing::TimeBudget};
use serde::{Deserialize, Serialize};

/// Wiring that only names the game's enum variant and search module.
macro_rules! wiring {
    ($variant:ident, $core:ident, $position:ident) => {
        const NAME: &'static str = stringify!($variant);
        const KIND: GameKind = GameKind::$variant;
        const EVALUATOR_ID: &'static str = mshogi_search::$core::EVALUATOR_ID;
        const SEARCH_ID: &'static str = mshogi_search::$core::SEARCH_ID;
        const WIN_SCORE: i32 = mshogi_search::$core::WIN_SCORE;
        const MAX_DEPTH: u16 = mshogi_search::$core::MAX_DEPTH;
        const MAX_NODES: u64 = mshogi_search::$core::MAX_NODES;
        const MAX_LINE: usize = mshogi_search::$core::MAX_LINE;

        type Side = $core::Side;
        type Conclusion = $core::Conclusion;
        type Game = $core::Game;
        type Record = $core::GameRecord;
        type Command = $core::GameCommand;
        type Position = mshogi_search::$core::position::$position;

        fn record(record: &CanonicalRecord) -> Option<&Self::Record> {
            match record {
                CanonicalRecord::$variant(record) => Some(record),
                _ => None,
            }
        }
        fn result(result: &ExecutionResult) -> Option<&EngineResult<Self>> {
            match result {
                ExecutionResult::$variant { result } => Some(result),
                _ => None,
            }
        }
        fn wrap(result: EngineResult<Self>) -> ExecutionResult {
            ExecutionResult::$variant { result }
        }
        fn native_side(side: Self::Side) -> NativeSide {
            NativeSide::$variant(side)
        }
        fn record_side(record: &Self::Record) -> Self::Side {
            record.side_to_move()
        }
        fn record_conclusion(record: &Self::Record) -> Option<Self::Conclusion> {
            record.conclusion()
        }
        fn terminal_score(conclusion: Self::Conclusion, side: Self::Side) -> i32 {
            mshogi_search::$core::terminal_score(conclusion, side, 0)
        }
        fn to_game(record: &Self::Record) -> Result<Self::Game, String> {
            record.to_game().map_err(|e| e.to_string())
        }
        fn side_to_move(game: &Self::Game) -> Self::Side {
            game.position().side_to_move()
        }
        fn new_engine() -> Searcher<Self::Position> {
            mshogi_search::$core::new_engine()
        }
    };
}

/// Search call of a game whose search result has no pending adjudication.
macro_rules! search {
    ($core:ident) => {
        fn search(
            game: &Self::Game,
            limits: WorkLimits,
            time: Option<TimeBudget>,
            engine: &mut Searcher<Self::Position>,
        ) -> Result<EngineResult<Self>, String> {
            use mshogi_search::$core as s;
            let r = s::search_with_state(
                game,
                s::Limits {
                    depth: limits.depth,
                    nodes: limits.nodes,
                },
                time,
                engine,
                || false,
            )
            .map_err(|e| e.to_string())?;
            let telemetry = EngineTelemetry::current(
                r.perspective,
                r.score,
                r.completed_depth,
                r.nodes,
                r.principal_variation.into_iter().map(Into::into).collect(),
                r.status.into(),
            );
            let decision = r.decision.map(|d| match d {
                s::Decision::Choose(choice) => Decision::Choose(choice.into()),
                s::Decision::Finished(conclusion) => Decision::Finished(conclusion),
            });
            Ok(EngineResult::searched(decision, telemetry))
        }
    };
}

/// Play limits of an untimed search.
const fn untimed(depth: u16, nodes: u64) -> WorkLimits {
    WorkLimits {
        depth,
        nodes,
        time_ms: None,
    }
}

/// The large-board games: a choice is one board action, the terminal
/// distance counts board actions and play is untimed.
macro_rules! board_action_game {
    ($core:ident) => {
        const DISTANCE_UNIT: [&'static str; 2] = ["board action", "board actions"];
        const PLAY_LIMITS: WorkLimits = untimed(
            mshogi_search::$core::DEFAULT_LIMITS.depth,
            mshogi_search::$core::DEFAULT_LIMITS.nodes,
        );
        type Choice = $core::Action;
        type Pending = NoPending;
        fn fork(game: &Self::Game) -> Result<Self::Game, String> {
            game.fork().map_err(|e| e.to_string())
        }
        fn command(choice: &$core::Action, turn: $core::Side) -> ($core::Side, $core::GameCommand) {
            let action = choice.clone();
            (turn, $core::GameCommand::Play { action })
        }
    };
}

/// Chu Shogi.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Chu;
impl EngineGame for Chu {
    wiring!(Chu, chu, ChuPosition);
    search!(chu);
    const SCORE_UNIT: &'static str = "Pawns";
    board_action_game!(chu);
    fn extends(current: &Self::Record, _game: &Self::Game, earlier: &Self::Record) -> bool {
        current.has_prefix(earlier)
    }
    fn submit(
        game: &mut chu::Game,
        actor: chu::Side,
        command: chu::GameCommand,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        let expected = game.context();
        let request = chu::GameRequest {
            expected,
            actor,
            command,
        };
        game.submit_with_stop(request, stop)
            .map(drop)
            .map_err(|e| e.to_string())
    }
}

/// Tenjiku Shogi, under the profile of its record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tenjiku;
impl EngineGame for Tenjiku {
    wiring!(Tenjiku, tenjiku, TenjikuPosition);
    search!(tenjiku);
    const SCORE_UNIT: &'static str = "Pawns";
    board_action_game!(tenjiku);
    fn extends(_current: &Self::Record, game: &Self::Game, earlier: &Self::Record) -> bool {
        game.record_prefix(earlier.event_count())
            .is_ok_and(|prefix| prefix == *earlier)
    }
    fn submit(
        game: &mut tenjiku::Game,
        actor: tenjiku::Side,
        command: tenjiku::GameCommand,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        let expected = game.context();
        let request = tenjiku::GameRequest {
            expected,
            actor,
            command,
        };
        game.submit_with_stop(request, stop)
            .map(drop)
            .map_err(|e| e.to_string())
    }
}

/// Tori Shogi.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tori;
impl EngineGame for Tori {
    wiring!(Tori, tori, ToriPosition);
    search!(tori);
    const SCORE_UNIT: &'static str = "Swallows";
    board_action_game!(tori);
    fn extends(current: &Self::Record, _game: &Self::Game, earlier: &Self::Record) -> bool {
        current.has_prefix(earlier)
    }
    /// A Tori command is short; the caller checks the stop between commands.
    fn submit(
        game: &mut tori::Game,
        actor: tori::Side,
        command: tori::GameCommand,
        _stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        let expected = game.context();
        let request = tori::GameRequest {
            expected,
            actor,
            command,
        };
        game.submit(request).map(drop).map_err(|e| e.to_string())
    }
}

/// Standard Shogi choice: a board move or a formal entering-king declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ShogiChoice {
    /// Complete move, drop or promotion.
    Move {
        /// Board action; never a resign or declaration token.
        action: shogi::Action,
    },
    /// Entering-king declaration; not a board action.
    Declare {
        /// Declaring player.
        actor: shogi::Side,
    },
}
impl From<mshogi_search::shogi::Choice> for ShogiChoice {
    fn from(choice: mshogi_search::shogi::Choice) -> Self {
        match choice {
            mshogi_search::shogi::Choice::Move(action) => Self::Move { action },
            mshogi_search::shogi::Choice::Declare { actor } => Self::Declare { actor },
        }
    }
}

/// Standard Shogi; play is time-primary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Shogi;
impl EngineGame for Shogi {
    wiring!(Shogi, shogi, ShogiPosition);
    search!(shogi);
    const SCORE_UNIT: &'static str = "pawns";
    const DISTANCE_UNIT: [&'static str; 2] = [
        "command edge (including declarations)",
        "command edges (including declarations)",
    ];
    /// Depth and nodes stay finite safety ceilings.
    const PLAY_LIMITS: WorkLimits = WorkLimits {
        time_ms: Some(1_000),
        depth: Self::MAX_DEPTH,
        nodes: Self::MAX_NODES,
    };
    type Choice = ShogiChoice;
    type Pending = NoPending;
    fn extends(current: &Self::Record, _game: &Self::Game, earlier: &Self::Record) -> bool {
        current.has_prefix(earlier)
    }
    fn fork(game: &Self::Game) -> Result<Self::Game, String> {
        Ok(game.clone())
    }
    fn command(choice: &ShogiChoice, turn: shogi::Side) -> (shogi::Side, shogi::GameCommand) {
        match *choice {
            ShogiChoice::Move { action } => (turn, shogi::GameCommand::Play(action)),
            ShogiChoice::Declare { actor } => (actor, shogi::GameCommand::DeclareEnteringKing),
        }
    }
    fn submit(
        game: &mut shogi::Game,
        actor: shogi::Side,
        command: shogi::GameCommand,
        _stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        let expected = game.context();
        let request = shogi::GameRequest {
            expected,
            actor,
            command,
        };
        game.submit(request)
            .map(drop)
            .map_err(|e| format!("Native shogi command rejected: {e:?}"))
    }
    /// The search declares before it searches: one iteration of two visits.
    fn command_shape(choice: &ShogiChoice, t: &EngineTelemetry<Self>) -> Option<bool> {
        let ShogiChoice::Declare { actor } = *choice else {
            return None;
        };
        Some(
            actor == t.perspective
                && t.score == Some(Self::WIN_SCORE - 1)
                && t.principal_variation.len() == 1
                && t.completed_depth == 1
                && t.nodes == 2
                && t.status == StopReason::Complete,
        )
    }
}

/// Xiangqi choice: a board move or a formal natural-count claim.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum XiangqiChoice {
    /// Complete board action.
    Move {
        /// Origin and destination.
        action: xiangqi::Action,
    },
    /// Count claim; not a board action or a concluded draw.
    Claim {
        /// Claimant.
        actor: xiangqi::Side,
    },
}
impl From<mshogi_search::xiangqi::Choice> for XiangqiChoice {
    fn from(value: mshogi_search::xiangqi::Choice) -> Self {
        match value {
            mshogi_search::xiangqi::Choice::Move(action) => Self::Move { action },
            mshogi_search::xiangqi::Choice::Claim { actor } => Self::Claim { actor },
        }
    }
}

/// Xiangqi; a required history adjudication stops the search unscored.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Xiangqi;
impl EngineGame for Xiangqi {
    wiring!(Xiangqi, xiangqi, XiangqiPosition);
    const SCORE_UNIT: &'static str = "soldiers";
    const DISTANCE_UNIT: [&'static str; 2] = ["native command edge", "native command edges"];
    const PLAY_LIMITS: WorkLimits = untimed(
        mshogi_search::xiangqi::DEFAULT_LIMITS.depth,
        mshogi_search::xiangqi::DEFAULT_LIMITS.nodes,
    );
    type Choice = XiangqiChoice;
    type Pending = xiangqi::PendingAdjudication;
    /// History changes only at command boundaries (MSH-226).
    fn extends(_current: &Self::Record, game: &Self::Game, earlier: &Self::Record) -> bool {
        game.committed_prefix(earlier.event_count())
            .ok()
            .and_then(|prefix| xiangqi::GameRecord::from_game(&prefix).ok())
            .is_some_and(|prefix| prefix == *earlier)
    }
    fn fork(game: &Self::Game) -> Result<Self::Game, String> {
        game.fork().map_err(|e| e.to_string())
    }
    fn command(
        choice: &XiangqiChoice,
        turn: xiangqi::Side,
    ) -> (xiangqi::Side, xiangqi::GameCommand) {
        match *choice {
            XiangqiChoice::Move { action } => (turn, xiangqi::GameCommand::Play { action }),
            XiangqiChoice::Claim { actor } => (actor, xiangqi::GameCommand::ClaimNaturalCount),
        }
    }
    fn submit(
        game: &mut xiangqi::Game,
        actor: xiangqi::Side,
        command: xiangqi::GameCommand,
        _stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        let expected = game.context();
        let request = xiangqi::GameRequest {
            expected,
            actor,
            command,
        };
        match game.submit(request).map_err(|e| e.to_string())?.result {
            xiangqi::CommandResult::Accepted => Ok(()),
            xiangqi::CommandResult::RejectedNaturalCount => {
                Err("Xiangqi natural count claim was rejected".into())
            }
        }
    }
    fn search(
        game: &xiangqi::Game,
        limits: WorkLimits,
        time: Option<TimeBudget>,
        engine: &mut Searcher<Self::Position>,
    ) -> Result<EngineResult<Self>, String> {
        use mshogi_search::xiangqi as s;
        let limits = s::Limits {
            depth: limits.depth,
            nodes: limits.nodes,
        };
        let r = s::search_with_state(game, limits, time, engine, || false)
            .map_err(|e| e.to_string())?;
        let telemetry = EngineTelemetry::current(
            r.perspective,
            r.score,
            r.completed_depth,
            r.nodes,
            r.principal_variation.into_iter().map(Into::into).collect(),
            r.status.into(),
        );
        let decision = match (r.unresolved, r.decision) {
            (Some(pending), _) => Some(Decision::Unresolved(pending)),
            (None, Some(s::Decision::Choose(choice))) => Some(Decision::Choose(choice.into())),
            (None, Some(s::Decision::Finished(conclusion))) => Some(Decision::Finished(conclusion)),
            (None, None) => None,
        };
        Ok(EngineResult::searched(decision, telemetry))
    }
    /// The search claims after a completed iteration that scores no better
    /// than a draw.
    fn command_shape(choice: &XiangqiChoice, t: &EngineTelemetry<Self>) -> Option<bool> {
        let XiangqiChoice::Claim { actor } = *choice else {
            return None;
        };
        Some(actor == t.perspective && t.score == Some(0) && t.principal_variation.len() == 1)
    }
}

/// Chu result.
pub type ChuResult = EngineResult<Chu>;
/// Chu telemetry.
pub type ChuTelemetry = EngineTelemetry<Chu>;
/// Tenjiku result.
pub type TenjikuResult = EngineResult<Tenjiku>;
/// Tenjiku telemetry.
pub type TenjikuTelemetry = EngineTelemetry<Tenjiku>;
/// Tori result.
pub type ToriResult = EngineResult<Tori>;
/// Tori telemetry.
pub type ToriTelemetry = EngineTelemetry<Tori>;
/// Shogi result.
pub type ShogiResult = EngineResult<Shogi>;
/// Shogi telemetry.
pub type ShogiTelemetry = EngineTelemetry<Shogi>;
/// Xiangqi result.
pub type XiangqiResult = EngineResult<Xiangqi>;
/// Xiangqi telemetry.
pub type XiangqiTelemetry = EngineTelemetry<Xiangqi>;

impl From<mshogi_search::xiangqi::Status> for StopReason {
    fn from(status: mshogi_search::xiangqi::Status) -> Self {
        match status {
            mshogi_search::xiangqi::Status::Complete => Self::Complete,
            mshogi_search::xiangqi::Status::NodeLimit => Self::NodeLimit,
            mshogi_search::xiangqi::Status::Stopped => Self::Stopped,
            mshogi_search::xiangqi::Status::Unresolved => Self::Unresolved,
        }
    }
}
