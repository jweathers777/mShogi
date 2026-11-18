//! Common finite paired-match settings. Native game code supplies the root.
use mshogi_core::native::{CanonicalRecord, RulesProfile};
use mshogi_engine::WorkLimits;
use mshogi_runner::experiments::{
    Config, FischerClock, Opening, Player, ProfileLimits, SCHEMA, Split, fresh_root,
};

#[allow(dead_code)]
pub struct Args {
    values: Vec<String>,
    pub seed: u64,
    watchdog_ms: u64,
    pub openings: Option<String>,
    pub opening_index: usize,
    /// Seeded random legal plies appended to a catalog opening (self-play corpora).
    pub random_plies: usize,
    clock: Option<FischerClock>,
}
impl Args {
    #[allow(dead_code)]
    pub fn read() -> Self {
        Self::parse(false).0
    }
    #[allow(dead_code)]
    pub fn read_with_profile() -> (Self, String) {
        let (args, profile) = Self::parse(true);
        (args, profile.unwrap())
    }
    fn parse(profile: bool) -> (Self, Option<String>) {
        let mut values: Vec<String> = std::env::args().collect();
        let mut watchdog_ms = None;
        let mut openings = None;
        let mut opening_index = 0;
        let mut random_plies = 0;
        let mut base = None;
        let mut increment = None;
        let positional = if profile { 11 } else { 10 };
        while values.len() > positional {
            let flag = values.remove(positional);
            let value = values.remove(positional);
            match flag.as_str() {
                "--watchdog-ms" => watchdog_ms = Some(value.parse().unwrap()),
                "--openings" => openings = Some(value),
                "--opening-index" => opening_index = value.parse().unwrap(),
                "--random-plies" => random_plies = value.parse().unwrap(),
                "--clock-base-ms" => base = Some(value.parse().unwrap()),
                "--clock-increment-ms" => increment = Some(value.parse().unwrap()),
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
        let profile = if profile { values.pop() } else { None };
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
            },
            profile,
        )
    }
    pub fn write(
        &self,
        task: &str,
        game: &str,
        profile: RulesProfile,
        record: CanonicalRecord,
        id: String,
        provenance: String,
    ) {
        let player = |id: &str, executable: &str, depth: u16, nodes: u64| Player {
            id: id.into(),
            executable: std::fs::canonicalize(executable)
                .unwrap()
                .to_str()
                .unwrap()
                .into(),
            build_provenance: format!(
                "{task}; executable hashes and campaign manifest record the builds"
            ),
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
            id: format!("{}-{id}", task.to_lowercase().replace('-', "")),
            protocol: format!(
                "Paired predecessor comparison. Native {game} controls outcomes. Opening provenance is recorded below. Mode: {}. Not a rating.",
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
                player(
                    "candidate",
                    &self.values[1],
                    64,
                    self.values[4].parse().unwrap(),
                ),
                player(
                    "baseline",
                    &self.values[2],
                    self.values[5].parse().unwrap(),
                    self.values[6].parse().unwrap(),
                ),
            ],
            openings: vec![Opening {
                id: id.clone(),
                group: id,
                root: fresh_root(record).unwrap(),
                split: Split::Test,
                provenance,
            }],
        };
        config.validate().unwrap();
        use std::io::Write;
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.values[9])
            .unwrap();
        file.write_all(serde_json::to_string_pretty(&config).unwrap().as_bytes())
            .unwrap();
    }
}
