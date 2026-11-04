//! Independently constructed terminal tactics and forged real-result admission.
use mshogi_core::{native::*, tenjiku as t};
use mshogi_engine::*;
use std::io::Cursor;
fn game(profile: t::Profile) -> t::Game {
    let pieces = [
        (16, 16, t::Side::Sente, t::Base::K),
        (8, 8, t::Side::Sente, t::Base::R),
        (8, 3, t::Side::Gote, t::Base::K),
        (2, 2, t::Side::Gote, t::Base::P),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (f, r, s, b))| {
        (
            t::Square::new(f, r).unwrap(),
            t::Piece::new(id as u16, s, b, false).unwrap(),
        )
    })
    .collect::<Vec<_>>();
    t::Game::from_root(
        t::Position::from_diagram(
            profile,
            &pieces,
            t::Side::Sente,
            t::RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap()
}
fn context(g: &t::Game) -> ExecutionContext {
    ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {},
        request_id: 1,
        session_version: 1,
        process_generation: 1,
        game: CanonicalRecord::tenjiku(t::GameRecord::from_game(g).unwrap()),
        limits: TENJIKU_PLAY_LIMITS,
    }
}
fn run(ctx: ExecutionContext) -> ExecutionObservation {
    let request = ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ctx,
    };
    let mut bytes = vec![];
    serve_once(
        Cursor::new(format!("{}\n", serde_json::to_string(&request).unwrap())),
        &mut bytes,
    )
    .unwrap();
    let response: ExecutionResponse = serde_json::from_slice(&bytes).unwrap();
    assert!(response.matches(&request));
    response.observation
}
#[test]
fn actual_royal_capture_score_and_native_pv_validation_for_both_profiles() {
    for profile in t::Profile::ALL {
        let g = game(profile);
        let before = t::GameRecord::from_game(&g).unwrap();
        let observation = run(context(&g));
        let ExecutionResult::Tenjiku { result } = observation.result else {
            panic!("real result required")
        };
        let TenjikuResult::Choice { action, telemetry } = &result else {
            panic!("completed choice required")
        };
        assert_eq!(action.from, t::Square::new(8, 8).unwrap());
        assert_eq!(action.route.last(), Some(&t::Square::new(8, 3).unwrap()));
        assert_eq!(telemetry.score, Some(mshogi_search::tenjiku::WIN_SCORE - 1));
        assert_eq!(telemetry.perspective, t::Side::Sente);
        assert_eq!(telemetry.principal_variation, vec![action.clone()]);
        let mut branch = before.to_game().unwrap();
        let start = branch.replay_work();
        for a in &telemetry.principal_variation {
            branch
                .submit(t::GameRequest {
                    expected: branch.context(),
                    actor: branch.position().side_to_move(),
                    command: t::GameCommand::Play { action: a.clone() },
                })
                .unwrap();
        }
        let expected_work = before.reconstruction_work() + branch.replay_work() - start;
        let mut polls = 0;
        assert_eq!(
            result
                .validate_with_work(&g, &mut || {
                    polls += 1;
                    false
                })
                .unwrap(),
            expected_work
        );
        for at in [1, polls / 2, polls - 1] {
            let mut n = 0;
            assert!(
                result
                    .validate_with_work(&g, &mut || {
                        n += 1;
                        n >= at
                    })
                    .is_err()
            );
            assert_eq!(t::GameRecord::from_game(&g).unwrap(), before);
        }
        assert_eq!(t::GameRecord::from_game(&g).unwrap(), before);
        for field in 0..6 {
            let mut forged = result.clone();
            let TenjikuResult::Choice { action, telemetry } = &mut forged else {
                unreachable!()
            };
            match field {
                0 => telemetry.score = Some(123),
                1 => telemetry.perspective = t::Side::Gote,
                2 => telemetry.evaluator_id.push_str("forged"),
                3 => telemetry.principal_variation[0].route.clear(),
                4 => telemetry.completed_depth = 0,
                _ => action.route.clear(),
            }
            assert!(forged.validate(&g).is_err(), "forgery {field}");
        }
        let mut terminal = before.to_game().unwrap();
        terminal
            .submit(t::GameRequest {
                expected: terminal.context(),
                actor: t::Side::Sente,
                command: t::GameCommand::Play {
                    action: action.clone(),
                },
            })
            .unwrap();
        assert_eq!(
            terminal.conclusion(),
            Some(t::Conclusion::RoyalCapture {
                winner: t::Side::Sente
            })
        );
        let ExecutionResult::Tenjiku { result: finished } = run(context(&terminal)).result else {
            panic!()
        };
        assert!(
            matches!(&finished,TenjikuResult::Finished{telemetry,..} if telemetry.score==Some(-mshogi_search::tenjiku::WIN_SCORE))
        );
        finished.validate(&terminal).unwrap();
    }
}
#[test]
fn no_completed_iteration_publishes_no_fallback_and_limits_are_enforced() {
    for profile in t::Profile::ALL {
        let g = game(profile);
        let mut ctx = context(&g);
        ctx.limits.nodes = 0;
        let ExecutionResult::Tenjiku { result } = run(ctx.clone()).result else {
            panic!()
        };
        assert!(
            matches!(&result,TenjikuResult::Incomplete{telemetry} if telemetry.score.is_none() && telemetry.nodes==0 && telemetry.principal_variation.is_empty())
        );
        result.validate(&g).unwrap();
        ctx.limits.depth = TENJIKU_MAX_DEPTH + 1;
        assert!(
            matches!(run(ctx).result,ExecutionResult::Failure{failure} if failure.kind==FailureKind::InvalidRequest)
        );
    }
}

#[test]
fn sourced_no_board_actions_is_an_unscored_search_failure_not_a_draw() {
    for profile in t::Profile::ALL {
        let entries = [
            (1, 1, t::Side::Sente, t::Base::K),
            (2, 1, t::Side::Sente, t::Base::P),
            (1, 2, t::Side::Sente, t::Base::L),
            (2, 2, t::Side::Sente, t::Base::N),
            (16, 16, t::Side::Gote, t::Base::K),
            (15, 14, t::Side::Gote, t::Base::P),
        ]
        .into_iter()
        .enumerate()
        .map(|(id, (f, r, s, b))| {
            (
                t::Square::new(f, r).unwrap(),
                t::Piece::new(id as u16, s, b, false).unwrap(),
            )
        })
        .collect::<Vec<_>>();
        let g = t::Game::from_root(
            t::Position::from_diagram(
                profile,
                &entries,
                t::Side::Sente,
                t::RootMetadata::diagram(0),
            )
            .unwrap(),
        )
        .unwrap();
        assert_eq!(g.status(), t::GameStatus::NoBoardActions);
        let before = t::GameRecord::from_game(&g).unwrap();
        let observation = run(context(&g));
        assert!(
            matches!(observation.result,ExecutionResult::Failure{failure} if failure.kind==FailureKind::Search)
        );
        assert_eq!(g.conclusion(), None);
        assert_eq!(t::GameRecord::from_game(&g).unwrap(), before);
    }
}
