//! Shared time admission, retained state and deterministic fixed work.
mod support;
use mshogi_core::{chess, chu, native::CanonicalRecord, shogi, tenjiku, tori, xiangqi};
use mshogi_engine::*;
use serde_json::{Value, json};

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
                result: ToriResult::Choice { choice: action, .. },
            },
        ) => native!(record, action, tori, tori),
        (
            CanonicalRecord::Chu(record),
            ExecutionResult::Chu {
                result: ChuResult::Choice { choice: action, .. },
            },
        ) => native!(record, action, chu, chu),
        (
            CanonicalRecord::Tenjiku(record),
            ExecutionResult::Tenjiku {
                result: TenjikuResult::Choice { choice: action, .. },
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
fn continuing_process_admits_native_suffixes_for_every_supported_profile() {
    use mshogi_engine::incremental::Client;
    use std::sync::Arc;

    for root in roots() {
        let mut client = Client::default();
        let first = Arc::new(request(&root, 1, true));
        let mut input = client.encode(&first).unwrap();
        input.push(b'\n');
        let mut initial = Vec::new();
        serve_once(input.as_slice(), &mut initial).unwrap();
        let next = played(&root, &client.decode(&initial).unwrap());
        let second = Arc::new(request(&next, 2, true));
        let line = client.encode(&second).unwrap();
        let wire: Value = serde_json::from_slice(&line).unwrap();
        if matches!(root, CanonicalRecord::Chess(_)) {
            assert!(wire.get("context").is_some());
        } else {
            assert_eq!(wire["update"]["advance"]["base"]["request_id"], 1);
            assert_eq!(
                wire["update"]["advance"]["delta"]["delta"]["start_event"],
                root.event_count()
            );
        }
        input.extend_from_slice(&line);
        input.push(b'\n');
        let mut output = Vec::new();
        serve(input.as_slice(), &mut output).unwrap();
        let response = client
            .decode(output.split(|&c| c == b'\n').nth(1).unwrap())
            .unwrap();
        assert!(response.matches(&second));
        assert!(!matches!(
            response.observation.result,
            ExecutionResult::Failure { .. }
        ));
    }
}
