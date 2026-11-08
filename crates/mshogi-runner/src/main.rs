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
        [_, command, executable, output] if command == "experiment-template" => {
            let c=mshogi_runner::experiments::template(executable).map_err(|e|e.to_string())?;
            write_new(output,&serde_json::to_string_pretty(&c).map_err(|e|e.to_string())?)
        }
        [_, command, input, output] if command == "experiment-run" => {
            let c=mshogi_runner::experiments::Config::from_json(&read(input,1_048_576)?).map_err(|e|e.to_string())?;
            let r=mshogi_runner::experiments::run(&c).map_err(|e|e.to_string())?;
            write_new(output,&r.to_json().map_err(|e|e.to_string())?)?;
            println!("{}",serde_json::to_string(&r.summaries().map_err(|e|e.to_string())?).map_err(|e|e.to_string())?);Ok(())
        }
        [_, command, input] if command == "experiment-replay" => {
            let r=mshogi_runner::experiments::Report::from_json(&read(input,mshogi_runner::experiments::MAX_BYTES as u64)?).map_err(|e|e.to_string())?;
            println!("replayed {} native experiment games",r.games.len());Ok(())
        }
        [_, command, input, output] if command == "experiment-data" => {
            let r=mshogi_runner::experiments::Report::from_json(&read(input,mshogi_runner::experiments::MAX_BYTES as u64)?).map_err(|e|e.to_string())?;
            write_new(output,&r.data_jsonl().map_err(|e|e.to_string())?)
        }
        [_, command, protocol, input, output] if command == "experiment-external" => {
            use mshogi_runner::experiments::{ExternalProtocol as P,run_external};
            let p=match protocol.as_str(){"uci"=>P::Uci,"usi"=>P::Usi,"xiangqi-uci09"=>P::XiangqiUci09,_=>return Err("unsupported actual external adapter".into())};
            let r=run_external(p,&read(input,1_048_576)?).map_err(|e|e.to_string())?;
            write_new(output,&r.to_json().map_err(|e|e.to_string())?)
        }
        [_, command, input] if command == "experiment-external-replay" => {
            let r=mshogi_runner::experiments::ExternalReport::from_json(&read(input,mshogi_runner::experiments::MAX_BYTES as u64)?).map_err(|e|e.to_string())?;r.validate().map_err(|e|e.to_string())?;println!("replayed actual external adapter evidence");Ok(())
        }
        [_, command, config, output] if command == "xiangqi-run" => {
            let config = mshogi_runner::xiangqi::Config::from_json(&read(config, 1_048_576)?)
                .map_err(|e| e.to_string())?;
            let report = mshogi_runner::xiangqi::run(&config).map_err(|e| e.to_string())?;
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
        [_, command, input] if command == "xiangqi-replay" => {
            let report = mshogi_runner::xiangqi::Report::from_json(&read(input, 67_108_864)?)
                .map_err(|e| e.to_string())?;
            println!("replayed {} xiangqi game(s)", report.replay().map_err(|e| e.to_string())?.len());
            Ok(())
        }
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
            "usage: mshogi-match experiment-template EXE CONFIG | experiment-run CONFIG REPORT | experiment-replay REPORT | experiment-data REPORT JSONL | experiment-external uci|usi|xiangqi-uci09 CONFIG REPORT | experiment-external-replay REPORT | run CONFIG REPORT | replay REPORT | shogi-run CONFIG REPORT | shogi-replay REPORT | xiangqi-run CONFIG REPORT | xiangqi-replay REPORT"
                .into(),
        ),
    }
}
fn write_new(path: &str, text: &str) -> Result<(), String> {
    use std::io::Write;
    let mut f = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| e.to_string())?;
    if let Err(e) = f.write_all(text.as_bytes()).and_then(|_| f.sync_all()) {
        drop(f);
        let _ = fs::remove_file(path);
        return Err(e.to_string());
    }
    Ok(())
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
