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
    PieceKind, Position, Promotion, SearchCursor, SearchKey, SearchPreview, Square,
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
// Two quiet forcing plies plus the response to a threat created on the second.
const Q_FORCING_MIN: u16 = Q_PLIES - 1;
const Q_DEFENCE_MIN: u16 = Q_FORCING_MIN - 1;
const TABLE_ENTRIES: usize = 4096;
const MAX_TABLE_ENTRIES: usize = 16_384;

/// Soft iteration boundary and hard cooperative deadline, independent of watchdogs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeBudget {
    /// Stop starting iterations after this duration.
    pub soft: Duration,
    /// Poll before native nodes and exchange-ordering work.
    pub hard: Duration,
}
impl TimeBudget {
    /// Fixed per-move allowance: keep searching until the cooperative deadline.
    /// Clock allocation can separately supply an earlier soft iteration boundary.
    pub fn from_hard_ms(ms: u64) -> Self {
        Self {
            soft: Duration::from_millis(ms),
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
/// Per-invocation experimental ablations; every default preserves playing behavior.
/// These controls are not exposed through the product execution protocol.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SearchDiagnostics {
    /// Disable verified null-move pruning while retaining the same quiescent frontier.
    pub disable_null: bool,
    /// Disable late quiet reductions without changing the quiescent frontier.
    pub disable_lmr: bool,
    /// Disable main-search late quiet and static futility omissions.
    pub disable_quiet_omissions: bool,
    /// Disable exchange-based main/quiescent omissions; retain delta pruning.
    pub disable_see_pruning: bool,
    /// Disable quiescent material-gain delta omissions; retain SEE pruning.
    pub disable_delta_pruning: bool,
    /// Experimentally admit low-material LMR, preserving null/omission guards.
    pub endgame_lmr: bool,
}
/// Bounded per-invocation configuration; no table or history leaks between requests.
#[derive(Clone, Copy, Debug)]
pub struct SearchOptions {
    /// Diagnostic versus playing algorithm.
    pub mode: SearchMode,
    /// Optional soft/hard wall-time allocation.
    pub time: Option<TimeBudget>,
    /// Entries in buckets of up to four, clamped to16384; zero disables reuse.
    pub table_entries: usize,
    /// Opt-in diagnostic controls; normal product callers use the default.
    pub diagnostics: SearchDiagnostics,
}
impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            mode: SearchMode::Conventional,
            time: None,
            table_entries: TABLE_ENTRIES,
            diagnostics: SearchDiagnostics::default(),
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
    /// Expanded quiescent nodes in check, excluding earlier terminal/TT exits.
    pub q_check_nodes: u64,
    /// Expanded quiescent nodes at the exhausted capture horizon.
    pub q_horizon_nodes: u64,
    /// Checked reply projections at the final quiet-threat defence boundary.
    pub defence_reply_probes: u64,
    /// Exact native future-context transposition hits.
    pub tt_hits: u64,
    /// Real main-node table lookups, including empty slots.
    pub tt_probes: u64,
    /// Occupied lookups whose complete position hash matches (hint eligibility).
    pub tt_position_hits: u64,
    /// Lookups with exact native future-context equality, before depth/extension checks.
    pub tt_context_hits: u64,
    /// Completed real-node entries stored, including replacements.
    pub tt_stores: u64,
    /// Quiescence-entry lookups at keyed real horizon nodes.
    pub q_tt_probes: u64,
    /// Exact native-context and remaining-q-budget hits.
    pub q_tt_hits: u64,
    /// Bound cutoffs from compatible quiescence entries.
    pub q_tt_cutoffs: u64,
    /// Completed quiescence entries stored in the shared bounded table.
    pub q_tt_stores: u64,
    /// Occupied slots replaced with a different native context or search phase.
    pub tt_replacements: u64,
    /// New shallow stores declined to retain useful current-generation entries.
    pub tt_store_refusals: u64,
    /// Native checked child previews offered without changing Game history.
    pub prepared_moves: u64,
    /// Pruned prepared children that never needed Game/evaluation push or pop.
    pub prepared_pruned: u64,
    /// Bound cutoffs using a collision-checked native future-context key.
    pub tt_cutoffs: u64,
    /// Aspiration-window widening/re-searches.
    pub aspiration_researches: u64,
    /// PVS/reduction re-searches.
    pub researches: u64,
    /// Full-depth zero-window verification after a reduced move improves alpha.
    pub reduction_researches: u64,
    /// Full-window PV verification after a zero-window move improves alpha.
    pub pv_researches: u64,
    /// Main move-loop beta cutoffs, excluding native, null and TT returns.
    pub main_beta_cutoffs: u64,
    /// Main move-loop cutoffs caused by the first actually searched move.
    pub main_first_move_cutoffs: u64,
    /// Quiescent move-loop beta cutoffs, excluding stand pat, claims and TT.
    pub q_beta_cutoffs: u64,
    /// Quiescent move-loop cutoffs caused by the first actually searched move.
    pub q_first_move_cutoffs: u64,
    /// Quiescent omissions with a failing enabled exchange estimate.
    pub q_see_pruned: u64,
    /// Quiescent omissions with a failing enabled delta margin (may overlap SEE).
    pub q_delta_pruned: u64,
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
    /// One-ply extensions after an existing null probe observes a mating threat.
    pub threat_extensions: u64,
    /// Quiet main moves protected from reductions/omission by a native-confirmed mating threat.
    pub threat_protected_moves: u64,
    /// Quiescent turns that must investigate real defences to a contact-mate threat.
    pub queen_threat_nodes: u64,
    /// Native-confirmed immediate contact mates admitted at a quiescent node.
    pub queen_contact_mates: u64,
    /// Reached quiet moves creating a confirmed contact-mate threat inside quiescence.
    pub quiet_threat_moves: u64,
    /// Native projections of reached quiet candidates before frontier admission.
    pub q_quiet_projections: u64,
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
    /// Cumulative work at each completed or interrupted nominal iteration.
    pub iterations: Vec<IterationStats>,
}

/// Diagnostic accounting only; an interrupted iteration never supplies a choice.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IterationStats {
    /// Nominal depth attempted.
    pub depth: u16,
    /// Whether all root work completed successfully.
    pub completed: bool,
    /// Cumulative elapsed microseconds, including earlier iterations.
    pub elapsed_us: u128,
    /// Cumulative counters, including unsuccessful aspiration windows.
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
        diagnostics: options.diagnostics,
        table: vec![None; options.table_entries.min(MAX_TABLE_ENTRIES)],
        history: vec![0; 2 * 64 * 64],
        continuation: vec![0; 64 * 64],
        captures: vec![0; 2 * 6 * 64],
        killers: vec![[None; 2]; usize::from(MAX_PLY) + 1],
        stats: SearchStats::default(),
        generation: 0,
    };
    let mut result = SearchResult {
        decision: None,
        score: None,
        completed_depth: 0,
        nodes: 0,
        principal_variation: vec![],
        status: SearchStatus::Complete,
    };
    let mut iterations = Vec::new();
    for depth in 1..=limits.depth {
        engine.generation = depth;
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
        iterations.push(IterationStats {
            depth,
            completed: iteration.is_ok(),
            elapsed_us: engine.work.start.elapsed().as_micros(),
            stats: engine.stats.clone(),
        });
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
        iterations,
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
    generation: u16,
}
struct Engine<'a> {
    cursor: SearchCursor,
    eval: EvalState,
    work: Work<'a>,
    mode: SearchMode,
    diagnostics: SearchDiagnostics,
    table: Vec<Option<Entry>>,
    history: Vec<i32>,
    continuation: Vec<i32>,
    captures: Vec<i32>,
    killers: Vec<[Option<Action>; 2]>,
    stats: SearchStats,
    generation: u16,
}
#[derive(Clone, Copy)]
struct Ordered {
    action: Action,
    tactical: bool,
    gain: i32,
    see: i32,
    quiet_quality: i32,
}
struct MovePicker {
    hint: Option<Action>,
    pending: Option<Vec<Action>>,
    rest: std::vec::IntoIter<Ordered>,
}
impl MovePicker {
    fn new(mut actions: Vec<Action>, hint: Option<Action>) -> Self {
        let hint = hint.filter(|a| actions.contains(a));
        if let Some(hint) = hint {
            actions.retain(|a| *a != hint);
        }
        Self {
            hint,
            pending: Some(actions),
            rest: Vec::new().into_iter(),
        }
    }
    fn next(
        &mut self,
        engine: &mut Engine<'_>,
        ply: u16,
        previous: Option<Action>,
    ) -> Result<Option<Ordered>, Halt> {
        // Admit the TT hint from the actual native list. Do no exchange work
        // for unrelated moves until this first move fails to produce a cutoff.
        if let Some(hint) = self.hint.take() {
            return Ok(engine.ordered(vec![hint], None, ply, previous)?.pop());
        }
        if let Some(actions) = self.pending.take() {
            self.rest = engine.ordered(actions, None, ply, previous)?.into_iter();
        }
        Ok(self.rest.next())
    }
}
fn history_update(slot: &mut i32, bonus: i32) {
    *slot = (*slot + bonus - bonus.abs() * *slot / 16_000).clamp(-16_000, 16_000);
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
    let Ok(after) = position.project(a) else {
        return Ok(0); // Actual search push reports representation errors.
    };
    let replies = after.legal_captures_to(a.to);
    let reply = replies.iter().copied().min_by_key(|r| {
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
    let response = if let Some(reply) = reply {
        if reply.promotion.is_some() {
            // LVA chooses an origin, not the opponent's promotion. Preserve
            // every native legal completion of that capture and choose its
            // best continued material return, including declining the exchange.
            // A fixed back-rank target has at most two promoting pawn origins:
            // at most4^2=16 variant paths, each still bounded by `left` (16).
            // Every recursive variant polls cancellation. Ordinary LVA remains
            // a single path with its existing piece-value/lexical tie policy.
            let mut best = 0;
            for variant in replies
                .into_iter()
                .filter(|candidate| candidate.from == reply.from && candidate.to == reply.to)
            {
                best = best.max(exchange(&after, variant, left - 1, work)?);
            }
            best
        } else {
            exchange(&after, reply, left - 1, work)?.max(0)
        }
    } else {
        0
    };
    Ok(immediate - response)
}

impl Engine<'_> {
    fn evaluate(&self) -> i32 {
        self.eval
            .evaluate(self.cursor.position())
            .score_for(self.cursor.position().side_to_move())
    }
    fn terminal(&mut self, ply: u16) -> Option<Value> {
        let side = self.cursor.position().side_to_move();
        let node = self.cursor.turn();
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
        if node.synthetic
            && matches!(
                node.classification,
                Classification::Checkmate | Classification::Stalemate
            )
        {
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
            .turn()
            .claims
            .first()
            .copied()
            .map(|c| (SearchChoice::Claim(c), forced))
    }
    fn push(&mut self, a: Action) -> Result<(), Halt> {
        self.push_with_key(a, true)
    }
    fn push_with_key(&mut self, a: Action, keyed: bool) -> Result<(), Halt> {
        let before = self.cursor.position().clone();
        let result = if keyed {
            self.cursor.push(a)
        } else {
            self.cursor.push_unkeyed(a)
        };
        result.map_err(|e| Halt::Error(SearchError::Transition(e)))?;
        self.eval.push(&before, self.cursor.position());
        Ok(())
    }
    fn push_preview(&mut self, preview: SearchPreview) -> Result<(), Halt> {
        let before = self.cursor.position().clone();
        self.cursor
            .push_preview(preview)
            .map_err(|e| Halt::Error(SearchError::Transition(e)))?;
        self.eval.push(&before, self.cursor.position());
        Ok(())
    }
    fn pop(&mut self) {
        assert!(self.eval.pop());
        assert!(self.cursor.pop());
    }
    fn table_index(&self, hash: u64) -> Option<usize> {
        (!self.table.is_empty()).then(|| (hash as usize % self.table.len().div_ceil(4)) * 4)
    }
    fn table_range(&self, hash: u64) -> std::ops::Range<usize> {
        let start = self.table_index(hash).unwrap_or(0);
        start..(start + 4).min(self.table.len())
    }
    fn entry_index(
        &self,
        hash: u64,
        key: Option<&SearchKey>,
        q_depth: Option<i32>,
    ) -> Option<usize> {
        self.table_range(hash)
            .filter(|&i| self.table[i].as_ref().is_some_and(|e| e.hash == hash))
            .max_by_key(|&i| {
                let e = self.table[i].as_ref().unwrap();
                (
                    Some(&e.key) == key,
                    q_depth.map_or(e.depth > 0, |d| e.depth == d),
                    e.depth.max(0),
                )
            })
    }
    fn store(&mut self, entry: Entry, quiet: bool) {
        let range = self.table_range(entry.hash);
        if range.is_empty() {
            return;
        }
        let same = range.clone().find(|&i| {
            self.table[i].as_ref().is_some_and(|old| {
                old.hash == entry.hash
                    && old.key == entry.key
                    && old.extensions == entry.extensions
                    && (old.depth > 0 && entry.depth > 0 || old.depth == entry.depth)
            })
        });
        let empty = range.clone().find(|&i| self.table[i].is_none());
        let victim = same.or(empty).unwrap_or_else(|| {
            range
                .min_by_key(|&i| {
                    let old = self.table[i].as_ref().unwrap();
                    old.depth.max(0) - 8 * i32::from(self.generation.saturating_sub(old.generation))
                })
                .unwrap()
        });
        if let Some(old) = &self.table[victim] {
            if old.generation == self.generation
                && (entry.depth < 0 && old.depth > 0 || old.depth.max(0) > entry.depth.max(0) + 2)
            {
                self.stats.tt_store_refusals += 1;
                return;
            }
            self.stats.tt_replacements += u64::from(same.is_none());
        }
        if quiet {
            self.stats.q_tt_stores += 1;
        } else {
            self.stats.tt_stores += 1;
        }
        self.table[victim] = Some(entry);
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
        if self.cursor.turn().synthetic {
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
                history_update(s, bonus);
            }
            &mut self.history
                [(side * 64 + square_index(m.action.from)) * 64 + square_index(m.action.to)]
        };
        history_update(slot, bonus);
    }
    fn penalize_quiets(&mut self, actions: &[Action], depth: i32, previous: Option<Action>) {
        if self.cursor.is_synthetic() {
            return;
        }
        let side = color_index(self.cursor.position().side_to_move());
        let malus = -(depth * depth * 16).min(1000);
        for a in actions {
            history_update(
                &mut self.history[(side * 64 + square_index(a.from)) * 64 + square_index(a.to)],
                malus,
            );
            if let Some(p) = previous {
                history_update(
                    &mut self.continuation[square_index(p.to) * 64 + square_index(a.to)],
                    malus,
                );
            }
        }
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
        mut beta: i32,
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
        if let Some((choice, _)) = claim.filter(|_| beta <= 0) {
            return Ok(Value {
                score: 0,
                pv: vec![choice],
            });
        }
        alpha = alpha.max(-MATE_SCORE + i32::from(ply));
        beta = beta.min(MATE_SCORE - i32::from(ply) - 1);
        if alpha >= beta {
            return Ok(Value::leaf(alpha));
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
        let node = self.cursor.turn();
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
            self.entry_index(hash, key.as_ref(), None)
                .and_then(|i| self.table[i].as_ref())
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
        let mut threat_extension = 0;
        if self.mode == SearchMode::Conventional
            && allow_null
            && !self.diagnostics.disable_null
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
            if let Some(probe) = self.probe_null(reduced, ply, beta, extensions, previous)? {
                // A hypothetical mate never becomes a real score/outcome.
                // Spend at most one of the shared bounded extension credits
                // investigating real defences to the detected threat instead.
                if probe.score >= MATE_BAND && extensions > 0 {
                    threat_extension = 1;
                    self.stats.threat_extensions += 1;
                } else if -probe.score >= beta && probe.score.abs() < MATE_BAND {
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
        let mut moves = MovePicker::new(actions, hint);
        let mut searched = 0;
        let mut quiet_seen = 0;
        let mut index = 0;
        let mut failed_quiets = Vec::new();
        while let Some(m) = moves.next(self, ply, previous)? {
            let move_index = index;
            index += 1;
            if !m.tactical {
                quiet_seen += 1;
            }
            self.work.poll()?;
            let preview = self
                .cursor
                .preview(m.action)
                .map_err(|e| Halt::Error(SearchError::Transition(e)))?;
            let pushed = preview.is_none();
            self.stats.prepared_moves += u64::from(!pushed);
            let child_node = if let Some(preview) = &preview {
                preview.turn().clone()
            } else {
                self.push(m.action)?;
                self.cursor.turn().clone()
            };
            let gives_check = matches!(
                child_node.classification,
                Classification::Check | Classification::Checkmate
            );
            let mut critical = child_node.outcome.is_some()
                || !child_node.claims.is_empty()
                || matches!(
                    child_node.classification,
                    Classification::Checkmate | Classification::Stalemate
                );
            let child_has_null_material = self.mode == SearchMode::Conventional
                && threat_extension == 0
                && !critical
                && !in_check
                && !gives_check
                && null_material(
                    preview
                        .as_ref()
                        .map_or_else(|| self.cursor.position(), SearchPreview::position),
                );
            let lmr_candidate = self.mode == SearchMode::Conventional
                && !self.diagnostics.disable_lmr
                && threat_extension == 0
                && !critical
                && !in_check
                && !gives_check
                && !m.tactical
                && (child_has_null_material || self.diagnostics.endgame_lmr)
                && depth >= 3
                && move_index >= 3;
            // Low-material diagnostic reductions require the same native threat
            // protection as ordinary reductions; the null guard is independent.
            if ((child_has_null_material && !m.tactical && (depth <= 2 || move_index >= 3))
                || lmr_candidate)
                && preview
                    .as_ref()
                    .map_or_else(|| self.cursor.position(), SearchPreview::position)
                    .has_queen_contact_mate_threat()
            {
                critical = true;
                self.stats.threat_protected_moves += 1;
            }
            let selective = self.mode == SearchMode::Conventional
                && threat_extension == 0
                && !critical
                && !pv
                && !in_check
                && !gives_check
                && child_has_null_material
                && searched > 0
                && alpha.abs() < MATE_BAND;
            if selective
                && !self.diagnostics.disable_quiet_omissions
                && !m.tactical
                && depth <= 2
                && quiet_seen > 6 + depth * depth
                && static_score <= alpha
                && m.quiet_quality <= 0
                && !self.killers[usize::from(ply)].contains(&Some(m.action))
            {
                self.stats.late_move_pruned += 1;
                if pushed {
                    self.pop();
                } else {
                    self.stats.prepared_pruned += 1;
                }
                continue;
            }
            if selective
                && !self.diagnostics.disable_quiet_omissions
                && !m.tactical
                && depth <= 2
                && static_score + 140 * depth <= alpha
            {
                self.stats.futility_pruned += 1;
                if pushed {
                    self.pop();
                } else {
                    self.stats.prepared_pruned += 1;
                }
                continue;
            }
            if selective
                && !self.diagnostics.disable_see_pruning
                && m.tactical
                && m.action.promotion.is_none()
                && depth <= 2
                && m.see < -100 * depth
            {
                self.stats.see_pruned += 1;
                if pushed {
                    self.pop();
                } else {
                    self.stats.prepared_pruned += 1;
                }
                continue;
            }
            if let Some(preview) = preview {
                self.push_preview(preview)?;
            }
            let reduction = if lmr_candidate && !critical {
                // A PV node must compare plausible quiet alternatives reliably.
                // Give these moves one more ply than a non-PV cut probe: a
                // reduced fail-low is otherwise accepted without verification.
                // Keep the deeper late-move reduction and every native guard;
                // this is selective search, not an exact minimax guarantee.
                (1 + i32::from(depth >= 6 && move_index >= 8) - i32::from(pv)).max(0)
            } else {
                0
            };
            let full_depth = depth - 1 + threat_extension;
            let child_extensions = extensions - threat_extension as u8;
            let child = if searched == 0 || depth == 1 {
                self.main(
                    full_depth,
                    ply + 1,
                    -beta,
                    -alpha,
                    pv,
                    true,
                    child_extensions,
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
                    child_extensions,
                    Some(m.action),
                );
                match first {
                    Ok(v) if -v.score > alpha && reduction > 0 => {
                        self.stats.researches += 1;
                        self.stats.reduction_researches += 1;
                        self.main(
                            full_depth,
                            ply + 1,
                            -alpha - 1,
                            -alpha,
                            false,
                            true,
                            child_extensions,
                            Some(m.action),
                        )
                    }
                    other => other,
                }
                .and_then(|v| {
                    if pv && -v.score > alpha && -v.score < beta {
                        self.stats.researches += 1;
                        self.stats.pv_researches += 1;
                        self.main(
                            full_depth,
                            ply + 1,
                            -beta,
                            -alpha,
                            true,
                            true,
                            child_extensions,
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
                self.stats.main_beta_cutoffs += 1;
                self.stats.main_first_move_cutoffs += u64::from(searched == 1);
                self.reward(m, depth, ply, previous);
                self.penalize_quiets(&failed_quiets, depth, previous);
                break;
            }
            if !m.tactical {
                failed_quiets.push(m.action);
            }
        }
        let best = best.expect("native nonterminal legal actions");
        if let Some(key) = key {
            let bound = if best.score <= original_alpha {
                Bound::Upper
            } else if best.score >= beta {
                Bound::Lower
            } else {
                Bound::Exact
            };
            self.store(
                Entry {
                    key,
                    hash,
                    depth,
                    extensions,
                    score: to_table(best.score, ply),
                    bound,
                    value: best.clone(),
                    generation: self.generation,
                },
                false,
            );
        }
        Ok(best)
    }

    fn quiescence(
        &mut self,
        ply: u16,
        alpha: i32,
        beta: i32,
        left: u16,
        previous: Option<Action>,
    ) -> Result<Value, Halt> {
        self.quiescence_with_mate_guard(ply, alpha, beta, left, previous, false)
    }
    fn quiescence_with_mate_guard(
        &mut self,
        ply: u16,
        mut alpha: i32,
        mut beta: i32,
        left: u16,
        previous: Option<Action>,
        verify_mate_reply: bool,
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
        if let Some((choice, _)) = claim.filter(|_| beta <= 0) {
            return Ok(Value {
                score: 0,
                pv: vec![choice],
            });
        }
        if verify_mate_reply && ply < MAX_PLY {
            // A final-layer defence can replace a contact threat with a remote
            // or discovered mate. Prove immediate replies only, without another
            // recursive quiet-threat layer. This caller just pushed an unkeyed
            // native q action; no differently selective bound is cached.
            debug_assert!(self.cursor.key().is_none());
            for action in self.cursor.node().actions.clone() {
                self.work.poll()?;
                self.stats.defence_reply_probes += 1;
                let after = self.cursor.position().project(action).map_err(|e| {
                    Halt::Error(SearchError::Transition(
                        mshogi_core::chess::PlayError::Action(e),
                    ))
                })?;
                if after.in_check() && !after.has_legal_action() {
                    // Board proof is a candidate only. Actual Game admission
                    // and recursive native terminal/claim handling are retained.
                    self.push_with_key(action, false)?;
                    let child = self.quiescence(
                        ply + 1,
                        -beta,
                        -alpha,
                        left.saturating_sub(1),
                        Some(action),
                    );
                    self.pop();
                    return Ok(Value::prepend(action, child?));
                }
            }
        }
        alpha = alpha.max(-MATE_SCORE + i32::from(ply));
        beta = beta.min(MATE_SCORE - i32::from(ply) - 1);
        if alpha >= beta {
            return Ok(Value::leaf(alpha));
        }
        let key = (ply < MAX_PLY).then(|| self.cursor.key()).flatten();
        let hash = self.cursor.position_hash();
        // Negative depths tag q entries and encode their exact remaining budget;
        // they never satisfy a main-search depth request or another q horizon.
        let q_depth = -1 - i32::from(left);
        if let Some(key) = &key {
            self.stats.q_tt_probes += u64::from(!self.table.is_empty());
            if let Some(i) = self.entry_index(hash, Some(key), Some(q_depth)) {
                let e = self.table[i].as_ref().unwrap();
                if e.key == *key && e.depth == q_depth {
                    self.stats.q_tt_hits += 1;
                    let score = from_table(e.score, ply);
                    if matches!(e.bound, Bound::Exact)
                        || matches!(e.bound, Bound::Lower) && score >= beta
                        || matches!(e.bound, Bound::Upper) && score <= alpha
                    {
                        self.stats.q_tt_cutoffs += 1;
                        return Ok(Value {
                            score,
                            pv: e.value.pv.clone(),
                        });
                    }
                }
            }
        }
        let original_alpha = alpha;
        let value = self.quiescence_expand(ply, alpha, beta, left, previous, claim)?;
        if let Some(key) = key {
            let bound = if value.score <= original_alpha {
                Bound::Upper
            } else if value.score >= beta {
                Bound::Lower
            } else {
                Bound::Exact
            };
            self.store(
                Entry {
                    key,
                    hash,
                    depth: q_depth,
                    extensions: 0,
                    score: to_table(value.score, ply),
                    bound,
                    value: value.clone(),
                    generation: self.generation,
                },
                true,
            );
        }
        Ok(value)
    }
    fn quiescence_expand(
        &mut self,
        ply: u16,
        mut alpha: i32,
        beta: i32,
        left: u16,
        previous: Option<Action>,
        claim: Option<(SearchChoice, bool)>,
    ) -> Result<Value, Halt> {
        let in_check = matches!(
            self.cursor.turn().classification,
            Classification::Check | Classification::Checkmate
        );
        self.stats.q_check_nodes += u64::from(in_check);
        self.stats.q_horizon_nodes += u64::from(left == 0);
        if ply < MAX_PLY
            && let Some(action) = self.cursor.position().queen_contact_mate()
        {
            // The positional query is only a candidate. Actual native Game
            // admission and terminal scoring remain mandatory, including
            // representation errors, claims, cancellation and restoration.
            self.push_with_key(action, false)?;
            let child =
                self.quiescence(ply + 1, -beta, -alpha, left.saturating_sub(1), Some(action));
            self.pop();
            self.stats.queen_contact_mates += 1;
            return Ok(Value::prepend(action, child?));
        }
        let threatened = ply < MAX_PLY
            && left >= Q_DEFENCE_MIN
            && !in_check
            && self.cursor.position().has_queen_contact_mate_threat();
        self.stats.queen_threat_nodes += u64::from(threatened);
        let unsettled = in_check || threatened;
        let score = self.evaluate();
        let mut best = claim.map(|(c, _)| Value {
            score: 0,
            pv: vec![c],
        });
        if ply >= MAX_PLY || !unsettled {
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
        // Quiet checks and contact-threat expansions belong only at the first
        // two frontier plies, with one additional ply to answer their threats.
        // Recursively treating every later mate threat as
        // check turns quiescence into a second full-width search. Actual checks
        // still search every evasion, and immediate mates are admitted above at
        // every ply. Deeper quiet plans belong to the next main iteration.
        // Stand-pat cutoffs do not need an owned copy of every native action.
        let candidates = if unsettled || left >= Q_FORCING_MIN {
            self.cursor.node().actions.clone()
        } else {
            self.cursor.position().captures_and_promotions()
        };
        // Score all candidates before searching so admitted moves retain their
        // original ordering and capture SEE. Delay native quiet projection until
        // its ordered turn: an earlier capture cutoff needs no later quiet work.
        let moves = self.ordered(candidates, None, ply, previous)?;
        let mut searched = 0;
        for m in moves {
            if !unsettled && !m.tactical {
                // Only the first two q plies consider quiet checks. Native
                // resolution derives discovered/castling effects; no separate
                // checking-move geometry or legal-move approximation lives here.
                self.work.poll()?;
                self.stats.q_quiet_projections += 1;
                let projected = self.cursor.position().project(m.action).map_err(|e| {
                    Halt::Error(SearchError::Transition(
                        mshogi_core::chess::PlayError::Action(e),
                    ))
                })?;
                let checking = projected.in_check();
                let threat = !checking && left > 0 && projected.has_queen_contact_mate_threat();
                if !checking && !threat {
                    continue;
                }
                self.stats.quiet_threat_moves += u64::from(threat);
            }
            self.push_with_key(m.action, false)?;
            let child_node = self.cursor.turn();
            let checking = matches!(
                child_node.classification,
                Classification::Check | Classification::Checkmate
            );
            let critical = child_node.outcome.is_some()
                || !child_node.claims.is_empty()
                || matches!(
                    child_node.classification,
                    Classification::Checkmate | Classification::Stalemate
                );
            let failing_see = !self.diagnostics.disable_see_pruning && m.see < 0;
            let failing_delta =
                !self.diagnostics.disable_delta_pruning && score + m.gain + 140 < alpha;
            if !unsettled
                && !critical
                && !checking
                && m.action.promotion.is_none()
                && (failing_see || failing_delta)
                && m.tactical
                && alpha.abs() < MATE_BAND
            {
                self.stats.see_pruned += 1;
                self.stats.q_see_pruned += u64::from(failing_see);
                self.stats.q_delta_pruned += u64::from(failing_delta);
                self.pop();
                continue;
            }
            let child = self.quiescence_with_mate_guard(
                ply + 1,
                -beta,
                -alpha,
                left.saturating_sub(1),
                Some(m.action),
                threatened && left == Q_DEFENCE_MIN,
            );
            self.pop();
            let value = Value::prepend(m.action, child?);
            searched += 1;
            if best.as_ref().is_none_or(|b| value.score > b.score) {
                best = Some(value);
            }
            alpha = alpha.max(best.as_ref().unwrap().score);
            if alpha >= beta {
                self.stats.q_beta_cutoffs += 1;
                self.stats.q_first_move_cutoffs += u64::from(searched == 1);
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
