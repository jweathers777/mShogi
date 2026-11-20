//! MSH-204 (Tori Strength Harness): tactical-suite mining and answer sets,
//! suite probes, late-game depth and Texel export, all under native Tori
//! authority. Inputs are native experiment reports (`mshogi-match
//! experiment-run`, uncompressed), validated by native replay.
//!
//! Usage:
//!   tori_strength mine REPORTS.json OUT.jsonl        (REPORTS: list of report paths)
//!   tori_strength complete SUITE.json OUT.json
//!   tori_strength probe SUITE.json OUT.json TIME_MS
//!   tori_strength depth REPORTS.json OUT.json TIME_MS MIN_PLY
//!   tori_strength export REPORTS.json OUT.jsonl EVERY
use mshogi_core::{native::CanonicalRecord, tori::*};
use mshogi_runner::experiments::ValidatedReport;
use mshogi_search::{alphabeta::timing::TimeBudget, tori as search};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, sync::Mutex, thread};
#[path = "support/tori_mate_proof.rs"]
mod tori_mate_proof;
use tori_mate_proof::{MAX_PROOF_PLIES, Prover};

/// Proof visit bound of one mined position.
const PROOF_VISITS: u64 = 400_000;
/// Full-width proof visit bound of one suite position.
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

fn side_code(side: Side) -> &'static str {
    match side {
        Side::Sente => "s",
        Side::Gote => "g",
    }
}

/// Compact diagram: board entries `owner:face:square`, hands by face code and
/// the side to move. Counters and history are not part of a diagram.
fn diagram(position: &Position) -> Value {
    let board: Vec<String> = Square::all()
        .filter_map(|s| {
            position
                .piece(s)
                .map(|p| format!("{}:{}:{s}", side_code(p.side), p.face.code()))
        })
        .collect();
    let hands: serde_json::Map<String, Value> = [Side::Sente, Side::Gote]
        .into_iter()
        .map(|side| {
            let hand = position.hand(side);
            let counts: serde_json::Map<String, Value> = Face::HANDS
                .into_iter()
                .filter(|&f| hand.count(f) > 0)
                .map(|f| (f.code().to_string(), json!(hand.count(f))))
                .collect();
            (side_code(side).to_string(), Value::Object(counts))
        })
        .collect();
    json!({"board": board, "hands": hands,
           "side_to_move": side_code(position.side_to_move())})
}

fn face(code: &str) -> Face {
    Face::ALL.into_iter().find(|f| f.code() == code).unwrap()
}

fn side(code: &str) -> Side {
    if code == "s" { Side::Sente } else { Side::Gote }
}

/// Native diagram root of a compact diagram.
fn root(d: &Value) -> Position {
    let board: Vec<(Square, Piece)> = strings(&d["board"])
        .iter()
        .map(|e| {
            let parts: Vec<&str> = e.split(':').collect();
            (
                parts[2].parse().unwrap(),
                Piece {
                    side: side(parts[0]),
                    face: face(parts[1]),
                },
            )
        })
        .collect();
    let hands = ["s", "g"].map(|s| {
        let counts: Vec<(Face, u8)> = d["hands"][s]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (face(k), v.as_u64().unwrap() as u8))
            .collect();
        Hand::from_counts(&counts).unwrap()
    });
    Position::from_diagram(
        &board,
        hands,
        side(d["side_to_move"].as_str().unwrap()),
        RootMetadata::diagram(0),
    )
    .unwrap()
}

/// One validated game: its native record and the event count of its opening.
struct Source {
    digest: String,
    game_index: usize,
    opening_id: String,
    game: Game,
    start_events: usize,
    stop: String,
}

fn tori_record(record: &CanonicalRecord) -> &GameRecord {
    match record {
        CanonicalRecord::Tori(r) => r,
        _ => panic!("not a Tori report"),
    }
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
    let all: Vec<Source> = strings(paths).iter().flat_map(|p| sources(p)).collect();
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
                    json!({"diagram": diagram(local.position()), "mate_plies": plies,
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

/// Complete answer sets from the full-width prover; rows with a one-ply win
/// or beyond the proof bound are dropped and counted.
fn complete(spec: &Value) -> Value {
    let rows = spec["positions"].as_array().unwrap();
    let done = parallel(rows, |r| {
        let position = root(&r["diagram"]);
        let local = LocalPosition::admit(position.clone()).unwrap();
        let plies = r["mate_plies"].as_u64().unwrap() as u8;
        let answers = Prover::full(FULL_VISITS).answer_set(&local, plies);
        let shortest = Prover::full(FULL_VISITS).shortest(&local, plies);
        match (answers, shortest) {
            (Ok(_), Ok(Some(1))) => Err("one_ply_win"),
            (Ok(a), Ok(Some(win))) => {
                let mut row = r.clone();
                let checks = strings(&r["checking_answers"]);
                let all: Vec<String> = a
                    .iter()
                    .map(|m| notation::format_action(&position, *m).unwrap())
                    .collect();
                assert!(checks.iter().all(|c| all.contains(c)), "{}", r["id"]);
                row["expected_moves"] = json!(all);
                row["shortest_win_plies"] = json!(win);
                Ok(row)
            }
            _ => Err("full_proof_bound"),
        }
    });
    let count = |why: &str| {
        done.iter()
            .filter(|r| r.as_ref().err() == Some(&why))
            .count()
    };
    let mut out = spec.clone();
    out["dropped_one_ply_win"] = json!(count("one_ply_win"));
    out["dropped_full_proof_bound"] = json!(count("full_proof_bound"));
    out["positions"] = json!(done.into_iter().flatten().collect::<Vec<_>>());
    out
}

/// Fresh-engine timed search of each suite position; a pass plays an action
/// of the native answer set.
fn probe(spec: &Value, ms: u64) -> Value {
    let rows = spec["positions"].as_array().unwrap();
    let results = parallel(rows, |r| {
        let game = Game::from_root(root(&r["diagram"])).unwrap();
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
            Some(search::Decision::Choose(a)) => {
                notation::format_action(game.position(), a).unwrap()
            }
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

/// Completed depth of a fresh timed search (with the game's full history) on
/// every fourth position from board ply `min_ply` after the opening.
fn depth(paths: &Value, ms: u64, min_ply: usize) -> Value {
    let all: Vec<Source> = strings(paths).iter().flat_map(|p| sources(p)).collect();
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

/// Sente's native result in half points / 2; None unless natively concluded.
fn sente_outcome(s: &Source) -> Option<f64> {
    if s.stop != "native_concluded" {
        return None;
    }
    let winner = match s.game.conclusion()? {
        Conclusion::Checkmate { winner } | Conclusion::Resignation { winner } => winner,
        Conclusion::StalemateLoss { loser } | Conclusion::RepetitionForfeit { loser } => {
            loser.opponent()
        }
    };
    Some(if winner == Side::Sente { 1.0 } else { 0.0 })
}

/// Native-labelled Texel rows: one JSON line per game, with every `every`-th
/// position after the opening. Unscored games keep no samples.
fn export(paths: &Value, every: usize) -> String {
    let all: Vec<Source> = strings(paths).iter().flat_map(|p| sources(p)).collect();
    let rows = parallel(&all, |s| {
        let outcome = sente_outcome(s);
        let mut samples = vec![];
        if let Some(outcome) = outcome {
            for (ply, (event, _)) in plays(s).into_iter().enumerate() {
                if ply % every == 0 {
                    let snapshot = s.game.event_prefix(event).unwrap();
                    samples.push(
                        json!({"diagram": diagram(snapshot.local_position().position()),
                                        "sente_outcome": outcome, "ply": ply}),
                    );
                }
            }
        }
        json!({"game_id": format!("{}/{}", s.digest, s.game_index), "source_report_sha256": s.digest,
               "opening_id": s.opening_id, "sente_outcome": outcome, "stop": s.stop,
               "truncated": outcome.is_none(), "samples": samples})
    });
    let mut text = String::new();
    for row in rows {
        text.push_str(&row.to_string());
        text.push('\n');
    }
    text
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input: Value = serde_json::from_str(&fs::read_to_string(&args[2]).unwrap()).unwrap();
    let output = match args[1].as_str() {
        "mine" => mine(&input),
        "complete" => serde_json::to_string_pretty(&complete(&input)).unwrap(),
        "probe" => serde_json::to_string_pretty(&probe(&input, args[4].parse().unwrap())).unwrap(),
        "export" => export(&input, args[4].parse().unwrap()),
        "depth" => serde_json::to_string_pretty(&depth(
            &input,
            args[4].parse().unwrap(),
            args[5].parse().unwrap(),
        ))
        .unwrap(),
        _ => panic!("usage: tori_strength mine|complete|probe|depth|export INPUT OUTPUT [ARGS]"),
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
    use tori_mate_proof::{plain_defender_mated, successors};

    fn answers(d: &Value, plies: u8) -> Vec<String> {
        let p = root(d);
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

    #[test]
    fn diagram_round_trip_keeps_board_hands_and_side() {
        let mut local = LocalPosition::admit(Position::standard()).unwrap();
        for _ in 0..30 {
            let a = local.legal_actions()[3 % local.legal_actions().len()];
            local.apply(a).unwrap();
        }
        let d = diagram(local.position());
        let back = root(&d);
        assert_eq!(
            RepetitionKey::from_position(&back),
            RepetitionKey::from_position(local.position())
        );
    }
}
