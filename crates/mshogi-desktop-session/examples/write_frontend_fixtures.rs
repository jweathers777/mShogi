//! Regenerate native-shaped frontend test fixtures; never bundled with the app.
use mshogi_core::chess::*;
use mshogi_desktop_session::{DesktopCommand, DesktopSession};
fn main() {
    let directory = std::path::PathBuf::from(
        std::env::args_os()
            .nth(1)
            .expect("usage: write_frontend_fixtures <output-directory>"),
    );
    std::fs::create_dir_all(&directory).unwrap();
    for (name, fen) in [
        ("standard", STANDARD_START_FEN),
        ("promotion", "7k/P7/8/8/8/8/8/7K w - - 0 1"),
        ("claim", "7k/8/8/8/8/8/R7/K7 w - - 99 1"),
        ("current-claim", "7k/8/8/8/8/8/R7/K7 w - - 100 1"),
        (
            "checkmate",
            "rnb1kbnr/pppp1ppp/8/4p3/6Pq/5P2/PPPPP2P/RNBQKBNR w KQkq - 1 3",
        ),
    ] {
        let game = AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap();
        let session = DesktopSession::from_record(&game.record(), Color::White).unwrap();
        std::fs::write(
            directory.join(format!("{name}.json")),
            serde_json::to_string_pretty(&session.snapshot()).unwrap() + "\n",
        )
        .unwrap();
    }
    let mut unresolved = DesktopSession::new();
    unresolved
        .command(DesktopCommand::Resign { version: 1 }, 0)
        .unwrap();
    let game =
        AdjudicatedGame::from_fen("7k/8/8/8/8/8/R7/K7 w - - 99 1", ClaimPolicy::ExplicitOnly)
            .unwrap();
    let mut claimed = DesktopSession::from_record(&game.record(), Color::White).unwrap();
    let claim = claimed
        .snapshot()
        .claims
        .into_iter()
        .find(|claim| claim.intended.is_some())
        .unwrap();
    claimed
        .command(
            DesktopCommand::Claim {
                version: claimed.snapshot().version,
                claim,
            },
            0,
        )
        .unwrap();
    for (name, session) in [("unresolved", unresolved), ("claimed", claimed)] {
        std::fs::write(
            directory.join(format!("{name}.json")),
            serde_json::to_string_pretty(&session.snapshot()).unwrap() + "\n",
        )
        .unwrap();
    }
}
