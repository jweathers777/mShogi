//! Real human console boundary. Expected routes follow independently established
//! central-Lion capture/return geometry from the accepted Chu local fixtures.
mod support;
use mshogi_core::{chu as c, game::CanonicalRecord};
use std::{fs, process::Output};
use support::cli::{Dir, records, run, stdout as output, success};
fn shell(text: &str) -> Output {
    run(&["--game", "chu", "shell"], text)
}
fn root() -> c::Game {
    c::Game::from_root(
        c::Position::from_diagram(
            &[
                ("12l", c::Side::Sente, "K"),
                ("1a", c::Side::Gote, "K"),
                ("6f", c::Side::Sente, "Ln"),
                ("6e", c::Side::Gote, "Si"),
                ("6d", c::Side::Gote, "R"),
                ("3f", c::Side::Sente, "DE+"),
                ("2j", c::Side::Gote, "FK"),
            ]
            .map(|(s, side, p)| (s.parse().unwrap(), c::Piece::from_code(side, p).unwrap())),
            c::Side::Sente,
            c::RootMetadata::diagram(31),
        )
        .unwrap(),
    )
    .unwrap()
}
fn encode(g: &c::Game) -> String {
    CanonicalRecord::chu(c::GameRecord::from_game(g).unwrap())
        .to_json()
        .unwrap()
}
#[test]
fn profile_help_protocol_and_normal_start_are_explicit() {
    let p = run(&["--game", "chu", "profile"], "");
    success(&p);
    assert!(output(&p).contains(c::PROFILE_ID));
    assert!(output(&p).contains(c::PROFILE_EDITION));
    let o = shell("help\nshow\nlegal\nperft 1\nanalyze 1 512\nrecord print\n");
    success(&o);
    let text = output(&o);
    for s in [
        "not USI/SFEN",
        "Ln6fx6ex6d",
        "sLn",
        "gFK",
        "royals sente K@7l",
        "royals gote K@6a",
        "analysis-read-only",
        "perft-local 1 leaves 36",
        "known-board-actions 0",
    ] {
        assert!(text.contains(s), "{s}\n{text}");
    }
    assert!(!text.contains("s:Ln"));
    assert!(!text.contains("g:FK"));
    for protocol in ["uci", "usi", "internal-search"] {
        assert_eq!(run(&["--game", "chu", protocol], "").status.code(), Some(2));
    }
}
#[test]
fn ordered_captures_returns_royals_history_and_undo_are_lossless() {
    let o = shell(&format!(
        "record load-json {}\nrecord print\nshow\nlegal\nmove Ln6fx6ex6d\nhistory\nundo\nrecord print\nmove Ln6fx6e-6f\nhistory\nundo\nmove Ln6f-5e-6f\nhistory\n",
        encode(&root())
    ));
    success(&o);
    assert_eq!(records(&o)[0], records(&o)[1]);
    let text = output(&o);
    for s in [
        "royals sente Pr@3f K@12l",
        "declared-root-plies 31",
        "action Ln6fx6ex6d",
        "action Ln6fx6e-6f",
        "action Ln6f-5e-6f",
        "known-board-actions 1",
    ] {
        assert!(text.contains(s), "{s}\n{text}");
    }
}
#[test]
fn malformed_batch_and_resource_diagnostics_preserve_canonical_session() {
    let o = shell(&format!(
        "record load-json {}\nrecord print\nmove Ln6fx6d Ln6d-6e\nrecord print\nmove Ln6f-6d\nrecord print\nperft 2 1\nrecord print\nrecord load-json {{\"game\":\"chu\",\"game\":\"chess\"}}\nrecord print\n",
        encode(&root())
    ));
    assert!(!o.status.success());
    let r = records(&o);
    assert_eq!(r.len(), 5);
    for x in &r[1..] {
        assert_eq!(&r[0], x);
    }
    let error = String::from_utf8_lossy(&o.stderr);
    assert!(error.contains("no actions applied"));
    assert!(error.contains("incomplete, unscored"));
}
#[test]
fn actual_offers_and_nonmove_commands_preserve_turn_and_atomic_failure() {
    let o = shell(&format!(
        "record load-json {}\nrecord print\noffer-draw gote\nhistory\ndecline-draw sente\nundo 2\nrecord print\noffer-draw sente\naccept-draw gote\nhistory\nundo 2\nresign gote\nhistory\nundo 1\nrecord print\n",
        encode(&root())
    ));
    success(&o);
    let r = records(&o);
    assert_eq!(r.len(), 3);
    assert_eq!(r[0], r[1]);
    assert_eq!(r[0], r[2]);
    let text = output(&o);
    for s in [
        "AgreedDraw",
        "Resignation { winner: Sente }",
        "board-ply 0 actor gote action OfferDraw",
    ] {
        assert!(text.contains(s), "{s}\n{text}");
    }
}

#[test]
fn atomic_canonical_save_and_strict_load_preserve_existing_files_and_live_game() {
    let dir = Dir::new();
    let path = dir.0.join("complete chu.json");
    let o = shell(&format!(
        "record load-json {}\nmove Ln6fx6e-6f\nrecord print\nrecord save {}\nrecord save {}\nposition startpos\nrecord load-file {}\nrecord print\n",
        encode(&root()),
        path.display(),
        path.display(),
        path.display()
    ));
    assert!(!o.status.success());
    assert!(String::from_utf8_lossy(&o.stderr).contains("existing files are preserved"));
    let r = records(&o);
    assert_eq!(r.len(), 2);
    assert_eq!(r[0], r[1]);
    assert_eq!(fs::read_to_string(&path).unwrap(), r[0]);
    let parsed = CanonicalRecord::from_json(&r[0]).unwrap();
    let CanonicalRecord::Chu(record) = parsed else {
        panic!()
    };
    assert_eq!(record.to_game().unwrap().snapshot().board_ply, 1);
}
#[test]
fn promotion_and_terminal_action_remain_not_display_inference() {
    let p = c::Position::from_diagram(
        &[
            ("12l", c::Side::Sente, "K"),
            ("1a", c::Side::Gote, "K"),
            ("6e", c::Side::Sente, "P"),
            ("4h", c::Side::Gote, "FK"),
        ]
        .map(|(s, side, p)| (s.parse().unwrap(), c::Piece::from_code(side, p).unwrap())),
        c::Side::Sente,
        c::RootMetadata::diagram(0),
    )
    .unwrap();
    let g = c::Game::from_root(p).unwrap();
    let o = shell(&format!(
        "record load-json {}\nmove P6e-6d^\nhistory\nshow\nundo\nmove P6e-6d\nhistory\n",
        encode(&g)
    ));
    success(&o);
    assert!(output(&o).contains("sP+"));
    assert!(output(&o).contains("action P6e-6d^"));
    assert!(output(&o).contains("action P6e-6d occurrences"));
    let p = c::Position::from_diagram(
        &[
            ("12l", c::Side::Sente, "K"),
            ("6f", c::Side::Sente, "Ln"),
            ("6e", c::Side::Gote, "K"),
            ("2j", c::Side::Gote, "FK"),
        ]
        .map(|(s, side, p)| (s.parse().unwrap(), c::Piece::from_code(side, p).unwrap())),
        c::Side::Sente,
        c::RootMetadata::diagram(0),
    )
    .unwrap();
    let o = shell(&format!(
        "record load-json {}\nmove Ln6fx6e\nstatus\nundo\nrecord print\n",
        encode(&c::Game::from_root(p).unwrap())
    ));
    success(&o);
    assert!(output(&o).contains("RoyalCapture { winner: Sente }"));
}

#[test]
fn accepted_repetition_fixture_keeps_obligation_admission_and_refusal_atomic() {
    // CHU-REP-QUIET independent expectation: 10 actions accepted, responsible
    // Sente must vary before move 11; the fourth occurrence is never committed.
    let p = c::Position::from_diagram(
        &[
            ("1l", c::Side::Sente, "K"),
            ("3l", c::Side::Sente, "G"),
            ("12a", c::Side::Gote, "K"),
            ("10a", c::Side::Gote, "G"),
        ]
        .map(|(s, side, p)| (s.parse().unwrap(), c::Piece::from_code(side, p).unwrap())),
        c::Side::Sente,
        c::RootMetadata::imported("CHU-REP-QUIET independent fixture".into(), 0).unwrap(),
    )
    .unwrap();
    let g = c::Game::from_root(p).unwrap();
    let cycle = "K1l-1k K12a-12b K1k-1l K12b-12a";
    let o = shell(&format!(
        "record load-json {}\nmove {cycle} {cycle} K1l-1k K12a-12b\nrecord print\nmove K1k-1l\nrecord print\nlegal\nhistory\n",
        encode(&g)
    ));
    assert!(!o.status.success());
    assert!(String::from_utf8_lossy(&o.stderr).contains("RepetitionDeparture"));
    let r = records(&o);
    assert_eq!(r.len(), 2);
    assert_eq!(r[0], r[1]);
    let text = output(&o);
    assert!(text.contains("known-board-actions 10"));
    assert!(text.contains("responsible_sides: [Sente]"));
    assert!(!text.lines().any(|l| l.starts_with("K1k-1l {")));
}

#[test]
fn real_bounded_analysis_is_read_only_and_returns_a_legal_zero_work_choice() {
    let o = shell("record print\nanalyze 1 512\nrecord print\nanalyze 1 0\nrecord print\n");
    success(&o);
    let r = records(&o);
    assert_eq!(r.len(), 3);
    assert_eq!(r[0], r[1]);
    assert_eq!(r[0], r[2]);
    let lines: Vec<_> = output(&o)
        .lines()
        .filter(|l| l.starts_with("analysis-read-only"))
        .map(str::to_owned)
        .collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].contains("completed-depth 1"));
    assert!(lines[0].contains("Sente"));
    assert!(
        lines[1].contains(&format!(
            "decision Some(Choose({:?})) score None completed-depth 0 visited 0",
            c::Game::standard().legal_actions()[0]
        )),
        "{}",
        lines[1]
    );
}
