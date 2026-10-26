//! Verification-only JSONL bridge. Not a console/engine protocol or bundled app.
use mshogi_core::shogi::{Action, Position};
use serde::Deserialize;
use serde_json::json;
use std::io::{self, Read};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    root_sfen: String,
    moves: Vec<String>,
    #[serde(default)]
    perft_depth: Option<u32>,
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().take(65_537).read_to_string(&mut input)?;
    if input.len() > 65_536 {
        return Err("probe input exceeds 64 KiB".into());
    }
    let request: Request = serde_json::from_str(&input)?;
    if request.moves.len() > 512 || request.perft_depth.is_some_and(|d| d > 2) {
        return Err("probe supports at most 512 plies and perft depth 2".into());
    }
    let mut position = Position::from_sfen(&request.root_sfen)?;
    let root = position.clone();
    let mut prefixes = Vec::new();
    let mut records = Vec::new();
    for text in &request.moves {
        let action: Action = text.parse()?;
        prefixes.push(position.clone());
        records.push(position.apply(action)?);
    }
    let mut response = json!({
        "sfen": position.to_sfen(),
        "legal_moves": position.legal_actions().iter().map(ToString::to_string).collect::<Vec<_>>(),
        "in_check": position.is_in_check(position.side_to_move()),
    });
    if let Some(depth) = request.perft_depth {
        response["perft"] = json!(position.perft(depth)?);
        response["divide"] = if depth == 0 {
            json!([])
        } else {
            json!(
                position
                    .divide(depth)?
                    .into_iter()
                    .map(|entry| json!({"action":entry.action.to_string(),"nodes":entry.nodes}))
                    .collect::<Vec<_>>()
            )
        };
    }
    while let Some(record) = records.pop() {
        position.undo(record)?;
        if position != prefixes.pop().ok_or("missing prefix")? {
            return Err("prefix restoration mismatch".into());
        }
    }
    response["undo_restored"] = json!(position == root);
    println!("{response}");
    Ok(())
}
fn main() {
    if let Err(error) = run() {
        eprintln!("shogi probe failed: {error}");
        std::process::exit(1);
    }
}
