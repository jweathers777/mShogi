//! Native state for `mshogi.shogi.csa36.v1`.
//!
//! SFEN admits locally valid analysis diagrams, including sparse material. It
//! does not establish reachability, repetition, prior checks or match outcomes.
//! The public legal layer includes complete promotion/drop choices, king safety
//! and pawn-drop mate. Structural transition helpers remain crate-private.
//! No chess state or action contract is reused.

use std::{error::Error, fmt, str::FromStr};

mod action;
mod game;
mod legal;
mod perft;
mod record;
mod sfen;
pub use legal::RepetitionIdentity;
pub use perft::{DivideEntry, PerftError};
pub use record::{GameRecord, MAX_RECORD_BYTES, MAX_RECORD_EVENTS, RecordError};
#[cfg(test)]
mod tests;
pub use action::{Action, ActionError, ActionParseError, Undo, UndoError};
pub use game::{
    Conclusion, DeclarationEligibility, DrawReason, Game, GameCommand, GameContext, GameError,
    GameEvent, GamePrefixError, GameRequest, GameRoot, GameUndo, GameUndoError,
    MutualCheckDiagnostic, RootCategory, RootError, WinReason,
};

/// Approved standard-shogi rules profile identity.
pub const PROFILE_ID: &str = "mshogi.shogi.csa36.v1";
/// Independently specified even-game initial SFEN.
pub const INITIAL_SFEN: &str = "lnsgkgsnl/1r5b1/ppppppppp/9/9/9/PPPPPPPPP/1B5R1/LNSGKGSNL b - 1";

/// Ownership and direction in standard shogi.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Side {
    /// First player, advancing toward rank a (numeric rank 1).
    Sente,
    /// Second player, advancing toward rank i (numeric rank 9).
    Gote,
}
impl Side {
    /// Other player.
    pub const fn opponent(self) -> Self {
        match self {
            Self::Sente => Self::Gote,
            Self::Gote => Self::Sente,
        }
    }
    pub(crate) const fn index(self) -> usize {
        match self {
            Self::Sente => 0,
            Self::Gote => 1,
        }
    }
    pub(crate) const fn forward(self) -> i8 {
        match self {
            Self::Sente => -1,
            Self::Gote => 1,
        }
    }
    pub(crate) const fn in_camp(self, rank: u8) -> bool {
        match self {
            Self::Sente => rank <= 3,
            Self::Gote => rank >= 7,
        }
    }
}

/// A bounded square with one-based numeric files and ranks, e.g. (7, 7) = 7g.
/// Board-array order is rank a to i, each rank file 9 down to 1 (SFEN order).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Square(u8);
impl Square {
    /// Construct from numeric file/rank in 1..=9.
    pub const fn new(file: u8, rank: u8) -> Option<Self> {
        if file >= 1 && file <= 9 && rank >= 1 && rank <= 9 {
            Some(Self((rank - 1) * 9 + (9 - file)))
        } else {
            None
        }
    }
    /// Numeric file, 1..=9.
    pub const fn file(self) -> u8 {
        9 - self.0 % 9
    }
    /// Numeric rank, 1..=9 (a..=i in USI).
    pub const fn rank(self) -> u8 {
        self.0 / 9 + 1
    }
    /// Array index in SFEN order.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
    /// Iterate every square in SFEN order.
    pub fn all() -> impl Iterator<Item = Self> {
        (0..81).map(Self)
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file(), char::from(b'a' + self.rank() - 1))
    }
}
impl FromStr for Square {
    type Err = ActionParseError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let b = value.as_bytes();
        if b.len() != 2 || !(b'1'..=b'9').contains(&b[0]) || !(b'a'..=b'i').contains(&b[1]) {
            return Err(ActionParseError);
        }
        Self::new(b[0] - b'0', b[1] - b'a' + 1).ok_or(ActionParseError)
    }
}

/// Unpromoted base identity; a captured promoted piece returns to this kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum PieceKind {
    /// King; never captured or held in hand.
    King,
    /// Rook; promotes to dragon.
    Rook,
    /// Bishop; promotes to horse.
    Bishop,
    /// Gold general; cannot promote.
    Gold,
    /// Silver general; promotes to gold movement.
    Silver,
    /// Knight; promotes to gold movement.
    Knight,
    /// Lance; promotes to gold movement.
    Lance,
    /// Pawn; promotes to tokin (gold movement).
    Pawn,
}
impl PieceKind {
    /// Hand kinds in canonical SFEN output order.
    pub const HAND_KINDS: [Self; 7] = [
        Self::Rook,
        Self::Bishop,
        Self::Gold,
        Self::Silver,
        Self::Knight,
        Self::Lance,
        Self::Pawn,
    ];
    /// Whether this base kind can promote.
    pub const fn can_promote(self) -> bool {
        !matches!(self, Self::King | Self::Gold)
    }
    pub(crate) const fn hand_index(self) -> Option<usize> {
        match self {
            Self::Rook => Some(0),
            Self::Bishop => Some(1),
            Self::Gold => Some(2),
            Self::Silver => Some(3),
            Self::Knight => Some(4),
            Self::Lance => Some(5),
            Self::Pawn => Some(6),
            Self::King => None,
        }
    }
    pub(crate) const fn inventory_cap(self) -> u8 {
        match self {
            Self::King | Self::Rook | Self::Bishop => 2,
            Self::Pawn => 18,
            _ => 4,
        }
    }
    pub(crate) const fn letter(self) -> char {
        match self {
            Self::King => 'K',
            Self::Rook => 'R',
            Self::Bishop => 'B',
            Self::Gold => 'G',
            Self::Silver => 'S',
            Self::Knight => 'N',
            Self::Lance => 'L',
            Self::Pawn => 'P',
        }
    }
    pub(crate) fn from_letter(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'K' => Some(Self::King),
            'R' => Some(Self::Rook),
            'B' => Some(Self::Bishop),
            'G' => Some(Self::Gold),
            'S' => Some(Self::Silver),
            'N' => Some(Self::Knight),
            'L' => Some(Self::Lance),
            'P' => Some(Self::Pawn),
            _ => None,
        }
    }
    pub(crate) const fn dead_unpromoted(self, side: Side, rank: u8) -> bool {
        let last = match side {
            Side::Sente => rank == 1,
            Side::Gote => rank == 9,
        };
        let last_two = match side {
            Side::Sente => rank <= 2,
            Side::Gote => rank >= 8,
        };
        match self {
            Self::Pawn | Self::Lance => last,
            Self::Knight => last_two,
            _ => false,
        }
    }
}

/// An owned board piece with independent base identity and promoted state.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct Piece {
    /// Current owner.
    pub side: Side,
    /// Base kind, unchanged by promotion.
    pub kind: PieceKind,
    /// Promotion state. Constructors reject promotion of king/gold.
    pub promoted: bool,
}
impl Piece {
    /// Create a piece, rejecting impossible promotion states.
    pub const fn new(side: Side, kind: PieceKind, promoted: bool) -> Option<Self> {
        if promoted && !kind.can_promote() {
            None
        } else {
            Some(Self {
                side,
                kind,
                promoted,
            })
        }
    }
}

/// One owner's unpromoted captured resources. Kings have count zero.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct Hand([u8; 7]);
impl Hand {
    /// Empty hand.
    pub const fn empty() -> Self {
        Self([0; 7])
    }
    /// Count of a base kind; kings are never hand resources.
    pub const fn count(self, kind: PieceKind) -> u8 {
        match kind.hand_index() {
            Some(i) => self.0[i],
            None => 0,
        }
    }
    /// Build one hand with per-kind inventory caps and no duplicate entries.
    /// Combined board and both-hand inventory is validated by the position.
    pub fn from_counts(counts: &[(PieceKind, u8)]) -> Result<Self, PositionError> {
        let mut hand = Self::empty();
        let mut seen = [false; 7];
        for &(kind, count) in counts {
            let i = kind.hand_index().ok_or(PositionError::InvalidHand)?;
            if seen[i] || count > kind.inventory_cap() {
                return Err(PositionError::InvalidHand);
            }
            seen[i] = true;
            hand.0[i] = count;
        }
        Ok(hand)
    }
}

/// How the analysis root's next-ply counter was obtained.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CounterSource {
    /// Counter was explicitly supplied by the importer.
    Supplied,
    /// A diagram adapter explicitly defaulted an absent counter to one.
    Defaulted,
}
/// Origin of the known history segment; position state does not store history.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum HistoryOrigin {
    /// Standard initial state, with no missing earlier play.
    Initial,
    /// Imported diagram; all history before this root remains unknown.
    Diagram {
        /// Declared root next-ply, preserved as subsequent actions advance.
        next_ply: u32,
        /// Supplied versus adapter default.
        counter_source: CounterSource,
    },
}

/// Rejected SFEN syntax or locally inconsistent diagram; not a reachability test.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PositionError {
    /// SFEN must have exactly four bounded ASCII fields.
    Syntax,
    /// Invalid rank width, piece character or promotion marker.
    BoardSyntax,
    /// Side field must be b or w.
    SideSyntax,
    /// Invalid hand, duplicate kind/owner, forbidden king or excessive count.
    InvalidHand,
    /// A positive decimal next-ply within u32 is required.
    Counter,
    /// Exactly one king of each owner is required.
    Kings,
    /// Board plus hands exceed the even-game base inventory.
    Inventory,
    /// King or gold was marked promoted.
    Promotion,
    /// An unpromoted pawn/lance/knight has no future movement.
    DeadPiece,
    /// One owner has multiple unpromoted pawns on a file.
    Nifu,
}
impl fmt::Display for PositionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid shogi position: {self:?}")
    }
}
impl Error for PositionError {}

/// Complete native local state. No prior repetition or match result is inferred.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position {
    board: [Option<Piece>; 81],
    hands: [Hand; 2],
    side: Side,
    next_ply: u32,
    origin: HistoryOrigin,
}
impl Position {
    /// Standard forty-piece even-game state.
    pub fn initial() -> Self {
        let mut position = Self::from_sfen(INITIAL_SFEN).expect("constant initial SFEN is valid");
        position.origin = HistoryOrigin::Initial;
        position
    }
    /// Build a local diagram. Sparse material is permitted; preceding play is unknown.
    /// `None` explicitly defaults the counter to one; supplied zero is rejected.
    pub fn from_diagram(
        board: [Option<Piece>; 81],
        hands: [Hand; 2],
        side: Side,
        next_ply: Option<u32>,
    ) -> Result<Self, PositionError> {
        let source = if next_ply.is_some() {
            CounterSource::Supplied
        } else {
            CounterSource::Defaulted
        };
        let next_ply = next_ply.unwrap_or(1);
        let position = Self {
            board,
            hands,
            side,
            next_ply,
            origin: HistoryOrigin::Diagram {
                next_ply,
                counter_source: source,
            },
        };
        position.validate()?;
        Ok(position)
    }
    /// Board in SFEN order (rank a..i, file 9..1).
    pub const fn board(&self) -> &[Option<Piece>; 81] {
        &self.board
    }
    /// Occupant of one square.
    pub const fn piece(&self, square: Square) -> Option<Piece> {
        self.board[square.index()]
    }
    /// Captured resources owned by one player.
    pub const fn hand(&self, side: Side) -> Hand {
        self.hands[side.index()]
    }
    /// Player whose turn it is.
    pub const fn side_to_move(&self) -> Side {
        self.side
    }
    /// Declared next-ply number; completed board actions are this value minus one.
    pub const fn next_ply(&self) -> u32 {
        self.next_ply
    }
    /// Completed-action count implied by the declaration, not verified history.
    pub const fn completed_actions(&self) -> u32 {
        self.next_ply - 1
    }
    /// Root provenance, unchanged by local board actions.
    pub const fn history_origin(&self) -> HistoryOrigin {
        self.origin
    }

    fn validate(&self) -> Result<(), PositionError> {
        if self.next_ply == 0 {
            return Err(PositionError::Counter);
        }
        let mut kings = [0_u8; 2];
        let mut totals = [0_u16; 7];
        let mut pawns = [[false; 9]; 2];
        for square in Square::all() {
            let Some(piece) = self.piece(square) else {
                continue;
            };
            if piece.promoted && !piece.kind.can_promote() {
                return Err(PositionError::Promotion);
            }
            if piece.kind == PieceKind::King {
                kings[piece.side.index()] += 1;
            }
            if let Some(i) = piece.kind.hand_index() {
                totals[i] += 1;
            }
            if !piece.promoted && piece.kind.dead_unpromoted(piece.side, square.rank()) {
                return Err(PositionError::DeadPiece);
            }
            if piece.kind == PieceKind::Pawn && !piece.promoted {
                let slot = &mut pawns[piece.side.index()][usize::from(square.file() - 1)];
                if *slot {
                    return Err(PositionError::Nifu);
                }
                *slot = true;
            }
        }
        if kings != [1, 1] {
            return Err(PositionError::Kings);
        }
        for kind in PieceKind::HAND_KINDS {
            let i = kind.hand_index().expect("hand kind");
            totals[i] +=
                u16::from(self.hands[0].count(kind)) + u16::from(self.hands[1].count(kind));
            if totals[i] > u16::from(kind.inventory_cap()) {
                return Err(PositionError::Inventory);
            }
        }
        Ok(())
    }
}
