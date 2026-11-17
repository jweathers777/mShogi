//! Mine forced checking mates from native-replayed Shogi game reports.
//!
//! Usage: shogi_tsume mine REPORT.json OUT.jsonl LAST_EVENTS NODE_CAP
//!
//! The report is replayed by the native experiment validator. For each of the
//! last `LAST_EVENTS` positions of every game, walking back over the native
//! winner's turns until the first position without a proven mate, the
//! native reference prover
//! finds the shortest checking mate of 3 to 11 plies for the side to move and
//! the complete set of checking first moves that mate within that length.
//! Positions whose proof exceeds the node bound are skipped and counted.
use mshogi_core::{
    native::CanonicalRecord,
    shogi::{Conclusion, Position},
};
use serde_json::json;
use std::{fs, io::Write};
#[path = "support/shogi_mate_proof.rs"]
mod shogi_mate_proof;
use shogi_mate_proof::{MAX_PROOF_PLIES, Prover};

/// `verify ROWS.json OUT.json NODE_CAP`: for each {"sfen","move"} row, prove
/// whether the move (a check) forces mate within 11 plies in total.
fn verify(args: &[String]) {
    let rows: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&args[2]).unwrap()).unwrap();
    let cap: u64 = args[4].parse().unwrap();
    let mut out = vec![];
    for row in rows.as_array().unwrap() {
        let position = Position::from_sfen(row["sfen"].as_str().unwrap()).unwrap();
        let action = row["move"].as_str().unwrap().parse().unwrap();
        let mut next = position.clone();
        let proven = if next.apply(action).is_err() {
            Some(false)
        } else if !next.is_in_check(next.side_to_move()) {
            None
        } else {
            Prover::new(cap)
                .defender_mated(&next, MAX_PROOF_PLIES - 1)
                .ok()
        };
        out.push(
            json!({"sfen": row["sfen"], "move": row["move"], "forces_mate_within_11": proven}),
        );
    }
    fs::write(&args[3], serde_json::to_string_pretty(&out).unwrap()).unwrap();
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.get(1).map(String::as_str) == Some("verify") {
        return verify(&args);
    }
    assert_eq!(
        args.get(1).map(String::as_str),
        Some("mine"),
        "usage: shogi_tsume mine REPORT OUT LAST_EVENTS NODE_CAP | verify ROWS OUT NODE_CAP"
    );
    let text = fs::read_to_string(&args[2]).unwrap();
    let last: usize = args[4].parse().unwrap();
    let cap: u64 = args[5].parse().unwrap();
    let report =
        mshogi_runner::experiments::ValidatedReport::from_json(&text).expect("native replay");
    let digest = mshogi_runner::experiments::digest(&text);
    let mut out = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[3])
        .unwrap();
    let mut skipped = 0u64;
    for (index, g) in report.games.iter().enumerate() {
        let opening = &report.config.openings[g.pairing.opening];
        let CanonicalRecord::Shogi(initial) = &opening.root.record else {
            panic!("Shogi report required")
        };
        let CanonicalRecord::Shogi(record) = &g.final_root.record else {
            panic!("Shogi final record required")
        };
        let end = record.event_count();
        let winner = match record.to_game().unwrap().conclusion() {
            Some(Conclusion::Win { winner, .. }) => Some(winner),
            _ => None,
        };
        let start = initial.event_count().max(end.saturating_sub(last));
        // Walk back over the winner's turns; stop at the first position
        // without a proven mate (earlier mates are rare and costly to refute).
        for event in (start..end).rev() {
            let game = record.prefix(event).unwrap().to_game().unwrap();
            if game.conclusion().is_some() {
                continue;
            }
            // Only the native winner's turns can start its forced mate.
            if winner != Some(game.position().side_to_move()) {
                continue;
            }
            // A diagram copy: the suite root has no earlier history.
            let position = Position::from_sfen(&game.position().to_sfen()).unwrap();
            let mut prover = Prover::new(cap);
            let shortest = match prover.shortest(&position, MAX_PROOF_PLIES) {
                Ok(Some(1)) => continue,
                Ok(Some(plies)) => plies,
                Ok(None) => break,
                Err(_) => {
                    skipped += 1;
                    break;
                }
            };
            let Ok(answers) = prover.answer_set(&position, shortest) else {
                skipped += 1;
                continue;
            };
            assert!(!answers.is_empty());
            let row = json!({
                "sfen": position.to_sfen(),
                "mate_plies": shortest,
                "expected_moves": answers.iter().map(|a| a.to_string()).collect::<Vec<_>>(),
                "proof_visits": prover.visits,
                "source_report_sha256": digest,
                "game_index": index,
                "event": event,
                "game_events": end,
                "opening_id": opening.id,
            });
            writeln!(out, "{row}").unwrap();
        }
    }
    eprintln!("skipped {skipped} positions at the node bound");
}
