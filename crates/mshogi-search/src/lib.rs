//! Native search components, independent of UI, console I/O and Python.
//!
//! All six games use the shared alpha-beta core with private search positions.
//! Adapters retain native authority for actual roots, actions, histories and
//! results. Private analysis can use documented approximations; static scores
//! never establish a real game outcome.

pub mod alphabeta;
pub mod chess_bitboard;
pub mod chess_engine;
pub mod chess_eval;
mod chess_kpk;
pub mod chu;
pub mod shogi;
mod square_set;
pub mod tenjiku;
pub mod tori;
pub mod xiangqi;

pub mod search;

pub mod coordination;
