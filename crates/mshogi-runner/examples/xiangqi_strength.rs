//! MSH-194 (Xiangqi Strength Harness): native validation, opening balance
//! screen, tactical-suite probes, late-game depth and Texel export, all under
//! native Xiangqi authority.
//!
//! Usage:
//!   xiangqi_strength screen CANDIDATES.json OUT.json DEPTH
//!   xiangqi_strength mine REPORTS.json OUT.jsonl     (REPORTS: list of xiangqi-match report paths)
//!   xiangqi_strength probe SUITE.json OUT.json TIME_MS
//!   xiangqi_strength complete SUITE.json OUT.json
//!   xiangqi_strength export REPORTS.json OUT.jsonl EVERY
//!   xiangqi_strength depth REPORTS.json OUT.json TIME_MS MIN_PLY
use mshogi_core::xiangqi::*;
use mshogi_search::{alphabeta::timing::TimeBudget, xiangqi as search};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, sync::Mutex, thread};
#[path = "support/xiangqi_mate_proof.rs"]
mod xiangqi_mate_proof;
use xiangqi_mate_proof::{MAX_PROOF_PLIES, Prover};

/// Proof visit bound of one mined position.
const PROOF_VISITS: u64 = 400_000;
/// Full-width proof visit bound of one suite position.
const FULL_VISITS: u64 = 20_000_000;

/// Native game from an optional FEN root and a UCI move list. `Err` names the
/// first refused move.
fn build(fen: Option<&str>, moves: &[String]) -> Result<Game, String> {
    let root = match fen {
        Some(fen) => PositionRoot::from_fen(fen).map_err(|e| e.to_string())?,
        None => PositionRoot::standard(),
    };
    let mut game =
        Game::from_root(root, ClaimPolicy::AutoValidClaims).map_err(|e| format!("root: {e:?}"))?;
    for (index, text) in moves.iter().enumerate() {
        let action: Action = text
            .parse()
            .map_err(|e| format!("move {}: {e}", index + 1))?;
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play { action },
        })
        .map_err(|e| format!("move {} {text}: {e:?}", index + 1))?;
    }
    if game.conclusion().is_some() || game.pending_adjudication().is_some() {
        return Err("concluded or pending".into());
    }
    Ok(game)
}

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

/// Fixed-depth score of each candidate's final position, side to move's view.
fn screen(spec: &Value, depth: u16) -> Value {
    let rows = spec["openings"].as_array().unwrap();
    let positions = parallel(rows, |r| {
        let id = r["id"].as_str().unwrap();
        match build(r["fen"].as_str(), &strings(&r["moves"])) {
            Err(e) => json!({"id": id, "legal": false, "error": e}),
            Ok(game) => {
                let result = search::search(
                    &game,
                    search::Limits {
                        depth,
                        nodes: search::MAX_NODES,
                    },
                )
                .unwrap();
                json!({"id": id, "legal": true, "score": result.score, "depth": result.completed_depth,
                       "nodes": result.nodes, "to_move": format!("{:?}", game.position().side_to_move())})
            }
        }
    });
    json!({"evaluator": search::EVALUATOR_ID, "search": search::SEARCH_ID, "depth": depth,
           "positions": positions})
}

/// FEN of a diagram with counters reset: a short mate does not depend on them.
fn diagram(position: &Position) -> String {
    let fen = position.to_fen().expect("native position");
    let fields: Vec<_> = fen.split(' ').collect();
    format!("{} {} - - 0 1", fields[0], fields[1])
}

/// Forced checking mates of 3..=7 plies in every position before a reply of
/// every native-replayed clocked report, with the full native answer set.
fn mine(paths: &Value) -> String {
    let paths: Vec<String> = strings(paths);
    let rows = parallel(&paths, |path| {
        let text = fs::read_to_string(path).unwrap();
        let digest = format!("{:x}", Sha256::digest(text.as_bytes()));
        let report = mshogi_runner::xiangqi_match::MatchReport::from_json(&text)
            .expect("native replay of the report");
        let opening = &report.config.opening;
        let mut position = match &opening.fen {
            Some(fen) => PositionRoot::from_fen(fen).unwrap().position().clone(),
            None => PositionRoot::standard().position().clone(),
        };
        for m in &opening.moves {
            position.apply(m.parse().unwrap()).unwrap();
        }
        let mut out = vec![];
        for (ply, turn) in report.turns.iter().enumerate() {
            let mut prover = Prover::new(PROOF_VISITS);
            if let Ok(Some(plies)) = prover.shortest(&position, MAX_PROOF_PLIES)
                && plies >= 3
                && let Ok(answers) = Prover::new(PROOF_VISITS).answer_set(&position, plies)
            {
                let played = turn
                    .reply
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("")
                    .to_string();
                let expected: Vec<String> = answers.iter().map(|a| a.to_string()).collect();
                out.push(json!({"fen": diagram(&position), "mate_plies": plies,
                    "expected_moves": expected, "played": played,
                    "played_in_answers": expected.contains(&played),
                    "source_report_sha256": digest, "game": report.config.id,
                    "opening_id": opening.id, "ply": ply}));
            }
            let Ok(action) = turn
                .reply
                .split_whitespace()
                .nth(1)
                .unwrap_or("")
                .parse::<Action>()
            else {
                break;
            };
            position.apply(action).unwrap();
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

/// Fresh-engine timed search of each suite position; a pass plays a move of
/// the native answer set.
fn probe(spec: &Value, ms: u64) -> Value {
    let rows = spec["positions"].as_array().unwrap();
    let results = parallel(rows, |r| {
        let game = build(r["fen"].as_str(), &strings(&r["moves"])).unwrap();
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
        let chosen = match result.decision {
            Some(search::Decision::Choose(search::Choice::Move(a))) => a.to_string(),
            other => format!("{other:?}"),
        };
        let pass = strings(&r["expected_moves"]).contains(&chosen);
        json!({"id": r["id"], "split": r["split"], "mate_plies": r["mate_plies"], "chosen": chosen,
               "pass": pass, "score": result.score, "depth": result.completed_depth, "nodes": result.nodes})
    });
    let count = |split: Option<&str>| {
        let selected: Vec<_> = results
            .iter()
            .filter(|r| split.is_none_or(|s| r["split"] == s))
            .collect();
        json!({"positions": selected.len(), "passed": selected.iter().filter(|r| r["pass"] == true).count()})
    };
    json!({"evaluator": search::EVALUATOR_ID, "search": search::SEARCH_ID, "time_ms": ms,
           "all": count(None), "development": count(Some("development")),
           "held_out": count(Some("held-out")), "results": results})
}

/// Native-labelled Texel rows: one JSON line per report, with every
/// `every`-th position after the opening. Labels are native results only
/// (Red half points / 2); unscored games keep no samples.
fn export(paths: &Value, every: usize) -> String {
    let paths: Vec<String> = strings(paths);
    let rows = parallel(&paths, |path| {
        let text = fs::read_to_string(path).unwrap();
        let digest = format!("{:x}", Sha256::digest(text.as_bytes()));
        let report = mshogi_runner::xiangqi_match::MatchReport::from_json(&text)
            .expect("native replay of the report");
        let opening = &report.config.opening;
        let mut position = match &opening.fen {
            Some(fen) => PositionRoot::from_fen(fen).unwrap().position().clone(),
            None => PositionRoot::standard().position().clone(),
        };
        for m in &opening.moves {
            position.apply(m.parse().unwrap()).unwrap();
        }
        let initial = position.to_fen().unwrap();
        let red_outcome = report.conclusion.and_then(|c| {
            (report.stop == mshogi_runner::xiangqi_match::MatchStop::NativeConclusion).then(|| {
                let [red, _] = match c {
                    Conclusion::Draw { .. } => [1, 1],
                    Conclusion::Win {
                        winner: Side::Red, ..
                    } => [2, 0],
                    Conclusion::Win { .. } => [0, 2],
                };
                f64::from(red) / 2.0
            })
        });
        let mut samples = vec![];
        for (ply, turn) in report.turns.iter().enumerate() {
            if let Some(outcome) = red_outcome
                && ply % every == 0
            {
                samples.push(
                    json!({"fen": position.to_fen().unwrap(), "red_outcome": outcome, "ply": ply}),
                );
            }
            let Ok(action) = turn
                .reply
                .split_whitespace()
                .nth(1)
                .unwrap_or("")
                .parse::<Action>()
            else {
                break;
            };
            position.apply(action).unwrap();
        }
        json!({"game_id": format!("{digest}/0"), "source_report_sha256": digest, "initial_fen": initial,
               "opening_id": opening.id, "red_outcome": red_outcome, "outcome": report.conclusion,
               "stop": report.stop, "truncated": red_outcome.is_none(), "samples": samples})
    });
    let mut text = String::new();
    for row in rows {
        text.push_str(&row.to_string());
        text.push('\n');
    }
    text
}

/// Completed depth of a fresh timed search on positions from ply `min_ply`
/// onward (every fourth reply), as a conservative late-game depth measure.
fn depth(paths: &Value, ms: u64, min_ply: usize) -> Value {
    let paths: Vec<String> = strings(paths);
    let mut positions = vec![];
    for path in &paths {
        let report = mshogi_runner::xiangqi_match::MatchReport::from_json(
            &fs::read_to_string(path).unwrap(),
        )
        .unwrap();
        let opening = &report.config.opening;
        let mut moves = opening.moves.clone();
        for (ply, turn) in report.turns.iter().enumerate() {
            if ply >= min_ply && ply % 4 == 0 {
                positions.push((opening.fen.clone(), moves.clone()));
            }
            moves.push(
                turn.reply
                    .split_whitespace()
                    .nth(1)
                    .unwrap_or("")
                    .to_string(),
            );
        }
    }
    // One search per core keeps the measurement close to a single game.
    let results = parallel(&positions, |(fen, moves)| {
        let game = build(fen.as_deref(), moves).unwrap();
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

/// Complete answer sets: every first move that forces a win (checkmate or
/// stalemate) within `mate_plies` with a full-width attacker, from the native
/// prover. Rows whose full proof exceeds the bound are dropped and counted.
fn complete(spec: &Value) -> Value {
    let rows = spec["positions"].as_array().unwrap();
    let done = parallel(rows, |r| {
        let position = PositionRoot::from_fen(r["fen"].as_str().unwrap())
            .unwrap()
            .position()
            .clone();
        let plies = r["mate_plies"].as_u64().unwrap() as u8;
        let mut prover = Prover::full(FULL_VISITS);
        let answers = prover.answer_set(&position, plies);
        let shortest = Prover::full(FULL_VISITS).shortest(&position, plies);
        match (answers, shortest) {
            (Ok(a), Ok(Some(win))) => {
                let mut row = r.clone();
                let checks = strings(&r["expected_moves"]);
                let all: Vec<String> = a.iter().map(|m| m.to_string()).collect();
                assert!(checks.iter().all(|c| all.contains(c)), "{}", r["id"]);
                row["expected_moves"] = json!(all);
                row["checking_answers"] = json!(checks);
                row["shortest_win_plies"] = json!(win);
                Some(row)
            }
            _ => None,
        }
    });
    let dropped = done.iter().filter(|r| r.is_none()).count();
    let mut out = spec.clone();
    out["positions"] = json!(done.into_iter().flatten().collect::<Vec<_>>());
    out["dropped_full_proof_bound"] = json!(dropped);
    out
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input: Value = serde_json::from_str(&fs::read_to_string(&args[2]).unwrap()).unwrap();
    let output = match args[1].as_str() {
        "screen" => {
            serde_json::to_string_pretty(&screen(&input, args[4].parse().unwrap())).unwrap()
        }
        "mine" => mine(&input),
        "probe" => serde_json::to_string_pretty(&probe(&input, args[4].parse().unwrap())).unwrap(),
        "export" => export(&input, args[4].parse().unwrap()),
        "complete" => serde_json::to_string_pretty(&complete(&input)).unwrap(),
        "depth" => serde_json::to_string_pretty(&depth(
            &input,
            args[4].parse().unwrap(),
            args[5].parse().unwrap(),
        ))
        .unwrap(),
        _ => panic!("usage: xiangqi_strength screen|mine|probe INPUT OUTPUT [DEPTH|TIME_MS]"),
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
    use xiangqi_mate_proof::plain_defender_mated;
    fn pos(fen: &str) -> Position {
        PositionRoot::from_fen(fen).unwrap().position().clone()
    }
    fn answers(fen: &str, plies: u8) -> Vec<String> {
        Prover::new(100_000)
            .answer_set(&pos(fen), plies)
            .unwrap()
            .iter()
            .map(|a| a.to_string())
            .collect()
    }
    #[test]
    fn two_chariot_mate_in_one_is_found_with_its_answer_set() {
        // Black General d9 alone; Red Chariots a8 and b1; Red General f0
        // covers the f-file. b1b9 checks along rank 9 while a8 covers rank 8:
        // mate. b1d1 is check but e9 is free; a8a9 frees rank 8; a8d8 can be
        // captured.
        let fen = "3k5/R8/9/9/9/9/9/9/1R7/5K3 w - - 0 1";
        assert_eq!(
            Prover::new(100_000).shortest(&pos(fen), 7).unwrap(),
            Some(1)
        );
        assert_eq!(answers(fen, 1), vec!["b1b9".to_string()]);
    }
    #[test]
    fn facing_generals_take_part_in_mates() {
        // Black General d9; Red Chariot a1. With the Red General on f0 the
        // check a1d1 leaves e9 and e8 free: no mate in one. With the Red
        // General on e0 the facing Generals cover the e-file: a1d1 mates.
        let away = "3k5/9/9/9/9/9/9/9/R8/5K3 w - - 0 1";
        assert_eq!(Prover::new(100_000).shortest(&pos(away), 1).unwrap(), None);
        let facing = "3k5/9/9/9/9/9/9/9/R8/4K4 w - - 0 1";
        assert_eq!(answers(facing, 1), vec!["a1d1".to_string()]);
    }
    #[test]
    fn memoized_prover_agrees_with_plain_recursion() {
        for fen in [
            "3k5/R8/9/9/9/9/9/9/1R7/5K3 w - - 0 1",
            "3k5/9/9/9/9/9/9/9/R8/5K3 w - - 0 1",
            "4k4/4a4/9/9/9/9/9/9/R8/R4K3 w - - 0 1",
            "3ak4/4a4/9/9/9/9/9/2C6/R8/5K3 w - - 0 1",
        ] {
            let p = pos(fen);
            for plies in [1u8, 3, 5] {
                for (a, next) in xiangqi_mate_proof::successors(&p, true) {
                    let memo = Prover::new(1_000_000)
                        .defender_mated(&next, plies - 1)
                        .unwrap();
                    assert_eq!(
                        memo,
                        plain_defender_mated(&next, plies - 1),
                        "{fen} {a} {plies}"
                    );
                }
            }
        }
    }
    #[test]
    fn suite_answer_sets_agree_with_plain_recursion() {
        // Every checking first move is classified again without memo: checking
        // answers mate within mate_plies, other checks do not.
        let text = include_str!("../../../tools/strength/data/xiangqi-tactics-v1.json");
        let spec: Value = serde_json::from_str(text).unwrap();
        let rows: Vec<&Value> = spec["positions"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|p| p["mate_plies"].as_u64().unwrap() <= 5)
            .collect();
        assert!(rows.len() > 200);
        parallel(&rows, |p| {
            let position = pos(p["fen"].as_str().unwrap());
            let plies = p["mate_plies"].as_u64().unwrap() as u8;
            let expected = strings(&p["checking_answers"]);
            assert!(!expected.is_empty());
            assert!(
                expected
                    .iter()
                    .all(|e| strings(&p["expected_moves"]).contains(e))
            );
            for (a, next) in xiangqi_mate_proof::successors(&position, true) {
                let mates = plain_defender_mated(&next, plies - 1);
                assert_eq!(mates, expected.contains(&a.to_string()), "{} {a}", p["id"]);
            }
            // No shorter mate exists.
            if plies > 1 {
                for (a, next) in xiangqi_mate_proof::successors(&position, true) {
                    assert!(
                        !plain_defender_mated(&next, plies - 3),
                        "{} {a} shorter",
                        p["id"]
                    );
                }
            }
        });
    }
}
