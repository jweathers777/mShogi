//! Game-independent classical alpha-beta search core.
//!
//! A game supplies a search position through [`SearchPosition`]: its fast board,
//! move generation, evaluation and rule approximations. This module owns
//! iterative deepening, principal-variation search, quiescence, the
//! transposition table, move ordering, selectivity and limits. It contains no
//! type or rule of a specific game.
//!
//! The search is private analysis. The game's native rules stay the authority
//! for real results, claims and histories; the caller checks the chosen move
//! against them.
pub mod table;

use std::time::{Duration, Instant};
use table::{Bound, TranspositionTable};

/// Score of a win at the root. A win found `n` plies from the root scores
/// `MATE_SCORE - n`; a loss scores the negative.
pub const MATE_SCORE: i32 = 1_000_000;
/// Maximum search ply from the root, including quiescence.
pub const MAX_PLY: usize = 128;
/// Scores with at least this absolute value are win/loss scores.
pub const MATE_BOUND: i32 = MATE_SCORE - 2 * MAX_PLY as i32;
const INF: i32 = MATE_SCORE + 1;

/// Move generation stage. The two stages together give each pseudo-legal move
/// one time.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stage {
    /// Moves that change material at once (captures, promotions to the main piece).
    Tactical,
    /// All other moves.
    Quiet,
}

/// A game's search-internal position.
///
/// Moves from `generate` are pseudo-legal. `make` rejects a move that is not
/// legal. The position owns its history of hashes, so that it can apply the
/// game's repetition and move-count rules in `rule_score`.
pub trait SearchPosition {
    /// Compact move. `Default` is only a filler value and is never played.
    type Move: Copy + Eq + std::fmt::Debug;
    /// Number of history slots; `history_slot` returns a value below this.
    const HISTORY_SLOTS: usize;

    /// Hash of the board state: equal for positions that have the same pieces,
    /// side to move and move rights, independent of the path.
    fn hash(&self) -> u64;
    /// True if the side to move is in check (or the game's equivalent).
    fn in_check(&self) -> bool;
    /// Append the pseudo-legal moves of one stage.
    fn generate(&self, stage: Stage, moves: &mut Vec<Self::Move>);
    /// Play a move. Returns false and leaves the position unchanged if the move
    /// is not legal.
    fn make(&mut self, m: Self::Move) -> bool;
    /// Take back the last move played with `make`.
    fn unmake(&mut self);
    /// Pass the turn (null move). Only called when `null_move_safe` is true.
    fn make_null(&mut self);
    /// Take back a null move.
    fn unmake_null(&mut self);
    /// False where a pass could be better than each move (zugzwang risk).
    fn null_move_safe(&self) -> bool;
    /// Score from the side to move if a rule already decides this position
    /// (repetition, move count, dead material), as a search approximation.
    fn rule_score(&self) -> Option<i32>;
    /// True if the side to move can end the game as a draw now by a rule
    /// claim (for example the fifty-move claim with an intended move). The
    /// search then scores the position as at least a draw for that side.
    fn draw_claimable(&self) -> bool;
    /// Score from the side to move when it has no legal move, `ply` plies from
    /// the root (for example a mate score or a stalemate draw).
    fn no_move_score(&self, ply: i32) -> i32;
    /// Static evaluation from the side to move. Not called when in check.
    fn evaluate(&self) -> i32;
    /// True for a move of the tactical stage.
    fn is_tactical(&self, m: Self::Move) -> bool;
    /// Ordering value of a tactical move; larger is tried first.
    fn tactical_order(&self, m: Self::Move) -> i32;
    /// Upper estimate of the material a tactical move gains at once.
    fn tactical_gain(&self, m: Self::Move) -> i32;
    /// True if the exchange that starts with this move gains at least
    /// `threshold`. A game without an exchange test returns true.
    fn exchange_at_least(&self, m: Self::Move, threshold: i32) -> bool;
    /// History slot of a move in this position (for example side, piece, target).
    fn history_slot(&self, m: Self::Move) -> usize;
    /// Nonzero table encoding of a move.
    fn encode(m: Self::Move) -> u32;
    /// Decode a table move. Returns None unless the move is pseudo-legal here.
    fn decode(&self, bits: u32) -> Option<Self::Move>;
    /// True if the move is pseudo-legal in this position.
    fn is_pseudo_legal(&self, m: Self::Move) -> bool;
}

/// Work limits of one search.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    /// Maximum nominal depth, at least 1.
    pub depth: u16,
    /// Stop before a node visit above this total.
    pub nodes: Option<u64>,
    /// Do not start a new iteration after this time.
    pub soft: Option<Duration>,
    /// Stop the search at this time.
    pub hard: Option<Duration>,
}

/// Why a search ended.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Status {
    /// The requested depth is complete.
    Complete,
    /// The node limit stopped the search.
    NodeLimit,
    /// The time limit or the stop request stopped the search.
    Stopped,
}

/// Result of one search.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Outcome<M> {
    /// Best move. None if no iteration gave a move.
    pub best: Option<M>,
    /// Score of the best move from the side to move.
    pub score: Option<i32>,
    /// Last fully completed depth.
    pub depth: u16,
    /// Largest ply reached, including quiescence.
    pub selective_depth: u16,
    /// Node visits, including quiescence.
    pub nodes: u64,
    /// Principal variation that starts with `best`.
    pub pv: Vec<M>,
    /// End condition.
    pub status: Status,
}

#[derive(Default)]
struct Buffers<M> {
    moves: Vec<M>,
    scores: Vec<i32>,
    bad: Vec<M>,
    tried: Vec<(M, usize)>,
}

struct Pick<M> {
    stage: u8,
    table_move: Option<M>,
    index: usize,
    killer: usize,
    in_check: bool,
    quiescence: bool,
}

const NO_SLOT: usize = usize::MAX;
const FUTILITY: [i32; 4] = [0, 150, 300, 450];
const HISTORY_LIMIT: i32 = 16_384;

fn gravity(slot: &mut i32, bonus: i32) {
    *slot += bonus - *slot * bonus.abs() / HISTORY_LIMIT;
}

/// Search state that the caller keeps between searches of one game: the
/// transposition table and the ordering heuristics.
pub struct Searcher<P: SearchPosition> {
    table: TranspositionTable,
    history: Vec<i32>,
    continuation: Vec<i32>,
    killers: Vec<[Option<P::Move>; 2]>,
    buffers: Vec<Buffers<P::Move>>,
    pv: Vec<Vec<P::Move>>,
    slots: Vec<usize>,
    reductions: Vec<[i32; 64]>,
}

impl<P: SearchPosition> Searcher<P> {
    /// Make a searcher with a table of about `megabytes`.
    pub fn new(megabytes: usize) -> Self {
        let mut reductions = vec![[0i32; 64]; 64];
        for (depth, row) in reductions.iter_mut().enumerate().skip(1) {
            for (count, r) in row.iter_mut().enumerate().skip(1) {
                *r = (0.75 + (depth as f64).ln() * (count as f64).ln() / 2.25) as i32;
            }
        }
        Self {
            table: TranspositionTable::new(megabytes, MATE_BOUND),
            history: vec![0; P::HISTORY_SLOTS],
            continuation: vec![0; P::HISTORY_SLOTS * P::HISTORY_SLOTS],
            killers: vec![[None; 2]; MAX_PLY + 2],
            buffers: (0..MAX_PLY + 2)
                .map(|_| Buffers {
                    moves: Vec::with_capacity(128),
                    scores: Vec::with_capacity(128),
                    bad: Vec::with_capacity(32),
                    tried: Vec::with_capacity(64),
                })
                .collect(),
            pv: (0..MAX_PLY + 2)
                .map(|_| Vec::with_capacity(MAX_PLY + 2))
                .collect(),
            slots: vec![NO_SLOT; MAX_PLY + 2],
            reductions,
        }
    }
    /// Table size in megabytes.
    pub fn table_megabytes(&self) -> usize {
        self.table.megabytes()
    }
    /// Replace the table with an empty one of a new size.
    pub fn resize_table(&mut self, megabytes: usize) {
        self.table = TranspositionTable::new(megabytes, MATE_BOUND);
    }
    /// Forget all results: use at the start of a new game.
    pub fn clear(&mut self) {
        self.table.clear();
        self.history.fill(0);
        self.continuation.fill(0);
        self.killers.fill([None; 2]);
    }
    /// Permille of sampled table entries written by the last search.
    pub fn hashfull(&self) -> u32 {
        self.table.hashfull()
    }

    /// Search `position` and return the best move found. The position is
    /// restored before return. `stop` is polled about each 2,000 nodes and
    /// must be cheap.
    pub fn search(
        &mut self,
        position: &mut P,
        limits: Limits,
        stop: &mut dyn FnMut() -> bool,
    ) -> Outcome<P::Move> {
        self.table.new_search();
        self.killers.fill([None; 2]);
        for h in &mut self.history {
            *h /= 2;
        }
        let mut run = Run {
            s: self,
            pos: position,
            stop,
            start: Instant::now(),
            limits,
            nodes: 0,
            seldepth: 0,
            abort: false,
            status: Status::Complete,
            root_move: None,
        };
        run.iterate()
    }
}

struct Run<'a, P: SearchPosition> {
    s: &'a mut Searcher<P>,
    pos: &'a mut P,
    stop: &'a mut dyn FnMut() -> bool,
    start: Instant,
    limits: Limits,
    nodes: u64,
    seldepth: usize,
    abort: bool,
    status: Status,
    /// Best root move of the current window, with its score, once one root
    /// move has raised alpha.
    root_move: Option<(P::Move, i32)>,
}

impl<P: SearchPosition> Run<'_, P> {
    fn iterate(&mut self) -> Outcome<P::Move> {
        let mut outcome = Outcome {
            best: None,
            score: None,
            depth: 0,
            selective_depth: 0,
            nodes: 0,
            pv: Vec::new(),
            status: Status::Complete,
        };
        if self.limits.nodes == Some(0)
            || self.limits.hard.is_some_and(|hard| hard.is_zero())
            || (self.stop)()
        {
            outcome.status = if self.limits.nodes == Some(0) {
                Status::NodeLimit
            } else {
                Status::Stopped
            };
            return outcome;
        }
        for depth in 1..=i32::from(self.limits.depth.max(1)).min(MAX_PLY as i32 - 1) {
            let previous = outcome.score.unwrap_or(0);
            let mut delta = 30;
            let (mut alpha, mut beta) = if depth >= 4 && previous.abs() < MATE_BOUND {
                (previous - delta, previous + delta)
            } else {
                (-INF, INF)
            };
            let score = loop {
                self.root_move = None;
                let score = self.search(depth, 0, alpha, beta, true, false);
                if self.abort {
                    break score;
                }
                if score <= alpha {
                    beta = (alpha + beta) / 2;
                    alpha = (score - delta).max(-INF);
                } else if score >= beta {
                    beta = (score + delta).min(INF);
                } else {
                    break score;
                }
                delta = delta.saturating_mul(2);
                if delta > 2_000 {
                    alpha = -INF;
                    beta = INF;
                }
            };
            if self.abort {
                // A root move that raised alpha in the stopped iteration is at
                // least as good as the earlier best move at this larger depth.
                // Without one complete iteration there is no result to improve.
                if outcome.depth >= 1
                    && let Some((m, s)) = self.root_move
                {
                    if outcome.best != Some(m) {
                        outcome.pv = vec![m];
                    }
                    outcome.best = Some(m);
                    outcome.score = Some(s);
                }
                break;
            }
            outcome.score = Some(score);
            outcome.depth = depth as u16;
            outcome.pv.clear();
            outcome.pv.extend_from_slice(&self.s.pv[0]);
            outcome.best = outcome.pv.first().copied();
            if outcome.best.is_none() {
                break;
            }
            if self
                .limits
                .soft
                .is_some_and(|soft| self.start.elapsed() >= soft)
            {
                self.status = Status::Stopped;
                break;
            }
        }
        outcome.status = self.status;
        outcome.nodes = self.nodes;
        outcome.selective_depth = self.seldepth as u16;
        outcome
    }

    /// Count a node visit. False means the search must stop.
    #[inline]
    fn visit(&mut self, ply: usize) -> bool {
        if self.limits.nodes.is_some_and(|limit| self.nodes >= limit) {
            self.abort = true;
            self.status = Status::NodeLimit;
            return false;
        }
        self.nodes += 1;
        if ply > self.seldepth {
            self.seldepth = ply;
        }
        if self.nodes & 2047 == 0
            && ((self.stop)()
                || self
                    .limits
                    .hard
                    .is_some_and(|hard| self.start.elapsed() >= hard))
        {
            self.abort = true;
            self.status = Status::Stopped;
            return false;
        }
        true
    }

    fn quiet_score(&self, ply: usize, slot: usize) -> i32 {
        let mut score = self.s.history[slot];
        if ply > 0 && self.s.slots[ply - 1] != NO_SLOT {
            score += self.s.continuation[self.s.slots[ply - 1] * P::HISTORY_SLOTS + slot];
        }
        score
    }

    fn next_move(&mut self, ply: usize, pick: &mut Pick<P::Move>) -> Option<P::Move> {
        loop {
            match pick.stage {
                0 => {
                    pick.stage = 1;
                    if let Some(m) = pick.table_move {
                        return Some(m);
                    }
                }
                1 => {
                    let buffers = &mut self.s.buffers[ply];
                    buffers.moves.clear();
                    buffers.scores.clear();
                    buffers.bad.clear();
                    self.pos.generate(Stage::Tactical, &mut buffers.moves);
                    for &m in &buffers.moves {
                        buffers.scores.push(self.pos.tactical_order(m));
                    }
                    pick.index = 0;
                    pick.stage = 2;
                }
                2 => {
                    let buffers = &mut self.s.buffers[ply];
                    if pick.index >= buffers.moves.len() {
                        pick.stage = if pick.quiescence && !pick.in_check {
                            7
                        } else if pick.quiescence {
                            4
                        } else {
                            3
                        };
                        continue;
                    }
                    let mut best = pick.index;
                    for i in pick.index + 1..buffers.moves.len() {
                        if buffers.scores[i] > buffers.scores[best] {
                            best = i;
                        }
                    }
                    buffers.moves.swap(pick.index, best);
                    buffers.scores.swap(pick.index, best);
                    let m = buffers.moves[pick.index];
                    pick.index += 1;
                    if Some(m) == pick.table_move {
                        continue;
                    }
                    if !pick.in_check && !self.pos.exchange_at_least(m, 0) {
                        if !pick.quiescence {
                            buffers.bad.push(m);
                        }
                        continue;
                    }
                    return Some(m);
                }
                3 => {
                    if pick.killer >= 2 {
                        pick.stage = 4;
                        continue;
                    }
                    let killer = self.s.killers[ply][pick.killer];
                    pick.killer += 1;
                    if let Some(m) = killer
                        && Some(m) != pick.table_move
                        && self.pos.is_pseudo_legal(m)
                        && !self.pos.is_tactical(m)
                    {
                        return Some(m);
                    }
                }
                4 => {
                    let previous = if ply > 0 {
                        self.s.slots[ply - 1]
                    } else {
                        NO_SLOT
                    };
                    let buffers = &mut self.s.buffers[ply];
                    buffers.moves.clear();
                    buffers.scores.clear();
                    self.pos.generate(Stage::Quiet, &mut buffers.moves);
                    for &m in &buffers.moves {
                        let slot = self.pos.history_slot(m);
                        let mut score = self.s.history[slot];
                        if previous != NO_SLOT {
                            score += self.s.continuation[previous * P::HISTORY_SLOTS + slot];
                        }
                        buffers.scores.push(score);
                    }
                    pick.index = 0;
                    pick.stage = 5;
                }
                5 => {
                    let buffers = &mut self.s.buffers[ply];
                    if pick.index >= buffers.moves.len() {
                        pick.index = 0;
                        pick.stage = 6;
                        continue;
                    }
                    let mut best = pick.index;
                    for i in pick.index + 1..buffers.moves.len() {
                        if buffers.scores[i] > buffers.scores[best] {
                            best = i;
                        }
                    }
                    buffers.moves.swap(pick.index, best);
                    buffers.scores.swap(pick.index, best);
                    let m = buffers.moves[pick.index];
                    pick.index += 1;
                    if Some(m) == pick.table_move
                        || (!pick.quiescence && self.s.killers[ply].contains(&Some(m)))
                    {
                        continue;
                    }
                    return Some(m);
                }
                6 => {
                    let buffers = &self.s.buffers[ply];
                    if pick.index >= buffers.bad.len() {
                        pick.stage = 7;
                        continue;
                    }
                    pick.index += 1;
                    return Some(buffers.bad[pick.index - 1]);
                }
                _ => return None,
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn search(
        &mut self,
        mut depth: i32,
        ply: usize,
        mut alpha: i32,
        mut beta: i32,
        pv: bool,
        allow_null: bool,
    ) -> i32 {
        let in_check = self.pos.in_check();
        if in_check {
            depth = depth.max(0) + 1;
        }
        if depth <= 0 {
            return self.quiesce(ply, alpha, beta, pv);
        }
        if !self.visit(ply) {
            return 0;
        }
        self.s.pv[ply].clear();
        let root = ply == 0;
        if !root {
            if let Some(score) = self.pos.rule_score() {
                return score;
            }
            if ply >= MAX_PLY - 1 {
                return if in_check { 0 } else { self.pos.evaluate() };
            }
            alpha = alpha.max(-MATE_SCORE + ply as i32);
            beta = beta.min(MATE_SCORE - ply as i32 - 1);
            if alpha >= beta {
                return alpha;
            }
        }
        // A draw that the side to move can claim is a lower bound of its score.
        let claimable = !root && self.pos.draw_claimable();
        if claimable {
            if beta <= 0 {
                return 0;
            }
            alpha = alpha.max(0);
        }
        let hash = self.pos.hash();
        let hit = self.s.table.probe(hash, ply as i32);
        let mut table_move = None;
        if let Some(hit) = hit {
            if hit.move_bits != 0 {
                table_move = self.pos.decode(hit.move_bits);
            }
            if !pv
                && hit.depth >= depth
                && match hit.bound {
                    Bound::Exact => true,
                    Bound::Lower => hit.score >= beta,
                    Bound::Upper => hit.score <= alpha,
                }
            {
                return hit.score;
            }
        }
        let eval = if in_check {
            None
        } else {
            Some(
                hit.and_then(|h| h.eval)
                    .unwrap_or_else(|| self.pos.evaluate()),
            )
        };
        if let Some(eval) = eval
            && !pv
            && allow_null
            && depth >= 3
            && eval >= beta
            && beta.abs() < MATE_BOUND
            && self.pos.null_move_safe()
        {
            let reduction = 3 + depth / 4 + ((eval - beta) / 200).min(3);
            self.pos.make_null();
            self.s.slots[ply] = NO_SLOT;
            let score = -self.search(
                depth - 1 - reduction,
                ply + 1,
                -beta,
                -beta + 1,
                false,
                false,
            );
            self.pos.unmake_null();
            if self.abort {
                return 0;
            }
            if score >= beta {
                let score = if score >= MATE_BOUND { beta } else { score };
                if depth < 10 {
                    return score;
                }
                let verified =
                    self.search(depth - 1 - reduction, ply, beta - 1, beta, false, false);
                if self.abort {
                    return 0;
                }
                if verified >= beta {
                    return score;
                }
                self.s.pv[ply].clear();
            }
        }
        let futile = !pv
            && depth <= 3
            && alpha.abs() < MATE_BOUND
            && eval.is_some_and(|e| e + FUTILITY[depth as usize] <= alpha);
        let mut pick = Pick {
            stage: 0,
            table_move,
            index: 0,
            killer: 0,
            in_check,
            quiescence: false,
        };
        let original_alpha = alpha;
        let mut best = if claimable { 0 } else { -INF };
        let mut best_move = None;
        let mut count = 0usize;
        self.s.buffers[ply].tried.clear();
        while let Some(m) = self.next_move(ply, &mut pick) {
            let tactical = self.pos.is_tactical(m);
            let slot = self.pos.history_slot(m);
            if !self.pos.make(m) {
                continue;
            }
            count += 1;
            let gives_check = self.pos.in_check();
            if futile && count > 1 && !tactical && !gives_check {
                self.pos.unmake();
                continue;
            }
            self.s.slots[ply] = slot;
            let new_depth = depth - 1;
            let mut score;
            if count == 1 {
                score = -self.search(new_depth, ply + 1, -beta, -alpha, pv, true);
            } else {
                let mut reduction = 0;
                if depth >= 3
                    && count >= 3 + usize::from(pv)
                    && !tactical
                    && !in_check
                    && !gives_check
                {
                    reduction = self.s.reductions[depth.min(63) as usize][count.min(63)];
                    reduction -= i32::from(pv);
                    reduction -= i32::from(self.s.killers[ply].contains(&Some(m)));
                    reduction -= (self.quiet_score(ply, slot) / 8_192).clamp(-2, 2);
                    reduction = reduction.clamp(0, (new_depth - 1).max(0));
                }
                score = -self.search(
                    new_depth - reduction,
                    ply + 1,
                    -alpha - 1,
                    -alpha,
                    false,
                    true,
                );
                if !self.abort && reduction > 0 && score > alpha {
                    score = -self.search(new_depth, ply + 1, -alpha - 1, -alpha, false, true);
                }
                if !self.abort && pv && score > alpha {
                    score = -self.search(new_depth, ply + 1, -beta, -alpha, true, true);
                }
            }
            self.pos.unmake();
            if self.abort {
                return 0;
            }
            if !tactical {
                self.s.buffers[ply].tried.push((m, slot));
            }
            if score > best {
                best = score;
                if score > alpha {
                    best_move = Some(m);
                    alpha = score;
                    if pv {
                        let (head, tail) = self.s.pv.split_at_mut(ply + 1);
                        head[ply].clear();
                        head[ply].push(m);
                        head[ply].extend_from_slice(&tail[0]);
                    }
                    if root {
                        self.root_move = Some((m, score));
                    }
                    if alpha >= beta {
                        if !tactical {
                            self.reward(ply, m, slot, depth);
                        }
                        break;
                    }
                }
            }
        }
        if count == 0 {
            return self.pos.no_move_score(ply as i32);
        }
        let bound = if best >= beta {
            Bound::Lower
        } else if best > original_alpha {
            Bound::Exact
        } else {
            Bound::Upper
        };
        self.s.table.store(
            hash,
            ply as i32,
            best_move.map_or(0, P::encode),
            best,
            eval,
            depth,
            bound,
        );
        best
    }

    fn reward(&mut self, ply: usize, m: P::Move, slot: usize, depth: i32) {
        let killers = &mut self.s.killers[ply];
        if killers[0] != Some(m) {
            killers[1] = killers[0];
            killers[0] = Some(m);
        }
        let bonus = (depth * depth * 16).min(2_000);
        let previous = if ply > 0 {
            self.s.slots[ply - 1]
        } else {
            NO_SLOT
        };
        gravity(&mut self.s.history[slot], bonus);
        if previous != NO_SLOT {
            gravity(
                &mut self.s.continuation[previous * P::HISTORY_SLOTS + slot],
                bonus,
            );
        }
        // Quiet moves tried before the cutoff move did not cut off.
        let tried = std::mem::take(&mut self.s.buffers[ply].tried);
        for &(other, other_slot) in &tried {
            if other == m {
                continue;
            }
            gravity(&mut self.s.history[other_slot], -bonus);
            if previous != NO_SLOT {
                gravity(
                    &mut self.s.continuation[previous * P::HISTORY_SLOTS + other_slot],
                    -bonus,
                );
            }
        }
        self.s.buffers[ply].tried = tried;
    }

    fn quiesce(&mut self, ply: usize, mut alpha: i32, beta: i32, pv: bool) -> i32 {
        if !self.visit(ply) {
            return 0;
        }
        self.s.pv[ply].clear();
        if let Some(score) = self.pos.rule_score() {
            return score;
        }
        let in_check = self.pos.in_check();
        if ply >= MAX_PLY - 1 {
            return if in_check { 0 } else { self.pos.evaluate() };
        }
        let hash = self.pos.hash();
        let hit = self.s.table.probe(hash, ply as i32);
        let mut table_move = None;
        if let Some(hit) = hit {
            if !pv
                && match hit.bound {
                    Bound::Exact => true,
                    Bound::Lower => hit.score >= beta,
                    Bound::Upper => hit.score <= alpha,
                }
            {
                return hit.score;
            }
            if hit.move_bits != 0 {
                table_move = self
                    .pos
                    .decode(hit.move_bits)
                    .filter(|&m| in_check || self.pos.is_tactical(m));
            }
        }
        let original_alpha = alpha;
        let eval = if in_check {
            None
        } else {
            Some(
                hit.and_then(|h| h.eval)
                    .unwrap_or_else(|| self.pos.evaluate()),
            )
        };
        let mut best = if self.pos.draw_claimable() { 0 } else { -INF };
        if let Some(eval) = eval {
            best = best.max(eval);
        }
        if best >= beta {
            return best;
        }
        alpha = alpha.max(best);
        let mut pick = Pick {
            stage: 0,
            table_move,
            index: 0,
            killer: 0,
            in_check,
            quiescence: true,
        };
        let mut best_move = None;
        let mut count = 0usize;
        while let Some(m) = self.next_move(ply, &mut pick) {
            if let Some(eval) = eval {
                let reach = eval + self.pos.tactical_gain(m) + 200;
                if reach <= alpha {
                    best = best.max(reach);
                    continue;
                }
            }
            if !self.pos.make(m) {
                continue;
            }
            count += 1;
            self.s.slots[ply] = NO_SLOT;
            let score = -self.quiesce(ply + 1, -beta, -alpha, pv);
            self.pos.unmake();
            if self.abort {
                return 0;
            }
            if score > best {
                best = score;
                if score > alpha {
                    best_move = Some(m);
                    alpha = score;
                    if pv {
                        let (head, tail) = self.s.pv.split_at_mut(ply + 1);
                        head[ply].clear();
                        head[ply].push(m);
                        head[ply].extend_from_slice(&tail[0]);
                    }
                    if alpha >= beta {
                        break;
                    }
                }
            }
        }
        if in_check && count == 0 {
            return self.pos.no_move_score(ply as i32);
        }
        let bound = if best >= beta {
            Bound::Lower
        } else if best > original_alpha {
            Bound::Exact
        } else {
            Bound::Upper
        };
        self.s.table.store(
            hash,
            ply as i32,
            best_move.map_or(0, P::encode),
            best,
            eval,
            0,
            bound,
        );
        best
    }
}

#[cfg(test)]
mod tests;
