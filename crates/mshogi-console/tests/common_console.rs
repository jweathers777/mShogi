//! Actual user-facing entry points; board expectation supplied by the owner.
use mshogi_core::{chess, native::CanonicalRecord, shogi};
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
fn shell(game: &str, input: &str) -> Output {
    run(&["--game", game, "shell"], input)
}
fn stdout(out: &Output) -> String {
    String::from_utf8(out.stdout.clone()).unwrap()
}
fn records(out: &Output) -> Vec<String> {
    stdout(out)
        .lines()
        .filter_map(|l| l.strip_prefix("record ").map(str::to_owned))
        .collect()
}
fn success(out: &Output) {
    assert!(out.status.success(), "{out:?}");
    assert!(out.stderr.is_empty(), "{out:?}");
}
fn initial(game: &str) -> String {
    match game {
        "chess" => {
            CanonicalRecord::chess(chess::Game::new(chess::ClaimPolicy::ExplicitOnly).record())
        }
        _ => CanonicalRecord::shogi(shogi::GameRecord::from_game(&shogi::Game::new())),
    }
    .to_json()
    .unwrap()
}
struct Dir(PathBuf);
impl Dir {
    fn new() -> Self {
        static N: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "mshogi-common-{}-{}",
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
const BOARD: &str = "a  l  n  s  g  k  g  s  n  l
b  .  r  .  .  .  .  .  b  .
c  p  p  p  p  p  p  .  p  p
d  .  .  .  .  .  .  p  .  .
e  .  .  .  .  .  .  .  .  .
f  .  .  P  .  .  .  .  .  .
g  P  P  .  P  P  P  P  P  P
h  .  B  .  .  .  .  .  R  .
i  L  N  S  G  K  G  S  N  L
   9  8  7  6  5  4  3  2  1\n";
#[test]
fn canonical_shells_route_native_commands_and_compatibility_aliases_match() {
    for (game, alias, moves, expected) in [
        ("chess", "shell", "e2e4 e7e5", "perft 1 20"),
        ("shogi", "shogi", "7g7f 3c3d", "perft-local 1 30"),
    ] {
        let input = format!(
            "position startpos\nmove {moves}\nshow\nlegal\nhistory\nstatus\nundo 2\nrecord print\nperft 1\ndivide 1\n"
        );
        let out = shell(game, &input);
        success(&out);
        let old = run(&[alias], &input);
        success(&old);
        assert_eq!(out.stdout, old.stdout);
        assert!(stdout(&out).contains(expected));
        assert_eq!(records(&out), vec![initial(game)]);
    }
}
#[test]
fn selected_help_profiles_and_argument_admission_are_explicit() {
    for (game, profile) in [
        ("chess", "mshogi.chess.position.v1"),
        ("shogi", shogi::PROFILE_ID),
    ] {
        let out = run(&["--game", game, "profile"], "");
        success(&out);
        assert!(stdout(&out).contains(profile));
        let out = run(&["--game", game, "--help"], "");
        success(&out);
        assert!(stdout(&out).contains(&format!("Read {game} commands")));
        let out = shell(game, "help\n");
        success(&out);
        for op in [
            "record print",
            "record save <path>",
            "record load-file <path>",
            "record load-json <JSON>",
        ] {
            assert!(stdout(&out).contains(op));
        }
    }
    for (args, message) in [
        (vec!["--game"], "requires"),
        (vec!["--game", "--help"], "missing game"),
        (vec!["--game", "xiangqii", "shell"], "unknown"),
        (
            vec!["--game", "chess", "--game", "shogi", "shell"],
            "conflicting",
        ),
        (
            vec!["--game", "chess", "--game", "chess", "shell"],
            "duplicate",
        ),
        (vec!["--game", "chess", "shogi"], "conflicts"),
        (vec!["--game", "shogi", "uci"], "UCI supports chess only"),
        (
            vec!["--game", "shogi", "internal-search"],
            "request supplies the game identity",
        ),
        (
            vec!["--game", "chess", "internal-search"],
            "request supplies the game identity",
        ),
        (
            vec!["--game", "shogi", "shell", "profile"],
            "extra arguments",
        ),
    ] {
        let out = run(&args, "");
        assert_eq!(out.status.code(), Some(2));
        assert!(out.stdout.is_empty());
        assert!(
            String::from_utf8_lossy(&out.stderr).contains(message),
            "{out:?}"
        );
    }
    success(&run(&["--game", "shogi", "shogi"], "status\n"));
    let out = run(&["--game", "chess", "uci"], "uci\nisready\nquit\n");
    success(&out);
    assert!(stdout(&out).contains("uciok"));
    assert!(!stdout(&out).contains("headless console"));
}
#[test]
fn exact_compact_board_and_inspection_preserve_state_and_native_notation() {
    let out = shell(
        "shogi",
        "position startpos\nmove 7g7f 3c3d\nrecord print\nshow\nhands\nstatus\nsfen\nrecord print\n",
    );
    success(&out);
    assert!(stdout(&out).contains(BOARD), "{}", stdout(&out));
    assert!(stdout(&out).contains(&format!("{BOARD}hands sente")));
    let rec = records(&out);
    assert_eq!(rec.len(), 2);
    assert_eq!(rec[0], rec[1]);
    let CanonicalRecord::Shogi(record) = CanonicalRecord::from_json(&rec[0]).unwrap() else {
        panic!()
    };
    assert_eq!(record.to_game().unwrap().events().len(), 2);
    let readme = include_str!("../../../docs/m4/shogi-console.md").replace("\r\n", "\n");
    assert!(
        readme.contains(BOARD),
        "documented board must be actual output"
    );
    let out = shell(
        "chess",
        "move e2e4 e7e5\nrecord print\nshow\nfen\nstatus\nhistory\nrecord print\n",
    );
    success(&out);
    let r = records(&out);
    assert_eq!(r[0], r[1]);
}
#[test]
fn promotions_use_fixed_stride_without_changing_long_labels() {
    let out = shell(
        "shogi",
        "position sfen reduced 8k/9/9/2+r3+B2/4+P4/9/9/9/K8 b - 1\nrecord print\nshow\nlegal\nrecord print\n",
    );
    success(&out);
    let s = stdout(&out);
    let rows: Vec<_> = s
        .lines()
        .filter(|l| l.starts_with("d ") || l.starts_with("e "))
        .collect();
    assert_eq!(rows[0], "d  .  . +r  .  .  . +B  .  .");
    assert_eq!(rows[1], "e  .  .  .  . +P  .  .  .  .");
    for (token, offset) in [("+r", 8), ("+B", 20)] {
        assert_eq!(rows[0].find(token), Some(offset));
    }
    assert!(s.contains("S:+Pawn[") && s.contains("S:+Bishop["));
    let r = records(&out);
    assert_eq!(r[0], r[1]);
}
#[test]
fn both_games_roundtrip_common_files_json_and_restored_undo() {
    for (game, moves, undo) in [
        ("chess", "g1f3 g8f6 f3g1 f6g8", 4),
        ("shogi", "7g7f 3c3d", 2),
    ] {
        let dir = Dir::new();
        let path = dir.0.join("complete record-é.json");
        let out = shell(
            game,
            &format!(
                "move {moves}\nrecord print\nrecord save {}\nposition startpos\nrecord load-file {}\nrecord print\nundo {undo}\nrecord print\n",
                path.display(),
                path.display()
            ),
        );
        success(&out);
        let r = records(&out);
        assert_eq!(r.len(), 3);
        assert_eq!(r[0], r[1]);
        assert_eq!(r[2], initial(game));
        assert_eq!(fs::read_to_string(&path).unwrap(), r[0]);
        let json = shell(
            game,
            &format!(
                "record load-json {}\nrecord print\nundo {undo}\nrecord print\n",
                r[0]
            ),
        );
        success(&json);
        assert_eq!(records(&json), vec![r[0].clone(), initial(game)]);
        let out = shell(
            game,
            &format!(
                "move {moves}\nrecord print\nrecord save {}\nrecord print\nundo {undo}\nrecord print\n",
                path.display()
            ),
        );
        assert_eq!(out.status.code(), Some(1));
        let rr = records(&out);
        assert_eq!(rr[0], rr[1]);
        assert_eq!(rr[2], initial(game));
        assert_eq!(fs::read_to_string(path).unwrap(), r[0]);
        assert_eq!(fs::read_dir(&dir.0).unwrap().count(), 1, "no temp residue");
    }
}
#[test]
fn hostile_common_loads_are_atomic_including_undo_and_strict_nested_fields() {
    for (game, moves, n, other) in [("chess", "e2e4", 1, "shogi"), ("shogi", "7g7f", 1, "chess")] {
        let valid = initial(game);
        let dir = Dir::new();
        let path = dir.0.join("hostile.json");
        let hostile = [
            "{".into(),
            initial(other),
            valid.replacen("\"payload\":", "\"payload\":{},\"payload\":", 1),
            valid.replace(
                "\"schema_version\":1",
                "\"schema_version\":1,\"schema_version\":1",
            ),
            valid.replacen("\"payload\":{", "\"payload\":{\"unknown\":true,", 1),
            valid.replace("\"envelope_version\":1", "\"envelope_version\":999"),
            valid.replacen("\"game_kind\":", "\"unknown\":0,\"game_kind\":", 1),
        ];
        for json in hostile {
            assert_ne!(json, valid, "hostile mutation must take effect");
            fs::write(&path, &json).unwrap();
            let out = shell(
                game,
                &format!(
                    "move {moves}\nrecord print\nrecord load-json {json}\nrecord print\nrecord load-file {}\nrecord print\nundo {n}\nrecord print\n",
                    path.display()
                ),
            );
            assert_eq!(out.status.code(), Some(1), "{out:?}");
            assert_eq!(
                String::from_utf8_lossy(&out.stderr).lines().count(),
                2,
                "{out:?}"
            );
            let r = records(&out);
            assert_eq!(r.len(), 4);
            assert_eq!(r[0], r[1]);
            assert_eq!(r[1], r[2]);
            assert_eq!(r[3], initial(game));
        }
        for bytes in [vec![b' '; 1024 * 1024 + 1], vec![255]] {
            fs::write(&path, bytes).unwrap();
            let out = shell(
                game,
                &format!(
                    "move {moves}\nrecord print\nrecord load-file {}\nrecord print\nundo {n}\nrecord print\n",
                    path.display()
                ),
            );
            assert_eq!(out.status.code(), Some(1));
            let r = records(&out);
            assert_eq!(r[0], r[1]);
            assert_eq!(r[2], initial(game));
        }
    }
}
#[test]
fn common_chess_restore_refuses_observations_and_full_modes_without_losing_undo() {
    use chess::*;
    let mut observed = Game::new(ClaimPolicy::ExplicitOnly).record();
    observed.push_event(RecordEvent {
        ply: 0,
        kind: RecordEventKind::Observation(MatchObservation::Command {
            actor: Color::White,
            command: RecordedCommand::OfferDraw,
        }),
    });
    let full = AdjudicatedGame::from_fen_with_mode(
        STANDARD_START_FEN,
        ClaimPolicy::ExplicitOnly,
        AdjudicationMode::Full {
            limits: ReachabilityLimits {
                max_positions: Some(0),
                max_transitions: None,
            },
        },
    )
    .unwrap()
    .record();
    for (record, message) in [
        (observed, "cannot restore match state"),
        (full, "cannot restore that mode"),
    ] {
        let envelope = CanonicalRecord::chess(record).to_json().unwrap();
        let dir = Dir::new();
        let path = dir.0.join("unsupported.json");
        fs::write(&path, &envelope).unwrap();
        let out = shell(
            "chess",
            &format!(
                "move e2e4\nrecord print\nrecord load-json {envelope}\nrecord load-file {}\nrecord print\nundo\nrecord print\n",
                path.display()
            ),
        );
        assert_eq!(out.status.code(), Some(1));
        assert!(String::from_utf8_lossy(&out.stderr).contains(message));
        let r = records(&out);
        assert_eq!(r[0], r[1]);
        assert_eq!(r[2], initial("chess"));
    }
}
#[test]
fn common_shogi_restore_preserves_import_and_terminal_event_undo() {
    let out = shell(
        "shogi",
        "position sfen reduced 8k/9/9/9/9/9/9/9/K8 b P 17\nmove P*5e\nresign sente\nrecord print\n",
    );
    success(&out);
    let r = records(&out);
    let restored = shell(
        "shogi",
        &format!(
            "record load-json {}\nhistory\nundo\nstatus\nundo\nsfen\n",
            r[0]
        ),
    );
    success(&restored);
    let s = stdout(&restored);
    assert!(s.contains("counter Some(Supplied)"));
    assert!(s.contains("events 2 known-board-actions 1"));
    assert!(s.contains("known-events 1 conclusion None"));
    assert!(s.ends_with("sfen 8k/9/9/9/9/9/9/9/K8 b P 17\n"));
}
#[test]
fn explicit_record_operations_never_guess_path_vs_json_and_recover_after_long_lines() {
    for game in ["chess", "shogi"] {
        let dir = Dir::new();
        let path = dir.0.join("not-json");
        fs::write(&path, initial(game)).unwrap();
        let long = "x".repeat(4097);
        let pretty = format!("{}{}", initial(game), " ".repeat(5000));
        let large = dir.0.join("large.json");
        fs::write(&large, pretty).unwrap();
        let out = shell(
            game,
            &format!(
                "record load-json {}\nrecord load-file {{}}\n{long}\nrecord load-file {}\nrecord print\n",
                path.display(),
                large.display()
            ),
        );
        assert_eq!(out.status.code(), Some(1));
        assert_eq!(String::from_utf8_lossy(&out.stderr).lines().count(), 3);
        assert_eq!(records(&out), vec![initial(game)]);
    }
}
