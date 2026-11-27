//! Game operations of `python3 -m tools.strength.cli`. JSON in, JSON out.
#[path = "support/internal_match_config.rs"]
mod config;
#[path = "support/strength_positions.rs"]
mod positions;
#[path = "support/tori_mate_proof.rs"]
mod tori_mate_proof;
use mshogi_core::game::{CanonicalRecord, RulesProfile};
use mshogi_engine::{
    BridgeOptions, EngineBridge, ExecutionContext, ExecutionPurpose, ExecutionRequest,
    ExecutionResult, PROTOCOL_VERSION, WorkLimits,
};
use mshogi_runner::experiments::{ValidatedReport, digest};
use positions::Result;
use serde_json::{Value, json};
use std::{
    io::{self, Read},
    thread,
    time::{Duration, Instant},
};

fn search(engine: &str, record: CanonicalRecord, limits: WorkLimits) -> Result<Value> {
    let watchdog = Duration::from_secs(60);
    let cleanup = Duration::from_secs(2);
    let bridge = EngineBridge::with_options(engine.into(), BridgeOptions { watchdog, cleanup })?;
    bridge.submit(ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: 1,
            session_version: 1,
            game: record,
            limits,
        },
    })?;
    let started = Instant::now();
    let result = loop {
        if let Some(response) = bridge.poll() {
            break response.observation.result;
        }
        if started.elapsed() > watchdog + cleanup {
            return Err("engine watchdog elapsed".into());
        }
        thread::sleep(Duration::from_millis(1));
    };
    bridge.shutdown();
    let started = Instant::now();
    while !bridge.shutdown_acknowledged() {
        if started.elapsed() > cleanup + cleanup {
            return Err("engine shutdown unacknowledged".into());
        }
        thread::sleep(Duration::from_millis(1));
    }
    if let ExecutionResult::Failure { failure } = result {
        return Err(failure.message.into());
    }
    Ok(serde_json::to_value(result)?)
}

/// Read reports through the validating decoder, which restores every omitted request history.
/// Sampling indexes board moves only. A formal claim or declaration has no board ply.
fn project(text: &str, profile: RulesProfile, mode: &str, count: usize) -> Result<()> {
    if count == 0 {
        return Err("sampling count must be positive".into());
    }
    let report = ValidatedReport::from_json(text)?;
    let digest = digest(text);
    for (index, game) in report.games.iter().enumerate() {
        if !game.cleanup.is_empty() {
            return Err("report contains a cleanup failure".into());
        }
        let opening = &report.config.openings[game.pairing.opening];
        if opening.root.record.profile() != profile {
            return Err("report profile differs".into());
        }
        let game_id = format!("{digest}/{index}");
        let moves: Vec<_> = game
            .steps
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                let after = game
                    .steps
                    .get(i + 1)
                    .map_or(&game.final_root.record, |s| &s.request.game);
                (s.applied && after.known_board_actions() > s.request.game.known_board_actions())
                    .then_some((s, after))
            })
            .collect();
        let mut samples = Vec::new();
        match mode {
            "export" => {
                if game.score_first_twice.is_some() {
                    for (ply, (step, _)) in moves.iter().enumerate().step_by(count) {
                        if let Some(position) = positions::export_position(&step.request.game)? {
                            samples.push(json!({"ply":ply,"position":position}));
                        }
                    }
                }
                println!(
                    "{}",
                    json!({"game_id":game_id,"opening_id":opening.group,"stop":game.stop,"outcome":game.outcome,
                    "score_first_twice":game.score_first_twice,"samples":samples})
                );
            }
            "judge" => {
                let n = moves.len().min(count);
                for i in 0..n {
                    let ply = (2 * i + 1) * moves.len() / (2 * n);
                    let (step, after) = moves[ply];
                    println!(
                        "{}",
                        json!({"game_id":game_id,"opening_id":opening.group,"ply":ply,"before":step.request.game,
                        "after":after,"player":step.player,"observation":step.observation.as_ref().map(|o| &o.result),
                        "mover_first": ((step.player == 0) != game.pairing.swapped),
                        "terminal_score":if after == &game.final_root.record {game.score_first_twice} else {None} })
                    );
                }
            }
            _ => return Err("unknown report operation".into()),
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.first().is_some_and(|op| op == "config") {
        return config::Args::write(args);
    }
    let mut text = String::new();
    io::stdin().read_to_string(&mut text)?;
    match args.as_slice() {
        [op, profile, seed, plies, mode] if op == "position" => {
            let profile = serde_json::from_value(json!(profile))?;
            let row = serde_json::from_str(&text)?;
            println!("{}",positions::prepare(profile,&row,seed.parse()?,plies.parse()?,mode=="prove")?);
        }
        [op,plies,visits] if op == "unlisted-tori" => {
            let input:Value=serde_json::from_str(&text)?;
            let length:u8=plies.parse()?;
            if !(1..=tori_mate_proof::MAX_PROOF_PLIES).contains(&length) { return Err("unsupported Tori proof length".into()); }
            let position=mshogi_core::tori::Position::from_compact_diagram(&input["diagram"])?;
            let choice=serde_json::from_value(input["action"].clone())?;
            let token=mshogi_core::tori::notation::format_action(&position,choice)?;
            let mut child=mshogi_core::tori::LocalPosition::admit(position)?;
            child.apply(choice)?;
            let cap:u64=visits.parse()?;
            let mut prover=tori_mate_proof::Prover::full(cap);
            let win = if cap == 0 { None } else { prover.defender_mated(&child,length-1).ok() };
            println!("{}",json!({"win":win,"visits":prover.visits,"token":token}));
        }
        [op,engine,depth,nodes,time @ ..] if op == "search" && time.len() <= 1 => {
            let record = CanonicalRecord::from_json(&text)?;
            println!("{}",search(engine,record,WorkLimits{depth:depth.parse()?,nodes:nodes.parse()?,time_ms:time.first().map(|v| v.parse()).transpose()?})?);
        }
        [op,profile,mode,count] if op == "report" => project(&text,serde_json::from_value(json!(profile))?,mode,count.parse()?)?,
        _ => return Err("expected position PROFILE SEED PLIES prepare|prove, search ENGINE DEPTH NODES, or report PROFILE export|judge COUNT".into()),
    }
    Ok(())
}
