//! Reusable mShogi rules library, independent of I/O and presentation.
//!
//! The chess module implements explicit position state and the profile's FEN
//! analysis boundary, checked actions including promotions, castling and en
//! passant, exact undo, geometric attack queries and pseudo-legal candidates.
//! It also provides the legal-action list, position-level classification and
//! perft/divide diagnostics. Game owns action/repetition history and versioned
//! records. The library also implements the selected game/match adjudication profiles.

/// Target chess position contract specified in `docs/rules/chess-position-v1.md`.
///
/// Position-level identity; game/match history and adjudication use the separate match profile.
pub const CHESS_POSITION_PROFILE_ID: &str = "mshogi.chess.position.v1";

/// Versioned chess match/history contract. Implemented coverage is in the ledger.
pub const CHESS_MATCH_PROFILE_ID: &str = "mshogi.chess.match.v1";

pub mod chess;

/// Native standard-shogi state and complete actions, with explicit coverage limits.
pub mod shogi;

/// Native Xiangqi position/action foundation; coverage is explicitly game-owned.
pub mod xiangqi;

/// Shared tagged native-record and command dispatch for all six games.
pub mod native;

/// Native Tori rules, history, inspection and complete-game consumer contracts.
pub mod tori;

/// Native modern JCSA Chu Shogi rules and complete-action admission.
pub mod chu;

/// Complete native Tenjiku state, actions, ordered effects and profile-qualified histories.
pub mod tenjiku;

pub mod perft;

mod record;
mod repetition;
