//! Actual native/controller Tori DTOs for contract tests. Never bundled.
#[path = "support/tori_presentation.rs"]
mod fixtures;
fn main() {
    if std::env::args().nth(1).as_deref() == Some("internal-search") {
        use std::io::BufRead;
        let mut line = String::new();
        std::io::stdin().lock().read_line(&mut line).unwrap();
        mshogi_engine::serve_once(std::io::Cursor::new(line), std::io::stdout().lock()).unwrap();
        return;
    }
    let dir = std::path::PathBuf::from(std::env::args_os().nth(1).expect("output directory"));
    std::fs::create_dir_all(&dir).unwrap();
    fixtures::write_all(&dir);
}
