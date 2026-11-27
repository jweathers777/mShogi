//! Actual USI subprocess lifecycle, using accepted MSH-64/67/70 fixtures.
mod support;
use std::{process::Command, time::Duration};
use support::protocol::Engine;
fn engine() -> Engine {
    Engine::new(&["--game", "shogi", "usi"])
}
#[test]
fn handshake_options_readiness_defaults_alias_and_incompatible_selection() {
    for args in [&["--game", "shogi", "usi"][..], &["usi"][..]] {
        let mut e = Engine::new(args);
        e.send("usi");
        let lines = e.until("usiok");
        assert!(
            lines
                .iter()
                .any(|s| s == "option name Nodes type spin default 300000 min 0 max 5000000")
        );
        assert!(
            lines
                .iter()
                .all(|s| s.starts_with("id ") || s.starts_with("option ") || s == "usiok")
        );
        assert!(
            lines
                .iter()
                .any(|s| s == "option name Hash type spin default 16 min 1 max 1024")
        );
        e.send("setoption name Hash value 1");
        e.send("isready");
        assert_eq!(e.until("readyok"), vec!["readyok"]);
        e.send("usinewgame");
        e.send("position startpos");
        e.send("go");
        let result = e.until("bestmove");
        assert!(result.iter().any(|s| s.starts_with("info depth ")));
        assert_best(&result, &mshogi_core::shogi::Position::initial());
        assert_eq!(e.finish(true).0, 0);
    }
    let out = Command::new(env!("CARGO_BIN_EXE_mshogi"))
        .args(["--game", "chess", "usi"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
}
#[test]
fn supplied_history_changes_search_without_invented_pre_root_history() {
    let mut e = engine();
    let moves = ["4c3c", "5a4a", "3c4c", "4a5a"].repeat(3);
    let prefix = moves[..11].join(" ");
    e.send(&format!(
        "position sfen 4k4/9/5R3/9/9/9/9/9/K8 b - 1 moves {prefix}"
    ));
    e.send("go depth 1 nodes 5000");
    assert_eq!(e.until("bestmove").last().unwrap(), "bestmove 4a5a");
    e.send("position sfen 5k3/9/5R3/9/9/9/9/9/K8 w - 12");
    e.send("go depth 1 nodes 5000");
    assert_best(
        &e.until("bestmove"),
        &mshogi_core::shogi::Position::from_sfen("5k3/9/5R3/9/9/9/9/9/K8 w - 12").unwrap(),
    );
    assert_eq!(e.finish(true).0, 0);
}
#[test]
fn promotion_drop_declaration_and_pv_use_usi_notation() {
    let mut e = engine();
    for (root, expected) in [
        // Sente's King on its castled square: a corner King step is worth
        // more than a depth-one promotion under the tuned MSH-188 terms.
        ("8k/9/9/4P4/9/9/9/1K7/9 b - 1", "5d5c+"),
        ("8k/4P4/9/9/9/9/9/1K7/9 b - 1", "5b5a+"),
        ("3lkl3/3p1p3/4K4/9/9/9/9/9/9 b G 1", "G*5b"),
        ("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1", "win"),
    ] {
        e.send(&format!("position sfen {root}"));
        e.send("go depth 1 nodes 5000");
        let lines = e.until("bestmove");
        assert_eq!(lines.last().unwrap(), &format!("bestmove {expected}"));
        assert!(!lines.iter().any(|s| s.contains("score mate")));
        assert!(!lines.iter().any(|s| s.contains("pv win")));
    }
    e.send("position sfen K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k w B5P 1");
    e.send("go depth 1 nodes 5000");
    let lines = e.until("bestmove");
    assert_best(
        &lines,
        &mshogi_core::shogi::Position::from_sfen("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k w B5P 1")
            .unwrap(),
    );
    assert!(
        !lines
            .iter()
            .any(|s| s.contains("pv 1i1h win") || s.contains("score mate"))
    );
    assert_eq!(e.finish(true).0, 0);
}
#[test]
fn zero_or_insufficient_work_never_becomes_resignation_or_fallback() {
    for nodes in [0, 1] {
        let mut e = engine();
        e.send("position startpos");
        e.send(&format!("go depth 2 nodes {nodes}"));
        e.until("info string mshogi execution_failure");
        let (code, error, lines) = e.finish(false);
        assert_eq!(code, 1);
        assert!(error.contains("no completed iteration"));
        assert!(!lines.iter().any(|s| s.starts_with("bestmove")));
    }
    let mut e = engine();
    e.send("go depth 1 nodes 5000");
    let lines = e.until("bestmove");
    assert!(lines.iter().any(|s| s.starts_with("info depth 1 nodes ")));
    assert_best(&lines, &mshogi_core::shogi::Position::initial());
    assert_eq!(e.finish(true).0, 0);
}
#[test]
fn stop_keeps_completed_result_and_readiness_stays_responsive() {
    let mut e = engine();
    e.send("go infinite depth 1 nodes 5000");
    e.until("info string mshogi reported_score");
    assert!(e.lines.recv_timeout(Duration::from_millis(30)).is_err());
    e.send("isready");
    assert_eq!(e.until("readyok"), vec!["readyok"]);
    e.send("stop");
    assert_best(
        &e.until("bestmove"),
        &mshogi_core::shogi::Position::initial(),
    );
    assert_eq!(e.finish(true).0, 0);
    // An active stop may precede the first completed iteration. It must then
    // fail explicitly, rather than inventing a searched action. No timing-based
    // assumption about how much work this machine can finish is made.
    let mut e = engine();
    e.send("position sfen 8k/9/9/9/9/9/9/9/K8 b RBGSNLPrbgsnlp 1");
    e.send("go infinite depth 4 nodes 20000");
    e.send("isready");
    e.until("readyok");
    e.send("stop");
    let first = e.until("info string mshogi reported_score");
    let depth = first[0]
        .split_ascii_whitespace()
        .nth(2)
        .unwrap()
        .parse::<u16>()
        .unwrap();
    if depth == 0 {
        e.until("info string mshogi execution_failure");
        assert_eq!(e.finish(false).0, 1);
    } else {
        e.until("bestmove");
        assert_eq!(e.finish(true).0, 0);
    }
}
#[test]
fn replacement_newgame_gameover_and_quit_retire_old_observations() {
    let mut e = engine();
    e.send("go infinite depth 1 nodes 5000");
    e.until("info string mshogi reported_score");
    e.send("position startpos moves 7g7f");
    e.send("go depth 1 nodes 5000");
    assert_best(&e.until("bestmove"), &after_7g7f());
    e.send("go infinite depth 1 nodes 5000");
    e.until("info string mshogi reported_score");
    e.send("usinewgame");
    e.send("go depth 1 nodes 5000");
    assert_best(
        &e.until("bestmove"),
        &mshogi_core::shogi::Position::initial(),
    );
    e.send("go infinite depth 1 nodes 5000");
    e.until("info string mshogi reported_score");
    e.send("gameover draw");
    e.send("isready");
    assert_eq!(e.until("readyok"), vec!["readyok"]);
    e.send("position startpos");
    e.send("go infinite depth 4 nodes 20000");
    let (code, errors, lines) = e.finish(true);
    assert_eq!(code, 0, "{errors}");
    assert!(!lines.iter().any(|s| s.starts_with("bestmove")));
}
#[test]
fn malformed_positions_controls_and_options_are_atomic_and_explicit() {
    let mut e = engine();
    e.send("position startpos moves 7g7f");
    for bad in [
        "position startpos moves 7g7f bad",
        "setoption name RulesProfile value wrong",
        "setoption name Unknown value 1",
        // Gote is to move: a clock without Gote's time is incomplete.
        "go btime 1000 binc 100",
        "go ponder",
        "go depth 0",
        "go nodes 5000001",
        "go depth 1 depth 2",
    ] {
        e.send(bad);
    }
    e.send("isready");
    e.until("readyok");
    e.send("go depth 1 nodes 5000");
    assert_best(&e.until("bestmove"), &after_7g7f());
    let (code, errors, _) = e.finish(true);
    assert_eq!(code, 0);
    assert_eq!(errors.lines().count(), 8, "{errors}");
}
#[test]
fn terminal_handling_does_not_turn_draw_or_nonmate_into_resignation() {
    let mut e = engine();
    e.send("position sfen k6r1/9/9/9/9/9/9/r8/8K b - 321");
    e.send("go depth 1 nodes 5000");
    let lines = e.until("info string mshogi execution_failure");
    assert!(!lines.iter().any(|s| s.starts_with("bestmove")));
    assert!(lines.iter().any(|s| s.contains("terminal")));
    assert_eq!(e.finish(false).0, 1);
    let mut e = engine();
    e.send("position sfen 4k4/9/5R3/9/9/9/9/9/K8 b - 321");
    e.send("go depth 1 nodes 5000");
    let lines = e.until("info string mshogi execution_failure");
    assert!(lines.iter().any(|s| s.contains("Length")));
    assert!(!lines.iter().any(|s| s.starts_with("bestmove")));
    assert_eq!(e.finish(false).0, 1);
}

#[test]
fn checked_no_action_root_has_the_protocol_mandated_resignation() {
    let mut e = engine();
    e.send("position sfen k6rr/9/9/9/9/9/9/9/8K b - 322");
    e.send("go depth 1 nodes 5000");
    assert_eq!(e.until("bestmove").last().unwrap(), "bestmove resign");
    assert_eq!(e.finish(true).0, 0);
}
#[test]
fn same_context_request_replacement_and_failed_position_retire_held_results() {
    let mut e = engine();
    e.send("go infinite depth 1 nodes 5000");
    e.until("info string mshogi reported_score");
    e.send("go depth 1 nodes 5000");
    assert_best(
        &e.until("bestmove"),
        &mshogi_core::shogi::Position::initial(),
    );
    e.send("position startpos moves 7g7f");
    e.send("go infinite depth 1 nodes 5000");
    e.until("info string mshogi reported_score");
    e.send("position invalid");
    e.send("isready");
    assert_eq!(e.until("readyok"), vec!["readyok"]);
    e.send("go depth 1 nodes 5000");
    assert_best(&e.until("bestmove"), &after_7g7f());
    let (code, error, _) = e.finish(true);
    assert_eq!(code, 0);
    assert!(error.contains("position requires"));
}

#[test]
fn tsume_request_has_standard_notimplemented_reply_without_work() {
    let mut e = engine();
    e.send("go mate 100");
    assert_eq!(e.until("checkmate"), vec!["checkmate notimplemented"]);
    e.send("go depth 1 nodes 5000");
    assert_best(
        &e.until("bestmove"),
        &mshogi_core::shogi::Position::initial(),
    );
    assert_eq!(e.finish(true).0, 0);
}

fn assert_best(lines: &[String], position: &mshogi_core::shogi::Position) {
    let action = lines
        .last()
        .unwrap()
        .strip_prefix("bestmove ")
        .unwrap()
        .parse()
        .unwrap();
    assert!(position.legal_actions().contains(&action));
}
fn after_7g7f() -> mshogi_core::shogi::Position {
    let mut p = mshogi_core::shogi::Position::initial();
    p.apply("7g7f".parse().unwrap()).unwrap();
    p
}

#[test]
fn bounded_movetime_and_fixed_options_keep_moves_and_reproducible_probes() {
    let mut e = engine();
    e.send("position startpos");
    e.send("go movetime 250");
    assert_best(
        &e.until("bestmove"),
        &mshogi_core::shogi::Position::initial(),
    );
    e.send("setoption name Depth value 1");
    e.send("setoption name Nodes value 5000");
    e.send("go");
    let first = e.until("bestmove");
    assert_best(&first, &mshogi_core::shogi::Position::initial());
    e.send("go");
    let second = e.until("bestmove");
    assert_eq!(first.last(), second.last());
    let work = |lines: &[String]| {
        let words: Vec<_> = lines
            .iter()
            .find(|s| s.starts_with("info depth "))
            .unwrap()
            .split_whitespace()
            .collect();
        (
            words[2].parse::<u64>().unwrap(),
            words[4].parse::<u64>().unwrap(),
        )
    };
    assert_eq!(work(&first), work(&second));
    assert_eq!(work(&first).0, 1);
    assert_eq!(e.finish(true).0, 0);
}
#[test]
fn fischer_clock_go_returns_a_legal_move_within_the_allocation() {
    let mut e = engine();
    e.send("position startpos moves 7g7f");
    let started = std::time::Instant::now();
    e.send("go btime 3000 wtime 2000 binc 100 winc 100");
    assert_best(&e.until("bestmove"), &after_7g7f());
    // Gote's allocation from 2 s + 0.1 s is far below one second.
    assert!(started.elapsed() < std::time::Duration::from_millis(1500));
    assert_eq!(e.finish(true).0, 0);
}
