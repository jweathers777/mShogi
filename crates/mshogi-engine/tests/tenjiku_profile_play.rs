//! Independently expected profile differences through the real native line server.
//! These are functional engine results, not second rules, subprocess supervision,
//! competitive-strength or packaged-GUI evidence. Expectations follow the accepted
//! profile T03 (Free Eagle/Tetrarch), T04 (Demon ordering) and T05 (ranked victims).
use mshogi_core::{native::CanonicalRecord, tenjiku as t};
use mshogi_engine::*;
use std::{io::Cursor, time::Instant};

fn sq(file: u8, rank: u8) -> t::Square {
    t::Square::new(file, rank).unwrap()
}
fn diagram(profile: t::Profile, entries: &[(u8, u8, t::Side, t::Base, bool)]) -> t::Game {
    let pieces = entries
        .iter()
        .enumerate()
        .map(|(id, (file, rank, side, base, promoted))| {
            (
                sq(*file, *rank),
                t::Piece::new(id as u16, *side, *base, *promoted).unwrap(),
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
/// Execute the actual request/parser/server/result boundary, then submit exactly
/// the returned action to a separately restored authoritative native Game.
fn played(case: &str, game: &t::Game) -> (t::Game, t::Action, TenjikuTelemetry) {
    let before = t::GameRecord::from_game(game).unwrap();
    let context = game.context();
    let snapshot = game.snapshot().clone();
    let request = ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: 1,
            session_version: 2,
            process_generation: 3,
            game: CanonicalRecord::tenjiku(before.clone()),
            limits: TENJIKU_PLAY_LIMITS,
        },
    };
    assert_eq!(
        request.context.limits,
        WorkLimits {
            time_ms: None,
            depth: 1,
            nodes: 1024
        }
    );
    let begin = Instant::now();
    let mut output = vec![];
    serve_once(
        Cursor::new(format!("{}\n", serde_json::to_string(&request).unwrap())),
        &mut output,
    )
    .unwrap();
    let elapsed_us = begin.elapsed().as_micros();
    let response: ExecutionResponse = serde_json::from_slice(&output).unwrap();
    assert!(response.matches(&request));
    assert_eq!(response.observation.context, request.context);
    assert_eq!(
        response.observation.context.game.profile(),
        game.profile().into()
    );
    let ExecutionResult::Tenjiku { result } = response.observation.result else {
        panic!("{case}: actual native result required")
    };
    result.validate(game).unwrap();
    let TenjikuResult::Choice { action, telemetry } = result else {
        panic!("{case}: complete depth1 result required")
    };
    assert_eq!(telemetry.completed_depth, 1);
    assert_eq!(telemetry.status, StopReason::Complete);
    assert_eq!(telemetry.perspective, t::Side::Sente);
    assert!(telemetry.nodes > 1 && telemetry.nodes <= 1024);
    assert_eq!(telemetry.principal_variation, vec![action.clone()]);
    assert!(telemetry.score.is_some());
    let mut branch = before.to_game().unwrap();
    branch
        .submit(t::GameRequest {
            expected: branch.context(),
            actor: t::Side::Sente,
            command: t::GameCommand::Play {
                action: action.clone(),
            },
        })
        .unwrap();
    assert_eq!(branch.profile(), game.profile());
    assert_eq!(branch.events().len(), 1);
    assert_eq!(
        branch.events()[0].command,
        t::GameCommand::Play {
            action: action.clone()
        }
    );
    assert_eq!(branch.events()[0].effects.as_ref().unwrap().action, action);
    assert_eq!(t::GameRecord::from_game(game).unwrap(), before);
    assert_eq!(game.context(), context);
    assert_eq!(game.snapshot(), &snapshot);
    println!(
        "TENJIKU-PROFILE-PLAY case={case} profile={} elapsed_us={elapsed_us} nodes={} action={action:?} score={:?} effects={:?}",
        game.profile().id(),
        telemetry.nodes,
        telemetry.score,
        branch.events()[0].effects
    );
    (branch, action, telemetry)
}
fn win(branch: &t::Game, telemetry: &TenjikuTelemetry) {
    assert_eq!(
        branch.conclusion(),
        Some(t::Conclusion::RoyalCapture {
            winner: t::Side::Sente
        })
    );
    assert_eq!(telemetry.score, Some(mshogi_search::tenjiku::WIN_SCORE - 1));
    assert!(branch.royals(t::Side::Gote).is_empty());
}
fn heuristic(branch: &t::Game, telemetry: &TenjikuTelemetry) {
    assert_eq!(branch.conclusion(), None);
    assert!(telemetry.score.unwrap().abs() < mshogi_search::tenjiku::WIN_SCORE - 2);
}
#[test]
fn free_eagle_orthogonal_jump_and_diagonal_two_leg_win_remain_distinct() {
    use t::{Base::*, Side::*};
    // OwnKnight8g blocks the Queen ray. Adams alone jumps directly8h->8f.
    // Muller reaches8f through two actual diagonal landings around the blocker.
    for profile in t::Profile::ALL {
        let game = diagram(
            profile,
            &[
                (8, 8, Sente, FEg, false),
                (8, 7, Sente, N, false),
                (8, 6, Gote, K, false),
                (1, 2, Gote, P, false),
                (16, 16, Sente, K, false),
            ],
        );
        let (branch, action, telemetry) = played("free_eagle", &game);
        win(&branch, &telemetry);
        assert_eq!(action.from, sq(8, 8));
        assert_eq!(action.route.last(), Some(&sq(8, 6)));
        assert_eq!(
            branch.position().piece(sq(8, 7)),
            game.position().piece(sq(8, 7))
        );
        match profile {
            t::Profile::AdamsModern => assert_eq!(action.route, vec![sq(8, 6)]),
            t::Profile::MullerCompleted => {
                assert_eq!(action.route.len(), 2);
                assert!([sq(7, 7), sq(9, 7)].contains(&action.route[0]));
            }
        }
        assert!(
            matches!(branch.events()[0].effects.as_ref().unwrap().phases.as_slice(),
            [t::EffectPhase::Capture { victim }] if victim.square==sq(8,6) && victim.piece.base==K)
        );
    }
}
#[test]
fn tetrarch_vertical_skipped_ray_wins_only_under_muller() {
    use t::{Base::*, Side::*};
    // Promoted Chariot Soldier is HT. Muller skips own adjacent8g and captures8e;
    // Adams has neither a vertical ray nor a royal within its adjacent igui reach.
    for profile in t::Profile::ALL {
        let game = diagram(
            profile,
            &[
                (8, 8, Sente, ChS, true),
                (8, 7, Sente, P, false),
                (8, 5, Gote, K, false),
                (1, 2, Gote, P, false),
                (16, 16, Sente, K, false),
            ],
        );
        let (branch, action, telemetry) = played("tetrarch", &game);
        match profile {
            t::Profile::MullerCompleted => {
                win(&branch, &telemetry);
                assert_eq!(action, t::Action::single(sq(8, 8), sq(8, 5)));
                assert_eq!(
                    branch.position().piece(sq(8, 7)),
                    game.position().piece(sq(8, 7))
                );
                assert!(
                    matches!(branch.events()[0].effects.as_ref().unwrap().phases.as_slice(),
                    [t::EffectPhase::Capture { victim }] if victim.square==sq(8,5) && victim.piece.base==K)
                );
            }
            t::Profile::AdamsModern => {
                heuristic(&branch, &telemetry);
                assert_ne!(action.to(), Some(sq(8, 5)));
                assert_eq!(
                    branch.position().piece(sq(8, 5)),
                    game.position().piece(sq(8, 5))
                );
            }
        }
    }
}
#[test]
fn jumped_higher_general_victim_is_selected_only_under_muller() {
    use t::{Base::*, Side::*};
    // OwnPawn7g is a lower-rank intervener; GGn6f outranks our BGn. Only Muller
    // permits that jumped nonroyal victim. Capturing it is the greatest material gain.
    for profile in t::Profile::ALL {
        let game = diagram(
            profile,
            &[
                (8, 8, Sente, BGn, false),
                (7, 7, Sente, P, false),
                (6, 6, Gote, GGn, false),
                (1, 1, Gote, K, false),
                (16, 2, Gote, P, false),
                (16, 16, Sente, K, false),
            ],
        );
        let (branch, action, telemetry) = played("ranked_victim", &game);
        heuristic(&branch, &telemetry);
        assert_eq!(
            branch.position().piece(sq(7, 7)),
            game.position().piece(sq(7, 7))
        );
        match profile {
            t::Profile::MullerCompleted => {
                assert_eq!(action, t::Action::single(sq(8, 8), sq(6, 6)));
                assert!(
                    matches!(branch.events()[0].effects.as_ref().unwrap().phases.as_slice(),
                    [t::EffectPhase::Capture { victim }] if victim.square==sq(6,6) && victim.piece.base==GGn)
                );
                assert_eq!(branch.position().piece(sq(6, 6)).unwrap().side, Sente);
            }
            t::Profile::AdamsModern => {
                assert_ne!(action.to(), Some(sq(6, 6)));
                assert_eq!(
                    branch.position().piece(sq(6, 6)),
                    game.position().piece(sq(6, 6))
                );
            }
        }
    }
}
#[test]
fn dying_demon_burns_last_royal_before_death_only_under_adams() {
    use t::{Base::*, Side::*};
    // King8g is four steps from FiD8k; ownPawn8i blocks the vertical ray, and
    // ownPawn7h occupies the only reachable safe square adjacent to that King.
    // Every remaining reachable burning square is hazardous from9g/9h Demons.
    for profile in t::Profile::ALL {
        let game = diagram(
            profile,
            &[
                (8, 11, Sente, FiD, false),
                (8, 9, Sente, P, false),
                (7, 8, Sente, P, false),
                (16, 16, Sente, K, false),
                (8, 7, Gote, K, false),
                (9, 7, Gote, FiD, false),
                (9, 8, Gote, FiD, false),
            ],
        );
        let (branch, action, telemetry) = played("dying_demon", &game);
        match profile {
            t::Profile::AdamsModern => {
                win(&branch, &telemetry);
                assert_eq!(action.from, sq(8, 11));
                assert!([sq(8, 8), sq(9, 8)].contains(&action.to().unwrap()));
                let effects = branch.events()[0].effects.as_ref().unwrap();
                assert!(effects.mover_after.is_none());
                let burn=effects.phases.iter().position(|phase| matches!(phase,
                    t::EffectPhase::Burn { victims } if victims.iter().any(|v|v.square==sq(8,7)&&v.piece.base==K))).unwrap();
                let death = effects
                    .phases
                    .iter()
                    .position(|phase| matches!(phase, t::EffectPhase::MoverDies { .. }))
                    .unwrap();
                assert!(burn < death);
            }
            t::Profile::MullerCompleted => {
                heuristic(&branch, &telemetry);
                assert_eq!(
                    branch.position().piece(sq(8, 7)),
                    game.position().piece(sq(8, 7))
                );
                assert!(branch.events()[0].effects.as_ref().unwrap().phases.iter().all(|phase|
                    !matches!(phase,t::EffectPhase::Burn{victims} if victims.iter().any(|v|v.square==sq(8,7)))));
            }
        }
    }
}
