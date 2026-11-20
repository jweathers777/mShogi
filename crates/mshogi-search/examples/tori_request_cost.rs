//! Time the native and search steps of one Tori request against game length
//! (MSH-204 diagnosis).
use mshogi_core::tori::*;
use mshogi_search::tori as search;
use std::time::Instant;

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}

fn main() {
    let mut game = Game::standard();
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    for ply in 0..=200 {
        if ply % 40 == 0 {
            let t = Instant::now();
            let record = GameRecord::from_game(&game).unwrap();
            let a = ms(t);
            let t = Instant::now();
            let _ = record.to_game().unwrap();
            let b = ms(t);
            let t = Instant::now();
            let legal = game.legal_actions();
            let c = ms(t);
            let mut copy = record.to_game().unwrap();
            let t = Instant::now();
            copy.submit(GameRequest {
                expected: copy.context(),
                actor: copy.position().side_to_move(),
                command: GameCommand::Play { action: legal[0] },
            })
            .unwrap();
            let d = ms(t);
            let t = Instant::now();
            let _ = search::search(
                &game,
                search::Limits {
                    depth: 1,
                    nodes: 1_000_000,
                },
            );
            let e = ms(t);
            println!(
                "ply {ply}: from_game {a:.2} to_game {b:.2} legal {c:.3} submit {d:.3} search_d1 {e:.2} ms"
            );
        }
        if game.conclusion().is_some() {
            println!("ended at {ply}");
            break;
        }
        let legal = game.legal_actions();
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play {
                action: legal[(seed % legal.len() as u64) as usize],
            },
        })
        .unwrap();
    }
}
