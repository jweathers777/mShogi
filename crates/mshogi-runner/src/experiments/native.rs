//! Experiment orchestration over the existing native owners. No rules live here.
use mshogi_core::{
    chess, chu,
    native::{CanonicalRecord, NativeSide, RulesProfile},
    shogi, tenjiku, tori, xiangqi,
};
use mshogi_engine::{
    ChessResult, ChuResult, ExecutionResult, ShogiChoice, ShogiResult, TenjikuResult, ToriResult,
    XiangqiChoice, XiangqiResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Complete canonical history plus the additional Chess match command ledger.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Root {
    /// Complete authoritative native record, retaining immutable profile/history.
    pub record: CanonicalRecord,
    /// Required only for Chess; an exact native MatchRecord JSON string.
    pub chess_match: Option<String>,
}

/// Concrete authoritative owners, borrowed by the experiment controller.
pub enum NativeState {
    /// Untimed M2 Chess match with its full command ledger.
    Chess(Box<chess::ChessMatch>),
    /// Standard Shogi's native game owner.
    Shogi(Box<shogi::Game>),
    /// Xiangqi's native history/adjudication owner.
    Xiangqi(Box<xiangqi::Game>),
    /// Modern Tori's native history owner.
    Tori(Box<tori::Game>),
    /// JCSA Chu's native complete-action/history owner.
    Chu(Box<chu::Game>),
    /// Tenjiku's native owner under one immutable whole profile.
    Tenjiku(Box<tenjiku::Game>),
}

/// Result of admitting a supervised native engine observation.
#[derive(Debug, Eq, PartialEq)]
pub enum Admission {
    /// Authoritative native owner committed the proposed command.
    Applied,
    /// Observation agrees with an already established native conclusion.
    Finished,
    /// Work ended without a usable completed choice or game result.
    Incomplete(String),
    /// Supervised execution failed without a native game result.
    Failed(String),
}

/// Begin a fresh untimed experiment match around an existing native history.
/// For Chess this explicitly creates a new match; it cannot recover an old
/// match's commands, offers or clocks from its canonical history alone.
pub fn fresh_root(record: CanonicalRecord) -> Result<Root, String> {
    let chess_match = if let CanonicalRecord::Chess(history) = &record {
        if history.adjudication_mode() != chess::AdjudicationMode::M2 {
            return Err("internal experiments do not support Chess Full live replay".into());
        }
        let game = chess::AdjudicatedGame::replay_record(history).map_err(|e| e.to_string())?;
        let mut game =
            chess::ChessMatch::new(game, chess::TimeControl::Untimed).map_err(|e| e.to_string())?;
        start_chess(&mut game)?;
        Some(game.record().to_json())
    } else {
        None
    };
    let root = Root {
        record,
        chess_match,
    };
    NativeState::from_root(&root)?.root()
}

/// The selected baseline starts are untimed; Chess uses explicit AutoClaim M2.
pub fn standard_root(profile: RulesProfile) -> Result<Root, String> {
    let state = match profile {
        RulesProfile::ChessMatchV1 => {
            let mut game = chess::ChessMatch::new(
                chess::AdjudicatedGame::new(chess::ClaimPolicy::AutoClaim),
                chess::TimeControl::Untimed,
            )
            .map_err(|e| e.to_string())?;
            start_chess(&mut game)?;
            NativeState::Chess(Box::new(game))
        }
        RulesProfile::ShogiCsa36V1 => NativeState::Shogi(Box::new(shogi::Game::new())),
        RulesProfile::XiangqiWxf2018V1 => NativeState::Xiangqi(Box::new(
            xiangqi::Game::standard(xiangqi::ClaimPolicy::AutoValidClaims)
                .map_err(|e| e.to_string())?,
        )),
        RulesProfile::ToriModernV1 => NativeState::Tori(Box::new(tori::Game::standard())),
        RulesProfile::ChuJcsa2019V1 => NativeState::Chu(Box::new(chu::Game::standard())),
        RulesProfile::TenjikuAdamsModernV1 | RulesProfile::TenjikuMullerCompletedV1 => {
            NativeState::Tenjiku(Box::new(tenjiku::Game::standard(
                profile.tenjiku().expect("explicit Tenjiku profile"),
            )))
        }
    };
    state.root()
}

fn chess_command(
    game: &mut chess::ChessMatch,
    actor: chess::Color,
    command: chess::MatchCommand,
) -> Result<(), String> {
    // Chess records rejected requests too. Keep a refused experiment proposal
    // atomic by committing the native candidate only after actual acceptance.
    let mut candidate = game.clone();
    let at_ns = candidate
        .record()
        .timestamp_floor()
        .checked_add(1)
        .ok_or("Chess logical timestamp exhausted")?;
    let reply = candidate.submit(chess::MatchRequest {
        at_ns,
        version: candidate.version(),
        actor,
        command,
    });
    if let Some(rejection) = reply.rejection {
        return Err(format!("Chess native admission: {rejection:?}"));
    }
    *game = candidate;
    Ok(())
}

fn start_chess(game: &mut chess::ChessMatch) -> Result<(), String> {
    if game.conclusion().is_none() && game.active().is_none() {
        chess_command(
            game,
            game.game().position().side_to_move(),
            chess::MatchCommand::Start,
        )?;
    }
    Ok(())
}

fn native_value<T: Serialize>(value: T) -> Value {
    serde_json::to_value(value).expect("native conclusion has finite serializable fields")
}

fn finished<T: Eq>(actual: Option<T>, reported: T) -> Result<Admission, String> {
    if actual == Some(reported) {
        Ok(Admission::Finished)
    } else {
        Err("finished observation disagrees with authoritative native conclusion".into())
    }
}

fn chess_choice(
    game: &chess::ChessMatch,
    telemetry: &mshogi_engine::Telemetry,
    choice: mshogi_engine::EngineChoice,
) -> Result<(), String> {
    if telemetry.completed_depth == 0
        || telemetry.nodes == 0
        || telemetry
            .score
            .is_none_or(|s| i64::from(s).abs() > i64::from(mshogi_search::search::MATE_SCORE))
        || telemetry.principal_variation.first() != Some(&choice)
        || telemetry
            .principal_variation
            .iter()
            .filter(|choice| matches!(choice, mshogi_engine::EngineChoice::Move { .. }))
            .count()
            > usize::from(telemetry.completed_depth)
        || telemetry.principal_variation.len() > usize::from(telemetry.completed_depth) + 1
    {
        return Err("Chess choice has no consistent completed search iteration".into());
    }
    let mut pv = game.clone();
    for (i, choice) in telemetry.principal_variation.iter().enumerate() {
        if pv.conclusion().is_some() {
            return Err("Chess PV continues after native conclusion".into());
        }
        let actor = pv.game().position().side_to_move();
        let command = match choice {
            mshogi_engine::EngineChoice::Move { action } => chess::MatchCommand::Move(*action),
            mshogi_engine::EngineChoice::Claim { claim } => {
                if claim.actor != actor || i + 1 != telemetry.principal_variation.len() {
                    return Err("Chess PV claim actor/continuation invalid".into());
                }
                chess::MatchCommand::Claim {
                    reason: claim.reason,
                    intended: claim.intended,
                }
            }
        };
        chess_command(&mut pv, actor, command)?;
    }
    Ok(())
}

impl NativeState {
    /// Replay the actual native owner and reject unsupported/mismatched roots.
    pub fn from_root(root: &Root) -> Result<Self, String> {
        if !matches!(root.record, CanonicalRecord::Chess(_)) && root.chess_match.is_some() {
            return Err("only Chess roots may carry a Chess match record".into());
        }
        match &root.record {
            CanonicalRecord::Chess(record) => {
                if record.adjudication_mode() != chess::AdjudicationMode::M2 {
                    return Err("internal experiments do not support Chess Full live replay".into());
                }
                let text = root
                    .chess_match
                    .as_ref()
                    .ok_or("Chess root requires its complete native match record")?;
                if text.len() > mshogi_core::native::MAX_RECORD_ENVELOPE_BYTES {
                    return Err("Chess match root exceeds one MiB".into());
                }
                let matched = chess::MatchRecord::from_json(text).map_err(|e| e.to_string())?;
                // Preflight before replay: Full replay can execute substantial
                // reachability, and timed play is a different experiment contract.
                if matched.time_control() != chess::TimeControl::Untimed
                    || matched
                        .initial_record()
                        .map_err(|e| e.to_string())?
                        .adjudication_mode()
                        != chess::AdjudicationMode::M2
                {
                    return Err("internal Chess experiments require untimed M2 matches".into());
                }
                let mut game = matched.replay().map_err(|e| e.to_string())?;
                if game.game().record() != **record {
                    return Err("Chess canonical history and match record disagree".into());
                }
                start_chess(&mut game)?;
                Ok(Self::Chess(Box::new(game)))
            }
            CanonicalRecord::Shogi(record) => record
                .to_game()
                .map(|g| Self::Shogi(Box::new(g)))
                .map_err(|e| e.to_string()),
            CanonicalRecord::Xiangqi(record) => record
                .to_game()
                .map(|g| Self::Xiangqi(Box::new(g)))
                .map_err(|e| e.to_string()),
            CanonicalRecord::Tori(record) => record
                .to_game()
                .map(|g| Self::Tori(Box::new(g)))
                .map_err(|e| e.to_string()),
            CanonicalRecord::Chu(record) => record
                .to_game()
                .map(|g| Self::Chu(Box::new(g)))
                .map_err(|e| e.to_string()),
            CanonicalRecord::Tenjiku(record) => record
                .to_game()
                .map(|g| Self::Tenjiku(Box::new(g)))
                .map_err(|e| e.to_string()),
        }
    }

    /// Capture complete native history and the additional Chess match ledger.
    pub fn root(&self) -> Result<Root, String> {
        let (record, chess_match) = match self {
            Self::Chess(game) => (
                CanonicalRecord::chess(game.game().record()),
                Some(game.record().to_json()),
            ),
            Self::Shogi(game) => (
                CanonicalRecord::shogi(shogi::GameRecord::from_game(game)),
                None,
            ),
            Self::Xiangqi(game) => (
                CanonicalRecord::xiangqi(
                    xiangqi::GameRecord::from_game(game).map_err(|e| e.to_string())?,
                ),
                None,
            ),
            Self::Tori(game) => (
                CanonicalRecord::tori(
                    tori::GameRecord::from_game(game).map_err(|e| e.to_string())?,
                ),
                None,
            ),
            Self::Chu(game) => (
                CanonicalRecord::chu(chu::GameRecord::from_game(game).map_err(|e| e.to_string())?),
                None,
            ),
            Self::Tenjiku(game) => (
                CanonicalRecord::tenjiku(
                    tenjiku::GameRecord::from_game(game).map_err(|e| e.to_string())?,
                ),
                None,
            ),
        };
        Ok(Root {
            record,
            chess_match,
        })
    }

    /// Native side to move, retaining the game-qualified side identity.
    pub fn turn(&self) -> NativeSide {
        match self {
            Self::Chess(g) => NativeSide::Chess(g.game().position().side_to_move()),
            Self::Shogi(g) => NativeSide::Shogi(g.position().side_to_move()),
            Self::Xiangqi(g) => NativeSide::Xiangqi(g.position().side_to_move()),
            Self::Tori(g) => NativeSide::Tori(g.position().side_to_move()),
            Self::Chu(g) => NativeSide::Chu(g.position().side_to_move()),
            Self::Tenjiku(g) => NativeSide::Tenjiku(g.position().side_to_move()),
        }
    }

    /// Exact native conclusion representation, absent for unfinished play.
    pub fn outcome(&self) -> Option<Value> {
        match self {
            Self::Chess(g) => g.conclusion().map(native_value),
            Self::Shogi(g) => g.conclusion().map(native_value),
            Self::Xiangqi(g) => g.conclusion().map(native_value),
            Self::Tori(g) => g.conclusion().map(native_value),
            Self::Chu(g) => g.conclusion().map(native_value),
            Self::Tenjiku(g) => g.conclusion().map(native_value),
        }
    }

    /// First native side's doubled score, absent for unfinished/uncertain play.
    pub fn score_first_twice(&self) -> Option<u8> {
        match self {
            Self::Chess(g) => g
                .conclusion()
                .and_then(chess::MatchConclusion::white_score_twice),
            Self::Shogi(g) => g.conclusion().map(|c| c.score_half_points()[0]),
            Self::Xiangqi(g) => g.conclusion().map(|c| match c {
                xiangqi::Conclusion::Draw { .. } => 1,
                xiangqi::Conclusion::Win { winner, .. } => {
                    if winner == xiangqi::Side::Red {
                        2
                    } else {
                        0
                    }
                }
            }),
            Self::Tori(g) => g.conclusion().map(|c| {
                let winner = match c {
                    tori::Conclusion::Checkmate { winner }
                    | tori::Conclusion::Resignation { winner } => winner,
                    tori::Conclusion::StalemateLoss { loser }
                    | tori::Conclusion::RepetitionForfeit { loser } => loser.opponent(),
                };
                if winner == tori::Side::Sente { 2 } else { 0 }
            }),
            Self::Chu(g) => g.conclusion().map(|c| match c.winner() {
                Some(chu::Side::Sente) => 2,
                Some(chu::Side::Gote) => 0,
                None => 1,
            }),
            Self::Tenjiku(g) => g.conclusion().map(|c| match c.winner() {
                Some(tenjiku::Side::Sente) => 2,
                Some(tenjiku::Side::Gote) => 0,
                None => 1,
            }),
        }
    }

    /// A stopped experiment never manufactures a native win or draw.
    pub fn stop_reason(&self) -> Option<String> {
        if self.outcome().is_some() {
            return Some(
                if self.score_first_twice().is_some() {
                    "native_concluded"
                } else {
                    "native_unresolved"
                }
                .into(),
            );
        }
        match self {
            Self::Xiangqi(g) if g.pending_adjudication().is_some() => {
                Some("native_pending_adjudication".into())
            }
            Self::Tori(g) if g.snapshot().incomplete.is_some() => Some(format!(
                "native_stop:{:?}",
                g.snapshot().incomplete.unwrap()
            )),
            Self::Chu(g) if g.snapshot().incomplete.is_some() => Some(format!(
                "native_stop:{:?}",
                g.snapshot().incomplete.unwrap()
            )),
            Self::Tenjiku(g) if g.snapshot().incomplete.is_some() => Some(format!(
                "native_stop:{:?}",
                g.snapshot().incomplete.unwrap()
            )),
            Self::Chu(g) if g.status() == chu::GameStatus::NoBoardActions => {
                Some("native_no_board_actions".into())
            }
            Self::Tenjiku(g) if g.status() == tenjiku::GameStatus::NoBoardActions => {
                Some("native_no_board_actions".into())
            }
            _ => None,
        }
    }

    /// Submit a proposed complete action/claim/declaration to its native owner.
    /// The caller separately verifies observation identity and requested limits.
    pub fn admit(&mut self, result: &ExecutionResult) -> Result<Admission, String> {
        if let ExecutionResult::Failure { failure } = result {
            return Ok(Admission::Failed(format!(
                "{:?}: {}",
                failure.kind, failure.message
            )));
        }
        match (self, result) {
            (Self::Chess(game), ExecutionResult::Chess { result }) => match result {
                ChessResult::Move { action, telemetry } => {
                    chess_choice(
                        game,
                        telemetry,
                        mshogi_engine::EngineChoice::Move { action: *action },
                    )?;
                    chess_command(
                        game,
                        game.game().position().side_to_move(),
                        chess::MatchCommand::Move(*action),
                    )?;
                    Ok(Admission::Applied)
                }
                ChessResult::Claim { claim, telemetry } => {
                    chess_choice(
                        game,
                        telemetry,
                        mshogi_engine::EngineChoice::Claim { claim: *claim },
                    )?;
                    chess_command(
                        game,
                        claim.actor,
                        chess::MatchCommand::Claim {
                            reason: claim.reason,
                            intended: claim.intended,
                        },
                    )?;
                    Ok(Admission::Applied)
                }
                ChessResult::Finished { outcome, .. } => {
                    finished(game.conclusion(), chess::MatchConclusion::Chess(*outcome))
                }
                ChessResult::Incomplete { telemetry } => {
                    if telemetry.completed_depth != 0
                        || telemetry.score.is_some()
                        || !telemetry.principal_variation.is_empty()
                        || telemetry.status == mshogi_engine::StopReason::Complete
                    {
                        return Err("Chess incomplete result claims completed work".into());
                    }
                    Ok(Admission::Incomplete(format!("{:?}", telemetry.status)))
                }
                ChessResult::Failure { failure } => Ok(Admission::Failed(format!(
                    "{:?}: {}",
                    failure.kind, failure.message
                ))),
                ChessResult::Reachability { .. } => {
                    Err("reachability analysis cannot be admitted as an engine turn".into())
                }
            },
            (Self::Shogi(game), ExecutionResult::Shogi { result }) => match result {
                ShogiResult::Choice { choice, telemetry } => {
                    let mut pv = (**game).clone();
                    for choice in &telemetry.principal_variation {
                        if pv.conclusion().is_some() {
                            return Err("Shogi PV continues after conclusion".into());
                        }
                        let (actor, command) = match choice {
                            ShogiChoice::Move { action } => (
                                pv.position().side_to_move(),
                                shogi::GameCommand::Play(*action),
                            ),
                            ShogiChoice::Declare { actor } => {
                                (*actor, shogi::GameCommand::DeclareEnteringKing)
                            }
                        };
                        pv.submit(shogi::GameRequest {
                            expected: pv.context(),
                            actor,
                            command,
                        })
                        .map_err(|e| format!("Shogi PV: {e:?}"))?;
                    }
                    let (actor, command) = match choice {
                        ShogiChoice::Move { action } => (
                            game.position().side_to_move(),
                            shogi::GameCommand::Play(*action),
                        ),
                        ShogiChoice::Declare { actor } => {
                            (*actor, shogi::GameCommand::DeclareEnteringKing)
                        }
                    };
                    game.submit(shogi::GameRequest {
                        expected: game.context(),
                        actor,
                        command,
                    })
                    .map_err(|e| format!("{e:?}"))?;
                    Ok(Admission::Applied)
                }
                ShogiResult::Finished { conclusion, .. } => {
                    finished(game.conclusion(), *conclusion)
                }
                ShogiResult::Incomplete { telemetry } => {
                    Ok(Admission::Incomplete(format!("{:?}", telemetry.status)))
                }
            },
            (Self::Xiangqi(game), ExecutionResult::Xiangqi { result }) => match result {
                XiangqiResult::Choice { choice, telemetry } => {
                    let mut pv = xiangqi::GameRecord::from_game(game)
                        .map_err(|e| e.to_string())?
                        .to_game()
                        .map_err(|e| e.to_string())?;
                    for choice in &telemetry.principal_variation {
                        if pv.conclusion().is_some() || pv.pending_adjudication().is_some() {
                            return Err("Xiangqi PV continues after native stop".into());
                        }
                        let (actor, command) = match choice {
                            XiangqiChoice::Move { action } => (
                                pv.position().side_to_move(),
                                xiangqi::GameCommand::Play { action: *action },
                            ),
                            XiangqiChoice::Claim { actor } => {
                                (*actor, xiangqi::GameCommand::ClaimNaturalCount)
                            }
                        };
                        let submitted = pv
                            .submit(xiangqi::GameRequest {
                                expected: pv.context(),
                                actor,
                                command,
                            })
                            .map_err(|e| format!("Xiangqi PV: {e}"))?;
                        if submitted.result != xiangqi::CommandResult::Accepted {
                            return Err("Xiangqi PV rejected claim".into());
                        }
                    }
                    let (actor, command) = match choice {
                        XiangqiChoice::Move { action } => (
                            game.position().side_to_move(),
                            xiangqi::GameCommand::Play { action: *action },
                        ),
                        XiangqiChoice::Claim { actor } => {
                            (*actor, xiangqi::GameCommand::ClaimNaturalCount)
                        }
                    };
                    let submission = game
                        .submit(xiangqi::GameRequest {
                            expected: game.context(),
                            actor,
                            command,
                        })
                        .map_err(|e| e.to_string())?;
                    if submission.result != xiangqi::CommandResult::Accepted {
                        game.undo(submission.undo).map_err(|e| e.to_string())?;
                        return Err("Xiangqi native count claim was rejected".into());
                    }
                    Ok(Admission::Applied)
                }
                XiangqiResult::Finished { conclusion, .. } => {
                    finished(game.conclusion(), *conclusion)
                }
                XiangqiResult::Incomplete { telemetry } => {
                    Ok(Admission::Incomplete(format!("{:?}", telemetry.status)))
                }
                XiangqiResult::Unresolved { pending, .. } => Ok(Admission::Incomplete(format!(
                    "native unresolved adjudication: {pending:?}"
                ))),
            },
            (Self::Tori(game), ExecutionResult::Tori { result }) => {
                result.validate(game)?;
                match result {
                    ToriResult::Choice { action, .. } => {
                        game.submit(tori::GameRequest {
                            expected: game.context(),
                            actor: game.position().side_to_move(),
                            command: tori::GameCommand::Play { action: *action },
                        })
                        .map_err(|e| e.to_string())?;
                        Ok(Admission::Applied)
                    }
                    ToriResult::Finished { conclusion, .. } => {
                        finished(game.conclusion(), *conclusion)
                    }
                    ToriResult::Incomplete { telemetry } => {
                        Ok(Admission::Incomplete(format!("{:?}", telemetry.status)))
                    }
                }
            }
            (Self::Chu(game), ExecutionResult::Chu { result }) => {
                result.validate(game)?;
                match result {
                    ChuResult::Choice { action, .. } => {
                        game.submit(chu::GameRequest {
                            expected: game.context(),
                            actor: game.position().side_to_move(),
                            command: chu::GameCommand::Play {
                                action: action.clone(),
                            },
                        })
                        .map_err(|e| e.to_string())?;
                        Ok(Admission::Applied)
                    }
                    ChuResult::Finished { conclusion, .. } => {
                        finished(game.conclusion(), *conclusion)
                    }
                    ChuResult::Incomplete { telemetry } => {
                        Ok(Admission::Incomplete(format!("{:?}", telemetry.status)))
                    }
                }
            }
            (Self::Tenjiku(game), ExecutionResult::Tenjiku { result }) => {
                result.validate(game)?;
                match result {
                    TenjikuResult::Choice { action, .. } => {
                        game.submit(tenjiku::GameRequest {
                            expected: game.context(),
                            actor: game.position().side_to_move(),
                            command: tenjiku::GameCommand::Play {
                                action: action.clone(),
                            },
                        })
                        .map_err(|e| e.to_string())?;
                        Ok(Admission::Applied)
                    }
                    TenjikuResult::Finished { conclusion, .. } => {
                        finished(game.conclusion(), *conclusion)
                    }
                    TenjikuResult::Incomplete { telemetry } => {
                        Ok(Admission::Incomplete(format!("{:?}", telemetry.status)))
                    }
                }
            }
            _ => Err("execution result belongs to a different native game".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mshogi_engine::{StopReason, Telemetry};

    const PROFILES: [RulesProfile; 7] = [
        RulesProfile::ChessMatchV1,
        RulesProfile::ShogiCsa36V1,
        RulesProfile::XiangqiWxf2018V1,
        RulesProfile::ToriModernV1,
        RulesProfile::ChuJcsa2019V1,
        RulesProfile::TenjikuAdamsModernV1,
        RulesProfile::TenjikuMullerCompletedV1,
    ];

    fn same_root(a: &Root, b: &Root) {
        assert_eq!(a.record, b.record);
        assert_eq!(a.chess_match, b.chess_match);
    }

    #[test]
    fn all_seven_standard_profiles_replay_exactly() {
        for profile in PROFILES {
            let root = standard_root(profile).unwrap();
            assert_eq!(root.record.profile(), profile);
            let state = NativeState::from_root(&root).unwrap();
            assert_eq!(state.turn().game(), profile.game());
            assert!(state.outcome().is_none());
            assert_eq!(state.score_first_twice(), None);
            same_root(&state.root().unwrap(), &root);
        }
    }

    #[test]
    fn root_refuses_missing_wrong_or_inconsistent_chess_match() {
        let mut chess = standard_root(RulesProfile::ChessMatchV1).unwrap();
        let original = chess.chess_match.take();
        assert!(NativeState::from_root(&chess).is_err());
        let mut shogi = standard_root(RulesProfile::ShogiCsa36V1).unwrap();
        shogi.chess_match = original.clone();
        assert!(NativeState::from_root(&shogi).is_err());
        chess.chess_match = original;
        chess.record = CanonicalRecord::chess(
            chess::AdjudicatedGame::from_fen(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
                chess::ClaimPolicy::AutoClaim,
            )
            .unwrap()
            .record(),
        );
        assert!(NativeState::from_root(&chess).is_err());
        let mut timed = chess::ChessMatch::new(
            chess::AdjudicatedGame::new(chess::ClaimPolicy::AutoClaim),
            chess::TimeControl::Increment {
                initial_ns: [100, 100],
                increment_ns: 1,
            },
        )
        .unwrap();
        start_chess(&mut timed).unwrap();
        assert!(
            NativeState::from_root(&NativeState::Chess(Box::new(timed)).root().unwrap()).is_err()
        );
        let full = chess::AdjudicatedGame::from_fen_with_mode(
            chess::STANDARD_START_FEN,
            chess::ClaimPolicy::AutoClaim,
            chess::AdjudicationMode::Full {
                limits: chess::ReachabilityLimits {
                    max_positions: Some(0),
                    max_transitions: Some(0),
                },
            },
        )
        .unwrap();
        let full = chess::ChessMatch::new(full, chess::TimeControl::Untimed).unwrap();
        assert!(
            NativeState::from_root(&NativeState::Chess(Box::new(full)).root().unwrap()).is_err()
        );
    }

    fn telemetry() -> Telemetry {
        Telemetry {
            score: None,
            completed_depth: 0,
            nodes: 0,
            principal_variation: vec![],
            status: StopReason::NodeLimit,
        }
    }

    fn move_telemetry(action: chess::Action) -> Telemetry {
        Telemetry {
            score: Some(0),
            completed_depth: 1,
            nodes: 21,
            principal_variation: vec![mshogi_engine::EngineChoice::Move { action }],
            status: StopReason::Complete,
        }
    }

    #[test]
    fn incomplete_and_wrong_family_do_not_mutate_and_invalid_move_is_atomic() {
        let root = standard_root(RulesProfile::ChessMatchV1).unwrap();
        let mut state = NativeState::from_root(&root).unwrap();
        let incomplete = ExecutionResult::Chess {
            result: ChessResult::Incomplete {
                telemetry: telemetry(),
            },
        };
        assert!(matches!(
            state.admit(&incomplete).unwrap(),
            Admission::Incomplete(_)
        ));
        same_root(&root, &state.root().unwrap());
        let mut shogi =
            NativeState::from_root(&standard_root(RulesProfile::ShogiCsa36V1).unwrap()).unwrap();
        let before = shogi.root().unwrap();
        assert!(shogi.admit(&incomplete).is_err());
        same_root(&before, &shogi.root().unwrap());
        let invalid = ExecutionResult::Chess {
            result: ChessResult::Move {
                action: "e2e5".parse().unwrap(),
                telemetry: move_telemetry("e2e5".parse().unwrap()),
            },
        };
        assert!(state.admit(&invalid).is_err());
        same_root(&root, &state.root().unwrap());
        let fallback = ExecutionResult::Chess {
            result: ChessResult::Move {
                action: "e2e4".parse().unwrap(),
                telemetry: telemetry(),
            },
        };
        assert!(state.admit(&fallback).is_err());
        same_root(&root, &state.root().unwrap());
    }

    #[test]
    fn chess_move_and_full_match_history_are_retained() {
        let root = standard_root(RulesProfile::ChessMatchV1).unwrap();
        let mut state = NativeState::from_root(&root).unwrap();
        let result = ExecutionResult::Chess {
            result: ChessResult::Move {
                action: "e2e4".parse().unwrap(),
                telemetry: move_telemetry("e2e4".parse().unwrap()),
            },
        };
        assert_eq!(state.admit(&result).unwrap(), Admission::Applied);
        let after = state.root().unwrap();
        assert_eq!(after.record.known_board_actions(), 1);
        let matched = chess::MatchRecord::from_json(after.chess_match.as_ref().unwrap()).unwrap();
        assert_eq!(matched.event_count(), 2);
        assert_eq!(matched.timestamp_floor(), 2);
        same_root(
            &after,
            &NativeState::from_root(&after).unwrap().root().unwrap(),
        );
        let false_finish = ExecutionResult::Chess {
            result: ChessResult::Finished {
                outcome: chess::ChessOutcome::Checkmate {
                    winner: chess::Color::White,
                },
                telemetry: telemetry(),
            },
        };
        assert!(state.admit(&false_finish).is_err());
        same_root(&after, &state.root().unwrap());
    }

    #[test]
    fn fresh_match_and_native_auto_claim_preserve_history_without_playing_intention() {
        let record = CanonicalRecord::chess(
            chess::AdjudicatedGame::from_fen(
                "7k/8/5KQ1/8/8/8/8/8 w - - 100 1",
                chess::ClaimPolicy::AutoClaim,
            )
            .unwrap()
            .record(),
        );
        let root = fresh_root(record).unwrap();
        let mut state = NativeState::from_root(&root).unwrap();
        let claim = mshogi_engine::EngineClaim {
            actor: chess::Color::White,
            reason: chess::DrawClaim::FiftyMove,
            intended: None,
        };
        let result = ExecutionResult::Chess {
            result: ChessResult::Claim {
                claim,
                telemetry: Telemetry {
                    score: Some(0),
                    completed_depth: 1,
                    nodes: 1,
                    principal_variation: vec![mshogi_engine::EngineChoice::Claim { claim }],
                    status: StopReason::Complete,
                },
            },
        };
        assert_eq!(state.admit(&result).unwrap(), Admission::Applied);
        assert_eq!(state.score_first_twice(), Some(1));
        let after = state.root().unwrap();
        assert_eq!(after.record.known_board_actions(), 0);
        assert_eq!(after.record.event_count(), 1);
        same_root(
            &after,
            &NativeState::from_root(&after).unwrap().root().unwrap(),
        );
    }

    #[test]
    fn complete_native_compound_actions_survive_record_replay() {
        // These committed native diagrams have independently exercised compound
        // routes. Initial armies deliberately block such routes in some games.
        for (profile, fixture) in [
            (
                RulesProfile::ChuJcsa2019V1,
                include_str!("../../../../desktop/scripts/fixtures/chu-royal-both.mshogi"),
            ),
            (
                RulesProfile::TenjikuAdamsModernV1,
                include_str!("../../../../desktop/scripts/fixtures/tenjiku-adams-compound.mshogi"),
            ),
            (
                RulesProfile::TenjikuMullerCompletedV1,
                include_str!("../../../../desktop/scripts/fixtures/tenjiku-muller-compound.mshogi"),
            ),
        ] {
            let fixture: Value = serde_json::from_str(fixture).unwrap();
            let record =
                CanonicalRecord::from_json(&fixture["payload"]["initial"].to_string()).unwrap();
            let mut state = NativeState::from_root(&fresh_root(record).unwrap()).unwrap();
            match &mut state {
                NativeState::Chu(g) => {
                    let action = g
                        .legal_actions()
                        .into_iter()
                        .find(|a| a.route.len() > 1)
                        .expect("accepted fixture Lion compound action");
                    g.submit(chu::GameRequest {
                        expected: g.context(),
                        actor: g.position().side_to_move(),
                        command: chu::GameCommand::Play { action },
                    })
                    .unwrap();
                }
                NativeState::Tenjiku(g) => {
                    let action = g
                        .legal_actions()
                        .into_iter()
                        .find(|a| a.route.len() > 1)
                        .expect("accepted fixture compound action");
                    g.submit(tenjiku::GameRequest {
                        expected: g.context(),
                        actor: g.position().side_to_move(),
                        command: tenjiku::GameCommand::Play { action },
                    })
                    .unwrap();
                }
                _ => unreachable!(),
            }
            let root = state.root().unwrap();
            assert_eq!(root.record.known_board_actions(), 1);
            assert_eq!(root.record.profile(), profile);
            same_root(
                &root,
                &NativeState::from_root(&root).unwrap().root().unwrap(),
            );
        }
    }

    #[test]
    fn native_tori_stop_is_unscored_and_resignation_is_exact() {
        let mut state =
            NativeState::from_root(&standard_root(RulesProfile::ToriModernV1).unwrap()).unwrap();
        let NativeState::Tori(g) = &mut state else {
            unreachable!()
        };
        g.submit(tori::GameRequest {
            expected: g.context(),
            actor: tori::Side::Sente,
            command: tori::GameCommand::Stop {
                reason: tori::StopReason::WorkLimit,
            },
        })
        .unwrap();
        assert!(state.stop_reason().is_some());
        assert!(state.outcome().is_none());
        assert_eq!(state.score_first_twice(), None);
        let NativeState::Tori(g) = &mut state else {
            unreachable!()
        };
        g.submit(tori::GameRequest {
            expected: g.context(),
            actor: tori::Side::Gote,
            command: tori::GameCommand::Resign,
        })
        .unwrap();
        assert_eq!(
            state.outcome(),
            Some(native_value(tori::Conclusion::Resignation {
                winner: tori::Side::Sente
            }))
        );
        assert_eq!(state.score_first_twice(), Some(2));
        let root = state.root().unwrap();
        same_root(
            &root,
            &NativeState::from_root(&root).unwrap().root().unwrap(),
        );
    }

    #[test]
    fn native_tori_validation_refuses_forged_finished_and_incomplete_is_noop() {
        let root = standard_root(RulesProfile::ToriModernV1).unwrap();
        let mut state = NativeState::from_root(&root).unwrap();
        let telemetry = mshogi_engine::ToriTelemetry {
            evaluator_id: "mshogi.tori.material.v1".into(),
            search_id: "mshogi.tori.full-width.v1".into(),
            perspective: tori::Side::Sente,
            score: None,
            completed_depth: 0,
            nodes: 0,
            principal_variation: vec![],
            status: StopReason::NodeLimit,
        };
        let incomplete = ExecutionResult::Tori {
            result: ToriResult::Incomplete {
                telemetry: telemetry.clone(),
            },
        };
        assert!(matches!(
            state.admit(&incomplete).unwrap(),
            Admission::Incomplete(_)
        ));
        same_root(&root, &state.root().unwrap());
        let forged = ExecutionResult::Tori {
            result: ToriResult::Finished {
                conclusion: tori::Conclusion::Checkmate {
                    winner: tori::Side::Sente,
                },
                telemetry,
            },
        };
        assert!(state.admit(&forged).is_err());
        same_root(&root, &state.root().unwrap());
    }
}
