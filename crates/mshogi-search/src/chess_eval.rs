//! Classical Chess evaluation on the search-internal bitboard position.
//!
//! The arithmetic is the conventional tapered evaluation that the earlier
//! native-position evaluator defined: material, placement, pawn structure,
//! mobility, king safety, threats, conversion and a few specialized endings.
//! Material, placement and phase are kept incrementally by the position. The
//! other terms use the current attack geometry. All coefficients are original,
//! transparent, untrained heuristics. A score never establishes a game result.
use crate::chess_bitboard::{BISHOP, ChessPosition, KING, KNIGHT, PAWN, QUEEN, ROOK, VALUE};

/// Identity of this evaluator.
pub const EVALUATOR_ID: &str = "mshogi.chess.conventional-tapered.v1";
/// Initial total non-pawn material; the blend is capped at this phase.
pub const MIDDLE_PHASE: i32 = 6_400;
const MAX_ENDGAME_KING_DANGER: i32 = 128;

/// One term's middle-game and endgame contributions in centipawns.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Tapered {
    /// Contribution at full middle-game phase.
    pub middle: i32,
    /// Contribution at zero non-pawn material.
    pub end: i32,
}
impl Tapered {
    fn add(&mut self, middle: i32, end: i32) {
        self.middle += middle;
        self.end += end;
    }
}

/// One color's contributions.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Terms {
    /// Material: 100/320/330/500/900, king zero.
    pub material: i32,
    /// Piece placement.
    pub placement: Tapered,
    /// Pawn structure and passed pawns.
    pub pawns: Tapered,
    /// Mobility, bishop pair and rook activity.
    pub mobility: Tapered,
    /// Shelter, pawn storms and attack pressure on the king.
    pub king_safety: Tapered,
    /// Attacked undefended or more valuable enemy pieces.
    pub threats: Tapered,
    /// Conversion encouragement with a large material lead.
    pub endgame: Tapered,
}
impl Terms {
    fn contribution(&self, phase: i32) -> i32 {
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

/// Specialized ending that replaces the general terms.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EndgameKind {
    /// King and pawn against king, from the win/draw bitbase.
    KingPawnKing,
    /// Wrong-colored bishop with a rook pawn, defender in the corner.
    WrongBishopRookPawn,
    /// Bishop and knight against a bare king.
    BishopKnightConversion,
    /// Two knights against a bare king.
    TwoKnightsDrawish,
    /// Queen against an advanced rook-file or bishop-file pawn with a remote king.
    QueenPawnFortress,
    /// Rook and pawn against rook with a third-rank defence.
    RookPawnThirdRankDefence,
}

/// Full decomposition for tests and diagnostics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Evaluation {
    /// White's contributions.
    pub white: Terms,
    /// Black's contributions.
    pub black: Terms,
    /// Blend in 0..=6400.
    pub middle_phase: i32,
    /// Specialized ending: kind, strong side (0 White, 1 Black) and its score.
    pub specialized: Option<(EndgameKind, usize, i32)>,
}
impl Evaluation {
    /// Score in centipawns from White.
    pub fn white_score(&self) -> i32 {
        match self.specialized {
            Some((_, strong, score)) => {
                if strong == 0 {
                    score
                } else {
                    -score
                }
            }
            None => {
                self.white.contribution(self.middle_phase)
                    - self.black.contribution(self.middle_phase)
            }
        }
    }
}

#[derive(Clone, Copy, Default)]
struct Man {
    color: usize,
    kind: usize,
    sq: usize,
    attacks: u64,
}

struct Features {
    men: [Man; 32],
    count: usize,
    occupied: [u64; 2],
    attacks: [u64; 2],
    double_attacks: [u64; 2],
    nonking_attacks: [u64; 2],
    pawn_attacks: [u64; 2],
    /// Squares attacked by each piece kind of each color.
    kind_attacks: [[u64; 6]; 2],
    pawn_files: [[u8; 8]; 2],
    pawn_ranks: [[u8; 8]; 2],
    bishops: [u8; 2],
    majors: [u8; 2],
    kings: [usize; 2],
}

#[inline]
fn bit(sq: usize) -> u64 {
    1u64 << sq
}
fn center(sq: usize) -> i32 {
    let (file, rank) = ((sq % 8) as i32, (sq / 8) as i32);
    (14 - (2 * file - 7).abs() - (2 * rank - 7).abs()) / 2
}
fn distance(a: usize, b: usize) -> i32 {
    let files = (a % 8).abs_diff(b % 8);
    let ranks = (a / 8).abs_diff(b / 8);
    files.max(ranks) as i32
}
fn relative_rank(color: usize, sq: usize) -> usize {
    if color == 0 { sq / 8 } else { 7 - sq / 8 }
}

impl Features {
    fn new(p: &ChessPosition) -> Self {
        let mut f = Self {
            men: [Man::default(); 32],
            count: 0,
            occupied: p.occupied,
            attacks: [0; 2],
            double_attacks: [0; 2],
            nonking_attacks: [0; 2],
            pawn_attacks: [0; 2],
            kind_attacks: [[0; 6]; 2],
            pawn_files: [[0; 8]; 2],
            pawn_ranks: [[0; 8]; 2],
            bishops: [0; 2],
            majors: [0; 2],
            kings: [0; 2],
        };
        let mut all = p.all();
        while all != 0 {
            let sq = all.trailing_zeros() as usize;
            all &= all - 1;
            let (color, kind) = p.kind_at(sq).expect("occupied square");
            let attacks = p.attacks_as(color, kind, sq);
            if f.count < 32 {
                f.men[f.count] = Man {
                    color,
                    kind,
                    sq,
                    attacks,
                };
                f.count += 1;
            }
            f.double_attacks[color] |= f.attacks[color] & attacks;
            f.attacks[color] |= attacks;
            f.kind_attacks[color][kind] |= attacks;
            if kind != KING {
                f.nonking_attacks[color] |= attacks;
            }
            match kind {
                PAWN => {
                    f.pawn_files[color][sq % 8] += 1;
                    f.pawn_ranks[color][sq % 8] |= 1 << (sq / 8);
                    f.pawn_attacks[color] |= attacks;
                }
                BISHOP => f.bishops[color] += 1,
                ROOK | QUEEN => f.majors[color] += 1,
                KING => f.kings[color] = sq,
                _ => {}
            }
        }
        f
    }
    fn men(&self) -> &[Man] {
        &self.men[..self.count]
    }
    fn backward(&self, man: &Man) -> bool {
        let i = man.color;
        let (file, rank) = (man.sq % 8, man.sq / 8);
        let behind_or_level = if i == 0 {
            ((1u16 << (rank + 1)) - 1) as u8
        } else {
            (0xffu16 << rank) as u8
        };
        let mut has_neighbor = false;
        let mut rear_support = false;
        for j in file.saturating_sub(1)..=(file + 1).min(7) {
            if j == file {
                continue;
            }
            has_neighbor |= self.pawn_files[i][j] != 0;
            rear_support |= self.pawn_ranks[i][j] & behind_or_level != 0;
        }
        let front = if i == 0 { man.sq + 8 } else { man.sq - 8 };
        has_neighbor && !rear_support && self.pawn_attacks[1 - i] & bit(front) != 0
    }
    fn pawn_storm(&self, king: &Man) -> i32 {
        let i = king.color;
        let mut penalty = 0;
        for pawn in self
            .men()
            .iter()
            .filter(|m| m.kind == PAWN && m.color != king.color)
        {
            if (pawn.sq % 8).abs_diff(king.sq % 8) > 1 {
                continue;
            }
            let forward = if i == 0 {
                (pawn.sq / 8) as i32 - (king.sq / 8) as i32
            } else {
                (king.sq / 8) as i32 - (pawn.sq / 8) as i32
            };
            if !(1..=3).contains(&forward) {
                continue;
            }
            let next = if pawn.color == 0 {
                pawn.sq / 8 + 1
            } else {
                pawn.sq / 8 - 1
            };
            let blocked = self.pawn_ranks[i][pawn.sq % 8] & (1 << next) != 0;
            penalty += (4 - forward) * if blocked { 3 } else { 6 };
        }
        penalty
    }
}

/// Static score in centipawns from the side to move.
pub fn evaluate(p: &ChessPosition) -> i32 {
    let score = decompose(p).white_score();
    if p.side == 0 { score } else { -score }
}

/// Evaluate with the full decomposition.
pub fn decompose(p: &ChessPosition) -> Evaluation {
    let f = Features::new(p);
    let all = p.all();
    let mut terms = [Terms::default(); 2];
    for (i, term) in terms.iter_mut().enumerate() {
        term.material = p.material[i];
        term.placement = Tapered {
            middle: p.placement[i].0,
            end: p.placement[i].1,
        };
        for count in f.pawn_files[i] {
            let extra = i32::from(count.saturating_sub(1));
            term.pawns.add(-12 * extra, -16 * extra);
        }
        if f.bishops[i] >= 2 {
            term.mobility.add(28, 36);
        }
    }
    for man in f.men() {
        let i = man.color;
        let other = 1 - i;
        let file = man.sq % 8;
        let board_rank = man.sq / 8;
        let rank = relative_rank(i, man.sq);
        let advance = (rank as i32 - 1).max(0);
        match man.kind {
            PAWN => {
                let adjacent = file.saturating_sub(1)..=(file + 1).min(7);
                let isolated = (file == 0 || f.pawn_files[i][file - 1] == 0)
                    && (file == 7 || f.pawn_files[i][file + 1] == 0);
                terms[i]
                    .pawns
                    .add(-10 * i32::from(isolated), -8 * i32::from(isolated));
                let near_ranks = ((1u16 << board_rank)
                    | (1u16 << board_rank.saturating_sub(1))
                    | (1u16 << (board_rank + 1).min(7))) as u8;
                let connected = adjacent
                    .clone()
                    .filter(|&j| j != file)
                    .any(|j| f.pawn_ranks[i][j] & near_ranks != 0);
                if connected {
                    terms[i].pawns.add(4 + advance, 6 + 2 * advance);
                }
                let ahead = if i == 0 {
                    (0xffu16 << (board_rank + 1)) as u8
                } else {
                    ((1u16 << board_rank) - 1) as u8
                };
                let passed = adjacent
                    .clone()
                    .all(|j| f.pawn_ranks[other][j] & ahead == 0);
                let front = if i == 0 { man.sq + 8 } else { man.sq - 8 };
                if passed && f.pawn_ranks[i][file] & ahead == 0 {
                    let middle = 4 + 3 * advance * advance;
                    let end = 10 + 7 * advance * advance;
                    let blocked = all & bit(front) != 0;
                    let escort =
                        4 * (distance(f.kings[other], front) - distance(f.kings[i], front));
                    let mut path = 0u64;
                    let mut at = front as isize;
                    let step: isize = if i == 0 { 8 } else { -8 };
                    while (0..64).contains(&at) {
                        path |= bit(at as usize);
                        at += step;
                    }
                    // Defending the destination does not prevent capture: a
                    // contested promotion square stays unsafe.
                    let unsafe_path = path & f.attacks[other] != 0;
                    let clear = path & all == 0;
                    let runway = if advance >= 3 && clear && !unsafe_path {
                        passer_runway(p, &f, man, 4 * advance * advance)
                    } else {
                        0
                    };
                    terms[i].pawns.add(
                        middle,
                        if blocked { end / 2 } else { end } + escort + runway
                            - if unsafe_path { end / 4 } else { 0 },
                    );
                }
                if f.backward(man) {
                    terms[i].pawns.add(-8, -6);
                }
            }
            KNIGHT | BISHOP | ROOK | QUEEN => {
                let mobility =
                    (man.attacks & !f.occupied[i] & !f.pawn_attacks[other]).count_ones() as i32;
                let (middle, end) = match man.kind {
                    KNIGHT => (5, 4),
                    BISHOP => (4, 5),
                    ROOK => (2, 4),
                    _ => (1, 2),
                };
                terms[i].mobility.add(middle * mobility, end * mobility);
                if man.kind == ROOK {
                    if f.pawn_files[i][file] == 0 {
                        if f.pawn_files[other][file] == 0 {
                            terms[i].mobility.add(16, 12);
                        } else {
                            terms[i].mobility.add(8, 6);
                        }
                    }
                    if rank == 6 {
                        terms[i].mobility.add(18, 24);
                    }
                }
            }
            _ => {
                let mut shelter = 0;
                for shelter_file in file.saturating_sub(1)..=(file + 1).min(7) {
                    for (ahead, bonus) in [(1usize, 10), (2, 5)] {
                        let shelter_rank = if i == 0 {
                            Some(board_rank + ahead).filter(|r| *r < 8)
                        } else {
                            board_rank.checked_sub(ahead)
                        };
                        if shelter_rank
                            .is_some_and(|r| f.pawn_ranks[i][shelter_file] & (1 << r) != 0)
                        {
                            shelter += bonus;
                        }
                    }
                    if f.pawn_files[i][shelter_file] == 0 {
                        shelter -= 12 + 8 * i32::from(f.pawn_files[other][shelter_file] == 0);
                    }
                }
                let castled = rank == 0 && matches!(file, 2 | 6);
                let ring = man.attacks | bit(man.sq);
                let mut pressure = 0;
                let mut attackers = 0i32;
                let mut checks = 0;
                let mut major_participates = false;
                let mut major_contact_checks = 0;
                // Squares from which each enemy piece kind would give check.
                let diagonal_origins = p.attacks_as(other, BISHOP, man.sq);
                let straight_origins = p.attacks_as(other, ROOK, man.sq);
                let knight_origins = p.attacks_as(other, KNIGHT, man.sq);
                for enemy in f.men().iter().filter(|m| m.color != i && m.kind != KING) {
                    let attacked = (enemy.attacks & ring).count_ones() as i32;
                    let weight = match enemy.kind {
                        PAWN => 2,
                        KNIGHT => 4,
                        BISHOP | ROOK => 3,
                        _ => 5,
                    };
                    pressure += attacked * weight;
                    attackers += i32::from(attacked != 0);
                    let major = matches!(enemy.kind, ROOK | QUEEN);
                    major_participates |= major && attacked != 0;
                    if enemy.kind != PAWN {
                        // Hypothetical checking origins: reachable, not friendly
                        // occupied, not defended by a non-king, and with a second
                        // supporter when next to the king. Pins and discovered
                        // lines stay the search's work.
                        let origins = match enemy.kind {
                            KNIGHT => knight_origins,
                            BISHOP => diagonal_origins,
                            ROOK => straight_origins,
                            _ => diagonal_origins | straight_origins,
                        };
                        let safe = !f.occupied[other]
                            & !f.nonking_attacks[i]
                            & (!man.attacks | f.double_attacks[other]);
                        let checking = origins & enemy.attacks & safe;
                        let count = checking.count_ones().min(2) as i32;
                        if major {
                            major_participates |= checking != 0;
                            major_contact_checks += (checking & man.attacks).count_ones() as i32;
                        }
                        let check_weight = match enemy.kind {
                            KNIGHT => 14,
                            BISHOP => 12,
                            ROOK => 16,
                            _ => 12,
                        };
                        checks += count * check_weight;
                    }
                }
                terms[i].king_safety.add(
                    shelter + 20 * i32::from(castled)
                        - pressure * (2 + attackers.min(4))
                        - checks
                        - f.pawn_storm(man),
                    if major_participates {
                        -(pressure + checks / 2 + 24 * major_contact_checks.min(2))
                            .min(MAX_ENDGAME_KING_DANGER)
                    } else {
                        0
                    },
                );
            }
        }
        if man.kind != KING && f.attacks[other] & bit(man.sq) != 0 {
            let victim = VALUE[man.kind];
            let undefended = f.attacks[i] & bit(man.sq) == 0;
            // Kinds are in rising value order, and the king is last.
            let least_attacker = (PAWN..=QUEEN)
                .find(|&kind| f.kind_attacks[other][kind] & bit(man.sq) != 0)
                .map_or(20_000, |kind| VALUE[kind]);
            let bonus = if undefended {
                victim / 16
            } else {
                ((victim - least_attacker).max(0) / 32).min(24)
            };
            terms[other].threats.add(bonus, bonus);
        }
    }
    for (i, term) in terms.iter_mut().enumerate() {
        if f.majors[i] != 0 && p.material[i] - p.material[1 - i] >= 400 {
            // A conversion preference, not an assertion that a mate is forced.
            let edge = 6 - center(f.kings[1 - i]);
            let approach = 7 - distance(f.kings[i], f.kings[1 - i]);
            let bare_defender = f.men().iter().all(|man| man.color == i || man.kind == KING);
            term.endgame.end = if bare_defender {
                10 * edge + 12 * approach - 14 * center(f.kings[i])
            } else {
                10 * edge + 4 * approach
            };
        }
    }
    let phase = p.phase.min(MIDDLE_PHASE);
    Evaluation {
        white: terms[0],
        black: terms[1],
        middle_phase: phase,
        specialized: specialized(p, &f, &terms, phase),
    }
}

/// Discount of the clear-runway premium of an advanced passed pawn. King
/// distance ignores obstructions and checks, so this is risk, not a race proof.
fn passer_runway(p: &ChessPosition, f: &Features, pawn: &Man, premium: i32) -> i32 {
    let own = pawn.color;
    let rear = |color: usize| {
        f.men().iter().any(|man| {
            man.color == color
                && matches!(man.kind, ROOK | QUEEN)
                && man.sq % 8 == pawn.sq % 8
                && (if own == 0 {
                    man.sq / 8 < pawn.sq / 8
                } else {
                    man.sq / 8 > pawn.sq / 8
                })
                && man.attacks & bit(pawn.sq) != 0
        })
    };
    if rear(1 - own) {
        return premium / 4;
    }
    if rear(own) {
        return premium;
    }
    let extra_king_move = i32::from(p.side != own);
    let remaining = 7 - relative_rank(own, pawn.sq);
    for pushes in 1..=remaining {
        let landing = if own == 0 {
            pawn.sq + 8 * pushes
        } else {
            pawn.sq - 8 * pushes
        };
        if f.attacks[own] & bit(landing) == 0
            && distance(f.kings[1 - own], landing) <= pushes as i32 + extra_king_move
        {
            return premium / 4;
        }
    }
    premium
}

fn specialized(
    p: &ChessPosition,
    f: &Features,
    terms: &[Terms; 2],
    phase: i32,
) -> Option<(EndgameKind, usize, i32)> {
    if f.count > 5 {
        return None;
    }
    let mut nonkings = [Man::default(); 3];
    let mut n = 0;
    for man in f.men().iter().filter(|m| m.kind != KING) {
        nonkings[n] = *man;
        n += 1;
    }
    let nonkings = &nonkings[..n];
    if let [pawn] = nonkings
        && pawn.kind == PAWN
    {
        if p.has_castling_rights() {
            return None;
        }
        let strong = pawn.color;
        let win = crate::chess_kpk::wins(crate::chess_kpk::index_parts(
            pawn.sq,
            strong == 1,
            f.kings[strong],
            f.kings[1 - strong],
            p.side == strong,
        ));
        // Stay below a rook so a winning underpromotion is not discouraged.
        return Some((
            EndgameKind::KingPawnKing,
            strong,
            if win {
                300 + 24 * relative_rank(strong, pawn.sq) as i32
            } else {
                0
            },
        ));
    }
    let attenuated = |kind, strong: usize| {
        (
            kind,
            strong,
            (terms[strong].contribution(phase) - terms[1 - strong].contribution(phase)) / 8,
        )
    };
    if n == 2
        && let Some(queen) = nonkings.iter().find(|m| m.kind == QUEEN)
        && let Some(pawn) = nonkings.iter().find(|m| m.kind == PAWN)
        && queen.color != pawn.color
    {
        let strong = queen.color;
        let weak = 1 - strong;
        let promotion = pawn.sq % 8 + if pawn.color == 0 { 56 } else { 0 };
        if relative_rank(pawn.color, pawn.sq) == 6
            && matches!(pawn.sq % 8, 0 | 2 | 5 | 7)
            && distance(f.kings[weak], pawn.sq) == 1
            && distance(f.kings[weak], promotion) <= 1
            && distance(f.kings[strong], pawn.sq) >= 4
            && f.attacks[strong] & bit(f.kings[weak]) == 0
            && f.attacks[weak] & (bit(f.kings[strong]) | bit(queen.sq)) == 0
        {
            return Some(attenuated(EndgameKind::QueenPawnFortress, strong));
        }
    }
    if n == 3
        && let Some(pawn) = nonkings.iter().find(|m| m.kind == PAWN)
        && let Some(own_rook) = nonkings
            .iter()
            .find(|m| m.kind == ROOK && m.color == pawn.color)
        && let Some(enemy_rook) = nonkings
            .iter()
            .find(|m| m.kind == ROOK && m.color != pawn.color)
    {
        let own = pawn.color;
        let weak = 1 - own;
        let promotion = pawn.sq % 8 + if own == 0 { 56 } else { 0 };
        let barrier = pawn.sq % 8 + 8 * if own == 0 { 5 } else { 2 };
        if relative_rank(own, pawn.sq) <= 4
            && relative_rank(own, f.kings[own]) <= 4
            && distance(f.kings[weak], promotion) <= 1
            && relative_rank(own, enemy_rook.sq) == 5
            && enemy_rook.attacks & bit(barrier) != 0
            && f.attacks[own] & (bit(enemy_rook.sq) | bit(f.kings[weak])) == 0
            && f.attacks[weak] & (bit(own_rook.sq) | bit(f.kings[own])) == 0
        {
            return Some(attenuated(EndgameKind::RookPawnThirdRankDefence, own));
        }
    }
    if n != 2 || nonkings[0].color != nonkings[1].color {
        return None;
    }
    if nonkings.iter().all(|m| m.kind == KNIGHT) {
        return Some((EndgameKind::TwoKnightsDrawish, nonkings[0].color, 0));
    }
    let bishop = nonkings.iter().find(|m| m.kind == BISHOP)?;
    let other = nonkings.iter().find(|m| m.kind != BISHOP)?;
    let strong = bishop.color;
    let defender = f.kings[1 - strong];
    let bishop_color = (bishop.sq % 8 + bishop.sq / 8) % 2;
    if other.kind == PAWN && matches!(other.sq % 8, 0 | 7) {
        let corner = other.sq % 8 + if strong == 0 { 56 } else { 0 };
        if defender == corner && (corner % 8 + corner / 8) % 2 != bishop_color {
            return Some((EndgameKind::WrongBishopRookPawn, strong, 0));
        }
    }
    if other.kind == KNIGHT {
        let corners = if bishop_color == 0 { [0, 63] } else { [7, 56] };
        let corner_distance = corners
            .into_iter()
            .map(|c| distance(defender, c))
            .min()
            .unwrap_or(0);
        let approach = 7 - distance(f.kings[strong], defender);
        // Not a win certificate: hanging pieces, stalemate and the fifty-move
        // rule are the search's and the native rules' work.
        return Some((
            EndgameKind::BishopKnightConversion,
            strong,
            650 + 24 * (7 - corner_distance) + 12 * approach,
        ));
    }
    None
}

#[cfg(test)]
mod tests;
