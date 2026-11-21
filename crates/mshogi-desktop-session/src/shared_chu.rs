//! Chu in the sole desktop runtime: the generic native session (`native_session`)
//! with the Chu presentation. Rules and persistence stay native-owned.
use super::native_session::*;
use super::*;
use crate::game_document::ValidatedGameDocument;
pub(crate) use crate::native_game::Chu;
use mshogi_core::chu as c;
use std::sync::Arc;

/// The Chu session.
pub(crate) type ChuSession = NativeSession<Chu>;

large_shogi_session!(Chu, chu, ChuResult, CHU_PLAY_LIMITS, CHU_MAX_DEPTH, CHU_MAX_NODES,
operation: Chu, preferences: ChuPreferences, runtime: ChuRuntime, save: ChuSave,
state: chu, chu_mut, field: chu,
    board: 12, "Chu Shogi 12×12 board", "6g",
    score_meaning: "Native root-side evaluation in hundredths of a Pawn. Win/loss distances count complete board actions, not necessarily mate. Failures and incomplete work are unscored.",
pv: |a| format!(
    "{}→{}{}",
    a.from,
    a.route.iter().map(ToString::to_string).collect::<Vec<_>>().join("→"),
    if a.promote { " (promote)" } else { "" }
));

#[path = "shared_chu_presentation.rs"]
mod presentation;
pub use presentation::Presentation as ChuPresentation;
pub(super) fn inspect(game: &c::Game, origin: &str) -> Result<inspection::Inspected, String> {
    presentation::inspect(game, origin)
}

#[cfg(test)]
mod observation_compatibility_tests {
    use super::*;
    #[test]
    fn historical_unsupported_and_search_failures_keep_distinct_unscored_meanings() {
        let record = c::GameRecord::from_game(&c::Game::standard()).unwrap();
        for (kind, label) in [
            (
                mshogi_engine::FailureKind::UnsupportedMode,
                "Unsupported execution · unscored",
            ),
            (
                mshogi_engine::FailureKind::Search,
                "Search could not complete · unscored",
            ),
            (
                mshogi_engine::FailureKind::Start,
                "Engine failure · unscored",
            ),
        ] {
            // These transport limits were permitted for MSH-78's unsupported
            // requests. Restoring their historical failures must not reinterpret
            // them as a real current Chu search or lose their native meaning.
            let observation = ExecutionObservation::new(
                ExecutionContext {
                    purpose: ExecutionPurpose::EngineTurn {},
                    request_id: 1,
                    session_version: 1,
                    process_generation: 1,
                    game: CanonicalRecord::chu(record.clone()),
                    limits: WorkLimits {
                        time_ms: None,
                        depth: 8,
                        nodes: 5_000_000,
                    },
                },
                ExecutionResult::Failure {
                    failure: mshogi_engine::EngineFailure {
                        kind,
                        message: "Historical native failure".into(),
                    },
                },
            );
            let doc =
                crate::chu_document::ValidatedChuDocument::new(record.clone(), c::Side::Sente)
                    .unwrap()
                    .with_observation(observation.clone(), &mut || false)
                    .unwrap();
            let decoded =
                crate::chu_document::ValidatedChuDocument::decode(doc.bytes(), &mut || false)
                    .unwrap();
            let state = ChuSession::restore(decoded, &mut || false).unwrap();
            let view = display_observation(0, &observation, &state);
            assert_eq!(view.status, label);
            assert_eq!(view.score, None);
            assert!(view.variation.is_empty() && view.historical && !view.selected);
            assert!(state.paused && state.pending.is_none());
            assert_eq!(state.document.record(), &record);
        }
    }
}
