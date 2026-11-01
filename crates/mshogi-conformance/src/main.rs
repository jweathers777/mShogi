//! Engine-independent development fixture CLI.
use mshogi_conformance::{Corpus, MAX_INPUT_BYTES, assertions};
use std::{fs::File, io::Read, process::ExitCode};
fn read(path: &str) -> Result<String, String> {
    let mut bytes = Vec::new();
    File::open(path)
        .map_err(|e| e.to_string())?
        .take((MAX_INPUT_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|e| e.to_string())?;
    if bytes.len() > MAX_INPUT_BYTES {
        return Err("input capacity exceeded".into());
    }
    String::from_utf8(bytes).map_err(|_| "input is not UTF-8".into())
}
fn execute() -> Result<bool, String> {
    let mut args = std::env::args().skip(1);
    let mut observations = None;
    let mut corpus_file = None;
    let mut owner_file = None;
    let mut require_complete = false;
    let mut require_owner: Option<String> = None;
    while let Some(a) = args.next() {
        match a.as_str() {
            "--help" => {
                println!(
                    "mshogi-conformance [--corpus FILE] [--ownership FILE] [--observations FILE] [--require-complete] [--require-owner MSH-N]\nNo observations: validate data and report native assertions pending. No external engine required."
                );
                return Ok(true);
            }
            "--corpus" | "--ownership" | "--observations" => {
                let value = args.next().ok_or("missing file argument")?;
                let slot = match a.as_str() {
                    "--corpus" => &mut corpus_file,
                    "--ownership" => &mut owner_file,
                    _ => &mut observations,
                };
                if slot.replace(value).is_some() {
                    return Err("duplicate option".into());
                }
            }
            "--require-owner" if require_owner.is_none() => {
                require_owner = Some(args.next().ok_or("missing owner")?);
            }
            "--require-complete" if !require_complete => require_complete = true,
            _ => return Err("unknown/duplicate option; use --help".into()),
        }
    }
    let c = corpus_file.map(|p| read(&p)).transpose()?;
    let o = owner_file.map(|p| read(&p)).transpose()?;
    let corpus = Corpus::load(
        c.as_deref().unwrap_or(mshogi_conformance::FIXTURES),
        o.as_deref().unwrap_or(mshogi_conformance::OWNERSHIP),
    )?;
    let items = observations
        .map(|p| read(&p).and_then(|s| assertions::observations(&corpus, &s)))
        .transpose()?
        .unwrap_or_default();
    let report = assertions::run(&corpus, &items)?;
    if let Some(owner) = &require_owner
        && !report.assertions.iter().any(|a| &a.owner == owner)
    {
        return Err("unknown assertion owner".into());
    }
    let owner_pending = require_owner.as_ref().is_some_and(|o| {
        report
            .assertions
            .iter()
            .any(|a| &a.owner == o && a.status == assertions::Status::Pending)
    });
    let success = !owner_pending
        && report.failed == 0
        && (!require_complete
            || report.pending == 0 && report.later_consumer_requirements.is_empty());
    println!(
        "{}",
        serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?
    );
    Ok(success)
}
fn main() -> ExitCode {
    match execute() {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!(
                "Conformance harness rejected input: {}",
                e.chars().take(1024).collect::<String>()
            );
            ExitCode::from(2)
        }
    }
}
