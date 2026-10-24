//! Independent MSH-15 command/clock expectations, with injected time and no sleep.
use mshogi_core::chess::*;
use serde_json::{Value, json};
fn a(s: &str) -> Action {
    s.parse().unwrap()
}
fn send(m: &mut ChessMatch, at: u64, actor: Color, command: MatchCommand) -> MatchReply {
    m.submit(MatchRequest {
        at_ns: at,
        version: m.version(),
        actor,
        command,
    })
}
fn started(fen: Option<&str>, control: TimeControl) -> ChessMatch {
    let g = match fen {
        None => AdjudicatedGame::new(ClaimPolicy::ExplicitOnly),
        Some(f) => AdjudicatedGame::from_fen(f, ClaimPolicy::ExplicitOnly).unwrap(),
    };
    let actor = g.position().side_to_move();
    let mut m = ChessMatch::new(g, control).unwrap();
    assert!(
        send(&mut m, 0, actor, MatchCommand::Start)
            .rejection
            .is_none()
    );
    m
}
fn timed() -> TimeControl {
    TimeControl::Increment {
        initial_ns: [100, 100],
        increment_ns: 10,
    }
}
fn assert_replay(m: &ChessMatch) {
    let json = m.record().to_json();
    let parsed = MatchRecord::from_json(&json).unwrap();
    assert_eq!(parsed.to_json(), json);
    assert_eq!(parsed.replay().unwrap(), *m);
}

#[test]
fn execute_independent_clock_profile_fixtures() {
    let data: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/chess/match-profile.json"
    ))
    .unwrap();
    for c in data["clocks"].as_array().unwrap() {
        let game = AdjudicatedGame::from_fen(
            c["fen"].as_str().unwrap_or(STANDARD_START_FEN),
            ClaimPolicy::ExplicitOnly,
        )
        .unwrap();
        let control = match c["initial"].as_u64() {
            Some(n) => TimeControl::Increment {
                initial_ns: [n, n],
                increment_ns: c["increment"].as_u64().unwrap(),
            },
            None => TimeControl::Untimed,
        };
        let mut m = ChessMatch::new(game, control).unwrap();
        let mut last = None;
        for event in c["events"].as_array().unwrap() {
            let cmd = match event["kind"].as_str().unwrap() {
                "start" => MatchCommand::Start,
                "move" => MatchCommand::Move(a(event["action"].as_str().unwrap())),
                "invalid_claim" => MatchCommand::Claim {
                    reason: DrawClaim::Threefold,
                    intended: None,
                },
                _ => panic!("fixture"),
            };
            let actor = m.game().position().side_to_move();
            last = Some(send(&mut m, event["at"].as_u64().unwrap(), actor, cmd));
        }
        let e = &c["expected"];
        if let Some(w) = e["white"].as_u64() {
            assert_eq!(
                m.remaining_ns(),
                Some([w, e["black"].as_u64().unwrap()]),
                "{}",
                c["id"]
            );
        }
        if let Some(s) = e["active"].as_str() {
            assert_eq!(
                m.active(),
                Some(if s == "white" {
                    Color::White
                } else {
                    Color::Black
                })
            );
        }
        if e["termination"] == "unresolved_mating_feasibility" {
            assert_eq!(
                m.conclusion(),
                Some(MatchConclusion::Unresolved {
                    actor: Color::White,
                    trigger: TerminationTrigger::FlagFall
                })
            );
            assert_eq!(m.game().history().ply_count(), 0);
        }
        if e["termination"] == "white_checkmate_win" {
            assert_eq!(
                m.conclusion(),
                Some(MatchConclusion::Chess(ChessOutcome::Checkmate {
                    winner: Color::White
                }))
            );
        }
        if e["error"] == "nonmonotonic_time" {
            assert_eq!(
                last.unwrap().rejection.unwrap().code,
                MatchRejectCode::NonmonotonicTime
            );
            assert_eq!(m.game().history().ply_count(), 0);
        }
        assert_replay(&m);
    }
}

#[test]
fn profile_resign_and_flag_possible_impossible_and_unresolved() {
    let data: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/chess/match-profile.json"
    ))
    .unwrap();
    for c in data["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["expected"]["command"].is_object())
    {
        let e = &c["expected"];
        let actor = if e["command"]["actor"] == "white" {
            Color::White
        } else {
            Color::Black
        };
        let flag = e["command"]["kind"] == "flag";
        let mut m = started(
            Some(c["fen"].as_str().unwrap()),
            if flag { timed() } else { TimeControl::Untimed },
        );
        let before = m.game().history().clone();
        let witness = e["witness"]
            .as_array()
            .map(|v| v.iter().map(|s| a(s.as_str().unwrap())).collect());
        let command = if flag {
            MatchCommand::Observe { witness }
        } else {
            MatchCommand::Resign { witness }
        };
        let result = send(&mut m, if flag { 100 } else { 1 }, actor, command);
        assert!(
            result.rejection.is_none(),
            "{} {:?}",
            c["id"],
            result.rejection
        );
        assert_eq!(m.game().history(), &before);
        let expected = e["termination"].as_str().unwrap();
        assert_eq!(
            m.conclusion().unwrap().white_score_twice(),
            match expected {
                "white_win" => Some(2),
                "black_win" => Some(0),
                "draw" => Some(1),
                "unresolved_mating_feasibility" => None,
                _ => panic!("fixture"),
            },
            "{}",
            c["id"]
        );
        assert_replay(&m);
    }
}

#[test]
fn offers_require_recorded_first_moves_persist_and_reject_atomically() {
    let mut m = started(None, TimeControl::Untimed);
    send(&mut m, 1, Color::White, MatchCommand::OfferDraw);
    assert_eq!(
        send(&mut m, 2, Color::Black, MatchCommand::AcceptDraw)
            .rejection
            .unwrap()
            .code,
        MatchRejectCode::OfferTooEarly
    );
    send(&mut m, 3, Color::White, MatchCommand::Move(a("e2e4"))); // offering player's move preserves it.
    assert_eq!(
        send(&mut m, 4, Color::White, MatchCommand::OfferDraw)
            .rejection
            .unwrap()
            .code,
        MatchRejectCode::OfferPending
    );
    send(&mut m, 5, Color::Black, MatchCommand::Move(a("e7e5"))); // opponent's move declines.
    assert_eq!(
        send(&mut m, 6, Color::Black, MatchCommand::AcceptDraw)
            .rejection
            .unwrap()
            .code,
        MatchRejectCode::NoOffer
    );
    send(&mut m, 7, Color::Black, MatchCommand::OfferDraw);
    send(&mut m, 8, Color::White, MatchCommand::AcceptDraw);
    assert_eq!(
        m.conclusion(),
        Some(MatchConclusion::Draw { trigger: None })
    );
    assert_replay(&m);
    let before = m.game().clone();
    let result = send(&mut m, 9, Color::White, MatchCommand::Move(a("g1f3")));
    assert_eq!(result.rejection.unwrap().code, MatchRejectCode::GameOver);
    assert_eq!(m.game(), &before);
    assert_replay(&m);
}

#[test]
fn rejected_moves_stale_context_and_increment_overflow_charge_time_without_moving() {
    let mut m = started(None, timed());
    let before = m.game().clone();
    assert_eq!(
        send(&mut m, 1, Color::White, MatchCommand::Move(a("e2e5")))
            .rejection
            .unwrap()
            .code,
        MatchRejectCode::InvalidMove
    );
    assert_eq!(m.game(), &before);
    assert_eq!(m.remaining_ns(), Some([99, 100]));
    let r = m.submit(MatchRequest {
        at_ns: 2,
        version: 0,
        actor: Color::White,
        command: MatchCommand::Move(a("e2e4")),
    });
    assert_eq!(r.rejection.unwrap().code, MatchRejectCode::WrongVersion);
    assert_eq!(m.remaining_ns(), Some([98, 100]));
    assert_eq!(m.game(), &before);
    assert_replay(&m);
    let mut m = started(
        None,
        TimeControl::Increment {
            initial_ns: [100, 100],
            increment_ns: u64::MAX,
        },
    );
    let before = m.game().clone();
    assert_eq!(
        send(&mut m, 1, Color::White, MatchCommand::Move(a("e2e4")))
            .rejection
            .unwrap()
            .code,
        MatchRejectCode::ClockOverflow
    );
    assert_eq!(m.game(), &before);
    assert_eq!(m.remaining_ns(), Some([99, 100]));
    assert_eq!(m.active(), Some(Color::White));
    assert_replay(&m);
}

#[test]
fn claims_stop_clock_and_intention_stays_unplayed() {
    let mut m = started(Some("7k/8/8/8/8/8/8/KR6 w - - 99 1"), timed());
    let before = m.game().history().clone();
    send(
        &mut m,
        20,
        Color::White,
        MatchCommand::Claim {
            reason: DrawClaim::FiftyMove,
            intended: Some(a("b1b2")),
        },
    );
    assert_eq!(
        m.conclusion(),
        Some(MatchConclusion::Chess(ChessOutcome::Draw(
            DrawReason::Claimed(DrawClaim::FiftyMove)
        )))
    );
    assert_eq!(m.remaining_ns(), Some([80, 100]));
    assert_eq!(m.active(), None);
    assert_eq!(m.game().history(), &before);
    assert_replay(&m);
}

#[test]
fn hypothetical_mate_is_article_three_reachability_not_counter_or_claim_policy() {
    let p = Position::from_fen("7k/8/5KQ1/8/8/8/8/8 w - - 149 4294967295").unwrap();
    assert_eq!(
        mating_feasibility(&p, Color::White, Some(&[a("g6g7")])),
        Ok(MatingFeasibility::Possible)
    );
    assert_eq!(
        mating_feasibility(&p, Color::Black, None),
        Ok(MatingFeasibility::Impossible)
    );
    assert_eq!(
        mating_feasibility(&p, Color::White, None),
        Ok(MatingFeasibility::Unknown)
    );
    assert!(mating_feasibility(&p, Color::White, Some(&[a("g6g8")])).is_err());
}

#[test]
fn expiry_precedes_move_and_invalid_witness_never_delays_it() {
    let mut m = started(None, timed());
    send(
        &mut m,
        100,
        Color::White,
        MatchCommand::Observe {
            witness: Some(vec![a("e2e5")]),
        },
    );
    assert_eq!(
        m.conclusion(),
        Some(MatchConclusion::Unresolved {
            actor: Color::White,
            trigger: TerminationTrigger::FlagFall
        })
    );
    assert_eq!(m.game().history().ply_count(), 0);
    assert_replay(&m);
}

#[test]
fn records_reject_corrupted_observations_and_unknown_schema_fields() {
    let mut m = started(None, timed());
    send(&mut m, 20, Color::White, MatchCommand::Move(a("e2e4")));
    let mut value: Value = serde_json::from_str(&m.record().to_json()).unwrap();
    value["events"][1]["reply"]["remaining_ns"] = json!([999, 100]);
    assert!(
        MatchRecord::from_json(&value.to_string())
            .unwrap()
            .replay()
            .is_err()
    );
    value["effects"] = json!([]);
    assert!(MatchRecord::from_json(&value.to_string()).is_err());
    assert!(
        ChessMatch::new(
            AdjudicatedGame::new(ClaimPolicy::ExplicitOnly),
            TimeControl::Increment {
                initial_ns: [0, 1],
                increment_ns: 0
            }
        )
        .is_err()
    );
}

#[test]
fn bad_certificate_does_not_cancel_an_explicit_resignation() {
    let mut m = started(None, TimeControl::Untimed);
    let reply = send(
        &mut m,
        1,
        Color::White,
        MatchCommand::Resign {
            witness: Some(vec![a("e2e5")]),
        },
    );
    assert_eq!(
        reply.rejection.unwrap().code,
        MatchRejectCode::InvalidWitness
    );
    assert_eq!(
        m.conclusion(),
        Some(MatchConclusion::Unresolved {
            actor: Color::White,
            trigger: TerminationTrigger::Resignation
        })
    );
    assert_eq!(m.active(), None);
    assert_replay(&m);
}

#[test]
fn nested_outcomes_and_direct_serde_identity_cannot_bypass_record_checks() {
    let mut m = started(None, TimeControl::Untimed);
    for (at, actor, mv) in [
        (1, Color::White, "f2f3"),
        (2, Color::Black, "e7e5"),
        (3, Color::White, "g2g4"),
        (4, Color::Black, "d8h4"),
    ] {
        send(&mut m, at, actor, MatchCommand::Move(a(mv)));
    }
    let mut v: Value = serde_json::from_str(&m.record().to_json()).unwrap();
    v["events"][4]["reply"]["conclusion"]["chess"]["checkmate"]["unknown_field"] = json!(true);
    assert!(MatchRecord::from_json(&v.to_string()).is_err());
    let mut v: Value = serde_json::from_str(&m.record().to_json()).unwrap();
    v["schema_version"] = json!(999);
    let unchecked: MatchRecord = serde_json::from_value(v).unwrap();
    assert!(unchecked.replay().is_err());
}
