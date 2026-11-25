//! Validate native Chess search and Full analysis evidence.
use super::*;
use crate::persistence::check;

pub(crate) fn validate_result(
    result: &ChessResult,
    context: &ExecutionContext,
    game: &AdjudicatedGame,
    stopped: &mut impl FnMut() -> bool,
) -> Result<(), String> {
    use mshogi_engine::{EngineChoice, ExecutionPurpose, StopReason};
    check(stopped)?;
    let full = matches!(
        context.purpose,
        ExecutionPurpose::ChessFullReachability { .. }
    );
    if let ChessResult::Failure { failure } = result {
        return if failure.message.len() <= mshogi_engine::MAX_STDERR_BYTES {
            Ok(())
        } else {
            Err("Failure diagnostic exceeds its byte limit.".into())
        };
    }
    if let ChessResult::Reachability { result } = result {
        let ExecutionPurpose::ChessFullReachability { limits, .. } = context.purpose else {
            return Err("Reachability observations require the Full purpose.".into());
        };
        let work = match result {
            MateReachability::Possible { witness, work } => {
                if work.positions == 0
                    || work.expanded == 0
                    || witness.len() as u64 >= work.positions
                    || witness.len() as u64 > work.transitions
                    || witness.len() as u64 > limits.max_positions
                {
                    return Err("Reachability witness exceeds recorded work evidence.".into());
                }
                let ExecutionPurpose::ChessFullReachability { mate, .. } = context.purpose else {
                    unreachable!()
                };
                let targets: &[Color] = match mate {
                    mshogi_engine::ReachabilityTarget::Either => &[Color::White, Color::Black],
                    mshogi_engine::ReachabilityTarget::White => &[Color::White],
                    mshogi_engine::ReachabilityTarget::Black => &[Color::Black],
                };
                let mut valid = false;
                for side in targets {
                    check(stopped)?;
                    match mating_feasibility_with_stop(game.position(), *side, Some(witness), &mut *stopped) {
                        Ok(MatingFeasibility::Possible) => { valid = true; break; },
                        Err("mating witness verification interrupted") => return Err("Full witness verification interrupted by the operation budget/cancellation.".into()),
                        _ => {},
                    }
                }
                check(stopped)?;
                if !valid {
                    return Err(
                        "Full cooperative witness does not establish the requested legal mate."
                            .into(),
                    );
                }
                check(stopped)?;
                work
            }
            MateReachability::Impossible { work } | MateReachability::Unknown { work, .. } => work,
        };
        if work.positions > limits.max_positions
            || work.transitions > limits.max_transitions
            || work.expanded > work.positions
            || work.static_leaves > work.expanded
        {
            return Err("Reachability observations exceed their recorded work limits.".into());
        }
        return Ok(());
    }
    if full {
        return Err("Full purpose has minimax observations.".into());
    }
    let telemetry = match result {
        ChessResult::Move { telemetry, .. }
        | ChessResult::Claim { telemetry, .. }
        | ChessResult::Finished { telemetry, .. }
        | ChessResult::Incomplete { telemetry } => telemetry,
        _ => unreachable!(),
    };
    let bound = telemetry
        .variation_depth_bound()
        .ok_or("Search selective depth exceeds its native bound.")?;
    let board_plies = telemetry
        .principal_variation
        .iter()
        .filter(|choice| matches!(choice, EngineChoice::Move { .. }))
        .count();
    if telemetry.completed_depth > context.limits.depth
        || telemetry.nodes > context.limits.nodes
        || board_plies > usize::from(bound)
        || telemetry.principal_variation.len() > usize::from(bound) + 1
        || telemetry.score.is_some() != (telemetry.completed_depth > 0)
        || telemetry
            .score
            .is_some_and(|score| score.unsigned_abs() > 1_000_000)
    {
        return Err("Minimax observations disagree with recorded limits/iteration shape.".into());
    }
    match result {
        ChessResult::Incomplete { .. }
            if telemetry.completed_depth != 0 || telemetry.status == StopReason::Complete =>
        {
            return Err("Incomplete result contains a completed iteration.".into());
        }
        ChessResult::Move { action, .. }
            if telemetry.principal_variation.first()
                != Some(&EngineChoice::Move { action: *action }) =>
        {
            return Err("Move decision disagrees with its continuation.".into());
        }
        ChessResult::Claim { claim, .. }
            if telemetry.principal_variation.first()
                != Some(&EngineChoice::Claim { claim: *claim }) =>
        {
            return Err("Claim decision disagrees with its continuation.".into());
        }
        ChessResult::Finished { outcome, .. }
            if game.outcome() != Some(*outcome) || !telemetry.principal_variation.is_empty() =>
        {
            return Err("Finished observation disagrees with native root outcome.".into());
        }
        _ => {}
    }
    let mut continuation = game.clone();
    for (index, choice) in telemetry.principal_variation.iter().enumerate() {
        check(stopped)?;
        match choice {
            EngineChoice::Move { action } => {
                continuation.apply(*action).map_err(|e| e.to_string())?
            }
            EngineChoice::Claim { claim } => {
                if index + 1 != telemetry.principal_variation.len() {
                    return Err("Search continuation goes beyond a terminal claim.".into());
                }
                continuation
                    .claim((*claim).into())
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mshogi_engine::{ExecutionPurpose, ExecutionTarget, FullLimits, ReachabilityTarget};
    fn proof() -> (AdjudicatedGame, ExecutionContext, ChessResult) {
        let game =
            AdjudicatedGame::from_fen("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let context = ExecutionContext {
            request_id: 1,
            session_version: 1,
            game: CanonicalRecord::chess(game.record()),
            limits: mshogi_engine::FULL_SENTINEL,
            purpose: ExecutionPurpose::ChessFullReachability {
                target: ExecutionTarget {
                    selection_id: 1,
                    event_index: 0,
                    board_ply: 0,
                    session_version: 1,
                    label: "root".into(),
                    turn: NativeSide::Chess(Color::White),
                },
                mate: ReachabilityTarget::White,
                limits: FullLimits {
                    max_positions: 10,
                    max_transitions: 10,
                },
            },
        };
        let result = ChessResult::Reachability {
            result: MateReachability::Possible {
                witness: vec!["g6g7".parse().unwrap()],
                work: ReachabilityWork {
                    positions: 2,
                    expanded: 1,
                    transitions: 1,
                    static_leaves: 0,
                },
            },
        };
        (game, context, result)
    }
    #[test]
    fn full_witness_requires_the_requested_legal_mate_and_work_evidence() {
        let (game, mut context, result) = proof();
        assert!(validate_result(&result, &context, &game, &mut || false).is_ok());
        let ExecutionPurpose::ChessFullReachability { mate, .. } = &mut context.purpose else {
            unreachable!()
        };
        *mate = ReachabilityTarget::Black;
        assert!(validate_result(&result, &context, &game, &mut || false).is_err());
        let (_, context, mut invalid) = proof();
        let ChessResult::Reachability {
            result: MateReachability::Possible { witness, .. },
        } = &mut invalid
        else {
            unreachable!()
        };
        witness[0] = "g6g5".parse().unwrap();
        assert!(validate_result(&invalid, &context, &game, &mut || false).is_err());
        let (_, context, mut invalid) = proof();
        let ChessResult::Reachability {
            result: MateReachability::Possible { work, .. },
        } = &mut invalid
        else {
            unreachable!()
        };
        work.positions = 11;
        assert!(validate_result(&invalid, &context, &game, &mut || false).is_err());
    }
    #[test]
    fn full_validation_stops_during_witness_check_and_refuses_a_moves_purpose() {
        let (game, mut context, result) = proof();
        let mut checks = 0;
        assert!(
            validate_result(&result, &context, &game, &mut || {
                checks += 1;
                checks >= 3
            })
            .is_err()
        );
        assert_eq!(checks, 3);
        let ExecutionPurpose::ChessFullReachability { target, .. } = context.purpose else {
            unreachable!()
        };
        context.purpose = ExecutionPurpose::MoveAnalysis { target };
        assert!(validate_result(&result, &context, &game, &mut || false).is_err());
    }
    #[test]
    fn minimax_continuations_are_native_and_cannot_be_full_proofs() {
        let (game, mut context, _) = proof();
        let result = ChessResult::Move {
            action: "g6g7".parse().unwrap(),
            telemetry: mshogi_engine::Telemetry {
                score: Some(999_999),
                completed_depth: 1,
                selective_depth: 1,
                nodes: 1,
                principal_variation: vec![mshogi_engine::EngineChoice::Move {
                    action: "g6g7".parse().unwrap(),
                }],
                status: mshogi_engine::StopReason::Complete,
            },
        };
        assert!(validate_result(&result, &context, &game, &mut || false).is_err());
        context.purpose = ExecutionPurpose::EngineTurn {};
        context.limits = WorkLimits {
            depth: 1,
            nodes: 1,
            time_ms: None,
        };
        assert!(validate_result(&result, &context, &game, &mut || false).is_ok());
        let mut invalid = result;
        let ChessResult::Move { action, .. } = &mut invalid else {
            unreachable!()
        };
        *action = "g6g5".parse().unwrap();
        assert!(validate_result(&invalid, &context, &game, &mut || false).is_err());
    }
}
