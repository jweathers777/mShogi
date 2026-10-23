//! Reusable mShogi rules library boundary.
//!
//! MSH-3 establishes the build boundary and shared target-profile identity only.
//! Position parsing and rule operations begin in MSH-4 and later tasks. No game
//! implementation or conformance is claimed by this crate yet.

/// Target chess position contract specified in `docs/rules/chess-position-v1.md`.
///
/// This identifies the intended contract, not an implemented capability.
pub const CHESS_POSITION_PROFILE_ID: &str = "mshogi.chess.position.v1";
