//! Common finite paired-match settings. Native game code supplies the root.
use super::positions::{self, Result};
use mshogi_core::native::RulesProfile;
use mshogi_engine::WorkLimits;
use mshogi_runner::experiments::{
    Config, FischerClock, Opening, Player, ProfileLimits, SCHEMA, Split,
};
use serde_json::{Value, json};

pub struct Args {
    values: Vec<String>,
    pub seed: u64,
    watchdog_ms: u64,
    pub openings: Option<String>,
    pub opening_index: usize,
    /// Seeded random legal plies appended to a catalog opening (self-play corpora).
    pub random_plies: usize,
    clock: Option<FischerClock>,
    /// The candidate's clock is this multiple of the shared clock (time odds).
    candidate_time_factor: u64,
}
impl Args {
    fn read(mut values: Vec<String>) -> (Self, RulesProfile) {
        let mut watchdog_ms = None;
        let mut openings = None;
        let mut opening_index = 0;
        let mut random_plies = 0;
        let mut base = None;
        let mut increment = None;
        let mut candidate_time_factor = 1;
        let mut profile = None;
        let positional = 10;
        while values.len() > positional {
            let flag = values.remove(positional);
            let value = values.remove(positional);
            match flag.as_str() {
                "--profile" => {
                    profile =
                        Some(serde_json::from_value(json!(value)).expect("supported whole profile"))
                }
                "--watchdog-ms" => watchdog_ms = Some(value.parse().unwrap()),
                "--openings" => openings = Some(value),
                "--opening-index" => opening_index = value.parse().unwrap(),
                "--random-plies" => random_plies = value.parse().unwrap(),
                "--clock-base-ms" => base = Some(value.parse().unwrap()),
                "--clock-increment-ms" => increment = Some(value.parse().unwrap()),
                "--candidate-time-factor" => candidate_time_factor = value.parse().unwrap(),
                _ => panic!("unknown flag {flag}"),
            }
        }
        let watchdog_ms = watchdog_ms.unwrap_or(if base.is_some() { 60_000 } else { 30_000 });
        assert!((1..=60_000).contains(&watchdog_ms));
        assert_eq!(
            values.len(),
            positional,
            "expected native config positional arguments"
        );
        assert_eq!(
            base.is_some(),
            increment.is_some(),
            "both clock flags required"
        );
        let clock = base.map(|base_ms| FischerClock {
            base_ms,
            increment_ms: increment.unwrap(),
        });
        (
            Self {
                seed: values[3].parse().unwrap(),
                values,
                watchdog_ms,
                openings,
                opening_index,
                random_plies,
                clock,
                candidate_time_factor,
            },
            profile.expect("profile required"),
        )
    }
    pub fn write(values: Vec<String>) -> Result<()> {
        let (args, profile) = Self::read(values);
        let (row, id, provenance, plies) = if let Some(path) = &args.openings {
            let text = std::fs::read_to_string(path)?;
            let catalog: Value = serde_json::from_str(&text)?;
            let row = catalog["openings"]
                .get(args.opening_index)
                .ok_or("catalog index out of range")?;
            (
                row.clone(),
                row["id"].as_str().ok_or("missing opening id")?.to_owned(),
                format!(
                    "Frozen catalog {} row {}; {} seeded additional plies",
                    mshogi_runner::experiments::digest(&text),
                    args.opening_index,
                    args.random_plies
                ),
                args.random_plies,
            )
        } else {
            let seed = args.seed;
            let plies = if seed == 0 {
                0
            } else {
                4 + 2 * (seed % 5) as usize
            };
            (
                json!({}),
                format!("seed-{seed}"),
                format!("Standard root plus {plies} seeded native legal actions"),
                plies,
            )
        };
        let prepared = positions::prepare(profile, &row, args.seed, plies, false)?;
        if !prepared["score_first_twice"].is_null() || prepared["stopped"] == true {
            return Err("opening has ended".into());
        }
        args.config(
            profile,
            serde_json::from_value(prepared["root"].clone())?,
            row["group"].as_str().unwrap_or(&id).to_owned(),
            id,
            provenance,
        )
    }

    fn config(
        &self,
        profile: RulesProfile,
        root: mshogi_runner::experiments::Root,
        group: String,
        id: String,
        provenance: String,
    ) -> Result<()> {
        assert!(
            (1..=8).contains(&self.candidate_time_factor)
                && (self.candidate_time_factor == 1 || self.clock.is_some()),
            "time odds need a clock and a factor of 1..=8"
        );
        let candidate_clock = self
            .clock
            .filter(|_| self.candidate_time_factor > 1)
            .map(|c| FischerClock {
                base_ms: c.base_ms * self.candidate_time_factor,
                increment_ms: c.increment_ms * self.candidate_time_factor,
            });
        let player = |id: &str, executable: &str, depth: u16, nodes: u64| Player {
            clock: None,
            id: id.into(),
            executable: std::fs::canonicalize(executable)
                .unwrap()
                .to_str()
                .unwrap()
                .into(),
            build_provenance: "Executable hashes and campaign manifest record the builds".into(),
            limits: vec![ProfileLimits {
                profile,
                work: WorkLimits {
                    depth,
                    nodes,
                    time_ms: None,
                },
            }],
        };
        let config = Config {
            schema: SCHEMA,
            id: format!("strength-{id}"),
            protocol: format!(
                "Paired predecessor comparison. Native rules control outcomes. Opening provenance is recorded below. Mode: {}. Not a rating.",
                if self.clock.is_some() {
                    "Fischer wall clock with finite search safety caps"
                } else {
                    "fixed work"
                }
            ),
            seed: self.seed,
            rounds: 1,
            max_steps: self.values[7].parse().unwrap(),
            wall_budget_ms: self.values[8].parse().unwrap(),
            watchdog_ms: self.watchdog_ms,
            cleanup_ms: 1_000,
            clock: self.clock,
            players: vec![
                Player {
                    clock: candidate_clock,
                    ..player(
                        "candidate",
                        &self.values[1],
                        if profile == RulesProfile::ChessMatchV1 {
                            mshogi_engine::MAX_ANALYSIS_DEPTH
                        } else {
                            64
                        },
                        self.values[4].parse().unwrap(),
                    )
                },
                player(
                    "baseline",
                    &self.values[2],
                    self.values[5].parse().unwrap(),
                    self.values[6].parse().unwrap(),
                ),
            ],
            openings: vec![Opening {
                id,
                group,
                root,
                split: Split::Test,
                provenance,
            }],
        };
        config.validate()?;
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.values[9])?;
        file.write_all(serde_json::to_string_pretty(&config)?.as_bytes())?;
        Ok(())
    }
}
