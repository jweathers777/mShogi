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
use mshogi_engine::WorkLimits;
use mshogi_runner::experiments::{
    Config, Opening, Player, ProfileLimits, SCHEMA, Split, fresh_root,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let [
        _,
        candidate,
        baseline,
        seed,
        candidate_nodes,
        baseline_depth,
        baseline_nodes,
        max_steps,
        wall_ms,
        out,
    ] = args.as_slice()
    else {
        panic!(
            "usage: CANDIDATE BASELINE SEED CANDIDATE_NODES BASELINE_DEPTH BASELINE_NODES MAX_STEPS WALL_MS OUT"
        )
    };
    let seed: u64 = seed.parse().unwrap();
    let player = |id: &str, executable: &str, depth: u16, nodes: u64| Player {
        id: id.into(),
        executable: std::fs::canonicalize(executable)
            .unwrap()
            .to_str()
            .unwrap()
            .into(),
        build_provenance: "MSH-180; executable hashes and campaign manifest record the builds"
            .into(),
        limits: vec![ProfileLimits {
            profile: RulesProfile::ShogiCsa36V1,
            work: WorkLimits {
                depth,
                nodes,
                time_ms: None,
            },
        }],
    };
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
    let config = Config {
        schema: SCHEMA,
        id: format!("msh180-{id}"),
        protocol: "Paired predecessor comparison. Native Shogi controls outcomes. Seeded project-generated openings, no external corpus. Fixed work limits calibrated on equal roots; not a rating.".into(),
        seed,
        rounds: 1,
        max_steps: max_steps.parse().unwrap(),
        wall_budget_ms: wall_ms.parse().unwrap(),
        watchdog_ms: 30_000,
        cleanup_ms: 1_000,
        players: vec![
            player("candidate", candidate, 64, candidate_nodes.parse().unwrap()),
            player("baseline", baseline, baseline_depth.parse().unwrap(), baseline_nodes.parse().unwrap()),
        ],
        openings: vec![Opening {
            id: id.clone(),
            group: id,
            root: fresh_root(CanonicalRecord::shogi(GameRecord::from_game(&game))).unwrap(),
            split: Split::Test,
            provenance: if tactical.is_some() {
                "reduced native regression diagram; calibration only".into()
            } else {
                format!("standard Shogi plus {plies} seeded native legal actions; xorshift64")
            },
        }],
    };
    config.validate().unwrap();
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(out)
        .unwrap();
    file.write_all(serde_json::to_string_pretty(&config).unwrap().as_bytes())
        .unwrap();
}
