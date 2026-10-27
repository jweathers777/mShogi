//! Developer-only bounded board probe for admitted-reference comparison.
//! Not a game protocol, console feature or product execution capability.
use mshogi_core::xiangqi::{Action, Position, Undo};
use serde::{Deserialize, Serialize};
use std::io::{self, Read};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    root_fen: String,
    #[serde(default)]
    moves: Vec<Action>,
    #[serde(default)]
    perft_depth: u8,
}
#[derive(Serialize)]
struct Branch {
    action: Action,
    nodes: u64,
}
#[derive(Serialize)]
struct Response {
    fen: String,
    legal_moves: Vec<String>,
    in_check: bool,
    undo_restored: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    perft: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    divide: Option<Vec<Branch>>,
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = Vec::new();
    io::stdin().take(4097).read_to_end(&mut input)?;
    if input.len() > 4096 {
        return Err("request byte limit4096".into());
    }
    // Direct typed parsing retains duplicate fields for rejection.
    let request: Request = serde_json::from_slice(&input)?;
    if request.moves.len() > 256 || request.perft_depth > 3 {
        return Err("replay/depth limit256/3".into());
    }
    let mut position = Position::from_fen(&request.root_fen)?;
    position.validate_local()?;
    let root = position.clone();
    let mut history: Vec<Undo> = Vec::new();
    let mut prefixes = Vec::new();
    for action in request.moves {
        prefixes.push(position.clone());
        history.push(position.apply(action)?);
    }
    let observed = position.clone();
    let actions = position.legal_actions()?;
    let mut legal_moves: Vec<_> = actions.iter().map(ToString::to_string).collect();
    legal_moves.sort();
    for action in actions {
        let undo = position.apply(action)?;
        position.undo(undo)?;
        if position != observed {
            return Err("choice restoration mismatch".into());
        }
    }
    let depth = request.perft_depth;
    let perft = if depth > 0 {
        Some(position.perft(depth)?)
    } else {
        None
    };
    let divide = if depth > 0 {
        Some(
            position
                .divide(depth)?
                .into_iter()
                .map(|(action, nodes)| Branch { action, nodes })
                .collect(),
        )
    } else {
        None
    };
    while let Some(undo) = history.pop() {
        position.undo(undo)?;
        if Some(position.clone()) != prefixes.pop() {
            return Err("prefix restoration mismatch".into());
        }
    }
    if position != root {
        return Err("root restoration mismatch".into());
    }
    let response = Response {
        fen: observed.to_fen()?,
        legal_moves,
        in_check: observed.is_in_check(observed.side_to_move()),
        undo_restored: true,
        perft,
        divide,
    };
    let output = serde_json::to_string(&response)?;
    if output.len() > 65536 {
        return Err("response limit65536".into());
    }
    println!("{output}");
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("native board probe refused: {error}");
        std::process::exit(1);
    }
}
