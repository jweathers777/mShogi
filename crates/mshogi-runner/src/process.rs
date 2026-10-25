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
        };
        if let Some(e) = thread_error {
            let _ = engine.cleanup(100);
            return Err(format!("pipe worker creation: {e}"));
        }
        Ok(engine)
    }
    /// Bounded queue and acknowledgement; blocked child reads never block caller.
    pub fn write(&mut self, bytes: Vec<u8>, timeout: Duration) -> Result<(), String> {
        let (send, receive) = mpsc::sync_channel(1);
        let Some(writer) = &self.writer else {
            return Err("stdin closed".into());
        };
        match writer.try_send(WriteRequest { bytes, ack: send }) {
            Ok(()) => {}
            Err(TrySendError::Full(_)) => return Err("stdin writer queue is full".into()),
            Err(TrySendError::Disconnected(_)) => return Err("stdin writer closed".into()),
        }
        receive
            .recv_timeout(timeout)
            .map_err(|e| format!("stdin write did not complete: {e}"))?
    }
    pub fn receive(&self, timeout: Duration) -> Result<Output, mpsc::RecvTimeoutError> {
        self.output
            .as_ref()
            .expect("active output")
            .recv_timeout(timeout)
    }
    pub fn observe_exit(&mut self) -> Result<Option<ProcessExit>, String> {
        if self.exit.is_none() {
            self.exit = self
                .child
                .try_wait()
                .map_err(|e| e.to_string())?
                .map(status);
        }
        Ok(self.exit.clone())
    }
    /// Brief bounded EOF/status race resolution; no wait() is used.
    pub fn exit_after_eof(&mut self) -> Option<ProcessExit> {
        let deadline = Instant::now() + Duration::from_millis(10);
        loop {
            if let Ok(Some(s)) = self.observe_exit() {
                return Some(s);
            }
            if Instant::now() >= deadline {
                return None;
            }
            thread::sleep(Duration::from_millis(1));
        }
    }
    pub fn cleanup(&mut self, cleanup_ms: u64) -> EngineObservation {
        let deadline = Instant::now() + Duration::from_millis(cleanup_ms);
        let mut problems = Vec::new();
        if self.observe_exit().ok().flatten().is_none() {
            // The controller records and attempts quit before entering cleanup.
            // This function never writes hidden protocol commands.
            let grace = Instant::now() + Duration::from_millis((cleanup_ms / 4).min(100));
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
