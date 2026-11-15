//! One paired Xiangqi opening for the native experiment controller.
//! Arguments match shogi_match_config. Seed zero uses the standard root;
//! ordinary seeds use four to twelve seeded native legal actions. Seeds
//! 1,000,000 through 1,000,003 are reduced tactical calibration roots only.
use mshogi_core::{
    native::{CanonicalRecord, RulesProfile},
    xiangqi::{ClaimPolicy, Game, GameCommand, GameRecord, GameRequest, PositionRoot},
};
#[path = "support/native_match_config.rs"]
mod native_match_config;

fn main() {
    let args = native_match_config::Args::read();
    let seed = args.seed;
    const TACTICAL: [&str; 4] = [
        "5k3/9/2r6/9/2P6/9/2C6/9/9/3K5 w - - 0 1",
        "5k3/9/9/9/3r5/9/2N6/9/9/3K5 w - - 0 1",
        "5k3/9/4R4/9/9/9/9/9/9/3K5 w - - 0 1",
        "3k5/9/9/9/9/9/9/4r4/9/5K3 b - - 0 1",
    ];
    let tactical = seed
        .checked_sub(1_000_000)
        .and_then(|n| TACTICAL.get(n as usize));
    // This is the existing unattended native controller policy. It decides
    // valid natural-count claims, conduct warnings and real conclusions.
    let mut game = tactical.map_or_else(
        || Game::standard(ClaimPolicy::AutoValidClaims).unwrap(),
        |fen| {
            Game::from_root(
                PositionRoot::from_fen(fen).unwrap(),
                ClaimPolicy::AutoValidClaims,
            )
            .unwrap()
        },
    );
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
        let legal = game.position().legal_actions().unwrap();
        let action = legal[state as usize % legal.len()];
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: GameCommand::Play { action },
        })
        .unwrap();
    }
    assert!(game.conclusion().is_none() && game.pending_adjudication().is_none());
    let id = if tactical.is_some() {
        format!("xiangqi-tactical-s{seed}")
    } else {
        format!("xiangqi-random{plies}-s{seed}")
    };
    args.write(
        "MSH-181", "Xiangqi", RulesProfile::XiangqiWxf2018V1,
        CanonicalRecord::xiangqi(GameRecord::from_game(&game).unwrap()), id,
        if tactical.is_some() {
            "reduced native regression diagram; calibration only; AutoValidClaims".into()
        } else {
            format!("standard Xiangqi plus {plies} seeded native legal actions; xorshift64; AutoValidClaims")
        },
    );
}
