use std::{
    io::{BufRead, BufReader, Write},
    process::{Child, ChildStdin, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
    time::{Duration, Instant},
};
pub struct Engine {
    pub child: Child,
    pub input: Option<ChildStdin>,
    pub lines: Receiver<String>,
    pub errors: Option<thread::JoinHandle<String>>,
}
impl Engine {
    pub fn new(args: &[&str]) -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_mshogi"))
            .args(args)
            .current_dir(std::env::temp_dir())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let input = child.stdin.take();
        let output = child.stdout.take().unwrap();
        let error = child.stderr.take().unwrap();
        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            for line in BufReader::new(output).lines() {
                if tx.send(line.unwrap()).is_err() {
                    break;
                }
            }
        });
        let errors = thread::spawn(move || {
            use std::io::Read;
            let mut text = String::new();
            BufReader::new(error).read_to_string(&mut text).unwrap();
            text
        });
        Self {
            child,
            input,
            lines: rx,
            errors: Some(errors),
        }
    }
    pub fn send(&mut self, text: &str) {
        let i = self.input.as_mut().unwrap();
        writeln!(i, "{text}").unwrap();
        i.flush().unwrap();
    }
    pub fn raw(&mut self, text: &str) {
        let input = self.input.as_mut().unwrap();
        input.write_all(text.as_bytes()).unwrap();
        input.flush().unwrap();
    }
    pub fn until(&self, prefix: &str) -> Vec<String> {
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut lines = vec![];
        loop {
            let line = self
                .lines
                .recv_timeout(deadline.saturating_duration_since(Instant::now()))
                .unwrap_or_else(|e| panic!("await {prefix}: {e}, lines {lines:?}"));
            let done = line.starts_with(prefix);
            lines.push(line);
            if done {
                return lines;
            }
        }
    }
    pub fn finish(self, quit: bool) -> (i32, String, Vec<String>) {
        self.finish_with_timeout(quit, Duration::from_secs(3))
    }
    pub fn finish_with_timeout(
        mut self,
        quit: bool,
        timeout: Duration,
    ) -> (i32, String, Vec<String>) {
        if quit {
            self.send("quit");
        } else {
            self.input.take();
        }
        let deadline = Instant::now() + timeout;
        let code = loop {
            if let Some(status) = self.child.try_wait().unwrap() {
                break status.code().unwrap_or(-1);
            }
            if Instant::now() >= deadline {
                self.child.kill().unwrap();
                panic!("protocol process failed to terminate promptly")
            }
            thread::sleep(Duration::from_millis(2));
        };
        let errors = self.errors.take().unwrap().join().unwrap();
        let lines = self.lines.iter().collect();
        (code, errors, lines)
    }
}
impl Drop for Engine {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}
