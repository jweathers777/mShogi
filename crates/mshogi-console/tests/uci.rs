//! Exercise the actual UCI child process, including simultaneous search/input.
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy};
use std::{
    io::{BufRead, BufReader, Read, Write},
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};

struct Engine {
    child: Child,
    input: ChildStdin,
    lines: Receiver<String>,
    stderr: Option<thread::JoinHandle<String>>,
}
impl Engine {
    fn new() -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_mshogi"))
            .arg("uci")
            .current_dir(std::env::temp_dir())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let input = child.stdin.take().unwrap();
        let output = child.stdout.take().unwrap();
        let mut error = child.stderr.take().unwrap();
        let (send, lines) = mpsc::channel();
        thread::spawn(move || {
            for line in BufReader::new(output).lines() {
                if send.send(line.unwrap()).is_err() {
                    break;
                }
            }
        });
        let stderr = Some(thread::spawn(move || {
            let mut text = String::new();
            error.read_to_string(&mut text).unwrap();
            text
        }));
        Self {
            child,
            input,
            lines,
            stderr,
        }
    }
    fn send(&mut self, text: &str) {
        self.input.write_all(text.as_bytes()).unwrap();
        self.input.flush().unwrap();
    }
    fn line(&self) -> String {
        let line = self
            .lines
            .recv_timeout(Duration::from_secs(15))
            .expect("UCI response deadline");
        assert!(
            line == "uciok"
                || line == "readyok"
                || ["id name ", "id author ", "option ", "info ", "bestmove "]
                    .iter()
                    .any(|prefix| line.starts_with(prefix)),
            "nonprotocol stdout: {line:?}"
        );
        line
    }
    fn best(&self) -> (String, Vec<String>) {
        let mut info = Vec::new();
        loop {
            let line = self.line();
            if let Some(action) = line.strip_prefix("bestmove ") {
                return (action.into(), info);
            }
            assert!(line.starts_with("info "), "unexpected response: {line}");
            info.push(line);
        }
    }
    fn finish(mut self) -> String {
        self.send("quit\n");
        let deadline = Instant::now() + Duration::from_secs(15);
        loop {
            if let Some(status) = self.child.try_wait().unwrap() {
                assert!(status.success());
                break;
            }
            assert!(Instant::now() < deadline, "quit did not terminate process");
            thread::sleep(Duration::from_millis(1));
        }
        self.stderr.take().unwrap().join().unwrap()
    }
}
impl Drop for Engine {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
fn legal(fen: &str, text: &str) {
    let game = AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap();
    assert!(
        game.position()
            .legal_actions()
            .contains(&text.parse().unwrap()),
        "illegal bestmove {text}"
    );
}
const START: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

#[test]
fn handshake_readiness_and_live_stop_keep_stdout_protocol_only() {
    let mut engine = Engine::new();
    engine.send("  ignored-token  uci\r\n");
    assert!(engine.line().starts_with("id name mShogi "));
    assert_eq!(engine.line(), "id author John Weathers");
    assert_eq!(
        engine.line(),
        "option name ClaimPolicy type combo default explicit var explicit var auto"
    );
    assert_eq!(engine.line(), "uciok");
    engine.send("ucinewgame\nisready\nposition startpos\ngo infinite\nisready\n");
    assert_eq!(engine.line(), "readyok");
    assert_eq!(
        engine.line(),
        "readyok",
        "readiness must answer while thinking"
    );
    assert!(
        engine
            .lines
            .recv_timeout(Duration::from_millis(20))
            .is_err(),
        "infinite must await stop"
    );
    engine.send("stop\n");
    let (best, info) = engine.best();
    legal(START, &best);
    assert_eq!(info.len(), 1, "one final bounded info line");
    let errors = engine.finish();
    assert!(errors.is_empty() || errors.contains("no completed iteration"));
}
#[test]
fn deterministic_depth_and_node_limits_return_legal_bestmoves() {
    let mut engine = Engine::new();
    engine.send("position fen 7k/8/5KQ1/8/8/8/8/8 w - - 0 1\ngo depth 1\n");
    let (best, info) = engine.best();
    assert_eq!(best, "g6g7");
    assert!(info[0].contains("score mate 1"));
    engine.send("position startpos\ngo nodes 0\n");
    let (best, info) = engine.best();
    assert_eq!(best, "a2a3", "zero-work deterministic legal fallback");
    assert!(info[0].contains("depth 0 nodes 0"));
    engine.send("go depth 1 nodes 21\n");
    let (best, info) = engine.best();
    legal(START, &best);
    assert!(info[0].contains("depth 1 nodes 21"));
    assert!(engine.finish().contains("no completed iteration"));
}
#[test]
fn position_replays_history_and_invalid_sequence_is_atomic() {
    let mut engine = Engine::new();
    engine.send("position startpos moves e2e4\nisready\n");
    assert_eq!(engine.line(), "readyok");
    engine.send("position startpos moves d2d4 e7e4\ngo depth 1\n");
    let (best, _) = engine.best();
    let mut game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    game.apply("e2e4".parse().unwrap()).unwrap();
    assert!(
        game.position()
            .legal_actions()
            .contains(&best.parse().unwrap())
    );
    let action: mshogi_core::chess::Action = best.parse().unwrap();
    assert_eq!(
        game.position().piece_at(action.from).unwrap().color,
        mshogi_core::chess::Color::Black
    );
    assert!(engine.finish().contains("move 2:"));
}
#[test]
fn optional_claim_channel_limitation_and_recorded_auto_policy_are_explicit() {
    let mut engine = Engine::new();
    let fen = "7k/8/5KQ1/8/8/8/8/8 w - - 100 1";
    engine.send(&format!("position fen {fen}\ngo depth 1\n"));
    assert_eq!(
        engine.best().0,
        "g6g7",
        "explicit policy may prefer mate over claim"
    );
    engine.send("setoption name ClaimPolicy value auto\n");
    engine.send(&format!("position fen {fen}\ngo depth 1\n"));
    let (fallback, info) = engine.best();
    legal(fen, &fallback);
    assert!(info[0].contains("score cp 0"));
    engine.send("setoption name ClaimPolicy value explicit\nposition fen 7k/8/8/8/8/8/8/KR6 b - - 98 1\ngo depth 1\n");
    assert!(engine.best().1[0].contains("score cp -500"));
    engine.send("setoption name ClaimPolicy value auto\nposition fen 7k/8/8/8/8/8/8/KR6 b - - 98 1\ngo depth 1\n");
    assert!(
        engine.best().1[0].contains("score cp 0"),
        "auto policy must reach future search nodes"
    );
    assert!(engine.finish().contains("UCI has no claim channel"));
}
#[test]
fn same_fen_with_different_known_repetition_history_changes_search() {
    let mut engine = Engine::new();
    engine.send("position fen 6nk/8/8/8/8/8/8/KR4N1 b - - 0 1 moves g8f6 g1f3 f6g8 f3g1 g8f6 g1f3 f6g8 f3g1\ngo depth 1\n");
    assert!(engine.best().1[0].contains("score cp 0"));
    engine.send("position fen 6nk/8/8/8/8/8/8/KR4N1 b - - 8 5\ngo depth 1\n");
    assert!(engine.best().1[0].contains("score cp -500"));
    assert!(engine.finish().contains("draw claim available"));
}
#[test]
fn movetime_and_increment_clock_budgets_stop_live_search() {
    let mut engine = Engine::new();
    for go in [
        "go movetime 15",
        "go wtime 100 btime 100 winc 2 binc 2 movestogo 10",
    ] {
        engine.send(&format!("position startpos\n{go}\n"));
        let (best, info) = engine.best();
        legal(START, &best);
        assert!(info[0].contains("time "));
    }
    engine.send("position startpos moves e2e4\ngo wtime 1000 btime 0 winc 10 binc 0\n");
    let (_, info) = engine.best();
    assert!(
        info[0].contains("depth 0 nodes 0"),
        "zero own clock stops before first node"
    );
    let errors = engine.finish();
    assert!(errors.is_empty() || errors.contains("no completed iteration"));
}
#[test]
fn infinite_terminal_waits_for_stop_and_new_position_suppresses_stale_output() {
    let mut engine = Engine::new();
    engine.send("position fen 7k/5K2/6Q1/8/8/8/8/8 b - - 0 1\ngo infinite\nisready\n");
    assert_eq!(engine.line(), "readyok");
    assert!(
        engine
            .lines
            .recv_timeout(Duration::from_millis(20))
            .is_err()
    );
    engine.send("position startpos\nisready\n");
    assert_eq!(engine.line(), "readyok");
    assert!(
        engine
            .lines
            .recv_timeout(Duration::from_millis(20))
            .is_err(),
        "no bestmove from superseded infinite search"
    );
    engine.send("position fen 7k/5K2/6Q1/8/8/8/8/8 b - - 0 1\ngo infinite\nstop\n");
    assert_eq!(engine.best().0, "0000");
    let errors = engine.finish();
    assert!(errors.is_empty() || errors.contains("no completed iteration"));
}
#[test]
fn invalid_input_and_unsupported_controls_recover_without_stdout_diagnostics() {
    let mut engine = Engine::new();
    engine.send(
        "go ponder\ngo depth -1\ngo searchmoves e2e4\ngo winc 100\nsetoption name Hash value 16\n",
    );
    engine.input.write_all(&[0xff, b'\n']).unwrap();
    engine.send(&format!("{}\nisready\n", "x".repeat(5000)));
    assert_eq!(engine.line(), "readyok");
    engine.send("position startpos\ngo depth 1\n");
    legal(START, &engine.best().0);
    let errors = engine.finish();
    assert!(errors.contains("unsupported go control"));
    assert!(errors.contains("unsigned decimal"));
    assert!(errors.contains("unsupported option"));
    assert!(errors.contains("moving side"));
    assert!(errors.contains("ASCII"));
    assert!(errors.contains("line exceeds"));
}
#[test]
fn quit_terminates_during_unbounded_search_without_waiting_for_join() {
    let mut engine = Engine::new();
    engine.send("position startpos\ngo infinite\nisready\n");
    assert_eq!(engine.line(), "readyok");
    let errors = engine.finish();
    assert!(errors.is_empty() || errors.contains("no completed iteration"));
}

fn assert_representation_failure(go: &str) {
    let mut engine = Engine::new();
    // White has representable legal root actions, but any Black reply would
    // overflow the fullmove counter. Do not disguise failed depth-two search
    // as a successfully selected root move or an alleged zero-node search.
    engine.send(&format!(
        "position fen 7k/8/8/8/8/8/8/KR6 w - - 0 4294967295\n{go}\n"
    ));
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        if let Some(status) = engine.child.try_wait().unwrap() {
            assert!(!status.success(), "search failure must fail the process");
            break;
        }
        assert!(Instant::now() < deadline, "failed search did not terminate");
        thread::sleep(Duration::from_millis(1));
    }
    let stdout: Vec<_> = engine.lines.iter().collect();
    assert!(
        stdout.is_empty(),
        "no invented info or bestmove: {stdout:?}"
    );
    let errors = engine.stderr.take().unwrap().join().unwrap();
    assert!(errors.contains("search request"), "{errors}");
    assert!(errors.contains("CounterOverflow"), "{errors}");
    assert!(!errors.contains("returning legal fallback"), "{errors}");
}

#[test]
fn descendant_search_representation_failure_exits_without_fabricated_result() {
    assert_representation_failure("go depth 2");
}
#[test]
fn infinite_search_failure_exits_immediately_without_waiting_for_stop() {
    assert_representation_failure("go infinite");
}
