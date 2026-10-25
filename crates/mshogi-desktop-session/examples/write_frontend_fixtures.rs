//! Regenerate native-shaped frontend test fixtures; never bundled with the app.
use mshogi_core::chess::*;
use mshogi_desktop_session::DesktopSession;
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
}
