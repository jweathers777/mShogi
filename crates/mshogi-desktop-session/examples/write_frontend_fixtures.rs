//! Regenerate native-shaped frontend test fixtures; never bundled with the app.
use mshogi_core::chess::*;
use mshogi_desktop_session::{
    AnalysisOperation, DesktopCommand, DesktopController, DesktopSession, IoPhase,
};
use mshogi_engine::{EngineResponse, ReachabilityTarget};
fn main() {
    let directory = std::path::PathBuf::from(
        std::env::args_os()
            .nth(1)
            .expect("usage: write_frontend_fixtures <output-directory>"),
    );
    std::fs::create_dir_all(&directory).unwrap();
    write_shared_fixtures(&directory);
    for (name, fen) in [
        ("standard", STANDARD_START_FEN),
        ("promotion", "7k/P7/8/8/8/8/8/7K w - - 0 1"),
        ("claim", "7k/8/8/8/8/8/R7/K7 w - - 99 1"),
        ("current-claim", "7k/8/8/8/8/8/R7/K7 w - - 100 1"),
        (
            "checkmate",
            "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3",
        ),
    ] {
        let game = AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap();
        let session = DesktopSession::from_record(&game.record(), Color::White).unwrap();
        std::fs::write(
            directory.join(format!("{name}.json")),
            serde_json::to_string_pretty(&session.snapshot()).unwrap() + "\n",
        )
        .unwrap();
    }
    let mut unresolved = DesktopSession::new();
    unresolved
        .command(DesktopCommand::Resign { version: 1 }, 0)
        .unwrap();
    let game =
        AdjudicatedGame::from_fen("7k/8/8/8/8/8/R7/K7 w - - 99 1", ClaimPolicy::ExplicitOnly)
            .unwrap();
    let mut claimed = DesktopSession::from_record(&game.record(), Color::White).unwrap();
    let claim = claimed
        .snapshot()
        .claims
        .into_iter()
        .find(|claim| claim.intended.is_some())
        .unwrap();
    claimed
        .command(
            DesktopCommand::Claim {
                version: claimed.snapshot().version,
                claim,
            },
            0,
        )
        .unwrap();
    for (name, session) in [("unresolved", unresolved), ("claimed", claimed)] {
        std::fs::write(
            directory.join(format!("{name}.json")),
            serde_json::to_string_pretty(&session.snapshot()).unwrap() + "\n",
        )
        .unwrap();
    }
    for (name, fen, operation) in [
        (
            "analysis-moves",
            STANDARD_START_FEN,
            AnalysisOperation::Moves {
                depth: 1,
                nodes: 1000,
            },
        ),
        (
            "analysis-mate",
            "7k/8/5KQ1/8/8/8/8/8 w - - 0 1",
            AnalysisOperation::Moves {
                depth: 1,
                nodes: 1000,
            },
        ),
        (
            "analysis-mated",
            "7k/6Q1/5K2/8/8/8/8/8 b - - 0 1",
            AnalysisOperation::Moves {
                depth: 1,
                nodes: 1000,
            },
        ),
        (
            "analysis-possible",
            "7k/6Q1/5K2/8/8/8/8/8 b - - 0 1",
            AnalysisOperation::FullReachability {
                target: ReachabilityTarget::Either,
                max_positions: 10,
                max_transitions: 10,
            },
        ),
        (
            "analysis-impossible",
            "7k/8/8/8/8/8/8/K7 w - - 0 1",
            AnalysisOperation::FullReachability {
                target: ReachabilityTarget::Either,
                max_positions: 10,
                max_transitions: 10,
            },
        ),
        (
            "analysis-unknown",
            STANDARD_START_FEN,
            AnalysisOperation::FullReachability {
                target: ReachabilityTarget::Either,
                max_positions: 0,
                max_transitions: 0,
            },
        ),
    ] {
        let game = AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap();
        let mut session = DesktopSession::from_record(&game.record(), Color::White).unwrap();
        session
            .command(
                DesktopCommand::Analyze {
                    version: session.snapshot().version,
                    operation,
                },
                0,
            )
            .unwrap();
        let request = session.analysis_request().unwrap();
        let mut input = serde_json::to_vec(&request).unwrap();
        input.push(b'\n');
        let mut output = Vec::new();
        mshogi_engine::serve_once(std::io::Cursor::new(input), &mut output).unwrap();
        let response: EngineResponse = serde_json::from_slice(&output).unwrap();
        assert!(session.accept_analysis(response));
        std::fs::write(
            directory.join(format!("{name}.json")),
            serde_json::to_string_pretty(&session.snapshot()).unwrap() + "\n",
        )
        .unwrap();
    }
    let mut cancelled = DesktopSession::new();
    cancelled
        .command(
            DesktopCommand::Analyze {
                version: 1,
                operation: AnalysisOperation::Moves {
                    depth: 8,
                    nodes: 5_000_000,
                },
            },
            0,
        )
        .unwrap();
    cancelled
        .command(
            DesktopCommand::CancelAnalysis {
                version: cancelled.snapshot().version,
            },
            0,
        )
        .unwrap();
    std::fs::write(
        directory.join("analysis-cancelled.json"),
        serde_json::to_string_pretty(&cancelled.snapshot()).unwrap() + "\n",
    )
    .unwrap();
    // Exercise the public file worker so restored/history fixtures are native,
    // including the same-ply distinction before an unresolved resignation.
    let temporary =
        std::env::temp_dir().join(format!("mshogi-fixture-{}.mshogi", std::process::id()));
    let mut controller =
        DesktopController::new(std::path::PathBuf::from("/unused/fixture-engine")).unwrap();
    let (_, receipt) = controller
        .command_with_receipt(DesktopCommand::Resign { version: 1 })
        .unwrap();
    while receipt.completion().is_none() {
        controller.snapshot();
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
    receipt.completion().unwrap().unwrap();
    let save = controller.queue_save(temporary.clone()).unwrap();
    wait_document(&mut controller, save);
    let token = controller.open_token();
    let open = controller.queue_open(temporary.clone(), token).unwrap();
    let mut reopened = wait_document(&mut controller, open);
    reopened.document.path = Some("fixture.mshogi".into());
    std::fs::write(
        directory.join("reopened.json"),
        serde_json::to_string_pretty(&reopened).unwrap() + "\n",
    )
    .unwrap();
    let pending = controller
        .command(DesktopCommand::Inspect {
            version: reopened.version,
            event_index: 1,
        })
        .unwrap();
    let mut history = wait_document(&mut controller, pending.io.id);
    history.document.path = Some("fixture.mshogi".into());
    std::fs::write(
        directory.join("history.json"),
        serde_json::to_string_pretty(&history).unwrap() + "\n",
    )
    .unwrap();
    std::fs::remove_file(temporary).unwrap();
    #[cfg(feature = "desktop-verification")]
    {
        let synthetic = mshogi_desktop_session::SyntheticController::new();
        std::fs::write(
            directory.join("synthetic.json"),
            serde_json::to_string_pretty(&synthetic.snapshot()).unwrap() + "\n",
        )
        .unwrap();
        let mut capacity = DesktopController::verification(
            std::path::PathBuf::from("/unused/fixture-engine"),
            "capacity",
        )
        .unwrap();
        std::fs::write(
            directory.join("capacity.json"),
            serde_json::to_string_pretty(&capacity.snapshot()).unwrap() + "\n",
        )
        .unwrap();
    }
}

fn wait_document(controller: &mut DesktopController, id: u64) -> mshogi_desktop_session::Snapshot {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    loop {
        let snapshot = controller.snapshot();
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
        assert!(std::time::Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

fn write_shared_fixtures(directory: &std::path::Path) {
    use mshogi_core::{
        native::{CanonicalRecord, GameKind, NativeSide},
        shogi as s, xiangqi as x,
    };
    use mshogi_desktop_session::{SessionOperation, SharedDesktopCommand};
    let mut controller = DesktopController::new("/unused/fixture-engine".into()).unwrap();
    for (name, sfen) in [
        (
            "shared-shogi-standard",
            "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1",
        ),
        ("shared-shogi-drops", "8k/9/9/9/9/9/9/9/K8 b 2RBGSNLP 1"),
        ("shared-shogi-promotion", "8k/9/9/4P4/9/9/9/9/K8 b - 1"),
        ("shared-shogi-forced", "8k/4P4/9/9/9/9/9/9/K8 b - 1"),
    ] {
        let game = if name == "shared-shogi-standard" {
            s::Game::new()
        } else {
            s::Game::from_root(
                s::GameRoot::import(
                    s::Position::from_sfen(sfen).unwrap(),
                    s::RootCategory::ReducedMaterialDiagram,
                    s::CounterSource::Supplied,
                )
                .unwrap(),
            )
        };
        let expected = controller.shared_token();
        let snapshot = shared_command(
            &mut controller,
            SharedDesktopCommand {
                expected,
                game: if name == "shared-shogi-standard" {
                    GameKind::Chess
                } else {
                    GameKind::Shogi
                },
                operation: SessionOperation::Replace {
                    record: CanonicalRecord::shogi(s::GameRecord::from_game(&game)),
                    human: NativeSide::Shogi(s::Side::Sente),
                },
            },
        )
        .unwrap();
        let snapshot = shared_command(
            &mut controller,
            SharedDesktopCommand {
                expected: snapshot.token,
                game: GameKind::Shogi,
                operation: SessionOperation::Resume {},
            },
        )
        .unwrap();
        std::fs::write(
            directory.join(format!("{name}.json")),
            serde_json::to_string_pretty(&snapshot).unwrap() + "\n",
        )
        .unwrap();
    }
    let game = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    let expected = controller.shared_token();
    let snapshot = shared_command(
        &mut controller,
        SharedDesktopCommand {
            expected,
            game: GameKind::Shogi,
            operation: SessionOperation::Replace {
                record: CanonicalRecord::xiangqi(x::GameRecord::from_game(&game).unwrap()),
                human: NativeSide::Xiangqi(x::Side::Red),
            },
        },
    )
    .unwrap();
    let snapshot = shared_command(
        &mut controller,
        SharedDesktopCommand {
            expected: snapshot.token,
            game: GameKind::Xiangqi,
            operation: SessionOperation::Resume {},
        },
    )
    .unwrap();
    std::fs::write(
        directory.join("shared-xiangqi-standard.json"),
        serde_json::to_string_pretty(&snapshot).unwrap() + "\n",
    )
    .unwrap();
    let inspection = controller
        .shared_inspection(mshogi_desktop_session::MovementInspectionQuery {
            expected: snapshot.token,
            origin: "b7".into(),
        })
        .unwrap();
    std::fs::write(
        directory.join("shared-xiangqi-inspection.json"),
        serde_json::to_string_pretty(&inspection).unwrap() + "\n",
    )
    .unwrap();
    #[cfg(feature = "desktop-verification")]
    for name in [
        "xiangqi-warning",
        "xiangqi-fourth",
        "xiangqi-count",
        "xiangqi-offer",
        "xiangqi-agreed",
        "xiangqi-resigned",
        "xiangqi-fast-proof",
    ] {
        let mut fixture =
            DesktopController::verification("/unused/fixture-engine".into(), name).unwrap();
        std::fs::write(
            directory.join(format!("shared-{name}.json")),
            serde_json::to_string_pretty(&fixture.shared_snapshot()).unwrap() + "\n",
        )
        .unwrap();
    }
    // Real native capture/recapture ledger, not fabricated frontend event labels.
    let mut game = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    for action in ["b2b9", "a9b9"] {
        game.submit(x::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: x::GameCommand::Play {
                action: action.parse().unwrap(),
            },
        })
        .unwrap();
    }
    let snapshot = shared_command(
        &mut controller,
        SharedDesktopCommand {
            expected: snapshot.token,
            game: GameKind::Xiangqi,
            operation: SessionOperation::Replace {
                record: CanonicalRecord::xiangqi(x::GameRecord::from_game(&game).unwrap()),
                human: NativeSide::Xiangqi(x::Side::Red),
            },
        },
    )
    .unwrap();
    std::fs::write(
        directory.join("shared-xiangqi-played.json"),
        serde_json::to_string_pretty(&snapshot).unwrap() + "\n",
    )
    .unwrap();
}

fn shared_command(
    controller: &mut DesktopController,
    command: mshogi_desktop_session::SharedDesktopCommand,
) -> Result<mshogi_desktop_session::SharedSnapshot, String> {
    let (_, receipt) = controller.shared_command_with_receipt(command)?;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    loop {
        let snapshot = controller.shared_snapshot();
        if let Some(result) = receipt.completion() {
            result?;
            return Ok(snapshot);
        }
        assert!(std::time::Instant::now() < deadline);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
