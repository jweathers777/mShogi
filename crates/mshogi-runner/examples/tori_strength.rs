//! MSH-204 (Tori Strength Harness): tactical-suite mining and answer sets,
//! suite probes, late-game depth and Texel export, all under native Tori
//! authority. Inputs are native experiment reports (`mshogi-match
//! experiment-run`, uncompressed), validated by native replay.
//!
//! Usage:
//!   tori_strength mine REPORTS.json OUT.jsonl        (REPORTS: list of report paths)
//!   tori_strength finalize SUITE.json OUT.json
//!   tori_strength probe SUITE.json OUT.json TIME_MS [STORE.json]
//!   tori_strength prove SUITE.json OUT.json STORE.json PROBE.json [PROBE.json ...]
//!   tori_strength depth REPORTS.json OUT.json TIME_MS MIN_PLY
//!   tori_strength export REPORTS.json OUT.jsonl EVERY
//!   tori_strength movegen REPORTS.json OUT.json REPEATS SUITE.json OPENINGS.json
//!   tori_strength judge REPORTS.json OUT.json NODES
use mshogi_core::{native::CanonicalRecord, tori::*};
use mshogi_runner::experiments::ValidatedReport;
use mshogi_search::{alphabeta::timing::TimeBudget, tori as search};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, sync::Mutex, thread};
#[path = "support/tori_mate_proof.rs"]
mod tori_mate_proof;
use tori_mate_proof::{MAX_PROOF_PLIES, Prover, successors};

/// Proof visit bound of one mined position.
const PROOF_VISITS: u64 = 400_000;
/// Prover identity in stored proof keys; change it when the prover changes.
const PROVER_VERSION: &str = "tori-mate-proof-v2";
/// Full-width proof visit bound of one played action.
const FULL_VISITS: u64 = 20_000_000;

fn strings(v: &Value) -> Vec<String> {
    v.as_array()
        .map(|a| a.iter().map(|m| m.as_str().unwrap().to_string()).collect())
        .unwrap_or_default()
}

/// Run `f` over `items` on all cores, keeping input order.
fn parallel<T: Sync, R: Send>(items: &[T], f: impl Fn(&T) -> R + Sync) -> Vec<R> {
    let threads = thread::available_parallelism().map_or(4, |n| n.get());
    let next = Mutex::new(0usize);
    let out: Mutex<Vec<Option<R>>> = Mutex::new((0..items.len()).map(|_| None).collect());
    thread::scope(|s| {
        for _ in 0..threads {
            s.spawn(|| {
                loop {
                    let i = {
                        let mut n = next.lock().unwrap();
                        let i = *n;
                        *n += 1;
                        i
                    };
                    if i >= items.len() {
                        break;
                    }
                    let r = f(&items[i]);
                    out.lock().unwrap()[i] = Some(r);
                }
            });
        }
    });
    out.into_inner()
        .unwrap()
        .into_iter()
        .map(Option::unwrap)
        .collect()
}

/// One validated game: its native record and the event count of its opening.
struct Source {
    digest: String,
    game_index: usize,
    opening_id: String,
    game: Game,
    start_events: usize,
    stop: String,
    /// Mover's clock before each board ply after the opening, when clocked.
    clocks: Vec<Option<u64>>,
}

fn tori_record(record: &CanonicalRecord) -> &GameRecord {
    match record {
        CanonicalRecord::Tori(r) => r,
        _ => panic!("not a Tori report"),
    }
}

/// The games of all reports in `paths`, in list order. Report validation
/// replays every game, so the reports load in parallel.
fn load(paths: &Value) -> Vec<Source> {
    parallel(&strings(paths), |p| sources(p))
        .into_iter()
        .flatten()
        .collect()
}

fn sources(path: &str) -> Vec<Source> {
    let text = fs::read_to_string(path).unwrap();
    let digest = format!("{:x}", Sha256::digest(text.as_bytes()));
    let report = ValidatedReport::from_json(&text).expect("native replay of the report");
    report
        .games
        .iter()
        .enumerate()
        .map(|(game_index, g)| {
            let opening = &report.config.openings[g.pairing.opening];
            Source {
                digest: digest.clone(),
                game_index,
                opening_id: opening.id.clone(),
                game: tori_record(&g.final_root.record).to_game().unwrap(),
                start_events: tori_record(&opening.root.record).event_count(),
                stop: g.stop.clone(),
                clocks: g
                    .steps
                    .iter()
                    .filter(|step| step.applied)
                    .map(|step| step.clock.as_ref().map(|c| c.before_ms))
                    .collect(),
            }
        })
        .collect()
}

/// Board plies after the opening: (event index before the action, action).
fn plays(source: &Source) -> Vec<(usize, Action)> {
    source.game.events()[source.start_events..]
        .iter()
        .filter_map(|e| match e.command {
            GameCommand::Play { action } => Some((e.index, action)),
            _ => None,
        })
        .collect()
}

/// Forced checking wins of 3..=7 plies before each move after the opening.
fn mine(paths: &Value) -> String {
    let all: Vec<Source> = load(paths);
    let rows = parallel(&all, |s| {
        let mut out = vec![];
        for (event, played) in plays(s) {
            let snapshot = s.game.event_prefix(event).unwrap();
            let local = snapshot.local_position();
            let mut prover = Prover::new(PROOF_VISITS);
            if let Ok(Some(plies)) = prover.shortest(&local, MAX_PROOF_PLIES)
                && plies >= 3
                && let Ok(answers) = Prover::new(PROOF_VISITS).answer_set(&local, plies)
            {
                let played = notation::format_action(local.position(), played).unwrap();
                let expected: Vec<String> = answers
                    .iter()
                    .map(|a| notation::format_action(local.position(), *a).unwrap())
                    .collect();
                out.push(
                    json!({"diagram": local.position().compact_diagram(), "mate_plies": plies,
                    "checking_answers": expected, "played": played,
                    "played_in_answers": expected.contains(&played),
                    "source_report_sha256": s.digest, "game": s.game_index,
                    "opening_id": s.opening_id, "event": event}),
                );
            }
        }
        out
    });
    let mut text = String::new();
    for row in rows.into_iter().flatten() {
        text.push_str(&row.to_string());
        text.push('\n');
    }
    text
}

/// Final suite rows: rows where the side to move has a one-ply win (any action
/// that leaves the defender without a legal reply) are dropped and counted.
/// The stored answers stay the checking answers from mining; a move outside
/// them is proved or refuted when it is played (`probe`).
fn finalize(spec: &Value) -> Value {
    let rows = spec["positions"].as_array().unwrap();
    let done = parallel(rows, |r| {
        let local =
            LocalPosition::admit(Position::from_compact_diagram(&r["diagram"]).unwrap()).unwrap();
        let one_ply = successors(&local, false)
            .iter()
            .any(|(_, next)| next.legal_actions().is_empty());
        if one_ply {
            Err("one_ply_win")
        } else {
            Ok(r.clone())
        }
    });
    let mut out = spec.clone();
    out["dropped_one_ply_win"] = json!(done.iter().filter(|r| r.is_err()).count());
    out["positions"] = json!(done.into_iter().flatten().collect::<Vec<_>>());
    out
}

/// Whether `action` wins within `plies` with a full-width attacker, from the
/// native prover: `Some(true)` proved, `Some(false)` refuted, `None` beyond
/// the proof bound.
fn wins(position: &Position, action: Action, plies: u8) -> Option<bool> {
    prove_one(position, action, plies).0
}

/// `wins` with the prover's visit count.
fn prove_one(position: &Position, action: Action, plies: u8) -> (Option<bool>, u64) {
    let mut after = LocalPosition::admit(position.clone()).unwrap();
    if after.apply(action).is_err() {
        return (None, 0);
    }
    let mut prover = Prover::full(FULL_VISITS);
    let result = prover.defender_mated(&after, plies - 1).ok();
    (result, prover.visits)
}

/// Stored proof results: key `sha256(canonical diagram)|action|mate_plies|
/// visit bound|prover version`. A stored
/// result is reused by later probes of any budget or engine version (the
/// suite and the native rules decide it, not the engine).
fn store_key(diagram: &Value, action: &str, plies: u8) -> String {
    // serde_json maps keep sorted keys: compact text is canonical JSON.
    let digest = Sha256::digest(serde_json::to_string(diagram).unwrap().as_bytes());
    format!("{digest:x}|{action}|{plies}|{FULL_VISITS}|{PROVER_VERSION}")
}

fn read_store(path: Option<&String>) -> serde_json::Map<String, Value> {
    path.and_then(|p| fs::read_to_string(p).ok())
        .map(|t| {
            serde_json::from_str::<Value>(&t).unwrap()["proofs"]
                .as_object()
                .unwrap()
                .clone()
        })
        .unwrap_or_default()
}

fn write_store(path: &str, proofs: serde_json::Map<String, Value>) {
    let text = serde_json::to_string_pretty(&json!({
        "rule": "native full-width prover; verdict proved_win | refuted | unknown (visit bound)",
        "visit_bound": FULL_VISITS, "proofs": proofs}))
    .unwrap();
    let tmp = format!("{path}.tmp");
    fs::write(&tmp, text).unwrap();
    fs::rename(tmp, path).unwrap();
}

/// Prove every unlisted action of earlier probe results; record verdict,
/// visits and time of each proof (benchmark and store seed).
fn prove(spec: &Value, probes: &[Value], store: Option<&String>) -> Value {
    let rows: std::collections::HashMap<String, &Value> = spec["positions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| (r["id"].as_str().unwrap().to_string(), r))
        .collect();
    let mut known = read_store(store);
    let mut todo = vec![];
    for probe in probes {
        for r in probe["results"].as_array().unwrap() {
            let id = r["id"].as_str().unwrap().to_string();
            let chosen = r["chosen"].as_str().unwrap().to_string();
            let row = rows[&id];
            let plies = row["mate_plies"].as_u64().unwrap() as u8;
            let key = store_key(&row["diagram"], &chosen, plies);
            if strings(&row["checking_answers"]).contains(&chosen) || known.contains_key(&key) {
                continue;
            }
            if !todo
                .iter()
                .any(|(k, _, _, _): &(String, _, _, _)| *k == key)
            {
                todo.push((key, id, chosen, plies));
            }
        }
    }
    let done = parallel(&todo, |(key, id, chosen, plies)| {
        let position = Position::from_compact_diagram(&rows[id]["diagram"]).unwrap();
        let action = notation::parse_action(&position, chosen).unwrap();
        let start = std::time::Instant::now();
        let (result, visits) = prove_one(&position, action, *plies);
        let verdict = match result {
            Some(true) => "proved_win",
            Some(false) => "refuted",
            None => "unknown",
        };
        (
            key.clone(),
            json!({"verdict": verdict, "visits": visits,
                             "ms": start.elapsed().as_millis() as u64}),
        )
    });
    let mut report = vec![];
    for (key, value) in done {
        report.push(json!({"key": key, "result": value}));
        known.insert(key, value);
    }
    if let Some(path) = store {
        write_store(path, known);
    }
    json!({"proved": report.len(), "results": report})
}

/// Fresh-engine timed search of each suite position. A pass plays a winning
/// action: one of the stored checking answers, or any other action that the
/// native full-width prover proves wins within the mate length.
fn probe(spec: &Value, ms: u64, store: Option<&String>) -> Value {
    let known = read_store(store);
    let rows = spec["positions"].as_array().unwrap();
    let results = parallel(rows, |r| {
        let game = Game::from_root(Position::from_compact_diagram(&r["diagram"]).unwrap()).unwrap();
        let result = search::search_with_state(
            &game,
            search::Limits {
                depth: search::MAX_DEPTH,
                nodes: search::MAX_NODES,
            },
            Some(TimeBudget::from_hard_ms(ms)),
            &mut search::new_engine(),
            || false,
        )
        .unwrap();
        let plies = r["mate_plies"].as_u64().unwrap() as u8;
        let (chosen, verdict) = match result.decision {
            Some(search::Decision::Choose(a)) => {
                let text = notation::format_action(game.position(), a).unwrap();
                let verdict = if strings(&r["checking_answers"]).contains(&text) {
                    "listed"
                } else if let Some(v) = known
                    .get(&store_key(&r["diagram"], &text, plies))
                    .and_then(|v| v["verdict"].as_str())
                {
                    match v {
                        "proved_win" => "proved_win",
                        "refuted" => "refuted",
                        _ => "unknown",
                    }
                } else {
                    match wins(game.position(), a, plies) {
                        Some(true) => "proved_win",
                        Some(false) => "refuted",
                        None => "unknown",
                    }
                };
                (text, verdict)
            }
            other => (format!("{other:?}"), "no_move"),
        };
        let pass = matches!(verdict, "listed" | "proved_win");
        json!({"id": r["id"], "split": r["split"], "mate_plies": plies, "chosen": chosen,
               "verdict": verdict, "pass": pass, "score": result.score,
               "depth": result.completed_depth, "nodes": result.nodes})
    });
    let count = |split: Option<&str>| {
        let selected: Vec<_> = results
            .iter()
            .filter(|r| split.is_none_or(|s| r["split"] == s))
            .collect();
        let verdict = |v: &str| selected.iter().filter(|r| r["verdict"] == v).count();
        json!({"positions": selected.len(), "passed": selected.iter().filter(|r| r["pass"] == true).count(),
               "proved_quiet_or_other_win": verdict("proved_win"), "unknown": verdict("unknown")})
    };
    json!({"evaluator": search::EVALUATOR_ID, "search": search::SEARCH_ID, "time_ms": ms,
           "all": count(None), "development": count(Some("development")),
           "held_out": count(Some("held-out")), "results": results})
}

/// Completed depth of a fresh timed search (with the game's full history) on
/// every fourth position from board ply `min_ply` after the opening.
fn depth(paths: &Value, ms: u64, min_ply: usize) -> Value {
    let all: Vec<Source> = load(paths);
    let mut positions = vec![];
    for (i, s) in all.iter().enumerate() {
        for (ply, (event, _)) in plays(s).into_iter().enumerate() {
            if ply >= min_ply && ply % 4 == 0 {
                positions.push((i, event));
            }
        }
    }
    let results = parallel(&positions, |&(i, event)| {
        let game = all[i].game.record_prefix(event).unwrap().to_game().unwrap();
        let r = search::search_with_state(
            &game,
            search::Limits {
                depth: search::MAX_DEPTH,
                nodes: search::MAX_NODES,
            },
            Some(TimeBudget::from_hard_ms(ms)),
            &mut search::new_engine(),
            || false,
        )
        .unwrap();
        (r.completed_depth, r.nodes)
    });
    let mut depths: Vec<u16> = results.iter().map(|r| r.0).collect();
    depths.sort_unstable();
    let nodes: u64 = results.iter().map(|r| r.1).sum();
    json!({"time_ms": ms, "min_ply": min_ply, "positions": depths.len(),
           "median_depth": depths.get(depths.len() / 2), "p10_depth": depths.get(depths.len() / 10),
           "mean_nodes": nodes / (results.len().max(1) as u64),
           "threads": thread::available_parallelism().map_or(4, |n| n.get())})
}

/// MSH-207 (Tori Strength Assessment) judge: a fresh `NODES` search (with the
/// game's full history) of every position after the opening, including the
/// final one, with the facts the blunder classes need. Scores are from the
/// side to move; a concluded position has its native result instead.
fn judge(paths: &Value, nodes: u64) -> Value {
    let all: Vec<Source> = load(paths);
    let mut positions = vec![];
    for (i, s) in all.iter().enumerate() {
        let plays = plays(s);
        // One clock per applied step; only a final non-play step can follow the plays.
        assert!(s.clocks.len() == plays.len() || s.clocks.len() == plays.len() + 1);
        for k in 0..=plays.len() {
            positions.push((i, k, plays.get(k).copied()));
        }
    }
    let rows = parallel(&positions, |&(i, k, play)| {
        let s = &all[i];
        let game = match play {
            Some((event, _)) => s.game.record_prefix(event).unwrap().to_game().unwrap(),
            None => {
                let count = s.game.events().len();
                s.game.record_prefix(count).unwrap().to_game().unwrap()
            }
        };
        let position = game.position();
        let text = |p: &Position, a: Action| notation::format_action(p, a).unwrap();
        let victim = |p: &Position, a: Action| match a {
            Action::Move { to, .. } => p.piece(to).map(|v| v.face.code()),
            Action::Drop { .. } => None,
        };
        let mut row = json!({"ply": k, "side": if position.side_to_move() == Side::Sente { "s" } else { "g" },
                             "diagram": position.compact_diagram()});
        if let Some(conclusion) = game.conclusion() {
            row["terminal"] = json!({"lost": winner(conclusion) != position.side_to_move()});
            return row;
        }
        let r = search::search(
            &game,
            search::Limits {
                depth: search::MAX_DEPTH,
                nodes,
            },
        )
        .unwrap();
        row["score"] = json!(r.score);
        row["depth"] = json!(r.completed_depth);
        row["nodes"] = json!(r.nodes);
        if let Some(&best) = r.principal_variation.first() {
            row["best"] = json!(text(position, best));
            row["best_captures"] = json!(victim(position, best));
            row["best_drop"] = json!(matches!(best, Action::Drop { .. }));
        }
        if let Some((_, played)) = play {
            row["played"] = json!(text(position, played));
            row["played_captures"] = json!(victim(position, played));
            row["clock_ms"] = json!(s.clocks.get(k).copied().flatten());
        }
        row
    });
    let mut games: Vec<Value> = all
        .iter()
        .map(|s| {
            json!({"game_id": format!("{}/{}", s.digest, s.game_index), "opening_id": s.opening_id,
                        "stop": s.stop, "positions": []})
        })
        .collect();
    for (&(i, _, _), row) in positions.iter().zip(rows) {
        games[i]["positions"].as_array_mut().unwrap().push(row);
    }
    json!({"nodes": nodes, "evaluator_id": search::EVALUATOR_ID, "search_id": search::SEARCH_ID,
           "games": games})
}

/// Sente's native result in half points / 2; None unless natively concluded.
fn sente_outcome(s: &Source) -> Option<f64> {
    if s.stop != "native_concluded" {
        return None;
    }
    Some(if winner(s.game.conclusion()?) == Side::Sente {
        1.0
    } else {
        0.0
    })
}

/// The winner of a native conclusion (Tori has no draw).
fn winner(conclusion: Conclusion) -> Side {
    match conclusion {
        Conclusion::Checkmate { winner } | Conclusion::Resignation { winner } => winner,
        Conclusion::StalemateLoss { loser } | Conclusion::RepetitionForfeit { loser } => {
            loser.opponent()
        }
    }
}

/// Native-labelled Texel rows: one JSON line per game, with every `every`-th
/// position after the opening. Unscored games keep no samples.
fn export(paths: &Value, every: usize) -> String {
    let all: Vec<Source> = load(paths);
    let rows = parallel(&all, |s| {
        let outcome = sente_outcome(s);
        let mut samples = vec![];
        if let Some(outcome) = outcome {
            for (ply, (event, _)) in plays(s).into_iter().enumerate() {
                if ply % every == 0 {
                    let snapshot = s.game.event_prefix(event).unwrap();
                    samples.push(
                        json!({"diagram": snapshot.local_position().position().compact_diagram(),
                                        "sente_outcome": outcome, "ply": ply}),
                    );
                }
            }
        }
        json!({"game_id": format!("{}/{}", s.digest, s.game_index), "source_report_sha256": s.digest,
               "opening_id": s.opening_id, "sente_outcome": outcome,
               "outcome": outcome.and(s.game.conclusion()), "stop": s.stop,
               "truncated": outcome.is_none(), "samples": samples})
    });
    let mut text = String::new();
    for row in rows {
        text.push_str(&row.to_string());
        text.push('\n');
    }
    text
}

/// Move-generation cost profile (MSH-200, Tori Move Generation Deep Dive) on
/// three position sets: real game positions (every 4th ply from ply 20 of the
/// reports), the suite diagrams and the final opening positions; plus the
/// fixed-depth search speed on each set.
fn movegen(paths: &Value, repeats: u32, suite: &str, openings: &str) -> Value {
    let mut games = vec![];
    for s in load(paths) {
        for (ply, (event, _)) in plays(&s).into_iter().enumerate() {
            if ply >= 20 && ply % 4 == 0 {
                games.push(
                    s.game
                        .event_prefix(event)
                        .unwrap()
                        .local_position()
                        .position()
                        .clone(),
                );
            }
        }
    }
    let suite: Value = serde_json::from_str(&fs::read_to_string(suite).unwrap()).unwrap();
    let diagrams: Vec<Position> = suite["positions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| Position::from_compact_diagram(&r["diagram"]).unwrap())
        .collect();
    let catalog: Value = serde_json::from_str(&fs::read_to_string(openings).unwrap()).unwrap();
    let opening_positions: Vec<Position> = catalog["openings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|o| {
            let mut local = LocalPosition::admit(Position::standard()).unwrap();
            for m in strings(&o["moves"]) {
                let a = notation::parse_action(local.position(), &m).unwrap();
                local.apply(a).unwrap();
            }
            local.position().clone()
        })
        .collect();
    let mut sets = serde_json::Map::new();
    for (name, positions) in [
        ("games", games),
        ("suite", diagrams),
        ("openings", opening_positions),
    ] {
        let rows = search::movegen_profile(&positions, repeats);
        let table: Vec<Value> = rows
            .iter()
            .map(|r| json!({"label": r.label, "nanos": r.nanos as u64, "count": r.count, "calls": r.calls,
                            "ns_per_call": r.nanos as f64 / r.calls.max(1) as f64,
                            "ns_per_count": r.nanos as f64 / r.count.max(1) as f64}))
            .collect();
        // Fixed-depth search speed (one thread, fresh engine per position).
        let (mut nodes, mut nanos) = (0u64, 0u128);
        for p in positions.iter().take(200) {
            let game = Game::from_root(
                Position::from_diagram(
                    &Square::all()
                        .filter_map(|s| p.piece(s).map(|x| (s, x)))
                        .collect::<Vec<_>>(),
                    [p.hand(Side::Sente), p.hand(Side::Gote)],
                    p.side_to_move(),
                    RootMetadata::diagram(0),
                )
                .unwrap(),
            )
            .unwrap();
            if game.conclusion().is_some() {
                continue;
            }
            let start = std::time::Instant::now();
            let r = search::search(
                &game,
                search::Limits {
                    depth: 6,
                    nodes: search::MAX_NODES,
                },
            )
            .unwrap();
            nanos += start.elapsed().as_nanos();
            nodes += r.nodes;
        }
        sets.insert(
            name.into(),
            json!({"positions": positions.len(), "rows": table,
            "search_depth6": {"positions": positions.len().min(200), "nodes": nodes,
                              "nodes_per_second": nodes as f64 / (nanos as f64 / 1e9)}}),
        );
    }
    json!({"repeats": repeats, "evaluator": search::EVALUATOR_ID, "search": search::SEARCH_ID, "sets": sets})
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input: Value = serde_json::from_str(&fs::read_to_string(&args[2]).unwrap()).unwrap();
    let output = match args[1].as_str() {
        "mine" => mine(&input),
        "finalize" => serde_json::to_string_pretty(&finalize(&input)).unwrap(),
        "probe" => {
            serde_json::to_string_pretty(&probe(&input, args[4].parse().unwrap(), args.get(5)))
                .unwrap()
        }
        // prove SUITE OUT STORE PROBE.json [PROBE.json ...]
        "prove" => {
            let probes: Vec<Value> = args[5..]
                .iter()
                .map(|p| serde_json::from_str(&fs::read_to_string(p).unwrap()).unwrap())
                .collect();
            serde_json::to_string_pretty(&prove(&input, &probes, Some(&args[4]))).unwrap()
        }
        "export" => export(&input, args[4].parse().unwrap()),
        "movegen" => serde_json::to_string_pretty(&movegen(
            &input,
            args[4].parse().unwrap(),
            &args[5],
            &args[6],
        ))
        .unwrap(),
        "depth" => serde_json::to_string_pretty(&depth(
            &input,
            args[4].parse().unwrap(),
            args[5].parse().unwrap(),
        ))
        .unwrap(),
        "judge" => serde_json::to_string(&judge(&input, args[4].parse().unwrap())).unwrap(),
        _ => panic!(
            "usage: tori_strength mine|finalize|probe|depth|export|judge INPUT OUTPUT [ARGS]"
        ),
    };
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[3])
        .unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use tori_mate_proof::plain_defender_mated;

    fn answers(d: &Value, plies: u8) -> Vec<String> {
        let p = Position::from_compact_diagram(d).unwrap();
        let local = LocalPosition::admit(p.clone()).unwrap();
        Prover::new(1_000_000)
            .answer_set(&local, plies)
            .unwrap()
            .iter()
            .map(|a| notation::format_action(&p, *a).unwrap())
            .collect()
    }

    #[test]
    fn independently_reviewed_mate_in_one_is_found_with_its_answer_set() {
        // TORI-O-MATE: the conformance corpus reaches its reviewed checkmate
        // by Ea4c-4b (Gote blockers 3a/5a/3b/5b, Sente Pheasant 4d protects 4b).
        let corpus: Value =
            serde_json::from_str(include_str!("../../../docs/m4/tori/fixtures.json")).unwrap();
        let mate = corpus["fixtures"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["id"] == "TORI-O-MATE")
            .unwrap();
        let board = &mate["given"]["root"]["board"];
        // Undo the mating move to obtain the pre-mate diagram: Eagle back to 4c.
        let before: Vec<Value> = board
            .as_array()
            .unwrap()
            .iter()
            .map(|e| json!(e.as_str().unwrap().replace("s:Ea:4b", "s:Ea:4c")))
            .collect();
        let d = json!({"board": before, "hands": {"s": {}, "g": {}}, "side_to_move": "s"});
        let found = answers(&d, 1);
        assert!(
            found.iter().any(|a| a.contains("4c") && a.contains("4b")),
            "{found:?}"
        );
    }

    #[test]
    fn memoized_prover_agrees_with_plain_recursion_on_generated_positions() {
        // Positions from seeded random play: every checking answer the memoized
        // prover finds within three plies is confirmed by plain recursion, and
        // every checking action plain recursion confirms is in the answer set.
        let mut checked = 0;
        for seed in 1..=200u64 {
            let mut state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
            let mut local = LocalPosition::admit(Position::standard()).unwrap();
            for _ in 0..(40 + seed % 60) {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                let legal = local.legal_actions();
                if legal.is_empty() {
                    break;
                }
                local
                    .apply(legal[(state % legal.len() as u64) as usize])
                    .unwrap();
            }
            let found = Prover::new(5_000_000).answer_set(&local, 3).unwrap();
            for (a, next) in successors(&local, true) {
                assert_eq!(
                    found.contains(&a),
                    plain_defender_mated(&next, 2),
                    "seed {seed}"
                );
                checked += 1;
            }
        }
        assert!(checked > 100, "{checked}");
    }

    /// Plain full-width recursion without memo: every attacker action.
    fn plain_full_defender_mated(position: &LocalPosition, plies: u8) -> bool {
        let replies = successors(position, false);
        if replies.is_empty() {
            return true;
        }
        if plies < 2 {
            return false;
        }
        replies.iter().all(|(_, next)| {
            successors(next, false)
                .iter()
                .any(|(_, n)| plain_full_defender_mated(n, plies - 2))
        })
    }

    #[test]
    fn played_move_proof_agrees_with_plain_full_width_recursion() {
        // The probe proves or refutes only the played action. It must agree
        // with a plain full-width recursion for every legal action, quiet ones
        // too, and every checking answer of the checking prover must be a win.
        // Positions come from seeded random play, kept where the checking
        // prover finds a three-ply win, so real wins occur.
        let mut compared = 0;
        let mut wins_found = 0;
        let mut positions = 0;
        for seed in 1..=400u64 {
            if positions >= 12 {
                break;
            }
            let mut state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
            let mut local = LocalPosition::admit(Position::standard()).unwrap();
            for _ in 0..160 {
                let legal = local.legal_actions();
                if legal.is_empty() {
                    break;
                }
                let one_ply = successors(&local, false)
                    .iter()
                    .any(|(_, n)| n.legal_actions().is_empty());
                if !one_ply
                    && Prover::new(200_000)
                        .attacker_mates(&local, 3)
                        .unwrap_or(false)
                {
                    let position = local.position().clone();
                    let checks = Prover::new(5_000_000).answer_set(&local, 3).unwrap();
                    for (a, after) in successors(&local, false) {
                        let proved = wins(&position, a, 3).expect("within bound");
                        assert_eq!(
                            proved,
                            plain_full_defender_mated(&after, 2),
                            "seed {seed} {a:?}"
                        );
                        if checks.contains(&a) {
                            assert!(proved, "checking answer not a full-width win");
                        }
                        wins_found += usize::from(proved);
                        compared += 1;
                    }
                    positions += 1;
                    break;
                }
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                local
                    .apply(legal[(state % legal.len() as u64) as usize])
                    .unwrap();
            }
        }
        eprintln!("{positions} positions, {compared} actions compared, {wins_found} wins");
        assert!(
            positions >= 5 && compared > 200 && wins_found >= positions,
            "{positions} positions, {compared} compared, {wins_found} wins"
        );
    }

    #[test]
    fn proof_store_key_uses_the_canonical_diagram_digest() {
        // Same canonical JSON as Python json.dumps(sort_keys=True,
        // separators=(',', ':')): sorted keys, compact.
        let d: Value = serde_json::from_str(
            r#"{"side_to_move":"s","hands":{"s":{"Sw":1},"g":{}},"board":["g:Ph:4a","s:Ph:4g"]}"#,
        )
        .unwrap();
        let key = store_key(&d, "Ph4g-4f", 3);
        assert!(
            key.starts_with("ee0e6ccc00e3b76ce11dc0798ede6e1de7b7c5e1aa840ccf35cec5b3d97aeba3|"),
            "{key}"
        );
        assert!(key.ends_with("|Ph4g-4f|3|20000000|tori-mate-proof-v2"));
    }

    /// Positions from seeded random play where the checking prover finds a
    /// win of exactly `plies` plies (and none shorter).
    fn random_wins(plies: u8, count: usize, seeds: u64) -> Vec<LocalPosition> {
        let mut out = vec![];
        for seed in 1..=seeds {
            if out.len() >= count {
                break;
            }
            let mut state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
            let mut local = LocalPosition::admit(Position::standard()).unwrap();
            for _ in 0..200 {
                let legal = local.legal_actions();
                if legal.is_empty() {
                    break;
                }
                if Prover::new(200_000).shortest(&local, plies).ok().flatten() == Some(plies) {
                    out.push(local.clone());
                    break;
                }
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                local
                    .apply(legal[(state % legal.len() as u64) as usize])
                    .unwrap();
            }
        }
        out
    }

    #[test]
    fn memo_reuse_across_depths_and_five_ply_wins_agree_with_plain_recursion() {
        // Five-ply: every checking first action of each position, memo prover
        // (full width) against plain full-width recursion.
        let positions = random_wins(5, 3, 600);
        let mut compared = 0;
        let mut wins = 0;
        for local in &positions {
            for (a, after) in successors(local, true) {
                let proved = wins_local(local, a, 5);
                assert_eq!(proved, plain_full_defender_mated(&after, 4), "{a:?}");
                wins += usize::from(proved);
                compared += 1;
                // The same prover reused across depths gives the fresh answers.
                let mut reused = Prover::full(FULL_VISITS);
                let deep = reused.defender_mated(&after, 4).unwrap();
                let shallow = reused.defender_mated(&after, 2).unwrap();
                let mut reused2 = Prover::full(FULL_VISITS);
                let shallow2 = reused2.defender_mated(&after, 2).unwrap();
                let deep2 = reused2.defender_mated(&after, 4).unwrap();
                let fresh2 = Prover::full(FULL_VISITS).defender_mated(&after, 2).unwrap();
                assert_eq!((deep, shallow), (proved, fresh2));
                assert_eq!((deep2, shallow2), (proved, fresh2));
            }
        }
        eprintln!(
            "{} positions, {compared} checking actions, {wins} five-ply wins",
            positions.len()
        );
        assert!(
            positions.len() >= 2 && wins >= positions.len(),
            "{compared} compared, {wins} wins"
        );
    }

    fn wins_local(local: &LocalPosition, action: Action, plies: u8) -> bool {
        wins(local.position(), action, plies).expect("within bound")
    }
}
