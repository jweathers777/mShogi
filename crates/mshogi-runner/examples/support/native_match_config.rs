//! Common finite paired-match settings. Native game code supplies the root.
use mshogi_core::native::{CanonicalRecord, RulesProfile};
use mshogi_engine::WorkLimits;
use mshogi_runner::experiments::{
    Config, Opening, Player, ProfileLimits, SCHEMA, Split, fresh_root,
};

pub struct Args {
    values: Vec<String>,
    pub seed: u64,
    watchdog_ms: u64,
}
impl Args {
    #[allow(dead_code)]
    pub fn read() -> Self {
        let (values, watchdog_ms) = Self::values();
        Self::from_values(values, watchdog_ms)
    }
    #[allow(dead_code)]
    pub fn read_with_profile() -> (Self, String) {
        let (mut values, watchdog_ms) = Self::values();
        assert_eq!(
            values.len(),
            11,
            "expected native config arguments followed by profile"
        );
        let profile = values.pop().unwrap();
        (Self::from_values(values, watchdog_ms), profile)
    }
    fn values() -> (Vec<String>, u64) {
        let mut values: Vec<String> = std::env::args().collect();
        let watchdog_ms = if values.len() >= 3 && values[values.len() - 2] == "--watchdog-ms" {
            let value: u64 = values.pop().unwrap().parse().unwrap();
            values.pop();
            assert!((1..=60_000).contains(&value), "invalid engine watchdog");
            value
        } else {
            30_000
        };
        (values, watchdog_ms)
    }
    fn from_values(values: Vec<String>, watchdog_ms: u64) -> Self {
        assert_eq!(
            values.len(),
            10,
            "usage: CANDIDATE BASELINE SEED CANDIDATE_NODES BASELINE_DEPTH BASELINE_NODES MAX_STEPS WALL_MS OUT"
        );
        Self {
            seed: values[3].parse().unwrap(),
            values,
            watchdog_ms,
        }
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
                "Paired predecessor comparison. Native {game} controls outcomes. Seeded project-generated openings, no external corpus. Fixed work limits calibrated on equal roots; not a rating."
            ),
            seed: self.seed,
            rounds: 1,
            max_steps: self.values[7].parse().unwrap(),
            wall_budget_ms: self.values[8].parse().unwrap(),
            watchdog_ms: self.watchdog_ms,
            cleanup_ms: 1_000,
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
