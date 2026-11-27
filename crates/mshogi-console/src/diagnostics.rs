//! Shared shell perft limits and reporting.

use std::fmt::Write as _;

use mshogi_core::perft::{PerftPosition, STANDARD_VISIT_LIMIT, divide, perft};

pub enum Style {
    Chess,
    Local,
    LocalWithVisits,
    Game,
}

pub fn execute<P: PerftPosition>(
    position: &P,
    command: &str,
    args: &[&str],
    max_depth: u32,
    style: Style,
    out: &mut String,
    action_text: impl Fn(P::Action) -> Result<String, String>,
) -> Result<(), String> {
    let (depth, visits) = limits(command, args, max_depth)?;
    let (perft_label, total_label, note) = match style {
        Style::Chess => ("perft", "total", None),
        Style::Local => ("perft-local", "total-local", None),
        Style::LocalWithVisits => (
            "perft-local",
            "total-local",
            Some("ignores history/outcomes"),
        ),
        Style::Game => (
            "perft-game",
            "total-game",
            Some("full game history/endings"),
        ),
    };
    let report = if command == "perft" {
        let report = perft(position, depth, visits, || false).map_err(|error| error.to_string())?;
        write!(out, "{perft_label} {depth}").unwrap();
        if note.is_some() {
            out.push_str(" leaves");
        }
        report
    } else {
        let report =
            divide(position, depth, visits, || false).map_err(|error| error.to_string())?;
        let mut rows = String::new();
        for entry in report.entries {
            writeln!(rows, "{} {}", action_text(entry.action)?, entry.leaves).unwrap();
        }
        out.push_str(&rows);
        out.push_str(total_label);
        report.total
    };
    write!(out, " {}", report.leaves).unwrap();
    if let Some(note) = note {
        write!(out, " visited {} ({note})", report.visited).unwrap();
    }
    out.push('\n');
    Ok(())
}

fn limits(command: &str, args: &[&str], max_depth: u32) -> Result<(u32, u64), String> {
    let (depth, visits) = match args {
        [depth] => (*depth, STANDARD_VISIT_LIMIT),
        [depth, visits] => (
            *depth,
            crate::shell::number::<u64>(visits)
                .filter(|n| (1..=STANDARD_VISIT_LIMIT).contains(n))
                .ok_or_else(|| format!("visits must be ASCII 1..{STANDARD_VISIT_LIMIT}"))?,
        ),
        _ => return Err(format!("usage: {command} <depth> [visits]")),
    };
    let depth = crate::shell::number::<u64>(depth)
        .filter(|&n| n <= u64::from(max_depth))
        .ok_or_else(|| {
            format!(
                "invalid depth '{}'; expected ASCII decimal from 0 to {max_depth}",
                crate::record_io::diagnostic(depth)
            )
        })?;
    Ok((depth as u32, visits))
}
