//! Reusable mShogi rules library, independent of I/O and presentation.
//!
//! The chess module implements explicit position state and the profile's FEN
//! analysis boundary. Actions, public attack queries, move generation and full
//! adjudication remain subsequent work; parsing is not a conformance certificate.

/// Target chess position contract specified in `docs/rules/chess-position-v1.md`.
///
/// The current implementation covers state and FEN only, not the whole contract.
pub const CHESS_POSITION_PROFILE_ID: &str = "mshogi.chess.position.v1";

pub mod chess;
