//! Atomic human-versus-engine decision restoration; native records remain authoritative.
use super::*;

pub(super) fn target(game: &ChessMatch, human: Color) -> Option<usize> {
    let record = game.record();
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

pub(super) fn restore(
    record: &MatchRecord,
    human: Color,
    stopped: &mut impl FnMut() -> bool,
) -> Result<ChessMatch, String> {
    let before = record
        .replay_with_stop(&mut *stopped)
        .map_err(|e| e.to_string())?;
    let index = target(&before, human).ok_or("No earlier playable human decision is available.")?;
    let restored = record
        .prefix(index)
        .ok_or("Invalid native takeback boundary.")?
        .replay_with_stop(stopped)
        .map_err(|e| e.to_string())?;
    if restored.conclusion().is_some()
        || restored.game().position().side_to_move() != human
        || restored.game().position().legal_actions().is_empty()
    {
        return Err("The preceding native decision is not playable by the human.".into());
    }
    Ok(restored)
}

impl DesktopSession {
    pub(super) fn take_back_target(&self) -> Option<usize> {
        target(&self.game, self.human)
    }
    pub(super) fn take_back(&mut self, _at_ns: u64) -> Result<(), String> {
        self.take_back_with_stop(&mut || false)
    }
    pub(super) fn take_back_with_stop(
        &mut self,
        stopped: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        if self.viewed.is_some() {
            return Err("Return to live before Take back.".into());
        }
        self.browse_running = false;
        let before = self.game.record().clone();
        let restored = restore(&before, self.human, stopped)?;
        self.takebacks.push(persistence::TakeBackEvidence {
            control_sequence: self.controls.len() as u64 + 1,
            match_record_json: before.to_json(),
        });
        self.pending = None;
        self.retire_analysis();
        self.game = restored;
        self.selection_id += 1;
        self.play_paused = true;
        self.engine = EngineState {
            status: EnginePhase::Paused,
            message: Some("Position restored. Resume play when ready.".into()),
            ..EngineState::default()
        };
        for observation in &mut self.observations {
            observation.historical = true;
        }
        self.changed();
        Ok(())
    }
}
