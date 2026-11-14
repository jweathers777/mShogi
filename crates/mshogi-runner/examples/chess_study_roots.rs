//! Frozen MSH-49 starting-history sampler; no engine or candidate is evaluated.
use mshogi_core::{
    chess::{AdjudicatedGame, ClaimPolicy, Color, PieceKind, Position, Square},
    native::CanonicalRecord,
};
use mshogi_runner::OpeningConfig;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, error::Error, fs::OpenOptions, io::Write};
const POPULATION: &str = "msh49-six-families-uniform-native-4to8-material100-v1";
const FAMILIES: [&[&str]; 6] = [
    &[],
    &["e2e4", "e7e5", "g1f3", "b8c6"],
    &["d2d4", "d7d5", "c2c4", "e7e6"],
    &["c2c4", "e7e5", "b1c3", "g8f6"],
    &["g1f3", "d7d5", "g2g3", "g8f6"],
    &["e2e4", "e7e6", "d2d4", "d7d5"],
];
/// White material minus Black material in centipawns (pawn 100, knight 320,
/// bishop 330, rook 500, queen 900), the filter of the frozen population.
fn white_material_balance(position: &Position) -> i32 {
    let mut balance = 0;
    for rank in 0..8 {
        for file in 0..8 {
            let Some(piece) = position.piece_at(Square::new(file, rank).unwrap()) else {
                continue;
            };
            let value = match piece.kind {
                PieceKind::Pawn => 100,
                PieceKind::Knight => 320,
                PieceKind::Bishop => 330,
                PieceKind::Rook => 500,
                PieceKind::Queen => 900,
                PieceKind::King => 0,
            };
            balance += if piece.color == Color::White {
                value
            } else {
                -value
            };
        }
    }
    balance
}
struct Random<'a> {
    seed: &'a str,
    label: &'a str,
    draw: usize,
    attempt: u64,
    counter: u64,
}
impl Random<'_> {
    fn below(&mut self, upper: usize) -> usize {
        let upper = upper as u64;
        let ceiling = u64::MAX - u64::MAX % upper;
        loop {
            let hash = Sha256::digest(format!(
                "{POPULATION}:{}:{}:{}:{}:{}",
                self.seed, self.label, self.draw, self.attempt, self.counter
            ));
            self.counter += 1;
            let n = u64::from_le_bytes(hash[..8].try_into().unwrap());
            if n < ceiling {
                return (n % upper) as usize;
            }
        }
    }
}
#[derive(Serialize)]
struct Sample {
    opening: OpeningConfig,
    record: CanonicalRecord,
    attempts: usize,
    family: usize,
    extra_plies: usize,
}
#[derive(Serialize)]
struct Catalog<'a> {
    schema: u32,
    population: &'a str,
    seed: &'a str,
    label: &'a str,
    selection: &'a str,
    excluded_sha256: Option<String>,
    samples: Vec<Sample>,
}
fn generate<'a>(
    seed: &'a str,
    label: &'a str,
    count: usize,
    excluded: &BTreeSet<Vec<String>>,
    excluded_sha256: Option<String>,
) -> Result<Catalog<'a>, Box<dyn Error>> {
    let mut samples = vec![];
    for draw in 0..count {
        let mut header = Random {
            seed,
            label,
            draw,
            attempt: u64::MAX,
            counter: 0,
        };
        let family = header.below(FAMILIES.len());
        let extra = header.below(5) + 4;
        let mut accepted = None;
        for attempt in 0..10_000 {
            let mut rng = Random {
                seed,
                label,
                draw,
                attempt: attempt as u64,
                counter: 0,
            };
            let mut game = AdjudicatedGame::new(ClaimPolicy::AutoClaim);
            let mut moves: Vec<String> = FAMILIES[family].iter().map(|s| s.to_string()).collect();
            for action in &moves {
                game.apply(action.parse()?)?;
            }
            for _ in 0..extra {
                if game.outcome().is_some() || !game.available_claims().is_empty() {
                    break;
                }
                let mut legal = game.position().legal_actions();
                legal.sort_by_key(ToString::to_string);
                if legal.is_empty() {
                    break;
                }
                let action = legal[rng.below(legal.len())];
                game.apply(action)?;
                moves.push(action.to_string());
            }
            if moves.len() != FAMILIES[family].len() + extra
                || game.outcome().is_some()
                || !game.available_claims().is_empty()
                || white_material_balance(game.position()).abs() > 100
                || excluded.contains(&moves)
            {
                continue;
            }
            accepted = Some(Sample {
                opening: OpeningConfig {
                    id: format!("{label}-{draw:03}"),
                    fen: None,
                    moves,
                },
                record: CanonicalRecord::chess(game.record()),
                attempts: attempt + 1,
                family,
                extra_plies: extra,
            });
            break;
        }
        samples.push(accepted.ok_or("bounded sampler exhausted; do not widen acceptance")?);
    }
    Ok(Catalog {
        schema: 1,
        population: POPULATION,
        seed,
        label,
        selection: "Independent draws with replacement; family and4..8extra plies drawn once independently per root, heldfixed acrosscontinuationrejection; each native legal action uniform. Rejectnonlive/claimable,materialdifference>100cp,orfixeddevelopmenthistory. Material-onlyv1 filter, no candidate/opponent evaluation. Duplicateholdoutdraws retained.",
        excluded_sha256,
        samples,
    })
}
fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 5 {
        return Err(
            "usage: chess_study_roots SEED LABEL COUNT NEW_OUTPUT EXCLUDED_CATALOG_OR_DASH".into(),
        );
    }
    let count: usize = args[2].parse()?;
    if !(1..=64).contains(&count)
        || args[0].is_empty()
        || args[0].len() > 128
        || args[1].is_empty()
        || args[1].len() > 64
    {
        return Err("invalid bounded sampler arguments".into());
    }
    let mut excluded = BTreeSet::new();
    let excluded_sha256 = if args[4] != "-" {
        let raw = std::fs::read(&args[4])?;
        if raw.len() > 1_048_576 {
            return Err("exclusioncatalogexceeds1MiB".into());
        }
        let value: serde_json::Value = serde_json::from_slice(&raw)?;
        let samples = value["samples"]
            .as_array()
            .ok_or("missingexcludedsamples")?;
        for sample in samples {
            let opening: OpeningConfig = serde_json::from_value(sample["opening"].clone())?;
            if opening.fen.is_some() {
                return Err("samplerexclusionrequiresstandardroot".into());
            }
            excluded.insert(opening.moves);
        }
        Some(format!("{:x}", Sha256::digest(&raw)))
    } else {
        None
    };
    let catalog = generate(&args[0], &args[1], count, &excluded, excluded_sha256)?;
    let bytes = serde_json::to_vec(&catalog)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[3])?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    println!("generated {count} native roots; population={POPULATION}; no search performed");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sampled_native_histories_are_reproducible_and_domain_separated() {
        let excluded = BTreeSet::new();
        let a = generate("test-seed", "relative", 3, &excluded, None).unwrap();
        let again = generate("test-seed", "relative", 3, &excluded, None).unwrap();
        let b = generate("test-seed", "external", 3, &excluded, None).unwrap();
        assert_eq!(
            serde_json::to_vec(&a).unwrap(),
            serde_json::to_vec(&again).unwrap()
        );
        assert_ne!(a.samples[0].opening.moves, b.samples[0].opening.moves);
        for sample in a.samples {
            assert_eq!(
                sample.opening.moves.len(),
                FAMILIES[sample.family].len() + sample.extra_plies
            );
            let record = CanonicalRecord::from_json(&sample.record.to_json().unwrap()).unwrap();
            let CanonicalRecord::Chess(record) = record else {
                panic!("wrong family")
            };
            let game = AdjudicatedGame::replay_record(&record).unwrap();
            assert!(game.outcome().is_none() && game.available_claims().is_empty());
            assert!(white_material_balance(game.position()).abs() <= 100);
        }
    }
    #[test]
    fn excluding_development_history_preserves_drawn_family_and_length() {
        let a = generate("excluded-test", "relative", 1, &BTreeSet::new(), None).unwrap();
        let old = &a.samples[0];
        let excluded = BTreeSet::from([old.opening.moves.clone()]);
        let b = generate(
            "excluded-test",
            "relative",
            1,
            &excluded,
            Some("fixed-catalog".into()),
        )
        .unwrap();
        assert_eq!(
            (b.samples[0].family, b.samples[0].extra_plies),
            (old.family, old.extra_plies)
        );
        assert_ne!(b.samples[0].opening.moves, old.opening.moves);
        assert!(b.samples[0].attempts > old.attempts);
    }
}
