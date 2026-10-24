//! Primary-rule cooperative reachability, independent finite graph expectations.
use mshogi_core::chess::*;
use std::collections::HashSet;
const WALL: &str = "8/7k/8/p1p1p1p1/P1P1P1P1/8/K7/8 w - - 0 1";
const FORCED: &str = "8/8/8/8/2b5/2k5/8/Kr6 w - - 0 1";
fn p(f: &str) -> Position {
    Position::from_fen(f).unwrap()
}
fn unlimited() -> ReachabilityLimits {
    ReachabilityLimits::default()
}
fn full(limits: ReachabilityLimits) -> AdjudicationMode {
    AdjudicationMode::Full { limits }
}
fn action(a: &str) -> Action {
    a.parse().unwrap()
}
fn witness(p: &Position, side: Color, actions: &[Action]) {
    assert_eq!(
        mating_feasibility(p, side, Some(actions)).unwrap(),
        MatingFeasibility::Possible
    );
}

#[test]
fn blocked_wall_exact_exhaustion_matches_independent_king_graph() {
    let initial = p(WALL);
    assert_eq!(initial.dead_recognition(), DeadRecognition::Unknown);
    let result = initial.mate_reachability(MateTarget::Either, unlimited(), || false);
    let MateReachability::Impossible { work } = result else {
        panic!("{result:?}");
    };
    // Each king is confined to its own 3x8 rectangle. Permanent pawn attacks
    // cover the four gaps; no pawn can move/capture or king cross the wall.
    // 24*24*2 vertices. Each rectangle's directed king graph has 130 arcs:
    // horizontal 42 + vertical 32 + diagonal 56. Thus 2*24*130 = 6240 arcs.
    assert_eq!(work.positions, 1152);
    assert_eq!(work.expanded, 1152);
    assert_eq!(work.transitions, 6240);
    assert_eq!(work.static_leaves, 0);
    // Independent traversal: DFS with canonical FEN string keys, no production
    // repetition keys, breadth-first queue, material pruning or solver calls.
    let mut stack = vec![initial];
    let mut seen = HashSet::new();
    let mut edges = 0;
    while let Some(pos) = stack.pop() {
        let fields: Vec<_> = pos
            .to_fen()
            .split_whitespace()
            .take(4)
            .map(str::to_owned)
            .collect();
        if !seen.insert(fields.join(" ")) {
            continue;
        }
        let actions = pos.legal_actions();
        assert!(!actions.is_empty());
        for a in actions {
            let mut next = pos.clone();
            next.apply(a).unwrap();
            let fields: Vec<_> = next
                .to_fen()
                .split_whitespace()
                .take(4)
                .map(str::to_owned)
                .collect();
            let next = p(&(fields.join(" ") + " 0 1"));
            stack.push(next);
            edges += 1;
        }
    }
    assert_eq!(seen.len(), 1152);
    assert_eq!(edges, 6240);
}

#[test]
fn forced_capture_dead_and_one_sided_impossibility_are_not_material_guesses() {
    let initial = p(FORCED);
    assert_eq!(initial.dead_recognition(), DeadRecognition::Unknown);
    assert_eq!(initial.legal_actions(), vec![action("a1b1")]);
    let mut leaf = initial.clone();
    leaf.apply(action("a1b1")).unwrap();
    assert_eq!(leaf.dead_recognition(), DeadRecognition::ProvenDead);
    assert!(matches!(
        initial.mate_reachability(MateTarget::Either, unlimited(), || false),
        MateReachability::Impossible {
            work: ReachabilityWork {
                positions: 2,
                static_leaves: 1,
                ..
            }
        }
    ));
    // Black has KBR, White K: the forced Kxb1 removes Black's rook.
    // The resulting KB versus K permits neither side to mate.
    for side in [Color::White, Color::Black] {
        assert!(matches!(
            initial.mate_reachability(MateTarget::Side(side), unlimited(), || false),
            MateReachability::Impossible { .. }
        ));
    }
    let rook = p("7k/8/8/8/8/8/8/KR6 w - - 0 1");
    assert!(matches!(
        rook.mate_reachability(MateTarget::Side(Color::Black), unlimited(), || false),
        MateReachability::Impossible { .. }
    ));
}

#[test]
fn positive_witnesses_are_complete_legal_actions_and_counter_independent() {
    // Qg7 is mate; no earlier terminal state or engine score supplies the answer.
    for counter in ["0 1", "149 4294967295"] {
        let position = p(&format!("7k/8/5KQ1/8/8/8/8/8 w - - {counter}"));
        let before = position.clone();
        let MateReachability::Possible {
            witness: actions, ..
        } = position.mate_reachability(MateTarget::Side(Color::White), unlimited(), || false)
        else {
            panic!("missing mate");
        };
        assert_eq!(actions.len(), 1);
        witness(&position, Color::White, &actions);
        assert_eq!(position, before);
    }
    // Already delivered mate permits the empty witness, for its winner only.
    let position = p("7k/6Q1/5K2/8/8/8/8/8 b - - 0 1");
    assert!(
        matches!(position.mate_reachability(MateTarget::Side(Color::White), unlimited(), || false), MateReachability::Possible { witness, .. } if witness.is_empty())
    );
    assert!(matches!(
        position.mate_reachability(MateTarget::Side(Color::Black), unlimited(), || false),
        MateReachability::Impossible { .. }
    ));
    let stalemate = p("7k/5K2/6Q1/8/8/8/8/8 b - - 0 1");
    assert!(matches!(
        stalemate.mate_reachability(MateTarget::Either, unlimited(), || false),
        MateReachability::Impossible { .. }
    ));
}

#[test]
fn budgets_and_cancellation_are_unknown_and_leave_input_unchanged() {
    let position = p(WALL);
    let before = position.clone();
    for (limits, reason) in [
        (
            ReachabilityLimits {
                max_positions: Some(0),
                max_transitions: None,
            },
            ReachabilityStop::PositionLimit,
        ),
        (
            ReachabilityLimits {
                max_positions: Some(1151),
                max_transitions: None,
            },
            ReachabilityStop::PositionLimit,
        ),
        (
            ReachabilityLimits {
                max_positions: None,
                max_transitions: Some(6239),
            },
            ReachabilityStop::TransitionLimit,
        ),
    ] {
        assert!(
            matches!(position.mate_reachability(MateTarget::Either, limits, || false), MateReachability::Unknown { reason: actual, .. } if actual == reason)
        );
    }
    let exact_budget = ReachabilityLimits {
        max_positions: Some(1152),
        max_transitions: Some(6240),
    };
    assert!(matches!(
        position.mate_reachability(MateTarget::Either, exact_budget, || false),
        MateReachability::Impossible { .. }
    ));
    let mut polls = 0;
    assert!(matches!(
        position.mate_reachability(MateTarget::Either, unlimited(), || {
            polls += 1;
            polls == 5
        }),
        MateReachability::Unknown {
            reason: ReachabilityStop::Cancelled,
            ..
        }
    ));
    assert!(matches!(
        position.mate_reachability(MateTarget::Either, unlimited(), || true),
        MateReachability::Unknown {
            reason: ReachabilityStop::Cancelled,
            ..
        }
    ));
    assert_eq!(position, before);
}

#[test]
fn full_live_adjudication_records_mode_preserves_legacy_and_restores_undo() {
    let legacy = AdjudicatedGame::from_fen(WALL, ClaimPolicy::ExplicitOnly).unwrap();
    assert_eq!(legacy.outcome(), None);
    assert!(
        legacy
            .record()
            .to_json()
            .starts_with("{\"schema_version\":1,")
    );
    let game =
        AdjudicatedGame::from_fen_with_mode(WALL, ClaimPolicy::ExplicitOnly, full(unlimited()))
            .unwrap();
    assert_eq!(
        game.outcome(),
        Some(ChessOutcome::Draw(DrawReason::DeadPosition))
    );
    assert!(game.available_claims().is_empty());
    let record = game.record();
    assert!(record.to_json().starts_with("{\"schema_version\":2,"));
    let decoded = GameRecord::from_json(&record.to_json()).unwrap();
    assert_eq!(AdjudicatedGame::replay_record(&decoded).unwrap(), game);
    let mut limited = AdjudicatedGame::from_fen_with_mode(
        WALL,
        ClaimPolicy::ExplicitOnly,
        full(ReachabilityLimits {
            max_positions: Some(1),
            max_transitions: None,
        }),
    )
    .unwrap();
    assert_eq!(limited.outcome(), None);
    let before = limited.clone();
    limited.apply(action("a2a1")).unwrap();
    limited.undo().unwrap();
    assert_eq!(limited, before);
    let mut v: serde_json::Value = serde_json::from_str(&record.to_json()).unwrap();
    v["schema_version"] = serde_json::json!(1);
    assert!(GameRecord::from_json(&v.to_string()).is_err());
    assert!(
        AdjudicatedGame::from_history_with_mode(
            {
                let mut g = Game::from_fen(WALL, ClaimPolicy::ExplicitOnly).unwrap();
                g.apply(action("a2a1")).unwrap();
                g
            },
            full(unlimited())
        )
        .is_err()
    );
}

fn send(m: &mut ChessMatch, at_ns: u64, actor: Color, command: MatchCommand) -> MatchReply {
    m.submit(MatchRequest {
        at_ns,
        version: m.version(),
        actor,
        command,
    })
}
#[test]
fn full_timeout_resignation_use_generated_proofs_and_replay_configuration() {
    // White can mate immediately; Black's resignation/flag needs no supplied
    // witness in full mode. Limited exploration still cannot invent a score.
    for flag in [false, true] {
        let g = AdjudicatedGame::from_fen_with_mode(
            "7k/8/5KQ1/8/8/8/8/8 w - - 0 1",
            ClaimPolicy::ExplicitOnly,
            full(ReachabilityLimits {
                max_positions: Some(1000),
                max_transitions: Some(1000),
            }),
        )
        .unwrap();
        let mut m = ChessMatch::new(
            g,
            if flag {
                TimeControl::Increment {
                    initial_ns: [100, 100],
                    increment_ns: 0,
                }
            } else {
                TimeControl::Untimed
            },
        )
        .unwrap();
        send(&mut m, 0, Color::White, MatchCommand::Start);
        let reply = if flag {
            // Give Black the clock while preserving a short White mating path.
            send(&mut m, 1, Color::White, MatchCommand::Move(action("g6g5")));
            send(
                &mut m,
                101,
                Color::Black,
                MatchCommand::Observe { witness: None },
            )
        } else {
            send(
                &mut m,
                1,
                Color::Black,
                MatchCommand::Resign { witness: None },
            )
        };
        if !flag {
            assert!(matches!(
                reply.conclusion,
                Some(MatchConclusion::Win {
                    winner: Color::White,
                    ..
                })
            ));
        } else {
            assert!(matches!(
                reply.conclusion,
                Some(MatchConclusion::Win {
                    winner: Color::White,
                    trigger: TerminationTrigger::FlagFall
                })
            ));
        }
        assert!(reply.feasibility_evidence.is_some());
        let json = m.record().to_json();
        assert!(json.starts_with("{\"schema_version\":2,"));
        assert_eq!(MatchRecord::from_json(&json).unwrap().replay().unwrap(), m);
        let mut corrupt: serde_json::Value = serde_json::from_str(&json).unwrap();
        let last = corrupt["events"]
            .as_array_mut()
            .unwrap()
            .last_mut()
            .unwrap();
        last["reply"]["feasibility_evidence"]["possible"]["work"]["positions"] =
            serde_json::json!(999999);
        assert!(
            MatchRecord::from_json(&corrupt.to_string())
                .unwrap()
                .replay()
                .is_err()
        );
        let mut altered: serde_json::Value = serde_json::from_str(&json).unwrap();
        altered["schema_version"] = serde_json::json!(1);
        assert!(
            MatchRecord::from_json(&altered.to_string())
                .unwrap()
                .replay()
                .is_err()
        );
    }
    let g = AdjudicatedGame::from_fen_with_mode(
        "7k/8/5KQ1/8/8/8/8/8 w - - 0 1",
        ClaimPolicy::ExplicitOnly,
        full(ReachabilityLimits {
            max_positions: Some(0),
            max_transitions: None,
        }),
    )
    .unwrap();
    let mut m = ChessMatch::new(g, TimeControl::Untimed).unwrap();
    send(&mut m, 0, Color::White, MatchCommand::Start);
    let reply = send(
        &mut m,
        1,
        Color::Black,
        MatchCommand::Resign { witness: None },
    );
    assert!(matches!(
        reply.conclusion,
        Some(MatchConclusion::Unresolved { .. })
    ));
    assert_eq!(reply.conclusion.unwrap().white_score_twice(), None);
    assert!(matches!(
        reply.feasibility_evidence,
        Some(MateReachability::Unknown { .. })
    ));
}

#[test]
fn a_committed_move_can_create_full_deadness_and_undo_restores_analysis() {
    let mut game = AdjudicatedGame::from_fen_with_mode(
        "8/7k/8/p1p1p1p1/P1P3P1/4P3/K7/8 w - - 0 1",
        ClaimPolicy::ExplicitOnly,
        full(ReachabilityLimits {
            max_positions: Some(2000),
            max_transitions: None,
        }),
    )
    .unwrap();
    assert_eq!(game.outcome(), None);
    let before = game.clone();
    game.apply(action("e3e4")).unwrap();
    assert_eq!(game.position().dead_recognition(), DeadRecognition::Unknown);
    assert_eq!(
        game.outcome(),
        Some(ChessOutcome::Draw(DrawReason::DeadPosition))
    );
    assert_eq!(
        AdjudicatedGame::replay_record(&GameRecord::from_json(&game.record().to_json()).unwrap())
            .unwrap(),
        game
    );
    let mut match_game = ChessMatch::new(game.clone(), TimeControl::Untimed).unwrap();
    let reply = send(
        &mut match_game,
        0,
        Color::Black,
        MatchCommand::Resign { witness: None },
    );
    assert_eq!(
        reply.conclusion,
        Some(MatchConclusion::Chess(ChessOutcome::Draw(
            DrawReason::DeadPosition
        )))
    );
    assert_eq!(reply.rejection.unwrap().code, MatchRejectCode::GameOver);
    game.undo().unwrap();
    assert_eq!(game, before);
}

#[test]
fn zero_full_budget_does_not_erase_an_existing_sound_side_specific_proof() {
    let g = AdjudicatedGame::from_fen_with_mode(
        "7k/8/8/8/8/8/8/KR6 w - - 0 1",
        ClaimPolicy::ExplicitOnly,
        full(ReachabilityLimits {
            max_positions: Some(0),
            max_transitions: None,
        }),
    )
    .unwrap();
    let mut m = ChessMatch::new(
        g,
        TimeControl::Increment {
            initial_ns: [1, 1],
            increment_ns: 0,
        },
    )
    .unwrap();
    send(&mut m, 0, Color::White, MatchCommand::Start);
    let reply = send(
        &mut m,
        1,
        Color::White,
        MatchCommand::Observe { witness: None },
    );
    assert_eq!(
        reply.conclusion,
        Some(MatchConclusion::Draw {
            trigger: Some(TerminationTrigger::FlagFall)
        })
    );
    assert_eq!(
        MatchRecord::from_json(&m.record().to_json())
            .unwrap()
            .replay()
            .unwrap(),
        m
    );
}
