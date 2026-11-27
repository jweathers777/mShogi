//! Private Tenjiku board and complete-action projection for the shared core.
//! Game remains authoritative for actual history, admission and outcomes.
use crate::alphabeta::{MATE_SCORE, SearchPosition, Stage};
use crate::repetition::earlier_occurrences;
use crate::square_set::SquareSet;
use mshogi_core::tenjiku::{
    Action, ActionKind, Base, Face, Game, GameCommand, LocalPosition, Piece, Position, Profile,
    PromotionRight, Side, Square,
};
use std::sync::LazyLock;
type Set = SquareSet<4>;
const N: usize = 256;
fn side(s: usize) -> Side {
    if s == 0 { Side::Sente } else { Side::Gote }
}
fn square(s: usize) -> Square {
    Square::new((s % 16 + 1) as u8, (s / 16 + 1) as u8).unwrap()
}
fn offset(s: usize, owner: usize, df: i8, dr: i8) -> Option<usize> {
    let sign = if owner == 0 { 1 } else { -1 };
    let f = (s % 16) as i16 + sign * i16::from(df);
    let r = (s / 16) as i16 + sign * i16::from(dr);
    ((0..16).contains(&f) && (0..16).contains(&r)).then_some((r * 16 + f) as usize)
}
fn distance(a: usize, b: usize) -> usize {
    (a % 16).abs_diff(b % 16).max((a / 16).abs_diff(b / 16))
}
fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e3779b97f4a7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}
fn code(p: Piece) -> u32 {
    let b = Base::ALL.iter().position(|&b| b == p.base).unwrap();
    let r = match p.right {
        PromotionRight::Unseen => 0,
        PromotionRight::FirstDeclinedNextMove => 1,
        PromotionRight::Consumed => 2,
    };
    1 + (p.side.index() * 216 + b * 6 + usize::from(p.promoted) * 3 + r) as u32
        + ((p.id as u32) << 16)
}
#[derive(Clone, Copy)]
struct Meta {
    piece: Piece,
    face: Face,
    rank: usize,
    value: i32,
    royal: bool,
}
static META: LazyLock<Vec<Meta>> = LazyLock::new(|| {
    (0usize..=432)
        .map(|c| {
            let i = c.saturating_sub(1);
            let owner = i / 216;
            let b = Base::ALL[(i % 216) / 6];
            let promoted = (i % 6) / 3 == 1;
            let mut p = Piece::new(0, side(owner), b, promoted)
                .unwrap_or_else(|_| Piece::new(0, side(owner), b, false).unwrap());
            p.right = match i % 3 {
                0 => PromotionRight::Unseen,
                1 => PromotionRight::FirstDeclinedNextMove,
                _ => PromotionRight::Consumed,
            };
            Meta {
                piece: p,
                face: p.face(),
                rank: p.face().rank() as usize,
                value: super::piece_value(p.face()),
                royal: p.is_royal(),
            }
        })
        .collect()
});
fn meta(c: u32) -> &'static Meta {
    &META[(c & 65535) as usize]
}
fn piece(c: u32) -> Piece {
    let mut p = meta(c).piece;
    p.id = (c >> 16) as u16;
    p
}
fn piece_hash(c: u32, at: usize) -> u64 {
    mix((u64::from(c) << 8) | at as u64)
}
fn repeat_hash(c: u32, at: usize) -> u64 {
    let p = meta(c).piece;
    let b = Base::ALL.iter().position(|&b| b == p.base).unwrap();
    mix(0x7265700000000000
        | ((p.side.index() * 72 + b * 2 + usize::from(p.promoted)) as u64) << 8
        | at as u64)
}
/// Source/first/second squares, a third-step direction, route length and flags.
/// Three-landing routes always have an adjacent final leg.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub struct Move(u32);
impl Move {
    fn travel(from: usize, path: &[usize], promote: bool) -> Self {
        let mut bits = from as u32
            | ((path[0] as u32) << 8)
            | (((path.len() - 1) as u32) << 27)
            | ((promote as u32) << 29);
        if path.len() > 1 {
            bits |= (path[1] as u32) << 16;
        }
        if path.len() > 2 {
            let d = A
                .iter()
                .position(|&(f, r)| offset(path[1], 0, f, r) == Some(path[2]))
                .unwrap();
            bits |= (d as u32) << 24;
        }
        Self(bits)
    }
    fn igui(from: usize, target: usize) -> Self {
        Self(from as u32 | ((target as u32) << 8) | (1 << 30))
    }
    fn from(self) -> usize {
        (self.0 & 255) as usize
    }
    fn first(self) -> usize {
        ((self.0 >> 8) & 255) as usize
    }
    fn len(self) -> usize {
        if self.igui_flag() {
            0
        } else {
            ((self.0 >> 27) & 3) as usize + 1
        }
    }
    fn igui_flag(self) -> bool {
        self.0 & (1 << 30) != 0
    }
    fn promotes(self) -> bool {
        self.0 & (1 << 29) != 0
    }
    fn promoted(self) -> Self {
        Self(self.0 | (1 << 29))
    }
    fn path(self) -> [usize; 3] {
        let second = ((self.0 >> 16) & 255) as usize;
        let (f, r) = A[((self.0 >> 24) & 7) as usize];
        [self.first(), second, offset(second, 0, f, r).unwrap_or(N)]
    }
    fn to(self) -> usize {
        if self.igui_flag() {
            self.from()
        } else {
            self.path()[self.len().min(3) - 1]
        }
    }
}

// Fixed movement facts transcribed from profile T02–T05; projection below is independent.
const O: &[(i8, i8)] = &[(0, -1), (0, 1), (-1, 0), (1, 0)];
const D: &[(i8, i8)] = &[(-1, -1), (1, -1), (-1, 1), (1, 1)];
const A: &[(i8, i8)] = &[
    (0, -1),
    (0, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (1, -1),
    (-1, 1),
    (1, 1),
];
#[derive(Clone, Copy)]
enum Special {
    None,
    Lion,
    Area,
    Ranked,
    RankedArea,
    ForwardLion,
    DiagonalLion,
    Tetrarch,
    DiagonalDouble,
}
struct Movement {
    steps: &'static [(i8, i8)],
    rays: &'static [(i8, i8)],
    jumps: &'static [(i8, i8)],
    limited: &'static [(i8, i8)],
    special: Special,
}
fn movement(face: Face, profile: Profile) -> Movement {
    use Face::*;
    use Special::*;
    type Directions = &'static [(i8, i8)];
    let (steps, rays, jumps, limited, special): (
        Directions,
        Directions,
        Directions,
        Directions,
        Special,
    ) = match face {
        K | CP => (A, &[], &[], &[], None),
        FiD => (
            &[],
            &[(0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            Area,
        ),
        GGn => (&[], A, &[], &[], Ranked),
        VGn => (&[], D, &[], &[], RankedArea),
        RGn => (&[], O, &[], &[], Ranked),
        BGn => (&[], D, &[], &[], Ranked),
        LHk => (&[], D, &[], &[], Lion),
        Ln => (&[], &[], &[], &[], Lion),
        WBf => (
            &[],
            &[(-1, 0), (1, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[(0, -1), (0, 1)],
            None,
        ),
        FEg => {
            if profile == Profile::AdamsModern {
                (&[], A, &[(0, -2), (0, 2), (-2, 0), (2, 0)], &[], None)
            } else {
                (&[], A, &[], &[], DiagonalDouble)
            }
        }
        FK => (&[], A, &[], &[], None),
        HT => (&[], &[], &[], &[], Tetrarch),
        SE => (
            &[],
            &[(0, -1), (0, 1), (-1, 0), (1, 0), (-1, 1), (1, 1)],
            &[],
            &[],
            DiagonalLion,
        ),
        HF => (
            &[],
            &[(0, 1), (-1, 0), (1, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            ForwardLion,
        ),
        ChS => (
            &[],
            &[(0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[(-1, 0), (1, 0)],
            None,
        ),
        DK => (D, O, &[], &[], None),
        DH => (O, D, &[], &[], None),
        FO => (
            &[],
            &[(0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            None,
        ),
        FB => (
            &[],
            &[(-1, 0), (1, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            None,
        ),
        R => (&[], O, &[], &[], None),
        B => (&[], D, &[], &[], None),
        Ph => (O, &[], &[(-2, -2), (2, -2), (-2, 2), (2, 2)], &[], None),
        VSo => (&[(0, 1)], &[(0, -1)], &[], &[(-1, 0), (1, 0)], None),
        VM => (&[(-1, 0), (1, 0)], &[(0, -1), (0, 1)], &[], &[], None),
        SSo => (&[(0, 1)], &[(-1, 0), (1, 0)], &[], &[(0, -1)], None),
        SM => (&[(0, -1), (0, 1)], &[(-1, 0), (1, 0)], &[], &[], None),
        WH => (&[], &[(0, -1), (0, 1), (-1, -1), (1, -1)], &[], &[], None),
        W => (&[], &[(0, -1), (0, 1), (-1, 1), (1, 1)], &[], &[], None),
        Ky => (D, &[], &[(0, -2), (0, 2), (-2, 0), (2, 0)], &[], None),
        FS => (
            &[(-1, 0), (1, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[(0, -1), (0, 1)],
            &[],
            &[],
            None,
        ),
        RC => (&[], &[(0, -1), (0, 1)], &[], &[], None),
        L => (&[], &[(0, -1)], &[], &[], None),
        MGn => (&[], &[(0, -1), (-1, 1), (1, 1)], &[], &[], None),
        G => (
            &[(0, -1), (0, 1), (-1, 0), (1, 0), (-1, -1), (1, -1)],
            &[],
            &[],
            &[],
            None,
        ),
        DE => (
            &[(0, -1), (-1, 0), (1, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            &[],
            None,
        ),
        BT => (
            &[(0, 1), (-1, 0), (1, 0), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            &[],
            None,
        ),
        FL => (
            &[(0, -1), (0, 1), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            &[],
            None,
        ),
        S => (
            &[(0, -1), (-1, -1), (1, -1), (-1, 1), (1, 1)],
            &[],
            &[],
            &[],
            None,
        ),
        C => (&[(0, -1), (0, 1), (-1, -1), (1, -1)], &[], &[], &[], None),
        I => (&[(0, -1), (-1, -1), (1, -1)], &[], &[], &[], None),
        N => (&[], &[], &[(-1, -2), (1, -2)], &[], None),
        Dg => (&[(0, -1), (-1, 1), (1, 1)], &[], &[], &[], None),
        P => (&[(0, -1)], &[], &[], &[], None),
    };
    Movement {
        steps,
        rays,
        jumps,
        limited,
        special,
    }
}
// A single landing depends on its displacement and, for rays, the pieces between
// its endpoints. Store displacements from Sente's perspective and rotate for Gote.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Direct {
    Unreachable,
    Leap,
    Ray,
    RankedRay,
    SkipAdjacent,
}
const DELTAS: usize = 31 * 31;
fn delta_index(file: i8, rank: i8) -> usize {
    (rank + 15) as usize * 31 + (file + 15) as usize
}
static DIRECT: LazyLock<[[[Direct; DELTAS]; Face::ALL.len()]; Profile::ALL.len()]> =
    LazyLock::new(|| Profile::ALL.map(|profile| Face::ALL.map(|face| direct_moves(face, profile))));

fn direct_moves(face: Face, profile: Profile) -> [Direct; DELTAS] {
    use Direct::*;
    let data = movement(face, profile);
    let mut table = [Unreachable; DELTAS];
    let mut put = |file, rank, kind| table[delta_index(file, rank)] = kind;
    let ray = if matches!(data.special, Special::Ranked | Special::RankedArea) {
        RankedRay
    } else {
        Ray
    };
    for &(f, r) in data.rays {
        for n in 1..=15 {
            put(f * n, r * n, ray);
        }
    }
    for &(f, r) in data.limited {
        put(f, r, Leap);
        put(f * 2, r * 2, Ray);
    }
    // A direct jump needs no empty intermediate square, even on a common ray.
    for &(f, r) in data.steps.iter().chain(data.jumps) {
        put(f, r, Leap);
    }
    match data.special {
        Special::Lion => {
            for f in -2..=2 {
                for r in -2..=2 {
                    if f != 0 || r != 0 {
                        put(f, r, Leap);
                    }
                }
            }
        }
        Special::ForwardLion | Special::DiagonalLion => {
            let dirs: &[(i8, i8)] = if matches!(data.special, Special::ForwardLion) {
                &[(0, -1)]
            } else {
                &[(-1, -1), (1, -1)]
            };
            for &(f, r) in dirs {
                put(f, r, Leap);
                put(f * 2, r * 2, Leap);
            }
        }
        Special::Area | Special::RankedArea => {
            for &(f, r) in A {
                put(f, r, Leap);
            }
        }
        Special::Tetrarch => {
            for &(f, r) in A {
                let limit = if r == 0 { 3 } else { 15 };
                if f != 0 || profile == Profile::MullerCompleted {
                    for n in 2..=limit {
                        put(f * n, r * n, SkipAdjacent);
                    }
                }
            }
        }
        Special::None | Special::Ranked | Special::DiagonalDouble => {}
    }
    table
}
#[derive(Clone, Copy)]
struct Board {
    cells: [u32; N],
    occupied: [Set; 2],
    royals: [Set; 2],
    demons: [Set; 2],
    side: usize,
    profile: Profile,
    hash: u64,
    repeat: u64,
    material: [i32; 2],
    placement: [i32; 2],
}
#[derive(Clone)]
struct Undo {
    changed: [(usize, u32); 12],
    count: usize,
    gain: i32,
    check: bool,
}
impl Undo {
    fn new() -> Self {
        Self {
            changed: [(0, 0); 12],
            count: 0,
            gain: 0,
            check: false,
        }
    }
}
struct ExchangeWork {
    projections: usize,
    routes: usize,
    geometry: usize,
}
impl Board {
    fn empty(profile: Profile, side: usize) -> Self {
        let hash = mix(0x74656e6a696b7500 | (profile as u64)) ^ if side == 1 { mix(7) } else { 0 };
        Self {
            cells: [0; N],
            occupied: [Set::default(); 2],
            royals: [Set::default(); 2],
            demons: [Set::default(); 2],
            side,
            profile,
            hash,
            repeat: hash,
            material: [0; 2],
            placement: [0; 2],
        }
    }
    fn placement(c: u32, at: usize) -> i32 {
        let p = meta(c).piece;
        let advance = if p.side == Side::Sente {
            15 - at / 16
        } else {
            at / 16
        };
        let central = 7 - (at % 16).abs_diff(7).min((at % 16).abs_diff(8)) as i32;
        match meta(c).face {
            Face::K | Face::CP => 0,
            Face::P | Face::Dg => advance as i32 * 4,
            Face::Ln | Face::LHk | Face::FiD => central * 6 + advance.min(10) as i32 * 2,
            _ => central,
        }
    }
    fn set(&mut self, at: usize, c: u32) {
        let old = self.cells[at];
        if old != 0 {
            let m = meta(old);
            let s = m.piece.side.index();
            self.occupied[s].remove(at);
            if m.royal {
                self.royals[s].remove(at);
            }
            if m.face == Face::FiD {
                self.demons[s].remove(at);
            }
            self.material[s] -= m.value;
            self.placement[s] -= Self::placement(old, at);
            self.hash ^= piece_hash(old, at);
            self.repeat ^= repeat_hash(old, at);
        }
        self.cells[at] = c;
        if c != 0 {
            let m = meta(c);
            let s = m.piece.side.index();
            self.occupied[s].add(at);
            if m.royal {
                self.royals[s].add(at);
            }
            if m.face == Face::FiD {
                self.demons[s].add(at);
            }
            self.material[s] += m.value;
            self.placement[s] += Self::placement(c, at);
            self.hash ^= piece_hash(c, at);
            self.repeat ^= repeat_hash(c, at);
        }
    }
    fn write(&mut self, at: usize, c: u32, u: &mut Undo) {
        if !u.changed[..u.count].iter().any(|&(s, _)| s == at) {
            assert!(u.count < u.changed.len());
            u.changed[u.count] = (at, self.cells[at]);
            u.count += 1;
        }
        self.set(at, c);
    }
    fn restore(&mut self, u: &Undo) {
        for &(at, c) in &u.changed[..u.count] {
            self.set(at, c);
        }
    }
    fn flip(&mut self) {
        self.side ^= 1;
        self.hash ^= mix(7);
        self.repeat ^= mix(7);
    }
    fn unmake(&mut self, u: &Undo) {
        self.flip();
        self.restore(u);
    }
    fn enemy(&self, at: usize, owner: usize) -> bool {
        self.cells[at] != 0 && meta(self.cells[at]).piece.side.index() != owner
    }
    fn adjacent_demon(&self, at: usize, owner: usize) -> bool {
        let mut demons = self.demons[owner ^ 1];
        while let Some(s) = demons.pop() {
            if distance(at, s) == 1 {
                return true;
            }
        }
        false
    }
    fn direct(&self, c: u32, from: usize, to: usize) -> bool {
        let mover = meta(c);
        let df = (to % 16) as i8 - (from % 16) as i8;
        let dr = (to / 16) as i8 - (from / 16) as i8;
        let delta = if mover.piece.side == Side::Sente {
            delta_index(df, dr)
        } else {
            delta_index(-df, -dr)
        };
        let kind = DIRECT[self.profile as usize][mover.face as usize][delta];
        match kind {
            Direct::Unreachable => return false,
            Direct::Leap => return true,
            _ => {}
        }
        let stride = isize::from(df.signum()) + 16 * isize::from(dr.signum());
        let mut at = from as isize + stride;
        if kind == Direct::SkipAdjacent {
            at += stride;
        }
        let mut jumped = false;
        while at != to as isize {
            let blocker = self.cells[at as usize];
            if blocker != 0 {
                if kind != Direct::RankedRay || meta(blocker).rank >= mover.rank {
                    return false;
                }
                jumped = true;
            }
            at += stride;
        }
        !jumped
            || (self.enemy(to, mover.piece.side.index())
                && meta(self.cells[to]).rank != 4
                && (self.profile == Profile::MullerCompleted
                    || meta(self.cells[to]).rank < mover.rank))
    }
    fn geometry(&self, m: Move) -> bool {
        if m.0 >> 31 != 0 || m.len() > 3 {
            return false;
        }
        let from = m.from();
        let c = self.cells[from];
        if c == 0 || meta(c).piece.side.index() != self.side {
            return false;
        }
        if m.len() == 1 {
            return self.direct(c, from, m.first());
        }
        let data = movement(meta(c).face, self.profile);
        let owner = self.side;
        if m.igui_flag() {
            return matches!(data.special, Special::Tetrarch)
                && !m.promotes()
                && m.0 & 0x1fff0000 == 0
                && distance(from, m.first()) == 1;
        }
        let path = m.path();
        if path[..m.len()].iter().any(|&s| s >= N) {
            return false;
        }
        if matches!(data.special, Special::Area | Special::RankedArea) {
            let mut at = from;
            for &to in &path[..m.len()] {
                if distance(at, to) != 1 {
                    return false;
                }
                at = to;
            }
            return true;
        }
        if m.len() == 2 {
            let mid = path[0];
            let to = path[1];
            return match data.special {
                Special::Lion => distance(from, mid) == 1 && distance(mid, to) == 1,
                Special::ForwardLion | Special::DiagonalLion => {
                    let dirs: &[(i8, i8)] = if matches!(data.special, Special::ForwardLion) {
                        &[(0, -1)]
                    } else {
                        &[(-1, -1), (1, -1)]
                    };
                    dirs.iter().any(|&(f, r)| {
                        offset(from, owner, f, r) == Some(mid)
                            && (to == from || offset(mid, owner, f, r) == Some(to))
                    })
                }
                Special::DiagonalDouble => {
                    (from % 16).abs_diff(mid % 16) == 1
                        && (from / 16).abs_diff(mid / 16) == 1
                        && (mid % 16).abs_diff(to % 16) == 1
                        && (mid / 16).abs_diff(to / 16) == 1
                }
                _ => false,
            };
        }
        false
    }
    fn mandatory(p: Piece, to: usize) -> bool {
        if p.promoted {
            return false;
        }
        let advance = if p.side == Side::Sente {
            to / 16
        } else {
            15 - to / 16
        };
        match p.base {
            Base::P | Base::L | Base::I => advance == 0,
            Base::N => advance <= 1,
            _ => false,
        }
    }
    fn make(&mut self, m: Move) -> Option<Undo> {
        if !self.geometry(m) {
            return None;
        }
        self.project(m)
    }
    // Geometry is known for generated routes, but full effects and promotion still
    // decide admission. At most source + 3 landings + 8 burn neighbors are changed.
    fn project(&mut self, m: Move) -> Option<Undo> {
        let c = self.cells[m.from()];
        let before = piece(c);
        let owner = before.side.index();
        let to = m.to();
        let path = m.path();
        let mut u = Undo::new();
        let value = self.material[owner] - self.material[owner ^ 1];
        let mut captured = false;
        if m.igui_flag() {
            if !self.enemy(m.first(), owner) {
                return None;
            }
            captured = true;
            self.write(m.first(), 0, &mut u);
        } else {
            self.write(m.from(), 0, &mut u);
            let area = matches!(
                movement(before.face(), self.profile).special,
                Special::Area | Special::RankedArea
            ) && m.len() > 1;
            let mut at = m.from();
            for (i, &next) in path[..m.len()].iter().enumerate() {
                let victim = self.cells[next];
                if victim != 0 {
                    if meta(victim).piece.side.index() == owner || (area && i + 1 < m.len()) {
                        self.restore(&u);
                        return None;
                    }
                    captured = true;
                }
                self.write(at, 0, &mut u);
                self.write(next, c, &mut u);
                at = next;
            }
        }
        let hazard = self.adjacent_demon(to, owner);
        if before.face() == Face::FiD && (!hazard || self.profile == Profile::AdamsModern) {
            for &(f, r) in A {
                if let Some(at) = offset(to, 0, f, r)
                    && self.enemy(at, owner)
                    && meta(self.cells[at]).face != Face::FiD
                {
                    self.write(at, 0, &mut u);
                }
            }
        }
        let eligible = !before.promoted
            && before.base.promotable()
            && (before.side.in_zone(square(m.from()).rank())
                || path[..m.len()]
                    .iter()
                    .any(|&s| before.side.in_zone(square(s).rank())));
        let must = Self::mandatory(before, to) && !hazard;
        if (m.promotes()
            && (!eligible
                || (before.right == PromotionRight::FirstDeclinedNextMove && !captured && !must)))
            || (!m.promotes() && must)
        {
            self.restore(&u);
            return None;
        }
        if hazard {
            self.write(to, 0, &mut u);
        } else {
            let mut after = before;
            if m.promotes() {
                after.promoted = true;
                after.right = PromotionRight::Consumed;
            } else if before.right == PromotionRight::FirstDeclinedNextMove {
                after.right = PromotionRight::Consumed;
            } else if eligible && before.right == PromotionRight::Unseen {
                after.right = PromotionRight::FirstDeclinedNextMove;
            }
            self.write(to, code(after), &mut u);
        }
        u.gain = self.material[owner] - self.material[owner ^ 1] - value;
        self.flip();
        Some(u)
    }
    fn area_routes(&self, from: usize, at: usize, path: &mut Vec<usize>, out: &mut Vec<Move>) {
        if path.len() == 3 {
            return;
        }
        for &(f, r) in A {
            if let Some(to) = offset(at, 0, f, r) {
                if to != from
                    && self.cells[to] != 0
                    && meta(self.cells[to]).piece.side.index() == self.side
                {
                    continue;
                }
                path.push(to);
                out.push(Move::travel(from, path, false));
                if to == from || self.cells[to] == 0 {
                    self.area_routes(from, to, path, out);
                }
                path.pop();
            }
        }
    }
    fn raw(&self, from: usize, out: &mut Vec<Move>) {
        let c = self.cells[from];
        let data = movement(meta(c).face, self.profile);
        let owner = meta(c).piece.side.index();
        let mut direct = Set::default();
        for &(f, r) in data.steps.iter().chain(data.jumps) {
            if let Some(to) = offset(from, owner, f, r) {
                direct.add(to);
            }
        }
        for &(f, r) in data.limited {
            for n in 1..=2 {
                let Some(to) = offset(from, owner, f * n, r * n) else {
                    break;
                };
                direct.add(to);
                if self.cells[to] != 0 {
                    break;
                }
            }
        }
        for &(f, r) in data.rays {
            for n in 1..=15 {
                let Some(to) = offset(from, owner, f * n, r * n) else {
                    break;
                };
                direct.add(to);
                let q = self.cells[to];
                if q != 0
                    && (!matches!(data.special, Special::Ranked | Special::RankedArea)
                        || meta(q).rank >= meta(c).rank)
                {
                    break;
                }
            }
        }
        match data.special {
            Special::Lion => {
                for f in -2..=2 {
                    for r in -2..=2 {
                        if (f != 0 || r != 0)
                            && let Some(to) = offset(from, owner, f, r)
                        {
                            direct.add(to);
                        }
                    }
                }
                for &(f, r) in A {
                    if let Some(mid) = offset(from, owner, f, r) {
                        for &(ff, rr) in A {
                            if let Some(to) = offset(mid, owner, ff, rr) {
                                out.push(Move::travel(from, &[mid, to], false));
                            }
                        }
                    }
                }
            }
            Special::Area | Special::RankedArea => {
                self.area_routes(from, from, &mut Vec::with_capacity(3), out)
            }
            Special::ForwardLion | Special::DiagonalLion => {
                let dirs: &[(i8, i8)] = if matches!(data.special, Special::ForwardLion) {
                    &[(0, -1)]
                } else {
                    &[(-1, -1), (1, -1)]
                };
                for &(f, r) in dirs {
                    if let Some(mid) = offset(from, owner, f, r) {
                        direct.add(mid);
                        out.push(Move::travel(from, &[mid, from], false));
                        if let Some(to) = offset(mid, owner, f, r) {
                            direct.add(to);
                            out.push(Move::travel(from, &[mid, to], false));
                        }
                    }
                }
            }
            Special::DiagonalDouble => {
                for &(f, r) in D {
                    if let Some(mid) = offset(from, owner, f, r) {
                        for &(ff, rr) in D {
                            if let Some(to) = offset(mid, owner, ff, rr) {
                                out.push(Move::travel(from, &[mid, to], false));
                            }
                        }
                    }
                }
            }
            Special::Tetrarch => {
                for &(f, r) in A {
                    if let Some(to) = offset(from, owner, f, r)
                        && self.enemy(to, owner)
                    {
                        out.push(Move::igui(from, to));
                    }
                    if r == 0 {
                        for n in 2..=3 {
                            let Some(to) = offset(from, owner, f * n, 0) else {
                                break;
                            };
                            direct.add(to);
                            if self.cells[to] != 0 {
                                break;
                            }
                        }
                    } else if f != 0 || self.profile == Profile::MullerCompleted {
                        for n in 2..=15 {
                            let Some(to) = offset(from, owner, f * n, r * n) else {
                                break;
                            };
                            direct.add(to);
                            if self.cells[to] != 0 {
                                break;
                            }
                        }
                    }
                }
            }
            Special::None | Special::Ranked => {}
        }
        while let Some(to) = direct.pop() {
            out.push(Move::travel(from, &[to], false));
        }
    }
    fn raw_unique(&self, from: usize) -> Vec<Move> {
        let mut out = Vec::new();
        self.raw(from, &mut out);
        out.sort_unstable();
        out.dedup();
        out
    }
    fn tactical_after(&self, m: Move, u: &Undo) -> bool {
        let owner = self.side ^ 1;
        m.promotes()
            || self.cells[m.to()] == 0
            || u.changed[..u.count].iter().any(|&(at, c)| {
                c != 0 && meta(c).piece.side.index() != owner && self.cells[at] != c
            })
    }
    fn generate_filtered(
        &self,
        out: &mut Vec<Move>,
        mut include: impl FnMut(&Board, Move, &Undo) -> bool,
    ) {
        let mut pieces = self.occupied[self.side];
        let mut scratch = *self;
        while let Some(from) = pieces.pop() {
            let p = meta(self.cells[from]).piece;
            for m in self.raw_unique(from) {
                for promote in [false, true] {
                    if promote && (m.igui_flag() || p.promoted || !p.base.promotable()) {
                        continue;
                    }
                    let m = if promote { m.promoted() } else { m };
                    if let Some(u) = scratch.make(m) {
                        if include(&scratch, m, &u) {
                            out.push(m);
                        }
                        scratch.unmake(&u);
                    }
                }
            }
        }
    }
    fn could_remove(&self, from: usize, target: usize) -> bool {
        let c = self.cells[from];
        let special = movement(meta(c).face, self.profile).special;
        if matches!(special, Special::Area | Special::RankedArea) && distance(from, target) <= 3 {
            return true;
        }
        // Muller's two diagonal steps can turn around a blocker on the
        // orthogonal Queen ray. The direct one-landing test is insufficient.
        if matches!(special, Special::DiagonalDouble)
            && distance(from, target) <= 2
            && (from % 16).abs_diff(target % 16).is_multiple_of(2)
            && (from / 16).abs_diff(target / 16).is_multiple_of(2)
        {
            return true;
        }
        if self.direct(c, from, target) {
            return true;
        }
        if meta(c).face == Face::HT && distance(from, target) == 1 {
            return true;
        }
        if meta(c).face == Face::FiD {
            for &(f, r) in A {
                if let Some(to) = offset(target, 0, f, r)
                    && (distance(from, to) <= 3 || self.direct(c, from, to))
                {
                    return true;
                }
            }
        }
        false
    }
    // Full action proof, including simultaneous burn/suicide. Geometric attacks
    // never establish a game outcome, and losing both royal sets is not a win.
    fn can_win(&self, attacker: usize) -> bool {
        if self.royals[attacker].count() == 0 || self.royals[attacker ^ 1].count() == 0 {
            return false;
        }
        let mut scratch = *self;
        if scratch.side != attacker {
            scratch.flip();
        }
        let mut pieces = scratch.occupied[attacker];
        while let Some(from) = pieces.pop() {
            let mut targets = scratch.royals[attacker ^ 1];
            if !targets
                .pop()
                .is_some_and(|target| scratch.could_remove(from, target))
            {
                continue;
            }
            let p = meta(scratch.cells[from]).piece;
            for m in scratch.raw_unique(from) {
                let mut touched = false;
                let mut targets = scratch.royals[attacker ^ 1];
                while let Some(target) = targets.pop() {
                    if (m.igui_flag() && m.first() == target)
                        || (!m.igui_flag() && m.path()[..m.len()].contains(&target))
                        || (meta(scratch.cells[from]).face == Face::FiD
                            && distance(m.to(), target) == 1)
                    {
                        touched = true;
                        break;
                    }
                }
                if !touched {
                    continue;
                }
                for promote in [false, true] {
                    if promote && (p.promoted || !p.base.promotable() || m.igui_flag()) {
                        continue;
                    }
                    let m = if promote { m.promoted() } else { m };
                    if let Some(u) = scratch.make(m) {
                        let wins = scratch.royals[attacker ^ 1].count() == 0
                            && scratch.royals[attacker].count() != 0;
                        scratch.unmake(&u);
                        if wins {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    fn bare_score(&self, ply: i32) -> Option<i32> {
        let bare = [
            self.occupied[0].count() == 1 && self.royals[0].count() == 1,
            self.occupied[1].count() == 1 && self.royals[1].count() == 1,
        ];
        if bare == [true, true] {
            return Some(0);
        }
        let victim = bare.iter().position(|&b| b)?;
        if victim != self.side {
            return Some(MATE_SCORE - ply);
        }
        let mut royals = self.royals[victim];
        let from = royals.pop().unwrap();
        let mut scratch = *self;
        for m in self.raw_unique(from) {
            if let Some(u) = scratch.make(m) {
                let escape = scratch.royals[victim ^ 1].count() == 0
                    || (scratch.royals[victim].count() != 0
                        && scratch.occupied[victim].count() == 1
                        && scratch.occupied[victim ^ 1].count() == 1
                        && scratch.royals[victim ^ 1].count() == 1);
                scratch.unmake(&u);
                if escape {
                    return None;
                }
            }
        }
        Some(-MATE_SCORE + ply)
    }
    // This exchange model may stop or remove the previous mover. It is a
    // material heuristic, not proof against checking/off-target sacrifices.
    // Removal includes Demon burns, direct captures, promotion and mover suicide.
    fn exchange_greater(
        &self,
        target: usize,
        limit: i32,
        depth: u8,
        work: &mut ExchangeWork,
    ) -> Option<bool> {
        if self.royals.iter().any(|r| r.count() == 0)
            || self.occupied.iter().any(|o| o.count() <= 1)
        {
            return None;
        }
        if limit < 0 {
            return Some(true);
        }
        if depth == 8 {
            return None;
        }
        let target_code = self.cells[target];
        if target_code == 0 {
            return Some(false);
        }
        let mut attackers = Vec::new();
        let mut pieces = self.occupied[self.side];
        while let Some(from) = pieces.pop() {
            if work.geometry == 0 {
                return None;
            }
            work.geometry -= 1;
            if self.could_remove(from, target) {
                attackers.push(from);
            }
        }
        attackers.sort_unstable_by_key(|&from| meta(self.cells[from]).value);
        let mut unknown = false;
        let mut seen: Vec<(u64, [u32; N], usize)> = Vec::new();
        let mut scratch = *self;
        for from in attackers {
            // At most 256 distinct direct destinations plus 64 two-step and 512
            // three-step routes. Reserve before generation, then refund unused.
            const MAX_ROUTES: usize = 832;
            if work.routes < MAX_ROUTES {
                return None;
            }
            work.routes -= MAX_ROUTES;
            let routes = self.raw_unique(from);
            debug_assert!(routes.len() <= MAX_ROUTES);
            work.routes += MAX_ROUTES - routes.len();
            let p = meta(self.cells[from]).piece;
            for m in routes {
                for promote in [false, true] {
                    if promote && (m.igui_flag() || p.promoted || !p.base.promotable()) {
                        continue;
                    }
                    let m = if promote { m.promoted() } else { m };
                    // A route must actually touch the target, or finish with an
                    // already-Demon next to it. Full projection confirms removal.
                    if !((m.igui_flag() && m.first() == target)
                        || (!m.igui_flag() && m.path()[..m.len()].contains(&target))
                        || (meta(self.cells[from]).face == Face::FiD
                            && distance(m.to(), target) == 1))
                    {
                        continue;
                    }
                    if work.projections == 0 {
                        return None;
                    }
                    work.projections -= 1;
                    let Some(u) = scratch.make(m) else {
                        continue;
                    };
                    if scratch.cells[target] == target_code {
                        scratch.unmake(&u);
                        continue;
                    }
                    if seen
                        .iter()
                        .any(|(h, c, t)| *h == scratch.hash && *c == scratch.cells && *t == m.to())
                    {
                        scratch.unmake(&u);
                        continue;
                    }
                    seen.push((scratch.hash, scratch.cells, m.to()));
                    let reply = if scratch.royals.iter().any(|r| r.count() == 0)
                        || scratch.occupied.iter().any(|o| o.count() <= 1)
                    {
                        None
                    } else if scratch.cells[m.to()] == 0 {
                        Some(0 > u.gain - limit - 1)
                    } else {
                        scratch.exchange_greater(m.to(), u.gain - limit - 1, depth + 1, work)
                    };
                    scratch.unmake(&u);
                    match reply {
                        Some(false) => return Some(true),
                        Some(true) => {}
                        None => unknown = true,
                    }
                }
            }
        }
        if unknown { None } else { Some(false) }
    }
    fn terminal(&self, ply: i32) -> Option<i32> {
        match (
            self.royals[self.side].count(),
            self.royals[self.side ^ 1].count(),
        ) {
            (0, 0) => Some(0),
            (0, _) => Some(-MATE_SCORE + ply),
            (_, 0) => Some(MATE_SCORE - ply),
            _ => self.bare_score(ply),
        }
    }
}
/// Fast private board. Game root action admission and PV validation
/// remain outside this type; deeper repetition is a board-keyed approximation.
#[derive(Clone)]
pub struct TenjikuPosition {
    board: Board,
    stack: Vec<Undo>,
    keys: Vec<u64>,
    check: bool,
    root_allowed: Option<Vec<Move>>,
}
impl TenjikuPosition {
    /// Build private state from one checked game board.
    pub fn from_position(p: &Position) -> Self {
        let mut board = Board::empty(p.profile(), p.side_to_move().index());
        for (s, p) in p.pieces() {
            board.set(s.index(), code(p));
        }
        let check = board.can_win(board.side ^ 1);
        Self {
            board,
            stack: Vec::new(),
            keys: vec![board.repeat],
            check,
            root_allowed: None,
        }
    }
    /// Build private state from local rules.
    pub fn from_local(p: &LocalPosition) -> Self {
        Self::from_position(p.position())
    }
    /// Replay known game board actions once; no Game is used below root.
    pub fn from_game(game: &Game) -> Self {
        let root = game.event_prefix(0).expect("game root").local_position();
        let mut p = Self::from_local(&root);
        for event in game.events() {
            if let GameCommand::Play { action } = &event.command {
                let m = p.action_move(action).expect("action shape");
                p.board.make(m).expect("admitted complete action");
                p.keys.push(p.board.repeat);
            }
        }
        p.check = p.board.can_win(p.board.side ^ 1);
        p
    }
    fn action_move(&self, a: &Action) -> Option<Move> {
        if a.kind == ActionKind::Igui {
            return (a.route.is_empty() && !a.promote)
                .then(|| a.target.map(|s| Move::igui(a.from.index(), s.index())))
                .flatten();
        }
        if a.target.is_some() || a.route.is_empty() || a.route.len() > 3 {
            return None;
        }
        let path = a.route.iter().map(|s| s.index()).collect::<Vec<_>>();
        if path.len() == 3 && distance(path[1], path[2]) != 1 {
            return None;
        }
        Some(Move::travel(a.from.index(), &path, a.promote))
    }
    /// Convert the complete private route into a action.
    pub fn to_action(&self, m: Move) -> Action {
        if m.igui_flag() {
            Action::igui(square(m.from()), square(m.first()))
        } else {
            Action {
                kind: ActionKind::Travel,
                from: square(m.from()),
                route: m.path()[..m.len()].iter().map(|&s| square(s)).collect(),
                promote: m.promotes(),
                target: None,
            }
        }
    }
    /// Decode a action if its local geometry is available.
    pub fn find(&self, a: &Action) -> Option<Move> {
        let m = self.action_move(a)?;
        self.is_pseudo_legal(m).then_some(m)
    }
    /// Apply exact game root admission before search representatives.
    pub fn restrict_root(&mut self, actions: &[Action]) {
        self.root_allowed = Some(
            actions
                .iter()
                .map(|a| self.action_move(a).expect("action"))
                .collect(),
        );
    }
    /// Read the complete game identity and promotion right at a square.
    pub fn piece(&self, at: Square) -> Option<Piece> {
        let c = self.board.cells[at.index()];
        (c != 0).then(|| piece(c))
    }
    /// Current rules owner.
    pub fn side_to_move(&self) -> Side {
        side(self.board.side)
    }
    /// Immutable selected rules package.
    pub fn profile(&self) -> Profile {
        self.board.profile
    }
    /// Incremental classical material for one owner.
    pub fn material(&self, side: Side) -> i32 {
        self.board.material[side.index()]
    }
    /// Recompute full board, profile, identity and rights hash for verification.
    pub fn hash_from_scratch(&self) -> u64 {
        let mut b = Board::empty(self.board.profile, self.board.side);
        for (at, &c) in self.board.cells.iter().enumerate() {
            if c != 0 {
                b.set(at, c);
            }
        }
        #[cfg(test)]
        {
            assert_eq!(b.occupied, self.board.occupied);
            assert_eq!(b.royals, self.board.royals);
            assert_eq!(b.demons, self.board.demons);
            assert_eq!(b.material, self.board.material);
            assert_eq!(b.placement, self.board.placement);
            assert_eq!(b.repeat, self.board.repeat);
        }
        b.hash
    }
    /// Complete local action set, or exact installed game root actions.
    pub fn legal_moves(&mut self) -> Vec<Move> {
        let mut out = Vec::new();
        self.generate(Stage::Tactical, &mut out);
        self.generate(Stage::Quiet, &mut out);
        out
    }
    /// Count complete local routes without terminal truncation.
    pub fn perft(&mut self, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let mut count = 0;
        for m in self.legal_moves() {
            assert!(self.make(m));
            count += self.perft(depth - 1);
            self.unmake();
        }
        count
    }
}
impl SearchPosition for TenjikuPosition {
    type Move = Move;
    const QUIESCENCE_PLIES: usize = 4;
    const QUIESCENCE_CHECKS: usize = 8;
    const HISTORY_SLOTS: usize = 2 * 44 * N;
    fn hash(&self) -> u64 {
        self.board.hash
    }
    fn in_check(&self) -> bool {
        self.check
    }
    fn generate(&self, stage: Stage, out: &mut Vec<Move>) {
        if self.stack.is_empty()
            && let Some(allowed) = &self.root_allowed
        {
            out.extend(
                allowed
                    .iter()
                    .copied()
                    .filter(|&m| self.is_tactical(m) == (stage == Stage::Tactical)),
            );
        } else {
            self.board.generate_filtered(out, |board, m, undo| {
                board.tactical_after(m, undo) == (stage == Stage::Tactical)
            });
        }
    }
    fn generate_search(&self, stage: Stage, out: &mut Vec<Move>) {
        let mut routes = Vec::new();
        self.generate(stage, &mut routes);
        let mut scratch = self.board;
        let mut seen: Vec<(u64, [u32; N])> = Vec::new();
        for m in routes {
            if let Some(u) = scratch.make(m) {
                if !seen
                    .iter()
                    .any(|(h, c)| *h == scratch.hash && *c == scratch.cells)
                {
                    seen.push((scratch.hash, scratch.cells));
                    out.push(m);
                }
                scratch.unmake(&u);
            }
        }
    }
    fn generate_checks(&self, out: &mut Vec<Move>) {
        let mut seen = Vec::new();
        let mut include = |board: &Board, m, undo: &Undo| {
            // Tactical candidates are a permitted superset. The core removes them
            // at the quiet frontier. Count equivalent quiet threats only once.
            if board.tactical_after(m, undo) {
                return true;
            }
            if !board.can_win(board.side ^ 1) || seen.contains(&(board.hash, board.cells)) {
                return false;
            }
            seen.push((board.hash, board.cells));
            true
        };
        if self.stack.is_empty()
            && let Some(allowed) = &self.root_allowed
        {
            let mut scratch = self.board;
            for &m in allowed {
                if let Some(undo) = scratch.make(m) {
                    if include(&scratch, m, &undo) {
                        out.push(m);
                    }
                    scratch.unmake(&undo);
                }
            }
        } else {
            self.board.generate_filtered(out, include);
        }
    }
    fn make(&mut self, m: Move) -> bool {
        if self.stack.is_empty() && self.root_allowed.as_ref().is_some_and(|a| !a.contains(&m)) {
            return false;
        }
        let Some(mut u) = self.board.make(m) else {
            return false;
        };
        u.check = self.check;
        self.stack.push(u);
        self.keys.push(self.board.repeat);
        self.check = self.board.can_win(self.board.side ^ 1);
        true
    }
    fn unmake(&mut self) {
        let u = self.stack.pop().expect("search undo");
        self.board.unmake(&u);
        self.keys.pop();
        self.check = u.check;
    }
    fn make_null(&mut self) {
        unreachable!("Tenjiku null move is not safe")
    }
    fn unmake_null(&mut self) {
        unreachable!("Tenjiku null move is not safe")
    }
    fn null_move_safe(&self) -> bool {
        false
    }
    fn rule_score(&self) -> Option<i32> {
        let ply = self.stack.len() as i32;
        if let Some(s) = self.board.terminal(ply) {
            return Some(s);
        }
        if self.board.can_win(self.board.side) {
            return Some(MATE_SCORE - ply - 1);
        }
        let current = self.keys.len() - 1;
        let mut occurrences =
            earlier_occurrences(&self.keys[..current], self.board.repeat, 2..=current);
        occurrences.next().map(|_| 0)
    }
    fn no_move_score(&self, _ply: i32) -> i32 {
        0
    }
    fn draw_claimable(&self) -> bool {
        false
    }
    fn evaluate(&self) -> i32 {
        let s = self.board.side;
        self.board.material[s] - self.board.material[s ^ 1] + self.board.placement[s]
            - self.board.placement[s ^ 1]
            + 10
    }
    fn is_tactical(&self, m: Move) -> bool {
        let mut b = self.board;
        if let Some(u) = b.make(m) {
            b.tactical_after(m, &u)
        } else {
            false
        }
    }
    fn tactical_gain(&self, m: Move) -> i32 {
        let mut b = self.board;
        b.make(m).map_or(0, |u| u.gain)
    }
    fn quiet_order(&self, m: Move) -> i32 {
        let piece = self.board.cells[m.from()];
        16 * (Board::placement(piece, m.to()) - Board::placement(piece, m.from()))
    }
    fn tactical_order(&self, m: Move) -> i32 {
        self.tactical_gain(m) * 16 - meta(self.board.cells[m.from()]).value / 16
    }
    fn exchange_at_least(&self, m: Move, threshold: i32) -> bool {
        let mut board = self.board;
        let Some(u) = board.make(m) else {
            return false;
        };
        if board.royals.iter().any(|r| r.count() == 0)
            || board.occupied.iter().any(|o| o.count() <= 1)
        {
            return true;
        }
        if board.cells[m.to()] == 0 {
            return u.gain >= threshold;
        }
        let mut work = ExchangeWork {
            projections: 63,
            routes: 2048,
            geometry: 4096,
        };
        board.exchange_greater(m.to(), u.gain - threshold, 0, &mut work) != Some(true)
    }
    fn history_slot(&self, m: Move) -> usize {
        let c = self.board.cells[m.from()];
        (meta(c).piece.side.index() * 44
            + Face::ALL.iter().position(|&f| f == meta(c).face).unwrap())
            * N
            + m.to()
    }
    fn encode(m: Move) -> u32 {
        m.0 + 1
    }
    fn decode(&self, bits: u32) -> Option<Move> {
        let m = Move(bits.checked_sub(1)?);
        self.is_pseudo_legal(m).then_some(m)
    }
    fn is_pseudo_legal(&self, m: Move) -> bool {
        self.board.geometry(m)
            && !(self.stack.is_empty()
                && self.root_allowed.as_ref().is_some_and(|a| !a.contains(&m)))
    }
}

#[cfg(test)]
mod exchange_bound_tests {
    use super::*;
    #[test]
    fn incomplete_exchange_work_keeps_unknown() {
        let mut b = Board::empty(Profile::MullerCompleted, 0);
        for (id, (at, owner, base)) in [
            (240, 0, Base::K),
            (255, 0, Base::P),
            (136, 0, Base::R),
            (0, 1, Base::K),
            (120, 1, Base::P),
        ]
        .into_iter()
        .enumerate()
        {
            b.set(
                at,
                code(Piece::new(id as u16, side(owner), base, false).unwrap()),
            );
        }
        for (projections, routes, geometry, depth) in [
            (0, 2048, 4096, 0),
            (64, 0, 4096, 0),
            (64, 2048, 0, 0),
            (64, 2048, 4096, 8),
        ] {
            let mut work = ExchangeWork {
                projections,
                routes,
                geometry,
            };
            assert_eq!(b.exchange_greater(120, 0, depth, &mut work), None);
        }
    }
}
