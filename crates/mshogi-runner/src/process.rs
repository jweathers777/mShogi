//! Bounded native subprocess transport. No shell commands or unbounded pipe joins.
use crate::{Diagnostic, EngineConfig, EngineObservation, IoLimits, ProcessExit};
use std::{
    io::{self, BufRead, BufReader, Read, Write},
    process::{Child, Command, ExitStatus, Stdio},
    sync::{
        Arc, Mutex,
        mpsc::{self, Receiver, SyncSender, TrySendError},
    },
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};
pub(crate) fn escaped(bytes: &[u8]) -> String {
    bytes
        .iter()
        .flat_map(|b| b.escape_ascii())
        .map(char::from)
        .collect()
}
pub(crate) fn status(status: ExitStatus) -> ProcessExit {
    #[cfg(unix)]
    let signal = {
        use std::os::unix::process::ExitStatusExt;
        status.signal()
    };
    #[cfg(not(unix))]
    let signal = None;
    ProcessExit {
        code: status.code(),
        signal,
        success: status.success(),
        description: status.to_string(),
    }
}
pub(crate) enum Output {
    Line(Vec<u8>),
    Oversized,
    Eof,
    Error(String),
}
struct WriteRequest {
    bytes: Vec<u8>,
    ack: SyncSender<Result<(), String>>,
}
/// Internal transport cause, independent of platform-specific error wording.
#[derive(Debug)]
pub(crate) enum WriteFailure {
    Transport(String),
    Timeout(String),
}
impl WriteFailure {
    pub fn timed_out(&self) -> bool {
        matches!(self, Self::Timeout(_))
    }
}
impl From<WriteFailure> for String {
    fn from(error: WriteFailure) -> Self {
        match error {
            WriteFailure::Transport(detail) | WriteFailure::Timeout(detail) => detail,
        }
    }
}
const EXIT_SETTLEMENT: Duration = Duration::from_millis(10);
fn settle_exit_poll(
    mut observe: impl FnMut() -> Option<ProcessExit>,
    mut expired: impl FnMut() -> bool,
    mut pause: impl FnMut(),
) -> Option<ProcessExit> {
    loop {
        if let Some(status) = observe() {
            return Some(status);
        }
        if expired() {
            return None;
        }
        pause();
    }
}
#[derive(Default)]
struct ErrorBuffer {
    diagnostic: Diagnostic,
    kept: usize,
}
pub(crate) struct Engine {
    pub index: usize,
    child: Child,
    writer: Option<SyncSender<WriteRequest>>,
    output: Option<Receiver<Output>>,
    stderr: Arc<Mutex<ErrorBuffer>>,
    threads: Vec<JoinHandle<()>>,
    pub identity: Vec<String>,
    pub identity_truncated: bool,
    pub last_output: Option<String>,
    pub exit: Option<ProcessExit>,
    // A real reaped child's status can be withheld to control observation order
    // in unit regressions. No synthesized exit or production timing hook.
    #[cfg(test)]
    deferred_exit: Option<(usize, ProcessExit)>,
    #[cfg(test)]
    single_exit_sample: bool,
}
fn bounded_line(reader: &mut impl BufRead, cap: usize) -> io::Result<Option<Result<Vec<u8>, ()>>> {
    let mut bytes = Vec::new();
    let mut oversized = false;
    let mut seen = false;
    loop {
        let chunk = reader.fill_buf()?;
        if chunk.is_empty() {
            return Ok(seen.then_some(if oversized { Err(()) } else { Ok(bytes) }));
        }
        seen = true;
        let end = chunk.iter().position(|&b| b == b'\n' || b == b'\r');
        let take = end.unwrap_or(chunk.len());
        if !oversized {
            if bytes.len() + take > cap {
                oversized = true;
                bytes.clear();
            } else {
                bytes.extend_from_slice(&chunk[..take]);
            }
        }
        reader.consume(take + usize::from(end.is_some()));
        if end.is_some() {
            return Ok(Some(if oversized { Err(()) } else { Ok(bytes) }));
        }
    }
}
impl Engine {
    pub fn spawn(index: usize, config: &EngineConfig, limits: IoLimits) -> Result<Self, String> {
        // Resolve a filesystem path explicitly: a bare name is relative to cwd,
        // rather than an undocumented PATH executable with unknown identity.
        let path =
            std::fs::canonicalize(&config.path).map_err(|e| format!("executable path: {e}"))?;
        if !std::fs::metadata(&path)
            .map_err(|e| e.to_string())?
            .is_file()
        {
            return Err("executable is not a regular file".into());
        }
        let mut child = Command::new(path)
            .args(&config.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?;
        let mut input = child.stdin.take().expect("piped stdin");
        let output = child.stdout.take().expect("piped stdout");
        let mut error = child.stderr.take().expect("piped stderr");
        let (write_send, write_recv) = mpsc::sync_channel::<WriteRequest>(1);
        let (out_send, out_recv) = mpsc::sync_channel(limits.queued_lines);
        let stderr = Arc::new(Mutex::new(ErrorBuffer::default()));
        let errors = stderr.clone();
        let writer = thread::Builder::new()
            .name("match-stdin".into())
            .spawn(move || {
                while let Ok(request) = write_recv.recv() {
                    let result = input
                        .write_all(&request.bytes)
                        .and_then(|_| input.flush())
                        .map_err(|e| e.to_string());
                    let failed = result.is_err();
                    let _ = request.ack.send(result);
                    if failed {
                        break;
                    }
                }
            });
        let reader = thread::Builder::new()
            .name("match-stdout".into())
            .spawn(move || {
                let mut reader = BufReader::new(output);
                loop {
                    let event = match bounded_line(&mut reader, limits.line_bytes) {
                        Ok(Some(Ok(bytes))) => Output::Line(bytes),
                        Ok(Some(Err(()))) => Output::Oversized,
                        Ok(None) => Output::Eof,
                        Err(e) => Output::Error(e.to_string()),
                    };
                    let ended = matches!(event, Output::Eof | Output::Error(_));
                    if out_send.send(event).is_err() || ended {
                        break;
                    }
                }
            });
        let error_reader = thread::Builder::new()
            .name("match-stderr".into())
            .spawn(move || {
                let mut chunk = [0u8; 2048];
                loop {
                    match error.read(&mut chunk) {
                        Ok(0) => break,
                        Ok(n) => {
                            let mut b = errors.lock().unwrap_or_else(|e| e.into_inner());
                            b.diagnostic.observed_bytes =
                                b.diagnostic.observed_bytes.saturating_add(n as u64);
                            let keep = n.min(limits.stderr_bytes.saturating_sub(b.kept));
                            b.diagnostic.text.push_str(&escaped(&chunk[..keep]));
                            b.kept += keep;
                            b.diagnostic.truncated |= keep < n;
                        }
                        Err(e) if e.kind() == io::ErrorKind::Interrupted => {}
                        Err(_) => break,
                    }
                }
            });
        let mut threads = Vec::new();
        let mut thread_error = None;
        for result in [writer, reader, error_reader] {
            match result {
                Ok(h) => threads.push(h),
                Err(e) => thread_error = Some(e.to_string()),
            }
        }
        let mut engine = Self {
            index,
            child,
            writer: Some(write_send),
            output: Some(out_recv),
            stderr,
            threads,
            identity: Vec::new(),
            identity_truncated: false,
            last_output: None,
            exit: None,
            #[cfg(test)]
            deferred_exit: None,
            #[cfg(test)]
            single_exit_sample: false,
        };
        if let Some(e) = thread_error {
            let _ = engine.cleanup(100);
            return Err(format!("pipe worker creation: {e}"));
        }
        Ok(engine)
    }
    /// Bounded queue and acknowledgement; blocked child reads never block caller.
    pub fn write(&mut self, bytes: Vec<u8>, timeout: Duration) -> Result<(), WriteFailure> {
        let (send, receive) = mpsc::sync_channel(1);
        let Some(writer) = &self.writer else {
            return Err(WriteFailure::Transport("stdin closed".into()));
        };
        match writer.try_send(WriteRequest { bytes, ack: send }) {
            Ok(()) => {}
            Err(TrySendError::Full(_)) => {
                return Err(WriteFailure::Transport("stdin writer queue is full".into()));
            }
            Err(TrySendError::Disconnected(_)) => {
                return Err(WriteFailure::Transport("stdin writer closed".into()));
            }
        }
        match receive.recv_timeout(timeout) {
            Ok(result) => result.map_err(WriteFailure::Transport),
            Err(mpsc::RecvTimeoutError::Timeout) => Err(WriteFailure::Timeout(
                "stdin write did not complete: timed out waiting on channel".into(),
            )),
            Err(mpsc::RecvTimeoutError::Disconnected) => Err(WriteFailure::Transport(
                "stdin writer acknowledgement channel closed".into(),
            )),
        }
    }
    pub fn receive(&self, timeout: Duration) -> Result<Output, mpsc::RecvTimeoutError> {
        self.output
            .as_ref()
            .expect("active output")
            .recv_timeout(timeout)
    }
    pub fn observe_exit(&mut self) -> Result<Option<ProcessExit>, String> {
        #[cfg(test)]
        if let Some((remaining, _)) = &mut self.deferred_exit {
            if *remaining != 0 {
                *remaining -= 1;
                return Ok(None);
            }
            self.exit = self.deferred_exit.take().map(|(_, status)| status);
        }
        if self.exit.is_none() {
            self.exit = self
                .child
                .try_wait()
                .map_err(|e| e.to_string())?
                .map(status);
        }
        Ok(self.exit.clone())
    }
    #[cfg(test)]
    pub(crate) fn defer_exit_observations(&mut self, count: usize) {
        let status = self
            .exit
            .take()
            .or_else(|| self.deferred_exit.take().map(|(_, status)| status))
            .expect("test first reaped actual child");
        self.deferred_exit = Some((count, status));
    }
    #[cfg(test)]
    pub(crate) fn single_exit_sample_control(&mut self) {
        self.single_exit_sample = true;
    }
    #[cfg(test)]
    pub(crate) fn close_stdin_for_test(&mut self) {
        self.writer.take();
    }
    /// Existing bounded pipe-failure/status settlement; no wait() or kill.
    pub fn settle_exit(&mut self) -> Option<ProcessExit> {
        self.observe_exit_for(EXIT_SETTLEMENT)
    }
    /// Observe natural exit before any cleanup protocol/kill intervention.
    pub(crate) fn observe_exit_for(&mut self, budget: Duration) -> Option<ProcessExit> {
        // Controlled tests use the same loop with virtual elapsed time. The
        // status is still from a real child; only its visibility is gated.
        #[cfg(test)]
        if self.deferred_exit.is_some() {
            let elapsed = std::cell::Cell::new(Duration::ZERO);
            let limit = if self.single_exit_sample {
                Duration::ZERO
            } else {
                budget
            };
            return settle_exit_poll(
                || self.observe_exit().ok().flatten(),
                || elapsed.get() >= limit,
                || elapsed.set(elapsed.get() + Duration::from_millis(1)),
            );
        }
        let deadline = Instant::now() + budget;
        settle_exit_poll(
            || self.observe_exit().ok().flatten(),
            || Instant::now() >= deadline,
            || thread::sleep(Duration::from_millis(1)),
        )
    }
    pub fn cleanup(&mut self, cleanup_ms: u64) -> EngineObservation {
        self.cleanup_with_grace(cleanup_ms, true)
    }
    /// The controller already consumed this cleanup's natural-exit grace.
    pub(crate) fn cleanup_after_exit_grace(&mut self, cleanup_ms: u64) -> EngineObservation {
        self.cleanup_with_grace(cleanup_ms, false)
    }
    fn cleanup_with_grace(&mut self, cleanup_ms: u64, grace_allowed: bool) -> EngineObservation {
        let deadline = Instant::now() + Duration::from_millis(cleanup_ms);
        let mut problems = Vec::new();
        if self.observe_exit().ok().flatten().is_none() {
            // The controller records and attempts quit before entering cleanup.
            // This function never writes hidden protocol commands.
            let grace = if grace_allowed {
                Instant::now() + Duration::from_millis((cleanup_ms / 4).min(100))
            } else {
                Instant::now()
            };
            while Instant::now() < grace && Instant::now() < deadline {
                if self.observe_exit().ok().flatten().is_some() {
                    break;
                }
                thread::sleep(Duration::from_millis(1));
            }
            if self.exit.is_none()
                && let Err(e) = self.child.kill()
            {
                problems.push(format!("kill: {e}"));
            }
            while self.exit.is_none() && Instant::now() < deadline {
                match self.observe_exit() {
                    Ok(_) => {}
                    Err(e) => {
                        problems.push(format!("reap: {e}"));
                        break;
                    }
                }
                if self.exit.is_none() {
                    thread::sleep(Duration::from_millis(1));
                }
            }
            if self.exit.is_none() {
                problems.push("child not reaped within cleanup watchdog".into());
            }
        }
        self.writer.take();
        self.output.take();
        while self.threads.iter().any(|h| !h.is_finished()) && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(1));
        }
        for h in self.threads.drain(..) {
            if h.is_finished() {
                if h.join().is_err() {
                    problems.push("pipe worker panicked".into())
                }
            } else {
                problems.push("pipe worker did not close within cleanup watchdog; detached".into());
            }
        }
        EngineObservation {
            engine: self.index,
            process_id: self.child.id(),
            uci_identity: self.identity.clone(),
            identity_truncated: self.identity_truncated,
            stderr: self
                .stderr
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .diagnostic
                .clone(),
            exit: self.exit.clone(),
            cleanup_failures: problems,
        }
    }
}
impl Drop for Engine {
    fn drop(&mut self) {
        if self.exit.is_none() {
            let _ = self.child.kill();
            let _ = self.child.try_wait();
        }
    }
}
