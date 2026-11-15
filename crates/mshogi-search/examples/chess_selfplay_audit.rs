//! Replay generator receipts through native rules. Does not compute fitting loss.
//! Usage: chess_selfplay_audit GAME_JSONL [GAME_JSONL ...]
use mshogi_core::chess::{AdjudicatedGame, ChessOutcome, ClaimPolicy, Color, Position};
use mshogi_search::{chess_bitboard::ChessPosition, chess_eval};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn require(ok: bool, why: &str) -> Result<()> {
    if ok { Ok(()) } else { Err(why.into()) }
}
fn pieces(p: &Position) -> usize {
    p.to_fen()
        .split_whitespace()
        .next()
        .unwrap()
        .chars()
        .filter(char::is_ascii_alphabetic)
        .count()
}
fn quiet(p: &Position) -> bool {
    let count = pieces(p);
    !p.in_check()
        && p.legal_actions().iter().all(|&action| {
            action.promotion.is_none()
                && pieces(&p.project(action).expect("native legal projection")) == count
        })
        && chess_eval::decompose(&ChessPosition::from_position(p))
            .specialized
            .is_none()
}
fn audit(row: &Value) -> Result<(usize, usize, bool)> {
    require(row["schema"] == 1, "schema")?;
    require(
        row["family_id"] == row["game_id"],
        "family ID differs from game ID",
    )?;
    let opening = row["opening_moves"].as_array().ok_or("opening array")?;
    let moves = row["moves"].as_array().ok_or("move array")?;
    let samples = row["samples"].as_array().ok_or("sample array")?;
    let accepted = row["accepted_opening"].as_bool().ok_or("accepted flag")?;
    if accepted {
        require((8..=16).contains(&opening.len()), "opening length")?;
    }
    require(opening.len() + moves.len() <= 512, "ply bound")?;
    let mut game = AdjudicatedGame::new(ClaimPolicy::AutoClaim);
    for action in opening {
        require(
            game.policy_claim().is_none(),
            "opening crossed an available policy claim",
        )?;
        game.apply(action.as_str().ok_or("opening action")?.parse()?)?;
    }
    require(
        row["initial_fen"] == game.position().to_fen(),
        "initial FEN differs from opening replay",
    )?;
    let mut expected = BTreeMap::new();
    for (ply, action) in moves.iter().enumerate() {
        require(game.outcome().is_none(), "move after outcome")?;
        require(
            game.policy_claim().is_none(),
            "move after available policy claim",
        )?;
        if accepted && ply % 4 == 0 && quiet(game.position()) {
            expected.insert(opening.len() + ply, game.position().to_fen());
        }
        game.apply(action.as_str().ok_or("action string")?.parse()?)?;
    }
    if !row["claim"].is_null() && row["stop"] == "native_outcome" {
        let claim = game.policy_claim().ok_or("recorded claim not available")?;
        require(
            row["claim"] == format!("{claim:?}"),
            "claim differs from native policy claim",
        )?;
        game.claim(claim)?;
    }
    require(
        serde_json::to_value(game.outcome())? == row["outcome"],
        "native outcome differs",
    )?;
    let label = if accepted {
        match game.outcome() {
            Some(ChessOutcome::Draw(_)) => Some(0.5),
            Some(ChessOutcome::Checkmate {
                winner: Color::White,
            }) => Some(1.0),
            Some(ChessOutcome::Checkmate {
                winner: Color::Black,
            }) => Some(0.0),
            None => None,
        }
    } else {
        None
    };
    require(row["white_outcome"] == json!(label), "game outcome label")?;
    require(
        row["truncated"] == json!(accepted && game.outcome().is_none()),
        "truncated flag",
    )?;
    if label.is_none() {
        require(
            samples.is_empty(),
            "unfinished/rejected game supplies samples",
        )?;
    } else {
        require(row["stop"] == "native_outcome", "finished game stop reason")?;
        require(samples.len() == expected.len(), "quiet sample count")?;
        for sample in samples {
            let ply = usize::try_from(sample["ply"].as_u64().ok_or("sample ply")?)?;
            let fen = expected
                .remove(&ply)
                .ok_or("duplicate or unexpected sample ply")?;
            require(
                sample["fen"] == fen,
                "sample FEN differs from native replay",
            )?;
            require(
                sample["white_outcome"] == json!(label),
                "sample label differs from outcome",
            )?;
        }
    }
    Ok((opening.len() + moves.len(), samples.len(), label.is_some()))
}
fn main() -> Result<()> {
    let mut ids = HashSet::new();
    let (mut games, mut plies, mut samples, mut finished) = (0, 0, 0, 0);
    let paths: Vec<_> = std::env::args().skip(1).collect();
    require(
        !paths.is_empty(),
        "usage: chess_selfplay_audit GAME_JSONL ...",
    )?;
    for path in &paths {
        for (index, line) in BufReader::new(File::open(path)?).lines().enumerate() {
            let row: Value = serde_json::from_str(&line?)?;
            let id = row["game_id"].as_u64().ok_or("game ID")?;
            require(ids.insert(id), "duplicate game ID")?;
            let (p, s, f) =
                audit(&row).map_err(|e| format!("{path}:{} game {id}: {e}", index + 1))?;
            games += 1;
            plies += p;
            samples += s;
            finished += usize::from(f);
        }
    }
    println!(
        "{}",
        json!({"schema":1,"games":games,"native_plies":plies,"samples":samples,"finished":finished,"not_finished":games-finished,"passed":true,"loss_computed":false,"files":paths})
    );
    Ok(())
}
