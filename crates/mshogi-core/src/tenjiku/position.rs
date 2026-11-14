//! Checked structural Tenjiku state. This layer makes no reachability or outcome claim.
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::BTreeSet, error::Error, fmt};

/// The selected complete rules package; immutable for the lifetime of a game.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub enum Profile {
    /// Adams's main-book Modern game, second edition1999.
    #[serde(rename = "mshogi.tenjiku.adams1999-modern.v1")]
    AdamsModern,
    /// Muller's movement/effects with the ratified named supplements.
    #[serde(rename = "mshogi.tenjiku.muller-completed.v1")]
    MullerCompleted,
}
impl Profile {
    /// All delivered native profile choices.
    pub const ALL: [Self; 2] = [Self::AdamsModern, Self::MullerCompleted];
    /// Exact canonical identity, never inferred from dimensions.
    pub const fn id(self) -> &'static str {
        match self {
            Self::AdamsModern => "mshogi.tenjiku.adams1999-modern.v1",
            Self::MullerCompleted => "mshogi.tenjiku.muller-completed.v1",
        }
    }
    /// Immutable source and completion edition.
    pub const fn edition(self) -> &'static str {
        match self {
            Self::AdamsModern => "ADAMS1999-MODERN+MSH83-2026-11-04",
            Self::MullerCompleted => "MULLER+ADAMS-COMPLETION+MSH83-2026-11-04",
        }
    }
}
/// Native owners; Sente moves first and toward decreasing rank.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Side {
    /// First player, moving toward rank1.
    Sente,
    /// Second player, moving toward rank16.
    Gote,
}
impl Side {
    /// Other native owner.
    pub const fn opponent(self) -> Self {
        match self {
            Self::Sente => Self::Gote,
            Self::Gote => Self::Sente,
        }
    }
    /// Stable array index.
    pub const fn index(self) -> usize {
        match self {
            Self::Sente => 0,
            Self::Gote => 1,
        }
    }
    /// Five farthest ranks, including their boundary.
    pub const fn in_zone(self, rank: u8) -> bool {
        match self {
            Self::Sente => rank >= 1 && rank <= 5,
            Self::Gote => rank >= 12 && rank <= 16,
        }
    }
    pub(crate) const fn last_rank(self, rank: u8) -> bool {
        match self {
            Self::Sente => rank == 1,
            Self::Gote => rank == 16,
        }
    }
}
/// One bounded square, with canonical `[file,rank]` serialization (both1..16).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Square(u16);
impl Square {
    /// Reject off-board coordinates rather than wrapping them.
    pub const fn new(file: u8, rank: u8) -> Option<Self> {
        if file >= 1 && file <= 16 && rank >= 1 && rank <= 16 {
            Some(Self((rank as u16 - 1) * 16 + file as u16 - 1))
        } else {
            None
        }
    }
    /// Native file1..16.
    pub const fn file(self) -> u8 {
        (self.0 % 16 + 1) as u8
    }
    /// Native rank1..16, a..p in human notation.
    pub const fn rank(self) -> u8 {
        (self.0 / 16 + 1) as u8
    }
    /// Row-major board index.
    pub const fn index(self) -> usize {
        self.0 as usize
    }
    /// All squares in canonical absolute rank/file order.
    pub fn all() -> impl Iterator<Item = Self> {
        (0..256).map(Self)
    }
    ///180-degree rotation; owners must be exchanged separately.
    pub const fn rotated(self) -> Self {
        Self(255 - self.0)
    }
    pub(crate) fn offset(self, side: Side, df: i8, dr: i8) -> Option<Self> {
        let sign = if side == Side::Sente { 1i16 } else { -1 };
        let f = i16::from(self.file()) + sign * i16::from(df);
        let r = i16::from(self.rank()) + sign * i16::from(dr);
        if (1..=16).contains(&f) && (1..=16).contains(&r) {
            Self::new(f as u8, r as u8)
        } else {
            None
        }
    }
    pub(crate) fn distance(self, other: Self) -> u8 {
        self.file()
            .abs_diff(other.file())
            .max(self.rank().abs_diff(other.rank()))
    }
}
impl Serialize for Square {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        [self.file(), self.rank()].serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for Square {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let [f, r] = <[u8; 2]>::deserialize(d)?;
        Self::new(f, r).ok_or_else(|| serde::de::Error::custom("Tenjiku square outside1..16"))
    }
}
impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.file(), char::from(b'a' + self.rank() - 1))
    }
}

macro_rules! kinds {
    ($($base:ident),+; $($only:ident),+) => {
        /// Original piece identity. A promoted piece retains this value forever.
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
        pub enum Base { $(#[doc=concat!("Original native ",stringify!($base)," identity.")] $base),+ }
        /// Current movement face, distinct from original identity/promotability.
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
        pub enum Face { $(#[doc=concat!("Native ",stringify!($base)," movement face.")] $base),+, $(#[doc=concat!("Promoted-only ",stringify!($only)," movement face.")] $only),+ }
        impl Base {
            /// All36 original native identities.
            pub const ALL: [Self;36] = [$(Self::$base),+];
            /// Native source abbreviation.
            pub const fn code(self) -> &'static str { match self { $(Self::$base => stringify!($base)),+ } }
            /// Original movement before promotion.
            pub const fn face(self) -> Face { match self { $(Self::$base => Face::$base),+ } }
        }
        impl Face {
            /// All44 movement faces.
            pub const ALL: [Self;44] = [$(Self::$base),+, $(Self::$only),+];
            /// Native source abbreviation.
            pub const fn code(self) -> &'static str { match self { $(Self::$base => stringify!($base)),+, $(Self::$only => stringify!($only)),+ } }
        }
    }
}
kinds!(K,FiD,GGn,VGn,RGn,BGn,LHk,Ln,WBf,FEg,FK,SE,HF,ChS,DK,DH,R,B,Ph,VSo,VM,SSo,SM,RC,L,Ky,G,DE,BT,FL,S,C,I,N,Dg,P; HT,FO,FB,WH,W,CP,FS,MGn);
impl Base {
    /// The sole once-only promotion; never follows a chain through current faces.
    pub const fn promotion(self) -> Option<Face> {
        use Face::*;
        Some(match self {
            Self::L => WH,
            Self::N => SSo,
            Self::FL => B,
            Self::I => VSo,
            Self::C => SM,
            Self::S => VM,
            Self::G => R,
            Self::DE => CP,
            Self::RC => W,
            Self::ChS => HT,
            Self::BT => FS,
            Self::Ky => Ln,
            Self::Ln => LHk,
            Self::FK => FEg,
            Self::Ph => FK,
            Self::SSo => WBf,
            Self::VSo => ChS,
            Self::B => DH,
            Self::DH => HF,
            Self::DK => SE,
            Self::WBf => FiD,
            Self::SM => FB,
            Self::VM => FO,
            Self::R => DK,
            Self::HF => BGn,
            Self::SE => RGn,
            Self::BGn => VGn,
            Self::RGn => GGn,
            Self::P => G,
            Self::Dg => MGn,
            Self::K | Self::FiD | Self::GGn | Self::VGn | Self::FEg | Self::LHk => return None,
        })
    }
    /// Whether this original can flip once.
    pub const fn promotable(self) -> bool {
        self.promotion().is_some()
    }
    fn cap(self) -> usize {
        match self {
            Self::K
            | Self::DE
            | Self::Ky
            | Self::Ln
            | Self::FK
            | Self::Ph
            | Self::LHk
            | Self::FEg
            | Self::GGn
            | Self::VGn => 1,
            Self::ChS => 4,
            Self::P => 16,
            _ => 2,
        }
    }
}
impl Face {
    /// Strict ranked-jump hierarchy, including the ratified Prince completion.
    pub const fn rank(self) -> u8 {
        match self {
            Self::K | Self::CP => 4,
            Self::GGn => 3,
            Self::VGn => 2,
            Self::BGn | Self::RGn => 1,
            _ => 0,
        }
    }
}
/// Per-original first-refusal state; unrelated pieces cannot consume it.
#[derive(
    Clone, Copy, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum PromotionRight {
    /// First eligible opportunity has not yet been used at the admitted root.
    #[default]
    Unseen,
    /// Only the next own move is restricted to capture-based promotion.
    FirstDeclinedNextMove,
    /// The one-time restriction is spent, or the piece cannot promote further.
    Consumed,
}
/// An original native participant, including stable identity and private-to-rules rights.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Piece {
    /// Stable original identity; unique across the whole admitted board.
    pub id: u16,
    /// Actual owner; captures never transfer ownership.
    pub side: Side,
    /// Original source kind, even after promotion.
    pub base: Base,
    /// Once-only promotion bit.
    pub promoted: bool,
    /// Per-original refusal state.
    pub right: PromotionRight,
}
impl Piece {
    /// Checked piece with declared-root default promotion rights.
    pub fn new(id: u16, side: Side, base: Base, promoted: bool) -> Result<Self, RootError> {
        if promoted && !base.promotable() {
            return Err(RootError::InvalidPromotion);
        }
        Ok(Self {
            id,
            side,
            base,
            promoted,
            right: if !promoted && base.promotable() {
                PromotionRight::Unseen
            } else {
                PromotionRight::Consumed
            },
        })
    }
    /// Current movement; no second promotion can follow this face.
    pub fn face(self) -> Face {
        if self.promoted {
            self.base.promotion().expect("checked promoted original")
        } else {
            self.base.face()
        }
    }
    /// King and promoted Elephant are the two possible native royals.
    pub const fn is_royal(self) -> bool {
        matches!(self.base, Base::K) || matches!(self.base, Base::DE) && self.promoted
    }
}
/// Provenance category; a diagram never proves earlier play.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootKind {
    /// Actual source initial setup with known empty history.
    Standard,
    /// Explicit diagram with no earlier-history claim.
    Diagram,
    /// Diagram with bounded external attribution.
    Imported,
}
/// Fixed beginning of known history, separate from current counters.
#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RootMetadata {
    kind: RootKind,
    source: Option<String>,
    declared_completed_plies: u64,
}
impl RootMetadata {
    /// Standard initial arrangement with complete empty initial history.
    pub const fn standard() -> Self {
        Self {
            kind: RootKind::Standard,
            source: None,
            declared_completed_plies: 0,
        }
    }
    /// Explicit unproved diagram with an optional declared earlier count.
    pub const fn diagram(declared_completed_plies: u64) -> Self {
        Self {
            kind: RootKind::Diagram,
            source: None,
            declared_completed_plies,
        }
    }
    /// Bounded attribution; it does not establish earlier repetition or rights.
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
    /// Optional bounded source.
    pub fn source(&self) -> Option<&str> {
        self.source.as_deref()
    }
    /// Declared earlier count, never known accepted actions.
    pub const fn declared_completed_plies(&self) -> u64 {
        self.declared_completed_plies
    }
    /// Whether the admitted root explicitly lacks earlier history.
    pub const fn earlier_history_unknown(&self) -> bool {
        !matches!(self.kind, RootKind::Standard)
    }
}
/// Structural rejection; never a game conclusion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RootError {
    /// Exactly256 board cells are required.
    BoardLength,
    /// Two supplied entries occupy the same square.
    DuplicateSquare,
    /// Stable original identities must be globally unique.
    DuplicateIdentity,
    /// An owner's original-kind inventory exceeds the source initial cap.
    ExcessInventory,
    /// An unpromotable original is marked promoted.
    InvalidPromotion,
    /// A promoted/unpromotable original carries an active promotion right.
    InvalidRights,
    /// Bounded root/source metadata is malformed.
    InvalidProvenance,
    /// A diagram or altered state claims the exact standard root.
    StandardRootNotDiagram,
    /// Declared earlier and known current counts overflow.
    Counter,
}
impl fmt::Display for RootError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tenjiku structural state rejected: {self:?}")
    }
}
impl Error for RootError {}
/// Fully serialized logical state; admission revalidates every field.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralState {
    pub(crate) board: Vec<Option<Piece>>,
    pub(crate) profile: Profile,
    pub(crate) side: Side,
    pub(crate) root: RootMetadata,
    pub(crate) accepted_plies: u64,
}
impl StructuralState {
    /// Exact immutable board cells.
    pub fn board(&self) -> &[Option<Piece>] {
        &self.board
    }
    /// Selected immutable profile.
    pub const fn profile(&self) -> Profile {
        self.profile
    }
}
/// Checked structural board, with no single-royal or Chess self-check assumption.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Position {
    pub(crate) state: StructuralState,
}
impl Position {
    /// Actual156-piece source setup with stable identities and canonical rights.
    pub fn standard(profile: Profile) -> Self {
        use Base::*;
        let rows: [(u8, &[Option<Base>]); 6] = [
            (
                16,
                &[
                    Some(L),
                    Some(N),
                    Some(FL),
                    Some(I),
                    Some(C),
                    Some(S),
                    Some(G),
                    Some(K),
                    Some(DE),
                    Some(G),
                    Some(S),
                    Some(C),
                    Some(I),
                    Some(FL),
                    Some(N),
                    Some(L),
                ],
            ),
            (
                15,
                &[
                    Some(RC),
                    None,
                    Some(ChS),
                    Some(ChS),
                    None,
                    Some(BT),
                    Some(Ky),
                    Some(Ln),
                    Some(FK),
                    Some(Ph),
                    Some(BT),
                    None,
                    Some(ChS),
                    Some(ChS),
                    None,
                    Some(RC),
                ],
            ),
            (
                14,
                &[
                    Some(SSo),
                    Some(VSo),
                    Some(B),
                    Some(DH),
                    Some(DK),
                    Some(WBf),
                    Some(FiD),
                    Some(LHk),
                    Some(FEg),
                    Some(FiD),
                    Some(WBf),
                    Some(DK),
                    Some(DH),
                    Some(B),
                    Some(VSo),
                    Some(SSo),
                ],
            ),
            (
                13,
                &[
                    Some(SM),
                    Some(VM),
                    Some(R),
                    Some(HF),
                    Some(SE),
                    Some(BGn),
                    Some(RGn),
                    Some(GGn),
                    Some(VGn),
                    Some(RGn),
                    Some(BGn),
                    Some(SE),
                    Some(HF),
                    Some(R),
                    Some(VM),
                    Some(SM),
                ],
            ),
            (12, &[Some(P); 16]),
            (
                11,
                &[
                    None,
                    None,
                    None,
                    None,
                    Some(Dg),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(Dg),
                    None,
                    None,
                    None,
                    None,
                ],
            ),
        ];
        let mut board = vec![None; 256];
        let mut id = 0;
        for side in [Side::Sente, Side::Gote] {
            for (rank, row) in rows {
                for (i, base) in row.iter().enumerate() {
                    if let Some(base) = base {
                        let mut s = Square::new(16 - i as u8, rank).expect("fixed native setup");
                        if side == Side::Gote {
                            s = s.rotated()
                        }
                        board[s.index()] = Some(
                            Piece::new(id, side, *base, false).expect("original native piece"),
                        );
                        id += 1;
                    }
                }
            }
        }
        Self {
            state: StructuralState {
                board,
                profile,
                side: Side::Sente,
                root: RootMetadata::standard(),
                accepted_plies: 0,
            },
        }
    }
    /// Admit structural placements without inventing historical reachability.
    pub fn from_diagram(
        profile: Profile,
        entries: &[(Square, Piece)],
        side: Side,
        root: RootMetadata,
    ) -> Result<Self, RootError> {
        if root.kind == RootKind::Standard {
            return Err(RootError::StandardRootNotDiagram);
        }
        let mut board = vec![None; 256];
        for &(s, p) in entries {
            if board[s.index()].replace(p).is_some() {
                return Err(RootError::DuplicateSquare);
            }
        }
        Self::install(StructuralState {
            board,
            profile,
            side,
            root,
            accepted_plies: 0,
        })
    }
    /// Checked semantic restoration/root reconstruction.
    pub fn install(state: StructuralState) -> Result<Self, RootError> {
        if state.board.len() != 256 {
            return Err(RootError::BoardLength);
        }
        let valid_root = match state.root.kind {
            RootKind::Standard => {
                state.root.source.is_none() && state.root.declared_completed_plies == 0
            }
            RootKind::Diagram => state.root.source.is_none(),
            RootKind::Imported => state
                .root
                .source
                .as_ref()
                .is_some_and(|s| !s.is_empty() && s.len() <= 1024),
        };
        if !valid_root {
            return Err(RootError::InvalidProvenance);
        }
        state
            .root
            .declared_completed_plies
            .checked_add(state.accepted_plies)
            .ok_or(RootError::Counter)?;
        let mut identities = BTreeSet::new();
        let mut inventory = std::collections::BTreeMap::new();
        for p in state.board.iter().flatten() {
            if !identities.insert(p.id) {
                return Err(RootError::DuplicateIdentity);
            }
            if p.promoted && !p.base.promotable() {
                return Err(RootError::InvalidPromotion);
            }
            if (p.promoted || !p.base.promotable()) && p.right != PromotionRight::Consumed {
                return Err(RootError::InvalidRights);
            }
            let n = inventory.entry((p.side, p.base)).or_insert(0usize);
            *n += 1;
            if *n > p.base.cap() {
                return Err(RootError::ExcessInventory);
            }
        }
        if state.root.kind == RootKind::Standard
            && state.accepted_plies == 0
            && state != Self::standard(state.profile).state
        {
            return Err(RootError::StandardRootNotDiagram);
        }
        Ok(Self { state })
    }
    /// Full read-only structural state.
    pub fn state(&self) -> &StructuralState {
        &self.state
    }
    /// Piece at one bounded square.
    pub fn piece(&self, s: Square) -> Option<Piece> {
        self.state.board[s.index()]
    }
    /// Occupied cells in absolute rank/file order.
    pub fn pieces(&self) -> impl Iterator<Item = (Square, Piece)> + '_ {
        Square::all().filter_map(|s| self.piece(s).map(|p| (s, p)))
    }
    /// Selected immutable complete profile.
    pub const fn profile(&self) -> Profile {
        self.state.profile
    }
    /// Actual side to move.
    pub const fn side_to_move(&self) -> Side {
        self.state.side
    }
    /// Known accepted actions since the root.
    pub const fn accepted_plies(&self) -> u64 {
        self.state.accepted_plies
    }
    /// Fixed root provenance.
    pub fn root(&self) -> &RootMetadata {
        &self.state.root
    }
}
