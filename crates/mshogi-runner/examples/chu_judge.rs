//! Full-history, fresh-table Chu move assessment. JSONL output is flushed per move.
//! chu_judge REPORT_PATHS OUT NODES SAMPLES_PER_GAME SHARD SHARDS
use mshogi_core::{chu::*, native::CanonicalRecord};
use mshogi_runner::experiments::ValidatedReport;
use mshogi_search::chu as search;
use serde_json::{Value, json};
use std::{fs, io::Write};

fn selected(len: usize, count: usize) -> Vec<usize> {
    let count = count.min(len);
    (0..count)
        .map(|i| (2 * i + 1) * len / (2 * count))
        .collect()
}

fn record(r: &CanonicalRecord) -> &GameRecord {
    let CanonicalRecord::Chu(r) = r else {
        panic!("not Chu")
    };
    r
}

fn facts(game: &Game, action: &Action) -> Value {
    let effects = game
        .local_position()
        .preview_for(game.position().side_to_move(), action)
        .unwrap();
    json!({"action":action, "captures":effects.captures,
        "captures_lion":effects.captures.iter().any(|c| c.piece.is_lion()),
        "promotes":action.promote})
}

fn judge(game: &Game, nodes: u64) -> Value {
    let side = game.position().side_to_move();
    if let Some(c) = game.conclusion() {
        return json!({"side":side,"terminal_probability":match c.winner() {
            Some(w) if w == side => 1.0, Some(_) => 0.0, None => 0.5},"conclusion":c});
    }
    let start = std::time::Instant::now();
    let result = search::search(
        game,
        search::Limits {
            depth: search::MAX_DEPTH,
            nodes,
        },
    )
    .unwrap();
    json!({"side":side,"score":result.score,"depth":result.completed_depth,"nodes":result.nodes,
        "elapsed_seconds":start.elapsed().as_secs_f64(),"pv":result.principal_variation,
        "best":result.principal_variation.first().map(|a|facts(game,a))})
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let paths: Vec<String> = serde_json::from_str(&fs::read_to_string(&a[1]).unwrap()).unwrap();
    let nodes: u64 = a[3].parse().unwrap();
    let samples: usize = a[4].parse().unwrap();
    let shard: usize = a[5].parse().unwrap();
    let shards: usize = a[6].parse().unwrap();
    assert!(nodes > 0 && nodes <= search::MAX_NODES && samples > 0 && shard < shards);
    let mut out = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&a[2])
        .unwrap();
    let mut ordinal = 0;
    for path in paths {
        let text = fs::read_to_string(path).unwrap();
        let digest = mshogi_runner::experiments::digest(&text);
        let report = ValidatedReport::from_json(&text).expect("native report replay");
        for (index, result) in report.games.iter().enumerate() {
            let opening = &report.config.openings[result.pairing.opening];
            let game = record(&result.final_root.record).to_game().unwrap();
            let start = record(&opening.root.record).event_count();
            let plays: Vec<_> = game.events()[start..]
                .iter()
                .filter(|e| matches!(e.command, GameCommand::Play { .. }))
                .collect();
            let steps: Vec<_> = result.steps.iter().filter(|s| s.applied).collect();
            assert!(steps.len() == plays.len() || steps.len() == plays.len() + 1);
            for ply in selected(plays.len(), samples) {
                let assigned = ordinal % shards == shard;
                ordinal += 1;
                if !assigned {
                    continue;
                }
                let event = plays[ply];
                let before_record = game.record_prefix(event.index).unwrap();
                let before = before_record.to_game().unwrap();
                let after = game
                    .record_prefix(event.index + 1)
                    .unwrap()
                    .to_game()
                    .unwrap();
                let GameCommand::Play { action } = &event.command else {
                    unreachable!()
                };
                let row = json!({"game_id":format!("{digest}/{index}"),"source_report_sha256":digest,
                    "opening_id":opening.id,"stop":result.stop,"total_plies":plays.len(),"ply":ply,
                    "event_index":event.index,"record":before_record,"played":facts(&before,action),
                    "clock_ms":steps[ply].clock.as_ref().map(|c|c.before_ms),
                    "player_observation":steps[ply].observation,"repeated":before.occurrences().len()>1,
                    "before":judge(&before,nodes),"after":judge(&after,nodes),"judge_nodes":nodes});
                writeln!(out, "{row}").unwrap();
                out.flush().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sampling_covers_the_game_without_duplicate_or_out_of_range_indices() {
        assert!(selected(0, 16).is_empty());
        assert_eq!(selected(4, 16), vec![0, 1, 2, 3]);
        assert_eq!(selected(100, 4), vec![12, 37, 62, 87]);
        for n in 1..1025 {
            let s = selected(n, 16);
            assert!(s.windows(2).all(|p| p[0] < p[1]));
            assert!(*s.last().unwrap() < n);
        }
    }
    #[test]
    fn fresh_search_scores_use_native_side_and_zero_work_is_unjudged() {
        let g = Game::standard();
        let r = judge(&g, 0);
        assert!(r["score"].is_null());
        assert_eq!(r["side"], json!(Side::Sente));
        let action = g.legal_actions().into_iter().next().unwrap();
        let f = facts(&g, &action);
        assert_eq!(f["action"], json!(action));
    }
}
