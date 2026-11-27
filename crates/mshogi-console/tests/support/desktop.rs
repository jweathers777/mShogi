use mshogi_core::{chess::Color, game::TaggedSide};
use mshogi_desktop_session::{
    ChessPresentation, DesktopController, SessionOperation, SessionToken, SharedDesktopCommand,
    SharedOpenToken, SharedSnapshot,
};
use std::{
    thread,
    time::{Duration, Instant},
};
pub fn command(c: &mut DesktopController, operation: SessionOperation) -> SharedSnapshot {
    let snapshot = c.shared_snapshot();
    let (_, receipt) = c
        .shared_command_with_receipt(SharedDesktopCommand {
            expected: snapshot.token,
            game: snapshot.game,
            operation,
        })
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let snapshot = c.shared_snapshot();
        if let Some(result) = receipt.completion() {
            result.unwrap();
            return snapshot;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(1));
    }
}
pub fn chess(snapshot: &SharedSnapshot) -> &ChessPresentation {
    snapshot.chess.as_ref().expect("Chess capabilities")
}
pub fn history(snapshot: &SharedSnapshot) -> Vec<String> {
    live_game(snapshot)
        .history()
        .actions()
        .map(|a| a.to_string())
        .collect()
}
pub fn fen(snapshot: &SharedSnapshot) -> String {
    live_game(snapshot).position().to_fen()
}
pub fn side_to_move(snapshot: &SharedSnapshot) -> Color {
    let TaggedSide::Chess(side) = snapshot.turn.id else {
        panic!("Chess turn")
    };
    side
}
pub fn human(snapshot: &SharedSnapshot) -> Color {
    let TaggedSide::Chess(side) = snapshot.human.id else {
        panic!("Chess human")
    };
    side
}
pub trait SettledCommand {
    fn settled(
        &mut self,
        expected: SessionToken,
        operation: SessionOperation,
    ) -> Result<SharedSnapshot, String>;
    fn fresh(&mut self, expected: SharedOpenToken, human: Color) -> Result<SharedSnapshot, String>;
}
pub fn complete(
    controller: &mut DesktopController,
    mut snapshot: SharedSnapshot,
    receipt: mshogi_desktop_session::AdmissionReceipt,
) -> Result<SharedSnapshot, String> {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        if let Some(result) = receipt.completion() {
            result?;
            return Ok(snapshot);
        }
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
        snapshot = controller.shared_snapshot();
    }
}
impl SettledCommand for DesktopController {
    fn settled(
        &mut self,
        expected: SessionToken,
        operation: SessionOperation,
    ) -> Result<SharedSnapshot, String> {
        let (snapshot, receipt) = self.shared_command_with_receipt(SharedDesktopCommand {
            expected,
            game: mshogi_desktop_session::GameKind::Chess,
            operation,
        })?;
        complete(self, snapshot, receipt)
    }
    fn fresh(&mut self, expected: SharedOpenToken, human: Color) -> Result<SharedSnapshot, String> {
        let (snapshot, receipt) = self.shared_new_game(
            expected,
            mshogi_core::game::RulesProfile::ChessMatchV1,
            TaggedSide::Chess(human),
        )?;
        complete(self, snapshot, receipt)
    }
}

pub fn live_game(snapshot: &SharedSnapshot) -> mshogi_core::chess::AdjudicatedGame {
    snapshot
        .chess_match
        .as_ref()
        .unwrap()
        .replay()
        .unwrap()
        .game()
        .clone()
}
pub fn shutdown(controller: &mut DesktopController) {
    controller.begin_shutdown();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !controller.shutdown_acknowledged() {
        assert!(Instant::now() < deadline);
        thread::sleep(Duration::from_millis(1));
    }
}
pub fn until(
    controller: &mut DesktopController,
    seconds: u64,
    ready: impl Fn(&SharedSnapshot) -> bool,
) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(seconds);
    loop {
        let snapshot = controller.shared_snapshot();
        if ready(&snapshot) {
            return snapshot;
        }
        assert!(
            Instant::now() < deadline,
            "completion deadline: {snapshot:?}"
        );
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn wait_file(controller: &mut DesktopController, id: u64) -> SharedSnapshot {
    use mshogi_desktop_session::IoPhase;
    let snapshot = until(controller, 20, |s| {
        s.io.id == id && matches!(s.io.phase, IoPhase::Complete | IoPhase::Failed)
    });
    assert_eq!(snapshot.io.phase, IoPhase::Complete, "{:?}", snapshot.io);
    snapshot
}
