//! Fixed Tenjiku development roots for the shared native match harness.
//! Arguments match shogi_match_config. Seed 0 is the standard full-game pilot;
//! 1..9999 select balanced reduced-position families; 10000..19999 select
//! standard-position calibration prefixes; 1000000..1000003 are tactical
//! calibration roots. None of these ranges is selected from game results.
use mshogi_core::{
    native::{CanonicalRecord, RulesProfile},
    tenjiku::{
        ActionKind, Base, Face, Game, GameCommand, GameRecord, GameRequest, Piece, Position,
        Profile, RootMetadata, Side, Square,
    },
};
#[path = "support/native_match_config.rs"]
mod native_match_config;

fn square(text: &str) -> Square {
    let (file, rank) = text.split_at(text.len() - 1);
    Square::new(file.parse().unwrap(), rank.as_bytes()[0] - b'a' + 1).unwrap()
}
fn diagram(profile: Profile, entries: &[(&str, Side, Base, bool)]) -> Game {
    let entries: Vec<_> = entries
        .iter()
        .enumerate()
        .map(|(id, (at, side, base, promoted))| {
            (
                square(at),
                Piece::new(id as u16, *side, *base, *promoted).unwrap(),
            )
        })
        .collect();
    Game::from_root(
        Position::from_diagram(profile, &entries, Side::Sente, RootMetadata::diagram(0)).unwrap(),
    )
    .unwrap()
}
/// Fixed mirrored material and royals; no reachability claim.
fn compact(profile: Profile, family: usize) -> Game {
    use Base::*;
    let mut pieces = vec![
        ("9o", K, false),
        ("9k", Ln, false),
        ("7i", P, false),
        ("10i", P, false),
        ("8n", G, false),
    ];
    let extra: &[(&str, Base, bool)] = match family {
        0 => &[
            ("13n", R, false),
            ("5o", B, false),
            ("10n", S, false),
            ("3n", I, false),
            ("4n", C, false),
            ("6n", N, false),
            ("11n", Dg, false),
        ],
        1 => &[
            ("13m", GGn, false),
            ("5n", BGn, false),
            ("12l", RGn, false),
            ("3n", FL, false),
            ("4n", BT, false),
            ("6n", L, false),
            ("10n", S, false),
        ],
        2 => &[
            ("13l", VGn, false),
            ("5m", LHk, false),
            ("10n", S, false),
            ("3n", RC, false),
            ("4n", VM, false),
            ("6n", SM, false),
            ("11n", SSo, false),
        ],
        3 => &[
            ("13l", FiD, false),
            ("4m", WBf, false),
            ("10n", S, false),
            ("3n", VSo, false),
            ("5n", DK, false),
            ("6n", DH, false),
            ("11n", FK, false),
        ],
        4 => &[
            ("9n", DE, true),
            ("13m", R, false),
            ("5o", B, false),
            ("10n", S, false),
        ],
        5 => &[
            ("3g", Ky, false),
            ("14h", Ph, false),
            ("12f", WBf, false),
            ("10n", S, false),
        ],
        6 => &[
            ("13n", ChS, false),
            ("5m", FEg, false),
            ("4l", SE, false),
            ("12m", HF, false),
            ("10n", S, false),
        ],
        7 => &[
            ("9n", DE, true),
            ("13m", GGn, false),
            ("5m", VGn, false),
            ("12k", FiD, false),
            ("4k", WBf, false),
            ("10n", S, false),
        ],
        _ => unreachable!(),
    };
    pieces.extend_from_slice(extra);
    let mut entries = Vec::new();
    for (at, base, promoted) in pieces {
        let square = square(at);
        let id = entries.len() as u16;
        entries.push((square, Piece::new(id, Side::Sente, base, promoted).unwrap()));
        entries.push((
            square.rotated(),
            Piece::new(id + 1, Side::Gote, base, promoted).unwrap(),
        ));
    }
    Game::from_root(
        Position::from_diagram(profile, &entries, Side::Sente, RootMetadata::diagram(0)).unwrap(),
    )
    .unwrap()
}
fn tactical(profile: Profile, index: u64) -> Game {
    use Base::*;
    use Side::*;
    let mut entries = vec![
        ("16p", Sente, K, false),
        ("1a", Gote, K, false),
        ("15p", Sente, G, false),
        ("2a", Gote, G, false),
    ];
    // Native local-test geometries augmented with distant kings and material.
    let extra: &[(&str, Side, Base, bool)] = match index {
        0 => &[
            ("8j", Sente, FiD, false),
            ("9h", Gote, FiD, false),
            ("8g", Gote, P, false),
            ("7i", Gote, G, false),
            ("7h", Sente, P, false),
        ],
        1 => &[
            ("8h", Sente, BGn, false),
            ("9g", Sente, P, false),
            ("11e", Gote, RGn, false),
        ],
        2 => &[
            ("8f", Sente, WBf, false),
            ("9e", Gote, P, false),
            ("10d", Gote, R, false),
        ],
        3 => &[
            ("8h", Sente, LHk, false),
            ("8g", Gote, P, false),
            ("9g", Gote, R, false),
            ("3b", Gote, DE, true),
        ],
        _ => panic!("unsupported tactical seed"),
    };
    entries.extend_from_slice(extra);
    diagram(profile, &entries)
}

fn quiet_setup(game: &mut Game, seed: u64, plies: u64, rear_only: bool) {
    assert!(game.events().is_empty());
    let mut other_state = serde_json::to_value(game.position().state()).unwrap();
    let other_profile = match game.profile() {
        Profile::AdamsModern => Profile::MullerCompleted,
        Profile::MullerCompleted => Profile::AdamsModern,
    };
    other_state["profile"] = serde_json::to_value(other_profile).unwrap();
    let mut other =
        Game::from_root(Position::install(serde_json::from_value(other_state).unwrap()).unwrap())
            .unwrap();
    let mut state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    for _ in 0..plies {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        // Setup changes neither material nor promotion. Every selected move is
        // still admitted by the full native Game, including repetition admission.
        let other_actions = other.legal_actions();
        let mut choices: Vec<_> = game
            .legal_actions()
            .into_iter()
            .filter(|a| {
                let source = game.position().piece(a.from).unwrap();
                let rear = |s: &Square| match source.side {
                    Side::Sente => s.rank() >= 14,
                    Side::Gote => s.rank() <= 3,
                };
                (!rear_only
                    || (!source.is_royal()
                        && !matches!(source.face(), Face::Ln | Face::LHk | Face::VGn | Face::FiD)
                        && a.route.iter().all(rear)))
                    && other_actions.contains(a)
                    && a.kind == ActionKind::Travel
                    && !a.promote
                    && a.route
                        .iter()
                        .all(|s| *s == a.from || game.position().piece(*s).is_none())
            })
            .collect();
        choices.sort_by_key(|action| serde_json::to_string(action).unwrap());
        assert!(!choices.is_empty());
        let action = choices[state as usize % choices.len()].clone();
        other
            .submit(GameRequest {
                expected: other.context(),
                actor: other.position().side_to_move(),
                command: GameCommand::Play {
                    action: action.clone(),
                },
            })
            .unwrap();
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play { action },
        })
        .unwrap();
        assert!(game.conclusion().is_none() && game.snapshot().incomplete.is_none());
        assert!(other.conclusion().is_none() && other.snapshot().incomplete.is_none());
        let mut normalized = serde_json::to_value(other.position().state()).unwrap();
        normalized["profile"] = serde_json::to_value(game.profile()).unwrap();
        assert_eq!(
            normalized,
            serde_json::to_value(game.position().state()).unwrap(),
            "setup must differ only by profile"
        );
    }
}

fn main() {
    let (args, profile_name) = native_match_config::Args::read_with_profile();
    let profile = match profile_name.as_str() {
        "adams" => Profile::AdamsModern,
        "muller" => Profile::MullerCompleted,
        _ => panic!("expected adams or muller"),
    };
    let seed = args.seed;
    let (mut game, plies, id, provenance) = match seed {
        0 => (
            Game::standard(profile),
            0,
            "tenjiku-standard-pilot".into(),
            "standard Tenjiku root; separate feasibility pilot, not the reduced-position test"
                .into(),
        ),
        1..=9999 => {
            let family = ((seed - 1) % 8) as usize;
            let plies = 4;
            (
                compact(profile, family),
                plies,
                format!("tenjiku-compact-f{family}-s{seed}"),
                format!(
                    "fixed balanced Tenjiku diagram family {family}; equal mirrored material and royals; {plies} seeded quiet rear-zone setup plies (royals and Lions fixed); earlier history unknown"
                ),
            )
        }
        10000..=19999 => {
            let plies = if seed == 10000 { 0 } else { 4 + 2 * (seed % 5) };
            (
                Game::standard(profile),
                plies,
                format!("tenjiku-standard-s{seed}"),
                format!(
                    "standard156-piece Tenjiku root plus {plies} seeded quiet native setup plies; calibration only"
                ),
            )
        }
        1000000..=1000003 => (
            tactical(profile, seed - 1000000),
            0,
            format!("tenjiku-tactical-s{seed}"),
            "existing native regression diagram; tactical calibration only".into(),
        ),
        _ => panic!("unsupported fixed corpus seed"),
    };
    assert!(game.conclusion().is_none() && game.snapshot().incomplete.is_none());
    quiet_setup(&mut game, seed, plies, (1..=9999).contains(&seed));
    args.write(
        "MSH-183",
        "Tenjiku Shogi",
        RulesProfile::from(profile),
        CanonicalRecord::tenjiku(GameRecord::from_game(&game).unwrap()),
        id,
        provenance,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn both_profiles_have_balanced_live_families_without_immediate_wins() {
        for profile in Profile::ALL {
            for (family, count) in [24, 24, 24, 24, 18, 18, 20, 22].into_iter().enumerate() {
                let game = compact(profile, family);
                assert!(game.conclusion().is_none());
                assert_eq!(game.position().pieces().count(), count);
                for (at, piece) in game.position().pieces() {
                    let opposite = game.position().piece(at.rotated()).unwrap();
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
                        "{profile:?} family{family} immediate terminal move"
                    );
                }
            }
        }
    }
    #[test]
    fn both_profiles_keep_all_scheduled_prefixes_live_without_material_change() {
        for profile in Profile::ALL {
            let mut distinct = std::collections::BTreeSet::new();
            for seed in 1..=600 {
                let mut game = compact(profile, ((seed - 1) % 8) as usize);
                let count = game.position().pieces().count();
                quiet_setup(&mut game, seed, 4, true);
                assert!(game.conclusion().is_none());
                assert_eq!(game.events().len(), 4);
                assert_eq!(game.position().pieces().count(), count);
                distinct.insert(serde_json::to_string(game.position().state()).unwrap());
            }
            eprintln!(
                "{profile:?}:600 seeds give {} distinct states",
                distinct.len()
            );
        }
    }
    #[test]
    fn families_cover_all_originals_and_keep_profiles_identical() {
        let mut originals = std::collections::BTreeSet::new();
        for family in 0..8 {
            let adams = compact(Profile::AdamsModern, family);
            let muller = compact(Profile::MullerCompleted, family);
            let mut normalized = serde_json::to_value(muller.position().state()).unwrap();
            normalized["profile"] = serde_json::to_value(Profile::AdamsModern).unwrap();
            assert_eq!(
                normalized,
                serde_json::to_value(adams.position().state()).unwrap()
            );
            originals.extend(adams.position().pieces().map(|(_, p)| p.base));
        }
        assert_eq!(originals, Base::ALL.into_iter().collect());
    }
    #[test]
    fn tactical_controls_are_live_in_both_profiles() {
        for profile in Profile::ALL {
            for index in 0..4 {
                let game = tactical(profile, index);
                assert!(game.conclusion().is_none());
                assert!(!game.legal_actions().is_empty());
            }
        }
    }
}
