//! Reproducible native full-width work measurement; not playing-strength evidence.
use mshogi_core::chu::*;
use mshogi_search::chu::{Limits, search};
use std::time::Instant;
fn main() {
    let depth = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
    let nodes = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(512);
    let tactical = Game::from_root(
        Position::from_diagram(
            &[
                (
                    "12l".parse().unwrap(),
                    Piece::new(Side::Sente, Base::King, false).unwrap(),
                ),
                (
                    "1a".parse().unwrap(),
                    Piece::new(Side::Gote, Base::King, false).unwrap(),
                ),
                (
                    "6f".parse().unwrap(),
                    Piece::new(Side::Sente, Base::Lion, false).unwrap(),
                ),
                (
                    "6e".parse().unwrap(),
                    Piece::new(Side::Gote, Base::Pawn, false).unwrap(),
                ),
                (
                    "5e".parse().unwrap(),
                    Piece::new(Side::Gote, Base::Rook, false).unwrap(),
                ),
            ],
            Side::Sente,
            RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap();
    for (name, game) in [("standard", Game::standard()), ("lion_tactical", tactical)] {
        let actions = game.legal_actions().len();
        let at = Instant::now();
        let r = search(&game, Limits { depth, nodes });
        match r {
            Ok(r) => println!(
                "{name} depth={depth} limit={nodes} actions={actions} elapsed_ms={} complete={} visits={} status={:?} score={:?} pv={:?}",
                at.elapsed().as_millis(),
                r.completed_depth,
                r.nodes,
                r.status,
                r.score,
                r.principal_variation
            ),
            Err(e) => println!("{name} elapsed_ms={} error={e}", at.elapsed().as_millis()),
        }
    }
}
