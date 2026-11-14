//! Native search components, independent of UI, console I/O and Python.
//!
//! Transparent native evaluation and bounded full-width Chess, Shogi and Xiangqi search.
//! Search must consult rules-owned game adjudication and claim policy before
//! evaluating a live leaf. Static scores never establish a game outcome.

pub mod alphabeta;
pub mod chess;
pub mod chess_bitboard;
pub mod chess_candidate;
pub mod chess_conventional;
pub mod chess_conventional_eval;
pub mod chess_engine;
pub mod chess_eval;
mod chess_kpk;
pub mod chu;
pub mod shogi;
pub mod tenjiku;
pub mod tori;
pub mod xiangqi;

pub mod search;

pub mod coordination;
