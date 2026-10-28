//! Actual UCI0-9 processes; native MSH-94/97/105 expectations remain independent.
use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};
struct Engine {
    child: Child,
    input: Option<ChildStdin>,
    lines: Receiver<String>,
    errors: thread::JoinHandle<String>,
}
impl Engine {
    fn new(args: &[&str]) -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_mshogi"))
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let input = child.stdin.take();
        let output = child.stdout.take().unwrap();
        let error = child.stderr.take().unwrap();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            for line in BufReader::new(output).lines() {
                if tx.send(line.unwrap()).is_err() {
                    break;
                }
            }
        });
        let errors = thread::spawn(move || {
            use std::io::Read;
            let mut text = String::new();
            BufReader::new(error).read_to_string(&mut text).unwrap();
            text
        });
        Self {
            child,
            input,
            lines: rx,
            errors,
        }
    }
    fn send(&mut self, text: &str) {
        let i = self.input.as_mut().unwrap();
        writeln!(i, "{text}").unwrap();
        i.flush().unwrap();
    }
    fn until(&self, prefix: &str) -> Vec<String> {
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut lines = vec![];
        loop {
            let line = self
                .lines
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
                .unwrap_or_else(|e| panic!("await {prefix}: {e}, lines{lines:?}"));
            let done = line.starts_with(prefix);
            lines.push(line);
            if done {
                return lines;
            }
        }
    }
    fn finish(mut self, quit: bool) -> (i32, String, Vec<String>) {
        if quit {
            self.send("quit");
        }
        self.input.take();
        let deadline = Instant::now() + Duration::from_secs(3);
        let code = loop {
            if let Some(status) = self.child.try_wait().unwrap() {
                break status.code().unwrap_or(-1);
            }
            if Instant::now() >= deadline {
                self.child.kill().unwrap();
                panic!("Xiangqi UCI failed to terminate promptly")
            }
            thread::sleep(Duration::from_millis(2));
        };
        let errors = self.errors.join().unwrap();
        let lines = self.lines.into_iter().collect();
        (code, errors, lines)
    }
}
fn engine() -> Engine {
    Engine::new(&["--game", "xiangqi", "uci"])
}
const ROOT: &str = "4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1";
const TACTIC: &str = "5k3/9/4R4/9/9/9/9/9/9/3K5 w - - 0 1";
#[test]
fn real_handshake_explicit_selection_and_measured_limits() {
    let mut e = engine();
    e.send("uci");
    let lines = e.until("uciok");
    assert!(
        lines
            .iter()
            .any(|s| s == "option name Depth type spin default 1 min 1 max 3")
    );
    assert!(
        lines
            .iter()
            .any(|s| s == "option name Nodes type spin default 128 min 0 max 512")
    );
    assert!(
        lines
            .iter()
            .any(|s| s.contains("mshogi.xiangqi.wxf2018.v1"))
    );
    e.send("isready");
    assert_eq!(e.until("readyok"), vec!["readyok"]);
    e.send("ucinewgame");
    e.send("position startpos");
    e.send("go");
    let lines = e.until("bestmove");
    let token = lines.last().unwrap().strip_prefix("bestmove ").unwrap();
    let action = token.parse::<mshogi_core::xiangqi::Action>().unwrap();
    assert!(
        mshogi_core::xiangqi::Position::initial()
            .legal_actions()
            .unwrap()
            .contains(&action)
    );
    assert!(lines.iter().any(|s| s.contains("mode sound_fast_only")));
    assert!(
        lines
            .iter()
            .all(|s| !s.contains("score cp") && !s.contains("score mate"))
    );
    let (code, err, _) = e.finish(true);
    assert_eq!(code, 0, "{err}");
}
#[test]
fn independent_stalemate_tactic_and_atomic_invalid_position_for_both_sides() {
    for (fen, action) in [
        (TACTIC, "e7e8"),
        ("3k5/9/9/9/9/9/9/4r4/9/5K3 b - - 0 1", "e2e1"),
    ] {
        let mut e = engine();
        e.send(&format!("position fen {fen}"));
        e.send("position fen invalid");
        e.send("go depth 1 nodes 128");
        assert_eq!(
            e.until("bestmove").last().unwrap(),
            &format!("bestmove {action}")
        );
        let (code, err, _) = e.finish(true);
        assert_eq!(code, 0);
        assert!(err.contains("six-field"), "{err}");
    }
}
#[test]
fn same_board_history_warning_changes_real_engine_choice() {
    let moves = ["e9f9", "f8e8", "f9e9", "e8f8"].repeat(3);
    let prefix = &moves[..11];
    let mut game = mshogi_core::xiangqi::Game::from_root(
        mshogi_core::xiangqi::PositionRoot::from_fen(ROOT).unwrap(),
        mshogi_core::xiangqi::ClaimPolicy::AutoValidClaims,
    )
    .unwrap();
    for text in prefix {
        let action = text.parse().unwrap();
        game.submit(mshogi_core::xiangqi::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: mshogi_core::xiangqi::GameCommand::Play { action },
        })
        .unwrap();
    }
    for (position, expected) in [
        (
            format!("position fen {ROOT} moves {}", prefix.join(" ")),
            "e8f8",
        ),
        (
            format!("position fen {}", game.position().to_fen().unwrap()),
            "e8e9",
        ),
    ] {
        let mut e = engine();
        e.send(&position);
        e.send("go depth 1 nodes 128");
        assert_eq!(
            e.until("bestmove").last().unwrap(),
            &format!("bestmove {expected}")
        );
        assert_eq!(e.finish(true).0, 0);
    }
}
#[test]
fn infinite_is_still_bounded_and_stop_publishes_only_completed_work() {
    let mut e = engine();
    e.send(&format!("position fen {TACTIC}"));
    e.send("go infinite depth 1 nodes 128");
    let actual = e.until("info string mshogi native_score");
    assert!(actual.iter().all(|s| !s.starts_with("bestmove")));
    e.send("isready");
    assert_eq!(e.until("readyok"), vec!["readyok"]);
    e.send("stop");
    assert_eq!(e.until("bestmove"), vec!["bestmove e7e8"]);
    assert_eq!(e.finish(true).0, 0);
}
#[test]
fn position_replacement_retires_held_result_and_quit_cancels_work() {
    let mut e = engine();
    e.send(&format!("position fen {TACTIC}"));
    e.send("go infinite");
    e.until("info string mshogi native_score");
    e.send("position fen 3k5/9/9/9/9/9/9/4r4/9/5K3 b - - 0 1");
    e.send("go");
    let lines = e.until("bestmove");
    assert_eq!(lines.last().unwrap(), "bestmove e2e1");
    assert!(!lines.iter().any(|s| s == "bestmove e7e8"));
    assert_eq!(e.finish(true).0, 0);
    let mut e = engine();
    e.send("go infinite depth 3 nodes 512");
    assert_eq!(e.finish(true).0, 0);
}
#[test]
fn zero_work_and_actual_terminal_never_invent_a_bestmove_or_score() {
    for position in [None, Some("position fen 4k4/9/9/9/9/9/9/9/9/3K5 w - - 0 1")] {
        let mut e = engine();
        if let Some(p) = position {
            e.send(p)
        };
        e.send("go depth 1 nodes 0");
        let lines = e.until("info string mshogi execution_failure");
        assert!(lines.iter().all(|s| !s.starts_with("bestmove")));
        let (code, err, _) = e.finish(false);
        assert_eq!(code, 1);
        assert!(
            err.contains("incomplete:") || err.contains("native_terminal:"),
            "{err}"
        );
    }
}
#[test]
fn unsupported_controls_options_and_oversized_input_are_explicit_and_drained() {
    let mut e = engine();
    e.send("setoption name RulesProfile value other");
    e.send("setoption name Nodes value 513");
    e.send("go movetime 1");
    e.send("go depth 1 depth 2");
    e.send(&"x".repeat(4097));
    e.send("isready");
    assert_eq!(e.until("readyok"), vec!["readyok"]);
    let (code, err, _) = e.finish(true);
    assert_eq!(code, 0);
    for expected in [
        "immutable",
        "depth1..3/nodes0..512",
        "unsupported go control",
        "duplicate go control",
        "line exceeds 4096",
    ] {
        assert!(err.contains(expected), "missing {expected}: {err}");
    }
}
