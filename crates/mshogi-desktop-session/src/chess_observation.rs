//! Validate native Chess search and Full analysis evidence.
use super::*;
use crate::persistence::check;

pub(crate) fn validate_result(
    observation: &ExecutionObservation,
    application_error: Option<&str>,
    game: &AdjudicatedGame,
    witness_verified: bool,
    stopped: &mut impl FnMut() -> bool,
) -> Result<(), String> {
    use mshogi_engine::{EngineChoice, ExecutionPurpose, StopReason};
    let context = &observation.context;
    let native_result =
        chess_observation_result(observation).ok_or("Wrong native observation result family.")?;
    let result = &native_result;
    let full = matches!(
        context.purpose,
        ExecutionPurpose::ChessFullReachability { .. }
    );
    // Rejected native choices remain explicit historical failure evidence.
    if let Some(message) = application_error {
        if message.is_empty() || message.len() > mshogi_engine::MAX_STDERR_BYTES {
            return Err("Invalid observation application diagnostic.".into());
        }
        let invalid = match result {
            ChessResult::Move { action, .. } if !full => game.clone().apply(*action).is_err(),
            ChessResult::Claim { claim, .. } if !full => {
                game.clone().claim((*claim).into()).is_err()
            }
            ChessResult::Finished { outcome, .. } if !full => game.outcome() != Some(*outcome),
            ChessResult::Reachability { .. } => !full,
            _ => full,
        };
        return if invalid {
            Ok(())
        } else {
            Err("Observation marks a valid choice as an application failure.".into())
        };
    }
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
                if !witness_verified {
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
                        Err("mating witness verification interrupted") => return Err("Stored witness verification interrupted by the operation budget/cancellation.".into()),
                        _ => {},
                    }
                    }
                    check(stopped)?;
                    if !valid {
                        return Err(
                        "Stored cooperative witness does not establish the requested legal mate."
                            .into(),
                    );
                    }
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
