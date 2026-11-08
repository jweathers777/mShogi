//! Bounded conventional Chess: native history, PVS, quiescence and guarded selectivity.
//!
//! The exact diagnostic mode disables quiescence/extensions/selective pruning.
//! Conventional scores are selective heuristic observations, never adjudication.
use crate::{
    chess::piece_value,
    chess_conventional_eval::{self as evaluation, EvalState},
    search::{
        MATE_SCORE, MAX_DEPTH, SearchChoice, SearchDecision, SearchError, SearchLimits,
        SearchResult, SearchStatus,
    },
};
use mshogi_core::chess::{
    Action, AdjudicatedGame, AdjudicationMode, ChessOutcome, ClaimPolicy, Classification, Color,
    PieceKind, Position, Promotion, SearchCursor, SearchKey, Square,
};
use std::{
    cmp::Reverse,
    time::{Duration, Instant},
};

/// Versioned search family; the former exact implementations remain diagnostic references.
pub const SEARCH_ID: &str = "mshogi.chess.conventional.v1";
/// Maximum actual board plies, including quiescence and check extensions.
pub const MAX_PLY: u16 = 128;
const INF: i32 = MATE_SCORE + 1;
const MATE_BAND: i32 = MATE_SCORE - MAX_PLY as i32;
const Q_PLIES: u16 = 16;
const TABLE_ENTRIES: usize = 4096;

/// Soft iteration boundary and hard cooperative deadline, independent of watchdogs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeBudget {
    /// Stop starting iterations after this duration.
    pub soft: Duration,
    /// Poll before native nodes and exchange-ordering work.
    pub hard: Duration,
}
impl TimeBudget {
    /// Conservative allocation within a caller's exact hard per-move allowance.
    pub fn from_hard_ms(ms: u64) -> Self {
        Self {
            soft: Duration::from_millis(ms.saturating_mul(7) / 10),
            hard: Duration::from_millis(ms),
        }
    }
}

/// Exact diagnostic backup or the integrated selective conventional engine.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SearchMode {
    /// PVS/ordering/TT, with static depth leaves and no selective approximations.
    Exact,
    /// Quiescence, bounded check extensions and guarded conventional pruning.
    Conventional,
}
/// Bounded per-invocation configuration; no table or history leaks between requests.
#[derive(Clone, Copy, Debug)]
pub struct SearchOptions {
    /// Diagnostic versus playing algorithm.
    pub mode: SearchMode,
    /// Optional soft/hard wall-time allocation.
    pub time: Option<TimeBudget>,
    /// Direct-mapped entries, clamped to4096; zero disables table reuse.
    pub table_entries: usize,
}
impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            mode: SearchMode::Conventional,
            time: None,
            table_entries: TABLE_ENTRIES,
        }
    }
}
/// Inspectable algorithm work, not a strength estimate.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SearchStats {
    /// Main-search visits.
    pub main_nodes: u64,
    /// Quiescence visits.
    pub q_nodes: u64,
    /// Exact-context transposition hits.
    pub tt_hits: u64,
    /// Real main-node table lookups, including empty slots.
    pub tt_probes: u64,
    /// Occupied lookups whose complete position hash matches (hint eligibility).
    pub tt_position_hits: u64,
    /// Lookups with exact full-history equality, before depth/extension checks.
    pub tt_context_hits: u64,
    /// Completed real-node entries stored, including replacements.
    pub tt_stores: u64,
    /// Bound cutoffs using an exact full-history key.
    pub tt_cutoffs: u64,
    /// Aspiration-window widening/re-searches.
    pub aspiration_researches: u64,
    /// PVS/reduction re-searches.
    pub researches: u64,
    /// Late quiet move reductions.
    pub reductions: u64,
    /// Detached hypothetical null probes.
    pub null_probes: u64,
    /// Real-node verified non-mating null cutoffs.
    pub null_verified: u64,
    /// Hypothetical probes abandoned after a synthetic representation failure.
    pub null_abandoned: u64,
    /// Bounded check extensions.
    pub extensions: u64,
    /// Conservative late quiet/futility omissions.
    pub futility_pruned: u64,
    /// Guarded late quiet omissions after the shallow move-count threshold.
    pub late_move_pruned: u64,
    /// Losing exchange or delta omissions.
    pub see_pruned: u64,
    /// Bounded native legal-exchange estimates.
    pub see_probes: u64,
}
/// Existing nominal result plus honest actual selective-depth and algorithm work.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConventionalResult {
    /// Last completed nominal iteration; PV can include quiescent continuation.
    pub result: SearchResult,
    /// Deepest actual visited board ply across all attempted iterations.
    pub selective_depth: u16,
    /// Inspectable bounded work counters.
    pub stats: SearchStats,
}

/// Search with ordinary deterministic depth/node ceilings.
pub fn search(
    game: &AdjudicatedGame,
    limits: SearchLimits,
) -> Result<ConventionalResult, SearchError> {
    search_with_stop(game, limits, || false)
}
/// Search with cancellation and no wall-time limit.
pub fn search_with_stop(
    game: &AdjudicatedGame,
    limits: SearchLimits,
    stop: impl FnMut() -> bool,
) -> Result<ConventionalResult, SearchError> {
    search_with_time(game, limits, None, stop)
}
/// Shared timed entrypoint for UCI and supervised native requests.
pub fn search_with_time(
    game: &AdjudicatedGame,
    limits: SearchLimits,
    time: Option<TimeBudget>,
    stop: impl FnMut() -> bool,
) -> Result<ConventionalResult, SearchError> {
    search_with_options(
        game,
        limits,
        SearchOptions {
            time,
            ..SearchOptions::default()
        },
        stop,
    )
}
/// Independently testable configured engine, with a fresh bounded table per request.
pub fn search_with_options(
    game: &AdjudicatedGame,
    limits: SearchLimits,
    options: SearchOptions,
    mut stop: impl FnMut() -> bool,
) -> Result<ConventionalResult, SearchError> {
    if !(1..=MAX_DEPTH).contains(&limits.depth) {
        return Err(SearchError::InvalidDepth(limits.depth));
    }
    if options.time.is_some() && !matches!(game.adjudication_mode(), AdjudicationMode::M2) {
        return Err(SearchError::TimedFullUnsupported);
    }
    let mut engine = Engine {
        cursor: SearchCursor::new(game),
        eval: EvalState::new(game.position()),
        work: Work {
            nodes: 0,
            limit: limits.nodes,
            start: Instant::now(),
            time: options.time,
            stop: &mut stop,
            seldepth: 0,
        },
        mode: options.mode,
        table: vec![None; options.table_entries.min(TABLE_ENTRIES)],
        history: vec![0; 2 * 64 * 64],
        continuation: vec![0; 64 * 64],
        captures: vec![0; 2 * 6 * 64],
        killers: vec![[None; 2]; usize::from(MAX_PLY) + 1],
        stats: SearchStats::default(),
    };
    let mut result = SearchResult {
        decision: None,
        score: None,
        completed_depth: 0,
        nodes: 0,
        principal_variation: vec![],
        status: SearchStatus::Complete,
    };
    for depth in 1..=limits.depth {
        let mut width = if depth >= 3 && result.score.is_some_and(|s| s.abs() < MATE_BAND) {
            40
        } else {
            INF
        };
        let center = result.score.unwrap_or(0);
        let iteration = loop {
            let alpha = if width == INF {
                -INF
            } else {
                (center - width).max(-INF)
            };
            let beta = if width == INF {
                INF
            } else {
                (center + width).min(INF)
            };
            match engine.main(i32::from(depth), 0, alpha, beta, true, true, 4, None) {
                Ok(value) if width < INF && (value.score <= alpha || value.score >= beta) => {
                    engine.stats.aspiration_researches += 1;
                    width = width.saturating_mul(4).min(INF);
                }
                other => break other,
            }
        };
        debug_assert_eq!(engine.cursor.game(), Some(game));
        debug_assert_eq!(
            engine.eval.evaluate(game.position()),
            evaluation::evaluate(game.position())
        );
        match iteration {
            Ok(value) => {
                result.decision = value
                    .pv
                    .first()
                    .copied()
                    .map(SearchDecision::Choose)
                    .or_else(|| game.outcome().map(SearchDecision::Finished));
                result.score = Some(value.score);
                result.completed_depth = depth;
                result.principal_variation = value.pv;
                if matches!(result.decision, Some(SearchDecision::Finished(_))) {
                    break;
                }
                if engine
                    .work
                    .time
                    .is_some_and(|b| engine.work.start.elapsed() >= b.soft)
                {
                    result.status = SearchStatus::Stopped;
                    break;
                }
            }
            Err(Halt::Limit(status)) => {
                result.status = status;
                break;
            }
            Err(Halt::Error(error)) => return Err(error),
        }
    }
    result.nodes = engine.work.nodes;
    Ok(ConventionalResult {
        result,
        selective_depth: engine.work.seldepth,
        stats: engine.stats,
    })
}

#[derive(Clone)]
struct Value {
    score: i32,
    pv: Vec<SearchChoice>,
}
impl Value {
    fn leaf(score: i32) -> Self {
        Self { score, pv: vec![] }
    }
    fn prepend(action: Action, child: Self) -> Self {
        let mut pv = vec![SearchChoice::Move(action)];
        pv.extend(child.pv);
        Self {
            score: -child.score,
            pv,
        }
    }
}
enum Halt {
    Limit(SearchStatus),
    Error(SearchError),
}
struct Work<'a> {
    nodes: u64,
    limit: Option<u64>,
    start: Instant,
    time: Option<TimeBudget>,
    stop: &'a mut dyn FnMut() -> bool,
    seldepth: u16,
}
impl Work<'_> {
    fn poll(&mut self) -> Result<(), Halt> {
        if (self.stop)() || self.time.is_some_and(|t| self.start.elapsed() >= t.hard) {
            return Err(Halt::Limit(SearchStatus::Stopped));
        }
        Ok(())
    }
    fn visit(&mut self, ply: u16) -> Result<(), Halt> {
        self.poll()?;
        if self.limit.is_some_and(|n| self.nodes >= n) {
            return Err(Halt::Limit(SearchStatus::NodeLimit));
        }
        self.nodes += 1;
        self.seldepth = self.seldepth.max(ply);
        Ok(())
    }
}
#[derive(Clone, Copy)]
enum Bound {
    Exact,
    Lower,
    Upper,
}
#[derive(Clone)]
struct Entry {
    key: SearchKey,
    hash: u64,
    depth: i32,
    extensions: u8,
    score: i32,
    bound: Bound,
    value: Value,
}
struct Engine<'a> {
    cursor: SearchCursor,
    eval: EvalState,
    work: Work<'a>,
    mode: SearchMode,
    table: Vec<Option<Entry>>,
    history: Vec<i32>,
    continuation: Vec<i32>,
    captures: Vec<i32>,
    killers: Vec<[Option<Action>; 2]>,
    stats: SearchStats,
}
#[derive(Clone, Copy)]
struct Ordered {
    action: Action,
    tactical: bool,
    gain: i32,
    see: i32,
    quiet_quality: i32,
}
fn square_index(s: Square) -> usize {
    usize::from(s.rank()) * 8 + usize::from(s.file())
}
fn kind_index(p: PieceKind) -> usize {
    match p {
        PieceKind::Pawn => 0,
        PieceKind::Knight => 1,
        PieceKind::Bishop => 2,
        PieceKind::Rook => 3,
        PieceKind::Queen => 4,
        PieceKind::King => 5,
    }
}
fn color_index(c: Color) -> usize {
    usize::from(c == Color::Black)
}
fn lexical(a: Action) -> (u8, u8, u8, u8, u8) {
    (
        a.from.file(),
        a.from.rank(),
        a.to.file(),
        a.to.rank(),
        match a.promotion {
            None => 0,
            Some(Promotion::Bishop) => 1,
            Some(Promotion::Knight) => 2,
            Some(Promotion::Queen) => 3,
            Some(Promotion::Rook) => 4,
        },
    )
}
fn gain(position: &Position, a: Action) -> i32 {
    let captured = position.piece_at(a.to).map_or_else(
        || {
            if position
                .piece_at(a.from)
                .is_some_and(|p| p.kind == PieceKind::Pawn)
                && a.from.file() != a.to.file()
            {
                100
            } else {
                0
            }
        },
        |p| piece_value(p.kind),
    );
    captured
        + a.promotion.map_or(0, |p| {
            piece_value(match p {
                Promotion::Queen => PieceKind::Queen,
                Promotion::Rook => PieceKind::Rook,
                Promotion::Bishop => PieceKind::Bishop,
                Promotion::Knight => PieceKind::Knight,
            }) - 100
        })
}

// Legal least-attacker exchange sequence, with an explicit16-capture bound.
// An ordering/pruning estimate only, never legality, effect resolution or proof.
fn exchange(position: &Position, a: Action, left: u8, work: &mut Work<'_>) -> Result<i32, Halt> {
    work.poll()?;
    let immediate = gain(position, a);
    if left == 0 {
        return Ok(immediate);
    }
    let mut after = position.clone();
    if after.apply(a).is_err() {
        return Ok(0);
    } // Actual search push reports representation errors.
    let reply = after.legal_actions_to(a.to).into_iter().min_by_key(|r| {
        (
            after.piece_at(r.from).map_or(0, |p| {
                if p.kind == PieceKind::King {
                    20_000
                } else {
                    piece_value(p.kind)
                }
            }),
            lexical(*r),
        )
    });
    Ok(immediate
        - if let Some(reply) = reply {
            exchange(&after, reply, left - 1, work)?.max(0)
        } else {
            0
        })
}

impl Engine<'_> {
    fn evaluate(&self) -> i32 {
        self.eval
            .evaluate(self.cursor.position())
            .score_for(self.cursor.position().side_to_move())
    }
    fn terminal(&mut self, ply: u16) -> Option<Value> {
        let side = self.cursor.position().side_to_move();
        let node = self.cursor.node();
        if let Some(outcome) = node.outcome {
            return Some(Value::leaf(match outcome {
                ChessOutcome::Draw(_) => 0,
                ChessOutcome::Checkmate { winner } => {
                    if winner == side {
                        MATE_SCORE - i32::from(ply)
                    } else {
                        -MATE_SCORE + i32::from(ply)
                    }
                }
            }));
        }
        // Hypothetical null probes cannot report a native Game conclusion.
        if node.synthetic && node.actions.is_empty() {
            return Some(Value::leaf(
                if node.classification == Classification::Checkmate {
                    -MATE_SCORE + i32::from(ply)
                } else {
                    0
                },
            ));
        }
        None
    }
    fn claim(&mut self) -> Option<(SearchChoice, bool)> {
        let forced = self
            .cursor
            .game()
            .is_some_and(|g| g.history().claim_policy() == ClaimPolicy::AutoClaim);
        self.cursor
            .node()
            .claims
            .first()
            .copied()
            .map(|c| (SearchChoice::Claim(c), forced))
    }
    fn push(&mut self, a: Action) -> Result<(), Halt> {
        let before = self.cursor.position().clone();
        self.cursor
            .push(a)
            .map_err(|e| Halt::Error(SearchError::Transition(e)))?;
        self.eval.push(&before, self.cursor.position());
        Ok(())
    }
    fn pop(&mut self) {
        assert!(self.eval.pop());
        assert!(self.cursor.pop());
    }
    fn table_index(&self, hash: u64) -> Option<usize> {
        (!self.table.is_empty()).then(|| (hash as usize) % self.table.len())
    }
    fn ordered(
        &mut self,
        actions: Vec<Action>,
        hint: Option<Action>,
        ply: u16,
        previous: Option<Action>,
    ) -> Result<Vec<Ordered>, Halt> {
        let position = self.cursor.position();
        let side = color_index(position.side_to_move());
        let mut moves = Vec::with_capacity(actions.len());
        for a in actions {
            self.work.poll()?;
            let g = gain(position, a);
            let tactical = g > 0 || a.promotion.is_some();
            let see = if tactical {
                self.stats.see_probes += 1;
                exchange(position, a, 16, &mut self.work)?
            } else {
                0
            };
            let stage = if Some(a) == hint {
                0
            } else if tactical && see >= 0 {
                1
            } else if !tactical && self.killers[usize::from(ply)].contains(&Some(a)) {
                2
            } else if !tactical {
                3
            } else {
                4
            };
            let pc = position.piece_at(a.from).expect("native mover");
            let score = if tactical {
                g * 16
                    - if pc.kind == PieceKind::King {
                        20_000
                    } else {
                        piece_value(pc.kind)
                    }
                    + self.captures[(side * 6 + kind_index(pc.kind)) * 64 + square_index(a.to)]
            } else {
                self.history[(side * 64 + square_index(a.from)) * 64 + square_index(a.to)]
                    + previous.map_or(0, |p| {
                        self.continuation[square_index(p.to) * 64 + square_index(a.to)]
                    })
            };
            moves.push((
                (stage, Reverse(score), lexical(a)),
                Ordered {
                    action: a,
                    tactical,
                    gain: g,
                    see,
                    quiet_quality: if tactical { 0 } else { score },
                },
            ));
        }
        moves.sort_by_key(|(key, _)| *key);
        Ok(moves.into_iter().map(|(_, m)| m).collect())
    }
    fn reward(&mut self, m: Ordered, depth: i32, ply: u16, previous: Option<Action>) {
        if self.cursor.node().synthetic {
            return;
        }
        let side = color_index(self.cursor.position().side_to_move());
        let bonus = (depth * depth * 24).min(1500);
        let slot = if m.tactical {
            let pc = self
                .cursor
                .position()
                .piece_at(m.action.from)
                .expect("native mover");
            &mut self.captures[(side * 6 + kind_index(pc.kind)) * 64 + square_index(m.action.to)]
        } else {
            let killers = &mut self.killers[usize::from(ply)];
            if killers[0] != Some(m.action) {
                killers[1] = killers[0];
                killers[0] = Some(m.action);
            }
            if let Some(p) = previous {
                let s = &mut self.continuation[square_index(p.to) * 64 + square_index(m.action.to)];
                *s = (*s + bonus).min(16_000);
            }
            &mut self.history
                [(side * 64 + square_index(m.action.from)) * 64 + square_index(m.action.to)]
        };
        *slot = (*slot + bonus - bonus * slot.abs() / 16_000).clamp(-16_000, 16_000);
    }

    fn probe_null(
        &mut self,
        depth: i32,
        ply: u16,
        beta: i32,
        extensions: u8,
        previous: Option<Action>,
    ) -> Result<Option<Value>, Halt> {
        let before = self.cursor.position().clone();
        if !self.cursor.push_null() {
            return Ok(None);
        }
        self.eval.push(&before, self.cursor.position());
        self.stats.null_probes += 1;
        let probe = self.main(
            depth,
            ply,
            -beta,
            -beta + 1,
            false,
            false,
            extensions,
            previous,
        );
        self.pop();
        match probe {
            Ok(value) => Ok(Some(value)),
            // A side flip can make a counter limit unrepresentable only in
            // this hypothesis. It is no evidence against the real legal game.
            Err(Halt::Error(SearchError::Transition(mshogi_core::chess::PlayError::Action(
                mshogi_core::chess::ActionError::CounterOverflow,
            )))) => {
                self.stats.null_abandoned += 1;
                Ok(None)
            }
            Err(other) => Err(other),
        }
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "Explicit small search-stack/window parameters; no public configuration surface"
    )]
    fn main(
        &mut self,
        mut depth: i32,
        ply: u16,
        mut alpha: i32,
        beta: i32,
        pv: bool,
        allow_null: bool,
        mut extensions: u8,
        previous: Option<Action>,
    ) -> Result<Value, Halt> {
        if depth <= 0 && self.mode == SearchMode::Conventional {
            return self.quiescence(ply, alpha, beta, Q_PLIES, previous);
        }
        self.work.visit(ply)?;
        self.stats.main_nodes += 1;
        if let Some(value) = self.terminal(ply) {
            return Ok(value);
        }
        let claim = self.claim();
        if let Some((choice, true)) = claim {
            return Ok(Value {
                score: 0,
                pv: vec![choice],
            });
        }
        if depth <= 0 || ply >= MAX_PLY {
            let static_score = self.evaluate();
            return Ok(if let Some((c, _)) = claim.filter(|_| static_score <= 0) {
                Value {
                    score: 0,
                    pv: vec![c],
                }
            } else {
                Value::leaf(static_score)
            });
        }
        let node = self.cursor.node();
        let in_check = matches!(
            node.classification,
            Classification::Check | Classification::Checkmate
        );
        let synthetic = node.synthetic;
        if self.mode == SearchMode::Conventional
            && in_check
            && ply > 0
            && extensions > 0
            && !synthetic
        {
            depth += 1;
            extensions -= 1;
            self.stats.extensions += 1;
        }
        let original_alpha = alpha;
        let key = self.cursor.key();
        let hash = self.cursor.position_hash();
        let entry = if synthetic {
            None
        } else {
            self.stats.tt_probes += u64::from(!self.table.is_empty());
            self.table_index(hash).and_then(|i| self.table[i].as_ref())
        };
        let same_context = entry.as_ref().is_some_and(|e| Some(&e.key) == key.as_ref());
        self.stats.tt_position_hits += u64::from(entry.as_ref().is_some_and(|e| e.hash == hash));
        self.stats.tt_context_hits += u64::from(same_context);
        let hint = entry
            .as_ref()
            .filter(|e| e.hash == hash)
            .and_then(|e| e.value.pv.first())
            .and_then(|c| match c {
                SearchChoice::Move(a) => Some(*a),
                _ => None,
            });
        if let Some(e) = entry.filter(|e| {
            same_context
                && e.extensions == extensions
                && (e.depth == depth || self.mode == SearchMode::Conventional && e.depth >= depth)
        }) {
            self.stats.tt_hits += 1;
            let score = from_table(e.score, ply);
            if !pv && matches!(e.bound, Bound::Exact)
                || !pv && matches!(e.bound, Bound::Lower) && score >= beta
                || !pv && matches!(e.bound, Bound::Upper) && score <= alpha
            {
                self.stats.tt_cutoffs += 1;
                return Ok(Value {
                    score,
                    pv: e.value.pv.clone(),
                });
            }
        }
        // Native outcome/claim admission stays before TT reuse. Dynamic static
        // evaluation and action copies are unnecessary for a reusable cutoff.
        let static_score = self.evaluate();
        if self.mode == SearchMode::Conventional
            && allow_null
            && !pv
            && ply > 0
            && depth >= 4
            && !in_check
            && !synthetic
            && claim.is_none()
            && static_score >= beta
            && matches!(self.cursor.adjudication_mode(), AdjudicationMode::M2)
            && beta.abs() < MATE_BAND
            && self.cursor.position().halfmove_clock() < 90
            && null_material(self.cursor.position())
        {
            let reduced = (depth - 3 - depth / 5).max(0);
            if let Some(probe) = self.probe_null(reduced, ply, beta, extensions, previous)?
                && -probe.score >= beta
                && probe.score.abs() < MATE_BAND
            {
                // Always verify on the real history, never declare a null mate.
                let verified = self.main(
                    reduced,
                    ply,
                    beta - 1,
                    beta,
                    false,
                    false,
                    extensions,
                    previous,
                )?;
                if verified.score >= beta && verified.score.abs() < MATE_BAND {
                    self.stats.null_verified += 1;
                    return Ok(verified);
                }
            }
        }
        let mut best = claim.map(|(c, _)| Value {
            score: 0,
            pv: vec![c],
        });
        if let Some(value) = &best {
            alpha = alpha.max(0);
            if alpha >= beta {
                return Ok(value.clone());
            }
        }
        let actions = self.cursor.node().actions.clone();
        let moves = self.ordered(actions, hint, ply, previous)?;
        let mut searched = 0;
        let mut quiet_seen = 0;
        for (index, m) in moves.into_iter().enumerate() {
            if !m.tactical {
                quiet_seen += 1;
            }
            self.push(m.action)?;
            let child_node = self.cursor.node();
            let gives_check = matches!(
                child_node.classification,
                Classification::Check | Classification::Checkmate
            );
            let critical = child_node.outcome.is_some()
                || !child_node.claims.is_empty()
                || child_node.actions.is_empty();
            let selective = self.mode == SearchMode::Conventional
                && !critical
                && !pv
                && !in_check
                && !gives_check
                && null_material(self.cursor.position())
                && searched > 0
                && alpha.abs() < MATE_BAND;
            if selective
                && !m.tactical
                && depth <= 2
                && quiet_seen > 6 + depth * depth
                && static_score <= alpha
                && m.quiet_quality <= 0
                && !self.killers[usize::from(ply)].contains(&Some(m.action))
            {
                self.stats.late_move_pruned += 1;
                self.pop();
                continue;
            }
            if selective && !m.tactical && depth <= 2 && static_score + 140 * depth <= alpha {
                self.stats.futility_pruned += 1;
                self.pop();
                continue;
            }
            if selective
                && m.tactical
                && m.action.promotion.is_none()
                && depth <= 2
                && m.see < -100 * depth
            {
                self.stats.see_pruned += 1;
                self.pop();
                continue;
            }
            let reduction = if self.mode == SearchMode::Conventional
                && !critical
                && !in_check
                && !gives_check
                && !m.tactical
                && null_material(self.cursor.position())
                && depth >= 3
                && index >= 3
            {
                1 + i32::from(depth >= 6 && index >= 8)
            } else {
                0
            };
            let full_depth = depth - 1;
            let child = if searched == 0 || depth == 1 {
                self.main(
                    full_depth,
                    ply + 1,
                    -beta,
                    -alpha,
                    pv,
                    true,
                    extensions,
                    Some(m.action),
                )
            } else {
                if reduction > 0 {
                    self.stats.reductions += 1;
                }
                let first = self.main(
                    (full_depth - reduction).max(0),
                    ply + 1,
                    -alpha - 1,
                    -alpha,
                    false,
                    true,
                    extensions,
                    Some(m.action),
                );
                match first {
                    Ok(v) if -v.score > alpha && reduction > 0 => {
                        self.stats.researches += 1;
                        self.main(
                            full_depth,
                            ply + 1,
                            -alpha - 1,
                            -alpha,
                            false,
                            true,
                            extensions,
                            Some(m.action),
                        )
                    }
                    other => other,
                }
                .and_then(|v| {
                    if pv && -v.score > alpha && -v.score < beta {
                        self.stats.researches += 1;
                        self.main(
                            full_depth,
                            ply + 1,
                            -beta,
                            -alpha,
                            true,
                            true,
                            extensions,
                            Some(m.action),
                        )
                    } else {
                        Ok(v)
                    }
                })
            };
            self.pop();
            let value = Value::prepend(m.action, child?);
            searched += 1;
            if best.as_ref().is_none_or(|b| value.score > b.score) {
                best = Some(value);
            }
            alpha = alpha.max(best.as_ref().unwrap().score);
            if alpha >= beta {
                self.reward(m, depth, ply, previous);
                break;
            }
        }
        let best = best.expect("native nonterminal legal actions");
        if let (Some(key), Some(index)) = (key, self.table_index(hash)) {
            self.stats.tt_stores += 1;
            let bound = if best.score <= original_alpha {
                Bound::Upper
            } else if best.score >= beta {
                Bound::Lower
            } else {
                Bound::Exact
            };
            self.table[index] = Some(Entry {
                key,
                hash,
                depth,
                extensions,
                score: to_table(best.score, ply),
                bound,
                value: best.clone(),
            });
        }
        Ok(best)
    }

    fn quiescence(
        &mut self,
        ply: u16,
        mut alpha: i32,
        beta: i32,
        left: u16,
        previous: Option<Action>,
    ) -> Result<Value, Halt> {
        self.work.visit(ply)?;
        self.stats.q_nodes += 1;
        if let Some(value) = self.terminal(ply) {
            return Ok(value);
        }
        let claim = self.claim();
        if let Some((c, true)) = claim {
            return Ok(Value {
                score: 0,
                pv: vec![c],
            });
        }
        let in_check = matches!(
            self.cursor.node().classification,
            Classification::Check | Classification::Checkmate
        );
        let score = self.evaluate();
        let mut best = claim.map(|(c, _)| Value {
            score: 0,
            pv: vec![c],
        });
        if ply >= MAX_PLY || !in_check {
            if best.as_ref().is_none_or(|b| score > b.score) {
                best = Some(Value::leaf(score));
            }
            alpha = alpha.max(best.as_ref().unwrap().score);
            if ply >= MAX_PLY || alpha >= beta || left == 0 {
                return Ok(best.unwrap());
            }
        } else if best.is_some() {
            alpha = alpha.max(0);
            if alpha >= beta {
                return Ok(best.unwrap());
            }
        }
        // Stand-pat cutoffs do not need an owned copy of every native action.
        let actions = self
            .cursor
            .node()
            .actions
            .clone()
            .into_iter()
            .filter(|a| in_check || gain(self.cursor.position(), *a) > 0 || a.promotion.is_some())
            .collect();
        let moves = self.ordered(actions, None, ply, previous)?;
        for m in moves {
            if !in_check && !m.tactical {
                continue;
            }
            self.push(m.action)?;
            let child_node = self.cursor.node();
            let checking = matches!(
                child_node.classification,
                Classification::Check | Classification::Checkmate
            );
            let critical = child_node.outcome.is_some()
                || !child_node.claims.is_empty()
                || child_node.actions.is_empty();
            if !in_check
                && !critical
                && !checking
                && m.action.promotion.is_none()
                && m.see < 0
                && score + m.gain + 140 < alpha
                && alpha.abs() < MATE_BAND
            {
                self.stats.see_pruned += 1;
                self.pop();
                continue;
            }
            let child = self.quiescence(
                ply + 1,
                -beta,
                -alpha,
                left.saturating_sub(1),
                Some(m.action),
            );
            self.pop();
            let value = Value::prepend(m.action, child?);
            if best.as_ref().is_none_or(|b| value.score > b.score) {
                best = Some(value);
            }
            alpha = alpha.max(best.as_ref().unwrap().score);
            if alpha >= beta {
                break;
            }
        }
        Ok(best.expect("in-check native node has legal evasions"))
    }
}
fn to_table(score: i32, ply: u16) -> i32 {
    if score >= MATE_BAND {
        score + i32::from(ply)
    } else if score <= -MATE_BAND {
        score - i32::from(ply)
    } else {
        score
    }
}
fn from_table(score: i32, ply: u16) -> i32 {
    if score >= MATE_BAND {
        score - i32::from(ply)
    } else if score <= -MATE_BAND {
        score + i32::from(ply)
    } else {
        score
    }
}
fn null_material(position: &Position) -> bool {
    let mut material = [0; 2];
    for rank in 0..8 {
        for file in 0..8 {
            if let Some(p) = position.piece_at(Square::new(file, rank).unwrap())
                && !matches!(p.kind, PieceKind::Pawn | PieceKind::King)
            {
                material[color_index(p.color)] += piece_value(p.kind);
            }
        }
    }
    material.into_iter().all(|n| n >= 600)
}

#[cfg(test)]
#[path = "chess_conventional_tests.rs"]
mod tests;
