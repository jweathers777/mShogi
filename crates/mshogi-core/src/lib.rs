//! Reusable mShogi rules library, independent of I/O and presentation.
//!
//! The chess module implements explicit position state and the profile's FEN
//! analysis boundary, checked ordinary actions/promotions and exact undo. Public
//! attack queries, special-move execution, move generation and full adjudication
//! remain subsequent work; this is not a full conformance certificate.

/// Target chess position contract specified in `docs/rules/chess-position-v1.md`.
///
/// The current implementation covers state/FEN and ordinary transitions, not the whole contract.
pub const CHESS_POSITION_PROFILE_ID: &str = "mshogi.chess.position.v1";

pub mod chess;
