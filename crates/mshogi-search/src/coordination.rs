//! One bounded native search worker, request identity and stale-result checking.
use crate::search::{SearchError, SearchLimits, SearchResult, search_with_stop};
use mshogi_core::chess::{AdjudicatedGame, GameRecord};
use std::{
    error::Error,
    fmt,
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
    thread,
};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Process-unique request identity, never reused or supplied by a consumer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestId(u64);
impl RequestId {
    /// Numeric value for logging and protocol diagnostics.
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Immutable exact request context. Board identity alone is insufficient.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestContext {
    id: RequestId,
    version: u64,
    base: GameRecord,
    limits: SearchLimits,
}
impl RequestContext {
    /// Unique request identity.
    pub const fn id(&self) -> RequestId {
        self.id
    }
    /// Consumer-supplied game revision, independent of ply or board equality.
    pub const fn version(&self) -> u64 {
        self.version
    }
    /// Exact canonical known game, including initial state, history and claims.
    pub const fn base(&self) -> &GameRecord {
        &self.base
    }
    /// Recorded deterministic search limits.
    pub const fn limits(&self) -> SearchLimits {
        self.limits
    }
}

/// Search result or explicit failure, permanently tied to its request snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SearchCompletion {
    context: Arc<RequestContext>,
    result: Result<SearchResult, SearchError>,
}
impl SearchCompletion {
    /// Exact request identity, version, known history and settings.
    pub fn context(&self) -> &RequestContext {
        &self.context
    }
    /// Completed search observations or explicit failure.
    pub const fn result(&self) -> &Result<SearchResult, SearchError> {
        &self.result
    }
}

/// Submission failure; existing work is left untouched.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubmitError {
    /// Global process identity space exhausted; IDs never wrap or repeat.
    IdentityExhausted,
    /// Coordinator is shutting down.
    ShuttingDown,
    /// Limits fail the same validation as synchronous search.
    Search(SearchError),
}
impl fmt::Display for SubmitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "search submission failed: {self:?}")
    }
}
impl Error for SubmitError {}

/// Why an envelope cannot affect the consumer's current game.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StaleResult {
    /// No current request, or request identity/settings/context was superseded.
    Superseded,
    /// Supplied revision differs from the request's base revision.
    VersionChanged,
    /// Exact current record differs, even if the board matches.
    ContextChanged,
}
impl fmt::Display for StaleResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "search result rejected: {self:?}")
    }
}
impl Error for StaleResult {}

struct Pending {
    context: Arc<RequestContext>,
    game: AdjudicatedGame,
    stop: Arc<AtomicBool>,
}
struct Current {
    context: Arc<RequestContext>,
    stop: Arc<AtomicBool>,
}
#[derive(Default)]
struct State {
    current: Option<Current>,
    pending: Option<Pending>,
    active: Option<RequestId>,
    result: Option<SearchCompletion>,
    shutdown: bool,
}
struct Shared {
    state: Mutex<State>,
    wake: Condvar,
    exited: AtomicBool,
}

/// A single native worker. Storage is bounded to one pending request, one
/// executing request and one completion, with no progress stream or I/O.
/// Submission/cancellation never join a worker or wait for search completion.
/// Short metadata mutex sections still depend on OS scheduling.
pub struct SearchCoordinator {
    shared: Arc<Shared>,
}
impl SearchCoordinator {
    /// Start one named native worker. Thread creation failures are explicit.
    pub fn new() -> Result<Self, std::io::Error> {
        let shared = Arc::new(Shared {
            state: Mutex::new(State::default()),
            wake: Condvar::new(),
            exited: AtomicBool::new(false),
        });
        let worker_shared = Arc::clone(&shared);
        // Dropping JoinHandle detaches, so Drop never blocks the caller.
        thread::Builder::new()
            .name("mshogi-search".into())
            .spawn(move || worker(worker_shared))?;
        Ok(Self { shared })
    }

    /// Snapshot the whole game privately and replace pending work. Previous
    /// work is asked to stop and its completion can no longer be accepted.
    /// A version may repeat; the new identity still prevents an ABA collision.
    pub fn submit(
        &self,
        game: &AdjudicatedGame,
        version: u64,
        limits: SearchLimits,
    ) -> Result<RequestContext, SubmitError> {
        if !(1..=crate::search::MAX_DEPTH).contains(&limits.depth) {
            return Err(SubmitError::Search(SearchError::InvalidDepth(limits.depth)));
        }
        let id = NEXT_ID
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |id| id.checked_add(1))
            .map(RequestId)
            .map_err(|_| SubmitError::IdentityExhausted)?;
        let context = Arc::new(RequestContext {
            id,
            version,
            base: game.record(),
            limits,
        });
        let pending = Pending {
            context: context.clone(),
            game: game.clone(),
            stop: Arc::new(AtomicBool::new(false)),
        };
        let mut state = self.shared.state.lock().expect("search metadata mutex");
        if state.shutdown {
            return Err(SubmitError::ShuttingDown);
        }
        if let Some(current) = &state.current {
            current.stop.store(true, Ordering::Release);
        }
        let retired_current = state.current.replace(Current {
            context: context.clone(),
            stop: Arc::clone(&pending.stop),
        });
        let retired_pending = state.pending.replace(pending);
        let retired_result = state.result.take();
        drop(state);
        self.shared.wake.notify_one();
        drop((retired_current, retired_pending, retired_result));
        Ok((*context).clone())
    }

    /// Request cancellation of the current request without invalidating its
    /// identity. A pending request produces an explicitly stopped completion;
    /// a running request retains only its last fully completed iteration.
    /// Returns the affected identity, or None when no request was submitted.
    pub fn cancel(&self) -> Option<RequestId> {
        let state = self.shared.state.lock().expect("search metadata mutex");
        state.current.as_ref().map(|current| {
            current.stop.store(true, Ordering::Release);
            current.context.id
        })
    }

    /// True once this request is neither executing nor pending. For a cancelled
    /// active request this acknowledges the search has reached a stop boundary.
    /// A superseded pending request is acknowledged immediately without running.
    pub fn acknowledged(&self, request: &RequestContext) -> bool {
        let state = self.shared.state.lock().expect("search metadata mutex");
        state.active != Some(request.id)
            && state
                .pending
                .as_ref()
                .is_none_or(|p| p.context.id != request.id)
    }

    /// Consume the single completion slot. Always validate with accept_result
    /// against the current game/version immediately before acting on it.
    pub fn poll(&self) -> Option<SearchCompletion> {
        self.shared
            .state
            .lock()
            .expect("search metadata mutex")
            .result
            .take()
    }

    /// Validate a possibly delayed envelope against the current request and
    /// exact caller context. Call and apply from the consumer's serialized game
    /// event loop; validation does not lock or mutate the consumer's game.
    pub fn accept_result(
        &self,
        completion: &SearchCompletion,
        game: &AdjudicatedGame,
        version: u64,
    ) -> Result<(), StaleResult> {
        // Build/compare records outside the metadata lock: history length must
        // not turn record work into a cancellation-critical section.
        let current_record = game.record();
        let current = {
            let state = self.shared.state.lock().expect("search metadata mutex");
            state
                .current
                .as_ref()
                .map(|current| Arc::clone(&current.context))
        };
        if current
            .as_ref()
            .is_none_or(|current| **current != *completion.context)
        {
            return Err(StaleResult::Superseded);
        }
        if version != completion.context.version {
            return Err(StaleResult::VersionChanged);
        }
        if current_record != completion.context.base {
            return Err(StaleResult::ContextChanged);
        }
        Ok(())
    }

    /// Request shutdown without joining. It cancels active work, discards
    /// pending/results and prevents further submissions.
    pub fn shutdown(&self) {
        let mut state = self.shared.state.lock().expect("search metadata mutex");
        state.shutdown = true;
        if let Some(current) = &state.current {
            current.stop.store(true, Ordering::Release);
        }
        let retired = (
            state.current.take(),
            state.pending.take(),
            state.result.take(),
        );
        drop(state);
        self.shared.wake.notify_one();
        drop(retired);
    }

    /// Observe worker exit after a shutdown request, without waiting or joining.
    pub fn shutdown_acknowledged(&self) -> bool {
        self.shared.exited.load(Ordering::Acquire)
    }
}
impl Drop for SearchCoordinator {
    fn drop(&mut self) {
        self.shutdown();
    }
}
fn worker(shared: Arc<Shared>) {
    loop {
        let pending = {
            let mut state = shared.state.lock().expect("search metadata mutex");
            while state.pending.is_none() && !state.shutdown {
                state = shared.wake.wait(state).expect("search metadata mutex");
            }
            if state.shutdown {
                break;
            }
            let pending = state.pending.take().expect("pending worker request");
            state.active = Some(pending.context.id);
            pending
        };
        let result = search_with_stop(&pending.game, pending.context.limits, || {
            pending.stop.load(Ordering::Acquire)
        });
        let mut state = shared.state.lock().expect("search metadata mutex");
        state.active = None;
        if !state.shutdown
            && state
                .current
                .as_ref()
                .is_some_and(|current| current.context.id == pending.context.id)
        {
            state.result = Some(SearchCompletion {
                context: pending.context,
                result,
            });
        }
    }
    shared.exited.store(true, Ordering::Release);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::search::{MAX_DEPTH, SearchStatus};
    use mshogi_core::chess::ClaimPolicy;
    use std::time::{Duration, Instant};

    #[test]
    fn already_running_worker_cancels_at_a_node_boundary() {
        let coordinator = SearchCoordinator::new().unwrap();
        let game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        let request = coordinator
            .submit(
                &game,
                1,
                SearchLimits {
                    depth: MAX_DEPTH,
                    nodes: None,
                },
            )
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(15);
        loop {
            let active = coordinator.shared.state.lock().unwrap().active;
            if active == Some(request.id()) {
                break;
            }
            assert!(Instant::now() < deadline);
            thread::yield_now();
        }
        assert!(!coordinator.acknowledged(&request));
        coordinator.cancel();
        let completion = loop {
            if let Some(result) = coordinator.poll() {
                break result;
            }
            assert!(Instant::now() < deadline);
            thread::sleep(Duration::from_millis(1));
        };
        assert_eq!(
            completion.result().as_ref().unwrap().status,
            SearchStatus::Stopped
        );
        assert!(coordinator.acknowledged(&request));
        assert_eq!(coordinator.accept_result(&completion, &game, 1), Ok(()));
    }
    #[test]
    fn active_superseded_request_cannot_publish_a_delayed_completion() {
        let coordinator = SearchCoordinator::new().unwrap();
        let mut game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
        let old = coordinator
            .submit(
                &game,
                1,
                SearchLimits {
                    depth: MAX_DEPTH,
                    nodes: None,
                },
            )
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(15);
        loop {
            if coordinator.shared.state.lock().unwrap().active == Some(old.id()) {
                break;
            }
            assert!(Instant::now() < deadline);
            thread::yield_now();
        }
        game.apply("e2e4".parse().unwrap()).unwrap();
        let latest = coordinator
            .submit(
                &game,
                2,
                SearchLimits {
                    depth: 1,
                    nodes: Some(0),
                },
            )
            .unwrap();
        let completion = loop {
            if let Some(result) = coordinator.poll() {
                break result;
            }
            assert!(Instant::now() < deadline);
            thread::sleep(Duration::from_millis(1));
        };
        assert_eq!(completion.context(), &latest);
        assert!(coordinator.acknowledged(&old));
        assert_eq!(coordinator.accept_result(&completion, &game, 2), Ok(()));
        assert!(coordinator.poll().is_none());
    }
}
