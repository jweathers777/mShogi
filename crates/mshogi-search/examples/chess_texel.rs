//! Bounded Texel fitting of the exact cached production evaluation.
//! fit TRAIN VALIDATION OUTPUT [SECONDS]; check FROZEN HELDOUT OUTPUT.
//! Held-out input is accepted only by the separate check command.
use mshogi_core::chess::Position;
use mshogi_search::chess_bitboard::ChessPosition;
use mshogi_search::chess_eval::weights::{COUNT, INITIAL, LOWER, NAMES, Prepared, UPPER, Weights};
use serde_json::{Value, json};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::time::{Duration, Instant};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const REGULARIZATION: f64 = 1e-5;
const STEPS: [i32; 5] = [16, 8, 4, 2, 1];

struct Sample {
    position: Prepared,
    result: f64,
}
struct Dataset {
    samples: Vec<Sample>,
    games: HashSet<String>,
    positions: HashSet<String>,
}

fn probability(score: i32, k: f64) -> f64 {
    1.0 / (1.0 + 10.0_f64.powf(-k * f64::from(score) / 400.0))
}
fn squared_error(score: i32, result: f64, k: f64) -> f64 {
    (probability(score, k) - result).powi(2)
}
fn load(path: &str, expected: &str) -> Result<Dataset> {
    let mut data = Dataset {
        samples: Vec::new(),
        games: HashSet::new(),
        positions: HashSet::new(),
    };
    for (index, line) in BufReader::new(File::open(path)?).lines().enumerate() {
        let row: Value = serde_json::from_str(&line?)?;
        let error = |message: &str| format!("{path}:{}: {message}", index + 1);
        if row.get("split").and_then(Value::as_str) != Some(expected) {
            return Err(error("unexpected split; fit never accepts heldout rows").into());
        }
        let fen = row
            .get("fen")
            .and_then(Value::as_str)
            .ok_or_else(|| error("missing FEN"))?;
        let result = row
            .get("result")
            .and_then(Value::as_f64)
            .ok_or_else(|| error("missing numeric result"))?;
        if ![0.0, 0.5, 1.0].contains(&result) {
            return Err(error("result must be White's native outcome: 0, 0.5 or 1").into());
        }
        let game = row
            .get("game_id")
            .filter(|v| v.is_string() || v.is_u64())
            .ok_or_else(|| error("game_id must be a string or unsigned integer"))?;
        // Evaluation sees the board; differing clock fields do not establish
        // independent positions. The producer must deduplicate before splitting.
        let key = fen.split_whitespace().take(4).collect::<Vec<_>>().join(" ");
        if !data.positions.insert(key) {
            return Err(error("duplicate board/side/rights position").into());
        }
        data.games.insert(game.to_string());
        let native = Position::from_fen(fen).map_err(|e| error(&format!("invalid FEN: {e:?}")))?;
        let position = Prepared::new(&ChessPosition::from_position(&native));
        data.samples.push(Sample { position, result });
    }
    if data.samples.is_empty() {
        return Err(format!("{path}: empty dataset").into());
    }
    Ok(data)
}
fn mse(data: &Dataset, weights: &Weights, k: f64) -> f64 {
    data.samples
        .iter()
        .map(|s| squared_error(s.position.score(weights), s.result, k))
        .sum::<f64>()
        / data.samples.len() as f64
}
fn penalty(weights: &Weights) -> f64 {
    REGULARIZATION
        * (0..COUNT)
            .map(|i| {
                let span = f64::from((UPPER[i] - LOWER[i]).max(1));
                (f64::from(weights.0[i] - INITIAL.0[i]) / span).powi(2)
            })
            .sum::<f64>()
        / COUNT as f64
}
fn objective(data: &Dataset, weights: &Weights, k: f64) -> f64 {
    mse(data, weights, k) + penalty(weights)
}
fn report(data: &Dataset, weights: &Weights, k: f64) -> Value {
    let mut sums = [0.0; 3];
    let mut counts = [0usize; 3];
    for sample in &data.samples {
        let phase = sample.position.phase();
        let bucket = if phase <= 1600 {
            0
        } else if phase <= 4800 {
            1
        } else {
            2
        };
        sums[bucket] += squared_error(sample.position.score(weights), sample.result, k);
        counts[bucket] += 1;
    }
    let by_phase: Vec<Value> = ["low_0_1600", "middle_1601_4800", "high_above_4800"]
        .iter()
        .enumerate()
        .map(|(i, label)| {
            json!({
                "phase": label, "positions": counts[i],
                "mse": if counts[i] == 0 { None } else { Some(sums[i] / counts[i] as f64) }
            })
        })
        .collect();
    json!({"positions": data.samples.len(), "games": data.games.len(),
        "mse": mse(data, weights, k), "by_phase": by_phase})
}
fn write_new(path: &str, value: &Value) -> Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    writeln!(file)?;
    Ok(())
}
fn fit(args: &[String]) -> Result<()> {
    if !(args.len() == 3 || args.len() == 4) {
        return Err("usage: chess_texel fit TRAIN VALIDATION OUTPUT [SECONDS<=600]".into());
    }
    if Path::new(&args[2]).exists() {
        return Err("output already exists".into());
    }
    let seconds: u64 = args.get(3).map_or(Ok(600), |x| x.parse())?;
    if !(1..=600).contains(&seconds) {
        return Err("seconds must be in 1..=600".into());
    }
    for i in 0..COUNT {
        if LOWER[i] > INITIAL.0[i] || INITIAL.0[i] > UPPER[i] {
            return Err(format!("initial parameter outside bounds: {}", NAMES[i]).into());
        }
    }
    let train = load(&args[0], "train")?;
    let validation = load(&args[1], "validation")?;
    if !train.games.is_disjoint(&validation.games)
        || !train.positions.is_disjoint(&validation.positions)
    {
        return Err("training and validation overlap by game or position".into());
    }
    let start = Instant::now();
    let limit = Duration::from_secs(seconds);
    // Calibrate only on INITIAL training predictions, then freeze K. Scaling
    // K and material together would otherwise leave a poorly identified fit.
    let mut k = 1.0;
    let mut calibration_loss = f64::INFINITY;
    let mut calibration = Vec::new();
    for tick in 5..=40 {
        if start.elapsed() >= limit {
            return Err("time limit reached during K calibration".into());
        }
        let trial_k = f64::from(tick) / 20.0;
        let loss = mse(&train, &INITIAL, trial_k);
        calibration.push(json!({"k": trial_k, "training_mse": loss}));
        if loss < calibration_loss {
            k = trial_k;
            calibration_loss = loss;
        }
    }
    let mut weights = INITIAL;
    let mut best = INITIAL;
    let mut best_validation = mse(&validation, &INITIAL, k);
    let initial_validation = best_validation;
    let mut current_objective = objective(&train, &weights, k);
    let mut stale = 0;
    let mut sweeps = Vec::new();
    let mut evaluations = calibration.len() + 2;
    let mut reason = "steps_complete";
    'steps: for step in STEPS {
        loop {
            let mut changes = 0;
            for i in 0..COUNT {
                if start.elapsed() >= limit {
                    reason = "time_limit";
                    break 'steps;
                }
                let original = weights.0[i];
                let mut selected = original;
                let mut selected_objective = current_objective;
                for direction in [-1, 1] {
                    if start.elapsed() >= limit {
                        reason = "time_limit";
                        break 'steps;
                    }
                    let value = (original + direction * step).clamp(LOWER[i], UPPER[i]);
                    if value == original {
                        continue;
                    }
                    let mut trial = weights;
                    trial.0[i] = value;
                    let loss = objective(&train, &trial, k);
                    evaluations += 1;
                    if loss < selected_objective - 1e-15 {
                        selected = value;
                        selected_objective = loss;
                    }
                }
                if selected != original {
                    weights.0[i] = selected;
                    current_objective = selected_objective;
                    changes += 1;
                }
            }
            // A partial sweep is never selected. Validation selects only among
            // completed sweeps optimized exclusively on training predictions.
            let val = mse(&validation, &weights, k);
            evaluations += 1;
            let selected = val < best_validation - 1e-15;
            if selected {
                best_validation = val;
                best = weights;
                stale = 0;
            } else if changes > 0 {
                stale += 1;
            }
            sweeps.push(json!({"step": step, "changes": changes,
                "training_objective": current_objective, "validation_mse": val,
                "selected": selected, "elapsed_seconds": start.elapsed().as_secs_f64()}));
            if stale >= 3 {
                reason = "validation_patience";
                break 'steps;
            }
            if changes == 0 {
                break;
            }
        }
    }
    write_new(
        &args[2],
        &json!({
            "schema": 1, "kind": "mshogi-chess-texel-fit", "names": NAMES,
            "weights": best.0, "initial": INITIAL.0, "lower": LOWER, "upper": UPPER,
            "k": k, "k_calibration": calibration, "regularization": REGULARIZATION,
            "regularization_definition": "mean squared displacement from INITIAL divided by parameter range squared",
            "objective": "training sigmoid MSE plus fixed regularization; validation selects completed sweeps",
            "train_path": args[0], "validation_path": args[1],
            "train_initial": report(&train, &INITIAL, k), "train_selected": report(&train, &best, k),
            "validation_initial_mse": initial_validation,
            "validation_selected": report(&validation, &best, k),
            "steps": STEPS, "sweeps": sweeps, "objective_evaluations": evaluations,
            "time_limit_seconds": seconds, "elapsed_seconds": start.elapsed().as_secs_f64(),
            "stop_reason": reason, "heldout_read": false
        }),
    )
}
fn frozen(path: &str) -> Result<(Weights, f64)> {
    let value: Value = serde_json::from_reader(File::open(path)?)?;
    if value.get("kind").and_then(Value::as_str) != Some("mshogi-chess-texel-fit")
        || value.get("names") != Some(&json!(NAMES))
    {
        return Err("weight file kind or parameter names do not match".into());
    }
    let values = value
        .get("weights")
        .and_then(Value::as_array)
        .ok_or("missing weights")?;
    if values.len() != COUNT {
        return Err("wrong parameter count".into());
    }
    let mut weights = INITIAL;
    for i in 0..COUNT {
        let number = values[i].as_i64().ok_or("weight must be an integer")?;
        weights.0[i] = i32::try_from(number)?;
        if !(LOWER[i]..=UPPER[i]).contains(&weights.0[i]) {
            return Err("weight outside bounds".into());
        }
    }
    let k = value.get("k").and_then(Value::as_f64).ok_or("missing K")?;
    if !k.is_finite() || !(0.25..=2.0).contains(&k) {
        return Err("K outside calibration range".into());
    }
    Ok((weights, k))
}
fn check(args: &[String]) -> Result<()> {
    if args.len() != 3 {
        return Err("usage: chess_texel check FROZEN HELDOUT OUTPUT".into());
    }
    if Path::new(&args[2]).exists() {
        return Err("output already exists".into());
    }
    let (weights, k) = frozen(&args[0])?;
    let data = load(&args[1], "heldout")?;
    write_new(
        &args[2],
        &json!({"schema": 1, "kind": "mshogi-chess-texel-heldout",
        "frozen_path": args[0], "heldout_path": args[1], "k": k,
        "initial": report(&data, &INITIAL, k), "frozen": report(&data, &weights, k)}),
    )
}
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (command, args) = args.split_first().ok_or("expected fit or check")?;
    match command.as_str() {
        "fit" => fit(args),
        "check" => check(args),
        _ => Err("expected fit or check".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sigmoid_and_error_have_independent_expected_values() {
        assert_eq!(probability(0, 1.0), 0.5);
        assert!((probability(400, 1.0) - 10.0 / 11.0).abs() < 1e-12);
        assert!((probability(-400, 1.0) - 1.0 / 11.0).abs() < 1e-12);
        assert_eq!(squared_error(0, 0.0, 1.0), 0.25);
        assert_eq!(squared_error(0, 0.5, 1.0), 0.0);
        assert!((probability(117, 0.8) + probability(-117, 0.8) - 1.0).abs() < 1e-12);
        assert_eq!(penalty(&INITIAL), 0.0);
    }
}

#[cfg(test)]
mod input_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    fn temporary(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "msh168-texel-{label}-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ))
    }
    #[test]
    fn dataset_rejects_heldout_labels_bad_results_and_duplicate_boards() {
        let path = temporary("rows");
        let text = path.to_str().unwrap();
        let mut row = json!({"fen":"7k/8/8/8/3K4/8/8/8 w - - 0 1",
            "result":0.5,"split":"heldout","game_id":"g1"});
        std::fs::write(&path, format!("{row}\n")).unwrap();
        assert!(load(text, "train").is_err());
        row["split"] = json!("train");
        row["result"] = json!(0.3);
        std::fs::write(&path, format!("{row}\n")).unwrap();
        assert!(load(text, "train").is_err());
        row["result"] = json!(0.5);
        std::fs::write(&path, format!("{row}\n")).unwrap();
        assert_eq!(load(text, "train").unwrap().samples.len(), 1);
        let mut duplicate = row.clone();
        duplicate["fen"] = json!("7k/8/8/8/3K4/8/8/8 w - - 4 3");
        std::fs::write(&path, format!("{row}\n{duplicate}\n")).unwrap();
        assert!(load(text, "train").is_err());
        row["fen"] = json!("not a FEN");
        std::fs::write(&path, format!("{row}\n")).unwrap();
        assert!(load(text, "train").is_err());
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn frozen_weights_reject_wrong_names_bounds_and_calibration() {
        let path = temporary("weights");
        let text = path.to_str().unwrap();
        let valid = json!({"kind":"mshogi-chess-texel-fit","names":NAMES,
            "weights":INITIAL.0,"k":1.0});
        std::fs::write(&path, valid.to_string()).unwrap();
        assert_eq!(frozen(text).unwrap().0.0, INITIAL.0);
        let mut invalid = valid.clone();
        invalid["weights"][0] = json!(UPPER[0] + 1);
        std::fs::write(&path, invalid.to_string()).unwrap();
        assert!(frozen(text).is_err());
        invalid = valid.clone();
        invalid["k"] = json!(0.0);
        std::fs::write(&path, invalid.to_string()).unwrap();
        assert!(frozen(text).is_err());
        invalid = valid;
        invalid["names"][0] = json!("different");
        std::fs::write(&path, invalid.to_string()).unwrap();
        assert!(frozen(text).is_err());
        std::fs::remove_file(path).unwrap();
    }
}
