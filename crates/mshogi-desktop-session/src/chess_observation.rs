//! Validate Chess search evidence.
use super::*;
use crate::persistence::check;

pub(crate) fn validate_result(
    result: &ChessResult,
    context: &ExecutionContext,
    game: &AdjudicatedGame,
    stopped: &mut impl FnMut() -> bool,
) -> Result<(), String> {
    use mshogi_engine::{EngineChoice, StopReason};
    check(stopped)?;
    let telemetry = match result {
        ChessResult::Failure { failure } => {
            return if failure.message.len() <= mshogi_engine::MAX_STDERR_BYTES {
                Ok(())
            } else {
                Err("Failure diagnostic exceeds its byte limit.".into())
            };
        }
        ChessResult::Move { telemetry, .. }
        | ChessResult::Claim { telemetry, .. }
        | ChessResult::Finished { telemetry, .. }
        | ChessResult::Incomplete { telemetry } => telemetry,
    };
    let bound = telemetry
        .variation_depth_bound()
        .ok_or("Search selective depth exceeds its bound.")?;
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
            return Err("Finished observation disagrees with game root outcome.".into());
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
    #[test]
    fn accepts_the_matching_legal_continuation() {
        let game =
            AdjudicatedGame::from_fen("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let context = ExecutionContext {
            request_id: 1,
            session_version: 1,
            game: CanonicalRecord::chess(game.record()),
            limits: WorkLimits {
                depth: 1,
                nodes: 1,
                time_ms: None,
            },
            purpose: mshogi_engine::ExecutionPurpose::EngineTurn {},
        };
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
        assert!(validate_result(&result, &context, &game, &mut || false).is_ok());
        let mut invalid = result;
        let ChessResult::Move { action, .. } = &mut invalid else {
            unreachable!()
        };
        *action = "g6g5".parse().unwrap();
        assert!(validate_result(&invalid, &context, &game, &mut || false).is_err());
    }
}
