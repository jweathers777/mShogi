//! MSH-204 (Tori Strength Harness): generate and screen Tori opening candidates.
//!
//! Usage:
//!   tori_openings time
//!   tori_openings candidates COUNT PLIES CHOICE_DEPTH MARGIN SEED OUT.json
//!   tori_openings screen CANDIDATES.json DEPTH OUT.json
//!
//! Candidates: from the standard root, each ply scores every legal action by
//! a fixed-depth search of the resulting position (negated, so the mover's
//! view) and chooses uniformly (seeded) among actions within MARGIN of the
//! best. Distinct final positions only. Screen: one fixed-depth search of each
//! final position from the side to move. Admission (a fixed limit) is applied
//! by the catalog writer, not here. Native admission checks every action.
use mshogi_core::tori::{Game, GameCommand, GameRequest, notation};
use mshogi_search::tori as search;
use serde_json::{Value, json};
use std::{collections::BTreeSet, time::Instant};

fn play(game: &mut Game, action: mshogi_core::tori::Action) {
    game.submit(GameRequest {
        expected: game.context(),
        actor: game.position().side_to_move(),
        command: GameCommand::Play { action },
    })
    .unwrap();
}

fn score(game: &Game, depth: u16) -> i32 {
    let r = search::search(
        game,
        search::Limits {
            depth,
            nodes: search::MAX_NODES,
        },
    )
    .unwrap();
    assert_eq!(r.completed_depth, depth, "depth not completed");
    r.score.unwrap()
}

/// Mover's view score of each legal action at a fixed depth.
fn scored(game: &Game, depth: u16) -> Vec<(mshogi_core::tori::Action, i32)> {
    game.legal_actions()
        .into_iter()
        .map(|a| {
            let mut child = game.fork();
            play(&mut child, a);
            let s = if child.conclusion().is_some() {
                search::terminal_score(
                    child.conclusion().unwrap(),
                    game.position().side_to_move(),
                    1,
                )
            } else {
                -score(&child, depth)
            };
            (a, s)
        })
        .collect()
}

fn rows(path: &str) -> Vec<Value> {
    let v: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    v["openings"].as_array().unwrap().clone()
}

fn replay(moves: &[Value]) -> Game {
    let mut game = Game::standard();
    for m in moves {
        let a = notation::parse_action(game.position(), m.as_str().unwrap()).unwrap();
        play(&mut game, a);
    }
    game
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args[1].as_str() {
        "time" => {
            let mut game = Game::standard();
            for _ in 0..8 {
                let a = game.legal_actions()[0];
                play(&mut game, a);
            }
            for depth in 3..=10 {
                let t = Instant::now();
                let s = score(&game, depth);
                println!(
                    "depth {depth}: score {s} in {:.0} ms",
                    t.elapsed().as_secs_f64() * 1e3
                );
            }
            let t = Instant::now();
            let n = scored(&Game::standard(), 4).len();
            println!(
                "root scoring of {n} actions at depth 4: {:.0} ms",
                t.elapsed().as_secs_f64() * 1e3
            );
        }
        "candidates" => {
            let count: usize = args[2].parse().unwrap();
            let plies: usize = args[3].parse().unwrap();
            let depth: u16 = args[4].parse().unwrap();
            let margin: i32 = args[5].parse().unwrap();
            let mut state: u64 = args[6]
                .parse::<u64>()
                .unwrap()
                .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                | 1;
            let mut next = move || {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                state
            };
            let mut seen = BTreeSet::new();
            let mut out = vec![];
            let mut attempts = 0;
            while out.len() < count {
                attempts += 1;
                assert!(attempts < count * 20, "too few distinct openings");
                let mut game = Game::standard();
                let mut moves = vec![];
                for _ in 0..plies {
                    let options = scored(&game, depth);
                    let best = options.iter().map(|o| o.1).max().unwrap();
                    let near: Vec<_> = options.iter().filter(|o| o.1 >= best - margin).collect();
                    let (a, _) = near[(next() % near.len() as u64) as usize];
                    moves.push(notation::format_action(game.position(), *a).unwrap());
                    play(&mut game, *a);
                    assert!(game.conclusion().is_none());
                }
                let key = format!("{:?}", game.repetition_key());
                if seen.insert(key) {
                    out.push(json!({"id": format!("tori-c{:03}", out.len() + 1), "moves": moves}));
                }
            }
            let doc = json!({
                "rule": format!(
                    "{plies} plies from the standard root; each ply chooses uniformly (xorshift seed {}) among \
                     legal actions whose depth-{depth} search score (mover's view) is within {margin} of the best; \
                     distinct final positions only; {attempts} attempts",
                    args[6]
                ),
                "openings": out,
            });
            std::fs::write(&args[7], serde_json::to_string_pretty(&doc).unwrap()).unwrap();
            println!("{} candidates in {attempts} attempts", count);
        }
        "screen" => {
            let depth: u16 = args[3].parse().unwrap();
            let mut out = vec![];
            for row in rows(&args[2]) {
                let game = replay(row["moves"].as_array().unwrap());
                let t = Instant::now();
                let s = score(&game, depth);
                out.push(json!({"id": row["id"], "depth": depth, "score": s,
                    "side_to_move": format!("{:?}", game.position().side_to_move()),
                    "ms": t.elapsed().as_millis() as u64}));
            }
            std::fs::write(
                &args[4],
                serde_json::to_string_pretty(&json!({"positions": out})).unwrap(),
            )
            .unwrap();
        }
        other => panic!("unknown mode {other}"),
    }
}
