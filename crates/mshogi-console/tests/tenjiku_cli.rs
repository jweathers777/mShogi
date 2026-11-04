//! Actual Tenjiku console adapter. Expected geometry/effects are specified from
//! the accepted Lion/Tetrarch/fire profile fixtures, never copied console output.
use mshogi_core::{native::CanonicalRecord, tenjiku as t};
use std::{
    fs,
    io::Write,
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
fn profile(p: t::Profile) -> &'static str {
    if p == t::Profile::AdamsModern {
        "adams"
    } else {
        "muller"
    }
}
fn shell(p: t::Profile, input: &str) -> Output {
    // Rich native roots exceed the intentional 4096-byte shell line bound.
    // Exercise the public record-file adapter for those fixtures, leaving small
    // records on the direct record-json path and preserving the production cap.
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let mut paths = Vec::new();
    let lines = input
        .lines()
        .map(|line| {
            if line.len() > 4096
                && let Some(json) = line.strip_prefix("position record-json ")
            {
                let path = std::env::temp_dir().join(format!(
                    "mshogi-ten44-fixture-{}-{}.json",
                    std::process::id(),
                    NEXT.fetch_add(1, Ordering::Relaxed)
                ));
                fs::write(&path, json).unwrap();
                let replacement = format!("position record-file {}", path.display());
                paths.push(path);
                replacement
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n";
    let result = run(
        &["--game", "tenjiku", "--profile", profile(p), "shell"],
        &lines,
    );
    for path in paths {
        fs::remove_file(path).unwrap();
    }
    result
}
fn output(o: &Output) -> String {
    String::from_utf8(o.stdout.clone()).unwrap()
}
fn success(o: &Output) {
    assert!(
        o.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&o.stderr),
        output(o)
    );
}
fn records(o: &Output) -> Vec<String> {
    output(o)
        .lines()
        .filter_map(|l| l.strip_prefix("record ").map(str::to_owned))
        .collect()
}
fn sq(f: u8, r: u8) -> t::Square {
    t::Square::new(f, r).unwrap()
}
fn root(p: t::Profile, extra: &[(u8, u8, t::Side, t::Base, bool)]) -> t::Game {
    let entries = [
        (16, 16, t::Side::Sente, t::Base::K, false),
        (16, 14, t::Side::Sente, t::Base::P, false),
        (1, 1, t::Side::Gote, t::Base::K, false),
        (1, 3, t::Side::Gote, t::Base::P, false),
    ]
    .into_iter()
    .chain(extra.iter().copied())
    .enumerate()
    .map(|(id, (f, r, side, base, promoted))| {
        (
            sq(f, r),
            t::Piece::new(id as u16, side, base, promoted).unwrap(),
        )
    })
    .collect::<Vec<_>>();
    t::Game::from_root(
        t::Position::from_diagram(
            p,
            &entries,
            t::Side::Sente,
            t::RootMetadata::imported("TEN-CONSOLE independently specified root".into(), 31)
                .unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}
fn encode(g: &t::Game) -> String {
    CanonicalRecord::tenjiku(t::GameRecord::from_game(g).unwrap())
        .to_json()
        .unwrap()
}
fn decode(s: &str) -> t::Game {
    let CanonicalRecord::Tenjiku(r) = CanonicalRecord::from_json(s).unwrap() else {
        panic!()
    };
    r.to_game().unwrap()
}
#[test]
fn explicit_whole_profile_selection_protocol_guards_and_standard_board() {
    assert_eq!(
        run(&["--game", "tenjiku", "shell"], "").status.code(),
        Some(2)
    );
    for p in t::Profile::ALL {
        let o = run(
            &["--game", "tenjiku", "--profile", profile(p), "profile"],
            "",
        );
        success(&o);
        assert!(output(&o).contains(p.id()));
        assert!(output(&o).contains(p.edition()));
        success(&run(
            &["--game", "tenjiku", "--profile", p.id(), "profile"],
            "",
        ));
        let o = shell(p, "help\nshow\n");
        success(&o);
        let text = output(&o);
        for expected in [
            "not USI/SFEN",
            "ChS+8h!x9h",
            "sVGn",
            "gFiD",
            "royals sente K@9p",
            "royals gote K@8a",
            "known-board-actions 0",
        ] {
            assert!(text.contains(expected), "{expected}\n{text}");
        }
        assert!(!text.contains("s:VGn"));
        for mode in ["uci", "usi", "internal-search"] {
            assert_eq!(
                run(&["--game", "tenjiku", "--profile", profile(p), mode], "")
                    .status
                    .code(),
                Some(2)
            );
        }
    }
    for args in [
        vec!["--game", "tenjiku", "--profile", "unknown", "shell"],
        vec!["--game", "chu", "--profile", "adams", "shell"],
        vec![
            "--game",
            "tenjiku",
            "--profile",
            "adams",
            "--profile",
            "muller",
            "shell",
        ],
    ] {
        assert_eq!(run(&args, "").status.code(), Some(2));
    }
}
#[test]
fn complete_routes_igui_original_identity_history_and_exact_event_undo() {
    for p in t::Profile::ALL {
        let g = root(
            p,
            &[
                (8, 8, t::Side::Sente, t::Base::Ln, false),
                (8, 7, t::Side::Gote, t::Base::S, false),
                (8, 6, t::Side::Gote, t::Base::R, false),
                (4, 8, t::Side::Sente, t::Base::VGn, false),
                (12, 8, t::Side::Sente, t::Base::ChS, true),
                (13, 8, t::Side::Gote, t::Base::G, false),
            ],
        );
        let o = shell(
            p,
            &format!(
                "position record-json {}\nrecord\nlegal\nmove Ln8hx8gx8f\nhistory\nundo\nmove Ln8hx8g-8h\nhistory\nundo\nmove Ln8h-7g-8h\nhistory\nundo\nmove VGn4h-4g-5g-5f\nhistory\nundo\nmove ChS+12h!x13h\nhistory\nshow\nundo\nrecord\n",
                encode(&g)
            ),
        );
        success(&o);
        assert_eq!(records(&o)[0], records(&o)[1]);
        let text = output(&o);
        for expected in [
            "action Ln8hx8gx8f",
            "action Ln8hx8g-8h",
            "action Ln8h-7g-8h",
            "action VGn4h-4g-5g-5f",
            "action ChS+12h!x13h",
            "sChS+",
            "declared-root-plies 31",
        ] {
            assert!(text.contains(expected), "{expected}\n{text}");
        }
    }
}
#[test]
fn bad_batches_notation_and_diagnostics_preserve_exact_canonical_session() {
    for p in t::Profile::ALL {
        let g = root(
            p,
            &[
                (8, 8, t::Side::Sente, t::Base::Ln, false),
                (8, 7, t::Side::Gote, t::Base::S, false),
            ],
        );
        let o = shell(
            p,
            &format!(
                "position record-json {}\nrecord\nmove Ln8hx8g-8h Ln8h-8g\nrecord\nmove Ln8h-8g\nrecord\nperft 2 1\nrecord\ndivide 0\nrecord\nrecord load-json {{\"game\":\"tenjiku\",\"game\":\"chess\"}}\nrecord\n",
                encode(&g)
            ),
        );
        assert!(!o.status.success());
        let r = records(&o);
        assert_eq!(r.len(), 6);
        assert!(r.iter().all(|x| x == &r[0]));
        let errors = String::from_utf8_lossy(&o.stderr);
        assert!(errors.contains("no actions applied"));
        assert!(errors.contains("perft incomplete"));
        assert!(!output(&o).contains("perft-game"));
    }
}
#[test]
fn full_history_diagnostics_have_independent_corner_counts_and_analysis_is_read_only() {
    // Each side has three corner King moves and one remote Pawn move. Two
    // plies have 4*4=16 leaves, with 1+4+16=21 visited nodes.
    for p in t::Profile::ALL {
        let o = shell(
            p,
            &format!(
                "position record-json {}\nrecord\nperft 0 1\nperft 1 5\nperft 2 21\ndivide 2 21\nanalyze 1 512\nrecord\nanalyze 1 0\nrecord\n",
                encode(&root(p, &[]))
            ),
        );
        success(&o);
        let text = output(&o);
        for expected in [
            "perft-game 0 leaves 1 visited 1",
            "perft-game 1 leaves 4 visited 5",
            "perft-game 2 leaves 16 visited 21",
            "total-game 16 visited 21",
            "K16p-15o 4",
            "P16n-16m 4",
            "analysis-read-only perspective Sente",
            "decision None score None completed-depth 0 visited 0",
        ] {
            assert!(text.contains(expected), "{expected}\n{text}");
        }
        let r = records(&o);
        assert_eq!(r.len(), 3);
        assert!(r.iter().all(|x| x == &r[0]));
    }
}
#[test]
fn promotion_derived_profile_specific_burn_and_mover_death_are_lossless() {
    for p in t::Profile::ALL {
        let g = root(p, &[(12, 6, t::Side::Sente, t::Base::P, false)]);
        let o = shell(
            p,
            &format!(
                "position record-json {}\nmove P12f-12e^\nhistory\nrecord\nundo\nmove P12f-12e\nrecord\n",
                encode(&g)
            ),
        );
        success(&o);
        let r = records(&o);
        assert!(decode(&r[0]).position().piece(sq(12, 5)).unwrap().promoted);
        assert!(!decode(&r[1]).position().piece(sq(12, 5)).unwrap().promoted);
        let fire = root(
            p,
            &[
                (8, 10, t::Side::Sente, t::Base::FiD, false),
                (9, 8, t::Side::Gote, t::Base::FiD, false),
                (8, 7, t::Side::Gote, t::Base::P, false),
            ],
        );
        let o = shell(
            p,
            &format!(
                "position record-json {}\nrecord\nmove FiD8j-8h\nhistory\nrecord\nundo\nrecord\n",
                encode(&fire)
            ),
        );
        success(&o);
        let r = records(&o);
        assert_eq!(r[0], r[2]);
        let played = decode(&r[1]);
        let phases = &played.events()[0].effects.as_ref().unwrap().phases;
        if p == t::Profile::AdamsModern {
            assert!(
                matches!(&phases[0],t::EffectPhase::Burn{victims} if victims.len()==1 && victims[0].square==sq(8,7))
            );
            assert_eq!(phases.len(), 2);
        } else {
            assert_eq!(phases.len(), 1);
            assert!(played.position().piece(sq(8, 7)).is_some());
        }
        assert!(
            matches!(phases.last().unwrap(),t::EffectPhase::MoverDies{square,..} if *square==sq(8,8))
        );
        assert!(played.position().piece(sq(8, 8)).is_none());
        assert!(output(&o).contains("action FiD8j-8h"));
    }
}
#[test]
fn canonical_save_load_profile_identity_existing_file_and_repetition_admission() {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    for p in t::Profile::ALL {
        let path = std::env::temp_dir().join(format!(
            "mshogi-ten44-{}-{}.json",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        let other = if p == t::Profile::AdamsModern {
            t::Profile::MullerCompleted
        } else {
            t::Profile::AdamsModern
        };
        let o = shell(
            other,
            &format!(
                "position record-json {}\nmove K16p-15o K1a-2b K15o-16p\nrecord\nmove K2b-1a\nrecord\nrecord save {}\nrecord save {}\nposition startpos\nrecord load-file {}\nrecord\n",
                encode(&root(p, &[])),
                path.display(),
                path.display(),
                path.display()
            ),
        );
        assert!(!o.status.success());
        let r = records(&o);
        assert_eq!(r.len(), 3);
        assert!(r.iter().all(|x| x == &r[0]));
        assert_eq!(decode(&r[0]).profile(), p);
        assert_eq!(decode(&r[0]).events().len(), 3);
        assert_eq!(fs::read_to_string(&path).unwrap(), r[0]);
        fs::remove_file(path).unwrap();
        let errors = String::from_utf8_lossy(&o.stderr);
        assert!(errors.contains("Repetition"));
        assert!(errors.contains("existing files are preserved"));
    }
}
#[test]
fn actual_draw_stop_resignation_and_terminal_diagnostics_remain_native() {
    for p in t::Profile::ALL {
        let o = shell(
            p,
            &format!(
                "position record-json {}\nrecord\noffer-draw gote\nhistory\ndecline-draw sente\nundo 2\nrecord\noffer-draw sente\naccept-draw gote\nhistory\nperft 1\nundo 2\nstop work\nresign gote\nhistory\nundo 2\nrecord\n",
                encode(&root(p, &[]))
            ),
        );
        success(&o);
        let text = output(&o);
        for expected in [
            "conclusion Some(Agreement)",
            "Resignation { winner: Sente }",
            "incomplete work is unscored",
            "board-ply 0 actor gote action OfferDraw",
            "perft-game 1 leaves 0 visited 1",
        ] {
            assert!(text.contains(expected), "{expected}\n{text}");
        }
        let r = records(&o);
        assert!(r.iter().all(|x| x == &r[0]));
    }
}

#[test]
fn loaded_profile_is_preserved_by_fresh_start_and_oversized_lines_are_drained() {
    for p in t::Profile::ALL {
        let other = if p == t::Profile::AdamsModern {
            t::Profile::MullerCompleted
        } else {
            t::Profile::AdamsModern
        };
        let o = shell(
            other,
            &format!(
                "position record-json {}\nposition startpos\nrecord\n{}\nrecord\n",
                encode(&root(p, &[])),
                "x".repeat(4097)
            ),
        );
        assert!(!o.status.success());
        assert!(String::from_utf8_lossy(&o.stderr).contains("line exceeds 4096 bytes"));
        let r = records(&o);
        assert_eq!(r.len(), 2);
        assert_eq!(r[0], r[1]);
        let game = decode(&r[0]);
        assert_eq!(game.profile(), p);
        assert_eq!(game.position().pieces().count(), 156);
        assert_eq!(game.position().root().kind(), t::RootKind::Standard);
    }
}
