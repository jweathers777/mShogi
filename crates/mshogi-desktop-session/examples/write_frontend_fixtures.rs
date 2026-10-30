//! Native current-contract fixtures; the example's child mode invokes the real engine.
//! Neither this example nor its fixture initializer is bundled with the application.
use mshogi_core::{
    chess::*,
    native::{CanonicalRecord, NativeSide},
};
use mshogi_desktop_session::{
    AnalysisOperation, AnalysisPhase, DesktopController, IoPhase, SessionOperation,
    SharedDesktopCommand, SharedSnapshot,
};
use mshogi_engine::ReachabilityTarget;
fn main() {
    if std::env::args().nth(1).as_deref() == Some("internal-search") {
        mshogi_engine::serve_once(std::io::stdin().lock(), std::io::stdout().lock()).unwrap();
        return;
    }
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
        let mut c = chess_controller(fen);
        send(&mut c, SessionOperation::Resume {});
        write_snapshot(
            &directory,
            &format!("shared-chess-{name}"),
            &c.shared_snapshot(),
        );
    }
    let mut c = chess_controller(STANDARD_START_FEN);
    let unresolved = send(&mut c, SessionOperation::Resign {});
    write_snapshot(&directory, "shared-chess-unresolved", &unresolved);
    let mut c = chess_controller("7k/8/8/8/8/8/R7/K7 w - - 99 1");
    send(&mut c, SessionOperation::Resume {});
    let claim = c
        .shared_snapshot()
        .chess
        .unwrap()
        .claims
        .into_iter()
        .find(|v| v.intended.is_some())
        .unwrap();
    let claimed = send(&mut c, SessionOperation::ChessClaim { claim });
    write_snapshot(&directory, "shared-chess-claimed", &claimed);
    for (name, fen, operation) in [
        (
            "moves",
            STANDARD_START_FEN,
            AnalysisOperation::Moves {
                depth: 1,
                nodes: 1000,
            },
        ),
        (
            "mate",
            "7k/8/5KQ1/8/8/8/8/8 w - - 0 1",
            AnalysisOperation::Moves {
                depth: 1,
                nodes: 1000,
            },
        ),
        (
            "mated",
            "7k/6Q1/5K2/8/8/8/8/8 b - - 0 1",
            AnalysisOperation::Moves {
                depth: 1,
                nodes: 1000,
            },
        ),
        (
            "possible",
            "7k/6Q1/5K2/8/8/8/8/8 b - - 0 1",
            AnalysisOperation::FullReachability {
                target: ReachabilityTarget::Either,
                max_positions: 10,
                max_transitions: 10,
            },
        ),
        (
            "impossible",
            "7k/8/8/8/8/8/8/K7 w - - 0 1",
            AnalysisOperation::FullReachability {
                target: ReachabilityTarget::Either,
                max_positions: 10,
                max_transitions: 10,
            },
        ),
        (
            "unknown",
            STANDARD_START_FEN,
            AnalysisOperation::FullReachability {
                target: ReachabilityTarget::Either,
                max_positions: 0,
                max_transitions: 0,
            },
        ),
    ] {
        let mut c = chess_controller(fen);
        send(&mut c, SessionOperation::Analyze { operation });
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
        let snapshot = loop {
            let snapshot = c.shared_snapshot();
            let phase = snapshot.chess.as_ref().unwrap().analysis.status;
            if phase != AnalysisPhase::Thinking {
                break snapshot;
            }
            assert!(std::time::Instant::now() < deadline);
            std::thread::sleep(std::time::Duration::from_millis(1));
        };
        assert!(snapshot.chess.as_ref().unwrap().analysis.result.is_some());
        write_snapshot(
            &directory,
            &format!("shared-chess-analysis-{name}"),
            &snapshot,
        );
    }
    let mut c = chess_controller(STANDARD_START_FEN);
    send(
        &mut c,
        SessionOperation::Analyze {
            operation: AnalysisOperation::Moves {
                depth: 8,
                nodes: 5_000_000,
            },
        },
    );
    let cancelled = send(&mut c, SessionOperation::Cancel {});
    write_snapshot(&directory, "shared-chess-analysis-cancelled", &cancelled);
    let mut c = chess_controller(STANDARD_START_FEN);
    send(&mut c, SessionOperation::Resign {});
    let temporary =
        std::env::temp_dir().join(format!("mshogi-fixture-{}.mshogi", std::process::id()));
    let id = c.queue_shared_save(temporary.clone()).unwrap();
    wait_document(&mut c, id);
    let id = c
        .queue_product_open(temporary.clone(), c.shared_open_token())
        .unwrap();
    let mut reopened = wait_document(&mut c, id);
    reopened.document.path = Some("fixture.mshogi".into());
    write_snapshot(&directory, "shared-chess-reopened", &reopened);
    let mut history = send(&mut c, SessionOperation::Inspect { event_index: 1 });
    history.document.path = Some("fixture.mshogi".into());
    write_snapshot(&directory, "shared-chess-history", &history);
    std::fs::remove_file(temporary).unwrap();
    #[cfg(feature = "desktop-verification")]
    {
        write(
            &directory,
            "synthetic",
            &mshogi_desktop_session::SyntheticController::new().snapshot(),
        );
        let mut capacity =
            DesktopController::verification(std::env::current_exe().unwrap(), "capacity").unwrap();
        write_snapshot(
            &directory,
            "shared-chess-capacity",
            &capacity.shared_snapshot(),
        );
    }
}
// Only frontend fixture output receives a deterministic untimed schedule.
// The product/controller retains its real monotonic timestamps. Native replay
// checks that changing these fixture timestamps changes no accepted command,
// derived reply, canonical history or outcome; timed records are never normalized.
fn normalized_untimed_fixture_match(record: &MatchRecord) -> Result<MatchRecord, String> {
    if record.time_control() != TimeControl::Untimed {
        return Err("Only untimed frontend fixtures may normalize match timestamps.".into());
    }
    let original = record.replay().map_err(|error| error.to_string())?;
    let initial = record.initial_record().map_err(|error| error.to_string())?;
    let game = AdjudicatedGame::replay_record(&initial).map_err(|error| error.to_string())?;
    let mut normalized =
        ChessMatch::new(game, TimeControl::Untimed).map_err(|error| error.to_string())?;
    for (index, (request, expected)) in record.events().enumerate() {
        let mut request = request.clone();
        request.at_ns = u64::try_from(index).map_err(|_| "Fixture event index exceeds u64")?;
        if normalized.submit(request) != *expected {
            return Err(format!(
                "Untimed fixture normalization changed event {index}'s native reply."
            ));
        }
    }
    if normalized.game().record() != original.game().record()
        || normalized.conclusion() != original.conclusion()
        || normalized.active() != original.active()
        || normalized.remaining_ns() != original.remaining_ns()
    {
        return Err("Untimed fixture normalization changed native game semantics.".into());
    }
    Ok(normalized.record().clone())
}
fn write_snapshot(directory: &std::path::Path, name: &str, snapshot: &SharedSnapshot) {
    let mut fixture = snapshot.clone();
    if let Some(record) = &snapshot.chess_match {
        fixture.chess_match = Some(normalized_untimed_fixture_match(record).unwrap());
    }
    write(directory, name, &fixture);
}
fn write(directory: &std::path::Path, name: &str, value: &impl serde::Serialize) {
    std::fs::write(
        directory.join(format!("{name}.json")),
        serde_json::to_string_pretty(value).unwrap() + "\n",
    )
    .unwrap();
}
fn chess_controller(fen: &str) -> DesktopController {
    let mut c = DesktopController::new(std::env::current_exe().unwrap()).unwrap();
    let game = AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap();
    send(
        &mut c,
        SessionOperation::Replace {
            record: CanonicalRecord::chess(game.record()),
            human: NativeSide::Chess(Color::White),
        },
    );
    c
}
fn send(c: &mut DesktopController, operation: SessionOperation) -> SharedSnapshot {
    let snapshot = c.shared_snapshot();
    shared_command(
        c,
        SharedDesktopCommand {
            expected: snapshot.token,
            game: snapshot.game,
            operation,
        },
    )
    .unwrap()
}
fn wait_document(c: &mut DesktopController, id: u64) -> SharedSnapshot {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    loop {
        let s = c.shared_snapshot();
        if s.io.id == id && matches!(s.io.phase, IoPhase::Complete | IoPhase::Failed) {
            assert_eq!(s.io.phase, IoPhase::Complete, "{:?}", s.io);
            return s;
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

#[cfg(test)]
mod fixture_tests {
    use super::*;
    #[test]
    fn untimed_fixture_schedule_preserves_native_claim_resignation_and_complete_context() {
        for (fen, command) in [
            (
                "7k/8/8/8/8/8/R7/K7 w - - 99 1",
                MatchCommand::Claim {
                    reason: DrawClaim::FiftyMove,
                    intended: Some("a1b1".parse().unwrap()),
                },
            ),
            (STANDARD_START_FEN, MatchCommand::Resign { witness: None }),
        ] {
            let game = AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap();
            let mut original = ChessMatch::new(game, TimeControl::Untimed).unwrap();
            for (at_ns, command) in [(123_456, MatchCommand::Start), (987_654, command)] {
                let reply = original.submit(MatchRequest {
                    at_ns,
                    version: original.version(),
                    actor: Color::White,
                    command,
                });
                assert!(reply.rejection.is_none());
            }
            let normalized = normalized_untimed_fixture_match(original.record()).unwrap();
            assert_ne!(&normalized, original.record());
            assert_eq!(
                normalized.initial_record().unwrap(),
                original.record().initial_record().unwrap()
            );
            assert_eq!(
                normalized
                    .events()
                    .map(|(request, _)| request.at_ns)
                    .collect::<Vec<_>>(),
                [0, 1]
            );
            for ((actual, reply), (expected, original_reply)) in
                normalized.events().zip(original.record().events())
            {
                assert_eq!(actual.actor, expected.actor);
                assert_eq!(actual.version, expected.version);
                assert_eq!(actual.command, expected.command);
                assert_eq!(reply, original_reply);
            }
            let replayed = normalized.replay().unwrap();
            assert_eq!(replayed.game().record(), original.game().record());
            assert_eq!(replayed.conclusion(), original.conclusion());
            assert_eq!(
                normalized_untimed_fixture_match(&normalized).unwrap(),
                normalized
            );
        }
    }
    #[test]
    fn fixture_timestamp_normalization_rejects_timed_matches() {
        let game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        let original = ChessMatch::new(
            game,
            TimeControl::Increment {
                initial_ns: [1000, 1000],
                increment_ns: 5,
            },
        )
        .unwrap();
        assert!(normalized_untimed_fixture_match(original.record()).is_err());
    }
}
