//! Actual executable journeys, with facts from the independent MSH-64/67 corpus.
mod support;
use mshogi_core::{native::*, shogi::*};
use std::{fs, process::Output};
use support::cli::{Dir, stdout as text, success};
fn run(input: &str) -> Output {
    support::cli::run(&["--game", "shogi", "shell"], input)
}
fn record(out: &Output) -> String {
    text(out)
        .lines()
        .find_map(|l| l.strip_prefix("record "))
        .unwrap()
        .to_string()
}

#[test]
fn drop_capture_hands_optional_forced_and_already_promoted_choices() {
    // SH-F011, SH-F005, SH-F003 and SH-F010, independently established roots.
    let out = run(
        "position sfen reduced 8k/9/9/9/9/9/9/9/K8 b 2RBGSNLP 1\nmove P*5e\nhands\nundo\nposition sfen reduced 8k/9/9/4P4/9/9/9/9/K8 b - 1\nlegal\nmove 5d5c\nundo\nmove 5d5c+\nposition sfen reduced 8k/4P4/9/9/9/9/9/9/K8 b - 1\nlegal\nmove 5b5a+\nposition sfen reduced 8k/9/9/9/4+P4/9/9/9/K8 b - 1\nmove 5e5d\nposition sfen reduced 8k/9/4+p4/9/4R4/9/9/9/K8 b - 1\nmove 5e5c+\nhands\n",
    );
    success(&out);
    let s = text(&out);
    for expected in [
        "P*5e S:Pawn[hand->5e;drop]",
        "sfen 8k/9/9/9/4P4/9/9/9/K8 w 2RBGSNL 2",
        "5d5c S:Pawn[5d->5c;keep]",
        "5d5c+ S:Pawn[5d->5c;promote]",
        "sfen 8k/9/4P4/9/9/9/9/9/K8 w - 2",
        "sfen 8k/9/4+P4/9/9/9/9/9/K8 w - 2",
        "5b5a+ S:Pawn[5b->5a;promote]",
        "sfen 4+P3k/9/9/9/9/9/9/9/K8 w - 2",
        "5e5d S:+Pawn[5e->5d;keep]",
        "sfen 8k/9/9/4+P4/9/9/9/9/K8 w - 2",
        "sfen 8k/9/4+R4/9/9/9/9/9/K8 w P 2",
        "Pawn:1",
    ] {
        assert!(s.contains(expected), "missing {expected}\n{s}");
    }
    assert!(
        !s.lines().any(|l| l.starts_with("5b5a ")),
        "mandatory nonpromotion must not be listed"
    );
}
#[test]
fn undo_reverses_actions_of_earlier_move_commands() {
    let out = run("record print\nmove 7g7f\nmove 3c3d 2g2f\nundo 3\nrecord print\n");
    success(&out);
    let s = text(&out);
    let records: Vec<_> = s.lines().filter(|l| l.starts_with("record ")).collect();
    assert_eq!(records.len(), 2);
    assert_eq!(records[0], records[1]);
}
#[test]
fn same_piece_destinations_have_explicit_sources_and_illegal_batches_are_atomic() {
    // Independent manual geometry: two silvers each have five unobstructed
    // choices; Sente king 9i has 3. Both 4e/6e can move 5d without promotion.
    let out = run(
        "position sfen reduced 8k/9/9/9/3S1S3/9/9/9/K8 b - 1\nlegal\nmove 4e5d\nundo\nmove 6e5d\nposition startpos\nrecord print\nmove 7g7f 3c3d 7f7e 7e7d\nrecord print\nposition sfen reduced 8k/4P4/9/9/9/9/9/9/K8 b - 1\nrecord print\nmove 5b5a\nrecord print\n",
    );
    assert!(!out.status.success());
    let s = text(&out);
    for expected in [
        "legal-local 13",
        "4e5d S:Silver[4e->5d;keep]",
        "6e5d S:Silver[6e->5d;keep]",
        "sfen 8k/9/9/4S4/3S5/9/9/9/K8 w - 2",
        "sfen 8k/9/9/4S4/5S3/9/9/9/K8 w - 2",
    ] {
        assert!(s.contains(expected), "{expected}");
    }
    let records: Vec<_> = s.lines().filter(|l| l.starts_with("record ")).collect();
    assert_eq!(records[0], records[1]);
    assert_eq!(records[2], records[3]);
    let errors = String::from_utf8(out.stderr).unwrap();
    assert!(errors.contains("WrongActor") || errors.contains("WrongSide"));
    assert!(errors.contains("MissingPromotion"));
}
#[test]
fn declarations_queries_and_resignation_preserve_native_terminal_semantics() {
    let root = "K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1";
    let out = run(&format!(
        "position sfen reduced {root}\nrecord print\neligibility sente\nrecord print\ndeclare sente\nhistory\nundo\nposition startpos\ndeclare gote\nhistory\nundo\nmove 7g7f 3c3d\nresign sente\nhistory\nundo\nstatus\n"
    ));
    success(&out);
    let s = text(&out);
    let records: Vec<_> = s.lines().filter(|l| l.starts_with("record ")).collect();
    assert_eq!(records[0], records[1]);
    for expected in [
        "eligibility sente true",
        "camp_pieces: 10, points: 28",
        "known-board-actions 0",
        "reason: EnteringKing",
        "reason: FalseDeclaration",
        "winner: Sente",
        "known-events 3 conclusion Some(Win { winner: Gote, reason: Resignation })",
        "known-events 2 conclusion None",
    ] {
        assert!(s.contains(expected), "{expected}\n{s}");
    }
}
#[test]
fn actual_history_conclusions_are_not_reconstructed_from_sfen_counter() {
    let root = "4k4/9/5R3/9/9/9/9/9/K8 b - 1";
    for (cycle, expected) in [
        (
            "4c3c 5a4a 3c4c 4a5a",
            "Draw(Fourfold { first: 0, fourth: 12 })",
        ),
        (
            "4c5c 5a4a 5c4c 4a5a",
            "Win { winner: Gote, reason: ContinuousCheck { first: 0, fourth: 12 } }",
        ),
    ] {
        let out = run(&format!(
            "position sfen reduced {root}\nmove {cycle} {cycle} {cycle}\nhistory\nundo\nstatus\n"
        ));
        success(&out);
        let s = text(&out);
        assert!(s.contains(expected));
        assert!(s.contains("occurrences [0, 4, 8, 12]"));
        assert!(s.contains("known-events 11 conclusion None"));
    }
    let out = run("position sfen reduced 4k4/9/5R3/9/9/9/9/9/K8 b - 13\nhistory\n");
    let s = text(&out);
    assert!(s.contains("earlier-history unknown"));
    assert!(s.contains("declared-completed 12 known-events 0 conclusion None"));
    assert!(s.contains("occurrences [0]"));
}
#[test]
fn complete_canonical_save_load_replay_and_restored_undo() {
    let dir = Dir::new();
    let path = dir.0.join("complete record-é.json");
    let out = run(&format!(
        "position sfen reduced 8k/9/4+p4/9/4R4/9/9/9/K8 b - 17\nmove 5e5c+\nresign gote\nrecord print\nrecord save {}\nposition startpos\nrecord load-file {}\nrecord print\nhistory\nundo\nstatus\nundo\nstatus\n",
        path.display(),
        path.display()
    ));
    success(&out);
    let s = text(&out);
    let records: Vec<_> = s
        .lines()
        .filter_map(|l| l.strip_prefix("record "))
        .collect();
    assert_eq!(records.len(), 2);
    assert_eq!(records[0], records[1]);
    assert_eq!(fs::read_to_string(path).unwrap(), records[0]);
    assert!(s.contains("events 2 known-board-actions 1"));
    assert!(s.contains("counter Some(Supplied)"));
    assert!(s.contains("b - 17"));
    assert!(s.contains("sfen 8k/9/4+R4/9/9/9/9/9/K8 w P 18"));
    let CanonicalRecord::Shogi(native) = CanonicalRecord::from_json(records[0]).unwrap() else {
        panic!()
    };
    let game = native.to_game().unwrap();
    assert_eq!(game.events().len(), 2);
    assert_eq!(
        game.conclusion(),
        Some(Conclusion::Win {
            winner: Side::Sente,
            reason: WinReason::Resignation
        })
    );
}
#[test]
fn malformed_unsupported_and_tampered_loads_and_failed_saves_preserve_session() {
    let dir = Dir::new();
    let valid = CanonicalRecord::shogi(GameRecord::from_game(&Game::new()))
        .to_json()
        .unwrap();
    let path = dir.0.join("bad.json");
    for bytes in [
        b"{".to_vec(),
        valid
            .replace("mshogi.shogi.csa36.v1", "unknown.profile")
            .into_bytes(),
        valid
            .replace("\"schema_version\":2", "\"schema_version\":999")
            .into_bytes(),
        vec![255],
        CanonicalRecord::chess(
            mshogi_core::chess::AdjudicatedGame::new(mshogi_core::chess::ClaimPolicy::ExplicitOnly)
                .record(),
        )
        .to_json()
        .unwrap()
        .into_bytes(),
    ] {
        fs::write(&path, &bytes).unwrap();
        let out = run(&format!(
            "move 7g7f\nrecord print\nrecord load-file {}\nrecord print\nrecord save {}\nrecord print\n",
            path.display(),
            path.display()
        ));
        assert!(!out.status.success());
        let s = text(&out);
        let records: Vec<_> = s.lines().filter(|l| l.starts_with("record ")).collect();
        assert_eq!(records.len(), 3);
        assert!(records.iter().all(|l| *l == records[0]));
        assert_eq!(fs::read(&path).unwrap(), bytes);
    }
    let mut game = Game::new();
    game.submit(GameRequest {
        expected: game.context(),
        actor: Side::Sente,
        command: GameCommand::Play("7g7f".parse().unwrap()),
    })
    .unwrap();
    let valid = CanonicalRecord::shogi(GameRecord::from_game(&game))
        .to_json()
        .unwrap();
    let malformed = valid.replacen(
        "\"actor\":\"sente\"",
        "\"actor\":\"sente\",\"gave_check\":true",
        1,
    );
    assert_ne!(malformed, valid);
    fs::write(&path, malformed).unwrap();
    let out = run(&format!(
        "record print\nrecord load-file {}\nrecord print\n",
        path.display()
    ));
    assert!(!out.status.success());
    let s = text(&out);
    let records: Vec<_> = s.lines().filter(|l| l.starts_with("record ")).collect();
    assert_eq!(records[0], records[1]);
}
#[test]
fn large_pretty_record_file_loads() {
    let dir = Dir::new();
    let path = dir.0.join("long.json");
    let moves = ["4c3c", "5a4a", "3c4c", "4a5a"].repeat(3).join(" ");
    let out = run(&format!(
        "position sfen reduced 4k4/9/5R3/9/9/9/9/9/K8 b - 1\nmove {moves}\nrecord save {}\n",
        path.display()
    ));
    success(&out);
    // Pretty JSON is a valid full-record encoding, not a command-line payload.
    let value: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    let pretty = serde_json::to_string_pretty(&value).unwrap();
    // Legal JSON whitespace makes a large file without changing the native history.
    let pretty = format!("{pretty}{}", " ".repeat(4097));
    fs::write(&path, pretty).unwrap();
    let out = run(&format!(
        "record load-file {}\nhistory\nperft 3\nperft 0\ndivide 0\n",
        path.display()
    ));
    let s = text(&out);
    assert!(s.contains("loaded complete native record"));
    assert!(s.contains("Draw(Fourfold"));
    assert!(s.contains("perft-local 0 1"));
}
#[test]
fn standard_diagnostics_remain_position_only_and_preserve_state() {
    let out = run("record print\nperft 0\nperft 1\nperft 2\ndivide 1\nrecord print\n");
    success(&out);
    let s = text(&out);
    assert!(s.contains("perft-local 0 1"));
    assert!(s.contains("perft-local 1 30"));
    assert!(s.contains("perft-local 2 900"));
    assert!(s.contains("total-local 30"));
    let records: Vec<_> = s.lines().filter(|l| l.starts_with("record ")).collect();
    assert_eq!(records[0], records[1]);
    let encoded = record(&out);
    assert_eq!(
        CanonicalRecord::from_json(&encoded).unwrap().event_count(),
        0
    );
}
