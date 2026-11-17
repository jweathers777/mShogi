//! Write one paired Shogi opening for the native experiment controller.
//!
//! Usage: CANDIDATE BASELINE SEED CANDIDATE_NODES BASELINE_DEPTH BASELINE_NODES
//! MAX_STEPS WALL_MS OUT.json. Seed zero is the standard root. Other seeds use
//! four to twelve native legal opening actions, always an even number. Seeds
//! 1,000,000 through 1,000,003 select reduced tactical calibration roots only.
use mshogi_core::{
    native::{CanonicalRecord, RulesProfile},
    shogi::{
        CounterSource, Game, GameCommand, GameRecord, GameRequest, GameRoot, Position, RootCategory,
    },
};
#[path = "support/native_match_config.rs"]
mod native_match_config;

fn main() {
    let args = native_match_config::Args::read();
    if let Some(path) = &args.openings {
        let text = std::fs::read_to_string(path).unwrap();
        let catalog: serde_json::Value = serde_json::from_str(&text).unwrap();
        let rows = catalog["openings"].as_array().expect("opening catalog");
        let row = rows
            .get(args.opening_index)
            .expect("opening index within catalog");
        let game = opening_game(row);
        args.write(
            "MSH-186",
            "Shogi",
            RulesProfile::ShogiCsa36V1,
            CanonicalRecord::shogi(GameRecord::from_game(&game)),
            row["id"].as_str().unwrap().into(),
            format!(
                "Frozen catalog SHA256 {}; index {}; source/rights: {}",
                mshogi_runner::experiments::digest(&text),
                args.opening_index,
                catalog["provenance"]
            ),
        );
        return;
    }
    let seed = args.seed;
    const TACTICAL: [&str; 4] = [
        "8k/9/9/4p4/4R4/9/9/9/K8 b - 1",
        "8k/9/9/4P4/9/9/9/9/K8 b - 1",
        "3lkl3/3p1p3/4K4/9/9/9/9/9/9 b G 1",
        "4k4/9/4R4/9/9/9/9/9/K8 w - 1",
    ];
    let tactical = seed
        .checked_sub(1_000_000)
        .and_then(|n| TACTICAL.get(n as usize));
    let mut game = tactical.map_or_else(Game::new, |sfen| {
        Game::from_root(
            GameRoot::import(
                Position::from_sfen(sfen).unwrap(),
                RootCategory::ReducedMaterialDiagram,
                CounterSource::Supplied,
            )
            .unwrap(),
        )
    });
    let mut state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let plies = if seed == 0 || tactical.is_some() {
        0
    } else {
        4 + 2 * (seed % 5)
    };
    for _ in 0..plies {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        let legal = game.position().legal_actions();
        let action = legal[state as usize % legal.len()];
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play(action),
        })
        .unwrap();
    }
    assert!(game.conclusion().is_none());
    let id = if tactical.is_some() {
        format!("shogi-tactical-s{seed}")
    } else {
        format!("shogi-random{plies}-s{seed}")
    };
    args.write(
        "MSH-180",
        "Shogi",
        RulesProfile::ShogiCsa36V1,
        CanonicalRecord::shogi(GameRecord::from_game(&game)),
        id,
        if tactical.is_some() {
            "reduced native regression diagram; calibration only".into()
        } else {
            format!("standard Shogi plus {plies} seeded native legal actions; xorshift64")
        },
    );
}

fn opening_game(row: &serde_json::Value) -> Game {
    let mut game = if let Some(sfen) = row["sfen"].as_str() {
        let position = Position::from_sfen(sfen).unwrap();
        let root = GameRoot::import(
            position.clone(),
            RootCategory::FullMaterialDiagram,
            CounterSource::Supplied,
        )
        .or_else(|_| {
            GameRoot::import(
                position,
                RootCategory::ReducedMaterialDiagram,
                CounterSource::Supplied,
            )
        })
        .unwrap();
        Game::from_root(root)
    } else {
        Game::new()
    };
    for token in row["moves"].as_array().expect("explicit opening moves") {
        let action = token.as_str().unwrap().parse().unwrap();
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play(action),
        })
        .unwrap();
    }
    assert!(game.conclusion().is_none(), "opening already concluded");
    game
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn catalog_moves_preserve_native_history_and_sfen() {
        let game = opening_game(&serde_json::json!({"moves":["7g7f"]}));
        let expected = "lnsgkgsnl/1r5b1/ppppppppp/9/9/2P6/PP1PPPPPP/1B5R1/LNSGKGSNL w - 2";
        assert_eq!(game.position().to_sfen(), expected);
        assert_eq!(GameRecord::from_game(&game).event_count(), 1);
        let imported = opening_game(&serde_json::json!({"sfen":expected,"moves":[]}));
        assert_eq!(imported.position().to_sfen(), expected);
        assert_eq!(GameRecord::from_game(&imported).event_count(), 0);
    }
    #[test]
    #[should_panic]
    fn illegal_catalog_move_is_rejected() {
        opening_game(&serde_json::json!({"moves":["7g7e"]}));
    }
}
