//! Inspectable conventional Chess evaluation with reversible incremental terms.
//!
//! Material, phase and original arithmetic piece placement are updated for only
//! changed board cells. Pawn/activity/safety/threat terms use the current native
//! geometry at evaluation time. Native outcomes and history/claims belong to
//! search, never this static heuristic. No learned parameters or upstream tables.
use mshogi_core::chess::{Color, Piece, PieceKind, Position, Square};

/// Identity of this original, untrained conventional evaluator.
pub const EVALUATOR_ID: &str = "mshogi.chess.conventional-tapered.v1";
/// Initial total non-pawn material; promotion may increase raw phase above this.
pub const MIDDLE_PHASE: i32 = 6_400;
/// Conservative absolute-score ceiling, well below the native mate-score band.
///
/// Validated native positions have at most16pieces and8pawns per color, one
/// king each. Bounding each color's material by14400, placement by1728, pawn
/// terms by4000, activity by1636, king safety by6500, threats by896 and conversion
/// by144 bounds a color's absolute contribution below30000; their difference is
/// therefore below60000. The larger100000 ceiling leaves conservative slack.
pub const MAX_ABS_STATIC_SCORE: i32 = 100_000;

/// One inspectable term's middle-game and endgame contributions in centipawns.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tapered {
    /// Contribution at full middle-game phase.
    pub middle: i32,
    /// Contribution at zero non-pawn-material phase.
    pub end: i32,
}
impl Tapered {
    const fn new(middle: i32, end: i32) -> Self {
        Self { middle, end }
    }
    fn add(&mut self, other: Self) {
        self.middle += other.middle;
        self.end += other.end;
    }
    fn subtract(&mut self, other: Self) {
        self.middle -= other.middle;
        self.end -= other.end;
    }
}

/// A color's contributions; terms are heuristics, not legal-move or draw proofs.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Terms {
    /// Conventional 100/320/330/500/900 material, king zero.
    pub material: i32,
    /// Original arithmetic piece placement, incrementally maintained.
    pub placement: Tapered,
    /// Isolated/doubled/connected/backward and passed-pawn structure.
    pub pawns: Tapered,
    /// Geometric activity, safe-from-pawn mobility and rook/bishop activity.
    pub mobility: Tapered,
    /// Shelter, pawn storms, ring pressure and geometrically safe checking threats.
    pub king_safety: Tapered,
    /// Attacked undefended or more valuable enemy pieces; geometric estimates.
    pub threats: Tapered,
    /// Conversion encouragement when up major material, not a win recognizer.
    pub endgame: Tapered,
}
impl Terms {
    fn contribution(self, phase: i32) -> i32 {
        let terms = [
            self.placement,
            self.pawns,
            self.mobility,
            self.king_safety,
            self.threats,
            self.endgame,
        ];
        let middle: i32 = terms.iter().map(|t| t.middle).sum();
        let end: i32 = terms.iter().map(|t| t.end).sum();
        self.material + (middle * phase + end * (MIDDLE_PHASE - phase)) / MIDDLE_PHASE
    }
}

/// Full transparent decomposition. Positive scores favor the requested color.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Evaluation {
    /// White's own contributions.
    pub white: Terms,
    /// Black's own contributions.
    pub black: Terms,
    /// Shared blend in0..=6400, capped at initial non-pawn material.
    pub middle_phase: i32,
    /// Material-specific advisory estimate, never a native outcome or claim.
    pub specialized: Option<EndgameEvaluation>,
}

/// Applicability of original classical endgame knowledge.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndgameKind {
    /// Native-generated KPK board win/draw knowledge; ignores history and clocks.
    KingPawnKing,
    /// Wrong-colored bishop and one rook pawn, defender on the promotion corner.
    WrongBishopRookPawn,
    /// Bishop and knight against a bare king: encourage the correct corner.
    BishopKnightConversion,
}

/// Transparent replacement for generic terms in a narrowly matched ending.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EndgameEvaluation {
    /// Material-specific applicability, including whether the score is heuristic.
    pub kind: EndgameKind,
    /// Side owning the pawn or mating material.
    pub strong_side: Color,
    /// Centipawns from that side, always outside the mate band. Even zero is
    /// advisory: search must still process native outcomes and draw claims.
    pub score: i32,
}
impl Evaluation {
    /// Static estimate in centipawns; zero does not establish a draw.
    pub fn score_for(self, side: Color) -> i32 {
        let score = self.specialized.map_or_else(
            || {
                self.white.contribution(self.middle_phase)
                    - self.black.contribution(self.middle_phase)
            },
            |e| {
                if e.strong_side == Color::White {
                    e.score
                } else {
                    -e.score
                }
            },
        );
        debug_assert!(score.abs() < MAX_ABS_STATIC_SCORE);
        if side == Color::White { score } else { -score }
    }
    /// Contributions for a named color, for inspectable diagnostics.
    pub const fn terms(self, side: Color) -> Terms {
        match side {
            Color::White => self.white,
            Color::Black => self.black,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Accumulator {
    material: [i32; 2],
    placement: [Tapered; 2],
    phase: i32,
}
fn side(color: Color) -> usize {
    usize::from(color == Color::Black)
}
fn square(index: usize) -> Square {
    Square::new((index % 8) as u8, (index / 8) as u8).unwrap()
}
fn bit(square: Square) -> u64 {
    1 << (square.rank() * 8 + square.file())
}
fn center(file: u8, rank: u8) -> i32 {
    (14 - (2 * i32::from(file) - 7).abs() - (2 * i32::from(rank) - 7).abs()) / 2
}
fn file_center(file: u8) -> i32 {
    3 - (i32::from(file) - 3).abs().min((i32::from(file) - 4).abs())
}
fn distance(a: Square, b: Square) -> i32 {
    i32::from(a.file().abs_diff(b.file()).max(a.rank().abs_diff(b.rank())))
}
fn relative_rank(piece: Piece, sq: Square) -> u8 {
    if piece.color == Color::White {
        sq.rank()
    } else {
        7 - sq.rank()
    }
}
fn placement(piece: Piece, sq: Square) -> Tapered {
    let rank = relative_rank(piece, sq);
    let advance = (i32::from(rank) - 1).max(0);
    let center = center(sq.file(), sq.rank());
    match piece.kind {
        PieceKind::Pawn => Tapered::new(
            4 * advance + 4 * file_center(sq.file()),
            10 * advance + 2 * file_center(sq.file()),
        ),
        PieceKind::Knight => Tapered::new(12 * center - 8 * i32::from(rank == 0), 8 * center),
        PieceKind::Bishop => Tapered::new(
            7 * center + 3 * advance - 8 * i32::from(rank == 0),
            6 * center,
        ),
        PieceKind::Rook => Tapered::new(2 * center, 4 * center),
        PieceKind::Queen => Tapered::new(3 * center - 6 * i32::from(rank > 1), 5 * center),
        PieceKind::King => Tapered::new(-10 * center - 8 * advance, 14 * center),
    }
}
impl Accumulator {
    fn rebuilt(position: &Position) -> Self {
        let mut result = Self::default();
        for index in 0..64 {
            let sq = square(index);
            if let Some(piece) = position.piece_at(sq) {
                result.insert(piece, sq);
            }
        }
        result
    }
    fn insert(&mut self, piece: Piece, sq: Square) {
        let i = side(piece.color);
        let value = crate::chess::piece_value(piece.kind);
        self.material[i] += value;
        self.placement[i].add(placement(piece, sq));
        if !matches!(piece.kind, PieceKind::Pawn | PieceKind::King) {
            self.phase += value;
        }
    }
    fn remove(&mut self, piece: Piece, sq: Square) {
        let i = side(piece.color);
        let value = crate::chess::piece_value(piece.kind);
        self.material[i] -= value;
        self.placement[i].subtract(placement(piece, sq));
        if !matches!(piece.kind, PieceKind::Pawn | PieceKind::King) {
            self.phase -= value;
        }
    }
}

/// Search-private material/placement/phase state with exact reversible updates.
///
/// Pair each successful native move with `push(before, after)` and each native
/// undo with `pop()`. Changed cells are detected from the two native snapshots,
/// which also handles castling, EP and promotions without reimplementing effects.
/// No history or native position is mutated here. Raw phase is kept uncapped so
/// promotions and undo restore it exactly; only evaluation clamps the blend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvalState {
    current: Accumulator,
    undo: Vec<Accumulator>,
}
impl EvalState {
    /// Initialize from a native position.
    pub fn new(position: &Position) -> Self {
        Self {
            current: Accumulator::rebuilt(position),
            undo: Vec::new(),
        }
    }
    /// Update changed cells after an actually admitted native transition.
    pub fn push(&mut self, before: &Position, after: &Position) {
        debug_assert_eq!(self.current, Accumulator::rebuilt(before));
        self.undo.push(self.current);
        for index in 0..64 {
            let sq = square(index);
            let previous = before.piece_at(sq);
            let next = after.piece_at(sq);
            if previous == next {
                continue;
            }
            if let Some(piece) = previous {
                self.current.remove(piece, sq);
            }
            if let Some(piece) = next {
                self.current.insert(piece, sq);
            }
        }
        debug_assert_eq!(self.current, Accumulator::rebuilt(after));
    }
    /// Restore the previous accumulator; false means no pending transition.
    pub fn pop(&mut self) -> bool {
        let Some(previous) = self.undo.pop() else {
            return false;
        };
        self.current = previous;
        true
    }
    /// Evaluate the matching native board, using cached material/placement/phase.
    pub fn evaluate(&self, position: &Position) -> Evaluation {
        debug_assert_eq!(self.current, Accumulator::rebuilt(position));
        dynamic(position, self.current)
    }
}

#[derive(Clone, Copy)]
struct Man {
    piece: Piece,
    sq: Square,
    attacks: u64,
}
struct Features {
    men: Vec<Man>,
    occupied: [u64; 2],
    attacks: [u64; 2],
    double_attacks: [u64; 2],
    nonking_attacks: [u64; 2],
    pawn_attacks: [u64; 2],
    pawn_files: [[u8; 8]; 2],
    pawn_ranks: [[u8; 8]; 2],
    bishops: [u8; 2],
    majors: [u8; 2],
    kings: [Square; 2],
}
impl Features {
    fn new(position: &Position) -> Self {
        let mut f = Self {
            men: Vec::with_capacity(32),
            occupied: [0; 2],
            attacks: [0; 2],
            double_attacks: [0; 2],
            nonking_attacks: [0; 2],
            pawn_attacks: [0; 2],
            pawn_files: [[0; 8]; 2],
            pawn_ranks: [[0; 8]; 2],
            bishops: [0; 2],
            majors: [0; 2],
            kings: [square(0); 2],
        };
        for index in 0..64 {
            let sq = square(index);
            let Some(piece) = position.piece_at(sq) else {
                continue;
            };
            let i = side(piece.color);
            let attacks = position.attacks_from(sq);
            f.men.push(Man { piece, sq, attacks });
            f.occupied[i] |= bit(sq);
            f.double_attacks[i] |= f.attacks[i] & attacks;
            f.attacks[i] |= attacks;
            if piece.kind != PieceKind::King {
                f.nonking_attacks[i] |= attacks;
            }
            match piece.kind {
                PieceKind::Pawn => {
                    f.pawn_files[i][usize::from(sq.file())] += 1;
                    f.pawn_ranks[i][usize::from(sq.file())] |= 1 << sq.rank();
                    f.pawn_attacks[i] |= attacks;
                }
                PieceKind::Bishop => f.bishops[i] += 1,
                PieceKind::Rook | PieceKind::Queen => f.majors[i] += 1,
                PieceKind::King => f.kings[i] = sq,
                PieceKind::Knight => {}
            }
        }
        f
    }
    fn backward(&self, man: &Man) -> bool {
        let i = side(man.piece.color);
        let file = usize::from(man.sq.file());
        let mut neighbors = (file.saturating_sub(1)..=(file + 1).min(7)).filter(|j| *j != file);
        let behind_or_level = if man.piece.color == Color::White {
            ((1u16 << (man.sq.rank() + 1)) - 1) as u8
        } else {
            (0xffu16 << man.sq.rank()) as u8
        };
        let has_neighbor = neighbors.clone().any(|j| self.pawn_files[i][j] != 0);
        let rear_support = neighbors.any(|j| self.pawn_ranks[i][j] & behind_or_level != 0);
        let next = if man.piece.color == Color::White {
            man.sq.rank() + 1
        } else {
            man.sq.rank() - 1
        };
        let front = Square::new(man.sq.file(), next).expect("validated pawn rank");
        has_neighbor && !rear_support && self.pawn_attacks[1 - i] & bit(front) != 0
    }
    fn pawn_storm(&self, king: &Man) -> i32 {
        let i = side(king.piece.color);
        let mut penalty = 0;
        for pawn in self
            .men
            .iter()
            .filter(|m| m.piece.kind == PieceKind::Pawn && m.piece.color != king.piece.color)
        {
            if pawn.sq.file().abs_diff(king.sq.file()) > 1 {
                continue;
            }
            let forward = if king.piece.color == Color::White {
                i32::from(pawn.sq.rank()) - i32::from(king.sq.rank())
            } else {
                i32::from(king.sq.rank()) - i32::from(pawn.sq.rank())
            };
            if !(1..=3).contains(&forward) {
                continue;
            }
            let next = if pawn.piece.color == Color::White {
                pawn.sq.rank() + 1
            } else {
                pawn.sq.rank() - 1
            };
            let blocked = self.pawn_ranks[i][usize::from(pawn.sq.file())] & (1 << next) != 0;
            penalty += (4 - forward) * if blocked { 3 } else { 6 };
        }
        penalty
    }
}

/// Rebuild all incremental terms and evaluate without altering the native board.
///
/// Dynamic attacks come from native geometric queries, including pinned pieces
/// and defended occupied squares. Mobility discounts own occupancy and enemy
/// pawn attacks; it is an activity estimate, not a count of legal moves. King
/// ring and threat estimates likewise never decide check, capture or outcome.
/// Generic phase blending does not depend on side to move. Specialized KPK
/// board knowledge includes the turn; clocks, claims and history stay native.
/// All coefficients are original, transparent untrained arithmetic heuristics.
pub fn evaluate(position: &Position) -> Evaluation {
    dynamic(position, Accumulator::rebuilt(position))
}

fn dynamic(position: &Position, base: Accumulator) -> Evaluation {
    let f = Features::new(position);
    let mut terms = [Terms::default(); 2];
    for (i, term) in terms.iter_mut().enumerate() {
        term.material = base.material[i];
        term.placement = base.placement[i];
        for count in f.pawn_files[i] {
            let extra = i32::from(count.saturating_sub(1));
            term.pawns.add(Tapered::new(-12 * extra, -16 * extra));
        }
        if f.bishops[i] >= 2 {
            term.mobility.add(Tapered::new(28, 36));
        }
    }
    for man in &f.men {
        let i = side(man.piece.color);
        let other = 1 - i;
        let file = usize::from(man.sq.file());
        let rank = relative_rank(man.piece, man.sq);
        let advance = (i32::from(rank) - 1).max(0);
        match man.piece.kind {
            PieceKind::Pawn => {
                let adjacent = file.saturating_sub(1)..=(file + 1).min(7);
                let isolated = (file == 0 || f.pawn_files[i][file - 1] == 0)
                    && (file == 7 || f.pawn_files[i][file + 1] == 0);
                terms[i].pawns.add(Tapered::new(
                    -10 * i32::from(isolated),
                    -8 * i32::from(isolated),
                ));
                let near_ranks = ((1u16 << man.sq.rank())
                    | (1u16 << man.sq.rank().saturating_sub(1))
                    | (1u16 << (man.sq.rank() + 1).min(7))) as u8;
                let connected = adjacent
                    .clone()
                    .filter(|&j| j != file)
                    .any(|j| f.pawn_ranks[i][j] & near_ranks != 0);
                if connected {
                    terms[i]
                        .pawns
                        .add(Tapered::new(4 + advance, 6 + 2 * advance));
                }
                let ahead = if man.piece.color == Color::White {
                    (0xffu16 << (u32::from(man.sq.rank()) + 1)) as u8
                } else {
                    ((1u16 << man.sq.rank()) - 1) as u8
                };
                let passed = adjacent
                    .clone()
                    .all(|j| f.pawn_ranks[other][j] & ahead == 0);
                let next_rank = if man.piece.color == Color::White {
                    man.sq.rank() + 1
                } else {
                    man.sq.rank() - 1
                };
                let front =
                    Square::new(man.sq.file(), next_rank).expect("validated pawn not on last rank");
                if passed && f.pawn_ranks[i][file] & ahead == 0 {
                    let middle = 4 + 3 * advance * advance;
                    let end = 10 + 7 * advance * advance;
                    let blocked = position.piece_at(front).is_some();
                    let escort =
                        4 * (distance(f.kings[other], front) - distance(f.kings[i], front));
                    let mut path = 0u64;
                    let mut path_rank = i32::from(next_rank);
                    let step = if man.piece.color == Color::White {
                        1
                    } else {
                        -1
                    };
                    while (0..8).contains(&path_rank) {
                        path |= bit(Square::new(man.sq.file(), path_rank as u8).unwrap());
                        path_rank += step;
                    }
                    // Defending the destination does not prevent capture:
                    // a contested promotion square remains unsafe.
                    let unsafe_path = path & f.attacks[other] != 0;
                    let clear = path & (f.occupied[i] | f.occupied[other]) == 0;
                    let runway = if advance >= 3 && clear && !unsafe_path {
                        4 * advance * advance
                    } else {
                        0
                    };
                    terms[i].pawns.add(Tapered::new(
                        middle,
                        if blocked { end / 2 } else { end } + escort + runway
                            - if unsafe_path { end / 4 } else { 0 },
                    ));
                }
                if f.backward(man) {
                    terms[i].pawns.add(Tapered::new(-8, -6));
                }
            }
            PieceKind::Knight | PieceKind::Bishop | PieceKind::Rook | PieceKind::Queen => {
                let mobility =
                    (man.attacks & !f.occupied[i] & !f.pawn_attacks[other]).count_ones() as i32;
                let (middle, end) = match man.piece.kind {
                    PieceKind::Knight => (5, 4),
                    PieceKind::Bishop => (4, 5),
                    PieceKind::Rook => (2, 4),
                    PieceKind::Queen => (1, 2),
                    _ => unreachable!(),
                };
                terms[i]
                    .mobility
                    .add(Tapered::new(middle * mobility, end * mobility));
                if man.piece.kind == PieceKind::Rook {
                    if f.pawn_files[i][file] == 0 {
                        terms[i].mobility.add(if f.pawn_files[other][file] == 0 {
                            Tapered::new(16, 12)
                        } else {
                            Tapered::new(8, 6)
                        });
                    }
                    if rank == 6 {
                        terms[i].mobility.add(Tapered::new(18, 24));
                    }
                }
            }
            PieceKind::King => {
                let king_file = usize::from(man.sq.file());
                let mut shelter = 0;
                for file in king_file.saturating_sub(1)..=(king_file + 1).min(7) {
                    for (ahead, bonus) in [(1u8, 10), (2, 5)] {
                        let rank = if man.piece.color == Color::White {
                            man.sq.rank().checked_add(ahead).filter(|r| *r < 8)
                        } else {
                            man.sq.rank().checked_sub(ahead)
                        };
                        if rank.is_some_and(|rank| f.pawn_ranks[i][file] & (1 << rank) != 0) {
                            shelter += bonus;
                        }
                    }
                    if f.pawn_files[i][file] == 0 {
                        shelter -= 12 + 8 * i32::from(f.pawn_files[other][file] == 0);
                    }
                }
                let castled = rank == 0 && matches!(man.sq.file(), 2 | 6);
                let ring = man.attacks | bit(man.sq);
                let mut pressure = 0;
                let mut attackers = 0i32;
                let mut checks = 0;
                for enemy in f
                    .men
                    .iter()
                    .filter(|m| m.piece.color != man.piece.color && m.piece.kind != PieceKind::King)
                {
                    let attacked = (enemy.attacks & ring).count_ones() as i32;
                    let weight = match enemy.piece.kind {
                        PieceKind::Pawn => 2,
                        PieceKind::Knight => 4,
                        PieceKind::Bishop | PieceKind::Rook => 3,
                        PieceKind::Queen => 5,
                        PieceKind::King => unreachable!(),
                    };
                    pressure += attacked * weight;
                    attackers += i32::from(attacked != 0);
                    if !matches!(enemy.piece.kind, PieceKind::Pawn | PieceKind::King) {
                        // Hypothetical checking origins are native attack maps
                        // from the king. A candidate is reachable geometrically,
                        // not friendly occupied or defended by a non-king; near
                        // the king it must have a second attacking supporter.
                        // Pins/discovered lines remain search's responsibility.
                        let origins = position.attacks_as(enemy.piece, man.sq);
                        let safe = !f.occupied[other]
                            & !f.nonking_attacks[i]
                            & (!man.attacks | f.double_attacks[other]);
                        let count = (origins & enemy.attacks & safe).count_ones().min(2) as i32;
                        let check_weight = match enemy.piece.kind {
                            PieceKind::Knight => 14,
                            PieceKind::Bishop => 12,
                            PieceKind::Rook => 16,
                            PieceKind::Queen => 12,
                            _ => unreachable!(),
                        };
                        checks += count * check_weight;
                    }
                }
                terms[i].king_safety.add(Tapered::new(
                    shelter + 20 * i32::from(castled)
                        - pressure * (2 + attackers.min(4))
                        - checks
                        - f.pawn_storm(man),
                    0,
                ));
            }
        }
        if man.piece.kind != PieceKind::King && f.attacks[other] & bit(man.sq) != 0 {
            let victim = crate::chess::piece_value(man.piece.kind);
            let undefended = f.attacks[i] & bit(man.sq) == 0;
            let least_attacker = f
                .men
                .iter()
                .filter(|m| side(m.piece.color) == other && m.attacks & bit(man.sq) != 0)
                .map(|m| {
                    if m.piece.kind == PieceKind::King {
                        20_000
                    } else {
                        crate::chess::piece_value(m.piece.kind)
                    }
                })
                .min()
                .unwrap_or(20_000);
            let bonus = if undefended {
                victim / 16
            } else {
                ((victim - least_attacker).max(0) / 32).min(24)
            };
            terms[other].threats.add(Tapered::new(bonus, bonus));
        }
    }
    for (i, term) in terms.iter_mut().enumerate() {
        if f.majors[i] != 0 && base.material[i] - base.material[1 - i] >= 400 {
            // A conversion preference, not an assertion that a mate is forced.
            let edge = 6 - center(f.kings[1 - i].file(), f.kings[1 - i].rank());
            let approach = 7 - distance(f.kings[i], f.kings[1 - i]);
            let bare_defender = f
                .men
                .iter()
                .all(|man| side(man.piece.color) == i || man.piece.kind == PieceKind::King);
            term.endgame.end = if bare_defender {
                // Generic endgame centrality must not oppose approaching a
                // cornered bare king. Cancel that king-only placement term,
                // then use confinement/proximity as original mop-up heuristics.
                10 * edge + 12 * approach - 14 * center(f.kings[i].file(), f.kings[i].rank())
            } else {
                10 * edge + 4 * approach
            };
        }
    }
    Evaluation {
        white: terms[0],
        black: terms[1],
        middle_phase: base.phase.min(MIDDLE_PHASE),
        specialized: specialized(position, &f),
    }
}

fn specialized(position: &Position, f: &Features) -> Option<EndgameEvaluation> {
    // These material sets cannot retain castling rights. Ignore raw EP targets:
    // with only one pawn no en-passant capture is possible (including immediately
    // after a legal double push). Native Game still owns all history semantics.
    if f.men.len() > 5 {
        return None;
    }
    let nonkings: Vec<_> = f
        .men
        .iter()
        .filter(|m| m.piece.kind != PieceKind::King)
        .collect();
    if let [pawn] = nonkings.as_slice()
        && pawn.piece.kind == PieceKind::Pawn
    {
        let win = crate::chess_kpk::probe(position)?;
        return Some(EndgameEvaluation {
            kind: EndgameKind::KingPawnKing,
            strong_side: pawn.piece.color,
            // WDL membership is exact board knowledge; conversion preferences
            // are ordinary original heuristics, not distance-to-mate data.
            // Stay below a rook so a winning underpromotion is not discouraged.
            score: if win {
                300 + 24 * i32::from(relative_rank(pawn.piece, pawn.sq))
            } else {
                0
            },
        });
    }
    if nonkings.len() != 2 || nonkings[0].piece.color != nonkings[1].piece.color {
        return None;
    }
    let bishop = nonkings
        .iter()
        .find(|m| m.piece.kind == PieceKind::Bishop)?;
    let other = nonkings
        .iter()
        .find(|m| m.piece.kind != PieceKind::Bishop)?;
    let strong_side = bishop.piece.color;
    let i = side(strong_side);
    let defender = f.kings[1 - i];
    let bishop_color = (bishop.sq.file() + bishop.sq.rank()) % 2;
    if other.piece.kind == PieceKind::Pawn && matches!(other.sq.file(), 0 | 7) {
        let corner = Square::new(
            other.sq.file(),
            if strong_side == Color::White { 7 } else { 0 },
        )
        .unwrap();
        if defender == corner && (corner.file() + corner.rank()) % 2 != bishop_color {
            return Some(EndgameEvaluation {
                kind: EndgameKind::WrongBishopRookPawn,
                strong_side,
                score: 0,
            });
        }
    }
    if other.piece.kind == PieceKind::Knight {
        let corners = if bishop_color == 0 {
            [square(0), square(63)]
        } else {
            [square(7), square(56)]
        };
        let corner_distance = corners
            .into_iter()
            .map(|c| distance(defender, c))
            .min()
            .unwrap();
        let approach = 7 - distance(f.kings[i], defender);
        return Some(EndgameEvaluation {
            kind: EndgameKind::BishopKnightConversion,
            strong_side,
            // Not a win certificate: hanging pieces, stalemate and the fifty-
            // move claim must be handled by actual search/native admission.
            score: 650 + 24 * (7 - corner_distance) + 12 * approach,
        });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy, STANDARD_START_FEN};

    fn position(fen: &str) -> Position {
        Position::from_fen(fen).unwrap()
    }
    fn reflect(position: &Position) -> Position {
        let fen = position.to_fen();
        let parts: Vec<_> = fen.split_whitespace().collect();
        assert_eq!((parts[2], parts[3]), ("-", "-"));
        let board = parts[0]
            .split('/')
            .rev()
            .map(|rank| {
                rank.chars()
                    .map(|c| {
                        if c.is_ascii_uppercase() {
                            c.to_ascii_lowercase()
                        } else {
                            c.to_ascii_uppercase()
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("/");
        Position::from_fen(&format!(
            "{board} {} - - {} {}",
            if position.side_to_move() == Color::White {
                "b"
            } else {
                "w"
            },
            parts[4],
            parts[5]
        ))
        .unwrap()
    }

    #[test]
    fn independently_derived_term_arithmetic_and_perspective() {
        // Bare king centrality: Kd4 has6, Kh8 has0; zero phase =>14*6=84.
        let p = position("7k/8/8/8/3K4/8/8/8 w - - 0 1");
        let e = evaluate(&p);
        assert_eq!(e.middle_phase, 0);
        assert_eq!(e.white.placement, Tapered::new(-76, 84));
        assert_eq!(e.score_for(Color::White), 84);
        assert_eq!(e.score_for(Color::Black), -84);
        // A2 pawn:100 material,0placement,-8isolated+10passed+20king escort.
        let e = evaluate(&position("7k/8/8/8/8/8/P7/K7 w - - 0 1"));
        assert_eq!(e.white.material, 100);
        assert_eq!(e.white.pawns.end, 22);
        assert_eq!(
            e.white.contribution(e.middle_phase) - e.black.contribution(e.middle_phase),
            122
        );
        // A6 pawn:40placement,-8isolated+122passed+4escort+64clear runway.
        let e = evaluate(&position("7k/8/P7/8/8/8/8/K7 w - - 0 1"));
        assert_eq!(e.white.placement.end, 40);
        assert_eq!(e.white.pawns.end, 182);
        assert_eq!(
            e.white.contribution(e.middle_phase) - e.black.contribution(e.middle_phase),
            322
        );
        // RookA1 attacks14squares, oneoccupiedbyownKh1 excluded:13mobility.
        let e = evaluate(&position("7k/8/8/8/8/8/8/R6K w - - 0 1"));
        assert_eq!(e.white.mobility, Tapered::new(13 * 2 + 16, 13 * 4 + 12));
    }
    #[test]
    fn independent_pawn_structure_shelter_and_hanging_threat_terms() {
        // Each adjacent e/f pawn is connected, neither side has a passed pawn.
        let e = evaluate(&position("7k/4pp2/8/8/8/8/4PP2/K7 w - - 0 1"));
        assert_eq!(e.white.pawns, Tapered::new(8, 12));
        // Two isolated a pawns blocked by the enemy file pawn: extra doubled
        // pawn -12/-16 plus two isolated penalties -20/-16.
        let e = evaluate(&position("7k/p7/8/8/8/P7/P7/K7 w - - 0 1"));
        assert_eq!(e.white.pawns, Tapered::new(-32, -32));
        let shelter = evaluate(&position("7k/8/8/8/8/8/5P2/6K1 w - - 0 1"));
        let exposed = evaluate(&position("7k/8/8/8/5P2/8/8/6K1 w - - 0 1"));
        assert_eq!(
            shelter.white.king_safety.middle - exposed.white.king_safety.middle,
            10
        );
        // Undefended queenA7 attacked by rookA1 gives900/16=56, once per victim.
        let e = evaluate(&position("7k/q7/8/8/8/8/8/R6K w - - 0 1"));
        assert_eq!(e.white.threats, Tapered::new(56, 56));
    }
    #[test]
    fn backward_pawn_support_direction_and_passed_pawn_runway_are_explicit() {
        // D3 cannot safely advance to d4, which e5 attacks. C4 is ahead and
        // cannot come back to support it; c2 can advance toward that support.
        for (fen, backward) in [
            ("7k/8/8/4p3/2P5/3P4/8/K7 w - - 0 1", true),
            ("7k/8/8/4p3/8/3P4/2P5/K7 w - - 0 1", false),
        ] {
            let p = position(fen);
            let f = Features::new(&p);
            let man = f.men.iter().find(|m| m.sq == square(19)).unwrap(); // d3
            assert_eq!(f.backward(man), backward);
            let mirror = reflect(&p);
            let m = Features::new(&mirror);
            let man = m.men.iter().find(|m| m.sq == square(43)).unwrap(); // d6
            assert_eq!(m.backward(man), backward);
        }
        let behind = evaluate(&position("7k/8/8/4p3/2P5/3P4/8/K7 w - - 0 1"));
        let supported = evaluate(&position("7k/8/8/4p3/8/3P4/2P5/K7 w - - 0 1"));
        assert_eq!(behind.white.pawns, Tapered::new(19, 54));
        assert_eq!(supported.white.pawns, Tapered::new(13, 36));
        // A6's clear, unattacked runway gets64end cp; a rook onb8 controls a8,
        // suppressing that bonus and discounting the122cp base by122/4=30.
        let clear = evaluate(&position("7k/8/P7/8/8/8/8/K7 w - - 0 1"));
        let controlled = evaluate(&position("1r5k/8/P7/8/8/8/8/K7 w - - 0 1"));
        assert_eq!(clear.white.pawns.end - controlled.white.pawns.end, 64 + 30);
        let supported = position("1r5k/8/P7/8/8/8/6B1/6K1 w - - 0 1");
        let f = Features::new(&supported);
        assert_ne!(f.attacks[0] & bit(square(56)), 0); // Bg2 supports a8
        assert_ne!(f.attacks[1] & bit(square(56)), 0); // Rb8 controls a8
        assert_eq!(
            evaluate(&supported).white.pawns.end,
            controlled.white.pawns.end
        );
        // A2 behind A3 is doubled, not a second independently running passer.
        let rear = evaluate(&position("7k/8/8/8/8/P7/P7/K7 w - - 0 1"));
        // doubled-16, isolated-16, onlyA3passed17+escort16 =>1.
        assert_eq!(rear.white.pawns.end, 1);
    }
    #[test]
    fn king_safety_counts_geometric_check_support_and_blocked_pawn_storms() {
        for (fen, expected) in [
            ("7k/8/8/8/8/6p1/8/6K1 w - - 0 1", 12),
            ("7k/8/8/8/8/6p1/6P1/6K1 w - - 0 1", 6),
            ("7k/8/8/6p1/8/8/8/6K1 w - - 0 1", 0),
        ] {
            let p = position(fen);
            let f = Features::new(&p);
            let king = f
                .men
                .iter()
                .find(|m| m.piece.kind == PieceKind::King && m.piece.color == Color::White)
                .unwrap();
            assert_eq!(f.pawn_storm(king), expected);
            let mirror = reflect(&p);
            let reflected = evaluate(&mirror);
            assert_eq!(evaluate(&p).white.king_safety, reflected.black.king_safety);
        }
        // Kf6 supportsQg7; a loneQ without support cannot safely check beside
        // Kh8. Explicit attack-map facts back this heuristic, never mate proof.
        let p = position("7k/8/5KQ1/8/8/8/8/8 w - - 0 1");
        let f = Features::new(&p);
        let g7 = bit(square(54));
        assert_ne!(f.double_attacks[0] & g7, 0);
        assert_eq!(f.nonking_attacks[1] & g7, 0);
        assert_ne!(
            p.attacks_as(
                Piece {
                    color: Color::White,
                    kind: PieceKind::Queen
                },
                square(63)
            ) & g7,
            0
        );
        assert!(evaluate(&p).black.king_safety.middle < -100);
        let mirrored = evaluate(&reflect(&p));
        assert_eq!(evaluate(&p).white, mirrored.black);
        assert_eq!(evaluate(&p).black, mirrored.white);
    }
    #[test]
    fn symmetry_and_no_history_turn_or_outcome_inference() {
        for fen in [
            "6nk/5p2/8/8/8/8/P1B5/KRQ5 w - - 0 1",
            "7k/8/P7/8/8/8/8/K7 w - - 0 1",
            "7k/8/8/8/3K4/8/8/8 b - - 150 999",
            "7k/8/8/8/8/8/1q6/K7 w - - 0 1",
        ] {
            let p = position(fen);
            let old = p.clone();
            let e = evaluate(&p);
            let r = evaluate(&reflect(&p));
            assert_eq!(r.white, e.black);
            assert_eq!(r.black, e.white);
            assert_eq!(r.score_for(Color::White), -e.score_for(Color::White));
            assert_eq!(p, old);
            assert!(e.score_for(Color::White).abs() < 100_000);
        }
        let a = position("7k/8/8/8/8/8/8/KQ6 w - - 0 1");
        let b = position("7k/8/8/8/8/8/8/KQ6 b - - 150 999");
        assert_eq!(evaluate(&a), evaluate(&b));
        let start = position(STANDARD_START_FEN);
        assert_eq!(evaluate(&start).middle_phase, MIDDLE_PHASE);
        assert_eq!(evaluate(&start).score_for(Color::White), 0);
    }
    #[test]
    fn incremental_special_effects_and_full_undo_restore_exact_accumulator() {
        for (fen, actions) in [
            (
                STANDARD_START_FEN,
                vec![
                    "e2e4", "e7e5", "g1f3", "b8c6", "f1b5", "a7a6", "b5c6", "d7c6",
                ],
            ),
            ("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1", vec!["e1g1", "e8c8"]),
            ("7k/8/8/3pP3/8/8/8/KR6 w - d6 0 1", vec!["e5d6"]),
            ("7k/P7/8/8/8/8/8/7K w - - 0 1", vec!["a7a8q"]),
            ("7k/8/8/8/8/8/q7/R6K w - - 0 1", vec!["a1a2"]),
        ] {
            let mut g = AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap();
            let original = g.clone();
            let mut state = EvalState::new(g.position());
            let initial = state.clone();
            for text in actions {
                let before = g.position().clone();
                g.apply(text.parse().unwrap()).unwrap();
                state.push(&before, g.position());
                assert_eq!(state.evaluate(g.position()), evaluate(g.position()));
            }
            while g.undo().is_some() {
                assert!(state.pop());
                assert_eq!(state.evaluate(g.position()), evaluate(g.position()));
            }
            assert_eq!(state, initial);
            assert!(!state.pop());
            assert_eq!(g, original);
        }
    }
    #[test]
    fn fixed_seed_legal_history_branching_rebuild_and_restoration() {
        let mut g = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        let mut state = EvalState::new(g.position());
        let old = g.clone();
        let initial = state.clone();
        let mut seed = 0x7193ab45u64;
        for _ in 0..96 {
            if g.outcome().is_some() {
                break;
            }
            let actions = g.position().legal_actions();
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            let before = g.position().clone();
            g.apply(actions[(seed as usize) % actions.len()]).unwrap();
            state.push(&before, g.position());
            assert_eq!(state.evaluate(g.position()), evaluate(g.position()));
        }
        while g.undo().is_some() {
            assert!(state.pop());
            assert_eq!(state.evaluate(g.position()), evaluate(g.position()));
        }
        assert_eq!(g, old);
        assert_eq!(state, initial);
    }
    #[test]
    fn promotion_above_initial_phase_keeps_uncapped_incremental_state() {
        // Native local diagram: ordinary initial non-pawn material6400;
        // a7xb8=Q replaces a pawn and captures a320cp knight, adding580phase.
        let mut g = AdjudicatedGame::from_fen(
            "rnbqkbnr/Pppppppp/8/8/8/8/1PPPPPPP/RNBQKBNR w KQkq - 0 1",
            ClaimPolicy::ExplicitOnly,
        )
        .unwrap();
        let mut state = EvalState::new(g.position());
        let old = state.clone();
        assert_eq!(state.current.phase, 6400);
        let before = g.position().clone();
        g.apply("a7b8q".parse().unwrap()).unwrap();
        state.push(&before, g.position());
        assert_eq!(state.current.phase, 6980);
        assert_eq!(state.current.material[0] - old.current.material[0], 800);
        assert_eq!(state.current.material[1] - old.current.material[1], -320);
        assert_eq!(state.evaluate(g.position()).middle_phase, MIDDLE_PHASE);
        assert_eq!(state.evaluate(g.position()), evaluate(g.position()));
        g.undo().unwrap();
        assert!(state.pop());
        assert_eq!(state, old);
    }
    #[test]
    fn endgame_conversion_is_preference_not_terminal_recognition() {
        let edge = evaluate(&position("7k/8/8/8/8/8/8/KQ6 w - - 0 1"));
        let center = evaluate(&position("8/8/8/4k3/8/8/8/KQ6 w - - 0 1"));
        // Enemy king oncorner:edge6 =>60; kingdistance7 =>0approach.
        assert_eq!(edge.white.endgame, Tapered::new(0, 60));
        assert!(edge.white.endgame.end > center.white.endgame.end);
        let minor = evaluate(&position("7k/8/8/8/8/8/8/KN6 w - - 0 1"));
        assert_eq!(minor.white.endgame, Tapered::default());
        assert!(
            minor.score_for(Color::White) > 0,
            "native search, not static value, owns dead-position draws"
        );
    }

    #[test]
    fn bare_king_mop_up_does_not_reward_staying_central_over_approach() {
        for major in ["Q", "R"] {
            let central = position(&format!("7k/8/8/8/3K4/8/8/{major}7 w - - 0 1"));
            let near = position(&format!("7k/8/5K2/8/8/8/8/{major}7 w - - 0 1"));
            let a = evaluate(&central);
            let b = evaluate(&near);
            // Both have the same cornered defender. Cancelling generic king
            // centrality leaves the two-distance-step approach worth24end cp.
            assert_eq!(
                b.white.placement.end + b.white.endgame.end
                    - a.white.placement.end
                    - a.white.endgame.end,
                24
            );
            assert!(b.score_for(Color::White) > a.score_for(Color::White));
            assert_eq!(evaluate(&reflect(&near)).black, b.white);
            assert!(
                AdjudicatedGame::from_fen(&near.to_fen(), ClaimPolicy::ExplicitOnly)
                    .unwrap()
                    .outcome()
                    .is_none()
            );
        }
    }

    #[test]
    fn specialized_endings_distinguish_turn_color_and_material_guards() {
        // Independent Syzygy WDL: the same e4 opposition board draws on White's
        // turn and wins for White on Black's. These are advisory finite scores.
        for (fen, kind, zero) in [
            (
                "8/4k3/8/4K3/4P3/8/8/8 w - - 0 1",
                EndgameKind::KingPawnKing,
                true,
            ),
            (
                "8/4k3/8/4K3/4P3/8/8/8 b - - 0 1",
                EndgameKind::KingPawnKing,
                false,
            ),
            (
                "k7/P7/1K6/8/8/8/8/2B5 w - - 0 1",
                EndgameKind::WrongBishopRookPawn,
                true,
            ),
        ] {
            let p = position(fen);
            let before = p.clone();
            let e = evaluate(&p);
            assert_eq!(e.specialized.unwrap().kind, kind);
            assert_eq!(e.score_for(Color::White) == 0, zero);
            assert!(e.score_for(Color::White).abs() < 1000);
            assert_eq!(
                evaluate(&reflect(&p)).score_for(Color::Black),
                e.score_for(Color::White)
            );
            assert_eq!(p, before);
        }
        // Right bishop (Be4#), far defender, additional pawn, non-rook pawn:
        // none may inherit the corner-fortress scale merely from similar material.
        for fen in [
            "k7/P7/1K6/8/8/8/8/1B6 w - - 0 1",
            "7k/P7/1K6/8/8/8/8/2B5 w - - 0 1",
            "k7/P7/1K6/8/8/8/1P6/2B5 w - - 0 1",
            "k7/2P5/1K6/8/8/8/8/2B5 w - - 0 1",
        ] {
            let p = position(fen);
            assert!(evaluate(&p).specialized.is_none(), "{fen}");
            assert!(evaluate(&reflect(&p)).specialized.is_none());
        }
        // Bare king must be driven to bishop-colored corners. Same material,
        // identical king distance; d2 bishop mates in h8, not a8.
        let correct = position("7k/8/8/8/3K4/8/3B4/4N3 w - - 0 1");
        let wrong = position("k7/8/8/8/3K4/8/3B4/4N3 w - - 0 1");
        assert!(
            evaluate(&correct).score_for(Color::White) > evaluate(&wrong).score_for(Color::White)
        );
        assert_eq!(
            evaluate(&correct).specialized.unwrap().kind,
            EndgameKind::BishopKnightConversion
        );
        assert_eq!(
            evaluate(&reflect(&correct)).score_for(Color::Black),
            evaluate(&correct).score_for(Color::White)
        );
    }

    #[test]
    #[ignore = "short manual evaluation profiling; no engine matches or strength inference"]
    fn profile_incremental_evaluation() {
        use std::{hint::black_box, time::Instant};
        let before = position(STANDARD_START_FEN);
        let mut after = before.clone();
        after.apply("e2e4".parse().unwrap()).unwrap();
        let mut state = EvalState::new(&before);
        state.push(&before, &after);
        let begin = Instant::now();
        let mut a = 0;
        for _ in 0..5000 {
            a += black_box(evaluate(black_box(&after))).score_for(Color::White);
        }
        let rebuild = begin.elapsed();
        let begin = Instant::now();
        let mut b = 0;
        for _ in 0..5000 {
            b += black_box(state.evaluate(black_box(&after))).score_for(Color::White);
        }
        let incremental = begin.elapsed();
        assert_eq!(a, b);
        println!(
            "evaluation5000calls rebuild={rebuild:?}, incremental={incremental:?}, checksum={a}; static terms only incremental, dynamicnativegeometry rebuilt"
        );
    }
}
