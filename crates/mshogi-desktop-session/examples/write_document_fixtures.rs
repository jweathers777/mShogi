//! Generate current-format, rules-admitted documents for ordinary packaged GUI
//! verification. This example is not a host fixture initializer or bundled code.
use mshogi_core::{
    native::{CanonicalRecord, NativeSide},
    shogi as s,
};
use mshogi_desktop_session::{
    CapacityPhase, DesktopController, IoPhase, SessionOperation, SharedDesktopCommand,
};
use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

fn settle(controller: &mut DesktopController) {
    let deadline = Instant::now() + Duration::from_secs(30);
    while controller.shared_snapshot().capacity.status == CapacityPhase::Checking {
        assert!(Instant::now() < deadline, "fixture admission timed out");
        std::thread::sleep(Duration::from_millis(1));
    }
}

fn main() {
    let output = PathBuf::from(std::env::args_os().nth(1).expect(
        "usage: cargo run --locked -p mshogi-desktop-session --example write_document_fixtures -- <output-directory>",
    ));
    std::fs::create_dir_all(&output).unwrap();
    let mut game = s::Game::from_root(
        s::GameRoot::import(
            s::Position::from_sfen("8k/9/4+p4/9/4Rr3/9/5P3/9/K8 b - 17").unwrap(),
            s::RootCategory::ReducedMaterialDiagram,
            s::CounterSource::Supplied,
        )
        .unwrap(),
    );
    // Independent semantic transcript: Sente captures a promoted pawn with a
    // promoting rook; Gote captures the other pawn with its promoting rook.
    for action in ["5e5c+", "4e4g+"] {
        game.submit(s::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: s::GameCommand::Play(action.parse().unwrap()),
        })
        .unwrap();
    }
    assert_eq!(game.events().len(), 2);
    assert_eq!(game.position().side_to_move(), s::Side::Sente);
    for (side, square) in [(s::Side::Sente, "5c"), (s::Side::Gote, "4g")] {
        assert_eq!(game.position().hand(side).count(s::PieceKind::Pawn), 1);
        let piece = game.position().piece(square.parse().unwrap()).unwrap();
        assert_eq!(piece.side, side);
        assert_eq!(piece.kind, s::PieceKind::Rook);
        assert!(piece.promoted);
    }
    let mut controller = DesktopController::new("/unused/document-fixture-engine".into()).unwrap();
    for operation in [
        SessionOperation::Replace {
            record: CanonicalRecord::shogi(s::GameRecord::from_game(&game)),
            human: NativeSide::Shogi(s::Side::Sente),
        },
        SessionOperation::Resume {},
    ] {
        let snapshot = controller.shared_snapshot();
        let (_, receipt) = controller
            .shared_command_with_receipt(SharedDesktopCommand {
                expected: snapshot.token,
                game: snapshot.game,
                operation,
            })
            .unwrap();
        settle(&mut controller);
        receipt.completion().unwrap().unwrap();
    }
    let path = output.join("shogi-captures.mshogi");
    let id = controller.queue_shared_save(path.clone()).unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let snapshot = controller.shared_snapshot();
        if snapshot.io.id == id {
            match snapshot.io.phase {
                IoPhase::Complete => break,
                IoPhase::Failed => panic!("fixture save failed: {:?}", snapshot.io.message),
                _ => {}
            }
        }
        assert!(Instant::now() < deadline, "fixture save timed out");
        std::thread::sleep(Duration::from_millis(1));
    }
    println!("{}", path.display());
}
