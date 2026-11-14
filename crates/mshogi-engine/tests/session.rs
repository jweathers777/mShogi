//! One engine process serves successive requests and keeps its Chess state
//! between the timed searches of one game (MSH-169).
use mshogi_core::{
    chess::{AdjudicatedGame, ClaimPolicy},
    native::CanonicalRecord,
};
use mshogi_engine::*;

fn game(moves: &[&str]) -> AdjudicatedGame {
    let mut game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    for text in moves {
        let action = game
            .position()
            .legal_actions()
            .into_iter()
            .find(|action| action.to_string() == *text)
            .unwrap_or_else(|| panic!("{text} is not legal"));
        game.apply(action).unwrap();
    }
    game
}
fn request(id: u64, game: &AdjudicatedGame, time_ms: Option<u64>) -> ExecutionRequest {
    ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: id,
            session_version: 1,
            process_generation: id,
            game: CanonicalRecord::chess(game.record()),
            // The depth limit ends each search, so the node counts are exact.
            limits: WorkLimits {
                depth: 8,
                nodes: 50_000_000,
                time_ms,
            },
        },
    }
}
/// Run the requests through one engine session; return the nodes of each.
fn nodes(requests: &[ExecutionRequest]) -> Vec<u64> {
    let mut input = Vec::new();
    for request in requests {
        input.extend(serde_json::to_vec(request).unwrap());
        input.push(b'\n');
    }
    let mut output = Vec::new();
    serve(input.as_slice(), &mut output).unwrap();
    let lines: Vec<&[u8]> = output.split(|byte| *byte == b'\n').collect();
    assert_eq!(lines.len(), requests.len() + 1, "one line for each request");
    assert!(lines[requests.len()].is_empty());
    requests
        .iter()
        .zip(lines)
        .map(|(request, line)| {
            let response: ExecutionResponse = serde_json::from_slice(line).unwrap();
            assert!(response.matches(request));
            let ExecutionResult::Chess {
                result: ChessResult::Move { telemetry, .. },
            } = response.observation.result
            else {
                panic!("no move: {:?}", response.observation.result)
            };
            assert_eq!(telemetry.completed_depth, 8);
            telemetry.nodes
        })
        .collect()
}
const TIMED: Option<u64> = Some(60_000);

#[test]
fn timed_searches_of_one_game_share_engine_state() {
    let first = game(&["e2e4", "e7e5"]);
    let later = game(&["e2e4", "e7e5", "g1f3", "b8c6"]);
    let cold = nodes(&[request(1, &later, TIMED)])[0];
    let counts = nodes(&[
        request(1, &first, TIMED),
        request(2, &first, TIMED),
        request(3, &later, TIMED),
    ]);
    // The same position again: the kept table answers most of the search.
    assert!(counts[1] * 2 < counts[0], "{counts:?}");
    // A continuation of the game: the kept state changes the search.
    assert_ne!(counts[2], cold, "{counts:?}");
}

#[test]
fn a_different_game_starts_with_empty_engine_state() {
    let first = game(&["e2e4", "e7e5"]);
    let other = game(&["d2d4", "d7d5"]);
    let shorter = game(&["e2e4"]);
    for next in [&other, &shorter] {
        let cold = nodes(&[request(1, next, TIMED)])[0];
        let counts = nodes(&[request(1, &first, TIMED), request(2, next, TIMED)]);
        assert_eq!(counts[1], cold);
    }
}

#[test]
fn searches_without_a_time_budget_stay_deterministic_and_leave_the_state() {
    let first = game(&["e2e4", "e7e5"]);
    let cold = nodes(&[request(1, &first, None)])[0];
    let counts = nodes(&[
        request(1, &first, TIMED),
        request(2, &first, None),
        request(3, &first, None),
        request(4, &first, TIMED),
    ]);
    assert_eq!(counts[1], cold);
    assert_eq!(counts[2], cold);
    // The timed state of request 1 is still there for request 4.
    assert!(counts[3] * 2 < counts[0], "{counts:?}");
}

#[test]
fn input_that_ends_before_a_request_or_inside_a_line_is_an_error() {
    assert!(serve(&b""[..], Vec::new()).is_err());
    let mut input = serde_json::to_vec(&request(1, &game(&[]), None)).unwrap();
    let mut output = Vec::new();
    assert!(serve(input.as_slice(), &mut output).is_err());
    assert!(output.is_empty());
    input.push(b'\n');
    input.extend(b"not json\n");
    let mut output = Vec::new();
    assert!(serve(input.as_slice(), &mut output).is_err());
    // The valid first request was answered before the malformed line.
    assert_eq!(output.iter().filter(|byte| **byte == b'\n').count(), 1);
}
