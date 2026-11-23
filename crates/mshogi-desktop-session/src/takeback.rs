//! Atomic human-versus-engine decision restoration; native records remain authoritative.
use super::*;

pub(super) fn target_mode(game: &ChessMatch, human: Color, mode: PlayMode) -> Option<usize> {
    let record = game.record();
    if mode.is_two_humans() {
        return record
            .events()
            .enumerate()
            .filter(|(_, (_, reply))| reply.rejection.is_none())
            .filter_map(|(i, (request, _))| {
                (!matches!(request.command, MatchCommand::Start)).then_some(i)
            })
            .last();
    }
    let last = record
        .events()
        .enumerate()
        .filter(|(_, (_, reply))| reply.rejection.is_none())
        .last();
    if let Some((index, (request, _))) = last
        && request.actor == human
        && game.conclusion().is_some()
        && game.game().position().side_to_move() == human
        && !game.game().position().legal_actions().is_empty()
        && matches!(
            request.command,
            MatchCommand::Resign { .. } | MatchCommand::Claim { .. } | MatchCommand::AcceptDraw
        )
    {
        return Some(index);
    }
    record
        .events()
        .enumerate()
        .filter_map(|(index, (request, reply))| {
            (reply.rejection.is_none()
                && request.actor == human
                && matches!(request.command, MatchCommand::Move(_)))
            .then_some(index)
        })
        .last()
}
