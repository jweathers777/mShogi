//! Native Chu self-play for evaluation fitting. Unfinished games have no labels.
//! chu_selfplay OPENINGS OUT START COUNT NODES MAX_PLIES SECONDS_PER_GAME
use mshogi_core::chu::*;
use mshogi_search::chu::{self, eval, position::ChuPosition};
use serde_json::{Value, json};
use std::{
    fs,
    io::{BufWriter, Write},
    time::{Duration, Instant},
};
#[path = "support/chu_roots.rs"]
mod chu_roots;

fn generate(openings: &[Value], id: u64, nodes: u64, max_plies: usize, seconds: u64) -> Value {
    let started = Instant::now();
    let opening = &openings[id as usize % openings.len()];
    let record: GameRecord = serde_json::from_value(opening["record"].clone()).unwrap();
    let mut game = record.to_game().unwrap();
    // Repeated catalog roots stay in one partition, including these randomized
    // quiet continuations. No filtering by final result or engine score.
    chu_roots::quiet_setup(&mut game, id + 1, 8, true);
    let initial = GameRecord::from_game(&game).unwrap();
    let mut engines = [chu::new_engine(), chu::new_engine()];
    let mut samples = Vec::new();
    let mut moves = Vec::new();
    let mut visited = 0;
    let mut stop = "ply_limit";
    let mut error = None;
    let deadline = Duration::from_secs(seconds);
    for ply in 0..max_plies {
        if game.conclusion().is_some() {
            stop = "native_outcome";
            break;
        }
        if started.elapsed() >= deadline {
            stop = "wall_limit";
            break;
        }
        if eval::quiet(&ChuPosition::from_local(game.local_position())) {
            let local = game.local_position();
            samples.push(json!({"ply":ply,"diagram":{
                "pieces":local.position().pieces().collect::<Vec<_>>(),
                "side":local.position().side_to_move(),"protection":local.protection()}}));
        }
        let searched = chu::search_with_state(
            &game,
            chu::Limits { depth: 64, nodes },
            None,
            &mut engines[game.position().side_to_move().index()],
            || started.elapsed() >= deadline,
        );
        let result = match searched {
            Ok(r) => r,
            Err(e) => {
                stop = "search_error";
                error = Some(e.to_string());
                break;
            }
        };
        visited += result.nodes;
        let Some(chu::Decision::Choose(action)) = result.decision else {
            stop = "no_decision";
            break;
        };
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play {
                action: action.clone(),
            },
        })
        .expect("native engine move");
        moves.push(action);
    }
    let outcome = game.conclusion();
    if outcome.is_some() {
        stop = "native_outcome";
    }
    let label = outcome.map(|c| match c.winner() {
        Some(Side::Sente) => 1.0,
        Some(Side::Gote) => 0.0,
        None => 0.5,
    });
    if label.is_none() {
        samples.clear();
    }
    for sample in &mut samples {
        sample["sente_outcome"] = json!(label);
    }
    json!({"schema":1,"game_id":format!("chu-eval-{id}"),"opening_id":opening["id"],
        "initial_record":initial,"moves":moves,"sente_outcome":label,"outcome":outcome,
        "samples":samples,"truncated":outcome.is_none(),"stop":stop,"error":error,
        "nodes_per_move":nodes,"max_plies":max_plies,"seconds_per_game":seconds,
        "nodes":visited,"elapsed_seconds":started.elapsed().as_secs_f64()})
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 7 {
        return Err(
            "usage: chu_selfplay OPENINGS OUT START COUNT NODES MAX_PLIES SECONDS_PER_GAME".into(),
        );
    }
    let catalog: Value = serde_json::from_str(&fs::read_to_string(&args[0])?)?;
    let openings = catalog["openings"].as_array().ok_or("missing openings")?;
    let start: u64 = args[2].parse()?;
    let count: u64 = args[3].parse()?;
    let nodes: u64 = args[4].parse()?;
    let plies: usize = args[5].parse()?;
    let seconds: u64 = args[6].parse()?;
    if openings.is_empty() || nodes == 0 || nodes > chu::MAX_NODES || plies == 0 || seconds == 0 {
        return Err("invalid limits".into());
    }
    let end = start.checked_add(count).ok_or("game ID overflow")?;
    let mut out = BufWriter::new(
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args[1])?,
    );
    for id in start..end {
        let game = generate(openings, id, nodes, plies, seconds);
        eprintln!(
            "game {id}: {} plies, {} samples, {} in {} s",
            game["moves"].as_array().unwrap().len(),
            game["samples"].as_array().unwrap().len(),
            game["stop"],
            game["elapsed_seconds"]
        );
        serde_json::to_writer(&mut out, &game)?;
        writeln!(out)?;
        out.flush()?;
    }
    Ok(())
}
