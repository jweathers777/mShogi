use super::*;
fn square(f: u8, r: u8) -> Square {
    Square::new(f, r).unwrap()
}
fn game(profile: Profile, entries: &[(Side, Base, bool, u8, u8)], turn: Side) -> Game {
    let pieces = entries
        .iter()
        .enumerate()
        .map(|(i, (side, base, promoted, f, r))| {
            (
                square(*f, *r),
                Piece::new(i as u16, *side, *base, *promoted).unwrap(),
            )
        })
        .collect::<Vec<_>>();
    Game::from_root(
        Position::from_diagram(profile, &pieces, turn, RootMetadata::diagram(0)).unwrap(),
    )
    .unwrap()
}
fn corner(profile: Profile) -> Game {
    game(
        profile,
        &[
            (Side::Sente, Base::K, false, 16, 16),
            (Side::Sente, Base::P, false, 16, 14),
            (Side::Gote, Base::K, false, 1, 1),
            (Side::Gote, Base::P, false, 1, 3),
        ],
        Side::Sente,
    )
}
fn submit(g: &mut Game, actor: Side, command: GameCommand) -> Result<GameUndo, GameError> {
    g.submit(GameRequest {
        expected: g.context(),
        actor,
        command,
    })
}
fn play(g: &mut Game, from: (u8, u8), to: (u8, u8)) -> GameUndo {
    let actor = g.position().side_to_move();
    submit(
        g,
        actor,
        GameCommand::Play {
            action: Action::single(square(from.0, from.1), square(to.0, to.1)),
        },
    )
    .unwrap()
}
#[test]
fn first_repetition_atomic_distinct_saved_and_runtime_domains() {
    for profile in Profile::ALL {
        let mut g = corner(profile);
        let root = g.repetition_key().clone();
        play(&mut g, (16, 16), (15, 16));
        play(&mut g, (1, 1), (2, 1));
        play(&mut g, (15, 16), (16, 16));
        let before = GameRecord::from_game(&g).unwrap();
        let context = g.context();
        let snap = g.snapshot().clone();
        assert_eq!(
            submit(
                &mut g,
                Side::Gote,
                GameCommand::Play {
                    action: Action::single(square(2, 1), square(1, 1))
                }
            )
            .unwrap_err(),
            GameError::Repetition
        );
        assert_eq!(before, GameRecord::from_game(&g).unwrap());
        assert_eq!(context, g.context());
        assert_eq!(snap, *g.snapshot());
        assert_ne!(&root, g.repetition_key());
    }
}
#[test]
fn exact_undo_redo_retire_runtime_and_new_branch_tokens() {
    for profile in Profile::ALL {
        let mut g = corner(profile);
        let before = GameRecord::from_game(&g).unwrap();
        let old = g.context();
        let mut u = play(&mut g, (16, 16), (15, 16));
        let after = GameRecord::from_game(&g).unwrap();
        let mut r = g.undo(&mut u).unwrap();
        assert_eq!(before, GameRecord::from_game(&g).unwrap());
        assert_ne!(old, g.context());
        assert!(old.same_semantics(&g.context()));
        assert_eq!(g.undo(&mut u).unwrap_err(), GameError::InvalidToken);
        let mut u2 = g.redo(&mut r).unwrap();
        assert_eq!(after, GameRecord::from_game(&g).unwrap());
        assert!(g.redo(&mut r).is_err());
        let mut abandoned = g.undo(&mut u2).unwrap();
        play(&mut g, (16, 16), (16, 15));
        assert_eq!(g.redo(&mut abandoned).unwrap_err(), GameError::InvalidToken);
        let mut other = corner(profile);
        assert_eq!(other.undo(&mut u2).unwrap_err(), GameError::InvalidToken);
    }
}
#[test]
fn two_profiles_contexts_cannot_cross_even_equal_boards() {
    let mut a = corner(Profile::AdamsModern);
    let b = corner(Profile::MullerCompleted);
    let before = GameRecord::from_game(&a).unwrap();
    assert_eq!(
        a.position().pieces().collect::<Vec<_>>(),
        b.position().pieces().collect::<Vec<_>>()
    );
    assert!(!a.context().same_semantics(&b.context()));
    assert_ne!(a.repetition_key(), b.repetition_key());
    assert_eq!(
        a.submit(GameRequest {
            expected: b.context(),
            actor: Side::Sente,
            command: GameCommand::Resign
        })
        .unwrap_err(),
        GameError::StaleContext
    );
    assert_eq!(before, GameRecord::from_game(&a).unwrap());
}
#[test]
fn actual_consent_stop_and_resign_do_not_become_board_actions() {
    for profile in Profile::ALL {
        let mut g = corner(profile);
        let state = g.position().state().clone();
        submit(&mut g, Side::Gote, GameCommand::OfferDraw).unwrap();
        assert_eq!(
            submit(&mut g, Side::Gote, GameCommand::AcceptDraw).unwrap_err(),
            GameError::DrawOffer
        );
        submit(&mut g, Side::Sente, GameCommand::DeclineDraw).unwrap();
        submit(
            &mut g,
            Side::Sente,
            GameCommand::Stop {
                reason: StopReason::WorkLimit,
            },
        )
        .unwrap();
        assert_eq!(g.position().state(), &state);
        assert_eq!(g.snapshot().board_ply, 0);
        assert_eq!(g.conclusion(), None);
        submit(&mut g, Side::Sente, GameCommand::OfferDraw).unwrap();
        submit(&mut g, Side::Gote, GameCommand::AcceptDraw).unwrap();
        assert_eq!(g.conclusion(), Some(Conclusion::Agreement));
        assert!(submit(&mut g, Side::Sente, GameCommand::Resign).is_err());
        let mut other = corner(profile);
        submit(&mut other, Side::Gote, GameCommand::Resign).unwrap();
        assert_eq!(
            other.conclusion(),
            Some(Conclusion::Resignation {
                winner: Side::Sente
            })
        );
    }
}
#[test]
fn historical_prefix_is_exact_immutable_and_retires_authority() {
    let mut g = corner(Profile::AdamsModern);
    let root = g.snapshot().clone();
    play(&mut g, (16, 16), (15, 16));
    let context = g.context();
    assert_eq!(g.select_prefix(0).unwrap(), root);
    assert_ne!(context, g.context());
    assert_eq!(g.snapshot().event_count, 1);
    let p = g.record_prefix(0).unwrap().to_game().unwrap();
    assert_eq!(p.snapshot(), &root);
    assert!(g.event_prefix(2).is_err());
}
#[test]
fn bare_winning_royal_reply_and_simultaneous_effect_priority() {
    for profile in Profile::ALL {
        let mut g = game(
            profile,
            &[
                (Side::Sente, Base::K, false, 8, 8),
                (Side::Sente, Base::P, false, 5, 10),
                (Side::Gote, Base::K, false, 9, 8),
                (Side::Gote, Base::R, false, 5, 1),
            ],
            Side::Gote,
        );
        play(&mut g, (5, 1), (5, 10));
        assert_eq!(g.snapshot().pending_bare, Some(Side::Sente));
        assert_eq!(g.conclusion(), None);
        play(&mut g, (8, 8), (9, 8));
        assert_eq!(
            g.conclusion(),
            Some(Conclusion::RoyalCapture {
                winner: Side::Sente
            })
        );
        let mut h = game(
            profile,
            &[
                (Side::Sente, Base::K, false, 8, 9),
                (Side::Sente, Base::P, false, 16, 14),
                (Side::Gote, Base::K, false, 8, 8),
                (Side::Gote, Base::FiD, false, 9, 8),
            ],
            Side::Sente,
        );
        let before = GameRecord::from_game(&h).unwrap();
        let mut undo = play(&mut h, (8, 9), (8, 8));
        assert_eq!(h.conclusion(), Some(Conclusion::SimultaneousRoyalLoss));
        let mut redo = h.undo(&mut undo).unwrap();
        assert_eq!(before, GameRecord::from_game(&h).unwrap());
        h.redo(&mut redo).unwrap();
        assert_eq!(h.conclusion(), Some(Conclusion::SimultaneousRoyalLoss));
    }
}
#[test]
fn cancellation_during_candidates_preserves_every_accepted_component() {
    let mut g = Game::standard(Profile::AdamsModern);
    let context = g.context();
    let before = GameRecord::from_game(&g).unwrap();
    let a = g.legal_actions()[0].clone();
    let mut calls = 0;
    let e = g
        .submit_with_stop(
            GameRequest {
                expected: context.clone(),
                actor: Side::Sente,
                command: GameCommand::Play { action: a },
            },
            &mut || {
                calls += 1;
                calls > 4
            },
        )
        .unwrap_err();
    assert_eq!(e, GameError::Stopped);
    assert_eq!(before, GameRecord::from_game(&g).unwrap());
    assert_eq!(context, g.context());
    let mut stopped = || true;
    assert_eq!(
        g.legal_actions_with_stop(&mut stopped).unwrap_err(),
        GameError::Stopped
    );
}
#[test]
fn root_provenance_and_declared_counter_are_not_played_history() {
    let profile = Profile::AdamsModern;
    let p = Position::standard(profile);
    let mut raw = serde_json::to_value(p.state()).unwrap();
    raw["accepted_plies"] = 1.into();
    let s: StructuralState = serde_json::from_value(raw).unwrap();
    assert!(Game::from_root(Position::install(s).unwrap()).is_err());
    let g = corner(profile);
    let saved = GameRecord::from_game(&g).unwrap();
    let context = g.context();
    let (mut restored, undo) = saved.to_game_with_undo().unwrap();
    assert!(!restored.context().eq(&context));
    assert!(undo.is_empty());
    assert!(restored.context().same_semantics(&context));
    restored.retire_context().unwrap();
}
#[test]
fn actual_ending_runtime_headroom_is_reserved_before_ordinary_admission() {
    let mut g = corner(Profile::AdamsModern);
    g.set_test_authority(u64::MAX - 3, u64::MAX - 3, u64::MAX - 3);
    let before = GameRecord::from_game(&g).unwrap();
    let context = g.context();
    assert_eq!(
        submit(&mut g, Side::Sente, GameCommand::OfferDraw).unwrap_err(),
        GameError::Capacity(Resource::Counter)
    );
    assert_eq!(before, GameRecord::from_game(&g).unwrap());
    assert_eq!(context, g.context());
    submit(
        &mut g,
        Side::Sente,
        GameCommand::Stop {
            reason: StopReason::ResourceLimit,
        },
    )
    .unwrap();
    submit(&mut g, Side::Gote, GameCommand::Resign).unwrap();
    submit(
        &mut g,
        Side::Sente,
        GameCommand::Stop {
            reason: StopReason::ResourceLimit,
        },
    )
    .unwrap();
    assert!(GameRecord::from_game(&g).is_ok());
}
#[test]
fn undo_cannot_spend_reserved_ending_authority_and_failure_keeps_token() {
    let mut g = corner(Profile::AdamsModern);
    let mut u = play(&mut g, (16, 16), (15, 16));
    g.set_test_authority(u64::MAX - 3, 5, 5);
    let before = GameRecord::from_game(&g).unwrap();
    let context = g.context();
    assert_eq!(
        g.undo(&mut u).unwrap_err(),
        GameError::Capacity(Resource::Counter)
    );
    assert_eq!(context, g.context());
    assert_eq!(before, GameRecord::from_game(&g).unwrap());
    g.set_test_authority(20, 5, 5);
    assert!(g.undo(&mut u).is_ok());
}
fn rotated_position(p: &Position) -> Position {
    let entries = p
        .pieces()
        .map(|(s, mut piece)| {
            piece.side = piece.side.opponent();
            (s.rotated(), piece)
        })
        .collect::<Vec<_>>();
    Position::from_diagram(
        p.profile(),
        &entries,
        p.side_to_move().opponent(),
        RootMetadata::diagram(0),
    )
    .unwrap()
}
fn rotated_action(a: &Action) -> Action {
    Action {
        kind: a.kind,
        from: a.from.rotated(),
        route: a.route.iter().map(|s| s.rotated()).collect(),
        promote: a.promote,
        target: a.target.map(Square::rotated),
    }
}
fn rotated_piece(mut p: Piece) -> Piece {
    p.side = p.side.opponent();
    p
}
fn rotated_effects(e: &Effects) -> Effects {
    let phases = e
        .phases
        .iter()
        .map(|phase| match phase {
            EffectPhase::Capture { victim } => EffectPhase::Capture {
                victim: Capture {
                    square: victim.square.rotated(),
                    piece: rotated_piece(victim.piece),
                },
            },
            EffectPhase::Burn { victims } => {
                let mut v = victims
                    .iter()
                    .map(|c| Capture {
                        square: c.square.rotated(),
                        piece: rotated_piece(c.piece),
                    })
                    .collect::<Vec<_>>();
                v.sort_by_key(|c| (c.square.rank(), c.square.file()));
                EffectPhase::Burn { victims: v }
            }
            EffectPhase::MoverDies { square, piece } => EffectPhase::MoverDies {
                square: square.rotated(),
                piece: rotated_piece(*piece),
            },
            EffectPhase::Promotion {
                square,
                before,
                after,
            } => EffectPhase::Promotion {
                square: square.rotated(),
                before: rotated_piece(*before),
                after: rotated_piece(*after),
            },
        })
        .collect();
    Effects {
        actor: e.actor.opponent(),
        action: rotated_action(&e.action),
        mover_before: rotated_piece(e.mover_before),
        mover_after: e.mover_after.map(rotated_piece),
        phases,
        work_units: 0,
    }
}
#[test]
fn owner_exchange_preserves_real_burn_phases_after_canonical_batch_sort() {
    for profile in Profile::ALL {
        let entries = [
            (Side::Sente, Base::K, false, 16, 16),
            (Side::Sente, Base::FiD, false, 8, 10),
            (Side::Sente, Base::P, false, 15, 14),
            (Side::Gote, Base::K, false, 1, 1),
            (Side::Gote, Base::P, false, 7, 7),
            (Side::Gote, Base::G, false, 9, 7),
            (Side::Gote, Base::P, false, 9, 9),
        ];
        let mut a = game(profile, &entries, Side::Sente);
        let mut b = Game::from_root(rotated_position(a.position())).unwrap();
        let action = Action::single(square(8, 10), square(8, 8));
        let rotated = rotated_action(&action);
        submit(&mut a, Side::Sente, GameCommand::Play { action }).unwrap();
        submit(&mut b, Side::Gote, GameCommand::Play { action: rotated }).unwrap();
        let actual = b.events()[0].effects.as_ref().unwrap();
        let expected = rotated_effects(a.events()[0].effects.as_ref().unwrap());
        let mut actual = actual.clone();
        actual.work_units = 0;
        assert_eq!(actual, expected);
        let expected_board = a
            .position()
            .pieces()
            .map(|(s, p)| (s.rotated(), rotated_piece(p)))
            .collect::<std::collections::BTreeMap<_, _>>();
        assert_eq!(
            b.position()
                .pieces()
                .collect::<std::collections::BTreeMap<_, _>>(),
            expected_board
        );
        assert_eq!(
            b.conclusion().map(Conclusion::winner),
            a.conclusion().map(|c| c.winner().map(Side::opponent))
        );
    }
}
#[test]
fn deterministic_admitted_sequences_preserve_capture_accounting_reversal_and_replay() {
    for profile in Profile::ALL {
        for seed in 1..=4u64 {
            let mut g = game(
                profile,
                &[
                    (Side::Sente, Base::K, false, 16, 16),
                    (Side::Sente, Base::Ln, false, 8, 8),
                    (Side::Sente, Base::P, false, 15, 14),
                    (Side::Gote, Base::K, false, 1, 1),
                    (Side::Gote, Base::FiD, false, 10, 6),
                    (Side::Gote, Base::P, false, 2, 3),
                ],
                Side::Sente,
            );
            let mut state = seed;
            let mut count = 0;
            for _ in 0..24 {
                let choices = g.legal_actions();
                if choices.is_empty() {
                    break;
                }
                state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
                let action = choices[state as usize % choices.len()].clone();
                let before = GameRecord::from_game(&g).unwrap();
                let snap = g.snapshot().clone();
                let pieces = g.position().pieces().count();
                let actor = g.position().side_to_move();
                let context = g.context();
                assert_eq!(
                    g.submit(GameRequest {
                        expected: context.clone(),
                        actor: actor.opponent(),
                        command: GameCommand::Play {
                            action: action.clone()
                        }
                    })
                    .unwrap_err(),
                    GameError::WrongActor
                );
                assert_eq!(context, g.context());
                let mut token = submit(&mut g, actor, GameCommand::Play { action }).unwrap();
                let effects = g.events().last().unwrap().effects.as_ref().unwrap();
                let removed =
                    effects.captures().count() + usize::from(effects.mover_after.is_none());
                assert_eq!(g.position().pieces().count() + removed, pieces);
                let ids = effects
                    .captures()
                    .map(|c| c.piece.id)
                    .collect::<std::collections::BTreeSet<_>>();
                assert_eq!(ids.len(), effects.captures().count());
                let after = GameRecord::from_game(&g).unwrap();
                let restored = after.to_game().unwrap();
                assert!(g.context().same_semantics(&restored.context()));
                assert_eq!(after, GameRecord::from_game(&restored).unwrap());
                let mut redo = g.undo(&mut token).unwrap();
                assert_eq!(before, GameRecord::from_game(&g).unwrap());
                assert_eq!(snap, *g.snapshot());
                g.redo(&mut redo).unwrap();
                assert_eq!(after, GameRecord::from_game(&g).unwrap());
                count += 1;
            }
            assert!(count > 0);
        }
    }
}
#[test]
fn native_counter_refusal_is_capacity_and_not_a_game_outcome() {
    let profile = Profile::AdamsModern;
    let p = corner(profile).position().clone();
    let entries = p.pieces().collect::<Vec<_>>();
    let p = Position::from_diagram(
        profile,
        &entries,
        Side::Sente,
        RootMetadata::diagram(u64::MAX),
    )
    .unwrap();
    let mut g = Game::from_root(p).unwrap();
    let before = GameRecord::from_game(&g).unwrap();
    assert_eq!(
        submit(
            &mut g,
            Side::Sente,
            GameCommand::Play {
                action: Action::single(square(16, 16), square(15, 16))
            }
        )
        .unwrap_err(),
        GameError::Capacity(Resource::Counter)
    );
    assert_eq!(before, GameRecord::from_game(&g).unwrap());
    assert_eq!(g.conclusion(), None);
    submit(&mut g, Side::Sente, GameCommand::Resign).unwrap();
}

#[test]
fn cancellable_prefix_reconstruction_preserves_live_state_and_fresh_authority() {
    for profile in Profile::ALL {
        let mut game = corner(profile);
        play(&mut game, (16, 16), (15, 16));
        let prefix = GameRecord::from_game(&game).unwrap();
        play(&mut game, (1, 1), (2, 1));
        let live = GameRecord::from_game(&game).unwrap();
        let context = game.context();
        let mut polls = 0;
        assert_eq!(
            game.record_prefix_with_stop(2, &mut || {
                polls += 1;
                polls >= 8
            }),
            Err(RecordError::Stopped)
        );
        assert_eq!(game.context(), context);
        assert_eq!(GameRecord::from_game(&game).unwrap(), live);
        let observed = game.record_prefix_with_stop(1, &mut || false).unwrap();
        assert_eq!(observed, prefix);
        let restored = observed.to_game().unwrap();
        assert_ne!(restored.context(), context);
        assert_eq!(restored.profile(), profile);
    }
}
