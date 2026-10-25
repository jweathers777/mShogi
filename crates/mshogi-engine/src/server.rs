use crate::*;
use mshogi_core::chess::{AdjudicatedGame, AdjudicationMode};
use mshogi_search::{
    coordination::SearchCoordinator,
    search::{MAX_DEPTH, SearchLimits},
};
use std::{
    io::{self, BufRead, Read, Write},
    thread,
    time::Duration,
};

/// Read one bounded versioned JSON line and emit one lossless final response.
///
/// Malformed/oversized envelopes produce an I/O error (there is no trustworthy
/// context to echo). Valid envelopes with unsupported settings/history return a
/// typed failure. This one-shot process is externally terminable by its host.
/// Full mode is rejected *before* any live replay can run reachability work.
pub fn serve_once(mut input: impl BufRead, mut output: impl Write) -> io::Result<()> {
    let mut bytes = Vec::new();
    input
        .by_ref()
        .take((MAX_REQUEST_BYTES + 2) as u64)
        .read_until(b'\n', &mut bytes)?;
    if bytes.last() != Some(&b'\n') || bytes.len() > MAX_REQUEST_BYTES + 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "missing newline or oversized internal request",
        ));
    }
    let context: EngineRequest = serde_json::from_slice(&bytes).map_err(io::Error::other)?;
    let result = execute(&context);
    let bytes =
        serde_json::to_vec(&EngineResponse { context, result }).map_err(io::Error::other)?;
    if bytes.len() > MAX_RESPONSE_BYTES {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "internal response exceeds byte cap",
        ));
    }
    output.write_all(&bytes)?;
    output.write_all(b"\n")?;
    output.flush()
}
fn execute(context: &EngineRequest) -> EngineResult {
    if context.protocol != PROTOCOL_VERSION
        || context.request_id == 0
        || context.process_generation == 0
        || !(1..=MAX_DEPTH).contains(&context.limits.depth)
    {
        return EngineResult::failure(
            FailureKind::InvalidRequest,
            "invalid internal version, identity, generation or depth",
        );
    }
    if context.game.adjudication_mode() != AdjudicationMode::M2 {
        return EngineResult::failure(
            FailureKind::UnsupportedMode,
            "internal desktop play supports M2 adjudication only; Full mode was not replayed or downgraded",
        );
    }
    let game = match AdjudicatedGame::replay_record(&context.game) {
        Ok(game) => game,
        Err(error) => return EngineResult::failure(FailureKind::InvalidRequest, error),
    };
    let coordinator = match SearchCoordinator::new() {
        Ok(coordinator) => coordinator,
        Err(error) => return EngineResult::failure(FailureKind::Search, error),
    };
    if let Err(error) = coordinator.submit(
        &game,
        context.session_version,
        SearchLimits {
            depth: context.limits.depth,
            nodes: Some(context.limits.nodes),
        },
    ) {
        return EngineResult::failure(FailureKind::Search, error);
    }
    loop {
        if let Some(completion) = coordinator.poll() {
            return match completion.result() {
                Ok(result) => EngineResult::searched(result.clone()),
                Err(error) => EngineResult::failure(FailureKind::Search, error),
            };
        }
        thread::sleep(Duration::from_millis(1));
    }
}
