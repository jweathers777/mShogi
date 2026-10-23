//! Verify the real headless process boundary, including stdout/stderr and errors.

use std::process::{Command, Output, Stdio};

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
        assert!(stdout.contains("Game commands are not available in this console yet."));
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
fn profile_identifies_partial_coverage_without_claiming_game_support() {
    let output = run(&["profile"]);
    assert!(output.status.success());
    assert!(output.stderr.is_empty());
    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        "Target rules profile: mshogi.chess.position.v1\nCoverage: position/FEN, ordinary actions/undo and attack queries; special moves unavailable\n"
    );
}

#[test]
fn unsupported_commands_and_extra_arguments_fail_without_stdout() {
    for args in [
        &["perft"][..],
        &["--help", "extra"][..],
        &["profile", "extra"][..],
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
