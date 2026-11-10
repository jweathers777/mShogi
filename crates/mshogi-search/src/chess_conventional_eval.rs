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
/// by444 bounds a color's absolute contribution below30000; their difference is
/// therefore below60000. The larger100000 ceiling leaves conservative slack.
pub const MAX_ABS_STATIC_SCORE: i32 = 100_000;
// Endgame danger is deliberately modest and cannot overwhelm material or the
// native mate band. Its128cp cap is inside the6500cp king-safety bound above.
const MAX_ENDGAME_KING_DANGER: i32 = 128;

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
    /// Shelter/storms and ring/check pressure; participating major pieces retain
    /// bounded danger in low-material positions, independently of king placement.
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
    /// Two knights against a bare king: drawish material, not a dead-position proof.
    TwoKnightsDrawish,
    /// Remote queen-side king versus a protected advanced rook/bishop-file pawn.
    QueenPawnFortress,
    /// Rook/pawn versus rook with an intact third-rank defensive barrier.
    RookPawnThirdRankDefence,
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
/// Passer interception estimates and specialized KPK board knowledge use the
/// turn; clocks, claims and history stay native.
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
                    let bishop_restriction =
                        advance >= 3 && secure_bishop_restriction(&f, man, front);
                    let middle = if bishop_restriction {
                        4
                    } else {
                        4 + 3 * advance * advance
                    };
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
                        passer_runway(position, &f, man, 4 * advance * advance)
                    } else {
                        0
                    };
                    terms[i].pawns.add(Tapered::new(
                        middle,
                        if bishop_restriction {
                            10 + escort
                        } else {
                            (if blocked { end / 2 } else { end }) + escort + runway
                                - if unsafe_path { end / 4 } else { 0 }
                        },
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
                let mut major_participates = false;
                let mut major_contact_checks = 0;
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
                    let major = matches!(enemy.piece.kind, PieceKind::Rook | PieceKind::Queen);
                    major_participates |= major && attacked != 0;
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
                        let checking = origins & enemy.attacks & safe;
                        let count = checking.count_ones().min(2) as i32;
                        if major {
                            major_participates |= checking != 0;
                            // Count supported contacts before the ordinary two-
                            // check cap: a new protected adjacent check can be
                            // masked by already available remote checking moves.
                            major_contact_checks += (checking & man.attacks).count_ones() as i32;
                        }
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
                    if major_participates {
                        // Keep actual attack pressure when total material falls,
                        // without extending middlegame shelter/storm penalties
                        // to active endgame kings. These native geometric maps
                        // are heuristics: legality, pins and mate remain search's.
                        -(pressure + checks / 2 + 24 * major_contact_checks.min(2))
                            .min(MAX_ENDGAME_KING_DANGER)
                    } else {
                        0
                    },
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
    for (i, bonus) in pawn_race_bonus(position, &f).into_iter().enumerate() {
        terms[i].endgame.end += bonus;
    }
    let phase = base.phase.min(MIDDLE_PHASE);
    Evaluation {
        white: terms[0],
        black: terms[1],
        middle_phase: phase,
        specialized: specialized(position, &f, &terms, phase),
    }
}

// Advisory current-board blockade or pressure on an undefended pawn, not an
// endgame result or future-tempo credit. Restrict the discount to opposite
// bishops without other pieces; unsupported king approach or a nonking attack
// releases the guard. An attacked, defended passer keeps its normal premium.
fn secure_bishop_restriction(f: &Features, pawn: &Man, front: Square) -> bool {
    if f.bishops != [1, 1]
        || f.men.iter().any(|m| {
            matches!(
                m.piece.kind,
                PieceKind::Knight | PieceKind::Rook | PieceKind::Queen
            )
        })
    {
        return false;
    }
    let own = side(pawn.piece.color);
    let mut bishops = f.men.iter().filter(|m| m.piece.kind == PieceKind::Bishop);
    let first = bishops.next().unwrap();
    let second = bishops.next().unwrap();
    if (first.sq.file() + first.sq.rank()) % 2 == (second.sq.file() + second.sq.rank()) % 2 {
        return false;
    }
    let defender = if first.piece.color != pawn.piece.color {
        first
    } else {
        second
    };
    let king_defends = distance(f.kings[1 - own], defender.sq) == 1;
    let defended = king_defends || f.pawn_attacks[1 - own] & bit(defender.sq) != 0;
    // A bishop cannot cheaply take a pawn advancing onto a protected square.
    // Promotion differs: trading the bishop for the new queen remains useful.
    // Physical occupation prevents a forward pawn move regardless of support.
    let promotion_front = front.rank()
        == if pawn.piece.color == Color::White {
            7
        } else {
            0
        };
    let controls_front = defender.sq == front
        || (defender.attacks & bit(front) != 0
            && (promotion_front || f.attacks[own] & bit(front) == 0));
    let pressures_undefended_pawn =
        defender.attacks & bit(pawn.sq) != 0 && f.attacks[own] & bit(pawn.sq) == 0;
    (controls_front || pressures_undefended_pawn)
        && f.nonking_attacks[own] & bit(defender.sq) == 0
        // Geometric king attacks do not imply a legal capture of a defended
        // bishop. Other attacks still release this conservative guard.
        && (distance(f.kings[own], defender.sq) > 2 || defended)
}

/// Optimistic promotion time: include one mandatory extra preparatory turn.
/// Strict passing and no friendly pawn farther forward exclude a current
/// capture bypass. King clearance or creating a later capture bypass consumes
/// an extra own turn; the king need not move in every possible continuation.
fn optimistic_promotion_plies(position: &Position, f: &Features, pawn: &Man) -> i32 {
    let rank = relative_rank(pawn.piece, pawn.sq);
    let king = f.kings[side(pawn.piece.color)];
    let strict_passed = !f.men.iter().any(|enemy| {
        enemy.piece.kind == PieceKind::Pawn
            && enemy.piece.color != pawn.piece.color
            && enemy.sq.file().abs_diff(pawn.sq.file()) <= 1
            && relative_rank(pawn.piece, enemy.sq) > rank
    });
    let no_friendly_ahead = !f.men.iter().any(|other| {
        other.piece.kind == PieceKind::Pawn
            && other.piece.color == pawn.piece.color
            && relative_rank(pawn.piece, other.sq) > rank
    });
    let clearance = king.file() == pawn.sq.file()
        && relative_rank(pawn.piece, king) > rank
        && strict_passed
        && no_friendly_ahead;
    2 * i32::from(7 - rank - u8::from(rank == 1))
        - i32::from(position.side_to_move() == pawn.piece.color)
        + 2 * i32::from(clearance)
}

/// Conservative pawn-only promotion forecast; heuristic, never native WDL.
fn pawn_race_bonus(position: &Position, f: &Features) -> [i32; 2] {
    if f.men
        .iter()
        .any(|m| !matches!(m.piece.kind, PieceKind::Pawn | PieceKind::King))
    {
        return [0; 2];
    }
    let tempo = |p: &Man| optimistic_promotion_plies(position, f, p);
    let mut fastest = [i32::MAX; 2];
    for p in f.men.iter().filter(|m| m.piece.kind == PieceKind::Pawn) {
        let i = side(p.piece.color);
        fastest[i] = fastest[i].min(tempo(p));
    }
    let mut result = [0; 2];
    for p in f.men.iter().filter(|m| m.piece.kind == PieceKind::Pawn) {
        let i = side(p.piece.color);
        let rank = relative_rank(p.piece, p.sq);
        // A one-ply lead may allow the opposing queen before ours acts.
        if fastest[1 - i].saturating_sub(tempo(p)) < 3
            || f.men.iter().any(|enemy| {
                enemy.piece.kind == PieceKind::Pawn
                    && enemy.piece.color != p.piece.color
                    && enemy.sq.file().abs_diff(p.sq.file()) <= 1
                    && relative_rank(p.piece, enemy.sq) > rank
            })
        {
            continue;
        }
        let clear = (1..=(7 - rank)).all(|advance| {
            let next_rank = if p.piece.color == Color::White {
                p.sq.rank() + advance
            } else {
                p.sq.rank() - advance
            };
            let sq = Square::new(p.sq.file(), next_rank).unwrap();
            let pushes = advance - u8::from(rank == 1 && advance >= 2);
            let king_moves =
                i32::from(pushes) + i32::from(position.side_to_move() != p.piece.color);
            (f.occupied[0] | f.occupied[1] | f.pawn_attacks[1 - i]) & bit(sq) == 0
                && distance(f.kings[1 - i], sq) > king_moves
        });
        if clear {
            result[i] = 300;
        }
    }
    result
}

/// Only the extra clear-runway premium is discounted. King distance ignores
/// obstructions, opposition and future checks, so this is risk, never a race
/// proof. The bounded walk is used only for advanced, currently clear passers.
fn passer_runway(position: &Position, f: &Features, pawn: &Man, premium: i32) -> i32 {
    let own = side(pawn.piece.color);
    let rear = |color| {
        f.men.iter().any(|man| {
            man.piece.color == color
                && matches!(man.piece.kind, PieceKind::Rook | PieceKind::Queen)
                && man.sq.file() == pawn.sq.file()
                && (if pawn.piece.color == Color::White {
                    man.sq.rank() < pawn.sq.rank()
                } else {
                    man.sq.rank() > pawn.sq.rank()
                })
                && man.attacks & bit(pawn.sq) != 0
        })
    };
    // Today's attack ray stops at the pawn. Rear pressure/support continues as
    // it advances, although the future file is absent from that attack map.
    if rear(pawn.piece.color.opponent()) {
        return premium / 4;
    }
    if rear(pawn.piece.color) {
        return premium;
    }
    let extra_king_move = i32::from(position.side_to_move() != pawn.piece.color);
    let remaining = 7 - relative_rank(pawn.piece, pawn.sq);
    for pushes in 1..=remaining {
        let rank = if pawn.piece.color == Color::White {
            pawn.sq.rank() + pushes
        } else {
            pawn.sq.rank() - pushes
        };
        let landing = Square::new(pawn.sq.file(), rank).unwrap();
        if f.attacks[own] & bit(landing) == 0
            && distance(f.kings[1 - own], landing) <= i32::from(pushes) + extra_king_move
        {
            return premium / 4;
        }
    }
    premium
}

fn specialized(
    position: &Position,
    f: &Features,
    terms: &[Terms; 2],
    phase: i32,
) -> Option<EndgameEvaluation> {
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
    let attenuated = |kind, strong_side| {
        let own = side(strong_side);
        EndgameEvaluation {
            kind,
            strong_side,
            // Retain a small gradient from the original terms, rather than invent
            // a proved draw or erase native tactical/history-dependent outcomes.
            score: (terms[own].contribution(phase) - terms[1 - own].contribution(phase)) / 8,
        }
    };
    if nonkings.len() == 2
        && let Some(queen) = nonkings.iter().find(|m| m.piece.kind == PieceKind::Queen)
        && let Some(pawn) = nonkings.iter().find(|m| m.piece.kind == PieceKind::Pawn)
        && queen.piece.color != pawn.piece.color
    {
        let strong = side(queen.piece.color);
        let weak = 1 - strong;
        let promotion = Square::new(
            pawn.sq.file(),
            if pawn.piece.color == Color::White {
                7
            } else {
                0
            },
        )
        .unwrap();
        if relative_rank(pawn.piece, pawn.sq) == 6
            && matches!(pawn.sq.file(), 0 | 2 | 5 | 7)
            && distance(f.kings[weak], pawn.sq) == 1
            && distance(f.kings[weak], promotion) <= 1
            && distance(f.kings[strong], pawn.sq) >= 4
            && f.attacks[strong] & bit(f.kings[weak]) == 0
            && f.attacks[weak] & (bit(f.kings[strong]) | bit(queen.sq)) == 0
        {
            return Some(attenuated(
                EndgameKind::QueenPawnFortress,
                queen.piece.color,
            ));
        }
    }
    if nonkings.len() == 3
        && let Some(pawn) = nonkings.iter().find(|m| m.piece.kind == PieceKind::Pawn)
        && let Some(own_rook) = nonkings
            .iter()
            .find(|m| m.piece.kind == PieceKind::Rook && m.piece.color == pawn.piece.color)
        && let Some(enemy_rook) = nonkings
            .iter()
            .find(|m| m.piece.kind == PieceKind::Rook && m.piece.color != pawn.piece.color)
    {
        let own = side(pawn.piece.color);
        let weak = 1 - own;
        let promotion = Square::new(
            pawn.sq.file(),
            if pawn.piece.color == Color::White {
                7
            } else {
                0
            },
        )
        .unwrap();
        let barrier = Square::new(
            pawn.sq.file(),
            if pawn.piece.color == Color::White {
                5
            } else {
                2
            },
        )
        .unwrap();
        if relative_rank(pawn.piece, pawn.sq) <= 4
            && relative_rank(pawn.piece, f.kings[own]) <= 4
            && distance(f.kings[weak], promotion) <= 1
            && relative_rank(pawn.piece, enemy_rook.sq) == 5
            && enemy_rook.attacks & bit(barrier) != 0
            && f.attacks[own] & (bit(enemy_rook.sq) | bit(f.kings[weak])) == 0
            && f.attacks[weak] & (bit(own_rook.sq) | bit(f.kings[own])) == 0
        {
            return Some(attenuated(
                EndgameKind::RookPawnThirdRankDefence,
                pawn.piece.color,
            ));
        }
    }
    if nonkings.len() != 2 || nonkings[0].piece.color != nonkings[1].piece.color {
        return None;
    }
    if nonkings.iter().all(|m| m.piece.kind == PieceKind::Knight) {
        return Some(EndgameEvaluation {
            kind: EndgameKind::TwoKnightsDrawish,
            strong_side: nonkings[0].piece.color,
            score: 0,
        });
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
    fn bishop_blockade_discounts_only_advancement_and_is_color_symmetric() {
        // Bd2 controls e1 now; Bc1 does not. Neither bishop is attacked.
        // The black king is four squares from Bd2. Both roots have legal moves.
        let guarded = position("8/8/6k1/7b/1K6/8/P2Bp3/8 b - - 0 1");
        let future = position("8/8/6k1/7b/1K6/8/P3p3/2B5 b - - 0 1");
        assert!(!guarded.legal_actions().is_empty());
        assert!(!future.legal_actions().is_empty());
        let a = evaluate(&guarded);
        let b = evaluate(&future);
        assert_eq!(a.black.material, b.black.material);
        assert_eq!(a.black.material, 430);
        assert_eq!(a.black.placement, b.black.placement);
        // Isolated -8, retained passer base10, escort4*(3-5)=-8.
        assert_eq!(a.black.pawns.end, -6);
        assert!(b.black.pawns.end > a.black.pawns.end + 100);
        let mirrored = evaluate(&reflect(&guarded));
        assert_eq!(a.black.pawns, mirrored.white.pawns);
        assert_eq!(a.white.pawns, mirrored.black.pawns);
        assert_eq!(a.score_for(Color::Black), mirrored.score_for(Color::White));
    }

    #[test]
    fn bishop_blockade_requires_current_secure_opposite_bishop_control() {
        let guarded = "8/8/6k1/7b/1K6/8/P2Bp3/8 b - - 0 1";
        let check = |fen: &str| {
            let p = position(fen);
            let f = Features::new(&p);
            let pawn = f
                .men
                .iter()
                .find(|m| m.sq == Square::new(4, 1).unwrap())
                .unwrap();
            secure_bishop_restriction(&f, pawn, Square::new(4, 0).unwrap())
        };
        assert!(check(guarded));
        for fen in [
            "8/8/6k1/7b/1K6/8/P3p3/2B5 b - - 0 1",  // no current control
            "8/8/6k1/7b/1K6/8/P3p3/8 b - - 0 1",    // bishop removed
            "8/8/6k1/8/1K5b/8/P2Bp3/8 b - - 0 1",   // same color
            "r7/8/6k1/7b/1K6/8/P2Bp3/8 b - - 0 1",  // major present
            "8/8/6k1/7b/1K6/4p3/P2Bp3/8 b - - 0 1", // pawn attacks bishop
            "8/8/8/7b/1K1k4/8/P2Bp3/8 b - - 0 1",   // king close, no defense
        ] {
            assert!(!check(fen), "{fen}");
        }
        // A nearby pawn-side king does not invalidate a king-defended bishop,
        // including when adjacent: capturing it would expose the king.
        assert!(check("8/8/8/7b/4k3/8/P1KBp3/8 b - - 0 1"));
        assert!(check("8/8/8/7b/8/4k3/P1KBp3/8 b - - 0 1"));
    }

    #[test]
    fn bishop_blockade_distinguishes_protected_from_capturable_adjacent_bishop() {
        // Bc4 controls a6. Black d5 protects it: Kc3xc4 is illegal, although
        // the king geometrically attacks c4. Removing d5 makes capture legal.
        let protected = position("7k/7p/8/P2p4/2b5/2K5/8/6B1 w - - 0 1");
        let exposed = position("7k/7p/8/P7/2b5/2K5/8/6B1 w - - 0 1");
        let capture = "c3c4".parse().unwrap();
        assert!(!protected.legal_actions().contains(&capture));
        assert!(exposed.legal_actions().contains(&capture));
        for (p, expected) in [(&protected, true), (&exposed, false)] {
            let f = Features::new(p);
            let pawn = f
                .men
                .iter()
                .find(|m| m.sq == Square::new(0, 4).unwrap())
                .unwrap();
            assert_eq!(
                secure_bishop_restriction(&f, pawn, Square::new(0, 5).unwrap()),
                expected
            );
            let mirror = reflect(p);
            let mf = Features::new(&mirror);
            let mp = mf
                .men
                .iter()
                .find(|m| m.sq == Square::new(0, 3).unwrap())
                .unwrap();
            assert_eq!(
                secure_bishop_restriction(&mf, mp, Square::new(0, 2).unwrap()),
                expected
            );
        }
        assert!(evaluate(&protected).white.pawns.end < evaluate(&exposed).white.pawns.end);
    }

    #[test]
    fn bishop_pressure_requires_present_attack_on_undefended_passer() {
        // Bc1 attacks e3 but neither e2 nor its promotion square e1.
        // Black f4 defends e3 in the second fixture, so pressure is insufficient.
        for (fen, rank, expected) in [
            ("8/8/6k1/7b/1K6/4p3/P7/2B5 b - - 0 1", 2, true),
            ("8/8/6k1/7b/1K3p2/4p3/P7/2B5 b - - 0 1", 2, false),
            ("8/8/6k1/7b/1K6/8/P3p3/2B5 b - - 0 1", 1, false),
        ] {
            let p = position(fen);
            let f = Features::new(&p);
            let pawn = f
                .men
                .iter()
                .find(|m| m.sq == Square::new(4, rank).unwrap())
                .unwrap();
            assert_eq!(
                secure_bishop_restriction(&f, pawn, Square::new(4, rank - 1).unwrap()),
                expected
            );
            let mirror = reflect(&p);
            let mf = Features::new(&mirror);
            let mp = mf
                .men
                .iter()
                .find(|m| m.sq == Square::new(4, 7 - rank).unwrap())
                .unwrap();
            assert_eq!(
                secure_bishop_restriction(&mf, mp, Square::new(4, 8 - rank).unwrap()),
                expected
            );
            assert_eq!(evaluate(&p).black.pawns, evaluate(&mirror).white.pawns);
        }
    }

    #[test]
    fn bishop_pressure_matched_development_continuation_ranking() {
        // Finite reference preference, not a rules/WDL oracle: compare the two
        // legal White replies at the same after-e3 development state. This is
        // a diagnostic static-ranking target, not evidence of a root benefit.
        let fen = "8/8/6k1/P2p3p/2bP1PpP/B1K1p1P1/8/8 w - - 0 49";
        let mut bishop = AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap();
        let mut king = bishop.clone();
        bishop.apply("a3c1".parse().unwrap()).unwrap();
        king.apply("c3c2".parse().unwrap()).unwrap();
        assert!(
            evaluate(bishop.position()).score_for(Color::White)
                > evaluate(king.position()).score_for(Color::White)
        );
    }

    #[test]
    fn bishop_restriction_respects_protected_intermediate_landing() {
        // Bc4 controls a6, but Kb6 protects the advancing pawn. Black d5
        // protects the bishop in both roots so the security guard is held fixed.
        let protected = "7k/7p/1K6/P2p4/2b5/8/8/6B1 w - - 0 1";
        let unprotected = "7k/7p/8/P2p4/2b5/2K5/8/6B1 w - - 0 1";
        // Promotion may trade a pawn for a bishop despite king protection.
        let promotion = "1K5k/P2p4/2b5/8/8/8/8/6B1 w - - 0 1";
        // Ba6 physically blocks a5-a6 even though Kb6 protects a6.
        let occupied = "7k/1p5p/bK6/P7/8/8/8/6B1 w - - 0 1";
        for (fen, rank, expected) in [
            (protected, 4, false),
            (unprotected, 4, true),
            (promotion, 6, true),
            (occupied, 4, true),
        ] {
            let p = position(fen);
            let f = Features::new(&p);
            let pawn = f
                .men
                .iter()
                .find(|m| m.sq == Square::new(0, rank).unwrap())
                .unwrap();
            assert_eq!(
                secure_bishop_restriction(&f, pawn, Square::new(0, rank + 1).unwrap()),
                expected,
                "{fen}"
            );
            let mirror = reflect(&p);
            let mf = Features::new(&mirror);
            let mp = mf
                .men
                .iter()
                .find(|m| m.sq == Square::new(0, 7 - rank).unwrap())
                .unwrap();
            assert_eq!(
                secure_bishop_restriction(&mf, mp, Square::new(0, 6 - rank).unwrap()),
                expected
            );
        }
        let advance = "a5a6".parse().unwrap();
        assert!(position(protected).legal_actions().contains(&advance));
        assert!(!position(occupied).legal_actions().contains(&advance));
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
        // The clear a-file pawn promotes in five own turns with its double
        // push; Kh8 needs seven king steps to a8, so add the300 race premium.
        // This inspects generic terms; the specialized KPK score still overrides.
        let e = evaluate(&position("7k/8/8/8/8/8/P7/K7 w - - 0 1"));
        assert_eq!(e.white.material, 100);
        assert_eq!(e.white.pawns.end, 22);
        assert_eq!(
            e.white.contribution(e.middle_phase) - e.black.contribution(e.middle_phase),
            422
        );
        // A6 pawn:40placement,-8isolated+122passed+4escort+64clear runway,
        // plus the same300 race premium (two own turns, three plies).
        let e = evaluate(&position("7k/8/P7/8/8/8/8/K7 w - - 0 1"));
        assert_eq!(e.white.placement.end, 40);
        assert_eq!(e.white.pawns.end, 182);
        assert_eq!(
            e.white.contribution(e.middle_phase) - e.black.contribution(e.middle_phase),
            622
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
    fn endgame_king_danger_preserves_supported_contact_and_native_defensive_resources() {
        // Independent exhaustive legal replies are preserved under msh149.
        // Both queen boards previously have the same capped middle check count.
        // Qg7 is protected by Kf6; with Kf5, ...Kxg7 is the sole legal reply.
        let supported = position("7k/8/5KQ1/8/8/8/8/8 w - - 0 1");
        let unsupported = position("7k/8/6Q1/5K2/8/8/8/8 w - - 0 1");
        let g7 = Square::new(6, 6).unwrap();
        assert_ne!(Features::new(&supported).double_attacks[0] & bit(g7), 0);
        assert_eq!(Features::new(&unsupported).double_attacks[0] & bit(g7), 0);
        let supported_eval = evaluate(&supported);
        let unsupported_eval = evaluate(&unsupported);
        assert_eq!(supported_eval.black.king_safety.middle, -109);
        assert_eq!(unsupported_eval.black.king_safety.middle, -109);
        // Three queen-attacked ring squares =>15pressure; capped check weight24.
        // The one additional safe contact supplies24end cp, despite the old cap.
        assert_eq!(supported_eval.black.king_safety.end, -51);
        assert_eq!(unsupported_eval.black.king_safety.end, -27);
        assert_eq!(
            supported_eval.black.placement,
            unsupported_eval.black.placement
        );
        assert_eq!(
            supported_eval.white.material,
            unsupported_eval.white.material
        );
        let mut mate = supported.clone();
        mate.apply("g6g7".parse().unwrap()).unwrap();
        assert!(mate.in_check() && !mate.has_legal_action());
        let mut captured = unsupported.clone();
        captured.apply("g6g7".parse().unwrap()).unwrap();
        assert_eq!(captured.legal_actions(), vec!["h8g7".parse().unwrap()]);

        let rook = position("7k/R7/6K1/8/8/8/8/8 w - - 0 1");
        let lone_rook = position("7k/R7/8/5K2/8/8/8/8 w - - 0 1");
        assert_eq!(evaluate(&rook).black.king_safety, Tapered::new(-90, -46));
        assert_eq!(
            evaluate(&lone_rook).black.king_safety,
            Tapered::new(-74, -14)
        );
        // Rh7+ is protected, not mate (...Kg8 survives). Ra8 IS mate in the
        // supported board. The unsupported king can capture Rh7 instead.
        let mut protected_check = rook.clone();
        protected_check.apply("a7h7".parse().unwrap()).unwrap();
        assert_eq!(
            protected_check.legal_actions(),
            vec!["h8g8".parse().unwrap()]
        );
        let mut unprotected_check = lone_rook.clone();
        unprotected_check.apply("a7h7".parse().unwrap()).unwrap();
        assert!(
            unprotected_check
                .legal_actions()
                .contains(&"h8h7".parse().unwrap())
        );
        let mut rook_mate = rook.clone();
        rook_mate.apply("a7a8".parse().unwrap()).unwrap();
        assert!(rook_mate.in_check() && !rook_mate.has_legal_action());

        // Rg8 defends g7 even though Kf6 still supports the checking queen.
        // Other immediate mates Qh6/Qh5 exist; this is NOT a globally safe king.
        let defended = position("6rk/8/5KQ1/8/8/8/8/8 w - - 0 1");
        let f = Features::new(&defended);
        assert_ne!(f.double_attacks[0] & bit(g7), 0);
        assert_ne!(f.nonking_attacks[1] & bit(g7), 0);
        assert_eq!(evaluate(&defended).black.king_safety.end, -27);
        let mut defence = defended.clone();
        defence.apply("g6g7".parse().unwrap()).unwrap();
        defence.apply("g8g7".parse().unwrap()).unwrap();
        for p in [supported, unsupported, rook, lone_rook, defended] {
            let before = p.clone();
            let e = evaluate(&p);
            let reflected = evaluate(&reflect(&p));
            assert_eq!(e.white, reflected.black);
            assert_eq!(e.black, reflected.white);
            assert_eq!(e.score_for(Color::White), reflected.score_for(Color::Black));
            assert_eq!(p, before);
        }
    }

    #[test]
    fn endgame_danger_requires_participation_and_preserves_active_kings_and_exchange() {
        // Qa8 cannot participate through its own a7/b7 pawns and Bb8. Bb8 does
        // attack the d4 king's ring at e5: mere queen presence is insufficient.
        let blocked = position("qb5k/pp6/8/8/3K4/8/8/8 w - - 0 1");
        let f = Features::new(&blocked);
        let queen = f
            .men
            .iter()
            .find(|m| m.piece.kind == PieceKind::Queen)
            .unwrap();
        let king = f
            .men
            .iter()
            .find(|m| m.piece.kind == PieceKind::King && m.piece.color == Color::White)
            .unwrap();
        assert_eq!(queen.attacks & (king.attacks | bit(king.sq)), 0);
        assert_ne!(f.attacks[1] & bit(Square::new(4, 4).unwrap()), 0);
        assert_eq!(evaluate(&blocked).white.king_safety.end, 0);
        let minor = position("7k/8/5n2/8/3K4/8/4P3/8 w - - 0 1");
        assert_eq!(evaluate(&minor).white.king_safety.end, 0);
        // Kd4 centrality84 plus the e2 pawn's central-file placement6.
        assert_eq!(evaluate(&minor).white.placement.end, 90);
        assert_eq!(evaluate(&reflect(&minor)).black.king_safety.end, 0);

        // Legal queen exchange removes both attacks; native bare-king outcome
        // remains independent of the now-zero end-danger term.
        let fen = "7k/7q/5KQ1/8/8/8/8/8 w - - 0 1";
        let mut game = AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly).unwrap();
        let mut incremental = EvalState::new(game.position());
        assert!(evaluate(game.position()).black.king_safety.end < 0);
        for action in ["g6h7", "h8h7"] {
            let before = game.position().clone();
            game.apply(action.parse().unwrap()).unwrap();
            incremental.push(&before, game.position());
            assert_eq!(
                incremental.evaluate(game.position()),
                evaluate(game.position())
            );
        }
        let e = evaluate(game.position());
        assert_eq!(e.white.king_safety.end, 0);
        assert_eq!(e.black.king_safety.end, 0);
        assert!(matches!(
            game.outcome(),
            Some(mshogi_core::chess::ChessOutcome::Draw(_))
        ));
        for _ in 0..2 {
            game.undo().unwrap();
            assert!(incremental.pop());
            assert_eq!(
                incremental.evaluate(game.position()),
                evaluate(game.position())
            );
        }
        assert_eq!(game.position().to_fen(), fen);

        // Nine queens (eight promotions) bound the heuristic even in a legal
        // high-pressure check. No static score is promoted to a mate conclusion.
        let promoted = position("Q6k/Q7/Q1Q1Q3/Q7/Q7/Q7/Q7/K7 b - - 0 1");
        let e = evaluate(&promoted);
        assert!(promoted.in_check());
        assert_eq!(e.black.king_safety.end, -MAX_ENDGAME_KING_DANGER);
        assert!(e.score_for(Color::White).abs() < MAX_ABS_STATIC_SCORE);
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
    fn promotion_clearance_tempo_and_bypass_guards() {
        let time = |fen: &str, file: u8, rank: u8| {
            let p = position(fen);
            let f = Features::new(&p);
            let pawn = f
                .men
                .iter()
                .find(|m| m.sq == Square::new(file, rank).unwrap())
                .unwrap();
            optimistic_promotion_plies(&p, &f, pawn)
        };
        let endpoint = "8/8/8/1K6/1P3kp1/P7/8/8 w - - 1 60";
        // b4-b8 needs four pawn turns plus one mandatory king-clearance turn.
        assert_eq!(time(endpoint, 1, 3), 9);
        assert_eq!(time(endpoint, 0, 2), 9);
        assert_eq!(time(endpoint, 6, 3), 6);
        // bxa5 can bypass Kb5: do not add a clearance turn.
        assert_eq!(time("8/8/8/pK6/1P3kp1/P7/8/8 w - - 1 60", 1, 3), 7);
        // A distant enemy pawn might change file by capturing a forward pawn.
        // Keep its tempo optimistic rather than assuming no future bypass.
        assert_eq!(time("8/5p2/4P3/1K6/1P3kp1/P7/8/8 w - - 1 60", 1, 3), 7);
        // Current cached endpoint and independently probed alternative q leaf:
        // Syzygy categorizes the former White loss and the latter White win.
        // These are evaluation sign/ranking regressions, not native WDL rules.
        let losing = evaluate(&position(endpoint));
        let winning = evaluate(&position("8/8/1K6/Pp4p1/8/P4k2/8/8 w - - 1 60"));
        assert!(losing.score_for(Color::White) < 0);
        assert!(winning.score_for(Color::White) > 0);
        assert!(winning.score_for(Color::White) > losing.score_for(Color::White));
        assert_eq!(losing.white.material, 200);
        assert_eq!(losing.black.material, 100);
        // Several eligible passers still receive at most one bonus per side.
        let many = position("8/8/8/PPP5/8/8/8/K6k w - - 0 1");
        assert_eq!(pawn_race_bonus(&many, &Features::new(&many)), [300, 0]);
    }

    #[test]
    fn pawn_race_forecast_respects_tempo_obstructions_and_material() {
        let race = |fen| {
            let p = position(fen);
            let result = pawn_race_bonus(&p, &Features::new(&p));
            let mirrored = reflect(&p);
            assert_eq!(
                pawn_race_bonus(&mirrored, &Features::new(&mirrored)),
                [result[1], result[0]]
            );
            result
        };
        // Mandatory king clearance makes White's earliest promotion nine plies.
        assert_eq!(race("8/8/8/1K6/1P3kp1/P7/8/8 w - - 1 60"), [0, 300]);
        // Equal push counts only give a one-ply lead, whichever side starts.
        assert_eq!(race("8/8/4k3/P7/7p/3K4/8/8 w - - 0 1"), [0, 0]);
        assert_eq!(race("8/8/4k3/P7/7p/3K4/8/8 b - - 0 1"), [0, 0]);
        // A faster opponent forbids a bonus despite a clear runway.
        assert_eq!(race("8/8/4k3/P7/8/3K3p/8/8 b - - 0 1"), [0, 300]);
        // King interception, occupied runway, non-pawn material, enemy pawn.
        assert_eq!(race("8/3k4/8/P7/8/8/8/7K w - - 0 1"), [0, 0]);
        assert_eq!(race("8/8/K7/P7/8/7k/8/8 w - - 0 1"), [0, 0]);
        assert_eq!(race("8/8/8/P7/8/7k/8/5N1K w - - 0 1"), [0, 0]);
        assert_eq!(race("8/8/1p6/P7/8/7k/8/7K w - - 0 1"), [0, 0]);
        // Initial double push keeps the opposing a2 pawn within one ply of h5.
        assert_eq!(race("8/8/4k3/7p/8/2K5/P7/8 w - - 0 1"), [0, 0]);
        assert_eq!(race("8/8/4k3/7p/8/2K5/P7/8 b - - 0 1"), [0, 300]);
        // After White spends a king move clearing b5, Black leads by three plies.
        assert_eq!(race("8/8/1K6/8/1P3kp1/P7/8/8 b - - 2 60"), [0, 300]);
        let kpk = evaluate(&position("8/8/8/P7/8/7k/8/7K w - - 0 1"));
        assert_eq!(kpk.specialized.unwrap().kind, EndgameKind::KingPawnKing);
    }

    #[test]
    fn passer_interception_distinguishes_tempo_escort_and_rear_support() {
        // Independent Syzygy: only a6 wins on White's turn; Black to move can
        // draw with Kd6/Kd7/Kd8. Ke7 needs four moves to a8, the pawn three
        // pushes. The fourth king move exists only when Black starts.
        let white = position("8/4k2p/8/P7/8/8/8/7K w - - 0 1");
        let black = position("8/4k2p/8/P7/8/8/8/7K b - - 0 1");
        let w = evaluate(&white);
        let b = evaluate(&black);
        assert!(w.specialized.is_none() && b.specialized.is_none());
        assert_eq!(w.white.pawns.end - b.white.pawns.end, 27);
        assert_eq!(w.white.pawns.middle, b.white.pawns.middle);
        assert_eq!(w.white.material, b.white.material);
        assert_eq!(w.black, b.black);
        // Independently winning for White despite Black's first move: Kb7
        // covers every a-file landing; Ra1 supports the pawn from behind.
        for fen in [
            "8/1K2k2p/8/P7/8/8/8/8 b - - 0 1",
            "8/4k2p/8/P7/8/8/8/R6K b - - 0 1",
        ] {
            let p = position(fen);
            let opposite_turn = position(&fen.replace(" b ", " w "));
            assert_eq!(
                evaluate(&p).white.pawns,
                evaluate(&opposite_turn).white.pawns
            );
            assert_eq!(evaluate(&p).white, evaluate(&reflect(&p)).black);
            assert_eq!(evaluate(&p).black, evaluate(&reflect(&p)).white);
        }
        for p in [&white, &black] {
            assert_eq!(evaluate(p).white, evaluate(&reflect(p)).black);
            assert_eq!(evaluate(p).black, evaluate(&reflect(p)).white);
        }
        // An enemy rook's ray currently stops at a5, but follows the pawn as
        // it advances. A rook on b1 has no such rear pressure on this passer.
        let rear = evaluate(&position("7k/8/8/P7/8/8/7K/r7 w - - 0 1"));
        let side = evaluate(&position("7k/8/8/P7/8/8/7K/1r6 w - - 0 1"));
        assert_eq!(side.white.pawns.end - rear.white.pawns.end, 27);
        // An occupied a6 does not earn a runway premium, regardless of tempo.
        let blocked = evaluate(&position("7k/8/N7/P7/8/8/7K/8 w - - 0 1"));
        let clear = evaluate(&position("7k/8/8/P7/8/8/7K/8 w - - 0 1"));
        assert!(blocked.white.pawns.end < clear.white.pawns.end);
        assert_eq!(blocked.white.pawns.middle, clear.white.pawns.middle);
    }

    #[test]
    fn conversion_scales_independent_draws_without_generalizing_to_nearby_wins() {
        // Independent complete-WDL receipts live under docs/m5/evidence/msh145.
        // These finite heuristic scales remain separate from native outcomes.
        for (fen, kind) in [
            (
                "8/8/8/8/8/2K5/NN6/7k w - - 0 1",
                EndgameKind::TwoKnightsDrawish,
            ),
            (
                "7K/8/8/8/3Q4/8/2p5/2k5 w - - 0 1",
                EndgameKind::QueenPawnFortress,
            ),
            (
                "8/4k3/r7/4K3/4P3/8/8/7R w - - 0 1",
                EndgameKind::RookPawnThirdRankDefence,
            ),
            (
                "8/4k3/r7/4K3/4P3/8/8/7R b - - 0 1",
                EndgameKind::RookPawnThirdRankDefence,
            ),
        ] {
            let p = position(fen);
            let before = p.clone();
            let e = evaluate(&p);
            assert_eq!(e.specialized.unwrap().kind, kind);
            let generic =
                e.white.contribution(e.middle_phase) - e.black.contribution(e.middle_phase);
            assert!(e.score_for(Color::White).abs() < generic.abs());
            assert!(e.score_for(Color::White).abs() < 150);
            assert_eq!(
                e.score_for(Color::White),
                evaluate(&reflect(&p)).score_for(Color::Black)
            );
            assert_eq!(e.white, evaluate(&reflect(&p)).black);
            assert_eq!(p, before);
            assert!(
                AdjudicatedGame::from_fen(fen, ClaimPolicy::ExplicitOnly)
                    .unwrap()
                    .outcome()
                    .is_none()
            );
        }
        // Central pawn and near attacking king win; the advanced Lucena
        // position wins. A shifted defending king alone need not lose, but
        // lies outside this deliberately narrow third-rank guard.
        for fen in [
            "7K/8/8/8/3Q4/8/3p4/3k4 w - - 0 1",
            "8/8/8/8/3Q4/3K4/2p5/2k5 w - - 0 1",
            // Independent win, DTZ7, with no immediate mating move: proximity
            // matters even when a one-ply mate search cannot explain the guard.
            "8/8/8/8/3Q1K2/8/2p5/2k5 w - - 0 1",
            "1K6/1P1k4/8/8/8/8/r7/5R2 w - - 0 1",
            "8/6k1/r7/4K3/4P3/8/8/7R w - - 0 1",
            // Undefended or less advanced pawn, extra material, and a hanging
            // defensive rook cannot inherit an exact-material pattern.
            "7K/8/8/8/3Q4/8/k1p5/8 w - - 0 1",
            "7K/8/8/8/3Q4/2p5/2k5/8 w - - 0 1",
            "7K/8/8/8/3Q4/8/2p4P/2k5 w - - 0 1",
            "8/8/8/8/8/2K5/NN5p/7k w - - 0 1",
            "8/4k3/r7/4K3/4P3/8/8/R7 w - - 0 1",
        ] {
            let p = position(fen);
            assert!(evaluate(&p).specialized.is_none(), "{fen}");
            assert!(evaluate(&reflect(&p)).specialized.is_none(), "{fen}");
        }
        let mut hanging = position("8/4k3/r7/4K3/4P3/8/8/R7 w - - 0 1");
        hanging.apply("a1a6".parse().unwrap()).unwrap();
        // The queen is checked/capturable by the defending king. Geometric
        // fortress scaling must not erase this immediate native resource.
        let mut queen = position("7K/8/8/8/8/8/2p5/2kQ4 b - - 0 1");
        assert!(evaluate(&queen).specialized.is_none());
        queen.apply("c1d1".parse().unwrap()).unwrap();
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
