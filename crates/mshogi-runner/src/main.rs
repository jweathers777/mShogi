//! Headless experiment CLI; stdout contains only summaries or replay status.
use mshogi_runner::{ExperimentConfig, ExperimentReport, run_experiment};
use std::{fs, io::Read, process::ExitCode};
fn read(path: &str, cap: u64) -> Result<String, String> {
    let mut text = String::new();
    fs::File::open(path)
        .map_err(|e| e.to_string())?
        .take(cap + 1)
        .read_to_string(&mut text)
        .map_err(|e| e.to_string())?;
    if text.len() as u64 > cap {
        return Err(format!("input exceeds {cap} bytes"));
    }
    Ok(text)
}
fn execute() -> Result<(), String> {
    let args: Vec<_> = std::env::args_os()
        .map(|arg| {
            arg.into_string()
                .map_err(|_| "arguments must be valid UTF-8".to_owned())
        })
        .collect::<Result<_, _>>()?;
    match args.as_slice() {
        [_, command, config, output] if command == "shogi-run" => {
            let config = mshogi_runner::shogi::Config::from_json(&read(config, 1_048_576)?)
                .map_err(|e| e.to_string())?;
            let report = mshogi_runner::shogi::run(&config).map_err(|e| e.to_string())?;
            let encoded = report.to_json().map_err(|e| e.to_string())?;
            use std::io::Write;
            let mut file = fs::OpenOptions::new().write(true).create_new(true)
                .open(output).map_err(|e| e.to_string())?;
            file.write_all(encoded.as_bytes()).map_err(|e| e.to_string())?;
            for (i, game) in report.games.iter().enumerate() {
                println!("game {i}: {:?} native {:?} score {:?}", game.stop, game.conclusion, game.score);
            }
            Ok(())
        }
        [_, command, input] if command == "shogi-replay" => {
            let report = mshogi_runner::shogi::Report::from_json(&read(input, 67_108_864)?)
                .map_err(|e| e.to_string())?;
            println!("replayed {} shogi game(s)", report.replay().map_err(|e| e.to_string())?.len());
            Ok(())
        }
        [_, command, config, output] if command == "run" => {
            let config = ExperimentConfig::from_json(&read(config, 1_048_576)?)
                .map_err(|e| e.to_string())?;
            let report = run_experiment(&config).map_err(|e| e.to_string())?;
            report
                .replay()
                .map_err(|e| format!("generated report did not replay: {e}"))?;
            // Refuse overwriting any existing evidence artifact.
            use std::io::Write;
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(output)
                .map_err(|e| e.to_string())?;
            file.write_all(report.to_json().as_bytes())
                .map_err(|e| e.to_string())?;
            println!(
                "{}",
                serde_json::to_string(&report.summaries).map_err(|e| e.to_string())?
            );
            Ok(())
        }
        [_, command, input] if command == "replay" => {
            let report = ExperimentReport::from_json(&read(input, 134_217_728)?)
                .map_err(|e| e.to_string())?;
            let games = report.replay().map_err(|e| e.to_string())?;
            println!("replayed {} game(s)", games.len());
            Ok(())
        }
        _ => Err(
            "usage: mshogi-match run CONFIG.json REPORT.json | mshogi-match replay REPORT.json | mshogi-match shogi-run CONFIG.json REPORT.json | mshogi-match shogi-replay REPORT.json"
                .into(),
        ),
    }
}
fn main() -> ExitCode {
    match execute() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("mshogi-match: {e}");
            ExitCode::FAILURE
        }
    }
}
