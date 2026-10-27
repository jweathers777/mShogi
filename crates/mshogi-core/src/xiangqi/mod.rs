//! Native intersection-board state for `mshogi.xiangqi.wxf2018.v1`.
//!
//! Position codecs describe diagrams and declared counters, not prior moves,
//! reachability or WXF history/adjudication. Local board legality is separate
//! from history-dependent permission and concluded games.

use std::{error::Error, fmt, str::FromStr};

mod action;
mod assessment;
mod codec;
mod conduct;
mod game;
pub use assessment::{NATURAL_WIN_GOAL, NoWinAssessment, assess_natural_win};
mod record;
pub use conduct::FaultKind;
pub use game::{
    ClaimPolicy, CommandResult, Conclusion, DrawReason, EventKind, Game, GameCommand, GameContext,
    GameError, GameEvent, GameRequest, GameUndo, HistorySnapshot, MAX_GAME_COMMANDS,
    MAX_GAME_EVENTS, NaturalCountEligibility, PendingAdjudication, RuntimeToken, Submission,
    Warning, WinReason,
};
pub use record::{GAME_RECORD_SCHEMA_VERSION, GameRecord, MAX_GAME_RECORD_BYTES, RecordError};
mod legal;
mod perft;
pub use action::{
    Action, ActionEffects, ActionError, ActionParseError, ResolvedAction, Undo, UndoError,
};
pub use codec::{MAX_FEN_BYTES, MAX_POSITION_BYTES, PositionRoot, RootOrigin};
pub use legal::{LocalPositionError, LocalStatus, NoActionReason};
pub use perft::{MAX_PERFT_DEPTH, PerftError};
#[cfg(test)]
mod tests;

/// Selected immutable native rules-profile identity; not a delivery claim.
pub const PROFILE_ID: &str = "mshogi.xiangqi.wxf2018.v1";
/// Governing bilingual publication and selected software interpretation edition.
pub const PROFILE_EDITION: &str = "WXF-2018+XIANGQI-P1-2026-10-27";
/// Native root-position document schema; this is not a canonical full game.
pub const POSITION_SCHEMA_ID: &str = "mshogi.xiangqi.position.v1";
/// Number of files of intersections, not a count of square cells.
pub const FILES: u8 = 9;
/// Number of ranks of intersections.
pub const RANKS: u8 = 10;
/// The river separates these two zero-based ranks.
pub const RIVER_BETWEEN_RANKS: (u8, u8) = (4, 5);
/// Standard initial position in the scoped native FEN dialect (Red = `w`).
pub const INITIAL_FEN: &str =
    "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1";

/// Xiangqi ownership, independent of Chess/Shogi side conventions.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    /// Red, moving first and forward toward increasing diagnostic ranks.
    Red,
    /// Black, forward toward decreasing diagnostic ranks.
    Black,
}
impl Side {
    /// The opposing native owner.
    pub const fn opponent(self) -> Self {
        match self {
            Self::Red => Self::Black,
            Self::Black => Self::Red,
        }
    }
    /// Forward rank displacement in the Red-bottom a0–i9 coordinate dialect.
    pub const fn forward(self) -> i8 {
        match self {
            Self::Red => 1,
            Self::Black => -1,
        }
    }
    /// Whether an intersection is inside this owner's nine-point palace.
    pub const fn in_palace(self, point: Intersection) -> bool {
        point.file() >= 3
            && point.file() <= 5
            && match self {
                Self::Red => point.rank() <= 2,
                Self::Black => point.rank() >= 7,
            }
    }
    /// Whether a point is on the opponent's side of the river for this owner.
    pub const fn has_crossed_river(self, point: Intersection) -> bool {
        match self {
            Self::Red => point.rank() >= 5,
            Self::Black => point.rank() <= 4,
        }
    }
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::Red => 0,
            Self::Black => 1,
        }
    }
}

/// One bounded intersection; files a–i and ranks0–9 from Red's baseline.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Intersection(u8);
impl Intersection {
    /// Construct from zero-based file/rank; no off-board sentinel is admitted.
    pub const fn new(file: u8, rank: u8) -> Option<Self> {
        if file < FILES && rank < RANKS {
            Some(Self(rank * FILES + file))
        } else {
            None
        }
    }
    /// Zero-based file a=0 through i=8.
    pub const fn file(self) -> u8 {
        self.0 % FILES
    }
    /// Zero-based rank0 through9, increasing away from Red's baseline.
    pub const fn rank(self) -> u8 {
        self.0 / FILES
    }
    /// Native rank-major storage index, not a shared board encoding.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
    /// Every native intersection in rank-major order.
    pub fn all() -> impl Iterator<Item = Self> {
        (0..FILES * RANKS).map(Self)
    }
}
impl fmt::Display for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", char::from(b'a' + self.file()), self.rank())
    }
}
impl FromStr for Intersection {
    type Err = ActionParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let bytes = text.as_bytes();
        if bytes.len() != 2 || !(b'a'..=b'i').contains(&bytes[0]) || !bytes[1].is_ascii_digit() {
            return Err(ActionParseError);
        }
        Self::new(bytes[0] - b'a', bytes[1] - b'0').ok_or(ActionParseError)
    }
}

/// Native piece identities; none promotes or becomes a hand resource.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PieceKind {
    /// Palace-confined royal.
    General,
    /// Diagonal palace stepper.
    Advisor,
    /// Two-diagonal mover with eye/river restrictions.
    Elephant,
    /// Leg-blockable horse.
    Horse,
    /// Orthogonal sliding rook (chariot).
    Rook,
    /// Quiet slider with exactly-one-screen captures.
    Cannon,
    /// Forward soldier, gaining sideways movement across the river.
    Soldier,
}
impl PieceKind {
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::General => 0,
            Self::Advisor => 1,
            Self::Elephant => 2,
            Self::Horse => 3,
            Self::Rook => 4,
            Self::Cannon => 5,
            Self::Soldier => 6,
        }
    }
    pub(crate) const fn cap(self) -> u8 {
        match self {
            Self::General => 1,
            Self::Soldier => 5,
            _ => 2,
        }
    }
    pub(crate) const fn letter(self) -> char {
        match self {
            Self::General => 'K',
            Self::Advisor => 'A',
            Self::Elephant => 'B',
            Self::Horse => 'N',
            Self::Rook => 'R',
            Self::Cannon => 'C',
            Self::Soldier => 'P',
        }
    }
    pub(crate) fn from_letter(letter: char) -> Option<Self> {
        Some(match letter.to_ascii_uppercase() {
            'K' => Self::General,
            'A' => Self::Advisor,
            'B' => Self::Elephant,
            'N' => Self::Horse,
            'R' => Self::Rook,
            'C' => Self::Cannon,
            'P' => Self::Soldier,
            _ => return None,
        })
    }
}

/// An owned native piece; no redundant promotion/drop fields.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Piece {
    /// Native owner.
    pub side: Side,
    /// Native unchanging kind.
    pub kind: PieceKind,
}
impl Piece {
    /// Construct a native piece.
    pub const fn new(side: Side, kind: PieceKind) -> Self {
        Self { side, kind }
    }
}

/// Declared notation counters, never proof of omitted moves/checks or outcomes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Counters {
    no_capture_plies: Option<u32>,
    fullmove_number: Option<u32>,
}
impl Counters {
    /// Construct counters; a supplied fullmove number must be positive.
    pub const fn new(
        no_capture_plies: Option<u32>,
        fullmove_number: Option<u32>,
    ) -> Result<Self, PositionError> {
        if matches!(fullmove_number, Some(0)) {
            return Err(PositionError::Counter);
        }
        Ok(Self {
            no_capture_plies,
            fullmove_number,
        })
    }
    /// Neither counter was supplied by the analysis diagram.
    pub const fn unknown() -> Self {
        Self {
            no_capture_plies: None,
            fullmove_number: None,
        }
    }
    /// Supplied capture-free ply count; no Article8 check adjustment is inferred.
    pub const fn no_capture_plies(self) -> Option<u32> {
        self.no_capture_plies
    }
    /// Supplied fullmove number, incremented after Black's move if known.
    pub const fn fullmove_number(self) -> Option<u32> {
        self.fullmove_number
    }
}

/// A structurally admitted native diagram and turn, without a game history.
///
/// Exactly one General per side and inventory bounds are required. Off-home
/// diagnostic placements are allowed; construction is not live-root admission
/// or proof of reachability. Local rules queries have a separate documented domain.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Position {
    pub(crate) board: [Option<Piece>; 90],
    pub(crate) side: Side,
    pub(crate) counters: Counters,
}
/// Syntax, structure or representation rejection, not a game conclusion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PositionError {
    /// Bad byte/field syntax or unsupported notation.
    Syntax,
    /// Invalid rank placement/token/width.
    BoardSyntax,
    /// More than one input piece at the same intersection.
    DuplicateIntersection,
    /// The diagram lacks exactly one General of each owner.
    GeneralCount,
    /// A side exceeds its original per-kind inventory.
    ExcessMaterial,
    /// Invalid/overflowing declared counter.
    Counter,
    /// FEN output cannot invent a missing declared counter.
    UnknownCounters,
    /// Wrong profile/edition/root-position schema/history identity.
    Identity,
    /// Bad/duplicate/unknown/missing JSON field or unsupported value.
    Json,
    /// Provenance tag contradicts the actual root data.
    OriginMismatch,
}
impl fmt::Display for PositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "xiangqi position rejected: {self:?}")
    }
}
impl Error for PositionError {}
impl Position {
    /// Build an analysis diagram, rejecting duplicate points and excess material.
    /// This does not certify live-root validity or historical reachability.
    pub fn from_diagram(
        pieces: &[(Intersection, Piece)],
        side: Side,
        counters: Counters,
    ) -> Result<Self, PositionError> {
        let mut board = [None; 90];
        for &(point, piece) in pieces {
            if board[point.index()].is_some() {
                return Err(PositionError::DuplicateIntersection);
            }
            board[point.index()] = Some(piece);
        }
        Self::from_board(board, side, counters)
    }
    pub(crate) fn from_board(
        board: [Option<Piece>; 90],
        side: Side,
        counters: Counters,
    ) -> Result<Self, PositionError> {
        let mut counts = [[0_u8; 7]; 2];
        for piece in board.iter().flatten() {
            let count = &mut counts[piece.side.index()][piece.kind.index()];
            *count += 1;
            if *count > piece.kind.cap() {
                return Err(PositionError::ExcessMaterial);
            }
        }
        if counts[0][0] != 1 || counts[1][0] != 1 {
            return Err(PositionError::GeneralCount);
        }
        Ok(Self {
            board,
            side,
            counters,
        })
    }
    /// The standard initial board, Red to move, counters0/1.
    pub fn initial() -> Self {
        Self::from_fen(INITIAL_FEN).expect("constant initial Xiangqi FEN")
    }
    /// Current native owner to move.
    pub const fn side_to_move(&self) -> Side {
        self.side
    }
    /// Owned piece at an intersection.
    pub fn piece(&self, point: Intersection) -> Option<Piece> {
        self.board[point.index()]
    }
    /// Supplied/derived local counters; omitted history remains unknown.
    pub const fn counters(&self) -> Counters {
        self.counters
    }
    /// All currently occupied intersections, in native rank-major order.
    pub fn pieces(&self) -> impl Iterator<Item = (Intersection, Piece)> + '_ {
        Intersection::all().filter_map(|point| self.piece(point).map(|piece| (point, piece)))
    }
}
