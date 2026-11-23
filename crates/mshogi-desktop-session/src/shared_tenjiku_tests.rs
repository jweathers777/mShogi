//! MSH-79 shared consumers. These are native controller/file tests, not GUI or search evidence.
//! Seven shared consumer obligations and five profile-boundary obligations are covered here;
//! the five full product obligations remain with MSH-44.
use super::*;
use crate::tenjiku_document::TenjikuSettings;
use mshogi_core::tenjiku as c;
use serde_json::{Value, json};
use std::{
    fs,
    io::Cursor,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

fn controller() -> DesktopController {
    let mut ctl = DesktopController::new("/unused/msh79-tenjiku-engine".into()).unwrap();
    ctl.shared.manual_engine = true;
    ctl
}
fn square(file: u8, rank: u8) -> c::Square {
    c::Square::new(file, rank).unwrap()
}
fn root(profile: c::Profile, extra: &[(u8, u8, c::Side, c::Base)]) -> c::Game {
    root_metadata(
        profile,
        extra,
        c::RootMetadata::imported("shared-tenjiku-integration".into(), 120).unwrap(),
    )
}
fn root_metadata(
    profile: c::Profile,
    extra: &[(u8, u8, c::Side, c::Base)],
    metadata: c::RootMetadata,
) -> c::Game {
    let entries = [
        (16, 16, c::Side::Sente, c::Base::K),
        (15, 14, c::Side::Sente, c::Base::P),
        (1, 1, c::Side::Gote, c::Base::K),
        (2, 3, c::Side::Gote, c::Base::P),
    ]
    .into_iter()
    .chain(extra.iter().copied())
    .enumerate()
    .map(|(id, (file, rank, side, base))| {
        (
            square(file, rank),
            c::Piece::new(id as u16, side, base, false).unwrap(),
        )
    })
    .collect::<Vec<_>>();
    c::Game::from_root(
        c::Position::from_diagram(profile, &entries, c::Side::Sente, metadata).unwrap(),
    )
    .unwrap()
}
fn fire_root(profile: c::Profile) -> c::Game {
    // Frozen native consumer root: Adams T04 burns before dying; Muller T04 dies first.
    root(
        profile,
        &[
            (8, 10, c::Side::Sente, c::Base::FiD),
            (9, 8, c::Side::Gote, c::Base::FiD),
            (8, 7, c::Side::Gote, c::Base::P),
        ],
    )
}
fn record(ctl: &DesktopController) -> c::GameRecord {
    c::GameRecord::from_game(&ctl.session.tenjiku().unwrap().game).unwrap()
}
fn document_bytes(ctl: &DesktopController) -> Vec<u8> {
    ctl.session.tenjiku().unwrap().document.bytes().to_vec()
}
fn settle(
    ctl: &mut DesktopController,
    receipt: AdmissionReceipt,
) -> Result<SharedSnapshot, String> {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = ctl.shared_snapshot();
        if let Some(result) = receipt.completion() {
            return result.map(|_| snapshot);
        }
        assert!(
            Instant::now() < deadline,
            "Tenjiku admission did not complete"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn send(
    ctl: &mut DesktopController,
    operation: SessionOperation,
) -> Result<SharedSnapshot, String> {
    let game = ctl.shared_snapshot().game;
    let (_, receipt) = ctl.shared_command_with_receipt(SharedDesktopCommand {
        expected: ctl.shared_token(),
        game,
        operation,
    })?;
    settle(ctl, receipt)
}
fn replace(ctl: &mut DesktopController, game: &c::Game, human: c::Side) -> SharedSnapshot {
    send(
        ctl,
        SessionOperation::Replace {
            record: CanonicalRecord::tenjiku(c::GameRecord::from_game(game).unwrap()),
            human: NativeSide::Tenjiku(human),
        },
    )
    .unwrap()
}
fn command(
    ctl: &mut DesktopController,
    actor: c::Side,
    command: c::GameCommand,
) -> Result<SharedSnapshot, String> {
    send(ctl, SessionOperation::Tenjiku { actor, command })
}
fn play(ctl: &mut DesktopController, action: c::Action) -> Result<SharedSnapshot, String> {
    let actor = ctl
        .session
        .tenjiku()
        .unwrap()
        .game
        .position()
        .side_to_move();
    command(ctl, actor, c::GameCommand::Play { action })
}
fn wait_file(ctl: &mut DesktopController, id: u64) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = ctl.shared_snapshot();
        if snapshot.io.id == id && matches!(snapshot.io.phase, IoPhase::Complete | IoPhase::Failed)
        {
            return snapshot;
        }
        assert!(
            Instant::now() < deadline,
            "Tenjiku file operation did not complete"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn save(ctl: &mut DesktopController, path: &Path) -> SharedSnapshot {
    let id = ctl.queue_shared_save(path.to_owned()).unwrap();
    let snapshot = wait_file(ctl, id);
    assert_eq!(
        snapshot.io.phase,
        IoPhase::Complete,
        "{:?}",
        snapshot.io.message
    );
    snapshot
}
struct TestDirectory(PathBuf);
impl TestDirectory {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "msh79-tenjiku-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}
impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn strict_profile_dispatch_fresh_new_other_profile_and_stale_contexts_in_both_directions() {
    // TEN-CONSUMER-STRICT_PROFILE_DISPATCH / FRESH-NEW-OTHER-PROFILE / NEW_GAME_IMMUTABLE.
    let mut ctl = controller();
    let mut previous_native_context = None;
    let mut previous_session_token = None;
    for profile in [
        c::Profile::AdamsModern,
        c::Profile::MullerCompleted,
        c::Profile::AdamsModern,
    ] {
        let before = ctl.shared_token();
        let expected = ctl.shared_open_token();
        let (_, receipt) = ctl
            .shared_new_game(
                expected,
                RulesProfile::from(profile),
                NativeSide::Tenjiku(c::Side::Gote),
            )
            .unwrap();
        let snapshot = settle(&mut ctl, receipt).unwrap();
        assert_eq!(snapshot.game, GameKind::Tenjiku);
        assert_eq!(snapshot.profile, RulesProfile::from(profile));
        assert_eq!(snapshot.edition, profile.edition());
        assert_eq!(snapshot.human.id, NativeSide::Tenjiku(c::Side::Gote));
        assert_eq!(snapshot.human.key, format!("{}:gote", profile.id()));
        assert_eq!(snapshot.turn.key, format!("{}:sente", profile.id()));
        assert!(snapshot.token.document > before.document);
        assert_eq!(snapshot.board.files.len(), 16);
        assert_eq!(snapshot.board.ranks.len(), 16);
        assert_eq!(snapshot.pieces.len(), 156);
        assert!(snapshot.hands.is_empty());
        assert!(
            snapshot.capabilities.engine_turn
                && snapshot.capabilities.move_analysis
                && !snapshot.capabilities.chess_full
        );
        assert!(snapshot.tenjiku.as_ref().unwrap().execution_supported);
        assert_eq!(record(&ctl).profile(), profile);
        assert_eq!(record(&ctl).event_count(), 0);
        let accepted = record(&ctl);
        assert!(
            ctl.shared_new_game(
                expected,
                RulesProfile::from(profile),
                NativeSide::Tenjiku(c::Side::Sente)
            )
            .is_err()
        );
        assert!(
            ctl.shared_new_game(
                ctl.shared_open_token(),
                RulesProfile::from(profile),
                NativeSide::Chu(mshogi_core::chu::Side::Sente)
            )
            .is_err()
        );
        assert_eq!(record(&ctl), accepted);
        if let Some(stale) = previous_native_context.take() {
            let game = &mut ctl.session.tenjiku_mut().unwrap().game;
            assert_eq!(
                game.submit(c::GameRequest {
                    expected: stale,
                    actor: c::Side::Sente,
                    command: c::GameCommand::Resign
                })
                .unwrap_err(),
                c::GameError::StaleContext
            );
            assert_eq!(record(&ctl), accepted);
        }
        if let Some(stale) = previous_session_token.take() {
            assert!(
                ctl.shared_command(SharedDesktopCommand {
                    expected: stale,
                    game: GameKind::Tenjiku,
                    operation: SessionOperation::Resign {}
                })
                .is_err()
            );
            assert_eq!(record(&ctl), accepted);
        }
        let mut forged = serde_json::to_value(SharedDesktopCommand {
            expected: ctl.shared_token(),
            game: GameKind::Tenjiku,
            operation: SessionOperation::TenjikuPreferences {
                settings: TenjikuSettings {
                    play_mode: PlayMode::HumanEngine,
                    orientation: c::Side::Sente,
                },
            },
        })
        .unwrap();
        forged["operation"]["rules_profile"] = json!(RulesProfile::from(match profile {
            c::Profile::AdamsModern => c::Profile::MullerCompleted,
            _ => c::Profile::AdamsModern,
        }));
        assert!(SharedDesktopCommand::from_json(&forged.to_string()).is_err());
        assert_eq!(record(&ctl), accepted);
        previous_native_context = Some(ctl.session.tenjiku().unwrap().game.context());
        previous_session_token = Some(ctl.shared_token());
    }
}

#[test]
fn native_burn_projection_preserves_profile_specific_order_and_dying_mover() {
    // TEN-CONSUMER-NATIVE_BURN_PROJECTION: source-derived effects, never travel arrows.
    let mut ctl = controller();
    for profile in c::Profile::ALL {
        replace(&mut ctl, &fire_root(profile), c::Side::Sente);
        let action = c::Action::single(square(8, 10), square(8, 8));
        let snapshot = play(&mut ctl, action.clone()).unwrap();
        let geometry = snapshot.moves.last_action.as_ref().unwrap();
        assert_eq!(geometry.action, serde_json::to_string(&action).unwrap());
        assert_eq!(geometry.steps, vec!["8h"]);
        assert!(geometry.effects.is_empty());
        let entry = &snapshot.moves.entries[0];
        assert_eq!(entry.notation, "FiD8j-8h");
        assert_eq!(
            entry.effects_description.as_deref(),
            Some(if profile == c::Profile::AdamsModern {
                "Burned together: Gote Pawn at 8g; Sente Fire Demon destroyed at 8h"
            } else {
                "Sente Fire Demon destroyed at 8h"
            })
        );
        let Some(crate::moves::NativeCompletedEffects::Tenjiku {
            stationary_target,
            phases,
        }) = &geometry.native_effects
        else {
            panic!("missing native phases")
        };
        assert!(stationary_target.is_none());
        if profile == c::Profile::AdamsModern {
            assert_eq!(phases.len(), 2);
            assert!(
                matches!(&phases[0], crate::moves::TenjikuEffectPhase::Burn { victims } if victims.len() == 1 && victims[0].square == "8g" && victims[0].piece.base == c::Base::P)
            );
            assert!(
                matches!(&phases[1], crate::moves::TenjikuEffectPhase::MoverDies { mover } if mover.square == "8h" && mover.piece.base == c::Base::FiD)
            );
            assert!(!snapshot.pieces.iter().any(|p| p.square == "8g"));
        } else {
            assert_eq!(phases.len(), 1);
            assert!(
                matches!(&phases[0], crate::moves::TenjikuEffectPhase::MoverDies { mover } if mover.square == "8h")
            );
            assert!(snapshot.pieces.iter().any(|p| p.square == "8g"));
        }
        assert!(!snapshot.pieces.iter().any(|p| p.square == "8h"));
        assert!(snapshot.pieces.iter().any(|p| p.square == "9h"));
        assert_eq!(
            snapshot
                .tenjiku
                .as_ref()
                .unwrap()
                .selected
                .last_effects
                .as_ref()
                .unwrap(),
            ctl.session.tenjiku().unwrap().game.events()[0]
                .effects
                .as_ref()
                .unwrap()
        );
        assert!(!snapshot.status.concluded);
        assert_eq!(snapshot.status.score_half_points, None);
    }
}

#[test]
fn stationary_capture_target_is_selectable_while_native_movement_route_stays_empty() {
    let mut ctl = controller();
    for profile in c::Profile::ALL {
        // TEN-TETRARCH-IGUI: an original Chariot Soldier promoted once to HT.
        let unpromoted = root(
            profile,
            &[
                (8, 8, c::Side::Sente, c::Base::ChS),
                (9, 8, c::Side::Gote, c::Base::P),
            ],
        );
        let entries = unpromoted
            .position()
            .pieces()
            .map(|(at, piece)| {
                (
                    at,
                    if piece.base == c::Base::ChS {
                        c::Piece::new(piece.id, piece.side, piece.base, true).unwrap()
                    } else {
                        piece
                    },
                )
            })
            .collect::<Vec<_>>();
        let game = c::Game::from_root(
            c::Position::from_diagram(
                profile,
                &entries,
                c::Side::Sente,
                unpromoted.position().root().clone(),
            )
            .unwrap(),
        )
        .unwrap();
        replace(&mut ctl, &game, c::Side::Sente);
        let action = c::Action::igui(square(8, 8), square(9, 8));
        let before = ctl.shared_snapshot();
        let choice = before
            .choices
            .iter()
            .find(|v| v.action == serde_json::to_string(&action).unwrap())
            .unwrap();
        assert_eq!(choice.from, "8h");
        assert_eq!(choice.steps, vec!["9h"]);
        assert!(
            matches!(&choice.tenjiku, Some(tenjiku::TenjikuChoiceShape::StationaryCapture { target }) if target == "9h")
        );
        let decoded: c::Action = serde_json::from_str(&choice.action).unwrap();
        assert!(decoded.route.is_empty());
        assert_eq!(decoded.target, Some(square(9, 8)));
        let snapshot = play(&mut ctl, decoded).unwrap();
        assert!(
            snapshot
                .pieces
                .iter()
                .any(|p| p.square == "8h" && p.kind == "ChS" && p.promoted)
        );
        assert!(!snapshot.pieces.iter().any(|p| p.square == "9h"));
        assert_eq!(snapshot.moves.entries[0].notation, "ChS+8h!x9h");
        assert_eq!(
            snapshot.moves.entries[0].effects_description.as_deref(),
            Some("Stationary capture at 9h; stationary origin 8h; Captured Gote Pawn at 9h")
        );
        let geometry = snapshot.moves.last_action.unwrap();
        assert!(geometry.steps.is_empty());
        assert!(
            matches!(geometry.native_effects, Some(crate::moves::NativeCompletedEffects::Tenjiku { stationary_target: Some(target), phases }) if target == "9h" && matches!(&phases[0], crate::moves::TenjikuEffectPhase::Capture { victim } if victim.square == "9h"))
        );
    }
}

#[test]
fn complete_route_choices_roundtrip_without_collapsing_landings_and_partial_wire_refuses() {
    // TEN-CONSUMER-COMPLETE_ROUTE_PARTIAL_SELECTION; frontend interaction has separate tests.
    let mut ctl = controller();
    for profile in c::Profile::ALL {
        replace(
            &mut ctl,
            &root(profile, &[(8, 8, c::Side::Sente, c::Base::Ln)]),
            c::Side::Sente,
        );
        let action = c::Action {
            kind: c::ActionKind::Travel,
            from: square(8, 8),
            route: vec![square(8, 7), square(9, 7)],
            promote: false,
            target: None,
        };
        let snapshot = ctl.shared_snapshot();
        let choice = snapshot
            .choices
            .iter()
            .find(|v| v.action == serde_json::to_string(&action).unwrap())
            .unwrap();
        assert_eq!(choice.steps, vec!["8g", "9g"]);
        assert!(
            matches!(&choice.tenjiku, Some(tenjiku::TenjikuChoiceShape::Travel { route }) if route == &vec!["8g", "9g"])
        );
        let accepted = record(&ctl);
        let mut partial = serde_json::to_value(SharedDesktopCommand {
            expected: ctl.shared_token(),
            game: GameKind::Tenjiku,
            operation: SessionOperation::Tenjiku {
                actor: c::Side::Sente,
                command: c::GameCommand::Play {
                    action: action.clone(),
                },
            },
        })
        .unwrap();
        partial["operation"]["command"]["action"]
            .as_object_mut()
            .unwrap()
            .remove("route");
        assert!(SharedDesktopCommand::from_json(&partial.to_string()).is_err());
        assert_eq!(record(&ctl), accepted);
        let snapshot = play(&mut ctl, serde_json::from_str(&choice.action).unwrap()).unwrap();
        assert_eq!(snapshot.moves.last_action.unwrap().steps, vec!["8g", "9g"]);
        assert_eq!(snapshot.board_ply, 1);
        assert_eq!(snapshot.event_index, 1);
    }
}

#[test]
fn promotion_choices_and_per_original_first_refusal_rights_survive_shared_admission() {
    let mut ctl = controller();
    for profile in c::Profile::ALL {
        replace(
            &mut ctl,
            &root(profile, &[(8, 6, c::Side::Sente, c::Base::G)]),
            c::Side::Sente,
        );
        let snapshot = ctl.shared_snapshot();
        assert_eq!(
            snapshot
                .choices
                .iter()
                .filter(|v| v.from == "8f" && v.steps == vec!["8e"])
                .count(),
            2
        );
        let snapshot = play(&mut ctl, c::Action::single(square(8, 6), square(8, 5))).unwrap();
        assert!(
            snapshot
                .tenjiku
                .unwrap()
                .selected
                .identities
                .iter()
                .any(|i| i.square == "8e"
                    && i.piece.right == c::PromotionRight::FirstDeclinedNextMove)
        );
        play(&mut ctl, c::Action::single(square(1, 1), square(2, 1))).unwrap();
        let accepted = record(&ctl);
        let snapshot = ctl.shared_snapshot();
        let choices = snapshot
            .choices
            .iter()
            .filter(|v| v.from == "8e" && v.steps == vec!["8d"])
            .collect::<Vec<_>>();
        assert_eq!(choices.len(), 1);
        assert!(
            !serde_json::from_str::<c::Action>(&choices[0].action)
                .unwrap()
                .promote
        );
        let mut refused = c::Action::single(square(8, 5), square(8, 4));
        refused.promote = true;
        assert!(play(&mut ctl, refused).is_err());
        assert_eq!(record(&ctl), accepted);
        play(&mut ctl, c::Action::single(square(8, 5), square(8, 4))).unwrap();
        play(&mut ctl, c::Action::single(square(2, 1), square(3, 1))).unwrap();
        let mut promote = c::Action::single(square(8, 4), square(8, 3));
        promote.promote = true;
        let snapshot = play(&mut ctl, promote).unwrap();
        assert!(
            snapshot
                .tenjiku
                .unwrap()
                .selected
                .identities
                .iter()
                .any(|i| i.square == "8c"
                    && i.piece.base == c::Base::G
                    && i.piece.promoted
                    && i.piece.right == c::PromotionRight::Consumed
                    && i.face == c::Face::R)
        );
        let Some(crate::moves::NativeCompletedEffects::Tenjiku { phases, .. }) =
            snapshot.moves.last_action.unwrap().native_effects
        else {
            panic!()
        };
        assert!(
            matches!(phases.last(), Some(crate::moves::TenjikuEffectPhase::Promotion { before, after }) if !before.piece.promoted && after.piece.promoted && before.piece.id == after.piece.id)
        );
    }
}

#[test]
fn selected_history_inspection_takeback_retained_branch_and_return_live_use_exact_prefixes() {
    // TEN-CONSUMER-SELECTED_HISTORY_INSPECTION / PROFILE-MIX-HISTORICAL_SELECTION.
    let mut ctl = controller();
    for profile in c::Profile::ALL {
        replace(&mut ctl, &fire_root(profile), c::Side::Sente);
        let root_record = record(&ctl);
        let origin_token = ctl.shared_token();
        let before = document_bytes(&ctl);
        let inspection = ctl
            .shared_inspection(MovementInspectionQuery {
                expected: origin_token,
                origin: "9h".into(),
            })
            .unwrap();
        assert!(inspection.read_only);
        assert_eq!(inspection.side.id, NativeSide::Tenjiku(c::Side::Gote));
        assert_eq!(inspection.side.key, format!("{}:gote", profile.id()));
        assert!(!inspection.choices.is_empty());
        assert_eq!(document_bytes(&ctl), before);
        play(&mut ctl, c::Action::single(square(8, 10), square(8, 8))).unwrap();
        let live = record(&ctl);
        assert!(
            ctl.shared_inspection(MovementInspectionQuery {
                expected: origin_token,
                origin: "9h".into()
            })
            .is_err()
        );
        let snapshot = send(&mut ctl, SessionOperation::Inspect { event_index: 0 }).unwrap();
        assert!(snapshot.historical);
        assert_eq!(
            snapshot.context,
            CanonicalRecord::tenjiku(root_record.clone())
        );
        assert_eq!(snapshot.profile, RulesProfile::from(profile));
        assert_eq!(snapshot.tenjiku.as_ref().unwrap().selected.event_index, 0);
        assert_eq!(snapshot.tenjiku.as_ref().unwrap().live.event_index, 1);
        assert!(snapshot.moves.last_action.is_none());
        let selected_token = snapshot.token;
        let inspection = ctl
            .shared_inspection(MovementInspectionQuery {
                expected: selected_token,
                origin: "8j".into(),
            })
            .unwrap();
        assert!(inspection.read_only);
        assert!(inspection.choices.iter().any(|v| v.steps == vec!["8h"]));
        assert_eq!(record(&ctl), live);
        assert!(send(&mut ctl, SessionOperation::Resign {}).is_err());
        let snapshot = send(&mut ctl, SessionOperation::ReturnLive {}).unwrap();
        assert!(!snapshot.historical);
        assert_eq!(snapshot.event_index, 1);
        assert!(
            ctl.shared_inspection(MovementInspectionQuery {
                expected: selected_token,
                origin: "9h".into()
            })
            .is_err()
        );
        let snapshot = send(&mut ctl, SessionOperation::TakeBack {}).unwrap();
        assert_eq!(snapshot.event_index, 0);
        assert_eq!(record(&ctl), root_record);
    }
}

fn execute(context: &ExecutionContext) -> ExecutionResponse {
    let mut bytes = vec![];
    mshogi_engine::serve_once(
        Cursor::new(format!(
            "{}\n",
            serde_json::to_string(&execution_request(context.clone())).unwrap()
        )),
        &mut bytes,
    )
    .unwrap();
    ExecutionResponse::decode(&bytes, &execution_request(context.clone())).unwrap()
}

#[test]
fn real_search_observations_reject_profile_purpose_and_all_runtime_context_changes() {
    // SAME_BOARD_CROSS_PROFILE_REJECTION / PROFILE-MIX-OBSERVATION_CONTEXT / EXECUTION_RESULT.
    let directory = TestDirectory::new();
    let mut ctl = controller();
    for profile in c::Profile::ALL {
        replace(&mut ctl, &root(profile, &[]), c::Side::Sente);
        play(&mut ctl, c::Action::single(square(16, 16), square(15, 16))).unwrap();
        let live = record(&ctl);
        send(&mut ctl, SessionOperation::Inspect { event_index: 0 }).unwrap();
        ctl.start_game_work::<Tenjiku>(
            WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 512,
            },
            true,
        )
        .unwrap();
        let pending = ctl.session.tenjiku().unwrap().pending.clone().unwrap();
        let accepted = document_bytes(&ctl);
        let dirty = ctl.shared_snapshot().document.dirty;
        for field in 0..9 {
            let mut other = pending.clone();
            match field {
                0 => {
                    other.game = CanonicalRecord::tenjiku(
                        c::GameRecord::from_game(&root(
                            match profile {
                                c::Profile::AdamsModern => c::Profile::MullerCompleted,
                                _ => c::Profile::AdamsModern,
                            },
                            &[],
                        ))
                        .unwrap(),
                    )
                }
                1 => other.purpose = ExecutionPurpose::EngineTurn {},
                2 => other.request_id += 1,
                3 => {
                    other.session_version += 1;
                    if let ExecutionPurpose::MoveAnalysis { target } = &mut other.purpose {
                        target.session_version += 1;
                    }
                }
                4 => other.process_generation += 1,
                5 => other.limits.nodes += 1,
                6 => {
                    if let ExecutionPurpose::MoveAnalysis { target } = &mut other.purpose {
                        target.selection_id += 1;
                    }
                }
                7 => {
                    if let ExecutionPurpose::MoveAnalysis { target } = &mut other.purpose {
                        target.label.push_str(" changed");
                    }
                }
                _ => {
                    other.game = CanonicalRecord::tenjiku(live.clone());
                    if let ExecutionPurpose::MoveAnalysis { target } = &mut other.purpose {
                        target.event_index = 1;
                        target.board_ply = 1;
                        target.turn = NativeSide::Tenjiku(c::Side::Gote);
                    }
                }
            }
            assert!(other.valid_limits());
            let wrong = execute(&other);
            assert!(matches!(
                wrong.observation.result,
                ExecutionResult::Tenjiku { .. }
            ));
            assert!(
                !ctl.accept_shared(wrong),
                "accepted changed context field {field}"
            );
            assert_eq!(
                ctl.session.tenjiku().unwrap().pending.as_ref(),
                Some(&pending)
            );
            assert_eq!(document_bytes(&ctl), accepted);
            assert_eq!(record(&ctl), live);
            assert_eq!(ctl.shared_snapshot().document.dirty, dirty);
        }
        let response = execute(&pending);
        assert!(matches!(
            response.observation.result,
            ExecutionResult::Tenjiku {
                result: mshogi_engine::TenjikuResult::Choice { .. }
            }
        ));
        assert!(ctl.accept_shared(response));
        let receipt = ctl.admission_receipt.clone().unwrap();
        let snapshot = settle(&mut ctl, receipt).unwrap();
        assert!(snapshot.historical && snapshot.paused);
        assert_eq!(snapshot.phase, EnginePhase::Paused);
        assert_eq!(record(&ctl), live);
        assert_eq!(snapshot.observations.len(), 1);
        assert!(
            snapshot
                .search_observations
                .iter()
                .all(|o| o.score.is_some()
                    && o.completed_depth == Some(1)
                    && !o.variation.is_empty())
        );
        assert!(ctl.session.tenjiku().unwrap().pending.is_none());
        assert!(snapshot.status.score_half_points.is_none());
        assert!(!snapshot.status.concluded);
        assert_eq!(snapshot.observations[0].observation.context, pending);
        let path = directory.path("real-analysis-observation.mshogi");
        save(&mut ctl, &path);
        let id = ctl
            .queue_shared_open(path, ctl.shared_open_token())
            .unwrap();
        let reopened = wait_file(&mut ctl, id);
        assert_eq!(reopened.io.phase, IoPhase::Complete);
        assert!(reopened.paused && !reopened.historical);
        assert!(ctl.session.tenjiku().unwrap().pending.is_none());
        assert_eq!(record(&ctl), live);
        send(&mut ctl, SessionOperation::ReturnLive {}).unwrap();
        ctl.start_game_work::<Tenjiku>(
            WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 512,
            },
            false,
        )
        .unwrap();
        let engine_turn = ctl.session.tenjiku().unwrap().pending.clone().unwrap();
        let response = execute(&engine_turn);
        assert!(matches!(
            response.observation.result,
            ExecutionResult::Tenjiku {
                result: mshogi_engine::TenjikuResult::Choice { .. }
            }
        ));
        assert!(ctl.accept_shared(response));
        let receipt = ctl.admission_receipt.clone().unwrap();
        let snapshot = settle(&mut ctl, receipt).unwrap();
        assert_eq!(record(&ctl).event_count(), live.event_count() + 1);
        assert!(!snapshot.paused && !snapshot.historical);
        // Results are not saved: only this session's engine turn is listed.
        assert_eq!(snapshot.observations.len(), 1);
        assert_eq!(snapshot.observations[0].observation.context, engine_turn);
        assert!(snapshot.status.score_half_points.is_none() && !snapshot.status.concluded);
    }
}

#[test]
fn raw_mixed_profile_forged_effects_and_duplicate_commands_never_reach_the_installed_owner() {
    let mut ctl = controller();
    for profile in c::Profile::ALL {
        replace(&mut ctl, &fire_root(profile), c::Side::Sente);
        play(&mut ctl, c::Action::single(square(8, 10), square(8, 8))).unwrap();
        let native = record(&ctl);
        let before = document_bytes(&ctl);
        let expected = ctl.shared_token();
        let request = SharedDesktopCommand {
            expected,
            game: GameKind::Tenjiku,
            operation: SessionOperation::Replace {
                record: CanonicalRecord::tenjiku(native.clone()),
                human: NativeSide::Tenjiku(c::Side::Sente),
            },
        };
        let raw = serde_json::to_string(&request).unwrap();
        let mut mixed: Value = serde_json::from_str(&raw).unwrap();
        let other = match profile {
            c::Profile::AdamsModern => c::Profile::MullerCompleted,
            _ => c::Profile::AdamsModern,
        };
        mixed["operation"]["record"]["rules_profile"] = json!(other.id());
        mixed["operation"]["record"]["profile_edition"] = json!(other.edition());
        assert!(SharedDesktopCommand::from_json(&mixed.to_string()).is_err());
        let duplicate = raw.replacen(
            "\"promote\":false",
            "\"promote\":false,\"promote\":false",
            1,
        );
        assert_ne!(duplicate, raw);
        assert!(SharedDesktopCommand::from_json(&duplicate).is_err());
        let mut forged: Value = serde_json::from_str(&raw).unwrap();
        // Effects are derived, so even a supplied copy is forbidden. Keep the
        // hostile native-effect probe without relying on an encoded cache.
        forged["operation"]["record"]["payload"]["events"][0]["effects"] =
            serde_json::to_value(&native.to_game().unwrap().events()[0].effects).unwrap();
        let phase =
            &mut forged["operation"]["record"]["payload"]["events"][0]["effects"]["phases"][0];
        if profile == c::Profile::AdamsModern {
            phase["victims"][0]["square"] = json!([8, 6]);
        } else {
            phase["square"] = json!([8, 9]);
        }
        assert!(SharedDesktopCommand::from_json(&forged.to_string()).is_err());
        assert_eq!(ctl.shared_token(), expected);
        assert_eq!(document_bytes(&ctl), before);
        assert_eq!(record(&ctl), native);
    }
}

#[test]
fn stopped_product_reopen_and_failed_files_preserve_whole_profile_prior_file_and_dirty_state() {
    // TEN-CONSUMER-SAVED_PROFILE_REOPEN_STOPPED; ordinary product Load restores the saved whole profile.
    let directory = TestDirectory::new();
    let mut ctl = controller();
    for (index, profile) in c::Profile::ALL.into_iter().enumerate() {
        let path = directory.path(&format!("profile-{index}.mshogi"));
        replace(&mut ctl, &fire_root(profile), c::Side::Sente);
        play(&mut ctl, c::Action::single(square(8, 10), square(8, 8))).unwrap();
        send(&mut ctl, SessionOperation::Inspect { event_index: 0 }).unwrap();
        let live = record(&ctl);
        let saved = save(&mut ctl, &path);
        assert!(!saved.document.dirty);
        let prior_file = fs::read(&path).unwrap();
        let other = if profile == c::Profile::AdamsModern {
            c::Profile::MullerCompleted
        } else {
            c::Profile::AdamsModern
        };
        replace(&mut ctl, &c::Game::standard(other), c::Side::Gote);
        let before_open = ctl.shared_open_token();
        let id = ctl.queue_product_open(path.clone(), before_open).unwrap();
        let reopened = wait_file(&mut ctl, id);
        assert_eq!(
            reopened.io.phase,
            IoPhase::Complete,
            "{:?}",
            reopened.io.message
        );
        assert!(reopened.token.document > before_open.document);
        assert_eq!(reopened.profile, RulesProfile::from(profile));
        assert!(reopened.paused && !reopened.historical);
        assert_eq!(reopened.human.id, saved.human.id);
        assert_eq!(fs::read(&path).unwrap(), prior_file);
        assert!(ctl.session.tenjiku().unwrap().pending.is_none());
        assert_eq!(record(&ctl), live);
        assert!(!reopened.document.dirty);
        send(
            &mut ctl,
            SessionOperation::TenjikuPreferences {
                settings: TenjikuSettings {
                    play_mode: PlayMode::HumanEngine,
                    orientation: c::Side::Gote,
                },
            },
        )
        .unwrap();
        assert!(ctl.shared_snapshot().document.dirty);
        let accepted = document_bytes(&ctl);
        let malformed = directory.path("malformed.mshogi");
        let mut mixed: Value = serde_json::from_slice(&prior_file).unwrap();
        mixed["rules_profile"] = json!(match profile {
            c::Profile::AdamsModern => c::Profile::MullerCompleted.id(),
            _ => c::Profile::AdamsModern.id(),
        });
        fs::write(&malformed, mixed.to_string()).unwrap();
        let id = ctl
            .queue_shared_open(malformed, ctl.shared_open_token())
            .unwrap();
        assert_eq!(wait_file(&mut ctl, id).io.phase, IoPhase::Failed);
        assert_eq!(document_bytes(&ctl), accepted);
        assert_eq!(record(&ctl), live);
        assert!(ctl.shared_snapshot().document.dirty);
        let id = ctl
            .queue_shared_save(directory.path("absent/save.mshogi"))
            .unwrap();
        assert_eq!(wait_file(&mut ctl, id).io.phase, IoPhase::Failed);
        assert_eq!(document_bytes(&ctl), accepted);
        assert!(ctl.shared_snapshot().document.dirty);
        assert_eq!(fs::read(&path).unwrap(), prior_file);
    }
}

#[test]
fn cancelled_replacement_receipt_cannot_install_other_profile_or_damage_the_saved_file() {
    let directory = TestDirectory::new();
    let mut ctl = controller();
    for profile in c::Profile::ALL {
        replace(&mut ctl, &root(profile, &[]), c::Side::Sente);
        let path = directory.path("accepted.mshogi");
        save(&mut ctl, &path);
        let bytes = document_bytes(&ctl);
        let file = fs::read(&path).unwrap();
        let native = record(&ctl);
        let authority = ctl.session.tenjiku().unwrap().game.context();
        let epoch = ctl.shared_token().document;
        let other = match profile {
            c::Profile::AdamsModern => c::Profile::MullerCompleted,
            _ => c::Profile::AdamsModern,
        };
        // The production queue does not poll. Retire before the main owner can
        // install even an already completed candidate; no timing race is assumed.
        let receipt = ctl
            .queue_game_candidate::<Tenjiku>(
                native_session::RuntimeSeed::<Tenjiku>::Record {
                    play_mode: PlayMode::HumanEngine,
                    record: Box::new(c::GameRecord::from_game(&root(other, &[])).unwrap()),
                    human: c::Side::Sente,
                },
                None,
                None,
                true,
            )
            .unwrap();
        let cancel_begin = Instant::now();
        ctl.retire_admission();
        let retire_admission_us = cancel_begin.elapsed().as_micros();
        assert!(receipt.completion().unwrap().is_err());
        ctl.shared_snapshot();
        assert_eq!(ctl.shared_token().document, epoch);
        assert_eq!(record(&ctl), native);
        assert_eq!(ctl.session.tenjiku().unwrap().game.context(), authority);
        assert_eq!(document_bytes(&ctl), bytes);
        assert!(!ctl.shared_snapshot().document.dirty);
        assert_eq!(fs::read(&path).unwrap(), file);
        let id = ctl
            .queue_shared_open(path.clone(), ctl.shared_open_token())
            .unwrap();
        let cancel_begin = Instant::now();
        ctl.retire_native_files().unwrap();
        let retire_file_us = cancel_begin.elapsed().as_micros();
        assert!(ctl.io_state.id > id);
        ctl.shared_snapshot();
        assert_eq!(record(&ctl), native);
        assert_eq!(ctl.session.tenjiku().unwrap().game.context(), authority);
        assert_eq!(document_bytes(&ctl), bytes);
        assert_eq!(fs::read(&path).unwrap(), file);
        println!(
            "{}",
            serde_json::json!({"cancel_profile":profile.id(),"retire_admission_us":retire_admission_us,"retire_file_us":retire_file_us,"authority_document_and_prior_file_preserved":true,"scope":"controller retirement latency; native cooperative checkpoints tested separately"})
        );
    }
}

#[test]
fn real_stationary_capture_analysis_preserves_target_in_presented_pv() {
    for profile in c::Profile::ALL {
        let pieces = [
            (16, 16, c::Side::Sente, c::Base::K, false),
            (8, 8, c::Side::Sente, c::Base::ChS, true),
            (9, 8, c::Side::Gote, c::Base::K, false),
            (2, 3, c::Side::Gote, c::Base::P, false),
        ]
        .into_iter()
        .enumerate()
        .map(|(id, (f, r, s, b, p))| (square(f, r), c::Piece::new(id as u16, s, b, p).unwrap()))
        .collect::<Vec<_>>();
        let game = c::Game::from_root(
            c::Position::from_diagram(
                profile,
                &pieces,
                c::Side::Sente,
                c::RootMetadata::diagram(0),
            )
            .unwrap(),
        )
        .unwrap();
        let mut ctl = controller();
        send(
            &mut ctl,
            SessionOperation::Replace {
                record: CanonicalRecord::tenjiku(c::GameRecord::from_game(&game).unwrap()),
                human: NativeSide::Tenjiku(c::Side::Sente),
            },
        )
        .unwrap();
        let before = record(&ctl);
        ctl.start_game_work::<Tenjiku>(mshogi_engine::TENJIKU_PLAY_LIMITS, true)
            .unwrap();
        let pending = ctl.session.tenjiku().unwrap().pending.clone().unwrap();
        let response = execute(&pending);
        let ExecutionResult::Tenjiku {
            result: mshogi_engine::TenjikuResult::Choice { action, .. },
        } = &response.observation.result
        else {
            panic!("{response:?}")
        };
        assert_eq!(action.kind, c::ActionKind::Igui);
        assert_eq!(action.target, Some(square(9, 8)));
        assert!(ctl.accept_shared(response));
        let receipt = ctl.admission_receipt.clone().unwrap();
        let snapshot = settle(&mut ctl, receipt).unwrap();
        assert_eq!(
            snapshot.search_observations[0].variation,
            vec!["8h!x9h (stationary capture)"]
        );
        assert_eq!(record(&ctl), before);
        assert!(!snapshot.status.concluded);
    }
}

#[test]
fn shared_core_analysis_bounds_preserve_both_profile_states_and_reject_excess() {
    for profile in c::Profile::ALL {
        let mut ctl = controller();
        let game = root(profile, &[]);
        send(
            &mut ctl,
            SessionOperation::Replace {
                record: CanonicalRecord::tenjiku(c::GameRecord::from_game(&game).unwrap()),
                human: NativeSide::Tenjiku(c::Side::Sente),
            },
        )
        .unwrap();
        let before = record(&ctl);
        let snapshot = send(
            &mut ctl,
            SessionOperation::Analyze {
                operation: AnalysisOperation::Moves {
                    depth: 64,
                    nodes: 5_000_000,
                },
            },
        )
        .unwrap();
        assert_eq!(snapshot.execution_purpose, Some("move_analysis"));
        assert_eq!(snapshot.play_limits, mshogi_engine::TENJIKU_PLAY_LIMITS);
        assert_eq!(record(&ctl), before);
        send(&mut ctl, SessionOperation::Cancel {}).unwrap();
        for (depth, nodes) in [(65, 1), (1, 5_000_001), (1, 0)] {
            let token = ctl.shared_token();
            assert!(
                send(
                    &mut ctl,
                    SessionOperation::Analyze {
                        operation: AnalysisOperation::Moves { depth, nodes },
                    }
                )
                .is_err()
            );
            let after = ctl.shared_token();
            assert_eq!(
                (after.document, after.version, after.selection),
                (token.document, token.version, token.selection)
            );
            assert_eq!(record(&ctl), before);
        }
    }
}
