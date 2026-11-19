//! Write one paired Tori Shogi opening for the native experiment controller.
//!
//! Usage: CANDIDATE BASELINE SEED CANDIDATE_NODES BASELINE_DEPTH BASELINE_NODES
//! MAX_STEPS WALL_MS OUT.json [--openings CATALOG --opening-index N]
//! [--random-plies K] [--clock-base-ms B --clock-increment-ms I]
//! [--watchdog-ms W]. With a catalog, the opening is the catalog row's native
//! notation moves from the standard root (MSH-204, Tori Strength Harness);
//! without one, the standard root plus four seeded random legal actions
//! (MSH-165). Native admission checks every action.
use mshogi_core::{
    native::{CanonicalRecord, RulesProfile},
    tori::{Game, GameCommand, GameRecord, GameRequest, notation},
};
#[path = "support/native_match_config.rs"]
mod native_match_config;

fn play(game: &mut Game, action: mshogi_core::tori::Action) {
    game.submit(GameRequest {
        expected: game.context(),
        actor: game.position().side_to_move(),
        command: GameCommand::Play { action },
    })
    .unwrap();
}

/// Native game of a catalog row: notation moves from the standard root.
pub fn opening_game(row: &serde_json::Value) -> Game {
    let mut game = Game::standard();
    for text in row["moves"].as_array().expect("moves") {
        let action = notation::parse_action(game.position(), text.as_str().unwrap())
            .unwrap_or_else(|e| panic!("opening {}: {e}", row["id"]));
        play(&mut game, action);
    }
    game
}

fn main() {
    let args = native_match_config::Args::read();
    let mut state = args.seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let mut random = |game: &mut Game, plies: usize| {
        for _ in 0..plies {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let legal = game.legal_actions();
            play(game, legal[(state % legal.len() as u64) as usize]);
        }
    };
    if let Some(path) = &args.openings {
        let text = std::fs::read_to_string(path).unwrap();
        let catalog: serde_json::Value = serde_json::from_str(&text).unwrap();
        let rows = catalog["openings"].as_array().expect("opening catalog");
        let row = rows
            .get(args.opening_index)
            .expect("opening index within catalog");
        let mut game = opening_game(row);
        assert!(args.random_plies <= 8, "at most eight random plies");
        random(&mut game, args.random_plies);
        assert!(game.conclusion().is_none(), "opening concluded");
        args.write(
            "MSH-204",
            "Tori",
            RulesProfile::ToriModernV1,
            CanonicalRecord::tori(GameRecord::from_game(&game).unwrap()),
            row["id"].as_str().unwrap().into(),
            format!(
                "Frozen catalog SHA256 {}; index {}; source/rights: {}{}",
                mshogi_runner::experiments::digest(&text),
                args.opening_index,
                catalog["provenance"],
                if args.random_plies > 0 {
                    format!(
                        "; plus {} seeded random legal plies (seed {})",
                        args.random_plies, args.seed
                    )
                } else {
                    String::new()
                }
            ),
        );
        return;
    }
    let mut game = Game::standard();
    random(&mut game, 4);
    assert!(game.conclusion().is_none());
    args.write(
        "MSH-165",
        "Tori",
        RulesProfile::ToriModernV1,
        CanonicalRecord::tori(GameRecord::from_game(&game).unwrap()),
        format!("tori-random4-s{}", args.seed),
        "standard Tori position plus four seeded random legal actions".into(),
    );
}
