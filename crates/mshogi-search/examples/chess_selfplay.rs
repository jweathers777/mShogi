//! Fresh native self-play records for classical evaluation tuning.
//! Usage: chess_selfplay START_GAME COUNT NODES SEED
//! Keep all samples of one game in the same corpus partition. A 60-second
//! cooperative deadline bounds a game; the external runner must also impose a
//! process watchdog because a native rules call cannot be interrupted here.
use mshogi_core::chess::{AdjudicatedGame, ChessOutcome, ClaimPolicy, Color, PieceKind};
use mshogi_search::{
    chess_bitboard::ChessPosition,
    chess_engine::{ChessEngine, SearchOptions},
    chess_eval,
    search::{SearchChoice, SearchDecision, SearchLimits},
};
use serde_json::{Value, json};
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    }
}

fn quiet(game: &AdjudicatedGame) -> bool {
    let p = game.position();
    !p.in_check()
        && !p.legal_actions().iter().any(|m| {
            m.promotion.is_some()
                || p.piece_at(m.to).is_some()
                || (p
                    .piece_at(m.from)
                    .is_some_and(|piece| piece.kind == PieceKind::Pawn)
                    && m.from.file() != m.to.file())
        })
        && chess_eval::decompose(&ChessPosition::from_position(p))
            .specialized
            .is_none()
}

fn generate(id: u64, nodes: u64, master_seed: u64) -> Value {
    let started = Instant::now();
    let deadline = Duration::from_secs(60);
    let seed = Rng(master_seed.wrapping_add(id.wrapping_mul(0x9e3779b97f4a7c15))).next();
    let mut rng = Rng(seed);
    let mut opening = Vec::new();
    let mut game = AdjudicatedGame::new(ClaimPolicy::AutoClaim);
    let mut accepted = false;
    let mut attempts = 0;
    // Rejection is restricted to the randomized opening, never to game result.
    for attempt in 1..=64 {
        if started.elapsed() >= deadline {
            break;
        }
        attempts = attempt;
        game = AdjudicatedGame::new(ClaimPolicy::AutoClaim);
        opening.clear();
        let plies = 8 + rng.next() % 9;
        for _ in 0..plies {
            if game.outcome().is_some() || game.policy_claim().is_some() {
                break;
            }
            let legal = game.position().legal_actions();
            if legal.is_empty() {
                break;
            }
            let action = legal[(rng.next() % legal.len() as u64) as usize];
            game.apply(action).expect("native legal opening move");
            opening.push(action.to_string());
        }
        if opening.len() == plies as usize
            && game.outcome().is_none()
            && game.policy_claim().is_none()
            && !game.position().in_check()
            && chess_eval::decompose(&ChessPosition::from_position(game.position()))
                .white_score()
                .abs()
                <= 200
        {
            accepted = true;
            break;
        }
    }
    let initial_fen = game.position().to_fen();
    let mut moves = Vec::new();
    let mut samples = Vec::new();
    let mut claim = None;
    let mut reason = "opening_rejected";
    let mut total_nodes = 0u64;
    if accepted {
        let mut engines = [ChessEngine::new(16), ChessEngine::new(16)];
        loop {
            if game.outcome().is_some() {
                reason = "native_outcome";
                break;
            }
            if started.elapsed() >= deadline {
                reason = "wall_limit";
                break;
            }
            if let Some(request) = game.policy_claim() {
                claim = Some(format!("{request:?}"));
                game.claim(request).expect("native available claim");
                continue;
            }
            if opening.len() + moves.len() >= 512 {
                reason = "ply_limit";
                break;
            }
            if moves.len() % 4 == 0 && quiet(&game) {
                samples.push(
                    json!({"ply": opening.len() + moves.len(), "fen": game.position().to_fen()}),
                );
            }
            let side = usize::from(game.position().side_to_move() == Color::Black);
            let searched = engines[side].search(
                &game,
                SearchLimits {
                    depth: 32,
                    nodes: Some(nodes),
                },
                SearchOptions::default(),
                || started.elapsed() >= deadline,
            );
            let result = match searched {
                Ok(value) => value.result,
                Err(error) => {
                    reason = "search_error";
                    claim = Some(format!("{error:?}"));
                    break;
                }
            };
            total_nodes += result.nodes;
            if started.elapsed() >= deadline {
                reason = "wall_limit";
                break;
            }
            match result.decision {
                Some(SearchDecision::Choose(SearchChoice::Move(action))) => {
                    game.apply(action).expect("native engine move");
                    moves.push(action.to_string());
                }
                Some(SearchDecision::Choose(SearchChoice::Claim(request))) => {
                    claim = Some(format!("{request:?}"));
                    game.claim(request).expect("native engine claim");
                }
                Some(SearchDecision::Finished(_)) => {
                    assert!(
                        game.outcome().is_some(),
                        "engine result is not a native outcome"
                    );
                }
                None => {
                    reason = "no_decision";
                    break;
                }
            }
        }
    }
    let outcome = game.outcome();
    let label = match outcome {
        Some(ChessOutcome::Checkmate {
            winner: Color::White,
        }) => Some(1.0),
        Some(ChessOutcome::Checkmate {
            winner: Color::Black,
        }) => Some(0.0),
        Some(ChessOutcome::Draw(_)) => Some(0.5),
        None => None,
    };
    // Unfinished/rejected games remain receipts, but supply no training rows.
    if !accepted || label.is_none() {
        samples.clear();
    }
    for sample in &mut samples {
        sample["white_outcome"] = json!(label);
    }
    json!({
        "schema": 1, "game_id": id, "family_id": id, "master_seed": master_seed,
        "seed": seed, "opening_attempts": attempts, "accepted_opening": accepted,
        "opening_moves": opening, "initial_fen": initial_fen, "moves": moves,
        "outcome": outcome, "claim": claim, "white_outcome": if accepted { label } else { None },
        "samples": samples, "truncated": accepted && outcome.is_none(), "stop": reason,
        "nodes_per_move": nodes, "search_depth_limit": 32, "nodes": total_nodes,
        "elapsed_seconds": started.elapsed().as_secs_f64(),
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 4 {
        return Err("usage: chess_selfplay START_GAME COUNT NODES SEED".into());
    }
    let start: u64 = args[0].parse()?;
    let count: u64 = args[1].parse()?;
    let nodes: u64 = args[2].parse()?;
    let seed: u64 = args[3].parse()?;
    if nodes == 0 {
        return Err("NODES must be positive".into());
    }
    let end = start.checked_add(count).ok_or("game ID overflow")?;
    let mut out = io::BufWriter::new(io::stdout().lock());
    for id in start..end {
        serde_json::to_writer(&mut out, &generate(id, nodes, seed))?;
        writeln!(out)?;
        out.flush()?;
    }
    Ok(())
}
