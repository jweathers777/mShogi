//! Private-analysis access to the existing authoritative Chess state/queries.
//!
//! This cursor owns no alternate move generator or adjudicator. One native legal
//! list feeds the turn's classification, outcome and claim queries; checked
//! Position transitions still resolve every played effect. Synthetic null probes
//! are separately labelled positions, never Game histories or canonical actions.
use super::*;
use std::{
    collections::{HashMap, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
    sync::Arc,
};

/// Native turn-boundary semantics without requiring an owned complete action set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchTurn {
    /// Native geometric/position-level classification, also for synthetic probes.
    pub classification: Classification,
    /// Rules-owned actual Game result; always absent for synthetic null probes.
    pub outcome: Option<ChessOutcome>,
    /// Native current/intended claims in profile policy order; absent in probes.
    pub claims: Vec<ClaimRequest>,
    /// True only inside a detached hypothetical null branch.
    pub synthetic: bool,
}
/// Cached native queries and complete legal-action set for an expanded turn.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchNode {
    /// Complete position-legal actions. A concluded native game cannot apply them.
    pub actions: Vec<Action>,
    /// Native semantics for this exact turn.
    pub turn: SearchTurn,
}
impl std::ops::Deref for SearchNode {
    type Target = SearchTurn;
    fn deref(&self) -> &Self::Target {
        &self.turn
    }
}
/// Opaque checked native transition for a claim-free M2 turn. Search can inspect
/// its semantics before deciding whether to expand; callers cannot author effects.
#[derive(Debug)]
pub struct SearchPreview {
    key: SearchKey,
    action: Action,
    after: Position,
    turn: SearchTurn,
}
impl SearchPreview {
    /// Exact native child semantics under the token's checked context.
    pub fn turn(&self) -> &SearchTurn {
        &self.turn
    }
    /// Checked resulting Position; this is not a committed Game/history.
    pub fn position(&self) -> &Position {
        &self.after
    }
}
#[derive(Debug)]
struct Root {
    game: AdjudicatedGame,
    prior: Vec<RepetitionIdentity>,
}
#[derive(Clone, Debug)]
struct Path {
    parent: Option<Arc<Path>>,
    action: Option<Action>,
    hash: u64,
    length: usize,
    position_hash: u64,
    identity: RepetitionIdentity,
    raw_ep: Option<Square>,
    halfmove: u32,
    fullmove: u32,
    repetitions_hash: u64,
    irreversible: bool,
}

/// Collision-checked native future-state context, never a board-only bound key.
///
/// Keys retain a shared immutable root and persistent action prefix, not a Game
/// clone per table entry. M2 keys compare exact current Position (including both
/// counters/raw EP) and the occurrence multiset since the last pawn move, capture
/// or castling-rights loss. Older identities cannot recur. This admits compatible
/// transposed histories without forgetting any future claim/adjudication rights.
/// The root/configuration must still match. Full mode conservatively retains
/// complete prefix identity, including its cached bounded proof provenance.
/// Hashes only reject candidates; exact comparison guards every collision.
/// This is an ephemeral search identity, never a serialized public record ID.
#[derive(Clone, Debug)]
pub struct SearchKey {
    root: Arc<Root>,
    path: Arc<Path>,
}
impl SearchKey {
    /// Deterministic in-process hash; equality remains the collision guard.
    pub fn hash64(&self) -> u64 {
        if self.root.game.adjudication_mode() == AdjudicationMode::M2 {
            mix(self.path.position_hash) ^ self.path.repetitions_hash
        } else {
            self.path.hash
        }
    }

    fn repetitions(&self) -> HashMap<&RepetitionIdentity, usize> {
        let mut counts = HashMap::new();
        let mut path = &self.path;
        loop {
            *counts.entry(&path.identity).or_default() += 1;
            if path.irreversible {
                break;
            }
            if let Some(parent) = &path.parent {
                path = parent;
            } else {
                for identity in &self.root.prior {
                    *counts.entry(identity).or_default() += 1;
                }
                break;
            }
        }
        counts
    }

    // Iterative deepening commonly revisits the identical ordered suffix.
    // Prove that inexpensive exact case before allocating count maps for a
    // genuinely reordered context. Root equality has already been checked.
    fn same_ordered_repetitions(&self, other: &Self) -> bool {
        let (mut a, mut b) = (&self.path, &other.path);
        loop {
            if Arc::ptr_eq(a, b) {
                return true;
            }
            if a.identity != b.identity || a.irreversible != b.irreversible {
                return false;
            }
            if a.irreversible {
                return true;
            }
            match (&a.parent, &b.parent) {
                (Some(ap), Some(bp)) => (a, b) = (ap, bp),
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}
impl PartialEq for SearchKey {
    fn eq(&self, other: &Self) -> bool {
        if !Arc::ptr_eq(&self.root, &other.root) && self.root.game != other.root.game {
            return false;
        }
        if self.root.game.adjudication_mode() == AdjudicationMode::M2 {
            return self.path.position_hash == other.path.position_hash
                && self.path.repetitions_hash == other.path.repetitions_hash
                // Identity already contains board, side and castling rights.
                // Add the exact representation fields it deliberately omits,
                // rather than retaining a second board snapshot per path node.
                && self.path.identity == other.path.identity
                && self.path.raw_ep == other.path.raw_ep
                && self.path.halfmove == other.path.halfmove
                && self.path.fullmove == other.path.fullmove
                && (self.same_ordered_repetitions(other)
                    || self.repetitions() == other.repetitions());
        }
        if self.path.hash != other.path.hash || self.path.length != other.path.length {
            return false;
        }
        let (mut a, mut b) = (&self.path, &other.path);
        loop {
            if Arc::ptr_eq(a, b) {
                return true;
            }
            if a.action != b.action {
                return false;
            }
            match (&a.parent, &b.parent) {
                (Some(ap), Some(bp)) => {
                    a = ap;
                    b = bp;
                }
                (None, None) => return true,
                _ => return false,
            }
        }
    }
}
impl Eq for SearchKey {}
impl Hash for SearchKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.hash64());
    }
}

#[derive(Debug)]
enum Frame {
    Real {
        cache: Option<SearchNode>,
        summary: Option<SearchTurn>,
        path: Arc<Path>,
        keyed: bool,
        hash: u64,
        dead: Option<MateReachability>,
    },
    Null {
        cache: Option<SearchNode>,
        summary: Option<SearchTurn>,
        hash: u64,
    },
    Synthetic {
        cache: Option<SearchNode>,
        summary: Option<SearchTurn>,
        before: Position,
        hash: u64,
    },
}

/// Owned analysis branch with native history, cached turn queries and exact undo.
///
/// The caller's Game is never mutated. Every push must be balanced by pop,
/// including while unwinding cancellation or an error. This cursor has no record
/// exporter; `game()` is deliberately unavailable inside synthetic probes.
#[derive(Debug)]
pub struct SearchCursor {
    game: AdjudicatedGame,
    root: Arc<Root>,
    path: Arc<Path>,
    keyed: bool,
    hash: u64,
    cache: Option<SearchNode>,
    summary: Option<SearchTurn>,
    synthetic: Option<Position>,
    frames: Vec<Frame>,
}
impl SearchCursor {
    /// Clone the complete native root, with no shared mutable state.
    pub fn new(game: &AdjudicatedGame) -> Self {
        let hash = full_position_hash(game.position());
        let mut hasher = DefaultHasher::new();
        game.record().to_json().hash(&mut hasher);
        let prior = game.history().reversible_prior_identities();
        let identity = game.history().repetition_identity().clone();
        let repetitions_hash = prior.iter().fold(identity_hash(&identity), |sum, id| {
            sum.wrapping_add(identity_hash(id))
        });
        let path = Arc::new(Path {
            parent: None,
            action: None,
            hash: hasher.finish(),
            length: 0,
            position_hash: hash,
            identity,
            raw_ep: game.position().en_passant_target(),
            halfmove: game.position().halfmove_clock(),
            fullmove: game.position().fullmove_number(),
            repetitions_hash,
            irreversible: false,
        });
        Self {
            game: game.clone(),
            root: Arc::new(Root {
                game: game.clone(),
                prior,
            }),
            path,
            keyed: true,
            hash,
            cache: None,
            summary: None,
            synthetic: None,
            frames: Vec::new(),
        }
    }
    /// Current real or explicitly hypothetical native Position.
    pub fn position(&self) -> &Position {
        self.synthetic
            .as_ref()
            .unwrap_or_else(|| self.game.position())
    }
    /// Current actual Game; no Game exists for a synthetic null branch.
    pub fn game(&self) -> Option<&AdjudicatedGame> {
        self.synthetic.is_none().then_some(&self.game)
    }
    /// Exact whole-profile consumer claim policy, including in a probe's parent.
    pub fn claim_policy(&self) -> ClaimPolicy {
        self.game.history().claim_policy()
    }
    /// Native adjudication configuration; probes are supported only for M2.
    pub fn adjudication_mode(&self) -> AdjudicationMode {
        self.game.adjudication_mode()
    }
    /// Whether the cursor is currently detached from real history.
    pub fn is_synthetic(&self) -> bool {
        self.synthetic.is_some()
    }
    /// Native future-context bound key; none for synthetic or unkeyed branches.
    pub fn key(&self) -> Option<SearchKey> {
        (self.synthetic.is_none() && self.keyed).then(|| SearchKey {
            root: Arc::clone(&self.root),
            path: Arc::clone(&self.path),
        })
    }
    /// Incrementally maintained complete-position hash, including raw EP/counters.
    /// This is a hint/index only; it does not establish history equality.
    pub fn position_hash(&self) -> u64 {
        self.hash
    }
    /// Number of local pushes including any null marker, for restoration checks.
    pub fn depth(&self) -> usize {
        self.frames.len()
    }
    /// Cache the native legal set and all turn-boundary queries exactly once.
    pub fn node(&mut self) -> &SearchNode {
        if self.cache.is_none() {
            let position = self.position();
            let actions = position.legal_actions();
            let synthetic = self.synthetic.is_some();
            let turn = if let Some(summary) = &self.summary {
                summary.clone()
            } else {
                let (classification, outcome, claims) = if synthetic {
                    (position.classify_with_actions(&actions), None, Vec::new())
                } else {
                    self.game.search_state(&actions)
                };
                SearchTurn {
                    classification,
                    outcome,
                    claims,
                    synthetic,
                }
            };
            self.summary = Some(turn.clone());
            self.cache = Some(SearchNode { actions, turn });
        }
        self.cache.as_ref().expect("populated native node cache")
    }
    /// Exact native terminal/claim/classification query with lazy move expansion.
    /// No-claim turns only need one checked legal witness to rule out mate or
    /// stalemate. Near claim thresholds, enumerate the complete intended-action
    /// catalog through the unchanged native validator and preserve its order.
    pub fn turn(&mut self) -> &SearchTurn {
        if self.summary.is_none() {
            if self.synthetic.is_none() && self.game.search_needs_claim_actions() {
                self.node();
            } else {
                let position = self.position();
                let classification = position.classify_with_existence(position.has_legal_action());
                let synthetic = self.synthetic.is_some();
                let outcome = if synthetic {
                    None
                } else {
                    self.game.search_outcome(classification)
                };
                self.summary = Some(SearchTurn {
                    classification,
                    outcome,
                    claims: Vec::new(),
                    synthetic,
                });
            }
        }
        self.summary
            .as_ref()
            .expect("populated native turn summary")
    }
    /// Apply one checked native board action; failure preserves every cursor field.
    /// Claimed/automatic outcomes cannot be continued. Intended claims are unplayed.
    pub fn push(&mut self, action: Action) -> Result<(), PlayError> {
        self.push_with_key(action, true, None)
    }
    /// Apply the same checked native action/history without retaining a TT key.
    ///
    /// Useful for quiescent branches that never probe/store bounds. All native
    /// history, claims, outcomes, errors and undo remain exact. `key()` is absent
    /// throughout this subtree, including ordinary pushes, until it is popped;
    /// a stale parent key can never identify an unkeyed child.
    pub fn push_unkeyed(&mut self, action: Action) -> Result<(), PlayError> {
        self.push_with_key(action, false, None)
    }
    /// Prepare a checked transition without adding a Game history step. Returns
    /// None when history-sensitive claims or Full/synthetic/unkeyed state require
    /// ordinary native admission instead. With no identity repeated and clock<98,
    /// a child can reach at most its second occurrence, cannot claim fifty moves,
    /// and an intended next action has the opposite side from that sole possible
    /// repeated identity. Thus child claims/automatic history draws are impossible.
    /// Mate, stalemate and native material death still use their ordinary queries.
    pub fn preview(&mut self, action: Action) -> Result<Option<SearchPreview>, PlayError> {
        let Some(key) = self.key() else {
            return Ok(None);
        };
        if self.game.adjudication_mode() != AdjudicationMode::M2
            || self.game.position().halfmove_clock() >= 98
            || self.game.history().has_repeated_identity()
        {
            return Ok(None);
        }
        if self.turn().outcome.is_some() {
            return Err(PlayError::GameOver);
        }
        let after = self
            .position()
            .checked_after(action)
            .map_err(PlayError::Action)?;
        let classification = after.classify_with_existence(after.has_legal_action());
        let outcome = after.position_outcome(classification);
        Ok(Some(SearchPreview {
            key,
            action,
            after,
            turn: SearchTurn {
                classification,
                outcome,
                claims: Vec::new(),
                synthetic: false,
            },
        }))
    }
    /// Commit a prepared native action. A stale/foreign token is discarded and
    /// its action goes through ordinary checked admission at the current state;
    /// only exact native-context equality permits reuse of its effects/summary.
    pub fn push_preview(&mut self, preview: SearchPreview) -> Result<(), PlayError> {
        let action = preview.action;
        let current = self.key();
        let preview = (Some(&preview.key) == current.as_ref()).then_some(preview);
        self.push_with_key(action, true, preview)
    }
    fn push_with_key(
        &mut self,
        action: Action,
        retain_key: bool,
        prepared: Option<SearchPreview>,
    ) -> Result<(), PlayError> {
        let was_uncached = self.cache.is_none();
        let was_unsummarized = self.summary.is_none();
        let node = self.turn();
        if node.outcome.is_some()
            || matches!(
                node.classification,
                Classification::Checkmate | Classification::Stalemate
            )
        {
            if was_uncached {
                self.cache = None;
            }
            if was_unsummarized {
                self.summary = None;
            }
            return Err(PlayError::GameOver);
        }
        let before = self.position().clone();
        let (prepared_after, prepared_turn) =
            prepared.map_or((None, None), |p| (Some(p.after), Some(p.turn)));
        if let Some(position) = self.synthetic.as_mut() {
            if let Err(error) = position.apply(action) {
                if was_uncached {
                    self.cache = None;
                }
                if was_unsummarized {
                    self.summary = None;
                }
                return Err(PlayError::Action(error));
            }
            let after = position.clone();
            self.frames.push(Frame::Synthetic {
                cache: self.cache.take(),
                summary: self.summary.take(),
                before: before.clone(),
                hash: self.hash,
            });
            self.hash = update_position_hash(self.hash, &before, &after);
        } else {
            let dead = match self.game.apply_for_search(action, prepared_after) {
                Ok(dead) => dead,
                Err(error) => {
                    if was_uncached {
                        self.cache = None;
                    }
                    if was_unsummarized {
                        self.summary = None;
                    }
                    return Err(error);
                }
            };
            let next_hash = update_position_hash(self.hash, &before, self.game.position());
            self.frames.push(Frame::Real {
                cache: self.cache.take(),
                summary: self.summary.take(),
                path: Arc::clone(&self.path),
                keyed: self.keyed,
                hash: self.hash,
                dead,
            });
            self.keyed &= retain_key;
            if self.keyed {
                let irreversible = self.game.position().halfmove_clock() == 0
                    || before.castling_rights() != self.game.position().castling_rights();
                let identity = self.game.history().repetition_identity().clone();
                let identity_hash = identity_hash(&identity);
                let mut hasher = DefaultHasher::new();
                hasher.write_u64(self.path.hash);
                hasher.write_u64(action_code(action));
                self.path = Arc::new(Path {
                    parent: if irreversible && self.game.adjudication_mode() == AdjudicationMode::M2
                    {
                        None
                    } else {
                        Some(Arc::clone(&self.path))
                    },
                    action: Some(action),
                    hash: hasher.finish(),
                    length: self.path.length + 1,
                    position_hash: next_hash,
                    identity,
                    raw_ep: self.game.position().en_passant_target(),
                    halfmove: self.game.position().halfmove_clock(),
                    fullmove: self.game.position().fullmove_number(),
                    repetitions_hash: if irreversible {
                        identity_hash
                    } else {
                        self.path.repetitions_hash.wrapping_add(identity_hash)
                    },
                    irreversible,
                });
            }
            self.hash = next_hash;
        }
        if let Some(turn) = prepared_turn {
            self.summary = Some(turn);
        }
        Ok(())
    }
    /// Start a detached null probe only from an ongoing quiet M2 turn with no claim.
    /// This flips side and clears raw EP on a private Position, without incrementing
    /// Game counters, inserting a repetition or committing a player action. Native
    /// outcomes, claims and TT bound keys are unavailable until this marker is popped.
    /// Search must separately guard material/zugzwang and verify a cutoff at a real
    /// node; this API does not establish the soundness of null-move pruning.
    pub fn push_null(&mut self) -> bool {
        if self.synthetic.is_some() || self.game.adjudication_mode() != AdjudicationMode::M2 {
            return false;
        }
        let was_uncached = self.cache.is_none();
        let was_unsummarized = self.summary.is_none();
        let node = self.turn();
        if node.outcome.is_some()
            || node.classification != Classification::Quiet
            || !node.claims.is_empty()
        {
            if was_uncached {
                self.cache = None;
            }
            if was_unsummarized {
                self.summary = None;
            }
            return false;
        }
        let before = self.game.position();
        let mut position = before.clone();
        position.side_to_move = position.side_to_move.opponent();
        position.en_passant_target = None;
        let hash = update_position_hash(self.hash, before, &position);
        self.frames.push(Frame::Null {
            cache: self.cache.take(),
            summary: self.summary.take(),
            hash: self.hash,
        });
        self.synthetic = Some(position);
        self.hash = hash;
        true
    }
    /// Reverse exactly one successful move or null marker; false at this root.
    pub fn pop(&mut self) -> bool {
        let Some(frame) = self.frames.pop() else {
            return false;
        };
        match frame {
            Frame::Real {
                cache,
                summary,
                path,
                keyed,
                hash,
                dead,
            } => {
                self.game
                    .undo_for_search(dead)
                    .expect("balanced native search history");
                self.cache = cache;
                self.summary = summary;
                self.path = path;
                self.keyed = keyed;
                self.hash = hash;
            }
            Frame::Null {
                cache,
                summary,
                hash,
            } => {
                self.synthetic = None;
                self.cache = cache;
                self.summary = summary;
                self.hash = hash;
            }
            Frame::Synthetic {
                cache,
                summary,
                before,
                hash,
            } => {
                self.synthetic = Some(before);
                self.cache = cache;
                self.summary = summary;
                self.hash = hash;
            }
        }
        true
    }
}
fn identity_hash(identity: &RepetitionIdentity) -> u64 {
    let mut hasher = DefaultHasher::new();
    identity.hash(&mut hasher);
    hasher.finish()
}
fn action_code(action: Action) -> u64 {
    let promotion = match action.promotion {
        None => 0,
        Some(Promotion::Queen) => 1,
        Some(Promotion::Rook) => 2,
        Some(Promotion::Bishop) => 3,
        Some(Promotion::Knight) => 4,
    };
    u64::from(action.from.0) | (u64::from(action.to.0) << 6) | (promotion << 12)
}
fn mix(mut value: u64) -> u64 {
    value = value.wrapping_add(0x9e3779b97f4a7c15);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
    value ^ (value >> 31)
}
fn piece_hash(square: usize, piece: Piece) -> u64 {
    let kind = match piece.kind {
        PieceKind::Pawn => 0,
        PieceKind::Knight => 1,
        PieceKind::Bishop => 2,
        PieceKind::Rook => 3,
        PieceKind::Queen => 4,
        PieceKind::King => 5,
    };
    mix(1 + square as u64 * 12 + kind + if piece.color == Color::Black { 6 } else { 0 })
}
fn metadata_hash(position: &Position) -> u64 {
    let rights = position.castling_rights;
    let castle = u64::from(rights.kingside(Color::White))
        | (u64::from(rights.queenside(Color::White)) << 1)
        | (u64::from(rights.kingside(Color::Black)) << 2)
        | (u64::from(rights.queenside(Color::Black)) << 3);
    mix(1024 + u64::from(position.side_to_move == Color::Black))
        ^ mix(2048 + castle)
        ^ mix(4096
            + position
                .en_passant_target
                .map_or(64, |square| u64::from(square.0)))
        ^ mix(0x100000000 ^ u64::from(position.halfmove_clock))
        ^ mix(0x200000000 ^ u64::from(position.fullmove_number))
}
fn full_position_hash(position: &Position) -> u64 {
    position
        .board
        .iter()
        .enumerate()
        .fold(metadata_hash(position), |hash, (square, piece)| {
            hash ^ piece.map_or(0, |piece| piece_hash(square, piece))
        })
}
fn update_position_hash(mut hash: u64, before: &Position, after: &Position) -> u64 {
    hash ^= metadata_hash(before) ^ metadata_hash(after);
    for (square, (old, new)) in before.board.iter().zip(after.board.iter()).enumerate() {
        if old != new {
            hash ^= old.map_or(0, |piece| piece_hash(square, piece));
            hash ^= new.map_or(0, |piece| piece_hash(square, piece));
        }
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    fn action(s: &str) -> Action {
        s.parse().unwrap()
    }
    #[test]
    fn exact_key_equality_rejects_an_intentionally_forced_hash_collision() {
        let original = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        let mut cursor = SearchCursor::new(&original);
        cursor.push(action("e2e4")).unwrap();
        let a = cursor.key().unwrap();
        cursor.pop();
        cursor.push(action("d2d4")).unwrap();
        let mut b = cursor.key().unwrap();
        b.path = Arc::new(Path {
            parent: b.path.parent.clone(),
            action: b.path.action,
            hash: a.hash64(),
            length: b.path.length,
            position_hash: a.path.position_hash,
            identity: b.path.identity.clone(),
            raw_ep: b.path.raw_ep,
            halfmove: b.path.halfmove,
            fullmove: b.path.fullmove,
            repetitions_hash: a.path.repetitions_hash,
            irreversible: b.path.irreversible,
        });
        assert_eq!(a.hash64(), b.hash64());
        assert_ne!(
            a, b,
            "a hash match never substitutes for exact history equality"
        );
    }
    #[test]
    fn rejected_push_restores_even_an_unpopulated_node_cache() {
        let original = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        let mut cursor = SearchCursor::new(&original);
        let key = cursor.key();
        let hash = cursor.hash;
        assert!(cursor.cache.is_none());
        assert!(cursor.push(action("e2e5")).is_err());
        assert!(cursor.cache.is_none());
        assert_eq!(cursor.key(), key);
        assert_eq!(cursor.hash, hash);
        assert_eq!(cursor.game(), Some(&original));
        assert_eq!(cursor.depth(), 0);
        assert!(cursor.summary.is_none());
    }
    #[test]
    fn quiet_lazy_summary_needs_no_complete_actions_and_restores_on_undo() {
        let original = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        let mut cursor = SearchCursor::new(&original);
        let summary = cursor.turn().clone();
        assert_eq!(summary.classification, Classification::Quiet);
        assert!(
            cursor.cache.is_none(),
            "unexpanded leaf has no full legal list"
        );
        cursor.push(action("e2e4")).unwrap();
        assert!(cursor.summary.is_none());
        cursor.turn();
        assert!(cursor.cache.is_none());
        cursor.pop();
        assert!(cursor.cache.is_none());
        assert_eq!(cursor.summary, Some(summary));
        assert_eq!(cursor.node().actions.len(), 20);
        assert_eq!(cursor.game(), Some(&original));
    }
    #[test]
    fn foreign_prepared_payload_is_discarded_even_when_its_action_is_legal() {
        let original = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        let mut other = SearchCursor::new(&AdjudicatedGame::new(ClaimPolicy::AutoClaim));
        let mut token = other.preview(action("e2e4")).unwrap().unwrap();
        // Deliberate private corruption tests the admission guard independently
        // of the only production constructor (whose fields callers cannot edit).
        token.after = Position::from_fen("7k/8/8/8/8/8/8/K7 b - - 0 1").unwrap();
        token.turn.outcome = Some(ChessOutcome::Draw(DrawReason::DeadPosition));
        let mut cursor = SearchCursor::new(&original);
        cursor.push_preview(token).unwrap();
        let mut checked = original.clone();
        checked.apply(action("e2e4")).unwrap();
        assert_eq!(cursor.game(), Some(&checked));
        assert_eq!(cursor.turn().outcome, None);
        cursor.pop();
        assert_eq!(cursor.game(), Some(&original));
    }
    #[test]
    fn forced_same_position_digest_collision_still_compares_occurrence_multiplicity() {
        let original = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        let mut a = SearchCursor::new(&original);
        let mut b = SearchCursor::new(&original);
        let first = ["g1f3", "g8f6", "f3g1", "f6g8"];
        let second = ["b1c3", "b8c6", "c3b1", "c6b8"];
        for cycle in [first, first, second] {
            for text in cycle {
                a.push(action(text)).unwrap();
            }
        }
        for cycle in [first, second, second] {
            for text in cycle {
                b.push(action(text)).unwrap();
            }
        }
        let key_a = a.key().unwrap();
        let mut key_b = b.key().unwrap();
        assert_eq!(a.position(), b.position());
        assert_eq!(a.game().unwrap().history().current_occurrences(), 4);
        assert_eq!(b.game().unwrap().history().current_occurrences(), 4);
        let counts_a = key_a.repetitions();
        let counts_b = key_b.repetitions();
        assert_eq!(counts_a.len(), counts_b.len());
        assert!(counts_a.keys().all(|id| counts_b.contains_key(id)));
        assert_ne!(
            counts_a, counts_b,
            "same set and current count are insufficient"
        );
        let mut path = (*key_b.path).clone();
        path.repetitions_hash = key_a.path.repetitions_hash;
        key_b.path = Arc::new(path);
        assert_eq!(key_a.hash64(), key_b.hash64());
        assert_ne!(
            key_a, key_b,
            "forced digest collision must check exact counts"
        );
        println!("native Path bytes: {}", std::mem::size_of::<Path>());
    }
    #[test]
    fn deterministic_native_walk_rebuilds_hashes_and_restores_cached_turns() {
        let original = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        let mut cursor = SearchCursor::new(&original);
        let root = cursor.key();
        let root_node = cursor.node().clone();
        let mut random = 0x90facada12345678u64;
        for _ in 0..96 {
            let node = cursor.node();
            if node.outcome.is_some() {
                break;
            }
            let actions = node.actions.clone();
            random = mix(random);
            cursor
                .push(actions[random as usize % actions.len()])
                .unwrap();
            assert_eq!(cursor.hash, full_position_hash(cursor.position()));
        }
        while cursor.pop() {
            assert_eq!(cursor.hash, full_position_hash(cursor.position()));
        }
        assert_eq!(cursor.key(), root);
        assert_eq!(cursor.game(), Some(&original));
        assert_eq!(cursor.node(), &root_node);
    }
}
