//! Private-analysis access to the existing authoritative Chess state/queries.
//!
//! This cursor owns no alternate move generator or adjudicator. One native legal
//! list feeds the turn's classification, outcome and claim queries; checked
//! Position transitions still resolve every played effect. Synthetic null probes
//! are separately labelled positions, never Game histories or canonical actions.
use super::*;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
};

/// Cached native queries for one exact cursor turn.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchNode {
    /// Complete position-legal actions. A concluded native game cannot apply them.
    pub actions: Vec<Action>,
    /// Native geometric/position-level classification, also for synthetic probes.
    pub classification: Classification,
    /// Rules-owned actual Game result; always absent for synthetic null probes.
    pub outcome: Option<ChessOutcome>,
    /// Native current/intended claims in profile policy order; absent in probes.
    pub claims: Vec<ClaimRequest>,
    /// True only inside a detached hypothetical null branch.
    pub synthetic: bool,
}
#[derive(Debug)]
struct Root {
    game: AdjudicatedGame,
}
#[derive(Debug)]
struct Path {
    parent: Option<Arc<Path>>,
    action: Option<Action>,
    hash: u64,
    length: usize,
}

/// Collision-checked search context, conservatively including the full history.
///
/// Keys retain a shared immutable root and persistent action prefix, not a Game
/// clone per table entry. Equal board positions with different histories never
/// share bounds. Equality checks exact context even when the hash collides.
/// Re-search/iterative-deepening of the identical prefix can reuse entries;
/// different but semantically compatible histories are deliberately not merged.
/// This is an ephemeral search identity, never a serialized public record ID.
#[derive(Clone, Debug)]
pub struct SearchKey {
    root: Arc<Root>,
    path: Arc<Path>,
}
impl SearchKey {
    /// Deterministic in-process hash; equality remains the collision guard.
    pub fn hash64(&self) -> u64 {
        self.path.hash
    }
}
impl PartialEq for SearchKey {
    fn eq(&self, other: &Self) -> bool {
        if self.path.hash != other.path.hash || self.path.length != other.path.length {
            return false;
        }
        if !Arc::ptr_eq(&self.root, &other.root) && self.root.game != other.root.game {
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
        state.write_u64(self.path.hash);
    }
}

#[derive(Debug)]
enum Frame {
    Real {
        cache: Option<SearchNode>,
        path: Arc<Path>,
        hash: u64,
        dead: Option<MateReachability>,
    },
    Null {
        cache: Option<SearchNode>,
        hash: u64,
    },
    Synthetic {
        cache: Option<SearchNode>,
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
    hash: u64,
    cache: Option<SearchNode>,
    synthetic: Option<Position>,
    frames: Vec<Frame>,
}
impl SearchCursor {
    /// Clone the complete native root, with no shared mutable state.
    pub fn new(game: &AdjudicatedGame) -> Self {
        let hash = full_position_hash(game.position());
        let mut hasher = DefaultHasher::new();
        game.record().to_json().hash(&mut hasher);
        let path = Arc::new(Path {
            parent: None,
            action: None,
            hash: hasher.finish(),
            length: 0,
        });
        Self {
            game: game.clone(),
            root: Arc::new(Root { game: game.clone() }),
            path,
            hash,
            cache: None,
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
    /// A full-history collision-checked bound key; none exists for probes.
    pub fn key(&self) -> Option<SearchKey> {
        self.synthetic.is_none().then(|| SearchKey {
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
            let (classification, outcome, claims) = if synthetic {
                (position.classify_with_actions(&actions), None, Vec::new())
            } else {
                self.game.search_state(&actions)
            };
            self.cache = Some(SearchNode {
                actions,
                classification,
                outcome,
                claims,
                synthetic,
            });
        }
        self.cache.as_ref().expect("populated native node cache")
    }
    /// Apply one checked native board action; failure preserves every cursor field.
    /// Claimed/automatic outcomes cannot be continued. Intended claims are unplayed.
    pub fn push(&mut self, action: Action) -> Result<(), PlayError> {
        let was_uncached = self.cache.is_none();
        let node = self.node();
        if node.outcome.is_some()
            || matches!(
                node.classification,
                Classification::Checkmate | Classification::Stalemate
            )
        {
            if was_uncached {
                self.cache = None;
            }
            return Err(PlayError::GameOver);
        }
        let before = self.position().clone();
        if let Some(position) = self.synthetic.as_mut() {
            if let Err(error) = position.apply(action) {
                if was_uncached {
                    self.cache = None;
                }
                return Err(PlayError::Action(error));
            }
            let after = position.clone();
            self.frames.push(Frame::Synthetic {
                cache: self.cache.take(),
                before: before.clone(),
                hash: self.hash,
            });
            self.hash = update_position_hash(self.hash, &before, &after);
        } else {
            let dead = match self.game.apply_for_search(action) {
                Ok(dead) => dead,
                Err(error) => {
                    if was_uncached {
                        self.cache = None;
                    }
                    return Err(error);
                }
            };
            let next_hash = update_position_hash(self.hash, &before, self.game.position());
            let mut hasher = DefaultHasher::new();
            hasher.write_u64(self.path.hash);
            hasher.write_u64(action_code(action));
            self.frames.push(Frame::Real {
                cache: self.cache.take(),
                path: Arc::clone(&self.path),
                hash: self.hash,
                dead,
            });
            self.path = Arc::new(Path {
                parent: Some(Arc::clone(&self.path)),
                action: Some(action),
                hash: hasher.finish(),
                length: self.path.length + 1,
            });
            self.hash = next_hash;
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
        let node = self.node();
        if node.outcome.is_some()
            || node.classification != Classification::Quiet
            || !node.claims.is_empty()
        {
            if was_uncached {
                self.cache = None;
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
                path,
                hash,
                dead,
            } => {
                self.game
                    .undo_for_search(dead)
                    .expect("balanced native search history");
                self.cache = cache;
                self.path = path;
                self.hash = hash;
            }
            Frame::Null { cache, hash } => {
                self.synthetic = None;
                self.cache = cache;
                self.hash = hash;
            }
            Frame::Synthetic {
                cache,
                before,
                hash,
            } => {
                self.synthetic = Some(before);
                self.cache = cache;
                self.hash = hash;
            }
        }
        true
    }
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
