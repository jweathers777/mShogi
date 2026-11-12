//! Bounded full-history investigation, not an opponent match or strength estimate.
//!
//! CLI: INPUTS [DEPTH] [timed|untimed] [NODES|none]. Existing defaults remain:
//! timed searches get 1000ms and no visit ceiling; untimed searches get300000visits.
//! Each input may override time_ms (timed mode only) and table_entries. `fen` is
//! the history origin; expected_root_fen, when supplied, checks the replayed root.
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy, Color, STANDARD_START_FEN};
use mshogi_search::{
    chess_conventional::{
        SearchDiagnostics, SearchMode, SearchOptions, TimeBudget, search_with_options,
    },
    chess_conventional_eval::{self as evaluation, Tapered, Terms},
    search::{MAX_DEPTH, SearchChoice, SearchDecision, SearchLimits, SearchStatus},
};
use serde_json::{Value, json};
use std::time::Instant;

fn choice_json(choice: SearchChoice) -> Value {
    match choice {
        SearchChoice::Move(action) => json!({"kind":"move", "uci":action.to_string()}),
        SearchChoice::Claim(claim) => json!({
            "kind":"claim", "actor":claim.actor, "reason":claim.reason,
            "intended_move":claim.intended.map(|a|a.to_string())
        }),
    }
}

fn tapered_json(term: Tapered) -> Value {
    json!({"middle":term.middle,"end":term.end})
}

fn terms_json(terms: Terms) -> Value {
    json!({
        "material":terms.material,
        "placement":tapered_json(terms.placement),
        "pawns":tapered_json(terms.pawns),
        "mobility":tapered_json(terms.mobility),
        "king_safety":tapered_json(terms.king_safety),
        "threats":tapered_json(terms.threats),
        "endgame":tapered_json(terms.endgame)
    })
}

fn evaluation_json(game: &AdjudicatedGame, root_side: Color) -> Value {
    let value = evaluation::evaluate(game.position());
    json!({
        "kind":"static_heuristic_not_native_outcome",
        "root_side":root_side,
        "score_root_side":value.score_for(root_side),
        "side_to_move":game.position().side_to_move(),
        "score_side_to_move":value.score_for(game.position().side_to_move()),
        "middle_phase":value.middle_phase,
        "white":terms_json(value.white),"black":terms_json(value.black),
        "specialized":value.specialized.map(|e|json!({
            "kind":format!("{:?}",e.kind),"strong_side":e.strong_side,"score_strong_side":e.score
        }))
    })
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let inputs: Value = serde_json::from_slice(&std::fs::read(&args[1]).unwrap()).unwrap();
    let depth = args.get(2).map_or(MAX_DEPTH, |x| x.parse().unwrap());
    let timed = args.get(3).is_none_or(|x| x == "timed");
    let nodes = match args.get(4).map(String::as_str) {
        Some("none") => None,
        Some(value) => Some(value.parse().unwrap()),
        None if timed => None,
        None => Some(300_000),
    };
    for input in inputs.as_array().unwrap() {
        let fen = input["fen"].as_str().unwrap_or(STANDARD_START_FEN);
        let mut game = AdjudicatedGame::from_fen(fen, ClaimPolicy::AutoClaim).unwrap();
        if let Some(history) = input["history"].as_array() {
            for action in history {
                game.apply(action.as_str().unwrap().parse().unwrap())
                    .unwrap();
            }
        }
        if let Some(expected) = input["expected_root_fen"].as_str() {
            assert_eq!(
                game.position().to_fen(),
                expected,
                "root FEN for {}",
                input["id"]
            );
        }
        let before = game.clone();
        let oracle_legal_set_verified = input.get("tb_moves").map(|moves| {
            let mut expected: Vec<_> = moves.as_object().unwrap().keys().cloned().collect();
            let mut actual: Vec<_> = game
                .position()
                .legal_actions()
                .iter()
                .map(ToString::to_string)
                .collect();
            expected.sort();
            actual.sort();
            assert_eq!(actual, expected, "oracle legal set for {}", input["id"]);
            true
        });
        // This validates a supplied line's native admission, not its optimality.
        let expected_line_replay = input.get("expected_line").map(|line| {
            let mut expected = game.clone();
            for action in line.as_array().expect("expected_line must be an array") {
                expected
                    .apply(action.as_str().unwrap().parse().unwrap())
                    .unwrap();
            }
            json!({"admitted":true,"final_fen":expected.position().to_fen(),
                "native_outcome":expected.outcome()})
        });
        let time_ms = timed.then(|| {
            input.get("time_ms").map_or(1000, |v| {
                v.as_u64().expect("time_ms must be a nonnegative integer")
            })
        });
        let requested_depth = input.get("depth").map_or(depth, |v| {
            u16::try_from(v.as_u64().expect("depth must be nonnegative")).unwrap()
        });
        let options = SearchOptions {
            allow_draw_claims: true,
            mode: match input["mode"].as_str() {
                Some("exact") => SearchMode::Exact,
                None | Some("conventional") => SearchMode::Conventional,
                _ => panic!("unknown search mode"),
            },
            diagnostics: SearchDiagnostics {
                disable_null: input["disable_null"].as_bool().unwrap_or(false),
                disable_lmr: input["disable_lmr"].as_bool().unwrap_or(false),
                disable_quiet_omissions: input["disable_quiet_omissions"]
                    .as_bool()
                    .unwrap_or(false),
                disable_see_pruning: input["disable_see_pruning"].as_bool().unwrap_or(false),
                disable_delta_pruning: input["disable_delta_pruning"].as_bool().unwrap_or(false),
                endgame_lmr: input["endgame_lmr"].as_bool().unwrap_or(false),
            },
            time: time_ms.map(TimeBudget::from_hard_ms),
            table_entries: input.get("table_entries").map_or(
                SearchOptions::default().table_entries,
                |v| {
                    usize::try_from(
                        v.as_u64()
                            .expect("table_entries must be a nonnegative integer"),
                    )
                    .unwrap()
                },
            ),
        };
        let root_side = game.position().side_to_move();
        let start = Instant::now();
        let r = search_with_options(
            &game,
            SearchLimits {
                depth: requested_depth,
                nodes,
            },
            options,
            || false,
        )
        .unwrap();
        // Exclude reporting, static diagnostic evaluation and PV replay from search time.
        let elapsed_ms = start.elapsed().as_secs_f64() * 1000.0;
        assert_eq!(before, game);
        let mut replay = game.clone();
        for choice in &r.result.principal_variation {
            match choice {
                SearchChoice::Move(a) => replay.apply(*a).unwrap(),
                SearchChoice::Claim(c) => replay.claim(*c).unwrap(),
            }
        }
        assert_eq!(before, game);
        let decision = r.result.decision.map(|decision| match decision {
            SearchDecision::Choose(choice) => choice_json(choice),
            SearchDecision::Finished(outcome) => {
                json!({"kind":"finished","native_outcome":outcome})
            }
        });
        let selected_move = match r.result.decision {
            Some(SearchDecision::Choose(SearchChoice::Move(action))) => Some(action.to_string()),
            _ => None,
        };
        let status = match r.result.status {
            SearchStatus::Complete => "complete",
            SearchStatus::NodeLimit => "node_limit",
            SearchStatus::Stopped => "stopped",
        };
        println!(
            "{}",
            json!({
                "id":input["id"],"fen":game.position().to_fen(),"elapsed_ms":elapsed_ms,
                "depth_limit":requested_depth,"mode":format!("{:?}",options.mode),"node_limit":nodes,"timed":timed,"time_ms":time_ms,
                "table_entries_requested":options.table_entries,
                "table_entries":options.table_entries.min(16_384),
                "diagnostics":format!("{:?}",options.diagnostics),
                "decision":decision,"move":selected_move,"score":r.result.score,
                "score_perspective":root_side,"completed_depth":r.result.completed_depth,
                "nodes":r.result.nodes,"status":status,
                "pv":r.result.principal_variation.iter().copied().map(choice_json).collect::<Vec<_>>(),
                "pv_final_fen":replay.position().to_fen(),"pv_native_outcome":replay.outcome(),
                "root_unchanged":true,"pv_native_admitted":true,
                "oracle_legal_set_verified":oracle_legal_set_verified,
                "expected_line_replay":expected_line_replay,
                "root_evaluation":evaluation_json(&game,root_side),
                "pv_leaf_evaluation":evaluation_json(&replay,root_side),
                "result":format!("{:?}",r.result),"seldepth":r.selective_depth,
                "stats":format!("{:?}",r.stats),
                "iterations":r.iterations.iter().map(|i|json!({"depth":i.depth,"completed":i.completed,
                    "elapsed_us":i.elapsed_us,"stats":format!("{:?}",i.stats)})).collect::<Vec<_>>()
            })
        );
    }
}
