use std::{
    fs,
    io::Write,
    path::PathBuf,
    process::{Command, Output, Stdio},
    sync::atomic::{AtomicU64, Ordering},
};

pub fn command() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_mshogi"));
    cmd.current_dir(std::env::temp_dir()).stdin(Stdio::null());
    cmd
}

/// Drain output while writing. A large inline record can fill both pipes.
pub fn run(args: &[&str], input: impl AsRef<[u8]>) -> Output {
    let mut child = command()
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();
    std::thread::scope(|scope| {
        let input = input.as_ref();
        let writer = scope.spawn(move || stdin.write_all(input).unwrap());
        let output = child.wait_with_output().unwrap();
        writer.join().unwrap();
        output
    })
}
pub fn stdout(out: &Output) -> String {
    String::from_utf8(out.stdout.clone()).unwrap()
}
pub fn success(out: &Output) {
    assert!(out.status.success(), "{out:?}");
    assert!(out.stderr.is_empty(), "{out:?}");
}
pub fn tagged(out: &Output, tag: &str) -> Vec<String> {
    stdout(out)
        .lines()
        .filter_map(|l| l.strip_prefix(tag).map(str::to_owned))
        .collect()
}
pub fn records(out: &Output) -> Vec<String> {
    tagged(out, "record ")
}
pub struct Dir(pub PathBuf);
impl Dir {
    pub fn new() -> Self {
        static N: AtomicU64 = AtomicU64::new(0);
        let path = std::env::temp_dir().join(format!(
            "mshogi-console-test-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Dir {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).unwrap();
    }
}
