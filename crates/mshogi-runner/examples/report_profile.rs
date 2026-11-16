//! Measure native report stages separately. Run under an external wall-time bound.
use mshogi_runner::experiments::Report;
use sha2::{Digest, Sha256};
use std::{error::Error, fs, time::Instant};

fn measured<T>(
    stage: &str,
    work: impl FnOnce() -> Result<T, Box<dyn Error>>,
) -> Result<(T, f64), Box<dyn Error>> {
    let start = Instant::now();
    let value = work()?;
    let elapsed = start.elapsed().as_secs_f64();
    eprintln!("{stage}: {elapsed:.6}s");
    Ok((value, elapsed))
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = std::env::args()
        .nth(1)
        .ok_or("usage: report_profile REPORT [--validated]")?;
    let mode = std::env::args().nth(2);
    if mode.as_deref().is_some_and(|value| value != "--validated") {
        return Err("expected --validated or no mode argument".into());
    }
    let text = fs::read_to_string(&path)?;
    if text.len() > mshogi_runner::experiments::MAX_BYTES {
        return Err("report exceeds native experiment bound".into());
    }
    // Native record deserialization itself performs admission work. No output
    // is trusted before the report's complete consistency validation below.
    let (report, parse): (Report, _) = measured("raw_parse", || Ok(serde_json::from_str(&text)?))?;
    let historical_request_seconds = report
        .games
        .iter()
        .flat_map(|game| &game.steps)
        .map(|step| step.elapsed_ns as f64 / 1e9)
        .sum::<f64>();
    let games = report.games.len();
    let steps = report
        .games
        .iter()
        .map(|game| game.steps.len())
        .sum::<usize>();
    let (validation, encoded, export, summary, summaries) = if mode.is_some() {
        let (report, validation) = measured("native_validation", || Ok(report.into_validated()?))?;
        let (encoded, export) = measured("json_export", || Ok(report.to_json()?))?;
        let (summary, summaries) = measured("summary", || {
            Ok(serde_json::to_string(&report.summaries()?)?)
        })?;
        (validation, encoded, export, summary, summaries)
    } else {
        let (_, validation) = measured("native_validation", || Ok(report.validate()?))?;
        let (encoded, export) = measured("json_export", || Ok(report.to_json()?))?;
        let (summary, summaries) = measured("summary", || {
            Ok(serde_json::to_string(&report.summaries()?)?)
        })?;
        (validation, encoded, export, summary, summaries)
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "mode": if mode.is_some() { "validated" } else { "legacy" },
            "input": path,
            "input_bytes": text.len(),
            "input_sha256": format!("{:x}", Sha256::digest(text.as_bytes())),
            "stages_seconds": {"raw_parse": parse, "native_validation": validation, "json_export": export, "summary": summaries},
            "historical_request_seconds": historical_request_seconds,
            "games": games,
            "steps": steps,
            "encoded_bytes": encoded.len(),
            "encoded_sha256": format!("{:x}", Sha256::digest(encoded.as_bytes())),
            "summary_sha256": format!("{:x}", Sha256::digest(summary.as_bytes())),
            "summary": serde_json::from_str::<serde_json::Value>(&summary)?
        }))?
    );
    Ok(())
}
