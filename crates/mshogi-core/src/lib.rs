//! Reusable mShogi rules library, independent of I/O and presentation.
//!
//! The chess module implements explicit position state and the profile's FEN
//! analysis boundary, checked actions including promotions, castling and en
//! passant, exact undo, geometric attack queries and pseudo-legal candidates.
//! It also provides the legal-action list, position-level classification and
//! perft/divide diagnostics. Game owns action/repetition history and versioned
//! records. Full game/match adjudication remains subsequent
//! work; this is not a full conformance certificate.

/// Target chess position contract specified in `docs/rules/chess-position-v1.md`.
///
/// The current implementation covers state/FEN, transitions, attacks, legal actions, classification and perft, not adjudication.
pub const CHESS_POSITION_PROFILE_ID: &str = "mshogi.chess.position.v1";

/// Versioned chess match/history contract. Implemented coverage is in the ledger.
pub const CHESS_MATCH_PROFILE_ID: &str = "mshogi.chess.match.v1";

pub mod chess;

/// Native standard-shogi state and complete actions, with explicit coverage limits.
pub mod shogi;

/// Small tagged native-record and command dispatch for actual chess/shogi consumers.
pub mod native;
