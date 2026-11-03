//! Native routing and actual bounded Chu execution retain complete identities.
use mshogi_core::{chu as t, native::*};
use mshogi_engine::*;
use std::io::Cursor;
fn context() -> ExecutionContext {
    ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {},
        request_id: 129,
        session_version: 7,
        process_generation: 3,
        game: CanonicalRecord::chu(t::GameRecord::from_game(&t::Game::standard()).unwrap()),
        limits: WorkLimits {
            depth: 1,
            nodes: 512,
        },
    }
}
fn run(context: &ExecutionContext) -> ExecutionResponse {
    let mut bytes = vec![];
    serve_once(
        Cursor::new(format!(
            "{}\n",
            serde_json::to_string(&ExecutionRequest {
                protocol: PROTOCOL_VERSION,
                context: context.clone()
            })
            .unwrap()
        )),
        &mut bytes,
    )
    .unwrap();
    serde_json::from_slice(&bytes).unwrap()
}
#[test]
fn real_internal_entry_point_supports_bounded_chu_without_foreign_operations() {
    assert_eq!(
        execution_capabilities(GameKind::Chu),
        &[
            ExecutionOperation::EngineTurn,
            ExecutionOperation::MoveAnalysis
        ]
    );
    let mut c = context();
    for analysis in [false, true] {
        if analysis {
            c.purpose = ExecutionPurpose::MoveAnalysis {
                target: ExecutionTarget {
                    selection_id: 3,
                    event_index: 0,
                    board_ply: 0,
                    session_version: c.session_version,
                    label: "Native Chu root".into(),
                    turn: NativeSide::Chu(t::Side::Sente),
                },
            };
        }
        assert!(c.valid_limits());
        let response = run(&c);
        assert!(response.matches(&ExecutionRequest {
            protocol: PROTOCOL_VERSION,
            context: c.clone()
        }));
        let ExecutionResult::Chu {
            result: ChuResult::Choice { action, telemetry },
        } = &response.observation.result
        else {
            panic!("{response:?}")
        };
        let game = match &c.game {
            CanonicalRecord::Chu(r) => r.to_game().unwrap(),
            _ => unreachable!(),
        };
        assert!(game.legal_actions().contains(action));
        assert_eq!(telemetry.perspective, t::Side::Sente);
        assert_eq!(telemetry.completed_depth, 1);
        assert!(telemetry.score.is_some());
        for index in 0..4 {
            let mut wrong = c.clone();
            match index {
                0 => wrong.request_id += 1,
                1 => wrong.session_version += 1,
                2 => wrong.process_generation += 1,
                _ => wrong.limits.nodes += 1,
            };
            assert!(!response.matches(&ExecutionRequest {
                protocol: PROTOCOL_VERSION,
                context: wrong
            }));
        }
    }
}
#[test]
fn typed_chu_selection_cannot_borrow_shogi_turn_or_future_history() {
    let mut c = context();
    c.purpose = ExecutionPurpose::MoveAnalysis {
        target: ExecutionTarget {
            selection_id: 3,
            event_index: 0,
            board_ply: 0,
            session_version: c.session_version,
            label: "Native root".into(),
            turn: NativeSide::Shogi(mshogi_core::shogi::Side::Sente),
        },
    };
    assert!(!c.valid_limits());
    assert!(matches!(
        run(&c).observation.result,
        ExecutionResult::Failure {
            failure: EngineFailure {
                kind: FailureKind::InvalidRequest,
                ..
            }
        }
    ));
    let ExecutionPurpose::MoveAnalysis { target } = &mut c.purpose else {
        unreachable!()
    };
    target.turn = NativeSide::Chu(t::Side::Gote);
    assert!(!c.valid_limits());
    let ExecutionPurpose::MoveAnalysis { target } = &mut c.purpose else {
        unreachable!()
    };
    target.turn = NativeSide::Chu(t::Side::Sente);
    target.event_index = 1;
    assert!(!c.valid_limits());
}
