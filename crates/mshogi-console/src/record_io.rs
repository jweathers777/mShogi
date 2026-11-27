//! Human-console file operations; adapters own restoration.
use std::{
    fmt::Write as _,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

pub enum Operation<'a> {
    Print,
    Save(&'a str),
    LoadFile(&'a str),
    LoadJson(&'a str),
}
pub fn parse(rest: &str) -> Result<Operation<'_>, String> {
    match crate::shell::words(rest) {
        ("print", "") => Ok(Operation::Print),
        (_, "") => Err(usage()),
        ("save", path) => Ok(Operation::Save(path)),
        ("load-file", path) => Ok(Operation::LoadFile(path)),
        ("load-json", json) => Ok(Operation::LoadJson(json)),
        _ => Err(usage()),
    }
}
fn usage() -> String {
    "usage: record print | record save <path> | record load-file <path> | record load-json <JSON>"
        .into()
}
pub fn diagnostic(text: &str) -> String {
    text.chars()
        .take(512)
        .flat_map(char::escape_default)
        .take(4096)
        .collect()
}
pub fn read(path: &str) -> Result<String, String> {
    let bytes =
        fs::read(path).map_err(|e| format!("open failed: {}", diagnostic(&e.to_string())))?;
    String::from_utf8(bytes).map_err(|_| "record is not UTF-8; session preserved".into())
}
pub fn save(path: &str, encoded: &str, out: &mut String) -> Result<(), String> {
    save_new(Path::new(path), encoded.as_bytes()).map_err(|e| {
        format!(
            "save failed (use a new path; existing files are preserved): {}",
            diagnostic(&e.to_string())
        )
    })?;
    writeln!(out, "saved {} bytes", encoded.len()).unwrap();
    Ok(())
}
/// Publish a fully written sibling with no replacement, including symlinks.
fn save_new(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let parent = path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let temp = parent.join(format!(
        ".mshogi-{}-{}.tmp",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)?;
    let result = (|| {
        file.write_all(bytes)?;
        file.sync_all()?;
        drop(file);
        fs::hard_link(&temp, path)
    })();
    let _ = fs::remove_file(temp);
    result
}
