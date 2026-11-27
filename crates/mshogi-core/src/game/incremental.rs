//! Authoritative incremental sessions, independent of engine transport.
//!
//! A transport must bind an advance to the exact previously acknowledged
//! session identity. Event counts alone do not identify history. Full roots,
//! takebacks, imports and mismatched histories use `reset`, never a delta.

/// Adapter for one game's already validated full record and typed suffix.
/// Other games adopt this lifecycle by implementing this trait, without adding
/// another session implementation. Saved-record formats are not changed.
pub trait IncrementalRecord {
    /// Rules-owned game, including history and outcomes.
    type Game;
    /// Strictly decoded, but not yet rules-admitted, suffix.
    type Delta;
    /// Reconstruction or suffix-validation failure.
    type Error;

    /// Reconstruct a full root through rules.
    fn reconstruct(&self) -> Result<Self::Game, Self::Error>;
    /// Apply only new commands and check every claimed effect and result.
    /// Failure must leave the complete prior game unchanged.
    fn append(game: &mut Self::Game, delta: &Self::Delta) -> Result<(), Self::Error>;
}

/// A suffix cannot be accepted without an exact retained base.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdvanceError<E> {
    /// No full root has been admitted, or the session was cleared.
    MissingRoot,
    /// Game command or redundant metadata validation failed atomically.
    Rejected(E),
}

impl<E: std::fmt::Display> std::fmt::Display for AdvanceError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRoot => f.write_str("incremental session has no admitted root"),
            Self::Rejected(error) => write!(f, "suffix rejected: {error}"),
        }
    }
}
impl<E: std::error::Error + 'static> std::error::Error for AdvanceError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MissingRoot => None,
            Self::Rejected(error) => Some(error),
        }
    }
}

/// One retained authoritative game. Process/request identity belongs to the
/// transport; rules validation and atomic state changes belong here.
pub struct Session<R: IncrementalRecord> {
    game: Option<R::Game>,
}
impl<R: IncrementalRecord> Default for Session<R> {
    fn default() -> Self {
        Self { game: None }
    }
}
impl<R: IncrementalRecord> Session<R> {
    /// Empty session; an advance is refused until a full reset succeeds.
    pub fn new() -> Self {
        Self::default()
    }
    /// Borrow the admitted game without serializing or copying its history.
    pub fn game(&self) -> Option<&R::Game> {
        self.game.as_ref()
    }
    /// Full rules validation, used for new games, takebacks and imported roots.
    /// A failure preserves the old game; callers may explicitly clear it.
    pub fn reset(&mut self, record: &R) -> Result<&R::Game, R::Error> {
        let game = record.reconstruct()?;
        self.game = Some(game);
        Ok(self.game.as_ref().expect("just installed game"))
    }
    /// Admit an exact-base suffix. The transport must authenticate that base
    /// before calling; this method does not treat a matching count as identity.
    pub fn advance(&mut self, delta: &R::Delta) -> Result<&R::Game, AdvanceError<R::Error>> {
        let game = self.game.as_mut().ok_or(AdvanceError::MissingRoot)?;
        R::append(game, delta).map_err(AdvanceError::Rejected)?;
        Ok(game)
    }
    /// Forget a stopped/restarted or otherwise invalidated session.
    pub fn clear(&mut self) {
        self.game = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shogi::{
        CounterSource, Game, GameCommand, GameDelta, GameRecord, GameRequest, GameRoot, Position,
        RootCategory, Side,
    };
    use serde_json::Value;

    fn play(game: &mut Game, text: &str) {
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play(text.parse().unwrap()),
        })
        .unwrap();
    }
    fn delta_value(record: &GameRecord, start: usize) -> Value {
        serde_json::from_str(&record.suffix(start).unwrap().to_json().unwrap()).unwrap()
    }
    fn altered(value: Value) -> GameDelta {
        GameDelta::from_json(&value.to_string()).unwrap()
    }

    #[test]
    fn exact_suffix_replays_new_commands_and_matches_complete_game() {
        let mut game = Game::new();
        let initial = GameRecord::from_game(&game);
        let mut session = Session::<GameRecord>::new();
        assert_eq!(
            session.advance(&initial.suffix(0).unwrap()),
            Err(AdvanceError::MissingRoot)
        );
        session.reset(&initial).unwrap();
        let mut previous = initial;
        let mut seed = 189u64;
        for _ in 0..160 {
            let actions = game.position().legal_actions();
            if game.conclusion().is_some() || actions.is_empty() {
                break;
            }
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let action = actions[seed as usize % actions.len()];
            game.submit(GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: GameCommand::Play(action),
            })
            .unwrap();
            let next = GameRecord::from_game(&game);
            assert!(next.has_prefix(&previous));
            let delta = GameDelta::from_json(
                &next
                    .suffix(previous.event_count())
                    .unwrap()
                    .to_json()
                    .unwrap(),
            )
            .unwrap();
            assert_eq!(delta.event_count(), 1);
            assert_eq!(delta.end_event(), Some(next.event_count()));
            assert_eq!(session.advance(&delta).unwrap(), &game);
            assert_eq!(GameRecord::from_game(session.game().unwrap()), next);
            previous = next;
        }
        assert!(game.events().len() > 40);
        session.clear();
        assert!(session.game().is_none());
        assert_eq!(
            session.advance(&previous.suffix(previous.event_count()).unwrap()),
            Err(AdvanceError::MissingRoot)
        );
    }

    #[test]
    fn invalid_suffix_effects_commands_and_final_result_leave_exact_base_unchanged() {
        let mut game = Game::new();
        let base = game.clone();
        let initial = GameRecord::from_game(&game);
        for action in ["7g7f", "3c3d", "2g2f"] {
            play(&mut game, action);
        }
        let next = GameRecord::from_game(&game);
        let good = delta_value(&next, 0);
        let mut variants = vec![];
        let mut v = good.clone();
        v["events"][1]["actor"] = Value::String("sente".into());
        variants.push(v);
        let mut v = good.clone();
        v["events"][1]["command"]["action"] = Value::String("3c3e".into());
        variants.push(v);
        let mut v = good.clone();
        v["events"][1]["after_sfen"] = Value::String("9/9/9/9/9/9/9/9/9 b - 1".into());
        variants.push(v);
        let mut v = good.clone();
        v["events"][1]["gave_check"] = Value::Bool(true);
        variants.push(v);
        let mut v = good.clone();
        v["events"][1]["score_half_points"] = serde_json::json!([2, 0]);
        variants.push(v);
        let mut v = good.clone();
        v["score_half_points"] = serde_json::json!([2, 0]);
        variants.push(v);
        let mut v = good.clone();
        v["events"][1]["index"] = serde_json::json!(9);
        variants.push(v);
        let mut v = good.clone();
        v["start_event"] = serde_json::json!(1);
        variants.push(v);
        let mut session = Session::<GameRecord>::new();
        session.reset(&initial).unwrap();
        for value in variants {
            assert!(session.advance(&altered(value)).is_err());
            assert_eq!(session.game(), Some(&base));
        }
        // A failure after an accepted prefix also restores hashes, repetition,
        // check history and conclusion, not just the current board.
        assert_eq!(session.advance(&altered(good)).unwrap(), &game);
        assert!(session.advance(&next.suffix(0).unwrap()).is_err());
        assert_eq!(session.game(), Some(&game));
    }

    #[test]
    fn strict_delta_decoding_rejects_unknown_duplicate_and_missing_fields() {
        let text = GameRecord::from_game(&Game::new())
            .suffix(0)
            .unwrap()
            .to_json()
            .unwrap();
        assert!(GameDelta::from_json(&text.replacen('{', "{\"start_event\":0,", 1)).is_err());
        assert!(GameDelta::from_json(&text.replacen('{', "{\"extra\":0,", 1)).is_err());
        let mut value: Value = serde_json::from_str(&text).unwrap();
        value.as_object_mut().unwrap().remove("conclusion");
        assert!(GameDelta::from_json(&value.to_string()).is_err());
        let mut value: Value = serde_json::from_str(&text).unwrap();
        value["start_event"] = serde_json::json!(322);
        assert!(GameDelta::from_json(&value.to_string()).is_ok());
        let mut game = Game::new();
        play(&mut game, "7g7f");
        let text = GameRecord::from_game(&game)
            .suffix(0)
            .unwrap()
            .to_json()
            .unwrap();
        assert!(
            GameDelta::from_json(&text.replace(
                "\"actor\":\"sente\"",
                "\"actor\":\"sente\",\"actor\":\"gote\""
            ))
            .is_err()
        );
    }

    #[test]
    fn terminal_events_and_metadata_are_checked_without_board_move_assumptions() {
        for command in [GameCommand::Resign, GameCommand::DeclareEnteringKing] {
            let mut game = Game::new();
            let initial = GameRecord::from_game(&game);
            game.submit(GameRequest {
                expected: game.context(),
                actor: Side::Gote,
                command,
            })
            .unwrap();
            let record = GameRecord::from_game(&game);
            let delta = record.suffix(0).unwrap();
            let mut session = Session::<GameRecord>::new();
            session.reset(&initial).unwrap();
            let mut wrong = delta_value(&record, 0);
            wrong["events"][0]["score_half_points"] = serde_json::json!([1, 1]);
            assert!(session.advance(&altered(wrong)).is_err());
            assert_eq!(session.game(), Some(&Game::new()));
            assert_eq!(session.advance(&delta).unwrap(), &game);
            assert_eq!(session.advance(&record.suffix(1).unwrap()).unwrap(), &game);
        }
    }

    #[test]
    fn reset_handles_takeback_newgame_import_and_unrelated_history() {
        let mut game = Game::new();
        let mut session = Session::<GameRecord>::new();
        for action in ["7g7f", "3c3d", "2g2f"] {
            play(&mut game, action);
        }
        let current = GameRecord::from_game(&game);
        session.reset(&current).unwrap();
        let takeback = game.prefix(1).unwrap();
        let record = GameRecord::from_game(&takeback);
        assert!(!record.has_prefix(&current));
        assert_eq!(session.reset(&record).unwrap(), &takeback);
        let mut other = Game::new();
        play(&mut other, "2g2f");
        let record = GameRecord::from_game(&other);
        assert!(!record.has_prefix(&GameRecord::from_game(&takeback)));
        assert_eq!(session.reset(&record).unwrap(), &other);
        let imported = Game::from_root(
            GameRoot::import(
                Position::from_sfen("4k4/9/9/9/9/9/9/9/4K4 b G 1").unwrap(),
                RootCategory::ReducedMaterialDiagram,
                CounterSource::Supplied,
            )
            .unwrap(),
        );
        assert_eq!(
            session.reset(&GameRecord::from_game(&imported)).unwrap(),
            &imported
        );
        assert_eq!(
            session.reset(&GameRecord::from_game(&Game::new())).unwrap(),
            &Game::new()
        );
    }

    #[test]
    fn repetition_and_declared_length_outcomes_match_full_admission() {
        for (sfen, actions) in [
            (
                "4k4/9/9/9/9/9/9/9/4K4 b - 1",
                ["5i6i", "5a6a", "6i5i", "6a5a"].repeat(3),
            ),
            ("4k4/9/9/9/9/9/9/9/4K4 b - 319", vec!["5i6i", "5a6a"]),
            (
                "4k4/9/4R4/9/9/9/9/9/K8 w - 1",
                ["5a4a", "5c4c", "4a5a", "4c5c"].repeat(3),
            ),
        ] {
            let mut game = Game::from_root(
                GameRoot::import(
                    Position::from_sfen(sfen).unwrap(),
                    RootCategory::ReducedMaterialDiagram,
                    CounterSource::Supplied,
                )
                .unwrap(),
            );
            let mut session = Session::<GameRecord>::new();
            session.reset(&GameRecord::from_game(&game)).unwrap();
            for action in actions {
                let start = game.events().len();
                play(&mut game, action);
                let record = GameRecord::from_game(&game);
                assert_eq!(
                    session.advance(&record.suffix(start).unwrap()).unwrap(),
                    &game
                );
            }
            assert!(game.conclusion().is_some());
        }
    }
}
