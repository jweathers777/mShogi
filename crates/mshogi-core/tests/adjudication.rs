//! Expected thresholds and positions come from the independently derived MSH-15 profile.
use mshogi_core::chess::*;
use serde_json::Value;
fn a(s: &str) -> Action {
    s.parse().unwrap()
}
fn fixture(c: &Value) -> Game {
    let mut g = Game::from_fen(c["fen"].as_str().unwrap(), ClaimPolicy::ExplicitOnly).unwrap();
    for text in c["moves"].as_array().unwrap() {
        g.apply(a(text.as_str().unwrap())).unwrap();
    }
    g
}
fn request(g: &Game, reason: DrawClaim, intended: Option<Action>) -> ClaimRequest {
    ClaimRequest {
        actor: g.position().side_to_move(),
        reason,
        intended,
    }
}
fn data() -> Value {
    serde_json::from_str(include_str!(
        "../../../tests/fixtures/chess/match-profile.json"
    ))
    .unwrap()
}

#[test]
fn profile_claim_thresholds_and_intended_actions() {
    for c in data()["cases"].as_array().unwrap() {
        let g = fixture(c);
        let expected = &c["expected"];
        if let Some(list) = expected["claims"].as_array() {
            for (name, reason) in [
                ("threefold", DrawClaim::Threefold),
                ("fifty", DrawClaim::FiftyMove),
            ] {
                assert_eq!(
                    g.validate_claim(request(&g, reason, None)).is_ok(),
                    list.iter().any(|x| x == name),
                    "{} {name}",
                    c["id"]
                );
            }
        }
        for (key, intended) in [
            ("fifty_current", None),
            ("fifty_intended", c["intended"].as_str().map(a)),
        ] {
            if let Some(want) = expected[key].as_bool() {
                assert_eq!(
                    g.validate_claim(request(&g, DrawClaim::FiftyMove, intended))
                        .is_ok(),
                    want,
                    "{} {key}",
                    c["id"]
                );
            }
        }
        if let Some(text) = c["intended"].as_str() {
            if let Some(reason) = expected["intended_claim"].as_str() {
                let reason = match reason {
                    "threefold" => DrawClaim::Threefold,
                    "fifty" => DrawClaim::FiftyMove,
                    _ => panic!("fixture"),
                };
                assert!(
                    g.validate_claim(request(&g, reason, Some(a(text)))).is_ok(),
                    "{}",
                    c["id"]
                );
            } else if expected.as_object().unwrap().contains_key("intended_claim") {
                assert!(
                    g.validate_claim(request(&g, DrawClaim::FiftyMove, Some(a(text))))
                        .is_err(),
                    "{}",
                    c["id"]
                );
            }
        }
    }
}

#[test]
fn independent_automatic_results_and_bounded_dead_recognition() {
    for c in data()["cases"].as_array().unwrap() {
        let g = fixture(c);
        if let Some(automatic) = c["expected"]["automatic"].as_str() {
            let expected = match automatic {
                "fivefold" => ChessOutcome::Draw(DrawReason::Fivefold),
                "seventy_five" => ChessOutcome::Draw(DrawReason::SeventyFiveMove),
                "checkmate" => ChessOutcome::Checkmate {
                    winner: Color::White,
                },
                "stalemate" => ChessOutcome::Draw(DrawReason::Stalemate),
                _ => panic!("fixture"),
            };
            assert_eq!(g.automatic_outcome(), Some(expected), "{}", c["id"]);
        }
        if let Some(recognition) = c["expected"]["recognition"].as_str() {
            assert_eq!(
                g.position().dead_recognition(),
                if recognition == "proven_dead" {
                    DeadRecognition::ProvenDead
                } else {
                    DeadRecognition::Unknown
                },
                "{}",
                c["id"]
            );
            assert_eq!(
                g.automatic_outcome(),
                if recognition == "proven_dead" {
                    Some(ChessOutcome::Draw(DrawReason::DeadPosition))
                } else {
                    None
                },
                "{}",
                c["id"]
            );
        }
    }
    let g = Game::from_fen("7k/8/8/8/8/8/8/K7 w - - 150 1", ClaimPolicy::ExplicitOnly).unwrap();
    assert_eq!(g.position().classify(), Classification::Quiet);
    assert_eq!(
        g.automatic_outcome(),
        Some(ChessOutcome::Draw(DrawReason::DeadPosition))
    );
}

#[test]
fn optional_claims_conclude_only_when_exercised_and_round_trip_without_playing_intention() {
    for intended in [false, true] {
        let mut g = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        let moves = [
            "g1f3", "g8f6", "f3g1", "f6g8", "g1f3", "g8f6", "f3g1", "f6g8",
        ];
        for text in &moves[..if intended { 7 } else { 8 }] {
            g.apply(a(text)).unwrap();
        }
        assert_eq!(g.outcome(), None);
        assert_eq!(g.policy_claim(), None);
        let before = g.clone();
        let req = request(
            g.history(),
            DrawClaim::Threefold,
            intended.then(|| a("f6g8")),
        );
        g.claim(req).unwrap();
        assert_eq!(g.history(), before.history());
        assert_eq!(
            g.outcome(),
            Some(ChessOutcome::Draw(DrawReason::Claimed(
                DrawClaim::Threefold
            )))
        );
        let concluded = g.clone();
        assert_eq!(g.apply(a("e2e4")), Err(PlayError::GameOver));
        assert_eq!(g.claim(req), Err(ClaimError::GameOver));
        assert_eq!(g, concluded);
        let r = GameRecord::from_json(&g.record().to_json()).unwrap();
        assert_eq!(AdjudicatedGame::replay_record(&r).unwrap(), g);
        assert_eq!(g.undo(), Some(AdjudicationUndo::Claim(req)));
        assert_eq!(g, before);
        g.undo().unwrap();
        assert_eq!(g.outcome(), None);
    }
}

#[test]
fn invalid_claims_and_wrong_actor_are_atomic() {
    let mut g = AdjudicatedGame::new(ClaimPolicy::AutoClaim);
    let before = g.clone();
    let mut req = request(g.history(), DrawClaim::Threefold, None);
    assert_eq!(g.claim(req), Err(ClaimError::NotAvailable));
    assert_eq!(g, before);
    req.actor = Color::Black;
    assert_eq!(g.claim(req), Err(ClaimError::WrongActor));
    assert_eq!(g, before);
    req.actor = Color::White;
    req.intended = Some(a("e2e5"));
    assert!(matches!(g.claim(req), Err(ClaimError::InvalidIntended(_))));
    assert_eq!(g, before);
}

#[test]
fn policy_selection_current_before_intended_and_intended_before_prospective_mate() {
    for c in data()["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c.get("policy").is_some())
    {
        let history = fixture(c);
        let mut record = serde_json::from_str::<Value>(&history.record().to_json()).unwrap();
        record["claim_policy"] = c["policy"].clone();
        let history = GameRecord::from_json(&record.to_string())
            .unwrap()
            .replay_history()
            .unwrap()
            .game;
        let mut live = AdjudicatedGame::from_history(history).unwrap();
        assert_eq!(live.outcome(), None);
        if let Some(claim) = live.policy_claim() {
            live.claim(claim).unwrap();
        }
        assert_eq!(
            live.outcome().is_some(),
            c["expected"]["concluded"] == "draw",
            "{}",
            c["id"]
        );
    }
    let mut live =
        AdjudicatedGame::from_fen("7k/8/5KQ1/8/8/8/8/8 w - - 99 1", ClaimPolicy::ExplicitOnly)
            .unwrap();
    let req = request(live.history(), DrawClaim::FiftyMove, Some(a("g6g7")));
    let before = live.history().clone();
    live.claim(req).unwrap();
    assert_eq!(live.history(), &before);
    let mut played =
        AdjudicatedGame::from_fen("7k/8/5KQ1/8/8/8/8/8 w - - 149 1", ClaimPolicy::ExplicitOnly)
            .unwrap();
    played.apply(a("g6g7")).unwrap();
    assert_eq!(
        played.outcome(),
        Some(ChessOutcome::Checkmate {
            winner: Color::White
        })
    );
}

#[test]
fn live_path_refuses_continuation_after_automatic_result_and_undo_restores_it() {
    let mut live = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    for _ in 0..4 {
        for text in ["g1f3", "g8f6", "f3g1", "f6g8"] {
            live.apply(a(text)).unwrap();
        }
    }
    assert_eq!(
        live.outcome(),
        Some(ChessOutcome::Draw(DrawReason::Fivefold))
    );
    let before = live.clone();
    assert_eq!(live.apply(a("e2e4")), Err(PlayError::GameOver));
    assert_eq!(live, before);
    live.undo().unwrap();
    assert_eq!(live.outcome(), None);
    live.apply(a("f6g8")).unwrap();
    assert_eq!(live, before);
    let mut raw = live.history().clone();
    raw.apply(a("e2e4")).unwrap();
    assert_eq!(AdjudicatedGame::from_history(raw), Err(PlayError::GameOver));
    let mut record = live.record();
    record.push_event(RecordEvent {
        ply: 16,
        kind: RecordEventKind::Move(a("e2e4")),
    });
    assert!(matches!(
        AdjudicatedGame::replay_record(&record),
        Err(LiveReplayError::Play(17, PlayError::GameOver))
    ));
}

#[test]
fn intended_claim_legality_does_not_depend_on_fullmove_representability() {
    let g = Game::from_fen(
        "7k/8/8/8/8/8/8/KR6 b - - 99 4294967295",
        ClaimPolicy::ExplicitOnly,
    )
    .unwrap();
    assert!(
        g.validate_claim(request(&g, DrawClaim::FiftyMove, Some(a("h8h7"))))
            .is_ok()
    );
    assert_eq!(
        g.position().clone().apply(a("h8h7")).unwrap_err(),
        ActionError::CounterOverflow
    );
}

#[test]
fn policy_returns_the_exact_profile_priority_and_square_order() {
    let g = AdjudicatedGame::from_fen("7k/8/8/8/8/2K5/R7/8 w - - 99 1", ClaimPolicy::AutoClaim)
        .unwrap();
    assert_eq!(
        g.policy_claim(),
        Some(ClaimRequest {
            actor: Color::White,
            reason: DrawClaim::FiftyMove,
            intended: Some(a("a2a1"))
        })
    );
    let g = AdjudicatedGame::from_fen("7k/8/8/8/8/8/8/KR6 w - - 100 1", ClaimPolicy::AutoClaim)
        .unwrap();
    assert_eq!(
        g.policy_claim(),
        Some(ClaimRequest {
            actor: Color::White,
            reason: DrawClaim::FiftyMove,
            intended: None
        })
    );
    let mut h = Game::from_fen(
        &STANDARD_START_FEN.replace("0 1", "92 1"),
        ClaimPolicy::AutoClaim,
    )
    .unwrap();
    for text in ["g1f3", "g8f6", "f3g1", "f6g8", "g1f3", "g8f6", "f3g1"] {
        h.apply(a(text)).unwrap();
    }
    let g = AdjudicatedGame::from_history(h.clone()).unwrap();
    assert_eq!(
        g.policy_claim(),
        Some(ClaimRequest {
            actor: Color::Black,
            reason: DrawClaim::Threefold,
            intended: Some(a("f6g8"))
        })
    );
    h.apply(a("f6g8")).unwrap();
    assert_eq!(
        AdjudicatedGame::from_history(h).unwrap().policy_claim(),
        Some(ClaimRequest {
            actor: Color::White,
            reason: DrawClaim::Threefold,
            intended: None
        })
    );
}
