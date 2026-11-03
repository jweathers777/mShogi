//! Authoritative native Chu Shogi rules for the selected modern JCSA profile.
//!
//! Structural diagrams, position-local admission and history-owned game commands
//! are separate boundaries. A diagram does not establish historical reachability.
mod legal;
/// Complete native human action formatting and local-choice parsing.
pub mod notation;
mod position;
pub use legal::*;
pub use position::*;

/// Immutable selected rules-profile identity.
pub const PROFILE_ID: &str = "mshogi.chu.jcsa2019.v1";
/// Immutable source and software interpretation edition.
pub const PROFILE_EDITION: &str = "JCSA-2019-09-14+2004-06-06+MSH82-2026-11-02";

#[cfg(test)]
mod conformance;
#[cfg(test)]
mod game_conformance;
#[cfg(test)]
mod local_tests;

mod game;
mod record;
pub use game::*;
pub use record::{GAME_RECORD_SCHEMA_VERSION, GameRecord, MAX_GAME_RECORD_BYTES, RecordError};

#[cfg(test)]
mod game_tests;
