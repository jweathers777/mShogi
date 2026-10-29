//! Presentation expectations, not a new rules oracle or canonical serialization.
use super::*;

fn label(actor: Option<x::Side>, event: x::EventKind) -> String {
    event_label(&x::GameEvent {
        index: 7,
        board_ply: 3,
        actor,
        event,
    })
}

#[test]
fn native_action_labels_distinguish_quiet_capture_and_check_without_debug_payloads() {
    for (actor, action, captured_id, gave_check, expected) in [
        (x::Side::Red, "a3a4", None, false, "Red: a3–a4"),
        (x::Side::Black, "a6a5", None, false, "Black: a6–a5"),
        (x::Side::Red, "b2b9", Some(4), false, "Red: b2×b9"),
        (
            x::Side::Black,
            "a9b9",
            Some(9),
            true,
            "Black: a9×b9 · check",
        ),
        (x::Side::Red, "e3e4", None, true, "Red: e3–e4 · check"),
    ] {
        assert_eq!(
            label(
                Some(actor),
                x::EventKind::BoardAction {
                    action: action.parse().unwrap(),
                    moving_id: 8,
                    captured_id,
                    gave_check,
                }
            ),
            expected
        );
    }
}

#[test]
fn native_non_board_events_keep_claimant_warning_pending_and_conclusion_distinct() {
    let evidence = x::NaturalCountEligibility {
        claimant: x::Side::Black,
        known_plies: 100,
        claimant_checks: 0,
        effective_plies: 100,
        eligible: true,
    };
    for (event, expected) in [
        (
            x::EventKind::ClaimSubmitted {
                evidence,
                automatic: false,
            },
            "Black: natural-count claim submitted",
        ),
        (
            x::EventKind::ClaimSubmitted {
                evidence,
                automatic: true,
            },
            "Black: natural-count claim submitted (automatic)",
        ),
        (
            x::EventKind::ClaimRejected { evidence },
            "Black: natural-count claim rejected",
        ),
        (
            x::EventKind::ClaimAccepted { evidence },
            "Black: natural-count claim accepted",
        ),
        (
            x::EventKind::Warning {
                warning: x::Warning {
                    side: x::Side::Black,
                    fault: x::FaultKind::PerpetualCheck,
                    occurrences: vec![0, 4, 8],
                    event_index: 8,
                    active: true,
                },
            },
            "Black: repetition warning · perpetual check",
        ),
        (
            x::EventKind::WarningRetired {
                warning_event_index: 8,
            },
            "Repetition warning cleared · event 9",
        ),
        (
            x::EventKind::DrawOffer {
                offered_by: Some(x::Side::Black),
            },
            "Black: draw offered",
        ),
        (
            x::EventKind::DrawOffer { offered_by: None },
            "Draw offer withdrawn",
        ),
        (x::EventKind::Resigned, "Black: resigned"),
        (
            x::EventKind::AgreementAccepted,
            "Black: draw agreement accepted",
        ),
        (
            x::EventKind::AdjudicationPending {
                pending: x::PendingAdjudication {
                    occurrences: vec![0, 4, 8],
                    reason: "internal diagnostic must stay out of visible label".into(),
                },
            },
            "Adjudication unresolved · unscored",
        ),
    ] {
        assert_eq!(label(Some(x::Side::Black), event), expected);
    }
    for (reason, expected) in [
        (x::WinReason::Resignation, "Red wins · resignation"),
        (
            x::WinReason::NoLegalAction {
                reason: x::NoActionReason::Checkmate,
            },
            "Red wins · checkmate",
        ),
        (
            x::WinReason::NoLegalAction {
                reason: x::NoActionReason::Stalemate,
            },
            "Red wins · no legal move (stalemate)",
        ),
        (
            x::WinReason::Repetition {
                fault: x::FaultKind::PerpetualChase,
            },
            "Red wins · perpetual chase",
        ),
    ] {
        assert_eq!(
            label(
                None,
                x::EventKind::Concluded {
                    conclusion: x::Conclusion::Win {
                        winner: x::Side::Red,
                        reason
                    }
                }
            ),
            expected
        );
    }
    for (reason, expected) in [
        (x::DrawReason::Repetition, "Draw · repetition"),
        (x::DrawReason::NaturalCount, "Draw · natural-count claim"),
        (x::DrawReason::Agreement, "Draw · agreement"),
        (x::DrawReason::NeitherCanWin, "Draw · neither side can win"),
    ] {
        assert_eq!(
            label(
                None,
                x::EventKind::Concluded {
                    conclusion: x::Conclusion::Draw { reason }
                }
            ),
            expected
        );
    }
}
