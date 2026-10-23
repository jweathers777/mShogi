//! Reusable mShogi rules library, independent of I/O and presentation.
//!
//! The chess module implements explicit position state and the profile's FEN
//! analysis boundary, checked actions including promotions, castling and en
//! passant, exact undo, geometric attack queries and pseudo-legal candidates.
//! It also provides the legal-action list, position-level classification and
//! perft/divide diagnostics. Full game/match adjudication remains subsequent
//! work; this is not a full conformance certificate.

/// Target chess position contract specified in `docs/rules/chess-position-v1.md`.
///
/// The current implementation covers state/FEN, transitions, attacks, legal actions, classification and perft, not adjudication.
pub const CHESS_POSITION_PROFILE_ID: &str = "mshogi.chess.position.v1";

pub mod chess;
