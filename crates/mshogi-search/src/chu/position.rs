//! Private Chu board and complete routes for the shared search core.
//! Native movement descriptors are geometry data; generation, projection,
//! Lion protection and restoration below are independent search operations.
use crate::alphabeta::{MATE_SCORE, SearchPosition, Stage};
use crate::repetition::{cycle_start, earlier_occurrences};
use mshogi_core::chu::{
    Action, Base, Game, GameCommand, LionProtection, LocalPosition, Movement, Piece, Position,
    Side, SpecialMovement, Square, movement,
};
use std::sync::LazyLock;
const N: usize = 144;
const STATES: usize = 42;
const KING_STEPS: &[(i8, i8)] = &[
    (0, -1),
    (1, 0),
    (0, 1),
    (-1, 0),
    (-1, -1),
    (1, -1),
    (1, 1),
    (-1, 1),
];
fn side(s: usize) -> Side {
    if s == 0 { Side::Sente } else { Side::Gote }
}
fn square(at: usize) -> Square {
    Square::new((12 - at % 12) as u8, (at / 12 + 1) as u8).unwrap()
}
fn offset(at: usize, s: usize, df: i8, dr: i8) -> Option<usize> {
    let sign = if s == 0 { 1 } else { -1 };
    let col = (at % 12) as i16 - i16::from(df) * sign;
    let row = (at / 12) as i16 + i16::from(dr) * sign;
    ((0..12).contains(&col) && (0..12).contains(&row)).then_some((row * 12 + col) as usize)
}
fn distance(a: usize, b: usize) -> usize {
    (a % 12).abs_diff(b % 12).max((a / 12).abs_diff(b / 12))
}
fn piece_code(p: Piece) -> u8 {
    (1 + p.side.index() * STATES
        + Base::ALL.iter().position(|&b| b == p.base).unwrap() * 2
        + usize::from(p.promoted)) as u8
}
type Set = crate::square_set::SquareSet<3>;
#[derive(Clone, Copy)]
struct Meta {
    piece: Piece,
    movement: Movement,
    value: i32,
    royal: bool,
    lion: bool,
    qualifies: bool,
}
struct Tables {
    meta: Vec<Meta>,
    pieces: Vec<[u64; N]>,
    protection: [[u64; N]; 2],
    turn: u64,
}
static TABLES: LazyLock<Tables> = LazyLock::new(|| {
    let mut meta = Vec::with_capacity(85);
    let mut pieces = vec![[0; N]; 85];
    for c in 0usize..=84 {
        let i = c.saturating_sub(1);
        let owner = i / STATES;
        let base = Base::ALL[(i % STATES) / 2];
        let p = Piece::new(side(owner), base, i % 2 == 1).unwrap_or(Piece {
            side: side(owner),
            base,
            promoted: false,
        });
        meta.push(Meta {
            piece: p,
            movement: movement(p.face()),
            value: super::piece_value(p.face()),
            royal: p.is_royal(),
            lion: p.is_lion(),
            qualifies: p.promoted || !matches!(p.base, Base::Pawn | Base::GoBetween),
        });
    }
    let mut seed = 0xc817_5eed_0000_0182u64;
    let mut next = || {
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
        let mut z = seed;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^ (z >> 31)
    };
    for row in pieces.iter_mut().skip(1) {
        for key in row {
            *key = next();
        }
    }
    let mut protection = [[0; N]; 2];
    for row in &mut protection {
        for key in row {
            *key = next();
        }
    }
    Tables {
        meta,
        pieces,
        protection,
        turn: next(),
    }
});
fn meta(c: u8) -> &'static Meta {
    &TABLES.meta[usize::from(c)]
}

/// Eight bits each for source, first landing and second landing, followed by
/// promotion. Second landing 255 means a one-landing action.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Move(u32);
impl Move {
    fn single(from: usize, to: usize) -> Self {
        Self(from as u32 | ((to as u32) << 8) | (255 << 16))
    }
    fn double(from: usize, mid: usize, to: usize) -> Self {
        Self(from as u32 | ((mid as u32) << 8) | ((to as u32) << 16))
    }
    fn from(self) -> usize {
        (self.0 & 255) as usize
    }
    fn first(self) -> usize {
        ((self.0 >> 8) & 255) as usize
    }
    fn second(self) -> usize {
        ((self.0 >> 16) & 255) as usize
    }
    fn is_double(self) -> bool {
        self.second() != 255
    }
    fn to(self) -> usize {
        if self.is_double() {
            self.second()
        } else {
            self.first()
        }
    }
    fn promotes(self) -> bool {
        self.0 & (1 << 24) != 0
    }
    fn promoted(self) -> Self {
        Self(self.0 | (1 << 24))
    }
}
#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
struct Protection {
    squares: Set,
    owner: usize,
}
#[derive(Clone, Copy)]
struct Undo {
    m: Move,
    moved: u8,
    captures: [(usize, u8); 2],
    count: usize,
    protection: Protection,
    hash: u64,
    reversible: usize,
    check: bool,
}
/// Fixed-size state: scratch projections never copy or allocate game history.
#[derive(Clone, Copy)]
struct Board {
    cells: [u8; N],
    occupied: [Set; 2],
    royals: [Set; 2],
    lions: [Set; 2],
    side: usize,
    protection: Protection,
    hash: u64,
    material: [i32; 2],
    placement: [i32; 2],
    counted: [u8; 2],
    unqualified: [u8; 2],
}
struct ExchangeWork {
    projections: usize,
    routes: usize,
    geometry: usize,
}
impl Board {
    fn empty(turn: usize) -> Self {
        Self {
            cells: [0; N],
            occupied: [Set::default(); 2],
            royals: [Set::default(); 2],
            lions: [Set::default(); 2],
            side: turn,
            protection: Protection::default(),
            hash: if turn == 1 { TABLES.turn } else { 0 },
            material: [0; 2],
            placement: [0; 2],
            counted: [0; 2],
            unqualified: [0; 2],
        }
    }
    fn dead(c: u8, at: usize) -> bool {
        let p = meta(c).piece;
        !p.promoted
            && matches!(p.base, Base::Pawn | Base::Lance)
            && if p.side == Side::Sente {
                at / 12 == 0
            } else {
                at / 12 == 11
            }
    }
    // C08 only: no general bare-royal or insufficient-material theorem.
    fn material_winner(&self) -> Option<usize> {
        if u16::from(self.counted[0]) + u16::from(self.counted[1]) != 3 {
            return None;
        }
        for bare in 0..2 {
            let stronger = bare ^ 1;
            if self.counted[bare] != 1
                || self.royals[bare].count() != 1
                || self.counted[stronger] != 2
                || self.royals[stronger].count() == 0
                || self.unqualified[stronger] != 0
            {
                continue;
            }
            if self.side == bare {
                let mut captures = Vec::new();
                self.generate(Stage::Tactical, &mut captures);
                for m in captures {
                    let mut after = *self;
                    if after.make(m).is_some()
                        && (after.royals[stronger].count() == 0
                            || (u16::from(after.counted[0]) + u16::from(after.counted[1]) == 2
                                && after.royals[0].count() + after.royals[1].count() == 2))
                    {
                        return None;
                    }
                }
            }
            return Some(stronger);
        }
        None
    }
    fn exchange_gain(&self, m: Move) -> i32 {
        let (captures, n) = self.captures(m);
        let c = self.cells[m.from()];
        captures[..n]
            .iter()
            .map(|&(_, c)| meta(c).value)
            .sum::<i32>()
            + if m.promotes() {
                meta(c + 1).value - meta(c).value
            } else {
                0
            }
    }

    // Actual royal captures and all possible C08 material terminals are outside
    // this material model. This constant-cost overestimate avoids terminal move
    // generation inside the bounded exchange calculation.
    fn exchange_uncertain(&self) -> bool {
        self.royals.iter().any(|r| r.count() == 0)
            || u16::from(self.counted[0]) + u16::from(self.counted[1]) <= 3
    }

    // Exact threshold minimax within the local material-exchange model. None
    // means unresolved, never proof that the initial capture should be pruned.
    fn exchange_greater(
        &self,
        target: usize,
        limit: i32,
        depth: u8,
        work: &mut ExchangeWork,
    ) -> Option<bool> {
        if self.exchange_uncertain() {
            return None;
        }
        if limit < 0 {
            return Some(true);
        } // decline for zero material gain
        if depth == 8 {
            return None;
        }
        let mut pieces = self.occupied[self.side];
        let mut moves = Vec::new();
        while let Some(from) = pieces.pop() {
            if work.geometry == 0 {
                return None;
            }
            work.geometry -= 1;
            if !self.direct_reaches(self.cells[from], from, target) {
                continue;
            }
            // A single piece has at most 416 routes on this fixed board. Reserve
            // that bound before generation, so a limit cannot hide extra work.
            // At most 144 distinct direct squares + 8*8 compound routes,
            // each with at most two promotion choices: (144 + 64)*2.
            if work.routes < 416 {
                return None;
            }
            work.routes -= 416;
            let start = moves.len();
            self.generate_from(from, Stage::Tactical, &mut moves);
            debug_assert!(moves.len() - start <= 416);
            work.routes += 416 - (moves.len() - start);
            let generated = moves.split_off(start);
            moves.extend(generated.into_iter().filter(|&m| {
                let (victims, n) = self.captures(m);
                victims[..n].iter().any(|&(at, _)| at == target)
            }));
        }
        moves.sort_unstable_by_key(|&m| (meta(self.cells[m.from()]).value, -self.exchange_gain(m)));
        let mut seen: Vec<(Board, usize)> = Vec::new();
        let mut unknown = false;
        for m in moves {
            if work.projections == 0 {
                return None;
            }
            work.projections -= 1;
            let mut after = *self;
            if after.make(m).is_none() {
                continue;
            }
            if seen.iter().any(|(b, t)| {
                *t == m.to()
                    && b.cells == after.cells
                    && b.side == after.side
                    && b.protection == after.protection
            }) {
                continue;
            }
            seen.push((after, m.to()));
            match after.exchange_greater(m.to(), self.exchange_gain(m) - limit - 1, depth + 1, work)
            {
                Some(false) => return Some(true),
                Some(true) => {}
                None => unknown = true,
            }
        }
        if unknown { None } else { Some(false) }
    }
    fn placement(c: u8, at: usize) -> i32 {
        super::eval::placement(meta(c).piece, square(at))
    }

    fn put(&mut self, c: u8, at: usize) {
        let m = meta(c);
        let s = m.piece.side.index();
        self.cells[at] = c;
        self.occupied[s].add(at);
        if m.royal {
            self.royals[s].add(at);
        }
        if m.lion {
            self.lions[s].add(at);
        }
        self.hash ^= TABLES.pieces[usize::from(c)][at];
        self.material[s] += m.value;
        self.placement[s] += Self::placement(c, at);
        if !Self::dead(c, at) {
            self.counted[s] += 1;
            self.unqualified[s] += u8::from(!m.qualifies);
        }
    }
    fn take(&mut self, at: usize) -> u8 {
        let c = self.cells[at];
        if c == 0 {
            return 0;
        }
        let m = meta(c);
        let s = m.piece.side.index();
        self.cells[at] = 0;
        self.occupied[s].remove(at);
        if m.royal {
            self.royals[s].remove(at);
        }
        if m.lion {
            self.lions[s].remove(at);
        }
        self.hash ^= TABLES.pieces[usize::from(c)][at];
        self.material[s] -= m.value;
        self.placement[s] -= Self::placement(c, at);
        if !Self::dead(c, at) {
            self.counted[s] -= 1;
            self.unqualified[s] -= u8::from(!m.qualifies);
        }
        c
    }
    fn set_protection(&mut self, p: Protection) {
        for old in [self.protection, p] {
            let mut squares = old.squares;
            while let Some(at) = squares.pop() {
                self.hash ^= TABLES.protection[old.owner][at];
            }
        }
        self.protection = p;
    }
    fn direct_reaches(&self, c: u8, from: usize, to: usize) -> bool {
        if from == to {
            return false;
        }
        let movement = meta(c).movement;
        let sign = if meta(c).piece.side == Side::Sente {
            1
        } else {
            -1
        };
        let df = ((from % 12) as i8 - (to % 12) as i8) * sign;
        let dr = ((to / 12) as i8 - (from / 12) as i8) * sign;
        if movement.steps.contains(&(df, dr)) || movement.jumps.contains(&(df, dr)) {
            return true;
        }
        if match movement.special {
            Some(SpecialMovement::Lion) => df.abs() <= 2 && dr.abs() <= 2,
            Some(SpecialMovement::Forward) => df == 0 && (-2..=-1).contains(&dr),
            Some(SpecialMovement::ForwardDiagonal) => (-2..=-1).contains(&dr) && df.abs() == -dr,
            None => false,
        } {
            return true;
        }
        // Only the ray towards the target can reach it. Check its direction
        // before looking at occupancy; the target itself may be a friendly
        // piece whose defense is being tested.
        if (df != 0 && dr != 0 && df.abs() != dr.abs())
            || !movement.rays.contains(&(df.signum(), dr.signum()))
        {
            return false;
        }
        let step = isize::from(dr.signum() * sign) * 12 - isize::from(df.signum() * sign);
        let mut at = from as isize + step;
        while at != to as isize {
            if self.cells[at as usize] != 0 {
                return false;
            }
            at += step;
        }
        true
    }
    fn defended(&self, owner: usize, target: usize) -> bool {
        let mut sources = self.occupied[owner];
        sources.remove(target);
        while let Some(from) = sources.pop() {
            if self.direct_reaches(self.cells[from], from, target) {
                return true;
            }
        }
        false
    }
    fn captures(&self, m: Move) -> ([(usize, u8); 2], usize) {
        let mut victims = [(0, 0); 2];
        let mut count = 0;
        for at in [m.first(), if m.is_double() { m.to() } else { m.from() }] {
            if at != m.from() && self.cells[at] != 0 {
                victims[count] = (at, self.cells[at]);
                count += 1;
            }
        }
        (victims, count)
    }
    fn promotion_allowed(&self, c: u8, m: Move) -> bool {
        let p = meta(c).piece;
        if p.promoted || !p.base.promotable() {
            return false;
        }
        let from_zone = p.side.in_zone(square(m.from()).rank());
        let to_zone = p.side.in_zone(square(m.to()).rank());
        (!from_zone && to_zone)
            || (self.captures(m).1 > 0 && (from_zone || to_zone))
            || (p.base == Base::Pawn && p.side.last_rank(square(m.to()).rank()))
    }
    fn pseudo(&self, m: Move) -> bool {
        let (from, first, to) = (m.from(), m.first(), m.to());
        if from >= N || first >= N || to >= N || m.0 >> 25 != 0 || from == first {
            return false;
        }
        let c = self.cells[from];
        if c == 0 || meta(c).piece.side.index() != self.side {
            return false;
        }
        if [first, to].into_iter().any(|at| {
            at != from
                && self.cells[at] != 0
                && meta(self.cells[at]).piece.side.index() == self.side
        }) {
            return false;
        }
        let s = self.side;
        let geometry = if !m.is_double() {
            self.direct_reaches(c, from, to)
        } else {
            match meta(c).movement.special {
                Some(SpecialMovement::Lion) => {
                    distance(from, first) == 1 && distance(first, to) == 1
                }
                Some(SpecialMovement::Forward) => {
                    offset(from, s, 0, -1) == Some(first)
                        && (to == from || offset(first, s, 0, -1) == Some(to))
                }
                Some(SpecialMovement::ForwardDiagonal) => [-1, 1].into_iter().any(|f| {
                    offset(from, s, f, -1) == Some(first)
                        && (to == from || offset(first, s, f, -1) == Some(to))
                }),
                None => false,
            }
        };
        geometry && (!m.promotes() || self.promotion_allowed(c, m))
    }
    fn append(&self, m: Move, stage: Stage, moves: &mut Vec<Move>) {
        // Geometry is already established by generation; test actual landings.
        if [m.first(), m.to()].into_iter().any(|at| {
            at != m.from()
                && self.cells[at] != 0
                && meta(self.cells[at]).piece.side.index() == self.side
        }) {
            return;
        }
        let capture = self.captures(m).1 != 0;
        if capture == (stage == Stage::Tactical) {
            moves.push(m);
        }
        if stage == Stage::Tactical && self.promotion_allowed(self.cells[m.from()], m) {
            moves.push(m.promoted());
        }
    }
    fn generate_from(&self, from: usize, stage: Stage, moves: &mut Vec<Move>) {
        let c = self.cells[from];
        let data = meta(c);
        let s = data.piece.side.index();
        let mut direct = Set::default();
        for &(f, r) in data.movement.steps.iter().chain(data.movement.jumps) {
            if let Some(to) = offset(from, s, f, r) {
                direct.add(to);
            }
        }
        for &(f, r) in data.movement.rays {
            let mut at = from;
            while let Some(to) = offset(at, s, f, r) {
                direct.add(to);
                if self.cells[to] != 0 {
                    break;
                }
                at = to;
            }
        }
        match data.movement.special {
            Some(SpecialMovement::Lion) => {
                for f in -2..=2 {
                    for r in -2..=2 {
                        if (f != 0 || r != 0)
                            && let Some(to) = offset(from, s, f, r)
                        {
                            direct.add(to);
                        }
                    }
                }
                for &(f, r) in KING_STEPS {
                    if let Some(mid) = offset(from, s, f, r) {
                        for &(f2, r2) in KING_STEPS {
                            if let Some(to) = offset(mid, s, f2, r2) {
                                self.append(Move::double(from, mid, to), stage, moves);
                            }
                        }
                    }
                }
            }
            Some(special) => {
                let dirs: &[(i8, i8)] = if special == SpecialMovement::Forward {
                    &[(0, -1)]
                } else {
                    &[(-1, -1), (1, -1)]
                };
                for &(f, r) in dirs {
                    if let Some(mid) = offset(from, s, f, r) {
                        direct.add(mid);
                        self.append(Move::double(from, mid, from), stage, moves);
                        if let Some(to) = offset(mid, s, f, r) {
                            direct.add(to);
                            self.append(Move::double(from, mid, to), stage, moves);
                        }
                    }
                }
            }
            None => {}
        }
        while let Some(to) = direct.pop() {
            self.append(Move::single(from, to), stage, moves);
        }
    }
    fn generate(&self, stage: Stage, moves: &mut Vec<Move>) {
        let mut pieces = self.occupied[self.side];
        while let Some(from) = pieces.pop() {
            self.generate_from(from, stage, moves);
        }
    }
    /// Complete board projection. Returns the small reversal record; a refusal
    /// restores all fixed state and never consumes the active Lion window.
    fn make(&mut self, m: Move) -> Option<Undo> {
        if !self.pseudo(m) {
            return None;
        }
        let moved = self.cells[m.from()];
        let (captures, count) = self.captures(m);
        let undo = Undo {
            m,
            moved,
            captures,
            count,
            protection: self.protection,
            hash: self.hash,
            reversible: 0,
            check: false,
        };
        let attacker = meta(moved);
        let owner = self.side;
        self.take(m.from());
        // The original defender survives for this test, even if a first-leg
        // Pawn is about to disappear; vacating the source uncovers sliders.
        let mut initially_defended = [false; 2];
        for i in 0..count {
            let (at, v) = captures[i];
            if attacker.lion && meta(v).lion && distance(m.from(), at) > 1 {
                initially_defended[i] = self.defended(owner ^ 1, at);
            }
        }
        for &(at, _) in &captures[..count] {
            self.take(at);
        }
        self.put(if m.promotes() { moved + 1 } else { moved }, m.to());
        for i in 0..count {
            let (at, v) = captures[i];
            if !meta(v).lion {
                continue;
            }
            let qualifying = captures[..i].iter().any(|&(_, c)| meta(c).qualifies);
            let protected =
                undo.protection.owner == owner ^ 1 && undo.protection.squares.contains(at);
            if !qualifying
                && (protected
                    || (attacker.lion
                        && distance(m.from(), at) > 1
                        && (initially_defended[i] || self.defended(owner ^ 1, at))))
            {
                self.restore_board(undo);
                return None;
            }
        }
        let mut protection = Protection::default();
        if !attacker.lion && captures[..count].iter().any(|&(_, c)| meta(c).lion) {
            protection.owner = owner;
            let mut lions = self.lions[owner];
            while let Some(at) = lions.pop() {
                if self.defended(owner, at) {
                    protection.squares.add(at);
                }
            }
        }
        self.set_protection(protection);
        self.side ^= 1;
        self.hash ^= TABLES.turn;
        Some(undo)
    }
    fn restore_board(&mut self, undo: Undo) {
        self.take(undo.m.to());
        self.put(undo.moved, undo.m.from());
        for &(at, c) in &undo.captures[..undo.count] {
            self.put(c, at);
        }
        self.protection = undo.protection;
        self.hash = undo.hash;
    }
    fn unmake(&mut self, undo: Undo) {
        self.side ^= 1;
        self.restore_board(undo);
    }
    /// An admitted one-action capture of all surviving royals. This is a search
    /// threat, not a Chess self-check rule or an adjudicated native mate.
    fn threatened(&self, victim: usize) -> bool {
        let mut victims = self.royals[victim];
        let count = victims.count();
        if count == 0 || count > 2 {
            return false;
        }
        if count == 1 {
            let target = victims.pop().expect("one royal");
            let mut attackers = self.occupied[victim ^ 1];
            while let Some(from) = attackers.pop() {
                // A royal is never a protected Lion. Every direct reach has
                // a one-landing capture, and Chu has no self-check filter.
                if self.direct_reaches(self.cells[from], from, target) {
                    return true;
                }
            }
            return false;
        }
        let mut scratch = *self;
        scratch.side = victim ^ 1;
        // Active protection constrains its owner's opponent, not its owner.
        if scratch.protection.owner == scratch.side {
            scratch.set_protection(Protection::default());
        }
        let mut pieces = scratch.occupied[scratch.side];
        let mut moves = Vec::new();
        while let Some(from) = pieces.pop() {
            let c = scratch.cells[from];
            let mut targets = victims;
            let mut reaches = true;
            while let Some(to) = targets.pop() {
                if !scratch.direct_reaches(c, from, to) {
                    reaches = false;
                    break;
                }
            }
            if !reaches {
                continue;
            }
            moves.clear();
            scratch.generate_from(from, Stage::Tactical, &mut moves);
            for &m in &moves {
                let (captures, n) = scratch.captures(m);
                if captures[..n]
                    .iter()
                    .filter(|&&(at, _)| victims.contains(at))
                    .count()
                    != count as usize
                {
                    continue;
                }
                let mut after = scratch;
                if after.make(m).is_some() {
                    return true;
                }
            }
        }
        false
    }
}
/// Search-internal state; only compact keys persist across the known game.
#[derive(Clone)]
pub struct ChuPosition {
    board: Board,
    stack: Vec<Undo>,
    keys: Vec<u64>,
    root: usize,
    reversible: usize,
    check: bool,
    root_allowed: Option<Vec<Move>>,
}
impl ChuPosition {
    /// Copy a diagram with no Lion protection or earlier history.
    pub fn from_position(position: &Position) -> Self {
        Self::from_local(&LocalPosition::admit(position.clone()))
    }
    /// Copy a native local board and its transient Lion rights.
    pub fn from_local(local: &LocalPosition) -> Self {
        let native = local.position();
        let mut board = Board::empty(native.side_to_move().index());
        for (at, p) in native.pieces() {
            board.put(piece_code(p), at.index());
        }
        let mut protection = Protection::default();
        if let Some(owner) = local.protection().owner {
            protection.owner = owner.index();
            for at in local.protection().protected_squares() {
                protection.squares.add(at.index());
            }
        }
        board.set_protection(protection);
        let check = board.threatened(board.side);
        let hash = board.hash;
        Self {
            board,
            stack: Vec::with_capacity(160),
            keys: vec![hash],
            root: 0,
            reversible: 0,
            check,
            root_allowed: None,
        }
    }
    /// Replay only board events once. The caller must install the native root
    /// legal actions with `restrict_root` before a search of a native game.
    pub fn from_game(game: &Game) -> Self {
        let root = game.event_prefix(0).expect("native root").local_position();
        let mut p = Self::from_local(&root);
        for event in game.events() {
            if let GameCommand::Play { action } = &event.command {
                let m = p.find(action).expect("native action encodes");
                assert!(p.make(m), "native action is legal");
            }
        }
        p.stack.clear();
        p.root = p.keys.len() - 1;
        p
    }
    /// Restrict the root to the native must-vary admission list.
    pub fn restrict_root(&mut self, actions: &[Action]) {
        self.root_allowed = Some(
            actions
                .iter()
                .map(|a| self.find(a).expect("native legal root action"))
                .collect(),
        );
    }
    /// Convert the entire chosen route, including optional promotion.
    pub fn to_native(&self, m: Move) -> Action {
        Action {
            from: square(m.from()),
            route: if m.is_double() {
                vec![square(m.first()), square(m.to())]
            } else {
                vec![square(m.first())]
            },
            promote: m.promotes(),
        }
    }
    /// Encode a complete pseudo-legal native action.
    pub fn find(&self, action: &Action) -> Option<Move> {
        let m = match action.route.as_slice() {
            [to] => Move::single(action.from.index(), to.index()),
            [mid, to] => Move::double(action.from.index(), mid.index(), to.index()),
            _ => return None,
        };
        let m = if action.promote { m.promoted() } else { m };
        self.is_pseudo_legal(m).then_some(m)
    }
    /// All legal local actions (or the native restricted root when installed).
    pub fn legal_moves(&mut self) -> Vec<Move> {
        let mut moves = Vec::new();
        self.generate(Stage::Tactical, &mut moves);
        self.generate(Stage::Quiet, &mut moves);
        moves.retain(|&m| {
            let ok = self.make(m);
            if ok {
                self.unmake();
            }
            ok
        });
        moves
    }
    /// Complete local-route leaf count; no game adjudication truncates it.
    pub fn perft(&mut self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let moves = self.legal_moves();
        let mut total = 0;
        for m in moves {
            assert!(self.make(m));
            total += self.perft(depth - 1);
            self.unmake();
        }
        total
    }
    /// Native side to move.
    pub fn side_to_move(&self) -> Side {
        side(self.board.side)
    }
    /// Original identity and promotion of an occupied square.
    pub fn piece(&self, at: Square) -> Option<Piece> {
        let c = self.board.cells[at.index()];
        (c != 0).then(|| meta(c).piece)
    }
    /// Native form of the transient Lion right.
    pub fn protection(&self) -> LionProtection {
        let mut set = self.board.protection.squares;
        let mut squares = Vec::new();
        while let Some(at) = set.pop() {
            squares.push(square(at));
        }
        LionProtection {
            owner: (!squares.is_empty()).then_some(side(self.board.protection.owner)),
            squares,
        }
    }
    /// Independent full-state hash recomputation.
    pub fn hash_from_scratch(&self) -> u64 {
        let mut hash = if self.board.side == 1 { TABLES.turn } else { 0 };
        for (at, &c) in self.board.cells.iter().enumerate() {
            if c != 0 {
                hash ^= TABLES.pieces[usize::from(c)][at];
            }
        }
        let p = self.board.protection;
        let mut rights = p.squares;
        while let Some(at) = rights.pop() {
            hash ^= TABLES.protection[p.owner][at];
        }
        hash
    }
    fn ply(&self) -> usize {
        self.keys.len() - 1 - self.root
    }
}
impl SearchPosition for ChuPosition {
    type Move = Move;
    // Chu threat replies can branch widely; bound private quiescence separately
    // from principal search depth while retaining complete evasions before it.
    const QUIESCENCE_PLIES: usize = 8;
    const HISTORY_SLOTS: usize = 2 * STATES * N;
    fn hash(&self) -> u64 {
        self.board.hash
    }
    fn in_check(&self) -> bool {
        self.check
    }
    fn generate(&self, stage: Stage, moves: &mut Vec<Move>) {
        if self.ply() == 0
            && let Some(allowed) = &self.root_allowed
        {
            moves.extend(
                allowed
                    .iter()
                    .copied()
                    .filter(|&m| self.is_tactical(m) == (stage == Stage::Tactical)),
            );
        } else {
            self.board.generate(stage, moves);
        }
    }
    fn generate_search(&self, stage: Stage, moves: &mut Vec<Move>) {
        let mut routes = Vec::new();
        // Native root restrictions are applied before choosing representatives.
        self.generate(stage, &mut routes);
        let mut seen: Vec<Board> = Vec::new();
        for m in routes {
            let mut after = self.board;
            if after.make(m).is_none() {
                continue;
            }
            if seen.iter().any(|b| {
                b.hash == after.hash
                    && b.cells == after.cells
                    && b.side == after.side
                    && b.protection == after.protection
            }) {
                continue;
            }
            seen.push(after);
            moves.push(m);
        }
    }
    fn make(&mut self, m: Move) -> bool {
        if self.ply() == 0 && self.root_allowed.as_ref().is_some_and(|a| !a.contains(&m)) {
            return false;
        }
        let Some(mut undo) = self.board.make(m) else {
            return false;
        };
        undo.reversible = self.reversible;
        undo.check = self.check;
        self.reversible = if undo.count > 0 || m.promotes() {
            0
        } else {
            self.reversible.saturating_add(1)
        };
        self.stack.push(undo);
        self.keys.push(self.board.hash);
        self.check = self.board.threatened(self.board.side);
        true
    }
    fn unmake(&mut self) {
        let undo = self.stack.pop().expect("balanced make/unmake");
        self.keys.pop();
        self.board.unmake(undo);
        self.reversible = undo.reversible;
        self.check = undo.check;
    }
    fn make_null(&mut self) {
        unreachable!("Chu has real compound passes; null move is disabled")
    }
    fn unmake_null(&mut self) {
        unreachable!("Chu null move is disabled")
    }
    fn null_move_safe(&self) -> bool {
        false
    }
    fn rule_score(&self) -> Option<i32> {
        let ply = self.ply() as i32;
        if self.board.royals[self.board.side].count() == 0 {
            return Some(-MATE_SCORE + ply);
        }
        if self.board.royals[self.board.side ^ 1].count() == 0 {
            return Some(MATE_SCORE - ply);
        }
        if let Some(winner) = self.board.material_winner() {
            return Some(if winner == self.board.side {
                MATE_SCORE - ply
            } else {
                -MATE_SCORE + ply
            });
        }
        // Capture-based games permit a move that leaves every royal exposed.
        // Prove the next complete capture now instead of expanding all quiet
        // replies in quiescence. This changes no local legal-action set. The
        // core skips rule_score at its root and still selects an actual move.
        if self.board.threatened(self.board.side ^ 1) {
            return Some(MATE_SCORE - ply - 1);
        }
        let current = self.keys.len() - 1;
        let occurrences =
            earlier_occurrences(&self.keys[..current], self.board.hash, 2..=self.reversible);
        cycle_start(occurrences, self.root, 2).map(|_| 0)
    }
    fn draw_claimable(&self) -> bool {
        false
    }
    fn no_move_score(&self, _ply: i32) -> i32 {
        0
    }
    fn evaluate(&self) -> i32 {
        let s = self.board.side;
        self.board.material[s] - self.board.material[s ^ 1] + self.board.placement[s]
            - self.board.placement[s ^ 1]
            + super::eval::DEFAULT[super::eval::COUNT - 1]
    }
    fn is_tactical(&self, m: Move) -> bool {
        m.promotes() || self.board.captures(m).1 != 0
    }
    fn tactical_order(&self, m: Move) -> i32 {
        self.tactical_gain(m) * 16 - meta(self.board.cells[m.from()]).value / 16
    }
    fn tactical_gain(&self, m: Move) -> i32 {
        self.board.exchange_gain(m)
    }
    /// Bounded material-exchange approximation, not a tactical proof. Each side
    /// may stop or recapture the last mover with a complete admitted action.
    /// Compound victims, promotion and the actual final square are included.
    /// Actual royal captures, material terminals and exhausted limits retain
    /// moves. Future checks and off-target replies are not searched, so this
    /// heuristic can undervalue a checking sacrifice; it is not a forced-loss proof.
    fn exchange_at_least(&self, m: Move, threshold: i32) -> bool {
        let mut after = self.board;
        if after.make(m).is_none() {
            return false;
        }
        let mut work = ExchangeWork {
            projections: 63,
            routes: 512,
            geometry: 4096,
        };
        after.exchange_greater(
            m.to(),
            self.board.exchange_gain(m) - threshold,
            0,
            &mut work,
        ) != Some(true)
    }
    fn history_slot(&self, m: Move) -> usize {
        (usize::from(self.board.cells[m.from()]) - 1) * N + m.to()
    }
    fn encode(m: Move) -> u32 {
        m.0 + 1
    }
    fn decode(&self, bits: u32) -> Option<Move> {
        let m = Move(bits.checked_sub(1)?);
        self.is_pseudo_legal(m).then_some(m)
    }
    fn is_pseudo_legal(&self, m: Move) -> bool {
        self.board.pseudo(m)
            && !(self.ply() == 0 && self.root_allowed.as_ref().is_some_and(|a| !a.contains(&m)))
    }
}

#[cfg(test)]
mod exchange_budget_tests {
    use super::*;
    #[test]
    fn unresolved_exchange_budgets_do_not_prove_a_loss() {
        let mut board = Board::empty(0);
        for (at, owner, name) in [
            (143, 0, "K"),
            (0, 1, "K"),
            (65, 0, "R"),
            (66, 1, "P"),
            (100, 1, "P"),
        ] {
            board.put(piece_code(Piece::from_code(side(owner), name).unwrap()), at);
        }
        for (projections, routes, geometry, depth) in [
            (0, 512, 4096, 0),
            (64, 0, 4096, 0),
            (64, 512, 0, 0),
            (64, 512, 4096, 8),
        ] {
            let mut work = ExchangeWork {
                projections,
                routes,
                geometry,
            };
            assert_eq!(board.exchange_greater(66, 0, depth, &mut work), None);
        }
    }
}
