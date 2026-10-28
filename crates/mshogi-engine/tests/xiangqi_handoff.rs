//! MSH-84: a consumer uses the public native contract as one system.
//! Expected rules reuse independently reviewed91/93/94/105 cases, not engine outcomes.
use mshogi_core::{native::*, xiangqi as x};
use mshogi_engine::*;
use serde_json::Value;
use std::io::Cursor;
const F002: &str = "4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1";
const C: [&str; 4] = ["e9f9", "f8e8", "f9e9", "e8f8"];
fn game(fen: &str) -> x::Game {
    x::Game::from_root(
        x::PositionRoot::from_fen(fen).unwrap(),
        x::ClaimPolicy::Explicit,
    )
    .unwrap()
}
fn submit(
    g: &mut x::Game,
    actor: x::Side,
    command: x::GameCommand,
) -> Result<x::Submission, x::GameError> {
    let expected = g.context();
    let NativeReply::Xiangqi(result) = dispatch_command(
        NativeCommandTarget::Xiangqi(g),
        NativeCommand::Xiangqi(Box::new(x::GameRequest {
            expected,
            actor,
            command,
        })),
    )
    .unwrap() else {
        panic!("wrong native adapter")
    };
    result.map(|s| *s)
}
fn play(g: &mut x::Game, action: &str) -> x::Submission {
    submit(
        g,
        g.position().side_to_move(),
        x::GameCommand::Play {
            action: action.parse().unwrap(),
        },
    )
    .unwrap()
}
fn record(g: &x::Game) -> CanonicalRecord {
    CanonicalRecord::xiangqi(x::GameRecord::from_game(g).unwrap())
}
fn reopen(g: &x::Game) -> x::Game {
    let r = record(g);
    let decoded = CanonicalRecord::from_json(&r.to_json().unwrap()).unwrap();
    assert_eq!(decoded, r);
    assert_eq!(decoded.game_kind(), GameKind::Xiangqi);
    assert_eq!(decoded.profile().id(), x::PROFILE_ID);
    assert_eq!(decoded.profile().edition(), x::PROFILE_EDITION);
    assert_eq!(decoded.native_schema(), NativeSchema::XiangqiV1);
    let CanonicalRecord::Xiangqi(native) = decoded else {
        panic!()
    };
    let reopened = native.to_game().unwrap();
    assert_eq!(reopened.root(), g.root());
    assert_eq!(reopened.position(), g.position());
    assert_eq!(reopened.events(), g.events());
    assert_eq!(reopened.warnings(), g.warnings());
    assert_eq!(reopened.conclusion(), g.conclusion());
    assert_eq!(reopened.pending_adjudication(), g.pending_adjudication());
    assert_eq!(reopened.no_win_assessment(), g.no_win_assessment());
    assert_ne!(reopened.context(), g.context());
    reopened
}
fn limits() -> x::ExactLimits {
    x::ExactLimits {
        states: 1000,
        transitions: 10_000,
        millis: 60_000,
        traversal: x::ExactTraversal::BreadthFirst,
        ..x::ExactLimits::default()
    }
}
#[test]
fn admission_complete_capture_effects_local_and_game_undo_cross_canonical_boundary() {
    assert_eq!(
        (x::FILES, x::RANKS, x::RIVER_BETWEEN_RANKS),
        (9, 10, (4, 5))
    );
    let structural = x::PositionRoot::from_fen("4k4/9/9/9/9/9/9/9/9/4K4 w - - 0 1").unwrap();
    assert!(structural.position().validate_local().is_err());
    assert!(matches!(
        x::Game::from_root(structural, x::ClaimPolicy::Explicit),
        Err(x::GameError::Root(_))
    ));
    let mut g = game("5k3/9/9/9/9/4p4/2N6/9/9/3K5 w - - 7 17");
    let original = record(&g);
    let a = "c3e4".parse().unwrap();
    assert!(g.position().legal_actions().unwrap().contains(&a));
    let prepared = g.position().resolve_action(a).unwrap();
    assert_eq!(
        prepared.effects().moving,
        x::Piece::new(x::Side::Red, x::PieceKind::Horse)
    );
    assert_eq!(
        prepared.effects().captured,
        Some(x::Piece::new(x::Side::Black, x::PieceKind::Soldier))
    );
    let mut local = g.position().clone();
    let undo = local.apply_resolved(prepared).unwrap();
    let old_context = g.context();
    let admitted = play(&mut g, "c3e4");
    assert_eq!(g.position(), &local);
    assert_eq!(g.position().counters().no_capture_plies(), Some(0));
    assert_eq!(record(&g).known_board_actions(), 1);
    reopen(&g);
    local.undo(undo).unwrap();
    g.undo(admitted.undo).unwrap();
    assert_eq!(&local, g.position());
    assert_eq!(record(&g), original);
    assert_ne!(g.context(), old_context);
    let before = g.context();
    assert_eq!(
        submit(
            &mut g,
            x::Side::Black,
            x::GameCommand::Play {
                action: "f9e9".parse().unwrap()
            }
        )
        .unwrap_err(),
        x::GameError::WrongActor
    );
    assert_eq!(g.context(), before);
    assert_eq!(
        submit(
            &mut g,
            x::Side::Red,
            x::GameCommand::Play {
                action: "d0d3".parse().unwrap()
            }
        )
        .unwrap_err(),
        x::GameError::Action(x::ActionError::Movement)
    );
    assert_eq!(record(&g), original);
    let other = game("5k3/9/9/9/9/4p4/2N6/9/9/3K5 w - - 7 17");
    assert_eq!(
        g.submit(x::GameRequest {
            expected: other.context(),
            actor: x::Side::Red,
            command: x::GameCommand::Resign
        })
        .unwrap_err(),
        x::GameError::StaleContext
    );
    // Different-game command cannot reach Xiangqi admission.
    let shogi = mshogi_core::shogi::Game::new();
    assert!(
        dispatch_command(
            NativeCommandTarget::Xiangqi(&mut g),
            NativeCommand::Shogi(Box::new(mshogi_core::shogi::GameRequest {
                expected: shogi.context(),
                actor: mshogi_core::shogi::Side::Sente,
                command: mshogi_core::shogi::GameCommand::Resign
            }))
        )
        .is_err()
    );
    assert_eq!(record(&g), original);
}
#[test]
fn warnings_same_board_different_history_undo_and_fourth_occurrence_are_native() {
    let mut g = game(F002);
    for a in C.into_iter().cycle().take(7) {
        play(&mut g, a);
    }
    let before_warning = record(&g);
    let last = play(&mut g, C[3]);
    assert_eq!(g.warnings().len(), 1);
    assert!(g.warnings()[0].active);
    assert_eq!(g.warnings()[0].fault, x::FaultKind::PerpetualCheck);
    assert_eq!(g.conclusion(), None);
    assert_eq!(record(&g).known_board_actions(), 8);
    assert_eq!(record(&g).event_count(), 9);
    let board_event = g.event_prefix(8).unwrap();
    let warned = g.event_prefix(9).unwrap();
    assert_eq!(board_event.position, warned.position);
    assert_eq!(board_event.board_ply, warned.board_ply);
    assert!(board_event.warnings.is_empty());
    assert_eq!(warned.warnings.len(), 1);
    let saved = record(&g);
    let old_context = g.context();
    reopen(&g);
    let fresh = game(&g.position().to_fen().unwrap());
    assert_eq!(fresh.position(), g.position());
    assert!(fresh.warnings().is_empty());
    assert_ne!(record(&fresh), saved);
    let mut analysis = x::ExactAnalysis::new().unwrap();
    let pending = analysis
        .begin(
            &g,
            x::ExactLimits {
                states: 0,
                ..limits()
            },
        )
        .unwrap()
        .run(|| false);
    g.undo(last.undo).unwrap();
    assert_eq!(record(&g), before_warning);
    assert!(g.warnings().is_empty());
    play(&mut g, C[3]);
    assert_eq!(record(&g), saved);
    assert_ne!(g.context(), old_context);
    assert_eq!(
        analysis.accept(&g, &pending),
        Err(x::GameError::StaleContext)
    );
    for a in C {
        play(&mut g, a);
    }
    assert_eq!(
        g.conclusion(),
        Some(x::Conclusion::Win {
            winner: x::Side::Black,
            reason: x::WinReason::Repetition {
                fault: x::FaultKind::PerpetualCheck
            }
        })
    );
    // Local actions still exist; they are not permission to continue a concluded game.
    assert!(!g.position().legal_actions().unwrap().is_empty());
    assert_eq!(
        submit(
            &mut g,
            x::Side::Red,
            x::GameCommand::Play {
                action: C[0].parse().unwrap()
            }
        )
        .unwrap_err(),
        x::GameError::Concluded
    );
    reopen(&g);
}
#[test]
fn claimant_specific_audit_conclusion_and_undo_preserve_full_record() {
    let corpus: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/xiangqi/natural-count.json"
    ))
    .unwrap();
    let f = corpus["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == "NATURAL-11-CHECKS")
        .unwrap();
    let mut g = game(f["root_fen"].as_str().unwrap());
    for a in f["moves"].as_array().unwrap().iter().take(100) {
        play(&mut g, a.as_str().unwrap());
    }
    assert_eq!(
        g.natural_count_eligibility(x::Side::Red).effective_plies,
        99
    );
    assert_eq!(
        g.natural_count_eligibility(x::Side::Black).effective_plies,
        100
    );
    let position = g.position().clone();
    let previous = g.context();
    let rejection = submit(&mut g, x::Side::Red, x::GameCommand::ClaimNaturalCount).unwrap();
    assert_eq!(rejection.result, x::CommandResult::RejectedNaturalCount);
    assert_eq!(g.position(), &position);
    assert_eq!(g.conclusion(), None);
    assert_ne!(g.context(), previous);
    let audited = record(&g);
    reopen(&g);
    let conclusion = submit(&mut g, x::Side::Black, x::GameCommand::ClaimNaturalCount).unwrap();
    assert_eq!(g.position(), &position);
    assert_eq!(
        g.conclusion(),
        Some(x::Conclusion::Draw {
            reason: x::DrawReason::NaturalCount
        })
    );
    reopen(&g);
    g.undo(conclusion.undo).unwrap();
    assert_eq!(record(&g), audited);
    assert_eq!(g.conclusion(), None);
    g.undo(rejection.undo).unwrap();
    assert_eq!(g.position(), &position);
    assert_eq!(record(&g).known_board_actions(), 100);
}
#[test]
fn check_terminal_reasons_agreement_resignation_and_command_admission_remain_distinct() {
    for (fen, reason) in [
        (
            "4k4/9/9/9/9/4p4/9/9/3r1r3/4K4 w - - 0 1",
            x::NoActionReason::Stalemate,
        ),
        (
            "4k4/9/9/9/9/4p4/9/2n6/5r3/3rK4 w - - 0 1",
            x::NoActionReason::Checkmate,
        ),
    ] {
        let mut g = game(fen);
        assert_eq!(
            g.position().local_status().unwrap(),
            x::LocalStatus::NoLegalAction { reason }
        );
        assert_eq!(
            g.conclusion(),
            Some(x::Conclusion::Win {
                winner: x::Side::Black,
                reason: x::WinReason::NoLegalAction { reason }
            })
        );
        assert_eq!(
            submit(&mut g, x::Side::Black, x::GameCommand::Resign).unwrap_err(),
            x::GameError::Concluded
        );
        reopen(&g);
    }
    let mut g = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    assert_eq!(g.position().legal_actions().unwrap().len(), 44);
    assert_eq!(
        submit(&mut g, x::Side::Red, x::GameCommand::AcceptDraw).unwrap_err(),
        x::GameError::DrawOffer
    );
    play(&mut g, "a3a4");
    submit(&mut g, x::Side::Red, x::GameCommand::OfferDraw).unwrap();
    let offer = record(&g);
    let board = g.position().clone();
    let accepted = submit(&mut g, x::Side::Black, x::GameCommand::AcceptDraw).unwrap();
    assert_eq!(
        g.conclusion(),
        Some(x::Conclusion::Draw {
            reason: x::DrawReason::Agreement
        })
    );
    assert_eq!(g.position(), &board);
    reopen(&g);
    g.undo(accepted.undo).unwrap();
    assert_eq!(record(&g), offer);
    submit(&mut g, x::Side::Black, x::GameCommand::DeclineDraw).unwrap();
    // Red is not to move but may resign; board actor validation does not apply.
    submit(&mut g, x::Side::Red, x::GameCommand::Resign).unwrap();
    assert_eq!(
        g.conclusion(),
        Some(x::Conclusion::Win {
            winner: x::Side::Black,
            reason: x::WinReason::Resignation
        })
    );
    assert_eq!(g.position(), &board);
    reopen(&g);
}
#[test]
fn fast_exact_provenance_witness_unknown_cancel_failure_and_retirement_cross_native_record() {
    let fast = game("5k3/9/9/9/9/9/9/9/9/3K5 w - - 0 1");
    assert_eq!(
        fast.no_win_assessment(),
        x::NoWinAssessment::ProvenNeitherCanWin
    );
    assert_eq!(
        fast.conclusion(),
        Some(x::Conclusion::Draw {
            reason: x::DrawReason::NeitherCanWin
        })
    );
    reopen(&fast);
    let negative = game("5k3/9/9/9/9/9/9/9/4A4/3K5 w - - 0 1");
    assert_eq!(
        negative.no_win_assessment(),
        x::NoWinAssessment::UnknownExactUnavailable
    );
    assert!(matches!(
        negative.position().local_status().unwrap(),
        x::LocalStatus::Ongoing { .. }
    ));
    // These are different questions: local choice availability is not resolved WXF adjudication.
    let original = record(&negative);
    let mut analysis = x::ExactAnalysis::new().unwrap();
    let request = analysis.begin(&negative, limits()).unwrap();
    let proof = request.run(|| false);
    assert_eq!(proof.request().purpose(), x::ExactPurpose::NaturalBoardWin);
    assert_eq!(
        proof.request().source_record(),
        &x::GameRecord::from_game(&negative).unwrap()
    );
    assert_eq!(proof.report().verdict, x::ExactVerdict::NeitherCanWin);
    assert_eq!(
        analysis.accept(&negative, &proof).unwrap().assessment(),
        x::NoWinAssessment::ProvenNeitherCanWin
    );
    assert_eq!(proof.report().work.states, 255); // independently cross-checked reachable component105
    assert_eq!(record(&negative), original);
    assert_eq!(negative.conclusion(), None);
    // Exact provenance stays with proof; flattening assessment alone would lose origin.
    assert_eq!(proof.request().limits(), limits());
    reopen(&negative);
    for (cancel, expected) in [
        (false, x::ExactStop::States),
        (true, x::ExactStop::Cancelled),
    ] {
        let r = analysis
            .begin(
                &negative,
                x::ExactLimits {
                    states: 0,
                    ..limits()
                },
            )
            .unwrap();
        let result = r.run(|| cancel);
        assert_eq!(
            analysis.accept(&negative, &result).unwrap().assessment(),
            x::NoWinAssessment::UnknownExactIncomplete { reason: expected }
        );
        assert_eq!(record(&negative), original);
        assert_eq!(negative.conclusion(), None);
    }
    let failure = analysis
        .begin(
            &negative,
            x::ExactLimits {
                states: x::MAX_EXACT_STATES + 1,
                ..limits()
            },
        )
        .unwrap()
        .run(|| false);
    assert_eq!(
        failure.report().verdict,
        x::ExactVerdict::Failure {
            reason: x::ExactFailure::UnsupportedLimits
        }
    );
    assert_eq!(
        analysis.accept(&negative, &failure).unwrap().assessment(),
        x::NoWinAssessment::Failure
    );
    let positive = game("5k3/R8/9/9/9/9/9/9/9/3K5 b - - 0 1");
    let request = analysis.begin(&positive, limits()).unwrap();
    let observed = request.run(|| false);
    let x::ExactVerdict::NaturalWin {
        winner,
        reason,
        witness,
    } = &observed.report().verdict
    else {
        panic!()
    };
    assert_eq!(
        (*winner, *reason, witness.len()),
        (x::Side::Red, x::NoActionReason::Stalemate, 2)
    );
    let mut path = positive.position().clone();
    for &a in witness {
        path.apply(a).unwrap();
    }
    assert_eq!(
        path.local_status().unwrap(),
        x::LocalStatus::NoLegalAction { reason: *reason }
    );
    analysis
        .select(x::ExactTarget::History { event_count: 0 })
        .unwrap();
    analysis.select(x::ExactTarget::Live).unwrap();
    assert_eq!(
        analysis.accept(&positive, &observed),
        Err(x::GameError::StaleContext)
    );
    let restored = reopen(&positive);
    let old = analysis.begin(&positive, limits()).unwrap().run(|| false);
    assert_eq!(
        analysis.accept(&restored, &old),
        Err(x::GameError::StaleContext)
    );
}
#[test]
fn strict_payload_context_and_transport_preserve_identity_through_real_execution() {
    let mut g = game(F002);
    for a in C.into_iter().cycle().take(8) {
        play(&mut g, a);
    }
    let saved = record(&g);
    let text = saved.to_json().unwrap();
    let duplicated = text.replacen(
        "\"envelope_version\":1",
        "\"envelope_version\":1,\"envelope_version\":1",
        1,
    );
    assert!(CanonicalRecord::from_json(&duplicated).is_err());
    let duplicate_native = text.replacen(
        "\"format\":\"mshogi.xiangqi.game\"",
        "\"format\":\"mshogi.xiangqi.game\",\"format\":\"mshogi.xiangqi.game\"",
        1,
    );
    assert!(CanonicalRecord::from_json(&duplicate_native).is_err());
    let mut forged: Value = serde_json::from_str(&text).unwrap();
    forged["payload"]["events"] = serde_json::json!([]);
    assert!(CanonicalRecord::from_json(&forged.to_string()).is_err());
    let conflict = text.replace(x::PROFILE_ID, "mshogi.shogi.csa36.v1");
    assert!(CanonicalRecord::from_json(&conflict).is_err());
    let native = x::GameRecord::from_game(&g).unwrap();
    assert_eq!(
        x::GameRecord::from_json(&native.to_json().replace("sound_fast_only", "bounded_exact")),
        Err(x::RecordError::Unsupported)
    );
    let mut resource = x::ExactAnalysis::new().unwrap();
    let request = resource.begin(&g, limits()).unwrap();
    let prior = request.run(|| true);
    let reopened = reopen(&g);
    assert_eq!(
        resource.accept(&reopened, &prior),
        Err(x::GameError::StaleContext)
    );
    let context = SharedRequest {
        protocol: SHARED_PROTOCOL_VERSION,
        purpose: SharedPurpose::EngineTurn {},
        request_id: 41,
        session_version: 7,
        process_generation: 3,
        game: saved.clone(),
        limits: WorkLimits {
            depth: 1,
            nodes: 100,
        },
    };
    let encoded = serde_json::to_string(&context).unwrap();
    let parsed: SharedRequest = serde_json::from_str(&encoded).unwrap();
    assert_eq!(parsed, context);
    assert_eq!(execution_capabilities(GameKind::Xiangqi).len(), 2);
    let mut response = vec![];
    serve_once(Cursor::new(format!("{encoded}\n")), &mut response).unwrap();
    let reply: SharedResponse = serde_json::from_slice(&response).unwrap();
    assert!(reply.matches(&context));
    assert!(matches!(
        reply.result,
        SharedResult::Xiangqi {
            result: XiangqiResult::Choice { .. }
        }
    ));
    let mut different = context.clone();
    different.request_id += 1;
    assert!(!reply.matches(&different));
    let mut fresh = context.clone();
    fresh.game = record(&game(&g.position().to_fen().unwrap()));
    assert!(!reply.matches(&fresh));
    assert_eq!(record(&g), saved);
}
