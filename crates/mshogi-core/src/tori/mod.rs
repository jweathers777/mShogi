//! Native structural and complete position-local rules for `mshogi.tori.modern.v1`.
//!
//! Public positions are structurally validated diagrams, NOT admitted live games.
//! LocalPosition owns complete local rules/admission/inspection, distinct from
//! structural Position. Game owns history/records/commands; LocalPosition remains independent of game permission. Structural mutation and
//! geometric candidates are crate-private and cannot admit a public game action.
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, str::FromStr, sync::Arc};
mod game;
pub mod notation;
mod record;
pub use game::*;
pub use record::{GAME_RECORD_SCHEMA_VERSION, GameRecord, MAX_GAME_RECORD_BYTES, RecordError};

#[cfg(test)]
mod conformance;
mod legal;
#[cfg(test)]
mod tests;
mod transition;
pub use legal::{
    ActionError, AdmissionError, Effects, InspectedChoice, InspectionOrigin, LocalPosition,
    LocalRedo, LocalStatus, LocalUndo, ReversalError,
};
mod perft;
pub use perft::{
    DivideEntry, DivideReport, MAX_PERFT_DEPTH, MAX_PERFT_VISITS, PerftError, PerftReport,
};

/// Selected immutable rules identity.
pub const PROFILE_ID: &str = "mshogi.tori.modern.v1";
/// Selected immutable source/software edition.
pub const PROFILE_EDITION: &str = "MSH81-MODERN-2026-11-01";

/// Native ownership and orientation, independent of other games.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    /// First player; forward is toward rank a.
    Sente,
    /// Second player; forward is toward rank g.
    Gote,
}
impl Side {
    /// Other native player.
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
    pub(crate) const fn in_zone(self, rank: u8) -> bool {
        match self {
            Self::Sente => rank <= 2,
            Self::Gote => rank >= 6,
        }
    }
    /// Project mover-right/forward onto Sente's right/forward axes.
    pub const fn relative_to_sente(self, x: i8, forward: i8) -> (i16, i16) {
        match self {
            Self::Sente => (x as i16, forward as i16),
            Self::Gote => (-(x as i16), -(forward as i16)),
        }
    }
}
/// Bounded7×7 square, array order a→g and7→1 within each rank.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Square(u8);
impl Square {
    /// Numeric file/rank, both1..=7; rank1 is a.
    pub const fn new(file: u8, rank: u8) -> Option<Self> {
        if file >= 1 && file <= 7 && rank >= 1 && rank <= 7 {
            Some(Self((rank - 1) * 7 + 7 - file))
        } else {
            None
        }
    }
    /// Native numeric file1..=7, decreasing toward Sente's right.
    pub const fn file(self) -> u8 {
        7 - self.0 % 7
    }
    /// Numeric rank1..=7 (a..g).
    pub const fn rank(self) -> u8 {
        self.0 / 7 + 1
    }
    /// Bounded board-array index.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
    /// All49 squares, canonical board order.
    pub fn all() -> impl Iterator<Item = Self> {
        (0..49).map(Self)
    }
    /// Rotate180°; native handed kind is unaffected.
    pub const fn rotated(self) -> Self {
        Self(48 - self.0)
    }
    pub(crate) fn offset(self, side: Side, x: i8, forward: i8) -> Option<Self> {
        let (x, f) = side.relative_to_sente(x, forward);
        let file = i16::from(self.file()) - x;
        let rank = i16::from(self.rank()) - f;
        Self::new(u8::try_from(file).ok()?, u8::try_from(rank).ok()?)
    }
}
/// Invalid native square spelling (not a display-action parser).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SquareError;
impl fmt::Display for SquareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid Tori7x7 square")
    }
}
impl Error for SquareError {}
impl FromStr for Square {
    type Err = SquareError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s.as_bytes();
        if b.len() != 2 || !(b'1'..=b'7').contains(&b[0]) || !(b'a'..=b'g').contains(&b[1]) {
            return Err(SquareError);
        }
        Self::new(b[0] - b'0', b[1] - b'a' + 1).ok_or(SquareError)
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file(), char::from(b'a' + self.rank() - 1))
    }
}

/// All native board faces; invalid base/promotion combinations are unrepresentable.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Face {
    /// Royal Phoenix.
    Phoenix,
    /// Unpromoted Falcon.
    Falcon,
    /// Promoted Falcon.
    Eagle,
    /// Crane.
    Crane,
    /// Pheasant.
    Pheasant,
    /// Left-handed Quail; identity survives capture.
    LeftQuail,
    /// Right-handed Quail; identity survives capture.
    RightQuail,
    /// Unpromoted Swallow.
    Swallow,
    /// Promoted Swallow.
    Goose,
}
impl Face {
    /// All nine native faces.
    pub const ALL: [Self; 9] = [
        Self::Phoenix,
        Self::Falcon,
        Self::Eagle,
        Self::Crane,
        Self::Pheasant,
        Self::LeftQuail,
        Self::RightQuail,
        Self::Swallow,
        Self::Goose,
    ];
    /// Base identities in canonical inventory order, royal first.
    pub const BASES: [Self; 7] = [
        Self::Phoenix,
        Self::Falcon,
        Self::Crane,
        Self::Pheasant,
        Self::LeftQuail,
        Self::RightQuail,
        Self::Swallow,
    ];
    /// Hand identities; royal and promoted faces never appear in a hand.
    pub const HANDS: [Self; 6] = [
        Self::Falcon,
        Self::Crane,
        Self::Pheasant,
        Self::LeftQuail,
        Self::RightQuail,
        Self::Swallow,
    ];
    /// Base identity, including handedness.
    pub const fn base(self) -> Self {
        match self {
            Self::Eagle => Self::Falcon,
            Self::Goose => Self::Swallow,
            _ => self,
        }
    }
    /// Whether this face is promoted.
    pub const fn is_promoted(self) -> bool {
        matches!(self, Self::Eagle | Self::Goose)
    }
    /// Stable native short identity; presentation may choose other labels.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Phoenix => "Ph",
            Self::Falcon => "Fa",
            Self::Eagle => "Ea",
            Self::Crane => "Cr",
            Self::Pheasant => "Pt",
            Self::LeftQuail => "LQ",
            Self::RightQuail => "RQ",
            Self::Swallow => "Sw",
            Self::Goose => "Go",
        }
    }
    pub(crate) const fn base_index(self) -> usize {
        match self.base() {
            Self::Phoenix => 0,
            Self::Falcon => 1,
            Self::Crane => 2,
            Self::Pheasant => 3,
            Self::LeftQuail => 4,
            Self::RightQuail => 5,
            Self::Swallow => 6,
            _ => unreachable!(),
        }
    }
    pub(crate) const fn cap(self) -> u8 {
        match self.base() {
            Self::Phoenix | Self::Falcon | Self::LeftQuail | Self::RightQuail => 2,
            Self::Crane | Self::Pheasant => 4,
            Self::Swallow => 16,
            _ => unreachable!(),
        }
    }
    pub(crate) const fn hand_index(self) -> Option<usize> {
        if matches!(self, Self::Phoenix | Self::Eagle | Self::Goose) {
            None
        } else {
            Some(self.base_index() - 1)
        }
    }
    pub(crate) const fn moved_face(self, side: Side, from: Square, to: Square) -> Self {
        if side.in_zone(from.rank()) || side.in_zone(to.rank()) {
            match self {
                Self::Falcon => Self::Eagle,
                Self::Swallow => Self::Goose,
                _ => self,
            }
        } else {
            self
        }
    }
}
/// Owned native face. Handedness belongs to the face, never to the owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Piece {
    /// Native owner.
    pub side: Side,
    /// Exact current face.
    pub face: Face,
}
/// One side's captured unpromoted, nonroyal resources.
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub struct Hand([u8; 6]);
impl Hand {
    /// Empty resource inventory.
    pub const fn empty() -> Self {
        Self([0; 6])
    }
    /// Count of an exact hand identity; royal/promoted queries yield zero.
    pub fn count(self, face: Face) -> u8 {
        face.hand_index().map_or(0, |i| self.0[i])
    }
    /// Checked unique hand entries. Combined inventory is checked by Position.
    pub fn from_counts(entries: &[(Face, u8)]) -> Result<Self, RootError> {
        let mut h = Self::empty();
        let mut seen = [false; 6];
        for &(face, n) in entries {
            let i = face.hand_index().ok_or(RootError::InvalidHandFace)?;
            if seen[i] {
                return Err(RootError::DuplicateHandFace);
            }
            seen[i] = true;
            if n > face.cap() {
                return Err(RootError::ExcessInventory);
            }
            h.0[i] = n;
        }
        Ok(h)
    }
}
/// Declared origin category, not evidence of reachability.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootKind {
    /// Exact standard arrangement/counter/turn.
    Standard,
    /// Explicit custom diagram, possibly reduced material.
    Diagram,
    /// Imported structural diagram; no previous history inferred.
    Imported,
}
/// Immutable root provenance; no constructor asserts historical reachability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RootMetadata {
    kind: RootKind,
    source: Option<String>,
    declared_completed_plies: u64,
}
impl RootMetadata {
    /// Root category.
    pub const fn kind(&self) -> RootKind {
        self.kind
    }
    /// Optional import source, bounded to1024 UTF-8 bytes.
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }
    /// Declared counter, not inferred known history or accepted board actions.
    pub const fn declared_completed_plies(&self) -> u64 {
        self.declared_completed_plies
    }
    /// Only the standard root starts with known complete prior history (empty).
    pub const fn earlier_history_unknown(&self) -> bool {
        !matches!(self.kind, RootKind::Standard)
    }
    /// Structural diagrams never establish historical reachability.
    pub const fn historical_reachability_proven(&self) -> bool {
        matches!(self.kind, RootKind::Standard)
    }
    /// Explicit diagram metadata, with an immutable declared counter.
    pub const fn diagram(declared_completed_plies: u64) -> Self {
        Self {
            kind: RootKind::Diagram,
            source: None,
            declared_completed_plies,
        }
    }
    /// Explicit imported diagram provenance; an empty/oversized source rejects.
    pub fn imported(source: String, declared_completed_plies: u64) -> Result<Self, RootError> {
        if source.is_empty() || source.len() > 1024 {
            return Err(RootError::InvalidProvenance);
        }
        Ok(Self {
            kind: RootKind::Imported,
            source: Some(source),
            declared_completed_plies,
        })
    }
}
/// Structural root errors; no game outcome is implied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RootError {
    /// Duplicate occupied square in root input.
    DuplicateSquare,
    /// Exactly one Phoenix per side is required even in a structural diagram.
    MissingOrDuplicateRoyal,
    /// Combined base board/hands inventory exceeds the initial inventory.
    ExcessInventory,
    /// A royal or promoted face is not a hand resource.
    InvalidHandFace,
    /// Repeated hand entry, including explicit zero counts.
    DuplicateHandFace,
    /// Missing or oversized import source.
    InvalidProvenance,
    /// Standard metadata cannot relabel a supplied diagram.
    StandardRootNotDiagram,
}
impl fmt::Display for RootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tori structural root rejected: {self:?}")
    }
}
impl Error for RootError {}
/// Complete logical structural state, excluding deliberately non-rewound token identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StructuralState {
    board: [Option<Piece>; 49],
    hands: [Hand; 2],
    side: Side,
    root: RootMetadata,
    accepted_plies: u64,
}
/// A structural position, NOT a live-game or full-legality admission boundary.
///
/// Correctly typed actions cannot access structural mutation from an external consumer:
/// ```compile_fail
/// use mshogi_core::tori::{Position, Action, Face};
/// let mut position = Position::standard();
/// let action = Action::Move {
///     from: "1e".parse().unwrap(), to: "1d".parse().unwrap(),
///     result_face: Face::Swallow,
/// };
/// position.apply_structural(action); // crate-private; not a public legality API
/// ```
#[derive(Debug)]
pub struct Position {
    state: StructuralState,
    owner: Arc<()>,
    node: u64,
    next_node: u64,
    branch: u64,
    revision: u64,
}
impl Clone for Position {
    /// A logical copy has independent reversal ownership; tokens never cross copies.
    fn clone(&self) -> Self {
        Self::install(self.state.clone())
    }
}
impl Position {
    fn install(state: StructuralState) -> Self {
        Self {
            state,
            owner: Arc::new(()),
            node: 0,
            next_node: 1,
            branch: 0,
            revision: 0,
        }
    }
    /// Full authoritative logical snapshot, including every square, hand and metadata.
    pub fn state(&self) -> &StructuralState {
        &self.state
    }
    /// Selected immutable rules identity (not a record/transport schema).
    pub const fn profile_id(&self) -> &'static str {
        PROFILE_ID
    }
    /// Selected immutable source/software edition.
    pub const fn profile_edition(&self) -> &'static str {
        PROFILE_EDITION
    }
    /// Exact owned face at a bounded square.
    pub fn piece(&self, square: Square) -> Option<Piece> {
        self.state.board[square.index()]
    }
    /// Resources belonging to the named side.
    pub fn hand(&self, side: Side) -> Hand {
        self.state.hands[side.index()]
    }
    /// Structural side to move; no live-game permission is implied.
    pub const fn side_to_move(&self) -> Side {
        self.state.side
    }
    /// Immutable root metadata.
    pub fn root(&self) -> &RootMetadata {
        &self.state.root
    }
    /// Accepted structural board actions since this installed root.
    pub const fn accepted_plies(&self) -> u64 {
        self.state.accepted_plies
    }
    /// Declared root counter plus actual accepted board actions; checked on admission.
    pub fn declared_total_plies(&self) -> u64 {
        self.state.root.declared_completed_plies + self.state.accepted_plies
    }
    /// Construct a structurally valid custom/imported root. No royal safety,
    /// dead-rank, Swallow-file, drop-mate or historical reachability check is claimed.
    pub fn from_diagram(
        entries: &[(Square, Piece)],
        hands: [Hand; 2],
        side: Side,
        root: RootMetadata,
    ) -> Result<Self, RootError> {
        if root.kind == RootKind::Standard {
            return Err(RootError::StandardRootNotDiagram);
        }
        let mut board = [None; 49];
        let mut inventory = [0u16; 7];
        let mut royals = [0; 2];
        for &(sq, piece) in entries {
            if board[sq.index()].replace(piece).is_some() {
                return Err(RootError::DuplicateSquare);
            }
            inventory[piece.face.base_index()] += 1;
            if piece.face == Face::Phoenix {
                royals[piece.side.index()] += 1;
            }
        }
        for hand in hands {
            for f in Face::HANDS {
                inventory[f.base_index()] += u16::from(hand.count(f));
            }
        }
        if royals != [1, 1] {
            return Err(RootError::MissingOrDuplicateRoyal);
        }
        if Face::BASES
            .into_iter()
            .any(|f| inventory[f.base_index()] > u16::from(f.cap()))
        {
            return Err(RootError::ExcessInventory);
        }
        Ok(Self::install(StructuralState {
            board,
            hands,
            side,
            root,
            accepted_plies: 0,
        }))
    }
    /// Standard32-piece selected setup with complete empty prior history.
    pub fn standard() -> Self {
        let mut entries = Vec::with_capacity(32);
        let home = [
            Face::LeftQuail,
            Face::Pheasant,
            Face::Crane,
            Face::Phoenix,
            Face::Crane,
            Face::Pheasant,
            Face::RightQuail,
        ];
        for (i, face) in home.into_iter().enumerate() {
            entries.push((
                Square::new(7 - i as u8, 7).unwrap(),
                Piece {
                    side: Side::Sente,
                    face,
                },
            ));
        }
        entries.push((
            Square::new(4, 6).unwrap(),
            Piece {
                side: Side::Sente,
                face: Face::Falcon,
            },
        ));
        for file in 1..=7 {
            entries.push((
                Square::new(file, 5).unwrap(),
                Piece {
                    side: Side::Sente,
                    face: Face::Swallow,
                },
            ));
        }
        entries.push((
            Square::new(3, 4).unwrap(),
            Piece {
                side: Side::Sente,
                face: Face::Swallow,
            },
        ));
        let other: Vec<_> = entries
            .iter()
            .map(|&(sq, p)| {
                (
                    sq.rotated(),
                    Piece {
                        side: p.side.opponent(),
                        ..p
                    },
                )
            })
            .collect();
        entries.extend(other);
        let mut position = Self::from_diagram(
            &entries,
            [Hand::empty(); 2],
            Side::Sente,
            RootMetadata::diagram(0),
        )
        .expect("fixed standard inventory");
        position.state.root.kind = RootKind::Standard;
        position
    }
}
/// A complete structural action. Captures/effects are always derived from native state.
/// This type alone does not grant legal admission or live-game permission.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Action {
    /// Exact source, destination and mandatory resulting face; no caller-supplied victim.
    Move {
        /// Existing owned source.
        from: Square,
        /// Complete endpoint, never an intermediate route.
        to: Square,
        /// Exact required resulting face.
        result_face: Face,
    },
    /// Drop one unpromoted resource; no capture, route or supplied owner.
    Drop {
        /// Exact base nonroyal identity (invalid faces reject).
        face: Face,
        /// Empty destination.
        to: Square,
    },
}

#[cfg(test)]
mod local_conformance;
#[cfg(test)]
mod local_tests;

impl Serialize for Square {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.to_string())
    }
}
impl<'de> Deserialize<'de> for Square {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        String::deserialize(d)?
            .parse()
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod game_conformance;
#[cfg(test)]
mod game_tests;
