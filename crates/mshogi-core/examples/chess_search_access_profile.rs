//! Bounded query-path diagnostic, not an engine match or strength measurement.
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy, STANDARD_START_FEN, SearchCursor};
use std::{hint::black_box, time::Instant};
fn public_walk(game: &mut AdjudicatedGame, depth: u8) -> u64 {
    if black_box(game.outcome()).is_some() {
        return 1;
    }
    black_box(game.available_claims());
    let actions = game.position().legal_actions();
    black_box(game.position().in_check());
    if depth == 0 {
        return 1;
    }
    let mut nodes = 1;
    for action in actions {
        game.apply(action).unwrap();
        nodes += public_walk(game, depth - 1);
        game.undo().unwrap();
    }
    nodes
}
fn cached_walk(cursor: &mut SearchCursor, depth: u8) -> u64 {
    let node = cursor.node();
    if black_box(node.outcome).is_some() {
        return 1;
    }
    black_box(&node.claims);
    let actions = node.actions.clone();
    black_box(node.classification);
    if depth == 0 {
        return 1;
    }
    let mut nodes = 1;
    for action in actions {
        cursor.push(action).unwrap();
        nodes += cached_walk(cursor, depth - 1);
        cursor.pop();
    }
    nodes
}
fn main() {
    for (name, fen) in [
        ("standard-depth2", STANDARD_START_FEN),
        ("tactical-depth2", "7k/8/8/8/8/8/q7/R6K w - - 0 1"),
    ] {
        let original = AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap();
        let mut game = original.clone();
        let began = Instant::now();
        let direct_nodes = public_walk(&mut game, 2);
        let direct = began.elapsed();
        assert_eq!(game, original);
        let mut cursor = SearchCursor::new(&original);
        let began = Instant::now();
        let cached_nodes = cached_walk(&mut cursor, 2);
        let cached = began.elapsed();
        assert_eq!(cursor.game(), Some(&original));
        assert_eq!(direct_nodes, cached_nodes);
        println!(
            "{name}: native_nodes={direct_nodes} public_query_us={} cached_cursor_us={}",
            direct.as_micros(),
            cached.as_micros()
        );
    }
}
