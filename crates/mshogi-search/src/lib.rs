//! Native search components, independent of UI, console I/O and Python.
//!
//! Transparent native evaluation and bounded full-width Chess, Shogi and Xiangqi search.
//! Search must consult rules-owned game adjudication and claim policy before
//! evaluating a live leaf. Static scores never establish a game outcome.

pub mod chess;
pub mod chu;
pub mod shogi;
pub mod tenjiku;
pub mod tori;
pub mod xiangqi;

pub mod search;

pub mod coordination;
