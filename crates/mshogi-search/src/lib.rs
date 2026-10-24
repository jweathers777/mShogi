//! Native search components, independent of UI, console I/O and Python.
//!
//! Currently contains only a transparent chess static evaluation baseline.
//! Search must consult rules-owned game adjudication and claim policy before
//! evaluating a live leaf. Static scores never establish a game outcome.

pub mod chess;
