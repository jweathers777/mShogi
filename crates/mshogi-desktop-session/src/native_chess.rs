//! Chess rules boundary for the shared document and session.
use super::*;
use crate::chess_record::{ChessCommand, ChessRecord};
use crate::native_game::*;

/// Native Chess match behind the generic desktop document.
pub struct Chess;

fn invalid(error: impl ToString) -> RecordFailure {
    RecordFailure::Invalid(error.to_string())
}
fn native_failure(error: RecordFailure) -> NativeFailure {
    match error {
        RecordFailure::Stopped => NativeFailure::Stopped,
        RecordFailure::Capacity(message) => {
            NativeFailure::Capacity(NativeResource::Counter, message)
        }
        RecordFailure::Invalid(message) => NativeFailure::Invalid(message),
    }
}
fn record_side(record: &GameRecord) -> Color {
    let initial = record.initial_position().side_to_move();
    if record
        .events()
        .iter()
        .filter(|e| matches!(e.kind, RecordEventKind::Move(_)))
        .count()
        % 2
        == 0
    {
        initial
    } else {
        initial.opponent()
    }
}
impl NativeGame for Chess {
    const NAME: &'static str = "Chess";
    const KIND: GameKind = GameKind::Chess;
    const SCOPE: ResourceScope = ResourceScope::Chess;
    const SIDES: [Color; 2] = [Color::White, Color::Black];
    const PLAY_LIMITS: WorkLimits = crate::PLAY_LIMITS;
    const MAX_DEPTH: u16 = 128;
    const MAX_NODES: u64 = crate::PLAY_NODE_CEILING;
    const ANALYSIS_BUDGETS: bool = true;
    const ANALYSIS_DEFAULTS: AnalysisOperation = AnalysisOperation {
        depth: 4,
        nodes: 100_000,
    };
    type Side = Color;
    type Command = MatchCommand;
    type Action = Action;
    type Game = ChessMatch;
    type Record = ChessRecord;
    type EngineResult = ChessResult;
    fn native_side(side: Color) -> NativeSide {
        NativeSide::Chess(side)
    }
    fn parse_record(
        text: &str,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(ChessRecord, ChessMatch), RecordFailure> {
        ChessRecord::decode(text, stop)
    }
    fn new_record(_profile: RulesProfile) -> Result<ChessRecord, String> {
        Ok(ChessRecord::new(
            &AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record(),
        ))
    }
    fn record_of(record: CanonicalRecord) -> Option<ChessRecord> {
        match record {
            CanonicalRecord::Chess(record) => Some(ChessRecord::new(&record)),
            _ => None,
        }
    }
    fn context(game: &ChessMatch) -> Result<CanonicalRecord, RecordFailure> {
        Ok(CanonicalRecord::chess(game.game().record()))
    }
    fn context_side(record: &CanonicalRecord) -> Color {
        let CanonicalRecord::Chess(record) = record else {
            panic!("validated Chess context")
        };
        record_side(record)
    }
    fn profile(_record: &ChessRecord) -> RulesProfile {
        RulesProfile::ChessMatchV1
    }
    fn game_profile(_game: &ChessMatch) -> RulesProfile {
        RulesProfile::ChessMatchV1
    }
    fn event_count(record: &ChessRecord) -> usize {
        record
            .initial_record()
            .expect("admitted initial record")
            .events()
            .len()
            + record.commands().len()
    }
    fn record_side_to_move(record: &ChessRecord) -> Color {
        let initial = record_side(&record.initial_record().expect("admitted initial record"));
        if record
            .commands()
            .iter()
            .filter(|e| matches!(e.command, MatchCommand::Move(_)))
            .count()
            % 2
            == 0
        {
            initial
        } else {
            initial.opponent()
        }
    }
    fn to_game(
        record: &ChessRecord,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<ChessMatch, RecordFailure> {
        record.replay(stop)
    }
    fn from_game(game: &ChessMatch) -> Result<ChessRecord, RecordFailure> {
        ChessRecord::from_match(game)
    }
    fn record_prefix(
        game: &ChessMatch,
        count: usize,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<ChessRecord, RecordFailure> {
        if stop() {
            return Err(RecordFailure::Stopped);
        }
        let record = Self::from_game(game)?;
        let initial = record.initial_record()?;
        if count < initial.events().len() {
            return initial
                .prefix(count)
                .map(|initial| ChessRecord::new(&initial))
                .ok_or_else(|| invalid("Invalid imported Chess history prefix"));
        }
        record
            .prefix(count - initial.events().len())
            .ok_or_else(|| invalid("Invalid Chess command prefix"))
    }
    fn submit(
        game: &mut ChessMatch,
        actor: Color,
        command: MatchCommand,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), NativeFailure> {
        if stop() {
            return Err(NativeFailure::Stopped);
        }

        // Native rejected requests are audit events. Publish only an accepted
        // candidate, so they cannot enter the durable command list.
        let mut candidate = game.clone();
        ChessRecord::submit(&mut candidate, &ChessCommand { actor, command }, stop)
            .map_err(native_failure)?;
        *game = candidate;
        Ok(())
    }
    fn events(game: &ChessMatch) -> usize {
        let record = game.record();
        record
            .initial_record()
            .expect("admitted initial history")
            .events()
            .len()
            + record
                .events()
                .filter(|(r, _)| {
                    !matches!(
                        r.command,
                        MatchCommand::Start | MatchCommand::Observe { .. }
                    )
                })
                .count()
    }
    fn side_to_move(game: &ChessMatch) -> Color {
        game.game().position().side_to_move()
    }
    fn concluded(game: &ChessMatch) -> bool {
        game.conclusion().is_some()
    }
    fn resign() -> MatchCommand {
        MatchCommand::Resign { witness: None }
    }
    fn failure(result: &ExecutionResult) -> Option<&mshogi_engine::EngineFailure> {
        match result {
            ExecutionResult::Failure { failure }
            | ExecutionResult::Chess {
                result: ChessResult::Failure { failure },
            } => Some(failure),
            _ => None,
        }
    }
    fn engine_result(result: &ExecutionResult) -> Option<&ChessResult> {
        match result {
            ExecutionResult::Chess { result } => Some(result),
            _ => None,
        }
    }
    fn engine_command(result: &ChessResult, game: &ChessMatch) -> Option<(Color, MatchCommand)> {
        match result {
            ChessResult::Move { action, .. } => {
                Some((Self::side_to_move(game), MatchCommand::Move(*action)))
            }
            ChessResult::Claim { claim, .. } => Some((
                claim.actor,
                MatchCommand::Claim {
                    reason: claim.reason,
                    intended: claim.intended,
                },
            )),
            _ => None,
        }
    }
    fn validate_result(
        result: &ChessResult,
        context: &ExecutionContext,
        game: &ChessMatch,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), String> {
        crate::chess_observation::validate_result(result, context, game.game(), stop)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_document::ValidatedGameDocument;
    #[test]
    fn chess_uses_the_same_minimal_document_without_losing_match_commands() {
        let game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        let doc =
            ValidatedGameDocument::<Chess>::new(ChessRecord::new(&game.record()), Color::Black)
                .unwrap();
        let doc = doc
            .with_command(
                Color::White,
                MatchCommand::Resign { witness: None },
                &mut || false,
            )
            .unwrap();
        let (decoded, game) =
            ValidatedGameDocument::<Chess>::decode_with_game(doc.bytes(), &mut || false).unwrap();
        assert_eq!(decoded.bytes(), doc.bytes());
        assert_eq!(decoded.record(), doc.record());
        assert_eq!(decoded.human(), Color::Black);
        assert!(matches!(
            game.conclusion(),
            Some(MatchConclusion::Unresolved { .. })
        ));
        assert_eq!(Chess::context(&game).unwrap().event_count(), 0);
        assert_eq!(Chess::events(&game), 1);
    }
}
