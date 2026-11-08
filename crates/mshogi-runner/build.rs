//! Observed harness build provenance; participant executables are fingerprinted separately.
use sha2::{Digest, Sha256};
use std::{env, process::Command};

fn output(program: &str, args: &[&str]) -> String {
    Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_else(|| "unavailable".into())
}
fn main() {
    println!("cargo:rerun-if-changed=../../.git/HEAD");
    println!("cargo:rerun-if-changed=../../.git/index");
    println!("cargo:rerun-if-changed=../../crates");
    for (key, value) in [
        (
            "EXPERIMENT_BUILD_REVISION",
            output("git", &["rev-parse", "HEAD"]),
        ),
        (
            "EXPERIMENT_BUILD_DIRTY_SHA256",
            format!(
                "{:x}",
                Sha256::digest(
                    output(
                        "git",
                        &[
                            "diff",
                            "--binary",
                            "HEAD",
                            "--",
                            "../../crates",
                            "../../Cargo.toml",
                            "../../Cargo.lock",
                            "../../rust-toolchain.toml"
                        ]
                    )
                    .as_bytes()
                )
            ),
        ),
        (
            "EXPERIMENT_BUILD_RUSTC",
            output(
                &env::var("RUSTC").unwrap_or_else(|_| "rustc".into()),
                &["--version"],
            ),
        ),
        (
            "EXPERIMENT_BUILD_PROFILE",
            env::var("PROFILE").unwrap_or_default(),
        ),
        (
            "EXPERIMENT_BUILD_TARGET",
            env::var("TARGET").unwrap_or_default(),
        ),
        (
            "EXPERIMENT_BUILD_FLAGS",
            env::var("CARGO_ENCODED_RUSTFLAGS")
                .unwrap_or_default()
                .replace('\x1f', " "),
        ),
    ] {
        println!("cargo:rustc-env={key}={value}");
    }
}
