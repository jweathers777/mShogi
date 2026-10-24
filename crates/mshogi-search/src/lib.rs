//! Native search components, independent of UI, console I/O and Python.
//!
//! Transparent material evaluation and deterministic full-width chess search.
//! Search must consult rules-owned game adjudication and claim policy before
//! evaluating a live leaf. Static scores never establish a game outcome.

pub mod chess;

pub mod search;
