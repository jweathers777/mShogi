//! Durable untimed Chess commands. The native match remains the rules authority;
//! clocks, rejected requests and controller identities are not document data.
use crate::native_game::RecordFailure;
use mshogi_core::chess::*;
use serde::{Deserialize, Serialize};

/// One accepted player command, including claims and draw offers.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChessCommand {
    /// The player who submitted the command.
    pub actor: Color,
    /// Complete native command; Start and Observe are not durable commands.
    pub command: MatchCommand,
}

/// Minimal match history. Initial history retains its origin and claim policy.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ChessRecord {
    initial_record_json: String,
    commands: Vec<ChessCommand>,
}

fn invalid(error: impl ToString) -> RecordFailure {
    RecordFailure::Invalid(error.to_string())
}
fn check(stop: &mut impl FnMut() -> bool) -> Result<(), RecordFailure> {
    if stop() {
        Err(RecordFailure::Stopped)
    } else {
        Ok(())
    }
}
impl ChessRecord {
    /// Retain an imported live history as the immutable document root.
    pub fn new(initial: &GameRecord) -> Self {
        Self {
            initial_record_json: initial.to_json(),
            commands: vec![],
        }
    }
    /// Retain an accepted untimed native match, without its runtime bookkeeping.
    pub fn from_match(game: &ChessMatch) -> Result<Self, RecordFailure> {
        let source = game.record();
        if source.time_control() != TimeControl::Untimed {
            return Err(invalid("Timed matches are not desktop documents."));
        }
        let mut record = Self::new(&source.initial_record().map_err(invalid)?);
        for (request, reply) in source.events() {
            if reply.rejection.is_some() {
                return Err(invalid(
                    "An admitted desktop match cannot contain rejected requests.",
                ));
            }
            if !matches!(
                request.command,
                MatchCommand::Start | MatchCommand::Observe { .. }
            ) {
                record.commands.push(ChessCommand {
                    actor: request.actor,
                    command: request.command.clone(),
                });
            }
        }
        Ok(record)
    }
    /// Accepted commands after the immutable root.
    pub fn commands(&self) -> &[ChessCommand] {
        &self.commands
    }
    /// Parse the initial history without running an adjudication solver.
    pub fn initial_record(&self) -> Result<GameRecord, RecordFailure> {
        GameRecord::from_json(&self.initial_record_json).map_err(invalid)
    }
    /// Restore through the native match API with deterministic untimed requests.
    pub fn replay(&self, stop: &mut impl FnMut() -> bool) -> Result<ChessMatch, RecordFailure> {
        check(stop)?;
        let initial = self.initial_record()?;
        if initial.adjudication_mode() != AdjudicationMode::M2 {
            return Err(invalid(
                "Full adjudication restoration is unsupported by untimed desktop play.",
            ));
        }
        if initial.events().len().saturating_add(self.commands.len()) > crate::MAX_ARCHIVE_EVENTS {
            return Err(RecordFailure::Capacity(
                "Application limit reached: Chess history exceeds 4096 events.".into(),
            ));
        }
        let live = AdjudicatedGame::replay_record_with_stop(&initial, &mut *stop);
        check(stop)?;
        let mut game =
            ChessMatch::new(live.map_err(invalid)?, TimeControl::Untimed).map_err(invalid)?;
        if game.conclusion().is_none() {
            let reply = game.submit(MatchRequest {
                at_ns: 0,
                version: game.version(),
                actor: game.game().position().side_to_move(),
                command: MatchCommand::Start,
            });
            if let Some(rejection) = reply.rejection {
                return Err(invalid(rejection.detail));
            }
        }
        for command in &self.commands {
            check(stop)?;
            Self::submit(&mut game, command, stop)?;
        }
        check(stop)?;
        Ok(game)
    }
    /// Keep a command only after the rules accept it. A refusal is atomic.
    pub fn with_command(
        &self,
        actor: Color,
        command: MatchCommand,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<Self, RecordFailure> {
        let mut game = self.replay(stop)?;
        if self
            .initial_record()?
            .events()
            .len()
            .saturating_add(self.commands.len())
            >= crate::MAX_ARCHIVE_EVENTS
        {
            return Err(RecordFailure::Capacity(
                "Application limit reached: Chess history exceeds 4096 events.".into(),
            ));
        }
        let command = ChessCommand { actor, command };
        Self::submit(&mut game, &command, stop)?;
        let mut record = self.clone();
        record.commands.push(command);
        Ok(record)
    }
    pub(crate) fn submit(
        game: &mut ChessMatch,
        entry: &ChessCommand,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), RecordFailure> {
        check(stop)?;
        if matches!(
            entry.command,
            MatchCommand::Start | MatchCommand::Observe { .. }
        ) {
            return Err(invalid("Clock bookkeeping is not a saved player command."));
        }
        // The desktop has never admitted witnessed match commands. Native
        // submit verifies them synchronously; read-only Full analysis has its
        // separate cooperative validation path.
        if matches!(entry.command, MatchCommand::Resign { witness: Some(_) }) {
            return Err(invalid(
                "Witnessed match commands are unsupported by desktop play.",
            ));
        }
        let reply = game.submit(MatchRequest {
            at_ns: 0,
            version: game.version(),
            actor: entry.actor,
            command: entry.command.clone(),
        });
        check(stop)?;
        if let Some(rejection) = reply.rejection {
            return Err(invalid(rejection.detail));
        }
        Ok(())
    }
    /// Cut the command list at a player decision; imported history stays intact.
    pub fn prefix(&self, count: usize) -> Option<Self> {
        if count > self.commands.len() {
            return None;
        }
        let mut record = self.clone();
        record.commands.truncate(count);
        Some(record)
    }
    /// Decode strictly and retain the native game produced by replay.
    pub fn decode(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(Self, ChessMatch), RecordFailure> {
        check(stop)?;
        crate::document::reject_duplicates(text.as_bytes()).map_err(invalid)?;
        let record: Self = serde_json::from_str(text).map_err(invalid)?;
        let game = record.replay(stop)?;
        Ok((record, game))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn initial() -> ChessRecord {
        ChessRecord::new(&AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record())
    }
    fn play(record: &ChessRecord, actor: Color, action: &str) -> ChessRecord {
        record
            .with_command(
                actor,
                MatchCommand::Move(action.parse().unwrap()),
                &mut || false,
            )
            .unwrap()
    }
    #[test]
    fn moves_offers_and_agreement_replay_without_runtime_data() {
        let record = play(&initial(), Color::White, "e2e4");
        let record = play(&record, Color::Black, "e7e5");
        let record = record
            .with_command(Color::White, MatchCommand::OfferDraw, &mut || false)
            .unwrap();
        let record = record
            .with_command(Color::Black, MatchCommand::AcceptDraw, &mut || false)
            .unwrap();
        let bytes = serde_json::to_string(&record).unwrap();
        let value: serde_json::Value = serde_json::from_str(&bytes).unwrap();
        let keys: Vec<_> = value
            .as_object()
            .unwrap()
            .keys()
            .map(String::as_str)
            .collect();
        assert_eq!(keys, ["commands", "initial_record_json"]);
        for entry in value["commands"].as_array().unwrap() {
            let keys: Vec<_> = entry
                .as_object()
                .unwrap()
                .keys()
                .map(String::as_str)
                .collect();
            assert_eq!(keys, ["actor", "command"]);
        }
        let (decoded, game) = ChessRecord::decode(&bytes, &mut || false).unwrap();
        assert_eq!(decoded, record);
        assert_eq!(
            game.conclusion(),
            Some(MatchConclusion::Draw { trigger: None })
        );
        assert_eq!(game.game().history().ply_count(), 2);
        assert_eq!(
            record
                .prefix(3)
                .unwrap()
                .replay(&mut || false)
                .unwrap()
                .conclusion(),
            None
        );
    }
    #[test]
    fn intended_claim_is_not_a_played_move() {
        let live =
            AdjudicatedGame::from_fen("7k/8/8/8/8/8/R7/K7 w - - 99 1", ClaimPolicy::ExplicitOnly)
                .unwrap();
        let record = ChessRecord::new(&live.record())
            .with_command(
                Color::White,
                MatchCommand::Claim {
                    reason: DrawClaim::FiftyMove,
                    intended: Some("a2b2".parse().unwrap()),
                },
                &mut || false,
            )
            .unwrap();
        let game = record.replay(&mut || false).unwrap();
        assert_eq!(game.game().history().ply_count(), 0);
        assert_eq!(game.game().position(), live.position());
        assert!(matches!(
            game.conclusion(),
            Some(MatchConclusion::Chess(ChessOutcome::Draw(_)))
        ));
        assert_eq!(
            record.prefix(0).unwrap().initial_record().unwrap(),
            live.record()
        );
    }
    #[test]
    fn rejected_commands_and_runtime_events_do_not_enter_the_record() {
        let record = initial();
        for (actor, command) in [
            (Color::Black, MatchCommand::Move("e7e5".parse().unwrap())),
            (Color::White, MatchCommand::Start),
            (Color::White, MatchCommand::Observe { witness: None }),
            (Color::White, MatchCommand::AcceptDraw),
        ] {
            assert!(record.with_command(actor, command, &mut || false).is_err());
        }
        assert!(record.commands().is_empty());
        assert!(
            record
                .with_command(
                    Color::White,
                    MatchCommand::Resign { witness: None },
                    &mut || true
                )
                .is_err()
        );
    }
    #[test]
    fn unresolved_resignation_and_imported_history_survive() {
        let record = initial()
            .with_command(
                Color::White,
                MatchCommand::Resign { witness: None },
                &mut || false,
            )
            .unwrap();
        assert_eq!(
            record.replay(&mut || false).unwrap().conclusion(),
            Some(MatchConclusion::Unresolved {
                actor: Color::White,
                trigger: TerminationTrigger::Resignation
            })
        );
        let mut live = AdjudicatedGame::new(ClaimPolicy::AutoClaim);
        live.apply("e2e4".parse().unwrap()).unwrap();
        let root = ChessRecord::new(&live.record());
        let moved = play(&root, Color::Black, "e7e5");
        let cut = moved.prefix(0).unwrap();
        assert_eq!(cut, root);
        assert_eq!(
            cut.replay(&mut || false).unwrap().game().record(),
            live.record()
        );
    }
    #[test]
    fn malformed_saved_commands_fail_native_replay() {
        let mut record = initial();
        record.commands.push(ChessCommand {
            actor: Color::White,
            command: MatchCommand::Move("e2e5".parse().unwrap()),
        });
        assert!(
            ChessRecord::decode(&serde_json::to_string(&record).unwrap(), &mut || false).is_err()
        );
        let bytes = serde_json::to_string(&initial()).unwrap();
        assert!(ChessRecord::decode(&bytes, &mut || true).is_err());
        assert!(
            ChessRecord::decode(
                &bytes.replace("\"commands\":[]", "\"commands\":[],\"commands\":[]"),
                &mut || false
            )
            .is_err()
        );
    }
}
