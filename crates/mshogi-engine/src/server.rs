use crate::*;
use mshogi_core::chess::{AdjudicatedGame, AdjudicationMode};
use mshogi_search::{coordination::SearchCoordinator, search::SearchLimits};
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
    let context: crate::wire::WireRequest = serde_json::from_slice(&bytes).map_err(|error| {
        let EngineResult::Failure { failure } =
            EngineResult::failure(FailureKind::InvalidRequest, error)
        else {
            unreachable!()
        };
        io::Error::other(failure.message)
    })?;
    let bytes = serde_json::to_vec(&context.execute()).map_err(io::Error::other)?;
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
pub(crate) fn execute(context: &EngineRequest) -> EngineResult {
    if context.protocol != PROTOCOL_VERSION
        || context.request_id == 0
        || context.process_generation == 0
        || !context.valid_limits()
    {
        return EngineResult::failure(
            FailureKind::InvalidRequest,
            "invalid internal version, identity, generation or depth",
        );
    }
    if context.game.adjudication_mode() != AdjudicationMode::M2 {
        return EngineResult::failure(
            FailureKind::UnsupportedMode,
            "Live record reconstruction supports M2 only; Full records were not replayed or downgraded. Advanced Full reachability is a separate analysis operation.",
        );
    }
    let game = match AdjudicatedGame::replay_record(&context.game) {
        Ok(game) => game,
        Err(error) => return EngineResult::failure(FailureKind::InvalidRequest, error),
    };
    if let RequestPurpose::MoveAnalysis { target } | RequestPurpose::FullReachability { target, .. } =
        &context.purpose
        && (target.ply != game.history().ply_count() as u64
            || target.turn != game.position().side_to_move())
    {
        return EngineResult::failure(
            FailureKind::InvalidRequest,
            "analysis target provenance disagrees with its exact history",
        );
    }
    if let RequestPurpose::FullReachability { mate, limits, .. } = context.purpose {
        return EngineResult::Reachability {
            result: game.position().mate_reachability(
                mate.into(),
                mshogi_core::chess::ReachabilityLimits {
                    max_positions: Some(limits.max_positions),
                    max_transitions: Some(limits.max_transitions),
                },
                || false,
            ),
        };
    }
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

pub(crate) fn execute_shared(context: &SharedRequest) -> SharedResult {
    if context.protocol != SHARED_PROTOCOL_VERSION
        || context.request_id == 0
        || context.process_generation == 0
        || !context.valid_limits()
    {
        return SharedResult::failure(
            FailureKind::InvalidRequest,
            "invalid shared identity, purpose, selection or limits",
        );
    }
    match context.game.game_kind() {
        mshogi_core::native::GameKind::Xiangqi => {
            return SharedResult::failure(
                FailureKind::UnsupportedMode,
                "Xiangqi execution is not implemented: no engine-turn, move-analysis or Chess Full capability is advertised. The complete native context is retained; no move, score or game result is inferred.",
            );
        }
        mshogi_core::native::GameKind::Shogi => return execute_shogi(context),
        mshogi_core::native::GameKind::Chess => {}
    }
    let Some(chess) = context.chess_adapter() else {
        return SharedResult::failure(
            FailureKind::InvalidRequest,
            "purpose target does not belong to chess",
        );
    };
    SharedResult::Chess {
        result: execute(&chess),
    }
}

fn execute_shogi(context: &SharedRequest) -> SharedResult {
    if matches!(context.purpose, SharedPurpose::ChessFullReachability { .. }) {
        return SharedResult::failure(
            FailureKind::UnsupportedMode,
            "Chess Full reachability is not a shogi operation; no shogi Full or tsume capability is advertised.",
        );
    }
    let mshogi_core::native::CanonicalRecord::Shogi(record) = &context.game else {
        unreachable!()
    };
    let game = match record.to_game() {
        Ok(game) => game,
        Err(error) => return SharedResult::failure(FailureKind::InvalidRequest, error),
    };
    if let SharedPurpose::MoveAnalysis { target } = &context.purpose
        && (target.turn != mshogi_core::native::NativeSide::Shogi(game.position().side_to_move())
            || target.event_index != game.events().len() as u64
            || target.board_ply != record.known_board_actions() as u64)
    {
        return SharedResult::failure(
            FailureKind::InvalidRequest,
            "shogi analysis target disagrees with exact native history and side",
        );
    }
    match mshogi_search::shogi::search(
        &game,
        mshogi_search::shogi::Limits {
            depth: context.limits.depth,
            nodes: context.limits.nodes,
        },
    ) {
        Ok(result) => SharedResult::Shogi {
            result: ShogiResult::searched(result),
        },
        Err(error) => SharedResult::failure(FailureKind::Search, error),
    }
}
