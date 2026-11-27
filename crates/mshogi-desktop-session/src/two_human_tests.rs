//! MSH-172 participant policy through the real shared controller and game records.
//! These checks exercise application integration, not new rules conformance.
use super::*;
use mshogi_core::game::{GameKind, RulesProfile, TaggedSide};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

const PROFILES: [RulesProfile; 7] = [
    RulesProfile::ChessMatchV1,
    RulesProfile::ShogiCsa36V1,
    RulesProfile::XiangqiWxf2018V1,
    RulesProfile::ToriModernV1,
    RulesProfile::ChuJcsa2019V1,
    RulesProfile::TenjikuAdamsModernV1,
    RulesProfile::TenjikuMullerCompletedV1,
];

struct Files(PathBuf);
impl Files {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "msh172-local-play-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn controller() -> DesktopController {
    let mut controller = DesktopController::new("/unused/msh172-manual-engine".into()).unwrap();
    controller.shared.manual_engine = true;
    controller
}
fn anchor(game: GameKind) -> TaggedSide {
    // The second side is deliberately the fixed orientation/human anchor. A local
    // first move must not depend on ownership of that old single-human anchor.
    match game {
        GameKind::Chess => TaggedSide::Chess(Color::Black),
        GameKind::Shogi => TaggedSide::Shogi(mshogi_core::shogi::Side::Gote),
        GameKind::Xiangqi => TaggedSide::Xiangqi(mshogi_core::xiangqi::Side::Black),
        GameKind::Tori => TaggedSide::Tori(mshogi_core::tori::Side::Gote),
        GameKind::Chu => TaggedSide::Chu(mshogi_core::chu::Side::Gote),
        GameKind::Tenjiku => TaggedSide::Tenjiku(mshogi_core::tenjiku::Side::Gote),
    }
}
fn settled(c: &mut DesktopController, receipt: AdmissionReceipt) -> Result<SharedSnapshot, String> {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = c.shared_snapshot();
        if let Some(result) = receipt.completion() {
            result?;
            return Ok(snapshot);
        }
        assert!(Instant::now() < deadline, "participant admission deadline");
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn local(c: &mut DesktopController, profile: RulesProfile) -> SharedSnapshot {
    let (_, receipt) = c
        .shared_new_game_with_mode(
            c.shared_open_token(),
            profile,
            anchor(profile.game()),
            PlayMode::TwoHumans,
        )
        .unwrap();
    settled(c, receipt).unwrap()
}
fn send(c: &mut DesktopController, operation: SessionOperation) -> Result<SharedSnapshot, String> {
    let game = c.shared_snapshot().game;
    let inspect = if let SessionOperation::Inspect { event_index } = &operation {
        (game == GameKind::Chess).then_some(*event_index)
    } else {
        None
    };
    let (started, receipt) = c.shared_command_with_receipt(SharedDesktopCommand {
        expected: c.shared_token(),
        game,
        operation,
    })?;
    let mut snapshot = settled(c, receipt)?;
    if let Some(event_index) = inspect {
        let deadline = Instant::now() + Duration::from_secs(30);
        while snapshot.io.phase == IoPhase::Viewing {
            assert_eq!(snapshot.io.id, started.io.id);
            assert!(Instant::now() < deadline, "history view deadline");
            std::thread::sleep(Duration::from_millis(1));
            snapshot = c.shared_snapshot();
        }
        if snapshot.io.phase == IoPhase::Failed {
            return Err(snapshot.io.message.unwrap_or_default());
        }
        assert_eq!(snapshot.event_index, event_index);
    }
    Ok(snapshot)
}
fn operation(snapshot: &SharedSnapshot) -> SessionOperation {
    SessionOperation::play(&snapshot.choices[0].action)
}
fn play(c: &mut DesktopController) -> SharedSnapshot {
    let snapshot = c.shared_snapshot();
    assert!(snapshot.capabilities.board_action, "{:?}", snapshot.profile);
    send(c, operation(&snapshot)).unwrap()
}
fn pending(c: &DesktopController) -> Option<ExecutionContext> {
    match &c.session {
        ActiveDocument::Chess { state, .. } => state.pending.clone(),
        ActiveDocument::Shogi { state, .. } => state.pending.clone(),
        ActiveDocument::Xiangqi { state, .. } => state.pending.clone(),
        ActiveDocument::Tori { state, .. } => state.pending.clone(),
        ActiveDocument::Chu { state, .. } => state.pending.clone(),
        ActiveDocument::Tenjiku { state, .. } => state.pending.clone(),
    }
}
fn accept(c: &mut DesktopController, observation: ExecutionObservation) -> bool {
    c.accept_observation(observation)
}
fn no_engine(c: &mut DesktopController) {
    let snapshot = c.shared_snapshot();
    assert!(!snapshot.capabilities.engine_turn, "{:?}", snapshot.profile);
    assert!(
        snapshot.execution_purpose.is_none(),
        "{:?}",
        snapshot.profile
    );
    assert!(pending(c).is_none(), "{:?}", snapshot.profile);
}
fn file(c: &mut DesktopController, id: u64) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = c.shared_snapshot();
        if snapshot.io.id == id && matches!(snapshot.io.phase, IoPhase::Complete | IoPhase::Failed)
        {
            assert_eq!(
                snapshot.io.phase,
                IoPhase::Complete,
                "{:?}",
                snapshot.io.message
            );
            return snapshot;
        }
        assert!(
            Instant::now() < deadline,
            "participant persistence deadline"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn wait_admission(c: &mut DesktopController) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = c.shared_snapshot();
        if snapshot.capacity.status != CapacityPhase::Checking {
            return snapshot;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
}

#[test]
fn local_both_sides_one_action_takeback_and_saved_mode_work_in_all_seven_profiles() {
    let files = Files::new();
    let mut c = controller();
    for profile in PROFILES {
        let root = local(&mut c, profile);
        assert_eq!(root.play_mode, PlayMode::TwoHumans);
        assert_eq!(root.local_actor.id, root.turn.id);
        assert!(root.local_actor.first_mover);
        assert!(!root.human.first_mover);
        no_engine(&mut c);
        let first = play(&mut c);
        assert_eq!(first.board_ply, 1, "{profile:?}");
        assert!(!first.local_actor.first_mover);
        assert_eq!(first.local_actor.id, first.turn.id);
        no_engine(&mut c);
        let second = play(&mut c);
        assert_eq!(second.board_ply, 2, "{profile:?}");
        assert!(second.local_actor.first_mover);
        no_engine(&mut c);

        let viewed = send(&mut c, SessionOperation::Inspect { event_index: 0 }).unwrap();
        assert!(viewed.historical);
        assert!(!viewed.capabilities.board_action);
        assert!(send(&mut c, operation(&second)).is_err());
        let live = send(&mut c, SessionOperation::ReturnLive {}).unwrap();
        assert_eq!(live.context, second.context);
        no_engine(&mut c);
        let undone = send(&mut c, SessionOperation::TakeBack {}).unwrap();
        assert_eq!(undone.context, first.context, "{profile:?}");
        assert_eq!(undone.board_ply, 1);
        assert!(undone.paused);
        assert!(!undone.capabilities.board_action);
        assert_eq!(undone.play_mode, PlayMode::TwoHumans);
        no_engine(&mut c);
        let resumed = send(&mut c, SessionOperation::Resume {}).unwrap();
        assert!(!resumed.paused);
        assert_eq!(resumed.local_actor.id, first.turn.id);
        no_engine(&mut c);
        let replayed = play(&mut c);
        assert_eq!(replayed.board_ply, 2);

        let path = files.0.join(format!("{}.mshogi", profile.id()));
        let id = c.queue_shared_save(path.clone()).unwrap();
        let saved = file(&mut c, id);
        assert!(!saved.document.dirty);
        let id = c.queue_shared_open(path, saved.open_token).unwrap();
        let opened = file(&mut c, id);
        assert_eq!(opened.play_mode, PlayMode::TwoHumans);
        assert_eq!(opened.context, replayed.context);
        assert_eq!(opened.board_ply, 2);
        assert_eq!(opened.moves.entries.len(), replayed.moves.entries.len());
        assert!(opened.paused && !opened.historical);
        assert!(!opened.capabilities.board_action);
        assert_eq!(opened.local_actor.id, opened.turn.id);
        no_engine(&mut c);
        send(&mut c, SessionOperation::Resume {}).unwrap();
        no_engine(&mut c);
        assert_eq!(play(&mut c).board_ply, 3);
    }
}

#[test]
fn local_resignation_names_current_player_and_terminal_takeback_restores_the_exact_board() {
    let mut c = controller();
    for profile in PROFILES {
        for plies in [0, 1] {
            let mut before = local(&mut c, profile);
            if plies == 1 {
                before = play(&mut c);
            }
            let resigned = send(&mut c, SessionOperation::Resign {}).unwrap();
            assert!(resigned.status.concluded, "{profile:?}");
            assert_eq!(resigned.board_ply, before.board_ply);
            if profile.game() == GameKind::Chess {
                // Chess does not invent a mating certificate for resignation.
                assert_eq!(resigned.status.score_half_points, None);
                assert!(matches!(resigned.chess.as_ref().unwrap().live_conclusion,
                    Some(MatchConclusion::Unresolved { actor, trigger: TerminationTrigger::Resignation })
                    if TaggedSide::Chess(actor) == before.turn.id));
            } else {
                assert_eq!(
                    resigned.status.score_half_points,
                    Some(if before.turn.first_mover {
                        [0, 2]
                    } else {
                        [2, 0]
                    }),
                    "{profile:?}"
                );
            }
            no_engine(&mut c);
            let undone = send(&mut c, SessionOperation::TakeBack {}).unwrap();
            assert!(!undone.status.concluded, "{profile:?}");
            assert_eq!(undone.context, before.context, "{profile:?}");
            assert!(undone.paused);
            assert!(!undone.capabilities.board_action);
            assert_eq!(undone.local_actor.id, before.turn.id);
        }
    }
}

#[test]
fn local_analysis_is_explicit_and_read_only_for_all_seven_profiles() {
    let mut c = controller();
    for profile in PROFILES {
        local(&mut c, profile);
        let before = play(&mut c);
        no_engine(&mut c);
        send(
            &mut c,
            SessionOperation::Analyze {
                operation: AnalysisOperation {
                    depth: 1,
                    nodes: 128,
                },
            },
        )
        .unwrap();
        let context = pending(&c).expect("explicit analysis must allocate work");
        assert!(matches!(
            context.purpose,
            mshogi_engine::ExecutionPurpose::MoveAnalysis { .. }
        ));
        let mut bytes = Vec::new();
        mshogi_engine::serve_once(
            std::io::Cursor::new(format!(
                "{}\n",
                serde_json::to_string(&execution_request(context.clone())).unwrap()
            )),
            &mut bytes,
        )
        .unwrap();
        assert!(accept(
            &mut c,
            decode_execution_observation(&bytes, &context)
        ));
        let analyzed = wait_admission(&mut c);
        assert_eq!(analyzed.context, before.context, "{profile:?}");
        assert_eq!(analyzed.local_actor.id, before.local_actor.id);
        assert_eq!(analyzed.play_mode, PlayMode::TwoHumans);
        assert!(analyzed.paused);
        assert!(!analyzed.capabilities.board_action);
        assert!(analyzed.analysis.is_some(), "{profile:?}");
        no_engine(&mut c);
        send(&mut c, SessionOperation::Resume {}).unwrap();
        no_engine(&mut c);
    }
}

#[test]
fn switching_to_local_play_retires_every_old_engine_authority() {
    let mut c = controller();
    for profile in PROFILES {
        let (_, receipt) = c
            .shared_new_game(c.shared_open_token(), profile, anchor(profile.game()))
            .unwrap();
        settled(&mut c, receipt).unwrap();
        c.shared_snapshot();
        let old = pending(&c).expect("old opponent turn");
        let before = local(&mut c, profile);
        let accepted = accept(
            &mut c,
            ExecutionObservation::new(
                old,
                ExecutionResult::Failure {
                    failure: mshogi_engine::EngineFailure {
                        kind: mshogi_engine::FailureKind::Start,
                        message: "retired old opponent".into(),
                    },
                },
            ),
        );
        assert!(!accepted, "{profile:?}");
        let after = c.shared_snapshot();
        assert_eq!(after.context, before.context);
        assert_eq!(after.token, before.token);
        assert_eq!(after.play_mode, PlayMode::TwoHumans);
        assert_eq!(after.local_actor.id, before.local_actor.id);
        no_engine(&mut c);
    }
}

fn local_record(c: &mut DesktopController, record: CanonicalRecord) -> SharedSnapshot {
    let play_mode = PlayMode::TwoHumans;
    let resume = Some(SessionOperation::Resume {});
    let receipt = match record {
        CanonicalRecord::Shogi(record) => c.queue_game_candidate::<Shogi>(
            game_session::RuntimeSeed::<Shogi>::Record {
                record,
                human: mshogi_core::shogi::Side::Gote,
                play_mode,
            },
            resume,
            None,
            true,
        ),
        CanonicalRecord::Tori(record) => c.queue_game_candidate::<Tori>(
            game_session::RuntimeSeed::<Tori>::Record {
                record,
                human: mshogi_core::tori::Side::Gote,
                play_mode,
            },
            resume,
            None,
            true,
        ),
        CanonicalRecord::Chu(record) => c.queue_game_candidate::<Chu>(
            game_session::RuntimeSeed::<Chu>::Record {
                record,
                human: mshogi_core::chu::Side::Gote,
                play_mode,
            },
            resume,
            None,
            true,
        ),
        CanonicalRecord::Tenjiku(record) => c.queue_game_candidate::<Tenjiku>(
            game_session::RuntimeSeed::<Tenjiku>::Record {
                record,
                human: mshogi_core::tenjiku::Side::Gote,
                play_mode,
            },
            resume,
            None,
            true,
        ),
        _ => unreachable!("complete-action fixture game"),
    }
    .unwrap();
    settled(c, receipt).unwrap()
}
fn complete(c: &mut DesktopController, action: &str) -> SharedSnapshot {
    let before = c.shared_snapshot();
    assert!(
        before.choices.iter().any(|choice| choice.action == action),
        "{action}"
    );
    // Reuse the exact selected wire action, not a frontend partial route.
    let mut chosen = c.shared_snapshot();
    let i = chosen
        .choices
        .iter()
        .position(|choice| choice.action == action)
        .unwrap();
    chosen.choices.swap(0, i);
    let after = send(c, operation(&chosen)).unwrap();
    assert_eq!(after.board_ply, before.board_ply + 1);
    assert_eq!(after.play_mode, PlayMode::TwoHumans);
    no_engine(c);
    after
}

#[test]
fn local_complete_drop_promotion_and_compound_actions_use_shared_transport() {
    use mshogi_core::{chu as ch, shogi as sh, tenjiku as ten, tori as to};
    let mut c = controller();
    // These reduced Shogi roots and explicit actions already appear in the
    // desktop complete-action regression, now with the old human anchor
    // deliberately on the other side.
    for (sfen, action, promoted) in [
        ("8k/9/9/4P4/9/9/9/9/K8 b - 1", "5d5c+", true),
        ("8k/9/9/9/9/9/9/9/K8 b 2RBGSNLP 1", "P*5e", false),
    ] {
        let game = sh::Game::from_root(
            sh::GameRoot::import(
                sh::Position::from_sfen(sfen).unwrap(),
                sh::RootCategory::ReducedMaterialDiagram,
                sh::CounterSource::Supplied,
            )
            .unwrap(),
        );
        local_record(
            &mut c,
            CanonicalRecord::shogi(sh::GameRecord::from_game(&game)),
        );
        let after = complete(&mut c, action);
        assert!(
            after
                .pieces
                .iter()
                .any(|piece| piece.square == if promoted { "5c" } else { "5e" }
                    && piece.promoted == promoted)
        );
    }
    // The shared Tori contract root has real hand resources and an optional
    // Falcon promotion. Select complete actions in both categories.
    for drop in [true, false] {
        let game = crate::tori_fixture_roots::contract_game();
        let snapshot = local_record(
            &mut c,
            CanonicalRecord::tori(to::GameRecord::from_game(&game).unwrap()),
        );
        let action = snapshot
            .choices
            .iter()
            .find(|choice| {
                let action: to::Action = serde_json::from_str(&choice.action).unwrap();
                if drop {
                    matches!(action, to::Action::Drop { .. })
                } else {
                    matches!(
                        action,
                        to::Action::Move {
                            result_face: to::Face::Eagle,
                            ..
                        }
                    )
                }
            })
            .expect("reviewed Tori root has requested complete action")
            .action
            .clone();
        complete(&mut c, &action);
    }
    // Frozen Chu corpus supplies exact complete routes and optional promotion.
    for id in ["CHU-LION-C2-1", "CHU-PROMOTE-P-YES"] {
        let fixture = crate::chu_fixture_roots::fixture(id);
        let game = crate::chu_fixture_roots::root(&fixture["given"]["root"]);
        let action = crate::chu_fixture_roots::action(if id == "CHU-LION-C2-1" {
            &fixture["given"]["commands"][0]
        } else {
            &fixture["given"]["action"]
        });
        if id == "CHU-LION-C2-1" {
            assert!(action.route.len() > 1);
        } else {
            assert!(action.promote);
        }
        local_record(
            &mut c,
            CanonicalRecord::chu(ch::GameRecord::from_game(&game).unwrap()),
        );
        complete(&mut c, &serde_json::to_string(&action).unwrap());
    }
    // Reuse the exact sparse root and route/promotion geometry from the reviewed
    // shared Tenjiku regressions for each immutable profile.
    for profile in ten::Profile::ALL {
        for promote in [false, true] {
            let entries = [
                (16, 16, ten::Side::Sente, ten::Base::K),
                (15, 14, ten::Side::Sente, ten::Base::P),
                (1, 1, ten::Side::Gote, ten::Base::K),
                (2, 3, ten::Side::Gote, ten::Base::P),
                (
                    8,
                    if promote { 6 } else { 8 },
                    ten::Side::Sente,
                    if promote { ten::Base::G } else { ten::Base::Ln },
                ),
            ]
            .into_iter()
            .enumerate()
            .map(|(id, (file, rank, side, base))| {
                (
                    ten::Square::new(file, rank).unwrap(),
                    ten::Piece::new(id as u16, side, base, false).unwrap(),
                )
            })
            .collect::<Vec<_>>();
            let game = ten::Game::from_root(
                ten::Position::from_diagram(
                    profile,
                    &entries,
                    ten::Side::Sente,
                    ten::RootMetadata::imported("shared-tenjiku-integration".into(), 120).unwrap(),
                )
                .unwrap(),
            )
            .unwrap();
            local_record(
                &mut c,
                CanonicalRecord::tenjiku(ten::GameRecord::from_game(&game).unwrap()),
            );
            let action = if promote {
                let mut action = ten::Action::single(
                    ten::Square::new(8, 6).unwrap(),
                    ten::Square::new(8, 5).unwrap(),
                );
                action.promote = true;
                action
            } else {
                ten::Action {
                    kind: ten::ActionKind::Travel,
                    from: ten::Square::new(8, 8).unwrap(),
                    route: vec![
                        ten::Square::new(8, 7).unwrap(),
                        ten::Square::new(9, 7).unwrap(),
                    ],
                    promote: false,
                    target: None,
                }
            };
            complete(&mut c, &serde_json::to_string(&action).unwrap());
        }
    }
}
