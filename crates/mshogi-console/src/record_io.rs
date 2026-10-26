//! Bounded human-console file operations; native adapters own restoration.
use mshogi_core::native::MAX_RECORD_ENVELOPE_BYTES;
use std::{
    fmt::Write as _,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

pub enum LegacyLoad {
    File,
    Json,
}
pub enum Operation<'a> {
    Print,
    LegacyPrint,
    Save(&'a str),
    LoadFile(&'a str),
    LoadJson(&'a str),
    LegacyJson(&'a str),
}
pub fn parse(rest: &str, legacy: LegacyLoad) -> Result<Operation<'_>, String> {
    if rest.is_empty() {
        return Ok(Operation::LegacyPrint);
    }
    if rest == "print" {
        return Ok(Operation::Print);
    }
    if let Some((op, argument)) = rest.split_once(|c: char| c.is_ascii_whitespace()) {
        let argument = argument.trim_matches(|c: char| c.is_ascii_whitespace());
        if !argument.is_empty() {
            return match op {
                "save" => Ok(Operation::Save(argument)),
                "load-file" => Ok(Operation::LoadFile(argument)),
                "load-json" => Ok(Operation::LoadJson(argument)),
                "load" => Ok(match legacy {
                    LegacyLoad::File => Operation::LoadFile(argument),
                    LegacyLoad::Json => Operation::LegacyJson(argument),
                }),
                _ => Err(usage()),
            };
        }
    }
    Err(usage())
}
fn usage() -> String {
    "usage: record print | record save <path> | record load-file <path> | record load-json <JSON> (legacy record/load remain game-specific)".into()
}
pub fn diagnostic(text: &str) -> String {
    text.chars()
        .take(512)
        .flat_map(char::escape_default)
        .take(4096)
        .collect()
}
pub fn read(path: &str) -> Result<String, String> {
    let mut bytes = Vec::new();
    File::open(path)
        .and_then(|file| {
            file.take((MAX_RECORD_ENVELOPE_BYTES + 1) as u64)
                .read_to_end(&mut bytes)
        })
        .map_err(|e| format!("open failed: {}", diagnostic(&e.to_string())))?;
    if bytes.len() > MAX_RECORD_ENVELOPE_BYTES {
        return Err(format!(
            "record exceeds {MAX_RECORD_ENVELOPE_BYTES}-byte limit; session preserved"
        ));
    }
    String::from_utf8(bytes).map_err(|_| "record is not UTF-8; session preserved".into())
}
pub fn save(path: &str, encoded: &str, out: &mut String) -> Result<(), String> {
    if encoded.len() > MAX_RECORD_ENVELOPE_BYTES {
        return Err("record exceeds save limit".into());
    }
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
