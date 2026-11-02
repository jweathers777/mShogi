//! Native structural state; no public structural mutation bypasses local admission.
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{error::Error, fmt, str::FromStr};

/// Native player and orientation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    /// First player; forward is toward rank1.
    Sente,
    /// Second player; forward is toward rank12.
    Gote,
}
impl Side {
    /// The other player.
    pub const fn opponent(self) -> Self {
        match self {
            Self::Sente => Self::Gote,
            Self::Gote => Self::Sente,
        }
    }
    /// Native two-player array index.
    pub const fn index(self) -> usize {
        match self {
            Self::Sente => 0,
            Self::Gote => 1,
        }
    }
    /// Enemy four-rank camp.
    pub const fn in_zone(self, rank: u8) -> bool {
        match self {
            Self::Sente => rank <= 4,
            Self::Gote => rank >= 9,
        }
    }
    /// Farthest rank.
    pub const fn last_rank(self, rank: u8) -> bool {
        match self {
            Self::Sente => rank == 1,
            Self::Gote => rank == 12,
        }
    }
}
/// A bounded native12×12 square, canonical order rank1..12/file12..1.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Square(u8);
impl Square {
    /// Construct from both numeric coordinates1..=12.
    pub const fn new(file: u8, rank: u8) -> Option<Self> {
        if file >= 1 && file <= 12 && rank >= 1 && rank <= 12 {
            Some(Self((rank - 1) * 12 + 12 - file))
        } else {
            None
        }
    }
    /// Native file.
    pub const fn file(self) -> u8 {
        12 - self.0 % 12
    }
    /// Native rank.
    pub const fn rank(self) -> u8 {
        self.0 / 12 + 1
    }
    /// Canonical board array index.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
    /// Every board square.
    pub fn all() -> impl Iterator<Item = Self> {
        (0..144).map(Self)
    }
    /// Rotate180degrees.
    pub const fn rotated(self) -> Self {
        Self(143 - self.0)
    }
    pub(crate) fn offset(self, side: Side, df: i8, dr: i8) -> Option<Self> {
        let sign = if side == Side::Sente { 1i16 } else { -1 };
        let f = i16::from(self.file()) + i16::from(df) * sign;
        let r = i16::from(self.rank()) + i16::from(dr) * sign;
        Self::new(u8::try_from(f).ok()?, u8::try_from(r).ok()?)
    }
    /// Chebyshev distance, used only for native Lion geometry.
    pub fn distance(self, other: Self) -> u8 {
        self.file()
            .abs_diff(other.file())
            .max(self.rank().abs_diff(other.rank()))
    }
}
impl Serialize for Square {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        [self.file(), self.rank()].serialize(s)
    }
}
impl<'de> Deserialize<'de> for Square {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let [f, r] = <[u8; 2]>::deserialize(d)?;
        Self::new(f, r)
            .ok_or_else(|| serde::de::Error::custom("Chu square coordinates must be1..12"))
    }
}
/// Invalid native square spelling.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SquareError;
impl fmt::Display for SquareError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid Chu square (file1..12/rank a..l)")
    }
}
impl Error for SquareError {}
impl FromStr for Square {
    type Err = SquareError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (file, rank) = s
            .split_at_checked(s.len().saturating_sub(1))
            .ok_or(SquareError)?;
        let b = rank.as_bytes();
        if b.len() != 1 || !(b'a'..=b'l').contains(&b[0]) {
            return Err(SquareError);
        }
        let file: u8 = file.parse().map_err(|_| SquareError)?;
        if file.to_string() != s[..s.len() - 1] {
            return Err(SquareError);
        }
        Self::new(file, b[0] - b'a' + 1).ok_or(SquareError)
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file(), char::from(b'a' + self.rank() - 1))
    }
}

/// Original identity; promotion never replaces this identity or permits another flip.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Base {
    /// Pawn.
    #[serde(rename = "P")]
    Pawn,
    /// Go-between.
    #[serde(rename = "GB")]
    GoBetween,
    /// Lance.
    #[serde(rename = "L")]
    Lance,
    /// Reverse chariot.
    #[serde(rename = "RC")]
    ReverseChariot,
    /// Copper general.
    #[serde(rename = "Cu")]
    Copper,
    /// Silver general.
    #[serde(rename = "Si")]
    Silver,
    /// Gold general.
    #[serde(rename = "G")]
    Gold,
    /// Ferocious leopard.
    #[serde(rename = "FL")]
    Leopard,
    /// Blind tiger.
    #[serde(rename = "BT")]
    Tiger,
    /// Drunk elephant.
    #[serde(rename = "DE")]
    Elephant,
    /// King.
    #[serde(rename = "K")]
    King,
    /// Kirin.
    #[serde(rename = "Ki")]
    Kirin,
    /// Phoenix.
    #[serde(rename = "Ph")]
    Phoenix,
    /// Side mover.
    #[serde(rename = "SM")]
    SideMover,
    /// Vertical mover.
    #[serde(rename = "VM")]
    VerticalMover,
    /// Rook.
    #[serde(rename = "R")]
    Rook,
    /// Bishop.
    #[serde(rename = "B")]
    Bishop,
    /// Dragon horse.
    #[serde(rename = "DH")]
    DragonHorse,
    /// Dragon king.
    #[serde(rename = "DK")]
    DragonKing,
    /// Lion.
    #[serde(rename = "Ln")]
    Lion,
    /// Free king.
    #[serde(rename = "FK")]
    FreeKing,
}
impl Base {
    /// Every original kind.
    pub const ALL: [Self; 21] = [
        Self::Pawn,
        Self::GoBetween,
        Self::Lance,
        Self::ReverseChariot,
        Self::Copper,
        Self::Silver,
        Self::Gold,
        Self::Leopard,
        Self::Tiger,
        Self::Elephant,
        Self::King,
        Self::Kirin,
        Self::Phoenix,
        Self::SideMover,
        Self::VerticalMover,
        Self::Rook,
        Self::Bishop,
        Self::DragonHorse,
        Self::DragonKing,
        Self::Lion,
        Self::FreeKing,
    ];
    /// Stable original code.
    pub const fn code(self) -> &'static str {
        match self {
            Self::Pawn => "P",
            Self::GoBetween => "GB",
            Self::Lance => "L",
            Self::ReverseChariot => "RC",
            Self::Copper => "Cu",
            Self::Silver => "Si",
            Self::Gold => "G",
            Self::Leopard => "FL",
            Self::Tiger => "BT",
            Self::Elephant => "DE",
            Self::King => "K",
            Self::Kirin => "Ki",
            Self::Phoenix => "Ph",
            Self::SideMover => "SM",
            Self::VerticalMover => "VM",
            Self::Rook => "R",
            Self::Bishop => "B",
            Self::DragonHorse => "DH",
            Self::DragonKing => "DK",
            Self::Lion => "Ln",
            Self::FreeKing => "FK",
        }
    }
    /// Parse only original identities.
    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|b| b.code() == code)
    }
    /// Original inventory per player; captures never transfer ownership.
    pub const fn cap(self) -> u8 {
        match self {
            Self::Pawn => 12,
            Self::King
            | Self::Elephant
            | Self::Kirin
            | Self::Phoenix
            | Self::Lion
            | Self::FreeKing => 1,
            _ => 2,
        }
    }
    /// Whether this original identity may flip once.
    pub const fn promotable(self) -> bool {
        !matches!(self, Self::King | Self::Lion | Self::FreeKing)
    }
}
/// Movement face, separate from original identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Face {
    /// Pawn.
    Pawn,
    /// Go-between.
    GoBetween,
    /// Lance.
    Lance,
    /// Reverse chariot.
    ReverseChariot,
    /// Copper general.
    Copper,
    /// Silver general.
    Silver,
    /// Gold general.
    Gold,
    /// Ferocious leopard.
    Leopard,
    /// Blind tiger.
    Tiger,
    /// Drunk elephant.
    Elephant,
    /// King.
    King,
    /// Kirin.
    Kirin,
    /// Phoenix.
    Phoenix,
    /// Side mover.
    SideMover,
    /// Vertical mover.
    VerticalMover,
    /// Rook.
    Rook,
    /// Bishop.
    Bishop,
    /// Dragon horse.
    DragonHorse,
    /// Dragon king.
    DragonKing,
    /// Lion.
    Lion,
    /// Free king.
    FreeKing,
    /// White horse.
    WhiteHorse,
    /// Whale.
    Whale,
    /// Flying stag.
    FlyingStag,
    /// Prince.
    Prince,
    /// Flying ox.
    FlyingOx,
    /// Free boar.
    FreeBoar,
    /// Soaring eagle.
    SoaringEagle,
    /// Horned falcon.
    HornedFalcon,
}
impl Face {
    /// All movement faces.
    pub const ALL: [Self; 29] = [
        Self::Pawn,
        Self::GoBetween,
        Self::Lance,
        Self::ReverseChariot,
        Self::Copper,
        Self::Silver,
        Self::Gold,
        Self::Leopard,
        Self::Tiger,
        Self::Elephant,
        Self::King,
        Self::Kirin,
        Self::Phoenix,
        Self::SideMover,
        Self::VerticalMover,
        Self::Rook,
        Self::Bishop,
        Self::DragonHorse,
        Self::DragonKing,
        Self::Lion,
        Self::FreeKing,
        Self::WhiteHorse,
        Self::Whale,
        Self::FlyingStag,
        Self::Prince,
        Self::FlyingOx,
        Self::FreeBoar,
        Self::SoaringEagle,
        Self::HornedFalcon,
    ];
    /// Source movement-face code, not a canonical original identity.
    pub const fn code(self) -> &'static str {
        match self {
            Self::WhiteHorse => "WH",
            Self::Whale => "W",
            Self::FlyingStag => "FS",
            Self::Prince => "Pr",
            Self::FlyingOx => "FO",
            Self::FreeBoar => "FB",
            Self::SoaringEagle => "SE",
            Self::HornedFalcon => "HF",
            Self::Pawn => "P",
            Self::GoBetween => "GB",
            Self::Lance => "L",
            Self::ReverseChariot => "RC",
            Self::Copper => "Cu",
            Self::Silver => "Si",
            Self::Gold => "G",
            Self::Leopard => "FL",
            Self::Tiger => "BT",
            Self::Elephant => "DE",
            Self::King => "K",
            Self::Kirin => "Ki",
            Self::Phoenix => "Ph",
            Self::SideMover => "SM",
            Self::VerticalMover => "VM",
            Self::Rook => "R",
            Self::Bishop => "B",
            Self::DragonHorse => "DH",
            Self::DragonKing => "DK",
            Self::Lion => "Ln",
            Self::FreeKing => "FK",
        }
    }
    /// Parse a movement-face code.
    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|b| b.code() == code)
    }
}
/// Owned original identity and one-flip state. Roots validate combinations.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Piece {
    /// Native owner.
    pub side: Side,
    /// Original identity.
    pub base: Base,
    /// Whether the original piece has flipped once.
    pub promoted: bool,
}
impl Piece {
    /// Checked original identity and one-flip state.
    pub const fn new(side: Side, base: Base, promoted: bool) -> Option<Self> {
        if promoted && !base.promotable() {
            None
        } else {
            Some(Self {
                side,
                base,
                promoted,
            })
        }
    }
    /// Parse original code with optional trailing+; never infer a base from a face.
    pub fn from_code(side: Side, code: &str) -> Option<Self> {
        let (base, promoted) = code.strip_suffix('+').map_or((code, false), |b| (b, true));
        Self::new(side, Base::from_code(base)?, promoted)
    }
    /// Original identity code including the one-flip marker.
    pub fn code(self) -> String {
        format!(
            "{}{}",
            self.base.code(),
            if self.promoted { "+" } else { "" }
        )
    }
    /// Exact movement face.
    pub const fn face(self) -> Face {
        use Base::*;
        if self.promoted {
            match self.base {
                Pawn => Face::Gold,
                GoBetween => Face::Elephant,
                Lance => Face::WhiteHorse,
                ReverseChariot => Face::Whale,
                Copper => Face::SideMover,
                Silver => Face::VerticalMover,
                Gold => Face::Rook,
                Leopard => Face::Bishop,
                Tiger => Face::FlyingStag,
                Elephant => Face::Prince,
                Kirin => Face::Lion,
                Phoenix => Face::FreeKing,
                SideMover => Face::FreeBoar,
                VerticalMover => Face::FlyingOx,
                Rook => Face::DragonKing,
                Bishop => Face::DragonHorse,
                DragonHorse => Face::HornedFalcon,
                DragonKing => Face::SoaringEagle,
                King => Face::King,
                Lion => Face::Lion,
                FreeKing => Face::FreeKing,
            }
        } else {
            match self.base {
                Pawn => Face::Pawn,
                GoBetween => Face::GoBetween,
                Lance => Face::Lance,
                ReverseChariot => Face::ReverseChariot,
                Copper => Face::Copper,
                Silver => Face::Silver,
                Gold => Face::Gold,
                Leopard => Face::Leopard,
                Tiger => Face::Tiger,
                Elephant => Face::Elephant,
                King => Face::King,
                Kirin => Face::Kirin,
                Phoenix => Face::Phoenix,
                SideMover => Face::SideMover,
                VerticalMover => Face::VerticalMover,
                Rook => Face::Rook,
                Bishop => Face::Bishop,
                DragonHorse => Face::DragonHorse,
                DragonKing => Face::DragonKing,
                Lion => Face::Lion,
                FreeKing => Face::FreeKing,
            }
        }
    }
    /// King or promoted original Elephant; a promoted Go-between is not a Prince.
    pub const fn is_royal(self) -> bool {
        matches!(self.face(), Face::King | Face::Prince)
    }
    /// Only actual Lion faces get the Lion exchange law.
    pub const fn is_lion(self) -> bool {
        matches!(self.face(), Face::Lion)
    }
    /// Last-rank dead originals retained as occupied squares.
    pub const fn dead(self, at: Square) -> bool {
        !self.promoted
            && matches!(self.base, Base::Pawn | Base::Lance)
            && self.side.last_rank(at.rank())
    }
}

/// Declared origin, not a reachability assertion for diagrams/imports.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootKind {
    /// Exact standard initial root.
    Standard,
    /// Explicit custom diagram.
    Diagram,
    /// Imported diagram with bounded source provenance.
    Imported,
}
/// Immutable structural root metadata.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootMetadata {
    kind: RootKind,
    source: Option<String>,
    declared_completed_plies: u64,
}
impl RootMetadata {
    /// Explicit diagram counter, with earlier history unknown.
    pub const fn diagram(declared_completed_plies: u64) -> Self {
        Self {
            kind: RootKind::Diagram,
            source: None,
            declared_completed_plies,
        }
    }
    /// Imported source bounded to1024 UTF-8 bytes.
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
    /// Root category.
    pub const fn kind(&self) -> RootKind {
        self.kind
    }
    /// Supplied import source.
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }
    /// Declared counter, not known historical actions.
    pub const fn declared_completed_plies(&self) -> u64 {
        self.declared_completed_plies
    }
    /// Imports and diagrams establish only the beginning of known history.
    pub const fn earlier_history_unknown(&self) -> bool {
        !matches!(self.kind, RootKind::Standard)
    }
    /// Only the standard root proves its initial reachability.
    pub const fn historical_reachability_proven(&self) -> bool {
        matches!(self.kind, RootKind::Standard)
    }
    fn valid(&self) -> bool {
        match self.kind {
            RootKind::Standard => self.source.is_none() && self.declared_completed_plies == 0,
            RootKind::Diagram => self.source.is_none(),
            RootKind::Imported => self
                .source
                .as_ref()
                .is_some_and(|s| !s.is_empty() && s.len() <= 1024),
        }
    }
}
/// Rejected structural data, never an illegal live move or game result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RootError {
    /// Duplicate occupied square.
    DuplicateSquare,
    /// Per-owner original inventory exceeds its initial cap.
    ExcessInventory,
    /// An unpromotable original is marked flipped.
    InvalidPromotion,
    /// Invalid bounded source metadata.
    InvalidProvenance,
    /// Standard identity cannot relabel a custom diagram.
    StandardRootNotDiagram,
    /// Native board length must be144.
    BoardLength,
    /// Counter addition overflows.
    Counter,
    /// Protection does not name actual defended owner Lions.
    InvalidProtection,
}
impl fmt::Display for RootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chu structural root rejected: {self:?}")
    }
}
impl Error for RootError {}
/// Full logical state excluding runtime reversal authority.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralState {
    pub(crate) board: Vec<Option<Piece>>,
    pub(crate) side: Side,
    pub(crate) root: RootMetadata,
    pub(crate) accepted_plies: u64,
}
/// A checked structural diagram, not game or history admission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position {
    pub(crate) state: StructuralState,
}
impl Position {
    /// Standard92-piece arrangement.
    pub fn standard() -> Self {
        use Base::*;
        let mut entries = Vec::new();
        let rows: [(u8, &[Option<Base>]); 5] = [
            (
                12,
                &[
                    Some(Lance),
                    Some(Leopard),
                    Some(Copper),
                    Some(Silver),
                    Some(Gold),
                    Some(King),
                    Some(Elephant),
                    Some(Gold),
                    Some(Silver),
                    Some(Copper),
                    Some(Leopard),
                    Some(Lance),
                ],
            ),
            (
                11,
                &[
                    Some(ReverseChariot),
                    None,
                    Some(Bishop),
                    None,
                    Some(Tiger),
                    Some(Kirin),
                    Some(Phoenix),
                    Some(Tiger),
                    None,
                    Some(Bishop),
                    None,
                    Some(ReverseChariot),
                ],
            ),
            (
                10,
                &[
                    Some(SideMover),
                    Some(VerticalMover),
                    Some(Rook),
                    Some(DragonHorse),
                    Some(DragonKing),
                    Some(Lion),
                    Some(FreeKing),
                    Some(DragonKing),
                    Some(DragonHorse),
                    Some(Rook),
                    Some(VerticalMover),
                    Some(SideMover),
                ],
            ),
            (9, &[Some(Pawn); 12]),
            (
                8,
                &[
                    None,
                    None,
                    None,
                    Some(GoBetween),
                    None,
                    None,
                    None,
                    None,
                    Some(GoBetween),
                    None,
                    None,
                    None,
                ],
            ),
        ];
        for (rank, row) in rows {
            for (i, base) in row.iter().enumerate() {
                if let Some(base) = base {
                    let sq = Square::new(12 - i as u8, rank).expect("bounded standard layout");
                    entries.push((
                        sq,
                        Piece {
                            side: Side::Sente,
                            base: *base,
                            promoted: false,
                        },
                    ));
                    entries.push((
                        sq.rotated(),
                        Piece {
                            side: Side::Gote,
                            base: *base,
                            promoted: false,
                        },
                    ));
                }
            }
        }
        let mut p = Self::from_diagram(&entries, Side::Sente, RootMetadata::diagram(0))
            .expect("valid standard inventory");
        p.state.root.kind = RootKind::Standard;
        p
    }
    /// Structural validation only. Missing royals can represent a captured terminal
    /// diagram; no royal-safety, prior history or reachability inference is made.
    pub fn from_diagram(
        entries: &[(Square, Piece)],
        side: Side,
        root: RootMetadata,
    ) -> Result<Self, RootError> {
        if root.kind == RootKind::Standard {
            return Err(RootError::StandardRootNotDiagram);
        }
        let mut board = vec![None; 144];
        for &(sq, piece) in entries {
            if board[sq.index()].replace(piece).is_some() {
                return Err(RootError::DuplicateSquare);
            }
        }
        Self::install(StructuralState {
            board,
            side,
            root,
            accepted_plies: 0,
        })
    }
    pub(crate) fn install(state: StructuralState) -> Result<Self, RootError> {
        if state.board.len() != 144 {
            return Err(RootError::BoardLength);
        }
        if !state.root.valid() {
            return Err(RootError::InvalidProvenance);
        }
        state
            .root
            .declared_completed_plies
            .checked_add(state.accepted_plies)
            .ok_or(RootError::Counter)?;
        let mut inventory = [[0u8; 21]; 2];
        for p in state.board.iter().flatten() {
            if p.promoted && !p.base.promotable() {
                return Err(RootError::InvalidPromotion);
            }
            let i = Base::ALL
                .iter()
                .position(|b| *b == p.base)
                .expect("enumerated base");
            inventory[p.side.index()][i] += 1;
            if inventory[p.side.index()][i] > p.base.cap() {
                return Err(RootError::ExcessInventory);
            }
        }
        Ok(Self { state })
    }
    /// Exact full logical structural snapshot.
    pub fn state(&self) -> &StructuralState {
        &self.state
    }
    /// Selected immutable rules identity.
    pub const fn profile_id(&self) -> &'static str {
        super::PROFILE_ID
    }
    /// Selected immutable edition.
    pub const fn profile_edition(&self) -> &'static str {
        super::PROFILE_EDITION
    }
    /// Exact occupant.
    pub fn piece(&self, at: Square) -> Option<Piece> {
        self.state.board[at.index()]
    }
    /// Exact occupant (coordinate-oriented synonym).
    pub fn at(&self, at: Square) -> Option<Piece> {
        self.piece(at)
    }
    /// All occupied squares in canonical order.
    pub fn pieces(&self) -> impl Iterator<Item = (Square, Piece)> + '_ {
        Square::all().filter_map(|s| self.piece(s).map(|p| (s, p)))
    }
    /// Native side to move.
    pub const fn side_to_move(&self) -> Side {
        self.state.side
    }
    /// Original root provenance.
    pub fn root(&self) -> &RootMetadata {
        &self.state.root
    }
    /// Actual admitted board actions since the root.
    pub const fn accepted_plies(&self) -> u64 {
        self.state.accepted_plies
    }
    /// Declared counter plus actual admitted board actions.
    pub fn declared_total_plies(&self) -> u64 {
        self.state.root.declared_completed_plies + self.state.accepted_plies
    }
}
