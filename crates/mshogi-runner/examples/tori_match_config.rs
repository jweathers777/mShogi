//! Write an experiment configuration for a Tori Shogi game test of two engine
//! executables (MSH-165). Each opening is the standard position after four
//! random legal actions from a fixed seed, played with both colors.
//!
//! Usage: tori_match_config CANDIDATE BASELINE SEED CANDIDATE_NODES
//!        BASELINE_DEPTH BASELINE_NODES OUT.json
use mshogi_core::{
    native::{CanonicalRecord, RulesProfile},
    tori::{Game, GameCommand, GameRecord, GameRequest},
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
        out,
    ] = args.as_slice()
    else {
        panic!("usage: CANDIDATE BASELINE SEED CANDIDATE_NODES BASELINE_DEPTH BASELINE_NODES OUT")
    };
    let seed: u64 = seed.parse().unwrap();
    let player = |id: &str, executable: &str, depth: u16, nodes: u64| Player {
        id: id.into(),
        executable: std::fs::canonicalize(executable)
            .unwrap()
            .to_str()
            .unwrap()
            .into(),
        build_provenance: "MSH-165 game test; see docs/m5/evidence/msh165/verification.json".into(),
        limits: vec![ProfileLimits {
            profile: RulesProfile::ToriModernV1,
            work: WorkLimits {
                depth,
                nodes,
                time_ms: None,
            },
        }],
    };
    let mut state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let mut next = || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let mut openings = Vec::new();
    while openings.len() < 8 {
        let mut game = Game::standard();
        for _ in 0..4 {
            let legal = game.legal_actions();
            let action = legal[(next() % legal.len() as u64) as usize];
            game.submit(GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: GameCommand::Play { action },
            })
            .unwrap();
        }
        if game.conclusion().is_some() {
            continue;
        }
        let record = CanonicalRecord::tori(GameRecord::from_game(&game).unwrap());
        let id = format!("tori-random4-s{seed}-{}", openings.len());
        openings.push(Opening {
            id: id.clone(),
            group: id,
            root: fresh_root(record).unwrap(),
            split: Split::Test,
            provenance: "standard Tori position plus four seeded random legal actions".into(),
        });
    }
    let config = Config {
        schema: SCHEMA,
        id: format!("msh165-tori-shared-core-s{seed}"),
        protocol: "MSH-165 development game test: Tori shared-core search against the earlier full-width search, unequal fixed work limits, eight seeded openings with both colors. Not a rating.".into(),
        seed,
        rounds: 1,
        max_steps: 256,
        wall_budget_ms: 600_000,
        watchdog_ms: 30_000,
        cleanup_ms: 1_000,
        players: vec![
            player("candidate", candidate, 64, candidate_nodes.parse().unwrap()),
            player(
                "baseline",
                baseline,
                baseline_depth.parse().unwrap(),
                baseline_nodes.parse().unwrap(),
            ),
        ],
        openings,
    };
    config.validate().unwrap();
    std::fs::write(out, serde_json::to_string_pretty(&config).unwrap()).unwrap();
}
