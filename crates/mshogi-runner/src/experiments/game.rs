//! Experiment orchestration over the existing rules owners. No rules live here.
use mshogi_core::{
    chess, chu,
    game::{CanonicalRecord, GameKind, RulesProfile},
    shogi, tenjiku, tori, xiangqi,
};
use mshogi_engine::{ChessResult, EngineGame, EngineResult, ExecutionResult, WorkLimits, game};
use mshogi_search::root::RootGame;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Complete canonical history plus the additional Chess match command ledger.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Root {
    /// Complete authoritative game record, retaining immutable profile/history.
    pub record: CanonicalRecord,
    /// Required only for Chess; an exact MatchRecord JSON string.
    pub chess_match: Option<String>,
}

/// Result of admitting a supervised mShogi engine observation.
#[derive(Debug, Eq, PartialEq)]
pub enum Admission {
    /// Authoritative rules owner committed the proposed command.
    Applied,
    /// Observation agrees with an already established game conclusion.
    Finished,
    /// Work ended without a usable completed choice or game result.
    Incomplete(String),
    /// Supervised execution failed without a game result.
    Failed(String),
}

/// Begin a fresh untimed experiment match around an existing game history.
/// For Chess this explicitly creates a new match; it cannot recover an old
/// match's commands, offers or clocks from its canonical history alone.
pub fn fresh_root(record: CanonicalRecord) -> Result<Root, String> {
    let chess_match = if let CanonicalRecord::Chess(history) = &record {
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
    MatchState::from_root(&root)?.root()
}

/// The selected baseline starts are untimed; Chess uses explicit AutoClaim M2.
pub fn standard_root(profile: RulesProfile) -> Result<Root, String> {
    MatchState::standard(profile)?.root()
}

fn standard_chess() -> Result<chess::ChessMatch, String> {
    let mut game = chess::ChessMatch::new(
        chess::AdjudicatedGame::new(chess::ClaimPolicy::AutoClaim),
        chess::TimeControl::Untimed,
    )
    .map_err(|e| e.to_string())?;
    start_chess(&mut game)?;
    Ok(game)
}

fn chess_command(
    game: &mut chess::ChessMatch,
    actor: chess::Color,
    command: chess::MatchCommand,
) -> Result<(), String> {
    // Chess records rejected requests too. Keep a refused experiment proposal
    // atomic by committing the candidate only after actual acceptance.
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
        return Err(format!("Chess game admission: {rejection:?}"));
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

fn conclusion_value<T: Serialize>(value: T) -> Value {
    serde_json::to_value(value).expect("game conclusion has finite serializable fields")
}

fn finished<T: Eq>(actual: Option<T>, reported: T) -> Result<Admission, String> {
    if actual == Some(reported) {
        Ok(Admission::Finished)
    } else {
        Err("finished observation disagrees with authoritative game conclusion".into())
    }
}

fn chess_choice(
    game: &chess::ChessMatch,
    telemetry: &mshogi_engine::Telemetry,
    choice: mshogi_engine::EngineChoice,
) -> Result<(), String> {
    let bound = telemetry
        .variation_depth_bound()
        .ok_or("Chess selective depth exceeds its search bound")?;
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
            > usize::from(bound)
        || telemetry.principal_variation.len() > usize::from(bound) + 1
    {
        return Err("Chess choice has no consistent completed search iteration".into());
    }
    let mut pv = game.clone();
    for (i, choice) in telemetry.principal_variation.iter().enumerate() {
        if pv.conclusion().is_some() {
            return Err("Chess PV continues after game conclusion".into());
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

fn replay_chess(root: &Root, record: &chess::GameRecord) -> Result<chess::ChessMatch, String> {
    let text = root
        .chess_match
        .as_ref()
        .ok_or("Chess root requires its complete match record")?;
    let matched = chess::MatchRecord::from_json(text).map_err(|e| e.to_string())?;
    // Timed play is a different experiment contract.
    if matched.time_control() != chess::TimeControl::Untimed {
        return Err("internal Chess experiments require untimed matches".into());
    }
    let mut game = matched.replay().map_err(|e| e.to_string())?;
    if game.game().record() != *record {
        return Err("Chess canonical history and match record disagree".into());
    }
    start_chess(&mut game)?;
    Ok(game)
}

fn admit_engine<G: EngineGame>(
    game: &mut G::Game,
    result: &ExecutionResult,
) -> Result<Admission, String> {
    let result = G::result(result).ok_or("execution result belongs to a different game")?;
    result.validate(game)?;
    match result {
        EngineResult::Choice { choice, .. } => {
            let (actor, command) = G::command(choice, G::side_to_move(game));
            G::submit(game, actor, command, &mut || false)?;
            Ok(Admission::Applied)
        }
        EngineResult::Finished { conclusion, .. } => finished(game.finished(), *conclusion),
        EngineResult::Incomplete { telemetry } => {
            Ok(Admission::Incomplete(format!("{:?}", telemetry.status)))
        }
        EngineResult::Unresolved { pending, .. } => Ok(Admission::Incomplete(format!(
            "unresolved adjudication: {pending:?}"
        ))),
    }
}

// The five shared-engine adapters use the same admission and replay operations.
// Each row supplies the starting position, record encoding and first side.
macro_rules! experiment_games {
    ($(($variant:ident, $rules:ident, $first:ident, $start:expr, $record:expr, $score:expr)),+ $(,)?) => {
        #[derive(Clone)]
        pub(super) enum MatchState {
            Chess(Box<chess::ChessMatch>),
            $($variant(Box<$rules::Game>),)+
        }

        pub(super) fn valid_work(profile: RulesProfile, work: WorkLimits) -> bool {
            work.time_ms.is_none() && match profile.game() {
                GameKind::Chess => (1..=mshogi_engine::MAX_ANALYSIS_DEPTH).contains(&work.depth)
                    && work.nodes <= mshogi_engine::MAX_ANALYSIS_NODES,
                $(GameKind::$variant => game::$variant::valid_limits(work),)+
            }
        }

        impl MatchState {
            fn standard(profile: RulesProfile) -> Result<Self, String> {
                Ok(match profile.game() {
                    GameKind::Chess => Self::Chess(Box::new(standard_chess()?)),
                    $(GameKind::$variant => Self::$variant(Box::new(($start)(profile)?)),)+
                })
            }

            pub fn from_root(root: &Root) -> Result<Self, String> {
                if !matches!(root.record, CanonicalRecord::Chess(_)) && root.chess_match.is_some() {
                    return Err("only Chess roots may carry a Chess match record".into());
                }
                Ok(match &root.record {
                    CanonicalRecord::Chess(record) => Self::Chess(Box::new(replay_chess(root, record)?)),
                    $(CanonicalRecord::$variant(record) => Self::$variant(Box::new(game::$variant::to_game(record)?)),)+
                })
            }

            pub fn root(&self) -> Result<Root, String> {
                let (record, chess_match) = match self {
                    Self::Chess(g) => (CanonicalRecord::chess(g.game().record()), Some(g.record().to_json())),
                    $(Self::$variant(g) => (CanonicalRecord::$rules(($record)(g)?), None),)+
                };
                Ok(Root { record, chess_match })
            }

            pub fn first_to_move(&self) -> bool {
                match self {
                    Self::Chess(g) => g.game().position().side_to_move() == chess::Color::White,
                    $(Self::$variant(g) => g.side() == $rules::Side::$first,)+
                }
            }

            pub fn outcome(&self) -> Option<Value> {
                match self {
                    Self::Chess(g) => g.conclusion().map(conclusion_value),
                    $(Self::$variant(g) => g.conclusion().map(conclusion_value),)+
                }
            }

            pub fn score_first_twice(&self) -> Option<u8> {
                match self {
                    Self::Chess(g) => g.conclusion().and_then(chess::MatchConclusion::white_score_twice),
                    $(Self::$variant(g) => g.conclusion().map($score),)+
                }
            }

            pub fn admit(&mut self, result: &ExecutionResult) -> Result<Admission, String> {
                if let ExecutionResult::Failure { failure } = result {
                    return Ok(Admission::Failed(format!("{:?}: {}", failure.kind, failure.message)));
                }
                match self {
                    Self::Chess(g) => admit_chess(g, result),
                    $(Self::$variant(g) => admit_engine::<game::$variant>(g, result),)+
                }
            }
        }
    };
}

fn winner_score<G: RootGame>(conclusion: G::Conclusion, first: G::Side) -> u8 {
    G::winner(conclusion).map_or(1, |winner| if winner == first { 2 } else { 0 })
}

experiment_games!(
    (
        Shogi,
        shogi,
        Sente,
        |_| Ok::<_, String>(shogi::Game::new()),
        |g| Ok::<_, String>(shogi::GameRecord::from_game(g)),
        |c| c.score_half_points()[0]
    ),
    (
        Xiangqi,
        xiangqi,
        Red,
        |_| xiangqi::Game::standard(xiangqi::ClaimPolicy::AutoValidClaims)
            .map_err(|e| e.to_string()),
        |g| xiangqi::GameRecord::from_game(g).map_err(|e| e.to_string()),
        |c| crate::xiangqi::score(c)[0]
    ),
    (
        Tori,
        tori,
        Sente,
        |_| Ok::<_, String>(tori::Game::standard()),
        |g| tori::GameRecord::from_game(g).map_err(|e| e.to_string()),
        |c| winner_score::<tori::Game>(c, tori::Side::Sente)
    ),
    (
        Chu,
        chu,
        Sente,
        |_| Ok::<_, String>(chu::Game::standard()),
        |g| chu::GameRecord::from_game(g).map_err(|e| e.to_string()),
        |c| winner_score::<chu::Game>(c, chu::Side::Sente)
    ),
    (
        Tenjiku,
        tenjiku,
        Sente,
        |p: RulesProfile| Ok::<_, String>(tenjiku::Game::standard(
            p.tenjiku().expect("Tenjiku profile")
        )),
        |g| tenjiku::GameRecord::from_game(g).map_err(|e| e.to_string()),
        |c| winner_score::<tenjiku::Game>(c, tenjiku::Side::Sente)
    ),
);

impl MatchState {
    /// A stopped experiment never manufactures a win or draw.
    pub fn stop_reason(&self) -> Option<String> {
        if self.outcome().is_some() {
            return Some(
                if self.score_first_twice().is_some() {
                    "game_concluded"
                } else {
                    "unresolved"
                }
                .into(),
            );
        }
        match self {
            Self::Xiangqi(g) if g.pending_adjudication().is_some() => {
                Some("pending_adjudication".into())
            }
            Self::Chu(g) if g.status() == chu::GameStatus::NoBoardActions => {
                Some("no_board_actions".into())
            }
            Self::Tenjiku(g) if g.status() == tenjiku::GameStatus::NoBoardActions => {
                Some("no_board_actions".into())
            }
            _ => None,
        }
    }
}

fn admit_chess(
    game: &mut chess::ChessMatch,
    result: &ExecutionResult,
) -> Result<Admission, String> {
    let ExecutionResult::Chess { result } = result else {
        return Err("execution result belongs to a different game".into());
    };
    match result {
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
    }
}

#[cfg(test)]
#[path = "../../tests/support/document_fixture.rs"]
mod document_fixture;

#[cfg(test)]
mod tests {
    use super::*;
    use mshogi_engine::{StopReason, Telemetry};

    use super::super::PROFILES;
    use mshogi_engine::ToriResult;

    fn same_root(a: &Root, b: &Root) {
        assert_eq!(a.record, b.record);
        assert_eq!(a.chess_match, b.chess_match);
    }

    #[test]
    fn all_seven_standard_profiles_replay_exactly() {
        for profile in PROFILES {
            let root = standard_root(profile).unwrap();
            assert_eq!(root.record.profile(), profile);
            let state = MatchState::from_root(&root).unwrap();
            assert!(state.first_to_move());
            assert!(state.outcome().is_none());
            assert_eq!(state.score_first_twice(), None);
            same_root(&state.root().unwrap(), &root);
        }
    }

    #[test]
    fn root_refuses_missing_wrong_or_inconsistent_chess_match() {
        let mut chess = standard_root(RulesProfile::ChessMatchV1).unwrap();
        let original = chess.chess_match.take();
        assert!(MatchState::from_root(&chess).is_err());
        let mut shogi = standard_root(RulesProfile::ShogiCsa36V1).unwrap();
        shogi.chess_match = original.clone();
        assert!(MatchState::from_root(&shogi).is_err());
        chess.chess_match = original;
        chess.record = CanonicalRecord::chess(
            chess::AdjudicatedGame::from_fen(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1",
                chess::ClaimPolicy::AutoClaim,
            )
            .unwrap()
            .record(),
        );
        assert!(MatchState::from_root(&chess).is_err());
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
            MatchState::from_root(&MatchState::Chess(Box::new(timed)).root().unwrap()).is_err()
        );
    }

    fn telemetry() -> Telemetry {
        Telemetry {
            selective_depth: 0,
            score: None,
            completed_depth: 0,
            nodes: 0,
            principal_variation: vec![],
            status: StopReason::NodeLimit,
        }
    }

    fn move_telemetry(action: chess::Action) -> Telemetry {
        Telemetry {
            selective_depth: 1,
            score: Some(0),
            completed_depth: 1,
            nodes: 21,
            principal_variation: vec![mshogi_engine::EngineChoice::Move { action }],
            status: StopReason::Complete,
        }
    }

    #[test]
    fn checks_selective_pv_bounds_and_replayed_actions() {
        // Constructed admission machinery, not an actual engine observation.
        let root = standard_root(RulesProfile::ChessMatchV1).unwrap();
        let action = "e2e4".parse().unwrap();
        let mut telemetry = move_telemetry(action);
        telemetry.principal_variation = ["e2e4", "e7e5", "g1f3"]
            .map(|a| mshogi_engine::EngineChoice::Move {
                action: a.parse().unwrap(),
            })
            .to_vec();
        for bound in [1, 2, 129] {
            telemetry.selective_depth = bound;
            let mut state = MatchState::from_root(&root).unwrap();
            let result = ExecutionResult::Chess {
                result: ChessResult::Move {
                    action,
                    telemetry: telemetry.clone(),
                },
            };
            assert!(state.admit(&result).is_err());
            same_root(&root, &state.root().unwrap());
        }
        telemetry.selective_depth = 3;
        let mut state = MatchState::from_root(&root).unwrap();
        let result = ExecutionResult::Chess {
            result: ChessResult::Move {
                action,
                telemetry: telemetry.clone(),
            },
        };
        assert!(matches!(state.admit(&result).unwrap(), Admission::Applied));
        assert_eq!(state.root().unwrap().record.known_board_actions(), 1);
        telemetry.principal_variation[2] = mshogi_engine::EngineChoice::Move {
            action: "g1g4".parse().unwrap(),
        };
        let mut state = MatchState::from_root(&root).unwrap();
        let result = ExecutionResult::Chess {
            result: ChessResult::Move { action, telemetry },
        };
        assert!(state.admit(&result).is_err());
        same_root(&root, &state.root().unwrap());
    }

    #[test]
    fn incomplete_and_wrong_family_do_not_mutate_and_invalid_move_is_atomic() {
        let root = standard_root(RulesProfile::ChessMatchV1).unwrap();
        let mut state = MatchState::from_root(&root).unwrap();
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
            MatchState::from_root(&standard_root(RulesProfile::ShogiCsa36V1).unwrap()).unwrap();
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
        let mut state = MatchState::from_root(&root).unwrap();
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
            &MatchState::from_root(&after).unwrap().root().unwrap(),
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
    fn fresh_match_and_auto_claim_preserve_history_without_playing_intention() {
        let record = CanonicalRecord::chess(
            chess::AdjudicatedGame::from_fen(
                "7k/8/5KQ1/8/8/8/8/8 w - - 100 1",
                chess::ClaimPolicy::AutoClaim,
            )
            .unwrap()
            .record(),
        );
        let root = fresh_root(record).unwrap();
        let mut state = MatchState::from_root(&root).unwrap();
        let claim = mshogi_engine::EngineClaim {
            actor: chess::Color::White,
            reason: chess::DrawClaim::FiftyMove,
            intended: None,
        };
        let result = ExecutionResult::Chess {
            result: ChessResult::Claim {
                claim,
                telemetry: Telemetry {
                    selective_depth: 1,
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
            &MatchState::from_root(&after).unwrap().root().unwrap(),
        );
    }

    #[test]
    fn complete_compound_actions_survive_record_replay() {
        // These committed diagrams have independently exercised compound
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
            let record = document_fixture::initial_record(fixture).unwrap();
            let mut state = MatchState::from_root(&fresh_root(record).unwrap()).unwrap();
            match &mut state {
                MatchState::Chu(g) => {
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
                MatchState::Tenjiku(g) => {
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
                &MatchState::from_root(&root).unwrap().root().unwrap(),
            );
        }
    }

    #[test]
    fn tori_live_game_is_unscored_and_resignation_is_exact() {
        let mut state =
            MatchState::from_root(&standard_root(RulesProfile::ToriModernV1).unwrap()).unwrap();
        assert!(state.stop_reason().is_none());
        assert!(state.outcome().is_none());
        assert_eq!(state.score_first_twice(), None);
        let MatchState::Tori(g) = &mut state else {
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
            Some(conclusion_value(tori::Conclusion::Resignation {
                winner: tori::Side::Sente
            }))
        );
        assert_eq!(state.score_first_twice(), Some(2));
        let root = state.root().unwrap();
        same_root(
            &root,
            &MatchState::from_root(&root).unwrap().root().unwrap(),
        );
    }

    #[test]
    fn tori_validation_refuses_forged_finished_and_incomplete_is_noop() {
        let root = standard_root(RulesProfile::ToriModernV1).unwrap();
        let mut state = MatchState::from_root(&root).unwrap();
        let telemetry = mshogi_engine::ToriTelemetry {
            evaluator_id: mshogi_search::tori::EVALUATOR_ID.into(),
            search_id: mshogi_search::tori::SEARCH_ID.into(),
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
