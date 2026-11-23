//! Fixed Chu development roots for the shared native match harness.
//! Arguments match shogi_match_config. Seed 0 is the standard full-game pilot;
//! 1..9999 select balanced reduced-position families; 10000..19999 select
//! standard-position calibration prefixes; 1000000..1000003 are tactical
//! calibration roots. None of these ranges is selected from game results.
use mshogi_core::{
    chu::{Game, GameRecord},
    native::{CanonicalRecord, RulesProfile},
};
#[path = "support/native_match_config.rs"]
mod native_match_config;

#[path = "support/chu_roots.rs"]
mod chu_roots;
use chu_roots::{compact, quiet_setup, tactical};

fn main() {
    let args = native_match_config::Args::read();
    if let Some(path) = &args.openings {
        let text = std::fs::read_to_string(path).unwrap();
        let catalog: serde_json::Value = serde_json::from_str(&text).unwrap();
        let row = &catalog["openings"][args.opening_index];
        let record: GameRecord = serde_json::from_value(row["record"].clone()).unwrap();
        let game = record.to_game().unwrap();
        assert!(game.conclusion().is_none(), "opening concluded");
        assert_eq!(args.random_plies, 0, "catalog records are frozen");
        args.write(
            "MSH-213",
            "Chu Shogi",
            RulesProfile::ChuJcsa2019V1,
            CanonicalRecord::chu(record),
            row["id"].as_str().unwrap().into(),
            format!(
                "Frozen catalog SHA256 {}; index {}; source: {}",
                mshogi_runner::experiments::digest(&text),
                args.opening_index,
                catalog["provenance"]
            ),
        );
        return;
    }
    let seed = args.seed;
    let (mut game, plies, id, provenance) = match seed {
        0 => (
            Game::standard(),
            0,
            "chu-standard-pilot".into(),
            "standard Chu root; separate feasibility pilot, not the reduced-position test".into(),
        ),
        1..=9999 => {
            let family = ((seed - 1) % 8) as usize;
            let plies = 4;
            (
                compact(family),
                plies,
                format!("chu-compact-f{family}-s{seed}"),
                format!(
                    "fixed balanced Chu diagram family {family}; equal mirrored material and royals; {plies} seeded quiet rear-zone setup plies (royals and Lions fixed); earlier history unknown"
                ),
            )
        }
        10000..=19999 => {
            let plies = if seed == 10000 { 0 } else { 4 + 2 * (seed % 5) };
            (
                Game::standard(),
                plies,
                format!("chu-standard-s{seed}"),
                format!(
                    "standard92-piece Chu root plus {plies} seeded quiet native setup plies; calibration only"
                ),
            )
        }
        1000000..=1000003 => (
            tactical(seed - 1000000),
            0,
            format!("chu-tactical-s{seed}"),
            "existing native regression diagram; tactical calibration only".into(),
        ),
        _ => panic!("unsupported fixed corpus seed"),
    };
    assert!(game.conclusion().is_none());
    quiet_setup(&mut game, seed, plies, (1..=9999).contains(&seed));
    args.write(
        "MSH-182",
        "Chu Shogi",
        RulesProfile::ChuJcsa2019V1,
        CanonicalRecord::chu(GameRecord::from_game(&game).unwrap()),
        id,
        provenance,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compact_families_have_equal_material_and_no_immediate_native_win() {
        for (family, expected_pieces) in [12, 16, 16, 20, 20, 24, 20, 24].into_iter().enumerate() {
            let game = compact(family);
            assert!(game.conclusion().is_none());
            assert_eq!(game.position().pieces().count(), expected_pieces);
            for (square, piece) in game.position().pieces() {
                let opposite = game.position().piece(square.rotated()).unwrap();
                assert_eq!(opposite.side, piece.side.opponent());
                assert_eq!(
                    (opposite.base, opposite.promoted),
                    (piece.base, piece.promoted)
                );
            }
            let record = GameRecord::from_game(&game).unwrap();
            for action in game.legal_actions() {
                let mut branch = record.to_game().unwrap();
                branch
                    .submit(GameRequest {
                        expected: branch.context(),
                        actor: branch.position().side_to_move(),
                        command: GameCommand::Play { action },
                    })
                    .unwrap();
                assert!(
                    branch.conclusion().is_none(),
                    "family {family} has an immediate terminal move"
                );
            }
        }
    }

    #[test]
    fn tactical_controls_are_live_native_roots() {
        for index in 0..4 {
            let game = tactical(index);
            assert!(game.conclusion().is_none());
            assert!(!game.legal_actions().is_empty());
        }
    }

    #[test]
    fn all_scheduled_compact_prefixes_keep_native_games_live_and_material_equal() {
        let mut positions = std::collections::BTreeSet::new();
        for seed in 1..=600 {
            let mut game = compact(((seed - 1) % 8) as usize);
            let pieces = game.position().pieces().count();
            quiet_setup(&mut game, seed, 4, true);
            assert!(game.conclusion().is_none());
            assert_eq!(game.events().len(), 4);
            assert_eq!(game.position().pieces().count(), pieces);
            positions.insert(serde_json::to_string(game.position().state()).unwrap());
        }
        eprintln!(
            "600 fixed seeds give {} distinct native position states",
            positions.len()
        );
    }
}
