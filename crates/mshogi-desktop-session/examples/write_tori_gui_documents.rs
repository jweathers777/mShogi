//! Rules-admitted current-envelope files for ordinary packaged GUI Load journeys.
//! Development example only: neither the generator nor documents are bundled.
use mshogi_core::{
    native::{CanonicalRecord, NativeSide},
    tori as t,
};
use mshogi_desktop_session::{DesktopController, IoPhase, SessionOperation, SharedDesktopCommand};
use serde_json::Value;
use std::{
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
#[path = "support/tori.rs"]
mod roots;
fn face(value: &str) -> t::Face {
    t::Face::ALL
        .into_iter()
        .find(|f| f.code() == value)
        .unwrap()
}
fn side(value: &str) -> t::Side {
    match value {
        "s" | "sente" => t::Side::Sente,
        "g" | "gote" => t::Side::Gote,
        _ => panic!("unknown fixture side"),
    }
}
fn position(root: &Value) -> t::Position {
    let board = root["board"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            let parts = entry.as_str().unwrap().split(':').collect::<Vec<_>>();
            (
                parts[2].parse().unwrap(),
                t::Piece {
                    side: side(parts[0]),
                    face: face(parts[1]),
                },
            )
        })
        .collect::<Vec<_>>();
    let hands = std::array::from_fn(|i| {
        t::Hand::from_counts(
            &root["hands"][if i == 0 { "s" } else { "g" }]
                .as_object()
                .unwrap()
                .iter()
                .map(|(f, n)| (face(f), u8::try_from(n.as_u64().unwrap()).unwrap()))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    });
    t::Position::from_diagram(
        &board,
        hands,
        side(root["side_to_move"].as_str().unwrap()),
        t::RootMetadata::diagram(root["declared_completed_plies"].as_u64().unwrap()),
    )
    .unwrap()
}
fn action(value: &Value) -> t::Action {
    match value["kind"].as_str().unwrap() {
        "move" => t::Action::Move {
            from: value["from"].as_str().unwrap().parse().unwrap(),
            to: value["to"].as_str().unwrap().parse().unwrap(),
            result_face: face(value["result_face"].as_str().unwrap()),
        },
        "drop" => t::Action::Drop {
            face: face(value["piece"].as_str().unwrap()),
            to: value["to"].as_str().unwrap().parse().unwrap(),
        },
        _ => panic!("unknown fixture action"),
    }
}
fn play(game: &mut t::Game, action: t::Action) {
    game.submit(t::GameRequest {
        expected: game.context(),
        actor: game.position().side_to_move(),
        command: t::GameCommand::Play { action },
    })
    .unwrap();
}
fn save(output: &Path, name: &str, game: &t::Game) {
    let mut controller =
        DesktopController::new("/unused/tori-gui-generator-engine".into()).unwrap();
    let snapshot = controller.shared_snapshot();
    let (_, receipt) = controller
        .shared_command_with_receipt(SharedDesktopCommand {
            expected: snapshot.token,
            game: snapshot.game,
            operation: SessionOperation::Replace {
                record: CanonicalRecord::tori(t::GameRecord::from_game(game).unwrap()),
                human: NativeSide::Tori(game.position().side_to_move()),
            },
        })
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        controller.shared_snapshot();
        if let Some(result) = receipt.completion() {
            result.unwrap();
            break;
        }
        assert!(
            Instant::now() < deadline,
            "native fixture admission timed out"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
    let path = output.join(format!("tori-{name}.mshogi"));
    let id = controller.queue_shared_save(path.clone()).unwrap();
    loop {
        let snapshot = controller.shared_snapshot();
        if snapshot.io.id == id {
            match snapshot.io.phase {
                IoPhase::Complete => break,
                IoPhase::Failed => panic!("native save: {:?}", snapshot.io.message),
                _ => {}
            }
        }
        assert!(Instant::now() < deadline, "native fixture save timed out");
        std::thread::sleep(Duration::from_millis(1));
    }
    println!("{}", path.display());
}
fn main() {
    let output = PathBuf::from(std::env::args_os().nth(1).expect("output directory"));
    std::fs::create_dir_all(&output).unwrap();
    let corpus: Value =
        serde_json::from_str(include_str!("../../../docs/m4/tori/fixtures.json")).unwrap();
    let given = |id: &str| {
        corpus["fixtures"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["id"] == id)
            .unwrap()["given"]
            .clone()
    };
    save(&output, "contract", &roots::contract_game());
    // Established TORI-O-MATE terminal diagram, with the previous Falcon4c
    // replacing Eagle4b. Mandatory Fa4c-4b=Ea reaches that exact reviewed board.
    let mut mate_root = given("TORI-O-MATE")["root"].clone();
    mate_root["side_to_move"] = "sente".into();
    for entry in mate_root["board"].as_array_mut().unwrap() {
        if entry == "s:Ea:4b" {
            *entry = "s:Fa:4c".into();
        }
    }
    let game = t::Game::from_root(position(&mate_root)).unwrap();
    assert!(game.conclusion().is_none());
    let mut completed = t::Game::from_root(position(&mate_root)).unwrap();
    play(
        &mut completed,
        t::Action::Move {
            from: "4c".parse().unwrap(),
            to: "4b".parse().unwrap(),
            result_face: t::Face::Eagle,
        },
    );
    assert_eq!(
        completed.conclusion(),
        Some(t::Conclusion::Checkmate {
            winner: t::Side::Sente
        })
    );
    for square in t::Square::all() {
        assert_eq!(
            completed.position().piece(square),
            position(&given("TORI-O-MATE")["root"]).piece(square)
        );
    }
    save(&output, "mate-in-one", &game);
    let quiet = t::Game::from_root(position(&given("TORI-O-QUIET")["root"])).unwrap();
    assert_eq!(
        quiet.conclusion(),
        Some(t::Conclusion::StalemateLoss {
            loser: t::Side::Gote
        })
    );
    save(&output, "quiet-loss", &quiet);
    for (id, name, prefix) in [
        ("TORI-H-FORCED-FULL", "forced-prefix7", 7),
        ("TORI-H-COUNT", "obligation8", 8),
    ] {
        let value = given(id);
        let mut game = t::Game::from_root(position(&value["root"])).unwrap();
        for value in value["actions"].as_array().unwrap().iter().take(prefix) {
            play(&mut game, action(value));
        }
        assert!(game.conclusion().is_none());
        if prefix == 8 {
            assert_eq!(game.occurrences(), [0, 4, 8]);
            assert!(game.obligation().is_some());
        }
        save(&output, name, &game);
    }
}
