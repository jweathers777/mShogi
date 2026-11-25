//! Actual Tori human executable consumer; expected examples are hand-derived T02–T08.
use mshogi_core::{native::CanonicalRecord, tori as t};
use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicU64, Ordering},
};
fn run(args: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mshogi"))
        .args(args)
        .current_dir(std::env::temp_dir())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    child.wait_with_output().unwrap()
}
fn shell(input: &str) -> Output {
    run(&["--game", "tori", "shell"], input)
}
fn output(o: &Output) -> String {
    String::from_utf8(o.stdout.clone()).unwrap()
}
fn success(o: &Output) {
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert!(o.stderr.is_empty());
}
fn records(o: &Output) -> Vec<String> {
    output(o)
        .lines()
        .filter_map(|l| l.strip_prefix("record ").map(str::to_owned))
        .collect()
}
fn root(pieces: &[(&str, t::Side, t::Face)], hand: t::Hand, counter: u64) -> t::Game {
    t::Game::from_root(
        t::Position::from_diagram(
            &pieces
                .iter()
                .map(|&(sq, side, face)| (sq.parse().unwrap(), t::Piece { side, face }))
                .collect::<Vec<_>>(),
            [hand, t::Hand::empty()],
            t::Side::Sente,
            t::RootMetadata::diagram(counter),
        )
        .unwrap(),
    )
    .unwrap()
}
fn capture_root() -> t::Game {
    root(
        &[
            ("4g", t::Side::Sente, t::Face::Phoenix),
            ("4a", t::Side::Gote, t::Face::Phoenix),
            ("7c", t::Side::Sente, t::Face::Falcon),
            ("7b", t::Side::Gote, t::Face::Swallow),
            ("6g", t::Side::Sente, t::Face::LeftQuail),
            ("1g", t::Side::Sente, t::Face::RightQuail),
        ],
        t::Hand::from_counts(&[(t::Face::RightQuail, 1)]).unwrap(),
        31,
    )
}
fn encode(game: &t::Game) -> String {
    CanonicalRecord::tori(t::GameRecord::from_game(game).unwrap())
        .to_json()
        .unwrap()
}
struct Dir(PathBuf);
impl Dir {
    fn new() -> Self {
        static N: AtomicU64 = AtomicU64::new(0);
        let p = std::env::temp_dir().join(format!(
            "mshogi-tori42-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        Self(p)
    }
}
impl Drop for Dir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn selector_profile_help_and_protocol_isolation_are_explicit() {
    let o = run(&["--game", "tori", "profile"], "");
    success(&o);
    assert!(output(&o).contains(t::PROFILE_ID));
    assert!(output(&o).contains(t::PROFILE_EDITION));
    let o = shell("help\n");
    success(&o);
    for text in ["not USI/SFEN", "record load-file", "RQ*5e"] {
        assert!(output(&o).contains(text), "{text}");
    }
    for protocol in ["uci", "usi"] {
        let o = run(&["--game", "tori", protocol], "");
        assert_eq!(o.status.code(), Some(2));
        assert!(o.stdout.is_empty());
    }
    for args in [
        &["--game", "tori", "--game", "chess"][..],
        &["--game", "tori", "internal-search"][..],
    ] {
        assert_eq!(run(args, "").status.code(), Some(2));
    }
}

#[test]
fn capture_forced_promotion_quails_drop_undo_and_native_history() {
    let o = shell(&format!(
        "record load-json {}\nshow\nlegal\nmove Fa7cx7b=Ea\nhands\nhistory\nundo\nmove RQ*5e\nshow\nhistory\n",
        encode(&capture_root())
    ));
    success(&o);
    let text = output(&o);
    for expected in [
        "sFa",
        "sLQ",
        "sRQ",
        "gSw",
        "Fa7cx7b=Ea",
        "hands sente Fa=0 Cr=0 Pt=0 LQ=0 RQ=1 Sw=1",
        "played RQ*5e",
        "hands sente Fa=0 Cr=0 Pt=0 LQ=0 RQ=0 Sw=0",
        "declared-root-plies 31 earlier-history-unknown true reachability-proven false",
        "event 1 board-ply 1 actor sente action RQ*5e",
    ] {
        assert!(text.contains(expected), "missing {expected}\n{text}");
    }
    assert!(!text.contains("s:Fa"));
    assert!(!text.contains("g:Sw"));
    assert!(!text.lines().any(|l| l.starts_with("Fa7cx7b ")));
}

#[test]
fn malformed_and_locally_legal_but_history_forbidden_requests_are_atomic() {
    let root = root(
        &[
            ("4g", t::Side::Sente, t::Face::Phoenix),
            ("4a", t::Side::Gote, t::Face::Phoenix),
        ],
        t::Hand::empty(),
        100,
    );
    // TORI-H-COUNT (MSH-208): Gote's Ph5a-4a would make the start position's
    // third occurrence; it is refused atomically, never a draw.
    let cycle = "Ph4g-5g Ph4a-5a Ph5g-4g Ph5a-4a";
    let o = shell(&format!(
        "record load-json {}\nmove {cycle} Ph4g-5g Ph4a-5a Ph5g-4g\nrecord print\nmove Ph5a-4a\nrecord print\nlegal\n",
        encode(&root)
    ));
    assert!(!o.status.success());
    assert!(String::from_utf8_lossy(&o.stderr).contains("ThirdOccurrence"));
    assert_eq!(records(&o)[0], records(&o)[1]);
    assert!(output(&o).contains("occurrences [3, 7]"));
    assert!(!output(&o).lines().any(|l| l.starts_with("Ph5a-4a {")));
    let o = shell(
        "record print\nmove Sw7e-7d Sw1c-1d Sw1d-1e\nrecord print\nmove Sw8e-7d\nrecord print\n",
    );
    assert!(!o.status.success());
    let r = records(&o);
    assert_eq!(r[0], r[1]);
    assert_eq!(r[0], r[2]);
}

#[test]
fn nonmove_events_stay_distinct_and_undo_restores_complete_semantics() {
    let o = shell(
        "move Sw7e-7d Sw1c-1d\nrecord print\nhistory\nresign sente\nhistory\nundo 1\nrecord print\n",
    );
    success(&o);
    let r = records(&o);
    assert_eq!(r[0], r[1]);
    let text = output(&o);
    assert!(text.contains("conclusion Some(Resignation { winner: Gote })"));
    assert!(text.contains("event 3 board-ply 2 actor sente action Resign"));
}

#[test]
fn canonical_atomic_save_load_retains_custom_root_events_hands_and_restored_undo() {
    let dir = Dir::new();
    let path = dir.0.join("Tori complete-é record.json");
    let o = shell(&format!(
        "record load-json {}\nmove Fa7cx7b=Ea\nresign gote\nrecord print\nrecord save {}\nposition startpos\nrecord load-file {}\nrecord print\nhistory\nundo 2\nrecord print\n",
        encode(&capture_root()),
        path.display(),
        path.display()
    ));
    success(&o);
    let r = records(&o);
    assert_eq!(r.len(), 3);
    assert_eq!(r[0], r[1]);
    assert_eq!(fs::read_to_string(&path).unwrap(), r[0]);
    assert_eq!(r[2], encode(&capture_root()));
    let CanonicalRecord::Tori(native) = CanonicalRecord::from_json(&r[0]).unwrap() else {
        panic!()
    };
    assert_eq!(native.known_board_actions(), 1);
    assert_eq!(native.event_count(), 2);
    assert_eq!(
        native.conclusion(),
        Some(t::Conclusion::Resignation {
            winner: t::Side::Sente
        })
    );
}

#[test]
fn strict_wrong_game_profile_derived_duplicate_and_large_loads_preserve_current_and_prior_file() {
    let dir = Dir::new();
    let path = dir.0.join("bad.json");
    let valid = encode(&capture_root());
    for bytes in [
        b"{".to_vec(),
        valid
            .replace(t::PROFILE_ID, "unknown.tori.profile")
            .into_bytes(),
        valid
            .replacen("\"edition\":", "\"edition\":\"duplicate\",\"edition\":", 1)
            .into_bytes(),
        valid
            .replace("\"declared_total_plies\":31", "\"declared_total_plies\":32")
            .into_bytes(),
        vec![255],
        CanonicalRecord::chess(
            mshogi_core::chess::Game::new(mshogi_core::chess::ClaimPolicy::ExplicitOnly).record(),
        )
        .to_json()
        .unwrap()
        .into_bytes(),
    ] {
        assert_ne!(bytes, valid.as_bytes(), "mutation must change the record");
        fs::write(&path, &bytes).unwrap();
        let o = shell(&format!(
            "move Sw7e-7d\nrecord print\nrecord load-file {}\nrecord print\nrecord save {}\nrecord print\n",
            path.display(),
            path.display()
        ));
        assert!(!o.status.success(), "unexpected accepted malformed record");
        let r = records(&o);
        assert_eq!(r.len(), 3);
        assert!(r.iter().all(|x| *x == r[0]));
        assert_eq!(fs::read(&path).unwrap(), bytes);
    }
}

#[test]
fn bounded_local_diagnostics_use_independent_counts_and_never_change_history() {
    // Native independent perft fixture: distant corner Phoenixes have 3 choices
    // each; depth 2=9, total visits 13. Explicit 12 visits must refuse, not count 9.
    let game = root(
        &[
            ("1a", t::Side::Sente, t::Face::Phoenix),
            ("7g", t::Side::Gote, t::Face::Phoenix),
        ],
        t::Hand::empty(),
        42,
    );
    let o = shell(&format!(
        "record load-json {}\nrecord print\nperft 0 1\nperft 1 4\nperft 2 13\ndivide 2 13\nperft 2 12\ndivide 0\nrecord print\n",
        encode(&game)
    ));
    assert!(!o.status.success());
    let text = output(&o);
    for expected in [
        "perft-local 0 leaves 1 visited 1",
        "perft-local 1 leaves 3 visited 4",
        "perft-local 2 leaves 9 visited 13",
        "total-local 9 visited 13",
    ] {
        assert!(text.contains(expected), "{text}");
    }
    assert!(String::from_utf8_lossy(&o.stderr).contains("WorkLimit"));
    assert_eq!(records(&o)[0], records(&o)[1]);
}

#[test]
fn read_only_native_analysis_reports_actual_work_and_no_fallback_for_zero_nodes() {
    let o = shell("record print\nanalyze 1 0\nanalyze 1 100\nrecord print\n");
    success(&o);
    let text = output(&o);
    assert!(text.contains("analysis-read-only perspective Sente"));
    assert!(text.contains("decision None score None completed-depth 0 visited 0"));
    assert!(text.contains("decision Some(Choose("));
    assert_eq!(records(&o)[0], records(&o)[1]);
}
