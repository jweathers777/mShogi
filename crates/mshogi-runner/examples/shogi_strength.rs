//! Bounded development probes and dataset export using native Shogi authority.
use mshogi_core::{native::CanonicalRecord, shogi::*};
use mshogi_engine::{
    BridgeOptions, EngineBridge, ExecutionContext, ExecutionPurpose, ExecutionRequest,
    ExecutionResult, PROTOCOL_VERSION, ShogiChoice, ShogiResult, WorkLimits,
};
use serde_json::{Value, json};
use std::{
    collections::BTreeSet,
    fs, thread,
    time::{Duration, Instant},
};
#[path = "support/shogi_mate_proof.rs"]
mod shogi_mate_proof;

/// Most rows of one catalog.
const MAX_ROWS: usize = 1024;
/// Proof visit bound of one catalog row.
const PROOF_VISITS: u64 = 3_000_000;

fn root(p: &Value) -> Game {
    let mut game = if let Some(sfen) = p["sfen"].as_str() {
        let pos = Position::from_sfen(sfen).expect("valid SFEN");
        let imported = GameRoot::import(
            pos.clone(),
            RootCategory::FullMaterialDiagram,
            CounterSource::Supplied,
        )
        .or_else(|e| {
            if e == RootError::CategoryMismatch {
                GameRoot::import(
                    pos,
                    RootCategory::ReducedMaterialDiagram,
                    CounterSource::Supplied,
                )
            } else {
                Err(e)
            }
        })
        .expect("accepted native root");
        Game::from_root(imported)
    } else {
        Game::new()
    };
    if let Some(moves) = p["moves"].as_array() {
        for m in moves {
            game = played(&game, m.as_str().unwrap().parse().expect("USI action"));
        }
    }
    assert!(
        game.conclusion().is_none(),
        "development root must not be concluded"
    );
    game
}
fn played(game: &Game, action: Action) -> Game {
    let mut next = game.clone();
    next.submit(GameRequest {
        expected: next.context(),
        actor: next.position().side_to_move(),
        command: GameCommand::Play(action),
    })
    .expect("native action admission");
    next
}
#[cfg(test)]
fn mates(game: &Game, attacker: Side, depth: u64, visits: &mut u64) -> bool {
    *visits += 1;
    assert!(
        *visits <= 500_000,
        "proof node bound exhausted; not a failed tactic"
    );
    if let Some(end) = game.conclusion() {
        return matches!(end, Conclusion::Win {winner, reason: WinReason::NoLegalAction {in_check:true}} if winner==attacker);
    }
    if depth == 0 {
        return false;
    }
    let attacking = game.position().side_to_move() == attacker;
    let actions = game.position().legal_actions();
    if attacking {
        actions.into_iter().any(|a| {
            let next = played(game, a);
            next.position().is_in_check(next.position().side_to_move())
                && mates(&next, attacker, depth - 1, visits)
        })
    } else {
        !actions.is_empty()
            && actions
                .into_iter()
                .all(|a| mates(&played(game, a), attacker, depth - 1, visits))
    }
}
fn validate(spec: &Value) -> Value {
    let rows = spec["positions"]
        .as_array()
        .or_else(|| spec["openings"].as_array())
        .expect("position catalog");
    assert!(!rows.is_empty() && rows.len() <= MAX_ROWS);
    let mut ids = BTreeSet::new();
    let mut out = vec![];
    for p in rows {
        assert!(ids.insert(p["id"].as_str().expect("position ID")));
        let game = root(p);
        let mut found = BTreeSet::new();
        let mut visits = 0;
        if let Some(depth) = p["mate_plies"].as_u64() {
            assert!(depth % 2 == 1 && depth <= u64::from(shogi_mate_proof::MAX_PROOF_PLIES));
            // Imported diagrams have no earlier history (see the prover notes).
            assert!(p["moves"].as_array().is_none_or(|m| m.is_empty()));
            let mut prover = shogi_mate_proof::Prover::new(PROOF_VISITS);
            let answers = prover
                .answer_set(game.position(), depth as u8)
                .expect("proof node bound exhausted; not a failed tactic");
            found.extend(answers.iter().map(|a| a.to_string()));
            visits = prover.visits;
            let expected: BTreeSet<_> = p["expected_moves"]
                .as_array()
                .unwrap()
                .iter()
                .map(|s| s.as_str().unwrap().to_owned())
                .collect();
            assert!(!expected.is_empty());
            assert_eq!(
                found, expected,
                "independently authored answers differ for {}",
                p["id"]
            );
        }
        out.push(json!({"id":p["id"],"sfen":game.position().to_sfen(),"legal_actions":game.position().legal_actions().len(),"proven_checking_mates":found,"proof_visits":visits}));
    }
    json!({"validated":out,"rules_profile":"mshogi.shogi.csa36.v1"})
}
fn probe(spec: &Value, engine: &str, time_ms: u64) -> Value {
    let rows = spec["positions"].as_array().expect("positions");
    assert!(!rows.is_empty() && rows.len() <= MAX_ROWS);
    assert!(time_ms <= 60_000);
    let mut out = vec![];
    for p in rows {
        let mut game = root(p);
        let limits = WorkLimits {
            depth: if time_ms == 0 {
                p["depth"].as_u64().unwrap().try_into().unwrap()
            } else {
                64
            },
            nodes: 5_000_000,
            time_ms: if time_ms == 0 { None } else { Some(time_ms) },
        };
        let request = ExecutionRequest {
            protocol: PROTOCOL_VERSION,
            context: ExecutionContext {
                purpose: ExecutionPurpose::EngineTurn {},
                request_id: 1,
                session_version: 1,
                process_generation: 1,
                game: CanonicalRecord::shogi(GameRecord::from_game(&game)),
                limits,
            },
        };
        let bridge = EngineBridge::with_options(
            engine.into(),
            BridgeOptions {
                watchdog: Duration::from_secs(120),
                cleanup: Duration::from_secs(2),
            },
        )
        .unwrap();
        let start = Instant::now();
        bridge.submit(request.clone()).unwrap();
        let response = loop {
            if let Some(r) = bridge.poll() {
                break r;
            }
            assert!(start.elapsed() < Duration::from_secs(125), "probe watchdog");
            thread::sleep(Duration::from_millis(1));
        };
        let seconds = start.elapsed().as_secs_f64();
        bridge.shutdown();
        let cleanup = Instant::now();
        while !bridge.shutdown_acknowledged() && cleanup.elapsed() < Duration::from_secs(3) {
            thread::sleep(Duration::from_millis(1));
        }
        assert!(
            bridge.shutdown_acknowledged(),
            "probe process cleanup unresolved"
        );
        assert_eq!(response.observation.context, request.context);
        let ExecutionResult::Shogi {
            result: ShogiResult::Choice { choice, telemetry },
        } = &response.observation.result
        else {
            panic!("incomplete/failed probe: {:?}", response.observation.result)
        };
        let (bestmove, command) = match choice {
            ShogiChoice::Move { action } => (action.to_string(), GameCommand::Play(*action)),
            ShogiChoice::Declare { actor } => {
                assert_eq!(*actor, game.position().side_to_move());
                ("win".into(), GameCommand::DeclareEnteringKing)
            }
        };
        game.submit(GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command,
        })
        .expect("native chosen action");
        if time_ms == 0 {
            assert_eq!(
                telemetry.completed_depth, limits.depth,
                "fixed depth not reached"
            );
        }
        out.push(json!({"id":p["id"],"bestmove":bestmove,"depth":telemetry.completed_depth,"nodes":telemetry.nodes,
            "score":telemetry.score,"wall_seconds":seconds,"limits":limits,"observation":response.observation}));
    }
    json!({"positions":out,"timing":"full native request including cold process startup; fresh process/cache per position"})
}
fn export(report_text: &str, every: usize) -> String {
    let report =
        mshogi_runner::experiments::ValidatedReport::from_json(report_text).expect("native replay");
    let hash = mshogi_runner::experiments::digest(report_text);
    let mut out = String::new();
    for (index, g) in report.games.iter().enumerate() {
        let opening = &report.config.openings[g.pairing.opening];
        let CanonicalRecord::Shogi(initial) = &opening.root.record else {
            panic!("Shogi report required")
        };
        let CanonicalRecord::Shogi(final_record) = &g.final_root.record else {
            panic!("Shogi final required")
        };
        let initial_game = initial.to_game().unwrap();
        let mut samples = vec![];
        if let Some(score) = g.score_first_twice {
            assert!(
                g.cleanup.is_empty(),
                "cleanup failure cannot supply training labels"
            );
            assert!(g.native_outcome.is_some());
            assert_eq!(
                final_record.conclusion().unwrap().score_half_points()[0],
                score
            );
            for event in (initial.event_count()..final_record.event_count()).step_by(every) {
                let before = final_record.prefix(event).unwrap().to_game().unwrap();
                if before.conclusion().is_none() {
                    samples.push(json!({"sfen":before.position().to_sfen(),"sente_outcome":f64::from(score)/2.0,"event":event}));
                }
            }
        }
        let row = json!({"game_id":format!("{hash}/{index}"),"source_report_sha256":hash,"initial_sfen":initial_game.position().to_sfen(),
            "opening_id":opening.id,"sente_outcome":g.score_first_twice.map(|s|f64::from(s)/2.0),"outcome":g.native_outcome,
            "stop":g.stop,"truncated":g.score_first_twice.is_none(),"samples":samples});
        out.push_str(&serde_json::to_string(&row).unwrap());
        out.push('\n');
    }
    out
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let input = fs::read_to_string(&args[2]).unwrap();
    assert!(input.len() <= mshogi_runner::experiments::MAX_BYTES);
    let output = match args[1].as_str() {
        "validate" => {
            serde_json::to_string_pretty(&validate(&serde_json::from_str(&input).unwrap())).unwrap()
        }
        "probe" => serde_json::to_string_pretty(&probe(
            &serde_json::from_str(&input).unwrap(),
            &args[4],
            args[5].parse().unwrap(),
        ))
        .unwrap(),
        "export" => export(&input, args.get(4).map_or(4, |n| n.parse().unwrap())),
        _ => panic!(
            "usage: shogi_strength validate|export INPUT OUTPUT | probe INPUT OUTPUT ENGINE TIME_MS(0=depth)"
        ),
    };
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[3])
        .unwrap();
    file.write_all(output.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn independent_gold_drop_and_unprotected_silver_refutation() {
        let game = root(&json!({"sfen":"3pkp3/9/9/4S4/9/9/9/9/K8 b 2G 1","moves":[]}));
        // This fixture has the king on5a, unlike the authored three-ply root.
        assert!(game.position().is_legal("G*5b".parse().unwrap()));
        let tactical = root(&json!({"sfen":"3p1p3/4k4/9/4S4/9/9/9/9/K8 b 2G 1","moves":[]}));
        let moved = played(&tactical, "5d5c+".parse().unwrap());
        assert!(moved.position().is_legal("5b5c".parse().unwrap()));
        assert!(!mates(&moved, Side::Sente, 2, &mut 0));
        let drop = played(&tactical, "G*5c".parse().unwrap());
        assert!(!drop.position().is_legal("5b5c".parse().unwrap()));
        assert!(mates(&drop, Side::Sente, 2, &mut 0));
    }
    #[test]
    fn native_validation_of_development_catalogs() {
        for text in [
            include_str!("../../../tools/strength/data/shogi-tactics-v1.json"),
            include_str!("../../../tools/strength/data/shogi-bench-v1.json"),
            include_str!("../../../tools/strength/data/shogi-openings-v1.json"),
        ] {
            validate(&serde_json::from_str(text).unwrap());
        }
    }
    #[test]
    fn memoized_prover_agrees_with_the_plain_recursive_proof() {
        let text = include_str!("../../../tools/strength/data/shogi-tactics-v1.json");
        let spec: Value = serde_json::from_str(text).unwrap();
        for p in spec["positions"].as_array().unwrap() {
            let game = root(p);
            let depth = p["mate_plies"].as_u64().unwrap();
            let side = game.position().side_to_move();
            for a in game.position().legal_actions() {
                let next = played(&game, a);
                if !next.position().is_in_check(next.position().side_to_move()) {
                    continue;
                }
                let plain = mates(&next, side, depth - 1, &mut 0);
                let memo = shogi_mate_proof::Prover::new(1_000_000)
                    .defender_mated(next.position(), (depth - 1) as u8)
                    .unwrap();
                assert_eq!(plain, memo, "{} {a}", p["id"]);
            }
        }
    }
    #[test]
    #[should_panic(expected = "independently authored answers differ")]
    fn rejects_an_unprotected_gold_drop_as_mate() {
        validate(
            &json!({"positions":[{"id":"negative","sfen":"4k4/9/9/9/9/9/9/9/K8 b G 1",
            "moves":[],"mate_plies":1,"expected_moves":["G*5b"]}]}),
        );
    }
}
