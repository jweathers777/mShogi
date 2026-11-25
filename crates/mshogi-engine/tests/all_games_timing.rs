//! Shared time admission, retained state and deterministic fixed work.
mod support;
use mshogi_core::{chess, chu, native::CanonicalRecord, shogi, tenjiku, tori, xiangqi};
use mshogi_engine::*;
use serde_json::{Value, json};
use std::{
    io::Write,
    process::{Command, Stdio},
};

fn roots() -> Vec<CanonicalRecord> {
    let mut records = vec![
        CanonicalRecord::chess(
            chess::AdjudicatedGame::new(chess::ClaimPolicy::ExplicitOnly).record(),
        ),
        CanonicalRecord::shogi(shogi::GameRecord::from_game(&shogi::Game::new())),
        CanonicalRecord::xiangqi(
            xiangqi::GameRecord::from_game(
                &xiangqi::Game::standard(xiangqi::ClaimPolicy::Explicit).unwrap(),
            )
            .unwrap(),
        ),
        CanonicalRecord::tori(tori::GameRecord::from_game(&tori::Game::standard()).unwrap()),
        CanonicalRecord::chu(chu::GameRecord::from_game(&chu::Game::standard()).unwrap()),
    ];
    for profile in tenjiku::Profile::ALL {
        records.push(CanonicalRecord::tenjiku(
            tenjiku::GameRecord::from_game(&tenjiku::Game::standard(profile)).unwrap(),
        ));
    }
    records
}
fn request(record: &CanonicalRecord, id: u64, timed: bool) -> ExecutionRequest {
    ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: id,
            session_version: id,
            game: record.clone(),
            limits: WorkLimits {
                depth: 2,
                nodes: 5_000_000,
                time_ms: timed.then_some(60_000),
            },
        },
    }
}
fn run(requests: &[ExecutionRequest]) -> Vec<ExecutionResponse> {
    let mut input = Vec::new();
    for request in requests {
        serde_json::to_writer(&mut input, request).unwrap();
        input.push(b'\n');
    }
    let mut output = Vec::new();
    serve(input.as_slice(), &mut output).unwrap();
    let responses: Vec<ExecutionResponse> = output
        .split(|&c| c == b'\n')
        .filter(|line| !line.is_empty())
        .zip(requests)
        .map(|(line, request)| support::decode_response(line, request).unwrap())
        .collect();
    assert_eq!(responses.len(), requests.len());
    for (response, request) in responses.iter().zip(requests) {
        assert!(
            response.matches(request),
            "{:?}",
            response.observation.result
        );
    }
    responses
}
fn fingerprint(response: &ExecutionResponse) -> Value {
    let result = serde_json::to_value(&response.observation.result).unwrap();
    let native = &result["result"];
    assert_eq!(native["telemetry"]["completed_depth"], 2, "{result}");
    json!({"profile": response.observation.context.game.profile(), "action": native.get("action").or_else(|| native.get("choice")), "nodes": native["telemetry"]["nodes"], "score": native["telemetry"]["score"]})
}

/// Fixed-work searches of the standard roots are deterministic. A change that
/// moves an action, node count or score updates the recorded baseline.
#[test]
fn fixed_search_matches_recorded_baseline() {
    let expected: Vec<Value> =
        serde_json::from_str(include_str!("data/fixed-baseline.json")).unwrap();
    let requests: Vec<_> = roots()
        .iter()
        .enumerate()
        .map(|(i, record)| request(record, i as u64 + 1, false))
        .collect();
    let actual: Vec<_> = run(&requests).iter().map(fingerprint).collect();
    assert_eq!(actual, expected);
}

#[test]
fn every_profile_admits_time_and_fixed_work_stays_fresh() {
    for record in roots() {
        let cold = run(&[request(&record, 1, false)])
            .remove(0)
            .observation
            .result;
        let responses = run(&[
            request(&record, 1, true),
            request(&record, 2, false),
            request(&record, 3, false),
            request(&record, 4, true),
        ]);
        assert_eq!(responses[1].observation.result, cold);
        assert_eq!(responses[2].observation.result, cold);
        let first = fingerprint(&responses[0]);
        let warm = fingerprint(&responses[3]);
        assert!(
            warm["nodes"].as_u64().unwrap() < first["nodes"].as_u64().unwrap(),
            "no retained search state: {:?}: {first} -> {warm}",
            record.profile()
        );
    }
}

/// Apply only the returned complete action with the authoritative native rules.
fn played(record: &CanonicalRecord, response: &ExecutionResponse) -> CanonicalRecord {
    macro_rules! native {
        ($record:expr, $action:expr, $module:ident, $wrap:ident) => {{
            let mut game = $record.to_game().unwrap();
            game.submit($module::GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: $module::GameCommand::Play {
                    action: $action.clone(),
                },
            })
            .unwrap();
            CanonicalRecord::$wrap($module::GameRecord::from_game(&game).unwrap())
        }};
    }
    match (record, &response.observation.result) {
        (
            CanonicalRecord::Chess(record),
            ExecutionResult::Chess {
                result: ChessResult::Move { action, .. },
            },
        ) => {
            let mut game = chess::AdjudicatedGame::replay_record(record).unwrap();
            game.apply(*action).unwrap();
            CanonicalRecord::chess(game.record())
        }
        (
            CanonicalRecord::Shogi(record),
            ExecutionResult::Shogi {
                result:
                    ShogiResult::Choice {
                        choice: ShogiChoice::Move { action },
                        ..
                    },
            },
        ) => {
            let mut game = record.to_game().unwrap();
            game.submit(shogi::GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: shogi::GameCommand::Play(*action),
            })
            .unwrap();
            CanonicalRecord::shogi(shogi::GameRecord::from_game(&game))
        }
        (
            CanonicalRecord::Xiangqi(record),
            ExecutionResult::Xiangqi {
                result:
                    XiangqiResult::Choice {
                        choice: XiangqiChoice::Move { action },
                        ..
                    },
            },
        ) => native!(record, action, xiangqi, xiangqi),
        (
            CanonicalRecord::Tori(record),
            ExecutionResult::Tori {
                result: ToriResult::Choice { action, .. },
            },
        ) => native!(record, action, tori, tori),
        (
            CanonicalRecord::Chu(record),
            ExecutionResult::Chu {
                result: ChuResult::Choice { action, .. },
            },
        ) => native!(record, action, chu, chu),
        (
            CanonicalRecord::Tenjiku(record),
            ExecutionResult::Tenjiku {
                result: TenjikuResult::Choice { action, .. },
            },
        ) => native!(record, action, tenjiku, tenjiku),
        _ => panic!("expected a native move: {:?}", response.observation.result),
    }
}

#[test]
fn takeback_and_game_switch_clear_timed_state() {
    let all = roots();
    for (index, root) in all.iter().enumerate() {
        let cold = run(&[request(root, 1, true)]).remove(0);
        let next = played(root, &cold);
        let next_response = run(&[request(&next, 2, false)]).remove(0);
        let later = played(&next, &next_response);
        // Go forward, then back. The returned root must have the cold count.
        let responses = run(&[
            request(root, 1, true),
            request(&later, 2, true),
            request(root, 3, true),
        ]);
        assert_eq!(
            responses[2].observation.result,
            cold.observation.result,
            "takeback {:?}",
            root.profile()
        );
        // Switching to another game/profile and back starts cold too.
        let other = &all[(index + 1) % all.len()];
        let responses = run(&[
            request(root, 1, true),
            request(other, 2, true),
            request(root, 3, true),
        ]);
        assert_eq!(
            responses[2].observation.result,
            cold.observation.result,
            "switch {:?}",
            root.profile()
        );
    }
}

#[test]
#[ignore = "bounded clock measurement with explicit built executable"]
fn record_timed_native_games() {
    use std::{
        io::{BufRead, BufReader},
        time::Instant,
    };
    let binary = std::env::var("MSH179_CLOCK_BINARY").unwrap();
    let destination = std::env::var("MSH179_CLOCK_OUTPUT").unwrap();
    let mut receipts = Vec::new();
    let started = Instant::now();
    for root in roots() {
        let mut child = Command::new(&binary)
            .arg("internal-search")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let mut input = child.stdin.take().unwrap();
        let mut output = BufReader::new(child.stdout.take().unwrap());
        let mut current = root;
        let mut remaining_ms = [10_000_u64; 2];
        for ply in 0..4 {
            let mut request = request(&current, ply + 1, true);
            request.context.limits.depth = 64;
            let side = ply as usize % 2;
            let before_ms = remaining_ms[side];
            let allocation =
                mshogi_search::alphabeta::timing::TimeBudget::from_clock_ms(before_ms, 100, 30);
            let clock_hard_ms = allocation.hard.as_millis() as u64;
            // Fixed thinking time excludes Chess setup/delivery. The caller
            // reserves transport time from its shared game-clock allocation.
            let reserve_ms = if ply == 0 { 300 } else { 50 }.min(clock_hard_ms / 2);
            let requested_ms = clock_hard_ms.saturating_sub(reserve_ms);
            request.context.limits.time_ms = Some(requested_ms);
            let sent = Instant::now();
            serde_json::to_writer(&mut input, &request).unwrap();
            input.write_all(b"\n").unwrap();
            input.flush().unwrap();
            let mut line = String::new();
            output.read_line(&mut line).unwrap();
            let elapsed = sent.elapsed();
            let response: ExecutionResponse =
                support::decode_response(line.as_bytes(), &request).unwrap();
            assert!(response.matches(&request));
            current = played(&current, &response);
            let spent_ms = elapsed.as_micros().div_ceil(1_000) as u64;
            remaining_ms[side] = before_ms.saturating_sub(spent_ms).saturating_add(100);
            receipts.push(json!({"profile": current.profile(), "ply": ply + 1, "elapsed_us": elapsed.as_micros(), "requested_ms": requested_ms, "transport_setup_reserve_ms": reserve_ms, "clock_hard_ms": clock_hard_ms, "clock_before_ms": before_ms, "clock_after_ms": remaining_ms[side], "increment_ms":100, "native_admitted": true, "request": request, "response": response}));
        }
        drop(input);
        assert!(child.wait().unwrap().success());
    }
    let evidence =
        json!({"elapsed_seconds": started.elapsed().as_secs_f64(), "requests": receipts});
    std::fs::write(
        destination,
        serde_json::to_string_pretty(&evidence).unwrap() + "\n",
    )
    .unwrap();
    for receipt in evidence["requests"].as_array().unwrap() {
        assert!(
            receipt["elapsed_us"].as_u64().unwrap()
                <= receipt["clock_hard_ms"].as_u64().unwrap() * 1_000,
            "hard limit exceeded: {} ply {} took {} us",
            receipt["profile"],
            receipt["ply"],
            receipt["elapsed_us"]
        );
    }
}
