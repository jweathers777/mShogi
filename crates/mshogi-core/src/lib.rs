//! Reusable mShogi rules library, independent of I/O and presentation.
//!
//! The chess module implements explicit position state and the profile's FEN
//! analysis boundary, checked actions including promotions, castling and en
//! passant, exact undo, geometric attack queries and pseudo-legal candidates.
//! The public legal-move list, classification and full adjudication remain
//! subsequent work; this is not a full conformance certificate.

/// Target chess position contract specified in `docs/rules/chess-position-v1.md`.
///
/// The current implementation covers state/FEN, transitions, attacks and candidates, not the whole contract.
pub const CHESS_POSITION_PROFILE_ID: &str = "mshogi.chess.position.v1";

pub mod chess;
