//! One controller crosses all six games and both immutable Tenjiku profiles.
//! Manual execution allocation exercises authority retirement, not subprocess play.
use super::*;
use mshogi_core::native::{GameKind, NativeSide, RulesProfile};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::{Duration, Instant},
};

struct Files(PathBuf);
impl Files {
    fn new() -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "msh88-seven-profiles-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}
impl Drop for Files {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn settled(c: &mut DesktopController, receipt: AdmissionReceipt) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = c.shared_snapshot();
        if let Some(result) = receipt.completion() {
            result.unwrap();
            return snapshot;
        }
        assert!(Instant::now() < deadline, "admission did not settle");
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn file(c: &mut DesktopController, id: u64, succeeds: bool) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = c.shared_snapshot();
        if snapshot.io.id == id && matches!(snapshot.io.phase, IoPhase::Complete | IoPhase::Failed)
        {
            assert_eq!(
                snapshot.io.phase == IoPhase::Complete,
                succeeds,
                "{:?}",
                snapshot.io
            );
            return snapshot;
        }
        assert!(Instant::now() < deadline, "file operation did not settle");
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn second_side(game: GameKind) -> NativeSide {
    use mshogi_core::{chu, shogi, tenjiku, tori, xiangqi};
    match game {
        GameKind::Chess => NativeSide::Chess(Color::Black),
        GameKind::Shogi => NativeSide::Shogi(shogi::Side::Gote),
        GameKind::Xiangqi => NativeSide::Xiangqi(xiangqi::Side::Black),
        GameKind::Tori => NativeSide::Tori(tori::Side::Gote),
        GameKind::Chu => NativeSide::Chu(chu::Side::Gote),
        GameKind::Tenjiku => NativeSide::Tenjiku(tenjiku::Side::Gote),
    }
}
fn pending(c: &mut DesktopController) -> ExecutionContext {
    c.shared_snapshot();
    match &c.session {
        ActiveDocument::Chess(_) => c
            .session
            .chess()
            .pending
            .clone()
            .unwrap_or_else(|| c.session.chess_mut().engine_request().unwrap().context),
        ActiveDocument::Shogi { state, .. } => state.pending.clone().unwrap(),
        ActiveDocument::Xiangqi { state, .. } => state.pending.clone().unwrap(),
        ActiveDocument::Tori { state, .. } => state.pending.clone().unwrap(),
        ActiveDocument::Chu { state, .. } => state.pending.clone().unwrap(),
        ActiveDocument::Tenjiku { state, .. } => state.pending.clone().unwrap(),
    }
}
struct Retired {
    snapshot: SharedSnapshot,
    execution: ExecutionContext,
}
fn reject_retired(c: &mut DesktopController, retired: &Retired, saved: &Path) {
    let before = c.shared_snapshot();
    assert!(
        c.shared_command(SharedDesktopCommand {
            expected: retired.snapshot.token,
            game: retired.snapshot.game,
            operation: SessionOperation::Resign {},
        })
        .is_err()
    );
    assert!(
        c.shared_inspection(MovementInspectionQuery {
            expected: retired.snapshot.token,
            origin: retired.snapshot.pieces[0].square.clone(),
        })
        .is_err()
    );
    assert!(
        c.shared_new_game(
            retired.snapshot.open_token,
            retired.snapshot.profile,
            retired.snapshot.human.id,
        )
        .is_err()
    );
    assert!(
        c.queue_shared_open(saved.to_path_buf(), retired.snapshot.open_token)
            .is_err()
    );
    let response = ExecutionResponse {
        protocol: mshogi_engine::PROTOCOL_VERSION,
        observation: ExecutionObservation::new(
            retired.execution.clone(),
            mshogi_engine::ExecutionResult::Failure {
                failure: mshogi_engine::EngineFailure {
                    kind: mshogi_engine::FailureKind::Start,
                    message: "retired cross-game work".into(),
                },
            },
        ),
    };
    let accepted = if matches!(c.session, ActiveDocument::Chess(_)) {
        c.session.chess_mut().accept(response, 1)
    } else {
        c.accept_shared(response)
    };
    assert!(!accepted);
    let after = c.shared_snapshot();
    assert_eq!(after.token, before.token);
    assert_eq!(after.open_token, before.open_token);
    assert_eq!(after.context, before.context);
    assert_eq!(after.document.dirty, before.document.dirty);
    assert_eq!(after.document.path, before.document.path);
    assert_eq!(after.observations.len(), before.observations.len());
}

#[test]
fn seven_profile_new_and_current_load_rings_retire_all_prior_authorities_atomically() {
    use RulesProfile::*;
    let profiles = [
        ChessMatchV1,
        ShogiCsa36V1,
        XiangqiWxf2018V1,
        ToriModernV1,
        ChuJcsa2019V1,
        TenjikuAdamsModernV1,
        TenjikuMullerCompletedV1,
    ];
    let files = Files::new();
    let corrupt = files.path("corrupt.mshogi");
    fs::write(&corrupt, b"{invalid current document").unwrap();
    let mut c = DesktopController::new("/unused/msh88-manual-engine".into()).unwrap();
    c.shared.manual_engine = true;
    let mut retired = Vec::new();
    let mut documents = Vec::new();
    for profile in profiles {
        let before = c.shared_snapshot();
        let (_, receipt) = c
            .shared_new_game(before.open_token, profile, second_side(profile.game()))
            .unwrap();
        let fresh = settled(&mut c, receipt);
        assert_eq!(fresh.profile, profile);
        assert_eq!(fresh.edition, profile.edition());
        assert!(fresh.token.document > before.token.document);
        assert!(fresh.token.version > before.token.version);
        assert!(!fresh.paused && !fresh.historical);
        assert!(fresh.document.dirty && fresh.document.path.is_none());
        let execution = pending(&mut c);
        assert_eq!(execution.game.profile(), profile);
        for old in &retired {
            reject_retired(&mut c, old, &corrupt);
        }
        assert_eq!(pending(&mut c), execution);
        let path = files.path(&format!("{}.mshogi", profile.id()));
        let id = c.queue_shared_save(path.clone()).unwrap();
        let saved = file(&mut c, id, true);
        assert!(!saved.document.dirty);
        let bytes = fs::read(&path).unwrap();
        let envelope = crate::document::DocumentEnvelope::parse(&bytes).unwrap();
        assert_eq!(envelope.rules_profile, profile);
        assert_eq!(envelope.game_kind, profile.game());
        assert_eq!(envelope.profile_edition, profile.edition());
        assert_eq!(envelope.native_schema_version, 1);
        let id = c
            .queue_shared_open(corrupt.clone(), saved.open_token)
            .unwrap();
        let failed = file(&mut c, id, false);
        // File progress changes the snapshot revision, not accepted content or
        // the document/session/selection authority used by outstanding work.
        assert_eq!(failed.open_token, saved.open_token);
        assert_eq!(failed.token.selection, saved.token.selection);
        assert_eq!(failed.context, saved.context);
        assert_eq!(failed.document.dirty, saved.document.dirty);
        assert_eq!(failed.document.path, saved.document.path);
        assert_eq!(pending(&mut c), execution);
        assert_eq!(fs::read(&path).unwrap(), bytes);
        let inspection = c
            .shared_inspection(MovementInspectionQuery {
                expected: failed.token,
                origin: failed.pieces[0].square.clone(),
            })
            .unwrap();
        assert_eq!(inspection.game, profile.game());
        assert!(inspection.read_only);
        documents.push((path, bytes, saved.context.clone()));
        retired.push(Retired {
            snapshot: failed,
            execution,
        });
    }
    // Reverse Load returns to earlier game/profile identities without restoring
    // any authority that happened to belong to that same game before replacement.
    for (path, bytes, expected) in documents.iter().rev() {
        let before = c.shared_snapshot();
        let id = c
            .queue_shared_open(path.clone(), before.open_token)
            .unwrap();
        let loaded = file(&mut c, id, true);
        assert_eq!(&loaded.context, expected);
        assert_eq!(loaded.profile, expected.profile());
        assert!(loaded.token.document > before.token.document);
        assert!(loaded.token.version > before.token.version);
        assert!(loaded.paused && !loaded.historical);
        assert!(!loaded.document.dirty);
        assert!(loaded.execution_purpose.is_none());
        for old in &retired {
            reject_retired(&mut c, old, path);
        }
        assert_eq!(&fs::read(path).unwrap(), bytes);
        // Reopening itself remains stopped. Explicit Resume allocates new work
        // for the next cross-game Load to retire, never saved pending work.
        let (_, receipt) = c
            .shared_command_with_receipt(SharedDesktopCommand {
                expected: c.shared_token(),
                game: loaded.game,
                operation: SessionOperation::Resume {},
            })
            .unwrap();
        let resumed = settled(&mut c, receipt);
        assert!(!resumed.paused);
        let execution = pending(&mut c);
        assert_eq!(execution.game.profile(), loaded.profile);
        for old in &retired {
            assert!(execution.request_id > old.execution.request_id);
            assert!(execution.process_generation > old.execution.process_generation);
        }
        retired.push(Retired {
            snapshot: c.shared_snapshot(),
            execution,
        });
    }
    c.begin_shutdown();
}
