use crate::*;
use std::{
    io::{self, Read, Write},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{
        Arc, Condvar, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

/// Off-path process watchdogs. They bound observations, not OS scheduling.
#[derive(Clone, Copy, Debug)]
pub struct BridgeOptions {
    /// Maximum launch/write/search/read/exit observation interval.
    pub watchdog: Duration,
    /// Maximum interval for observing child reaping and pipe worker shutdown.
    pub cleanup: Duration,
}
impl Default for BridgeOptions {
    fn default() -> Self {
        Self {
            watchdog: Duration::from_secs(15),
            cleanup: Duration::from_millis(250),
        }
    }
}
/// Submission rejected before replacing current work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubmitError {
    /// Bridge is shutting down.
    ShuttingDown,
    /// IDs/generations must be nonzero and strictly increasing per bridge.
    IdentityReused,
    /// Unsupported protocol version or invalid depth.
    InvalidRequest,
}
impl std::fmt::Display for SubmitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "internal engine submission rejected: {self:?}")
    }
}
impl std::error::Error for SubmitError {}
struct Pending {
    context: Arc<EngineRequest>,
    cancelled: Arc<AtomicBool>,
}
#[derive(Default)]
struct State {
    current: Option<Arc<EngineRequest>>,
    cancel: Option<Arc<AtomicBool>>,
    pending: Option<Pending>,
    result: Option<EngineResponse>,
    last_id: u64,
    last_generation: u64,
    shutdown: bool,
}
struct Shared {
    state: Mutex<State>,
    wake: Condvar,
    exited: AtomicBool,
}
/// One supervisory worker with one replaceable pending request and one result.
///
/// Submit/cancel/poll never launch, perform pipe I/O, wait or join. A fresh child
/// per search is killed on cancellation/replacement/shutdown. Three bounded pipe
/// helpers per child keep blocked pipes away from the controller. Cleanup only
/// joins finished helpers; an OS pipe that fails to close is detached after the
/// cleanup watchdog. Uncertain reaping/helper shutdown is a typed Cleanup failure
/// and is logged off-path even when cancellation suppresses result publication.
/// Shutdown acknowledgement observes worker exit, not proof of cleanup success.
/// This is not hard realtime.
pub struct EngineBridge {
    shared: Arc<Shared>,
}
impl EngineBridge {
    /// Create supervision for a bundled executable; filesystem/launch checks are
    /// deferred to the worker, so a missing executable is a typed Start failure.
    pub fn new(path: PathBuf) -> io::Result<Self> {
        Self::with_options(path, BridgeOptions::default())
    }
    /// Create a bridge with explicit watchdogs (also useful for fixture tests).
    pub fn with_options(path: PathBuf, options: BridgeOptions) -> io::Result<Self> {
        if options.watchdog.is_zero() || options.cleanup.is_zero() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "watchdogs must be positive",
            ));
        }
        let shared = Arc::new(Shared {
            state: Mutex::new(State::default()),
            wake: Condvar::new(),
            exited: AtomicBool::new(false),
        });
        let worker_shared = shared.clone();
        thread::Builder::new()
            .name("mshogi-engine-supervisor".into())
            .spawn(move || worker(worker_shared, path, options))?;
        Ok(Self { shared })
    }
    /// Replace prior work atomically without launching or serializing on caller.
    pub fn submit(&self, context: EngineRequest) -> Result<(), SubmitError> {
        if context.protocol != PROTOCOL_VERSION
            || !(1..=mshogi_search::search::MAX_DEPTH).contains(&context.limits.depth)
        {
            return Err(SubmitError::InvalidRequest);
        }
        let context = Arc::new(context);
        let cancelled = Arc::new(AtomicBool::new(false));
        let mut state = self.shared.state.lock().expect("engine metadata mutex");
        if state.shutdown {
            return Err(SubmitError::ShuttingDown);
        }
        if context.request_id <= state.last_id
            || context.process_generation <= state.last_generation
        {
            return Err(SubmitError::IdentityReused);
        }
        if let Some(cancel) = &state.cancel {
            cancel.store(true, Ordering::Release);
        }
        state.last_id = context.request_id;
        state.last_generation = context.process_generation;
        let retired = (
            state.current.replace(context.clone()),
            state.cancel.replace(cancelled.clone()),
            state.pending.replace(Pending { context, cancelled }),
            state.result.take(),
        );
        drop(state);
        self.shared.wake.notify_one();
        drop(retired);
        Ok(())
    }
    /// Invalidate even an already published result; child cleanup runs off-path.
    pub fn cancel(&self) {
        let mut state = self.shared.state.lock().expect("engine metadata mutex");
        if let Some(cancel) = &state.cancel {
            cancel.store(true, Ordering::Release);
        }
        let retired = (
            state.current.take(),
            state.cancel.take(),
            state.pending.take(),
            state.result.take(),
        );
        drop(state);
        self.shared.wake.notify_one();
        drop(retired);
    }
    /// Consume one current completion. Host must recheck the full context and
    /// native choice legality under its serialized authoritative session lock.
    pub fn poll(&self) -> Option<EngineResponse> {
        self.shared
            .state
            .lock()
            .expect("engine metadata mutex")
            .result
            .take()
    }
    /// Cancel and stop the supervisory worker, without joining on the caller.
    pub fn shutdown(&self) {
        let mut state = self.shared.state.lock().expect("engine metadata mutex");
        state.shutdown = true;
        if let Some(cancel) = &state.cancel {
            cancel.store(true, Ordering::Release);
        }
        let retired = (
            state.current.take(),
            state.cancel.take(),
            state.pending.take(),
            state.result.take(),
        );
        drop(state);
        self.shared.wake.notify_one();
        drop(retired);
    }
    /// Observe worker exit after its cleanup attempt. Cleanup uncertainty is logged
    /// even on cancellation/shutdown; this flag is not proof that cleanup succeeded.
    pub fn shutdown_acknowledged(&self) -> bool {
        self.shared.exited.load(Ordering::Acquire)
    }
}
impl Drop for EngineBridge {
    fn drop(&mut self) {
        self.shutdown();
    }
}
fn worker(shared: Arc<Shared>, path: PathBuf, options: BridgeOptions) {
    loop {
        let pending = {
            let mut state = shared.state.lock().expect("engine metadata mutex");
            while state.pending.is_none() && !state.shutdown {
                state = shared.wake.wait(state).expect("engine metadata mutex");
            }
            if state.shutdown {
                break;
            }
            state.pending.take().expect("pending engine request")
        };
        let result = run(&path, options, &pending);
        // Canonical history cloning is off the cancellation metadata lock.
        let response = EngineResponse {
            context: (*pending.context).clone(),
            result,
        };
        let mut state = shared.state.lock().expect("engine metadata mutex");
        if !state.shutdown
            && !pending.cancelled.load(Ordering::Acquire)
            && state
                .current
                .as_ref()
                .is_some_and(|current| Arc::ptr_eq(current, &pending.context))
        {
            state.result = Some(response);
        }
    }
    shared.exited.store(true, Ordering::Release);
}
struct Process {
    child: Child,
    pipes: Vec<JoinHandle<()>>,
    stderr: Arc<Mutex<Vec<u8>>>,
    written: Receiver<io::Result<()>>,
    stdout: Receiver<io::Result<Vec<u8>>>,
}
enum SpawnError {
    Start(io::Error),
    Cleanup(String),
}
impl From<io::Error> for SpawnError {
    fn from(error: io::Error) -> Self {
        Self::Start(error)
    }
}
impl Process {
    fn spawn(path: &PathBuf, bytes: Vec<u8>) -> Result<Self, SpawnError> {
        let path = std::fs::canonicalize(path)?;
        if !std::fs::metadata(&path)?.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "engine is not a regular file",
            )
            .into());
        }
        let mut child = Command::new(path)
            .arg("internal-search")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        let mut input = child.stdin.take().expect("piped stdin");
        let output = child.stdout.take().expect("piped stdout");
        let mut error = child.stderr.take().expect("piped stderr");
        let (write_send, written) = mpsc::sync_channel(1);
        let (out_send, stdout) = mpsc::sync_channel(1);
        let stderr = Arc::new(Mutex::new(Vec::new()));
        let errors = stderr.clone();
        let writer = thread::Builder::new()
            .name("internal-stdin".into())
            .spawn(move || {
                let _ = write_send.send(input.write_all(&bytes).and_then(|()| input.flush()));
            });
        let reader = thread::Builder::new()
            .name("internal-stdout".into())
            .spawn(move || {
                let mut bytes = Vec::new();
                let result = output
                    .take((MAX_RESPONSE_BYTES + 2) as u64)
                    .read_to_end(&mut bytes)
                    .map(|_| bytes);
                let _ = out_send.send(result);
            });
        let error_reader = thread::Builder::new()
            .name("internal-stderr".into())
            .spawn(move || {
                let mut chunk = [0u8; 2048];
                loop {
                    match error.read(&mut chunk) {
                        Ok(0) => break,
                        Ok(n) => {
                            let mut kept = errors.lock().expect("stderr buffer");
                            let count = n.min(MAX_STDERR_BYTES.saturating_sub(kept.len()));
                            kept.extend_from_slice(&chunk[..count]);
                        }
                        Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                        Err(_) => break,
                    }
                }
            });
        let mut process = Self {
            child,
            pipes: Vec::new(),
            stderr,
            written,
            stdout,
        };
        let mut spawn_error = None;
        for pipe in [writer, reader, error_reader] {
            match pipe {
                Ok(pipe) => process.pipes.push(pipe),
                Err(error) => spawn_error = Some(error),
            }
        }
        if let Some(error) = spawn_error {
            return match process.cleanup(Duration::from_millis(250)) {
                Ok(()) => Err(SpawnError::Start(error)),
                Err(detail) => Err(SpawnError::Cleanup(format!(
                    "pipe worker creation: {error}; {detail}"
                ))),
            };
        }
        Ok(process)
    }
    fn cleanup(&mut self, timeout: Duration) -> Result<(), String> {
        let deadline = Instant::now() + timeout;
        let _ = self.child.kill();
        let reaped = loop {
            match self.child.try_wait() {
                Ok(Some(_)) => break true,
                Err(_) => break false,
                Ok(None) => {}
            }
            if Instant::now() >= deadline {
                break false;
            }
            thread::sleep(Duration::from_millis(1));
        };
        let mut problems = finish_pipes(&mut self.pipes, deadline);
        if !reaped {
            problems.push("child was not observed reaped within cleanup watchdog");
        }
        if problems.is_empty() {
            Ok(())
        } else {
            Err(problems.join("; "))
        }
    }
}
fn finish_pipes(pipes: &mut Vec<JoinHandle<()>>, deadline: Instant) -> Vec<&'static str> {
    while pipes.iter().any(|pipe| !pipe.is_finished()) && Instant::now() < deadline {
        thread::sleep(Duration::from_millis(1));
    }
    let mut problems = Vec::new();
    for pipe in pipes.drain(..) {
        if pipe.is_finished() {
            if pipe.join().is_err() {
                problems.push("pipe helper panicked during cleanup");
            }
        } else {
            problems.push("pipe helper did not finish within cleanup watchdog; detached");
        }
    }
    problems
}
fn cleanup_failure(context: &EngineRequest, detail: String) -> EngineResult {
    let result = EngineResult::failure(FailureKind::Cleanup, detail);
    if let EngineResult::Failure { failure } = &result {
        // This must precede cancellation/shutdown publication filtering. The
        // diagnostic is capped by EngineResult::failure, independently of pipes.
        eprintln!(
            "Engine cleanup uncertainty (request {}, generation {}): {}",
            context.request_id, context.process_generation, failure.message
        );
    }
    result
}
impl Drop for Process {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.try_wait();
    }
}
fn run(path: &PathBuf, options: BridgeOptions, pending: &Pending) -> EngineResult {
    let mut bytes = match serde_json::to_vec(&*pending.context) {
        Ok(bytes) => bytes,
        Err(error) => return EngineResult::failure(FailureKind::InvalidRequest, error),
    };
    if bytes.len() > MAX_REQUEST_BYTES {
        return EngineResult::failure(FailureKind::InvalidRequest, "request exceeds byte cap");
    }
    bytes.push(b'\n');
    if pending.cancelled.load(Ordering::Acquire) {
        return EngineResult::failure(FailureKind::Io, "request cancelled before launch");
    }
    let deadline = Instant::now() + options.watchdog;
    let mut process = match Process::spawn(path, bytes) {
        Ok(process) => process,
        Err(SpawnError::Start(error)) => return EngineResult::failure(FailureKind::Start, error),
        Err(SpawnError::Cleanup(detail)) => return cleanup_failure(&pending.context, detail),
    };
    let mut output = None;
    let mut exit = None;
    let result = loop {
        if pending.cancelled.load(Ordering::Acquire) {
            break EngineResult::failure(FailureKind::Io, "request cancelled");
        }
        if let Ok(Err(error)) = process.written.try_recv() {
            break EngineResult::failure(FailureKind::Io, error);
        }
        if output.is_none() {
            match process.stdout.try_recv() {
                Ok(Ok(bytes)) => {
                    if bytes.len() > MAX_RESPONSE_BYTES + 1 {
                        break EngineResult::failure(
                            FailureKind::OversizedResponse,
                            "response exceeds byte cap",
                        );
                    }
                    output = Some(bytes);
                }
                Ok(Err(error)) => break EngineResult::failure(FailureKind::Io, error),
                Err(mpsc::TryRecvError::Disconnected) => {
                    break EngineResult::failure(FailureKind::Io, "stdout worker disconnected");
                }
                Err(mpsc::TryRecvError::Empty) => {}
            }
        }
        if exit.is_none() {
            match process.child.try_wait() {
                Ok(status) => exit = status,
                Err(error) => break EngineResult::failure(FailureKind::Io, error),
            }
        }
        if let Some(status) = exit {
            if !status.success() {
                let diagnostic =
                    String::from_utf8_lossy(&process.stderr.lock().expect("stderr buffer"))
                        .into_owned();
                break EngineResult::failure(
                    FailureKind::UnexpectedExit,
                    format!("{status}: {diagnostic}"),
                );
            }
            if let Some(bytes) = &output {
                if bytes.is_empty() {
                    break EngineResult::failure(
                        FailureKind::MissingResponse,
                        "engine exited without a response",
                    );
                }
                if bytes.last() != Some(&b'\n') {
                    break EngineResult::failure(
                        FailureKind::MalformedResponse,
                        "response missing final newline",
                    );
                }
                let response: EngineResponse = match serde_json::from_slice(bytes) {
                    Ok(response) => response,
                    Err(error) => {
                        break EngineResult::failure(FailureKind::MalformedResponse, error);
                    }
                };
                if response.context != *pending.context {
                    break EngineResult::failure(
                        FailureKind::ContextMismatch,
                        "engine echoed a different complete context",
                    );
                }
                break response.result;
            }
        }
        if Instant::now() >= deadline {
            break EngineResult::failure(
                FailureKind::Timeout,
                "internal engine response/exit watchdog expired",
            );
        }
        thread::sleep(Duration::from_millis(2));
    };
    if let Err(detail) = process.cleanup(options.cleanup) {
        return cleanup_failure(&pending.context, detail);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy};
    fn request(id: u64, generation: u64) -> EngineRequest {
        EngineRequest {
            protocol: PROTOCOL_VERSION,
            request_id: id,
            session_version: 0,
            process_generation: generation,
            game: AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record(),
            limits: WorkLimits { depth: 1, nodes: 1 },
        }
    }
    fn published(bridge: &EngineBridge) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if bridge.shared.state.lock().unwrap().result.is_some() {
                return;
            }
            assert!(Instant::now() < deadline);
            thread::sleep(Duration::from_millis(1));
        }
    }
    #[test]
    fn blocked_pipe_cleanup_is_explicit_bounded_and_does_not_join_unfinished_work() {
        let (release, blocked) = mpsc::sync_channel(1);
        let (done, finished) = mpsc::sync_channel(1);
        let mut pipes = vec![thread::spawn(move || {
            blocked.recv().unwrap();
            done.send(()).unwrap();
        })];
        // The helper cannot finish until this test explicitly releases it.
        let problems = finish_pipes(&mut pipes, Instant::now());
        assert_eq!(
            problems,
            ["pipe helper did not finish within cleanup watchdog; detached"]
        );
        assert!(pipes.is_empty());
        let result = cleanup_failure(&request(31, 42), problems.join("; "));
        assert!(matches!(
            result,
            EngineResult::Failure {
                failure: EngineFailure {
                    kind: FailureKind::Cleanup,
                    ..
                }
            }
        ));
        release.send(()).unwrap();
        finished.recv_timeout(Duration::from_secs(5)).unwrap();
    }
    #[test]
    fn cancel_and_replace_discard_even_an_already_published_completion() {
        let bridge = EngineBridge::new(PathBuf::from("/definitely/missing/engine")).unwrap();
        bridge.submit(request(1, 1)).unwrap();
        published(&bridge);
        bridge.cancel();
        assert!(bridge.poll().is_none());
        bridge.submit(request(2, 2)).unwrap();
        published(&bridge);
        assert_eq!(
            bridge.submit(request(3, 2)),
            Err(SubmitError::IdentityReused)
        );
        bridge.submit(request(3, 3)).unwrap();
        published(&bridge);
        assert_eq!(bridge.poll().unwrap().context.request_id, 3);
        assert!(bridge.poll().is_none());
    }
}
