//! Independently frozen geometry/count facts, not comparator outcome helpers.
use mshogi_core::xiangqi::*;
use std::collections::{BTreeSet, HashSet};
fn limits() -> ExactLimits {
    ExactLimits {
        states: 5000,
        traversal: ExactTraversal::BreadthFirst,
        transitions: 50_000,
        millis: 60_000,
        ..ExactLimits::default()
    }
}
fn positive() -> Position {
    Position::from_fen("5k3/R8/9/9/9/9/9/9/9/3K5 b - - 0 1").unwrap()
}
fn replay(p: &Position, report: &ExactReport) {
    let ExactVerdict::NaturalWin {
        winner,
        reason,
        witness,
    } = &report.verdict
    else {
        panic!("{report:?}")
    };
    let mut next = p.clone();
    // Counters are declared notation, not proof constraints, including overflow.
    next = Position::from_diagram(
        &next.pieces().collect::<Vec<_>>(),
        next.side_to_move(),
        Counters::unknown(),
    )
    .unwrap();
    for &action in witness {
        assert!(matches!(
            next.local_status(),
            Ok(LocalStatus::Ongoing { .. })
        ));
        next.apply(action).unwrap();
    }
    assert_eq!(next.side_to_move().opponent(), *winner);
    assert_eq!(
        next.local_status().unwrap(),
        LocalStatus::NoLegalAction { reason: *reason }
    );
}
#[test]
fn independently_expected_stalemate_witness_beyond_fast_path_and_counter_quotient() {
    let p1 = Position::from_fen("5k3/9/4R4/9/9/9/9/9/9/3K5 w - - 0 1").unwrap();
    let mut p1_end = p1.clone();
    p1_end.apply("e7e8".parse().unwrap()).unwrap();
    assert_eq!(
        p1_end.local_status().unwrap(),
        LocalStatus::NoLegalAction {
            reason: NoActionReason::Stalemate
        }
    );
    replay(&p1, &p1.verify_natural_win(limits(), || false));
    let p = positive();
    assert_eq!(
        assess_natural_win(&p),
        NoWinAssessment::UnknownExactUnavailable
    );
    assert_eq!(p.legal_actions().unwrap(), vec!["f9e9".parse().unwrap()]);
    let mut expected = p.clone();
    expected.apply("f9e9".parse().unwrap()).unwrap();
    assert!(matches!(
        expected.local_status(),
        Ok(LocalStatus::Ongoing { .. })
    ));
    expected.apply("a8f8".parse().unwrap()).unwrap();
    assert_eq!(
        expected.local_status().unwrap(),
        LocalStatus::NoLegalAction {
            reason: NoActionReason::Stalemate
        }
    );
    let report = p.verify_natural_win(limits(), || false);
    replay(&p, &report);
    if let ExactVerdict::NaturalWin { witness, .. } = &report.verdict {
        assert_eq!(witness.len(), 2);
    }
    let counters = Position::from_diagram(
        &p.pieces().collect::<Vec<_>>(),
        p.side_to_move(),
        Counters::new(Some(u32::MAX), Some(u32::MAX)).unwrap(),
    )
    .unwrap();
    let other = counters.verify_natural_win(limits(), || false);
    assert_eq!(report.verdict, other.verdict);
    assert_eq!(report.work.states, other.work.states);
    assert_eq!(counters.counters().fullmove_number(), Some(u32::MAX));
    for fen in [
        "4k4/9/9/9/9/4p4/9/9/3r1r3/4K4 w - - 0 1",
        "4k4/9/9/9/9/4p4/9/2n6/5r3/3rK4 w - - 0 1",
    ] {
        let p = Position::from_fen(fen).unwrap();
        let r = p.verify_natural_win(limits(), || false);
        replay(&p, &r);
        assert!(
            matches!(r.verdict, ExactVerdict::NaturalWin { witness, .. } if witness.is_empty())
        );
    }
}
// Independent explicit palace model, without product attack/legal/apply helpers.
// Tuple is Red General, Black General, optional Red/Black Advisor and whose turn.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Small {
    red: (u8, u8),
    black: (u8, u8),
    advisor: Option<(u8, u8, Side)>,
    turn: Side,
}
impl Small {
    fn valid(self) -> bool {
        let (r, b) = (self.red, self.black);
        if self
            .advisor
            .is_some_and(|(x, y, _)| (x, y) == r || (x, y) == b)
        {
            return false;
        }
        r.0 != b.0
            || self
                .advisor
                .is_some_and(|(x, y, _)| x == r.0 && y > r.1 && y < b.1)
    }
    fn choices(self) -> Vec<(Action, Self)> {
        let mut out = vec![];
        let (x, y) = if self.turn == Side::Red {
            self.red
        } else {
            self.black
        };
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let (nx, ny) = (x as i8 + dx, y as i8 + dy);
            let home = if self.turn == Side::Red { 0..=2 } else { 7..=9 };
            if !(3..=5).contains(&nx) || !home.contains(&ny) {
                continue;
            }
            let mut next = self;
            if self.turn == Side::Red {
                next.red = (nx as u8, ny as u8);
            } else {
                next.black = (nx as u8, ny as u8);
            }
            next.turn = self.turn.opponent();
            if next.valid() {
                out.push((
                    Action {
                        from: Intersection::new(x, y).unwrap(),
                        to: Intersection::new(nx as u8, ny as u8).unwrap(),
                    },
                    next,
                ));
            }
        }
        if let Some((x, y, side)) = self.advisor.filter(|(_, _, s)| *s == self.turn) {
            for (dx, dy) in [(1, 1), (-1, 1), (1, -1), (-1, -1)] {
                let (nx, ny) = (x as i8 + dx, y as i8 + dy);
                let home = if side == Side::Red { 0..=2 } else { 7..=9 };
                if !(3..=5).contains(&nx) || !home.contains(&ny) {
                    continue;
                }
                let next = Self {
                    advisor: Some((nx as u8, ny as u8, side)),
                    turn: side.opponent(),
                    ..self
                };
                if next.valid() {
                    out.push((
                        Action {
                            from: Intersection::new(x, y).unwrap(),
                            to: Intersection::new(nx as u8, ny as u8).unwrap(),
                        },
                        next,
                    ));
                }
            }
        }
        out
    }
    fn position(self) -> Position {
        let mut pieces = vec![
            (
                Intersection::new(self.red.0, self.red.1).unwrap(),
                Piece::new(Side::Red, PieceKind::General),
            ),
            (
                Intersection::new(self.black.0, self.black.1).unwrap(),
                Piece::new(Side::Black, PieceKind::General),
            ),
        ];
        if let Some((x, y, s)) = self.advisor {
            pieces.push((
                Intersection::new(x, y).unwrap(),
                Piece::new(s, PieceKind::Advisor),
            ));
        }
        Position::from_diagram(&pieces, self.turn, Counters::unknown()).unwrap()
    }
}
#[test]
fn exhaustive_independent_palace_universes_and_closed_graph_negative_proofs() {
    for advisor_side in [None, Some(Side::Red), Some(Side::Black)] {
        let mut universe = HashSet::new();
        for rx in 3..=5 {
            for ry in 0..=2 {
                for bx in 3..=5 {
                    for by in 7..=9 {
                        for turn in [Side::Red, Side::Black] {
                            let advisors = if let Some(s) = advisor_side {
                                vec![(3, 0), (5, 0), (4, 1), (3, 2), (5, 2)]
                                    .into_iter()
                                    .map(|(x, y)| {
                                        Some((x, if s == Side::Red { y } else { 9 - y }, s))
                                    })
                                    .collect()
                            } else {
                                vec![None]
                            };
                            for advisor in advisors {
                                let state = Small {
                                    red: (rx, ry),
                                    black: (bx, by),
                                    advisor,
                                    turn,
                                };
                                if state.valid() {
                                    universe.insert(state);
                                }
                            }
                        }
                    }
                }
            }
        }
        assert_eq!(
            universe.len(),
            if advisor_side.is_none() { 108 } else { 510 }
        );
        for &s in &universe {
            let expected = s.choices();
            assert!(!expected.is_empty(), "{s:?}");
            let p = s.position();
            let choices = p
                .legal_actions()
                .unwrap()
                .into_iter()
                .map(|a| a.to_string())
                .collect::<BTreeSet<_>>();
            assert_eq!(
                choices,
                expected.iter().map(|(a, _)| a.to_string()).collect(),
                "{s:?}"
            );
            for (a, next) in expected {
                assert!(universe.contains(&next));
                let mut native = p.clone();
                native.apply(a).unwrap();
                assert_eq!(native, next.position());
            }
        }
        let root = Small {
            red: (3, 0),
            black: (5, 9),
            advisor: advisor_side.map(|s| (4, if s == Side::Red { 1 } else { 8 }, s)),
            turn: Side::Red,
        };
        let mut seen = HashSet::from([root]);
        let mut stack = vec![root];
        let mut edges = 0;
        while let Some(s) = stack.pop() {
            for (_, next) in s.choices() {
                edges += 1;
                if seen.insert(next) {
                    stack.push(next);
                }
            }
        }
        for traversal in [ExactTraversal::BreadthFirst, ExactTraversal::DepthFirst] {
            let report = root.position().verify_natural_win(
                ExactLimits {
                    traversal,
                    ..limits()
                },
                || false,
            );
            assert_eq!(report.verdict, ExactVerdict::NeitherCanWin, "{report:?}");
            assert_eq!(report.work.states as usize, seen.len());
            assert_eq!(report.work.expanded as usize, seen.len());
            assert_eq!(report.work.transitions, edges);
            assert_eq!(report.work.cache_hits, edges - (seen.len() as u64 - 1));
        }
    }
}
#[test]
fn every_resource_stop_is_unknown_and_failures_are_separate() {
    let p = positive();
    for (l, reason) in [
        (
            ExactLimits {
                states: 0,
                ..limits()
            },
            ExactStop::States,
        ),
        (
            ExactLimits {
                states: 1,
                ..limits()
            },
            ExactStop::States,
        ),
        (
            ExactLimits {
                transitions: 0,
                ..limits()
            },
            ExactStop::Transitions,
        ),
        (
            ExactLimits {
                memory_bytes: 0,
                ..limits()
            },
            ExactStop::Memory,
        ),
        (
            ExactLimits {
                millis: 0,
                ..limits()
            },
            ExactStop::Deadline,
        ),
        (
            ExactLimits {
                witness_plies: 0,
                ..limits()
            },
            ExactStop::Witness,
        ),
    ] {
        assert_eq!(
            p.verify_natural_win(l, || false).verdict,
            ExactVerdict::Unknown { reason }
        );
    }
    assert_eq!(
        p.verify_natural_win(limits(), || true).verdict,
        ExactVerdict::Unknown {
            reason: ExactStop::Cancelled
        }
    );
    let mut polls = 0;
    let report = p.verify_natural_win(limits(), || {
        polls += 1;
        polls == 8
    });
    assert_eq!(
        report.verdict,
        ExactVerdict::Unknown {
            reason: ExactStop::Cancelled
        }
    );
    assert!(report.work.states > 0);
    assert!(matches!(
        p.verify_natural_win(
            ExactLimits {
                states: MAX_EXACT_STATES + 1,
                ..limits()
            },
            || false
        )
        .verdict,
        ExactVerdict::Failure {
            reason: ExactFailure::UnsupportedLimits
        }
    ));
    let invalid = Position::from_fen("4k4/9/9/9/9/9/9/9/9/4K4 w - - 0 1").unwrap();
    assert!(matches!(
        invalid.verify_natural_win(limits(), || false).verdict,
        ExactVerdict::Failure {
            reason: ExactFailure::InvalidPosition
        }
    ));
}
#[test]
fn exact_native_request_history_selection_replacement_undo_reopen_and_save_are_safe() {
    let mut game =
        Game::from_root(PositionRoot::analysis(positive()), ClaimPolicy::Explicit).unwrap();
    let original = GameRecord::from_game(&game).unwrap();
    let mut resource = ExactAnalysis::new().unwrap();
    let first = resource.begin(&game, limits()).unwrap();
    assert_eq!(first.purpose(), ExactPurpose::NaturalBoardWin);
    assert_eq!(first.source_record(), &original);
    let result = first.run(|| false);
    replay(game.position(), result.report());
    let current = resource
        .begin(
            &game,
            ExactLimits {
                transitions: 0,
                ..limits()
            },
        )
        .unwrap();
    assert_ne!(first.identity(), current.identity());
    assert_eq!(
        resource.accept(&game, &result),
        Err(GameError::StaleContext)
    );
    let unknown = current.run(|| false);
    assert!(matches!(
        resource.accept(&game, &unknown).unwrap().verdict,
        ExactVerdict::Unknown { .. }
    ));
    assert_eq!(GameRecord::from_game(&game).unwrap(), original);
    let request = resource.begin(&game, limits()).unwrap();
    let result = request.run(|| false);
    let undo = game
        .submit(GameRequest {
            expected: game.context(),
            actor: Side::Black,
            command: GameCommand::Play {
                action: "f9e9".parse().unwrap(),
            },
        })
        .unwrap()
        .undo;
    assert_eq!(
        resource.accept(&game, &result),
        Err(GameError::StaleContext)
    );
    game.undo(undo).unwrap();
    assert_eq!(GameRecord::from_game(&game).unwrap(), original);
    assert_eq!(
        resource.accept(&game, &result),
        Err(GameError::StaleContext)
    );
    let request = resource.begin(&game, limits()).unwrap();
    let result = request.run(|| false);
    resource
        .select(ExactTarget::History { event_count: 0 })
        .unwrap();
    resource.select(ExactTarget::Live).unwrap();
    assert_eq!(
        resource.accept(&game, &result),
        Err(GameError::StaleContext)
    );
    let request = resource.begin(&game, limits()).unwrap();
    let cancelled = request.run(|| true);
    resource.retire().unwrap();
    assert_eq!(
        resource.accept(&game, &cancelled),
        Err(GameError::StaleContext)
    );
    let reopened = original.to_game().unwrap();
    let req = resource.begin(&game, limits()).unwrap();
    assert_eq!(
        resource.accept(&reopened, &req.run(|| false)),
        Err(GameError::StaleContext)
    );
    resource
        .select(ExactTarget::History { event_count: 0 })
        .unwrap();
    let historical = resource.begin(&game, limits()).unwrap();
    assert_eq!(historical.position(), game.root().position());
    assert_eq!(GameRecord::from_game(&game).unwrap(), original);
    // Saved fast-only mode remains honest: no implicit exact replay or pending work.
    let unsupported = original
        .to_json()
        .replace("sound_fast_only", "bounded_exact");
    assert_eq!(
        GameRecord::from_json(&unsupported),
        Err(RecordError::Unsupported)
    );
}

#[test]
fn actual_same_board_histories_do_not_create_intrinsic_win_or_revive_requests() {
    let mut game = Game::from_root(
        PositionRoot::from_fen("4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1").unwrap(),
        ClaimPolicy::Explicit,
    )
    .unwrap();
    let before = GameRecord::from_game(&game).unwrap();
    let mut resource = ExactAnalysis::new().unwrap();
    let request = resource
        .begin(
            &game,
            ExactLimits {
                states: 1,
                ..limits()
            },
        )
        .unwrap();
    let observation = request.run(|| false);
    for _ in 0..3 {
        for a in ["e9f9", "f8e8", "f9e9", "e8f8"] {
            game.submit(GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: GameCommand::Play {
                    action: a.parse().unwrap(),
                },
            })
            .unwrap();
        }
    }
    assert_eq!(
        game.position().pieces().collect::<Vec<_>>(),
        request.position().pieces().collect::<Vec<_>>()
    );
    assert!(matches!(
        game.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Black,
            reason: WinReason::Repetition { .. }
        })
    ));
    assert_eq!(
        resource.accept(&game, &observation),
        Err(GameError::StaleContext)
    );
    let after = GameRecord::from_game(&game).unwrap();
    assert_ne!(after, before);
    let now = resource
        .begin(
            &game,
            ExactLimits {
                states: 1,
                ..limits()
            },
        )
        .unwrap();
    assert_ne!(now.source_record(), request.source_record());
    assert_eq!(
        now.run(|| false).report().verdict,
        observation.report().verdict
    );
    assert_eq!(
        game.submit(GameRequest {
            expected: game.context(),
            actor: Side::Red,
            command: GameCommand::Play {
                action: "e9f9".parse().unwrap()
            }
        })
        .unwrap_err(),
        GameError::Concluded
    );
    resource
        .select(ExactTarget::History { event_count: 0 })
        .unwrap();
    let historical = resource.begin(&game, limits()).unwrap();
    assert_eq!(historical.position(), game.root().position());
    let result = historical.run(|| true);
    let accepted = resource.accept(&game, &result).unwrap();
    assert_eq!(
        accepted.assessment(),
        NoWinAssessment::UnknownExactIncomplete {
            reason: ExactStop::Cancelled
        }
    );
    assert_eq!(GameRecord::from_game(&game).unwrap(), after);
    assert_eq!(after.to_game().unwrap().conclusion(), game.conclusion());
}
#[test]
fn exact_witness_native_sides_and_memory_boundary_are_preserved() {
    let p = positive();
    let pieces = p
        .pieces()
        .map(|(at, piece)| {
            (
                Intersection::new(8 - at.file(), 9 - at.rank()).unwrap(),
                Piece::new(piece.side.opponent(), piece.kind),
            )
        })
        .collect::<Vec<_>>();
    let mirrored =
        Position::from_diagram(&pieces, p.side_to_move().opponent(), Counters::unknown()).unwrap();
    let r = mirrored.verify_natural_win(limits(), || false);
    replay(&mirrored, &r);
    assert!(matches!(
        r.verdict,
        ExactVerdict::NaturalWin {
            winner: Side::Black,
            ..
        }
    ));
    let memory = r.work.memory_bytes;
    let at = p.verify_natural_win(
        ExactLimits {
            memory_bytes: memory,
            ..limits()
        },
        || false,
    );
    assert!(matches!(at.verdict, ExactVerdict::NaturalWin { .. }));
    let below = p.verify_natural_win(
        ExactLimits {
            memory_bytes: memory - 1,
            ..limits()
        },
        || false,
    );
    assert_eq!(
        below.verdict,
        ExactVerdict::Unknown {
            reason: ExactStop::Memory
        }
    );
    assert!(below.work.memory_bytes < memory);
}
