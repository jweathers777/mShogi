//! Verify the real headless process boundary, including stdout/stderr and errors.

use std::{
    io::Write,
    process::{Command, Output, Stdio},
};

fn command() -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_mshogi"));
    // No checkout-relative resources or input stream may be required to start.
    command
        .current_dir(std::env::temp_dir())
        .stdin(Stdio::null());
    command
}

fn run(args: &[&str]) -> Output {
    command().args(args).output().expect("console must start")
}

#[test]
fn help_and_default_invocation_are_headless() {
    for args in [&[][..], &["--help"][..], &["-h"][..]] {
        let output = run(args);
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.contains("Usage: mshogi"));
        assert!(stdout.contains("shell          Read chess commands from standard input"));
        assert!(stdout.contains("not a UCI engine"));
    }
}

#[test]
fn version_is_available_without_resources() {
    for flag in ["--version", "-V"] {
        let output = run(&[flag]);
        assert!(output.status.success());
        assert!(output.stderr.is_empty());
        assert_eq!(
            String::from_utf8(output.stdout).unwrap(),
            format!("mshogi {}\n", env!("CARGO_PKG_VERSION"))
        );
    }
}

#[test]
fn profile_identifies_coverage_without_claiming_game_adjudication() {
    let output = run(&["profile"]);
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Target rules profile: mshogi.chess.position.v1\nCoverage: position/FEN, all chess actions with undo, attacks, legal actions, check/mate/stalemate and perft; no game adjudication\n"
    );
}

#[test]
fn unsupported_commands_and_extra_arguments_fail_without_stdout() {
    for args in [
        &["perft"][..],
        &["--help", "extra"][..],
        &["profile", "extra"][..],
        &["shell", "extra"][..],
        &["--version", "--help"][..],
        &[""][..],
    ] {
        let output = run(args);
        assert_eq!(output.status.code(), Some(2));
        assert!(output.stdout.is_empty());
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.starts_with("error:"));
        assert!(stderr.contains("mshogi --help"));
    }
}

#[cfg(unix)]
#[test]
fn non_unicode_argument_is_a_usage_error_instead_of_a_panic() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt};

    let output = command()
        .arg(OsString::from_vec(vec![0xff]))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(
        String::from_utf8(output.stderr)
            .unwrap()
            .starts_with("error:")
    );
}

// Run `mshogi shell` with the given standard input.
fn shell(input: &[u8]) -> Output {
    let mut child = command()
        .arg("shell")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("console must start");
    child.stdin.take().unwrap().write_all(input).unwrap();
    child.wait_with_output().unwrap()
}

fn text(bytes: Vec<u8>) -> String {
    String::from_utf8(bytes).unwrap()
}

#[test]
fn shell_runs_the_full_diagnostic_workflow() {
    let input = "\
# load, inspect, move, undo, classify and count
position fen r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1
show
move e1g1 e8c8
fen
legal
undo
status
undo
fen
position fen 7k/5Q2/6K1/8/8/8/8/8 b - - 0 1
status
legal
perft 0
perft 2
divide 1
position startpos
perft 3
divide 2
quit
";
    let output = shell(input.as_bytes());
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(text(output.stderr), "");
    let stdout = text(output.stdout);
    let expected_start = "\
fen r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1
8 r . . . k . . r
7 . . . . . . . .
6 . . . . . . . .
5 . . . . . . . .
4 . . . . . . . .
3 . . . . . . . .
2 . . . . . . . .
1 R . . . K . . R
  a b c d e f g h
fen r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1
side white
castling KQkq
en-passant -
halfmove 0
fullmove 1
status quiet
history 0
fen 2kr3r/8/8/8/8/8/8/R4RK1 w - - 2 2
fen 2kr3r/8/8/8/8/8/8/R4RK1 w - - 2 2
";
    assert!(stdout.starts_with(expected_start), "{stdout}");
    let rest = &stdout[expected_start.len()..];
    let mut lines = rest.lines();
    let legal = lines.next().unwrap();
    assert!(legal.starts_with("legal "), "{legal}");
    let fields: Vec<&str> = legal.split(' ').collect();
    assert_eq!(fields[1].parse::<usize>().unwrap(), fields.len() - 2);
    let expected_rest = "\
fen r3k2r/8/8/8/8/8/8/R4RK1 b kq - 1 1
status quiet
fen r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1
fen r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1
fen 7k/5Q2/6K1/8/8/8/8/8 b - - 0 1
status stalemate
legal 0
perft 0 1
perft 2 0
total 0
fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
perft 3 8902
";
    let remaining: Vec<&str> = lines.collect();
    let (fixed, divide) = remaining.split_at(expected_rest.lines().count());
    assert_eq!(fixed.join("\n") + "\n", expected_rest);
    // Independent expectation: 20 root actions, each with 20 replies.
    assert_eq!(divide.len(), 21);
    assert_eq!(divide[0], "b1a3 20");
    assert_eq!(divide[19], "h2h4 20");
    assert_eq!(divide[20], "total 400");
}

#[test]
fn failed_commands_report_errors_and_preserve_session_state() {
    let input = "\
move e2e4
move e7e5 e1e3 g1f3
undo 2
position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0
position fen 8/8/8/8/8/8/8/8 w - - 0 1
position
move e9e4
move e7e8x
move
perft -1
perft
divide 0
status now
frobnicate
fen
undo
fen
";
    let output = shell(input.as_bytes());
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        text(output.stdout),
        "\
fen rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1
fen rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1
fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
"
    );
    let stderr = text(output.stderr);
    let lines: Vec<&str> = stderr.lines().collect();
    assert_eq!(lines.len(), 13, "{stderr}");
    assert_eq!(
        lines[0],
        "error: line 2: e1e3 (action 2 of 3): action rejected: invalid movement or blocked path; no actions applied"
    );
    assert_eq!(
        lines[1],
        "error: line 3: cannot undo 2: only 1 action(s) since the last load"
    );
    assert!(
        lines[2].starts_with("error: line 4: FEN syntax:"),
        "{}",
        lines[2]
    );
    assert!(lines[3].starts_with("error: line 5: FEN "), "{}", lines[3]);
    assert_eq!(
        lines[4],
        "error: line 6: usage: position startpos | position fen <FEN>"
    );
    assert_eq!(
        lines[5],
        "error: line 7: invalid action 'e9e4'; expected e.g. e2e4 or e7e8q"
    );
    assert_eq!(
        lines[6],
        "error: line 8: invalid action 'e7e8x'; expected e.g. e2e4 or e7e8q"
    );
    assert_eq!(lines[7], "error: line 9: usage: move <action>...");
    assert_eq!(
        lines[8],
        "error: line 10: invalid depth '-1'; expected an integer from 0 to 4294967295"
    );
    assert_eq!(lines[9], "error: line 11: usage: perft <depth>");
    assert_eq!(
        lines[10],
        "error: line 12: perft: divide requires depth of at least 1"
    );
    assert_eq!(lines[11], "error: line 13: 'status' takes no arguments");
    assert_eq!(
        lines[12],
        "error: line 14: unknown command 'frobnicate'; type 'help'"
    );
}

#[test]
fn quit_ends_the_session_and_end_of_input_is_success() {
    let output = shell(b"perft 1\nquit\nfrobnicate\n");
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(text(output.stdout), "perft 1 20\n");
    assert!(output.stderr.is_empty());

    let output = shell(b"");
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stdout.is_empty() && output.stderr.is_empty());

    // Windows line endings and a missing final newline are accepted.
    let output = shell(b"move e2e4\r\nfen");
    assert_eq!(output.status.code(), Some(0), "{output:?}");
    assert_eq!(
        text(output.stdout),
        "fen rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1\n".repeat(2)
    );
}

#[test]
fn invalid_utf8_lines_are_reported_and_skipped() {
    let output = shell(b"\xff\xfe\nperft 1\n");
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(text(output.stdout), "perft 1 20\n");
    assert_eq!(
        text(output.stderr),
        "error: line 1: input is not valid UTF-8\n"
    );
}

#[test]
fn help_lists_every_shell_command() {
    let output = shell(b"help\n");
    assert_eq!(output.status.code(), Some(0));
    let stdout = text(output.stdout);
    for command in [
        "position startpos",
        "position fen <FEN>",
        "fen",
        "show",
        "legal",
        "move <action>...",
        "undo [n]",
        "status",
        "perft <depth>",
        "divide <depth>",
        "help",
        "quit",
    ] {
        assert!(stdout.contains(&format!("  {command}")), "{command}");
    }
}

// The README example must reproduce byte for byte.
#[test]
fn readme_console_example_reproduces() {
    // A Windows checkout may convert the README to CRLF line endings.
    let readme = include_str!("../../../README.md").replace("\r\n", "\n");
    let block = |marker: &str| -> String {
        let start = readme.find(marker).unwrap_or_else(|| panic!("{marker}"));
        let fence = readme[start..].find("```text\n").unwrap() + start + "```text\n".len();
        let end = readme[fence..].find("```").unwrap() + fence;
        readme[fence..end].to_owned()
    };
    let output = shell(block("<!-- console-example:input -->").as_bytes());
    assert_eq!(output.status.code(), Some(1));
    assert_eq!(
        text(output.stdout),
        block("<!-- console-example:stdout -->")
    );
    assert_eq!(
        text(output.stderr),
        block("<!-- console-example:stderr -->")
    );
}

#[test]
fn legal_order_matches_library_divide_order_and_errors_are_clear() {
    let output = shell(b"position fen 1n2k2r/P6P/8/8/8/8/8/4K3 w k - 0 1\nlegal\ndivide 1\nposition fen 7k/5Q2/6K1/8/8/8/8/8 b - - 0 1\nmove h8g8\n");
    assert_eq!(output.status.code(), Some(1));
    let stdout = text(output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    let legal: Vec<&str> = lines[1].split(' ').skip(2).collect();
    let divide: Vec<&str> = lines[2..]
        .iter()
        .take_while(|line| !line.starts_with("total"))
        .map(|line| line.split(' ').next().unwrap())
        .collect();
    assert_eq!(legal, divide);
    assert!(legal.contains(&"a7b8n") && legal.contains(&"a7a8q"));
    assert_eq!(
        text(output.stderr),
        "error: line 5: h8g8 (action 1 of 1): action rejected: own king would be in check (FEN structure: nonmoving king is attacked); no actions applied\n"
    );
}
