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

impl DesktopController {
    /// Verification entry into the real native Chu request path.
    #[cfg(any(test, feature = "desktop-verification"))]
    pub fn verify_chu_execution(
        &mut self,
        limits: WorkLimits,
        analysis: bool,
    ) -> Result<ExecutionContext, String> {
        self.verify_game_execution::<Chu>(limits, analysis)
    }
}

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

#[cfg(test)]
mod stop_tests {
    use super::*;

    /// A record with a few played events, and a document that browses one.
    fn played() -> c::GameRecord {
        played_n(6)
    }
    fn played_n(n: usize) -> c::GameRecord {
        let mut g = c::Game::standard();
        for _ in 0..n {
            let action = g.legal_actions()[0].clone();
            let actor = g.position().side_to_move();
            g.submit(c::GameRequest {
                expected: g.context(),
                actor,
                command: c::GameCommand::Play { action },
            })
            .unwrap();
        }
        c::GameRecord::from_game(&g).unwrap()
    }

    /// Every stop point of `run` refuses; a run that is never stopped succeeds.
    fn stops_everywhere<T>(mut run: impl FnMut(&mut dyn FnMut() -> bool) -> Result<T, String>) {
        let mut calls = 0;
        assert!(
            run(&mut || {
                calls += 1;
                false
            })
            .is_ok()
        );
        assert!(calls > 2, "the path checks the stop more than at its ends");
        for limit in 0..calls {
            let mut seen = 0;
            let stopped = run(&mut || {
                seen += 1;
                seen > limit
            });
            assert!(stopped.is_err(), "stop after {limit} of {calls} checks");
        }
    }

    /// The stop is checked during replay: a longer history has more checks.
    #[test]
    fn chu_restore_checks_the_stop_per_replayed_event() {
        let checks = |n| {
            let document =
                crate::chu_document::ValidatedChuDocument::new(played_n(n), c::Side::Sente)
                    .unwrap();
            let mut calls = 0;
            ChuSession::restore(document, &mut || {
                calls += 1;
                false
            })
            .unwrap();
            calls
        };
        assert!(checks(8) >= checks(2) + 6);
    }

    #[test]
    fn chu_restore_and_browse_stop_at_every_check() {
        let document =
            crate::chu_document::ValidatedChuDocument::new(played(), c::Side::Sente).unwrap();
        let browsed = document.with_selection(Some(3), &mut || false).unwrap();
        stops_everywhere(|stop| {
            ChuSession::restore(browsed.clone(), &mut || stop())
                .map(|_| ())
                .map_err(|e| e.diagnostic)
        });
        stops_everywhere(|stop| {
            execute(
                RuntimeWork::<Chu> {
                    seed: RuntimeSeed::Record {
                        record: Box::new(played()),
                        human: c::Side::Sente,
                        play_mode: PlayMode::HumanEngine,
                    },
                    operation: Some(SessionOperation::Inspect { event_index: 2 }),
                    response: None,
                    floor: 1,
                    replacement: false,
                },
                &mut || stop(),
            )
            .map(|_| ())
            .map_err(|e| e.diagnostic)
        });
    }
}
