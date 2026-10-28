//! Actual subprocess journeys using independently established MSH-92/94/105 facts.
use mshogi_core::{native::CanonicalRecord, xiangqi as x};
use serde_json::Value;
use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicU64, Ordering},
};

const CYCLE_ROOT: &str = "4R4/5k3/9/9/9/9/9/9/9/3K5 w - - 0 1";
const CYCLE: &str = "e9f9 f8e8 f9e9 e8f8";
const POSITIVE: &str = "5k3/R8/9/9/9/9/9/9/9/3K5 b - - 0 1";
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
    run(&["--game", "xiangqi", "shell"], input)
}
fn text(out: &Output) -> String {
    String::from_utf8(out.stdout.clone()).unwrap()
}
fn success(out: &Output) {
    assert!(out.status.success(), "{out:?}");
    assert!(out.stderr.is_empty(), "{out:?}");
}
fn tagged(out: &Output, tag: &str) -> Vec<String> {
    text(out)
        .lines()
        .filter_map(|l| l.strip_prefix(tag).map(str::to_owned))
        .collect()
}
fn games(out: &Output) -> Vec<x::Game> {
    tagged(out, "record ")
        .into_iter()
        .map(|s| {
            let CanonicalRecord::Xiangqi(r) = CanonicalRecord::from_json(&s).unwrap() else {
                panic!("wrong game")
            };
            r.to_game().unwrap()
        })
        .collect()
}
fn reports(out: &Output) -> Vec<x::ExactReport> {
    tagged(out, "nowin-result ")
        .into_iter()
        .map(|s| serde_json::from_str(&s).unwrap())
        .collect()
}
struct Dir(PathBuf);
impl Dir {
    fn new() -> Self {
        static N: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "mshogi-96-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Dir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn explicit_selection_profile_initial_choices_and_local_diagnostics() {
    let out = run(&["--game", "xiangqi", "profile"], "");
    success(&out);
    assert!(text(&out).contains(x::PROFILE_ID));
    let out = shell(
        "position startpos\nrecord print\nshow\nstatus\nlegal\nperft 0\nperft 1\ndivide 1\nrecord print\n",
    );
    success(&out);
    let s = text(&out);
    assert!(
        s.contains("legal-local 44"),
        "initial complete choices must be44\n{s}"
    );
    assert!(s.contains("perft-local 0 1"), "{s}");
    assert!(s.contains("perft-local 1 44"), "{s}");
    let r = tagged(&out, "record ");
    assert_eq!(r.len(), 2);
    assert_eq!(r[0], r[1], "inspection must not mutate history");
    assert_eq!(games(&out)[0].position().legal_actions().unwrap().len(), 44);
}

#[test]
fn failed_move_batch_and_malformed_position_are_atomic() {
    let out = shell(
        "record print\nmove a0a1 a9a8 a1a2 a2a3\nrecord print\nposition fen nonsense\nrecord print\nmove a0a1\nrecord print\nundo\nrecord print\n",
    );
    assert!(!out.status.success());
    let r = tagged(&out, "record ");
    assert_eq!(r.len(), 5);
    assert_eq!(r[0], r[1]);
    assert_eq!(r[0], r[2]);
    assert_ne!(r[0], r[3]);
    assert_eq!(r[0], r[4]);
    assert_eq!(games(&out)[3].events().len(), 1);
}

#[test]
fn third_occurrence_warns_fourth_prohibited_check_loses_and_undo_restores() {
    let out = shell(&format!(
        "position fen {CYCLE_ROOT}\nmove {CYCLE} {CYCLE}\nwarnings\nhistory\nrecord print\nmove {CYCLE}\nrecord print\nundo\nrecord print\n"
    ));
    success(&out);
    let g = games(&out);
    assert_eq!(g.len(), 3);
    assert_eq!(g[0].warnings().len(), 1);
    assert_eq!(g[0].warnings()[0].occurrences, [0, 4, 8]);
    assert!(g[0].warnings()[0].active);
    assert_eq!(g[0].events().len(), 9);
    assert_eq!(g[0].conclusion(), None);
    assert_eq!(
        g[1].conclusion(),
        Some(x::Conclusion::Win {
            winner: x::Side::Black,
            reason: x::WinReason::Repetition {
                fault: x::FaultKind::PerpetualCheck
            }
        })
    );
    assert_eq!(g[2].conclusion(), None);
    // Root-anchor obligation remains; other anchors can warn on later plies.
    assert!(
        g[2].warnings()
            .iter()
            .any(|w| w.occurrences == [0, 4, 8] && w.active)
    );
    assert_ne!(g[1].position(), g[2].position());
}

#[test]
fn advisory_count_claims_are_claimant_specific_and_terminal_commands_preserve_ply() {
    let fixtures: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/xiangqi/natural-count.json"
    ))
    .unwrap();
    let f = fixtures["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == "NATURAL-11-CHECKS")
        .unwrap();
    let moves = f["moves"]
        .as_array()
        .unwrap()
        .iter()
        .take(100)
        .map(|v| v.as_str().unwrap())
        .collect::<Vec<_>>()
        .join(" ");
    let out = shell(&format!(
        "position explicit fen {}\nmove {moves}\nrecord print\neligibility red\neligibility black\nrecord print\nclaim red\nrecord print\nclaim black\nrecord print\nundo\nrecord print\n",
        f["root_fen"].as_str().unwrap()
    ));
    success(&out);
    let r = tagged(&out, "record ");
    assert_eq!(r[0], r[1], "eligibility must be advisory");
    let g = games(&out);
    assert_eq!(
        g[0].natural_count_eligibility(x::Side::Red).effective_plies,
        99
    );
    assert_eq!(
        g[0].natural_count_eligibility(x::Side::Black)
            .effective_plies,
        100
    );
    assert_eq!(
        g[2].conclusion(),
        None,
        "invalid claim retains its native rejection audit"
    );
    assert!(g[2].events().len() > g[1].events().len());
    assert_eq!(
        g[3].conclusion(),
        Some(x::Conclusion::Draw {
            reason: x::DrawReason::NaturalCount
        })
    );
    assert_eq!(g[4].conclusion(), None);
    assert!(g.iter().all(|s| s.position() == g[0].position()));
}

#[test]
fn agreement_and_off_turn_resignation_are_not_board_actions() {
    let out = shell(
        "move a0a1\nrecord print\noffer red\naccept black\nrecord print\nundo\nresign red\nrecord print\nundo\nrecord print\n",
    );
    success(&out);
    let g = games(&out);
    assert_eq!(g[0].position().side_to_move(), x::Side::Black);
    assert_eq!(
        g[1].conclusion(),
        Some(x::Conclusion::Draw {
            reason: x::DrawReason::Agreement
        })
    );
    assert_eq!(
        g[2].conclusion(),
        Some(x::Conclusion::Win {
            winner: x::Side::Black,
            reason: x::WinReason::Resignation
        })
    );
    assert_eq!(g[3].conclusion(), None);
    assert!(g.iter().all(|s| s.position() == g[0].position()));
}

#[test]
fn offer_windows_and_decline_keep_native_admission_and_atomic_refusal() {
    // Documented minimal untimed offer window: after one's own move, once per
    // window. Declining does not permit a repeated offer at the same board ply.
    let out = shell(
        "move a0a1\nrecord print\noffer black\nrecord print\noffer red\ndecline black\nrecord print\noffer red\nrecord print\n",
    );
    assert!(!out.status.success());
    assert_eq!(
        String::from_utf8_lossy(&out.stderr)
            .matches("DrawOffer")
            .count(),
        2
    );
    let r = tagged(&out, "record ");
    assert_eq!(r.len(), 4);
    assert_eq!(r[0], r[1]);
    assert_eq!(r[2], r[3]);
    let g = games(&out);
    assert!(
        g.iter()
            .all(|s| s.position() == g[0].position() && s.conclusion().is_none())
    );
    assert!(g[2].events().len() > g[0].events().len());
}

#[test]
fn canonical_save_reopen_preserves_provenance_warning_and_restores_semantic_undo() {
    let dir = Dir::new();
    let path = dir.0.join("complete xiangqi-é.json");
    let out = shell(&format!(
        "position fen {CYCLE_ROOT}\nmove {CYCLE} {CYCLE}\nresign black\nrecord print\nrecord save {}\nposition startpos\nrecord load-file {}\nrecord print\nundo\nrecord print\nundo 8\nrecord print\n",
        path.display(),
        path.display()
    ));
    success(&out);
    let r = tagged(&out, "record ");
    assert_eq!(r[0], r[1]);
    assert_eq!(fs::read_to_string(&path).unwrap(), r[0]);
    let g = games(&out);
    assert_eq!(
        g[1].conclusion(),
        Some(x::Conclusion::Win {
            winner: x::Side::Red,
            reason: x::WinReason::Resignation
        })
    );
    assert_eq!(g[2].conclusion(), None);
    assert_eq!(g[2].warnings().len(), 1);
    assert_eq!(g[3].events().len(), 0);
    assert_eq!(g[3].position().to_fen().unwrap(), CYCLE_ROOT);
    assert_eq!(g[0].root(), g[3].root());
}

#[test]
fn strict_outer_nested_identity_and_bounded_file_failures_preserve_live_session() {
    let dir = Dir::new();
    let path = dir.0.join("bad.json");
    let initial = x::Game::standard(x::ClaimPolicy::Explicit).unwrap();
    let valid = CanonicalRecord::xiangqi(x::GameRecord::from_game(&initial).unwrap())
        .to_json()
        .unwrap();
    let chess = CanonicalRecord::chess(
        mshogi_core::chess::AdjudicatedGame::new(mshogi_core::chess::ClaimPolicy::ExplicitOnly)
            .record(),
    )
    .to_json()
    .unwrap();
    for bytes in [
        b"{".to_vec(),
        valid
            .replace("mshogi.xiangqi.wxf2018.v1", "unsupported.profile")
            .into_bytes(),
        valid
            .replacen(
                "\"envelope_version\":1",
                "\"envelope_version\":1,\"envelope_version\":1",
                1,
            )
            .into_bytes(),
        valid
            .replacen(
                "\"no_win_mode\":",
                "\"schema_version\":1,\"no_win_mode\":",
                1,
            )
            .into_bytes(),
        valid
            .replacen("\"envelope_version\":1", "\"envelope_version\":999", 1)
            .into_bytes(),
        chess.into_bytes(),
        vec![b' '; mshogi_core::native::MAX_RECORD_ENVELOPE_BYTES + 1],
        vec![255],
    ] {
        fs::write(&path, &bytes).unwrap();
        let out = shell(&format!(
            "move a0a1\nrecord print\nrecord load-file {}\nrecord print\nrecord save {}\nrecord print\n",
            path.display(),
            path.display()
        ));
        assert!(!out.status.success(), "{out:?}");
        let r = tagged(&out, "record ");
        assert_eq!(r.len(), 3);
        assert!(r.iter().all(|s| s == &r[0]));
        assert_eq!(fs::read(&path).unwrap(), bytes);
    }
    let root = x::PositionRoot::from_fen(CYCLE_ROOT).unwrap().to_json();
    let out = shell(&format!(
        "position json {root}\nrecord print\nrecord load-json {valid}\nrecord print\n"
    ));
    success(&out);
    assert_ne!(tagged(&out, "record ")[0], tagged(&out, "record ")[1]);
}

#[test]
fn exact_positive_negative_and_resource_unknown_are_read_only_with_source_provenance() {
    for (fen, states, expected) in [
        (POSITIVE, 5000, "positive"),
        ("5k3/9/9/9/9/9/9/9/4A4/3K5 w - - 0 1", 5000, "negative"),
        (POSITIVE, 0, "unknown"),
    ] {
        let out = shell(&format!(
            "position fen {fen}\nrecord print\nnowin fast\nnowin exact {states} 50000 60000 4194304 bfs\nnowin wait\nrecord print\n"
        ));
        success(&out);
        let r = tagged(&out, "record ");
        assert_eq!(r.len(), 2);
        assert_eq!(r[0], r[1]);
        assert_eq!(tagged(&out, "nowin-source "), vec![r[0].clone()]);
        let reports = reports(&out);
        assert_eq!(reports.len(), 1);
        match (&reports[0].verdict, expected) {
            (
                x::ExactVerdict::NaturalWin {
                    winner,
                    reason,
                    witness,
                },
                "positive",
            ) => {
                assert_eq!(*winner, x::Side::Red);
                assert_eq!(*reason, x::NoActionReason::Stalemate);
                assert_eq!(witness.len(), 2);
                let mut p = x::Position::from_fen(fen).unwrap();
                for &a in witness {
                    p.apply(a).unwrap();
                }
                assert_eq!(
                    p.local_status().unwrap(),
                    x::LocalStatus::NoLegalAction {
                        reason: x::NoActionReason::Stalemate
                    }
                );
            }
            (x::ExactVerdict::NeitherCanWin, "negative") => assert_eq!(reports[0].work.states, 255),
            (
                x::ExactVerdict::Unknown {
                    reason: x::ExactStop::States,
                },
                "unknown",
            ) => {}
            _ => panic!("unexpected {reports:?} for {expected}"),
        }
    }
}

#[test]
fn cancellation_replacement_and_undo_never_publish_retired_exact_work() {
    let out = shell(&format!(
        "record print\nnowin exact\nnowin cancel\nnowin poll\nrecord print\nnowin exact\nposition fen {POSITIVE}\nnowin poll\nnowin exact 5000 50000 60000 4194304 bfs\nnowin wait\nrecord print\nnowin exact\nmove f9e9\nundo\nnowin poll\nrecord print\n"
    ));
    assert!(!out.status.success());
    assert_eq!(
        String::from_utf8_lossy(&out.stderr)
            .matches("no active exact request")
            .count(),
        3
    );
    let r = tagged(&out, "record ");
    assert_eq!(r.len(), 4);
    assert_eq!(r[0], r[1]);
    assert_eq!(r[2], r[3]);
    let reports = reports(&out);
    assert_eq!(
        reports.len(),
        1,
        "retired results must not later be accepted\n{}",
        text(&out)
    );
    assert!(matches!(
        reports[0].verdict,
        x::ExactVerdict::NaturalWin { .. }
    ));
    assert_eq!(tagged(&out, "nowin-source "), vec![r[2].clone()]);
}

#[test]
fn oversized_input_is_drained_and_later_native_commands_still_work() {
    let out = shell(&format!(
        "record print\n{}\nmove a0a1\nrecord print\n",
        "x".repeat(4097)
    ));
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("line exceeds"));
    let g = games(&out);
    assert_eq!(g.len(), 2);
    assert_eq!(g[0].events().len(), 0);
    assert_eq!(g[1].events().len(), 1);
}
