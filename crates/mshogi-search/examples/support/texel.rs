//! Shared Texel fitting tool for the classical evaluations of all games.
//! Included by each game's `*_texel` example; the model trait lives in
//! `mshogi_search::texel`. Corpus rows are JSON lines `{POSITION_FIELD,
//! "result", "split", "game_id"}`; `fit` reads only training and validation
//! rows and `check` reads only held-out rows.
#![allow(dead_code)]
pub use mshogi_search::texel::{TexelModel, probability};
use serde_json::{Value, json};
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::time::{Duration, Instant};

/// Boxed error of the tool.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const REGULARIZATION: f64 = 1e-5;

struct Sample<P> {
    position: P,
    result: f64,
}

/// Loaded rows of one split.
pub struct Dataset<P> {
    samples: Vec<Sample<P>>,
    games: HashSet<String>,
    positions: HashSet<String>,
    skipped: usize,
}

impl<P> Dataset<P> {
    /// Number of retained samples.
    pub fn len(&self) -> usize {
        self.samples.len()
    }
    /// True if no sample was retained.
    pub fn is_empty(&self) -> bool {
        self.samples.is_empty()
    }
}

fn squared_error(score: i32, result: f64, k: f64) -> f64 {
    (probability(score, k) - result).powi(2)
}

/// Load one split; rows of another split are an error.
pub fn load<M: TexelModel>(path: &str, expected: &str) -> Result<Dataset<M::Prepared>> {
    let mut data = Dataset {
        samples: Vec::new(),
        games: HashSet::new(),
        positions: HashSet::new(),
        skipped: 0,
    };
    for (index, line) in BufReader::new(File::open(path)?).lines().enumerate() {
        let row: Value = serde_json::from_str(&line?)?;
        let error = |message: &str| format!("{path}:{}: {message}", index + 1);
        if row.get("split").and_then(Value::as_str) != Some(expected) {
            return Err(error("unexpected split; fit never accepts heldout rows").into());
        }
        let text = row
            .get(M::POSITION_FIELD)
            .and_then(Value::as_str)
            .ok_or_else(|| error("missing position"))?;
        let result = row
            .get("result")
            .and_then(Value::as_f64)
            .ok_or_else(|| error("missing numeric result"))?;
        if ![0.0, 0.5, 1.0].contains(&result) {
            return Err(
                error("result must be the first player's native outcome: 0, 0.5 or 1").into(),
            );
        }
        let game = row
            .get("game_id")
            .filter(|v| v.is_string() || v.is_u64())
            .ok_or_else(|| error("game_id must be a string or unsigned integer"))?;
        // Evaluation sees the board; differing clock fields do not establish
        // independent positions. The producer must deduplicate before splitting.
        if !data.positions.insert(M::key(text)) {
            return Err(error("duplicate board/side/rights position").into());
        }
        let Some(position) =
            M::prepare(text).map_err(|e| error(&format!("invalid position: {e}")))?
        else {
            data.skipped += 1;
            continue;
        };
        data.games.insert(game.to_string());
        data.samples.push(Sample { position, result });
    }
    if data.samples.is_empty() {
        return Err(format!("{path}: empty dataset").into());
    }
    Ok(data)
}

/// Mean squared error of the logistic prediction, computed in parallel.
pub fn mse<M: TexelModel>(data: &Dataset<M::Prepared>, weights: &[i32], k: f64) -> f64 {
    // A fixed chunk count keeps the floating-point sum order, and therefore
    // fit selection, independent of the host's CPU count.
    let chunk = data.samples.len().div_ceil(16).max(1);
    let total: f64 = std::thread::scope(|scope| {
        let handles: Vec<_> = data
            .samples
            .chunks(chunk)
            .map(|part| {
                scope.spawn(move || {
                    part.iter()
                        .map(|s| squared_error(M::score(&s.position, weights), s.result, k))
                        .sum::<f64>()
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    });
    total / data.samples.len() as f64
}
/// Regularization term of a weight vector.
pub fn penalty<M: TexelModel>(weights: &[i32]) -> f64 {
    let (initial, lower, upper) = (M::initial(), M::lower(), M::upper());
    REGULARIZATION
        * (0..weights.len())
            .map(|i| {
                let span = f64::from((upper[i] - lower[i]).max(1));
                (f64::from(weights[i] - initial[i]) / span).powi(2)
            })
            .sum::<f64>()
        / weights.len() as f64
}
fn objective<M: TexelModel>(data: &Dataset<M::Prepared>, weights: &[i32], k: f64) -> f64 {
    mse::<M>(data, weights, k) + penalty::<M>(weights)
}
fn report<M: TexelModel>(data: &Dataset<M::Prepared>, weights: &[i32], k: f64) -> Value {
    let labels = M::buckets();
    let mut sums = vec![0.0; labels.len()];
    let mut counts = vec![0usize; labels.len()];
    for sample in &data.samples {
        let bucket = M::bucket(&sample.position);
        sums[bucket] += squared_error(M::score(&sample.position, weights), sample.result, k);
        counts[bucket] += 1;
    }
    let by_bucket: Vec<Value> = labels
        .iter()
        .enumerate()
        .map(|(i, label)| {
            json!({"phase": label, "positions": counts[i],
                "mse": if counts[i] == 0 { None } else { Some(sums[i] / counts[i] as f64) }})
        })
        .collect();
    json!({"positions": data.samples.len(), "games": data.games.len(), "skipped": data.skipped,
        "mse": mse::<M>(data, weights, k), "by_phase": by_bucket})
}
fn write_new(path: &str, value: &Value) -> Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    serde_json::to_writer_pretty(&mut file, value)?;
    writeln!(file)?;
    Ok(())
}

/// `fit TRAIN VALIDATION OUTPUT [SECONDS]`: K calibration on initial training
/// predictions, then bounded coordinate descent; validation selects among
/// completed sweeps.
pub fn fit<M: TexelModel>(args: &[String], max_seconds: u64) -> Result<()> {
    if !(args.len() == 3 || args.len() == 4) {
        return Err(format!("usage: fit TRAIN VALIDATION OUTPUT [SECONDS<={max_seconds}]").into());
    }
    if Path::new(&args[2]).exists() {
        return Err("output already exists".into());
    }
    let seconds: u64 = args.get(3).map_or(Ok(max_seconds), |x| x.parse())?;
    if !(1..=max_seconds).contains(&seconds) {
        return Err(format!("seconds must be in 1..={max_seconds}").into());
    }
    let (initial, lower, upper, names) = (M::initial(), M::lower(), M::upper(), M::names());
    let count = names.len();
    for i in 0..count {
        if lower[i] > initial[i] || initial[i] > upper[i] {
            return Err(format!("initial parameter outside bounds: {}", names[i]).into());
        }
    }
    let train = load::<M>(&args[0], "train")?;
    let validation = load::<M>(&args[1], "validation")?;
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
    let (k_low, k_high) = M::K_RANGE;
    for tick in 0..=35 {
        if start.elapsed() >= limit {
            return Err("time limit reached during K calibration".into());
        }
        let trial_k = k_low + (k_high - k_low) * f64::from(tick) / 35.0;
        let loss = mse::<M>(&train, &initial, trial_k);
        calibration.push(json!({"k": trial_k, "training_mse": loss}));
        if loss < calibration_loss {
            k = trial_k;
            calibration_loss = loss;
        }
    }
    let mut weights = initial.clone();
    let mut best = initial.clone();
    let mut best_validation = mse::<M>(&validation, &initial, k);
    let initial_validation = best_validation;
    let mut current_objective = objective::<M>(&train, &weights, k);
    let mut stale = 0;
    let mut sweeps = Vec::new();
    let mut evaluations = calibration.len() + 2;
    let mut reason = "steps_complete";
    'steps: for &step in M::STEPS {
        loop {
            let mut changes = 0;
            for i in 0..count {
                if start.elapsed() >= limit {
                    reason = "time_limit";
                    break 'steps;
                }
                let original = weights[i];
                let mut selected = original;
                let mut selected_objective = current_objective;
                for direction in [-1, 1] {
                    if start.elapsed() >= limit {
                        reason = "time_limit";
                        break 'steps;
                    }
                    let value = (original + direction * step).clamp(lower[i], upper[i]);
                    if value == original {
                        continue;
                    }
                    let mut trial = weights.clone();
                    trial[i] = value;
                    let loss = objective::<M>(&train, &trial, k);
                    evaluations += 1;
                    if loss < selected_objective - 1e-15 {
                        selected = value;
                        selected_objective = loss;
                    }
                }
                if selected != original {
                    weights[i] = selected;
                    current_objective = selected_objective;
                    changes += 1;
                }
            }
            // A partial sweep is never selected. Validation selects only among
            // completed sweeps optimized exclusively on training predictions.
            let val = mse::<M>(&validation, &weights, k);
            evaluations += 1;
            let selected = val < best_validation - 1e-15;
            if selected {
                best_validation = val;
                best = weights.clone();
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
            "schema": 1, "kind": format!("mshogi-{}-texel-fit", M::KIND), "names": names,
            "weights": best, "initial": initial, "lower": lower, "upper": upper,
            "k": k, "k_calibration": calibration, "regularization": REGULARIZATION,
            "regularization_definition": "mean squared displacement from INITIAL divided by parameter range squared",
            "objective": "training sigmoid MSE plus fixed regularization; validation selects completed sweeps",
            "train_path": args[0], "validation_path": args[1],
            "train_initial": report::<M>(&train, &initial, k), "train_selected": report::<M>(&train, &best, k),
            "validation_initial_mse": initial_validation,
            "validation_selected": report::<M>(&validation, &best, k),
            "steps": M::STEPS, "sweeps": sweeps, "objective_evaluations": evaluations,
            "time_limit_seconds": seconds, "elapsed_seconds": start.elapsed().as_secs_f64(),
            "stop_reason": reason, "heldout_read": false
        }),
    )
}

/// Read a frozen fit for this model, checking names, bounds and K.
pub fn frozen<M: TexelModel>(path: &str) -> Result<(Vec<i32>, f64)> {
    let value: Value = serde_json::from_reader(File::open(path)?)?;
    if value.get("kind").and_then(Value::as_str)
        != Some(format!("mshogi-{}-texel-fit", M::KIND).as_str())
        || value.get("names") != Some(&json!(M::names()))
    {
        return Err("weight file kind or parameter names do not match".into());
    }
    let values = value
        .get("weights")
        .and_then(Value::as_array)
        .ok_or("missing weights")?;
    let (lower, upper) = (M::lower(), M::upper());
    if values.len() != lower.len() {
        return Err("wrong parameter count".into());
    }
    let mut weights = Vec::with_capacity(values.len());
    for (i, v) in values.iter().enumerate() {
        let number = i32::try_from(v.as_i64().ok_or("weight must be an integer")?)?;
        if !(lower[i]..=upper[i]).contains(&number) {
            return Err("weight outside bounds".into());
        }
        weights.push(number);
    }
    let k = value.get("k").and_then(Value::as_f64).ok_or("missing K")?;
    if !k.is_finite() || !(M::K_RANGE.0..=M::K_RANGE.1).contains(&k) {
        return Err("K outside calibration range".into());
    }
    Ok((weights, k))
}

/// `check FROZEN HELDOUT OUTPUT`: held-out loss of the initial and frozen weights.
pub fn check<M: TexelModel>(args: &[String]) -> Result<()> {
    if args.len() != 3 {
        return Err("usage: check FROZEN HELDOUT OUTPUT".into());
    }
    if Path::new(&args[2]).exists() {
        return Err("output already exists".into());
    }
    let (weights, k) = frozen::<M>(&args[0])?;
    let data = load::<M>(&args[1], "heldout")?;
    let initial = M::initial();
    write_new(
        &args[2],
        &json!({"schema": 1, "kind": format!("mshogi-{}-texel-heldout", M::KIND),
        "frozen_path": args[0], "heldout_path": args[1], "k": k,
        "initial": report::<M>(&data, &initial, k), "frozen": report::<M>(&data, &weights, k)}),
    )
}

/// `stats FILE...`: rows, retained (quiet) positions, label balance and
/// duplicate boards per split, without fitting.
pub fn stats<M: TexelModel>(args: &[String]) -> Result<()> {
    let mut out = Vec::new();
    for path in args {
        let mut rows = 0usize;
        let mut kept = 0usize;
        let mut duplicates = 0usize;
        let mut labels = [0usize; 3];
        let mut games = HashSet::new();
        let mut keys = HashSet::new();
        let mut splits = std::collections::BTreeMap::new();
        for line in BufReader::new(File::open(path)?).lines() {
            let row: Value = serde_json::from_str(&line?)?;
            rows += 1;
            let text = row
                .get(M::POSITION_FIELD)
                .and_then(Value::as_str)
                .ok_or("missing position")?;
            if !keys.insert(M::key(text)) {
                duplicates += 1;
                continue;
            }
            if M::prepare(text)?.is_none() {
                continue;
            }
            kept += 1;
            let result = row
                .get("result")
                .and_then(Value::as_f64)
                .ok_or("missing result")?;
            labels[(result * 2.0).round() as usize] += 1;
            games.insert(
                row.get("game_id")
                    .map(|g| g.to_string())
                    .unwrap_or_default(),
            );
            *splits
                .entry(
                    row.get("split")
                        .and_then(Value::as_str)
                        .unwrap_or("none")
                        .to_string(),
                )
                .or_insert(0usize) += 1;
        }
        out.push(json!({"path": path, "rows": rows, "duplicate_rows": duplicates, "retained": kept,
            "games_with_retained": games.len(), "first_player_loss_draw_win": labels, "retained_by_split": splits}));
    }
    println!("{}", serde_json::to_string_pretty(&out)?);
    Ok(())
}

/// Command-line entry: `fit ...`, `check ...` or `stats ...`.
pub fn main<M: TexelModel>(max_seconds: u64) -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (command, args) = args.split_first().ok_or("expected fit or check")?;
    match command.as_str() {
        "fit" => fit::<M>(args, max_seconds),
        "check" => check::<M>(args),
        "stats" => stats::<M>(args),
        _ => Err("expected fit, check or stats".into()),
    }
}
