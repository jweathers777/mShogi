//! Time the native and adapter steps of one Xiangqi search request against
//! game length (MSH-194 diagnosis).
use mshogi_core::xiangqi::*;
use mshogi_search::xiangqi as search;
use std::time::Instant;

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1000.0
}

fn main() {
    let mut game = Game::standard(ClaimPolicy::AutoValidClaims).unwrap();
    let mut seed = 0x9e37_79b9_7f4a_7c15u64;
    for ply in 0..=80 {
        if ply % 20 == 0 {
            let t = Instant::now();
            let record = GameRecord::from_game(&game).unwrap();
            let a = ms(t);
            let t = Instant::now();
            let copy = record.to_game().unwrap();
            let b = ms(t);
            let t = Instant::now();
            let _ = copy.no_win_assessment();
            let c = ms(t);
            let t = Instant::now();
            let _ = game.committed_prefix(game.events().len()).unwrap();
            let d = ms(t);
            let t = Instant::now();
            let _ = game.position().legal_actions();
            let e = ms(t);
            let t = Instant::now();
            let _ = search::search(
                &game,
                search::Limits {
                    depth: 1,
                    nodes: 1_000_000,
                },
            );
            let f = ms(t);
            println!(
                "ply {ply}: from_game {a:.2} to_game {b:.2} no_win {c:.2} committed_prefix {d:.2} legal {e:.2} search_d1 {f:.2} ms"
            );
        }
        let legal = game.position().legal_actions().unwrap();
        if legal.is_empty() || game.conclusion().is_some() || game.pending_adjudication().is_some()
        {
            println!("ended at ply {ply}");
            break;
        }
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let action = legal[seed as usize % legal.len()];
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play { action },
        })
        .unwrap();
    }
}
