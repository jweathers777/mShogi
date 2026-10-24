//! Explicit chess position state for `mshogi.chess.position.v1`.
//!
//! FEN validates a local analysis position, not historical reachability. This
//! Position contains no history. Game owns action/repetition history; match
//! adjudication and clocks remain subsequent work. No search cache key is defined.
//! Public construction, replacement and checked transitions preserve
//! the profile's local invariants. Actions/undo remain intentionally chess-specific.
//! `apply` executes every chess action; `apply_ordinary` is its ordinary-only subset.
//! Attack queries are geometric and candidates are pseudo-legal; neither is
//! legal-move membership. `legal_actions` and `classify` provide that.

use std::fmt;

mod action;
mod attack;
mod fen;
mod game;
mod generate;
mod legal;
mod notation;
mod perft;
mod record;
pub use action::{Action, ActionError, Promotion, Undo, UndoError};
pub use fen::{FenError, FenErrorCategory, FenErrorReason};
pub use game::{ClaimPolicy, Game, HistoryOrigin, RepetitionIdentity, STANDARD_START_FEN};
pub use legal::Classification;
pub use notation::ActionParseError;
pub use perft::{DivideEntry, PerftError};
pub use record::*;

/// One of the two colors in orthodox chess.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Color {
    /// White moves toward increasing ranks.
    White,
    /// Black moves toward decreasing ranks.
    Black,
}

impl Color {
    /// Return the other color.
    pub const fn opponent(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }

    const fn index(self) -> usize {
        match self {
            Self::White => 0,
            Self::Black => 1,
        }
    }
}

/// An orthodox piece's current kind; promotion history is not encoded in FEN.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PieceKind {
    /// Pawn.
    Pawn,
    /// Knight.
    Knight,
    /// Bishop.
    Bishop,
    /// Rook.
    Rook,
    /// Queen.
    Queen,
    /// King.
    King,
}

/// Kind and color of one piece occupying a square.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Piece {
    /// The piece's color.
    pub color: Color,
    /// The piece's current kind.
    pub kind: PieceKind,
}

/// A bounded square, with a1 at file/rank (0, 0) and h8 at (7, 7).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Square(u8);

impl Square {
    /// Construct a square from zero-based file and rank, or reject either >= 8.
    pub const fn new(file: u8, rank: u8) -> Option<Self> {
        if file < 8 && rank < 8 {
            Some(Self(rank * 8 + file))
        } else {
            None
        }
    }

    /// Zero-based file (a = 0, h = 7).
    pub const fn file(self) -> u8 {
        self.0 % 8
    }

    /// Zero-based rank (rank 1 = 0, rank 8 = 7).
    pub const fn rank(self) -> u8 {
        self.0 / 8
    }

    const fn index(self) -> usize {
        self.0 as usize
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", char::from(b'a' + self.file()), self.rank() + 1)
    }
}

/// Historical castling permissions; these do not assert immediate legality.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct CastlingRights {
    kingside: [bool; 2],
    queenside: [bool; 2],
}

impl CastlingRights {
    /// Whether the color retains its kingside permission (not a legal-move query).
    pub const fn kingside(self, color: Color) -> bool {
        self.kingside[color.index()]
    }

    /// Whether the color retains its queenside permission (not a legal-move query).
    pub const fn queenside(self, color: Color) -> bool {
        self.queenside[color.index()]
    }
}

impl fmt::Display for CastlingRights {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut any = false;
        for (present, letter) in [
            (self.kingside(Color::White), 'K'),
            (self.queenside(Color::White), 'Q'),
            (self.kingside(Color::Black), 'k'),
            (self.queenside(Color::Black), 'q'),
        ] {
            if present {
                write!(f, "{letter}")?;
                any = true;
            }
        }
        if !any {
            write!(f, "-")?;
        }
        Ok(())
    }
}

/// A locally validated analysis position under `mshogi.chess.position.v1`.
///
/// Private state is a direct a1-to-h8 array plus the five remaining FEN fields.
/// There are no incremental caches yet. `Eq` compares the entire saved-state
/// tuple, including raw en passant target and both counters. It is **not**
/// repetition equivalence, match-history equality, or a search-cache identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position {
    board: [Option<Piece>; 64],
    side_to_move: Color,
    castling_rights: CastlingRights,
    en_passant_target: Option<Square>,
    halfmove_clock: u32,
    fullmove_number: u32,
}

impl Position {
    /// Inspect a piece without exposing mutable board access.
    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        self.board[square.index()]
    }

    /// Side whose next action is to be chosen; that side may currently be in check.
    pub const fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    /// Historical castling rights; blocked or attacked paths do not remove them.
    pub const fn castling_rights(&self) -> CastlingRights {
        self.castling_rights
    }

    /// Raw last-double-push target, retained even without a legal capturing pawn.
    pub const fn en_passant_target(&self) -> Option<Square> {
        self.en_passant_target
    }

    /// Plies since a pawn move/capture; this alone does not adjudicate a draw.
    pub const fn halfmove_clock(&self) -> u32 {
        self.halfmove_clock
    }

    /// Positive fullmove number, incremented after a Black action.
    pub const fn fullmove_number(&self) -> u32 {
        self.fullmove_number
    }
}
