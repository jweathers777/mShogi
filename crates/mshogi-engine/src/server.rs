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
    let context: ExecutionRequest = serde_json::from_slice(&bytes).map_err(|error| {
        let ChessResult::Failure { failure } =
            ChessResult::failure(FailureKind::InvalidRequest, error)
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
fn execute_chess(context: &ExecutionContext) -> ChessResult {
    let mshogi_core::native::CanonicalRecord::Chess(record) = &context.game else {
        unreachable!()
    };
    if record.adjudication_mode() != AdjudicationMode::M2 {
        return ChessResult::failure(
            FailureKind::UnsupportedMode,
            "Live record reconstruction supports M2 only; Full records were not replayed or downgraded. Advanced Full reachability is a separate analysis operation.",
        );
    }
    let game = match AdjudicatedGame::replay_record(record) {
        Ok(game) => game,
        Err(error) => return ChessResult::failure(FailureKind::InvalidRequest, error),
    };
    if let ExecutionPurpose::MoveAnalysis { target }
    | ExecutionPurpose::ChessFullReachability { target, .. } = &context.purpose
        && (target.board_ply != game.history().ply_count() as u64
            || target.turn
                != mshogi_core::native::NativeSide::Chess(game.position().side_to_move()))
    {
        return ChessResult::failure(
            FailureKind::InvalidRequest,
            "analysis target provenance disagrees with its exact history",
        );
    }
    if let ExecutionPurpose::ChessFullReachability { mate, limits, .. } = context.purpose {
        return ChessResult::Reachability {
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
        Err(error) => return ChessResult::failure(FailureKind::Search, error),
    };
    if let Err(error) = coordinator.submit(
        &game,
        context.session_version,
        SearchLimits {
            depth: context.limits.depth,
            nodes: Some(context.limits.nodes),
        },
    ) {
        return ChessResult::failure(FailureKind::Search, error);
    }
    loop {
        if let Some(completion) = coordinator.poll() {
            return match completion.result() {
                Ok(result) => ChessResult::searched(result.clone()),
                Err(error) => ChessResult::failure(FailureKind::Search, error),
            };
        }
        thread::sleep(Duration::from_millis(1));
    }
}

pub(crate) fn execute(context: &ExecutionContext) -> ExecutionResult {
    match context.game.game_kind() {
        mshogi_core::native::GameKind::Xiangqi => execute_xiangqi(context),
        mshogi_core::native::GameKind::Shogi => execute_shogi(context),
        mshogi_core::native::GameKind::Chess => ExecutionResult::Chess {
            result: execute_chess(context),
        },
    }
}

fn execute_shogi(context: &ExecutionContext) -> ExecutionResult {
    if matches!(
        context.purpose,
        ExecutionPurpose::ChessFullReachability { .. }
    ) {
        return ExecutionResult::failure(
            FailureKind::UnsupportedMode,
            "Chess Full reachability is not a shogi operation; no shogi Full or tsume capability is advertised.",
        );
    }
    let mshogi_core::native::CanonicalRecord::Shogi(record) = &context.game else {
        unreachable!()
    };
    let game = match record.to_game() {
        Ok(game) => game,
        Err(error) => return ExecutionResult::failure(FailureKind::InvalidRequest, error),
    };
    if let ExecutionPurpose::MoveAnalysis { target } = &context.purpose
        && (target.turn != mshogi_core::native::NativeSide::Shogi(game.position().side_to_move())
            || target.event_index != game.events().len() as u64
            || target.board_ply != record.known_board_actions() as u64)
    {
        return ExecutionResult::failure(
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
        Ok(result) => ExecutionResult::Shogi {
            result: ShogiResult::searched(result),
        },
        Err(error) => ExecutionResult::failure(FailureKind::Search, error),
    }
}

fn execute_xiangqi(context: &ExecutionContext) -> ExecutionResult {
    if matches!(
        context.purpose,
        ExecutionPurpose::ChessFullReachability { .. }
    ) {
        return ExecutionResult::failure(
            FailureKind::UnsupportedMode,
            "Chess Full is not Xiangqi natural-board reachability. Exact Xiangqi work remains an explicit bounded native operation, not an advertised child capability.",
        );
    }
    let mshogi_core::native::CanonicalRecord::Xiangqi(record) = &context.game else {
        unreachable!()
    };
    let game = match record.to_game() {
        Ok(game) => game,
        Err(error) => return ExecutionResult::failure(FailureKind::InvalidRequest, error),
    };
    match mshogi_search::xiangqi::search(
        &game,
        mshogi_search::xiangqi::Limits {
            depth: context.limits.depth,
            nodes: context.limits.nodes,
        },
    ) {
        Ok(result) => match XiangqiResult::searched(result) {
            Ok(result) => ExecutionResult::Xiangqi { result },
            Err(error) => ExecutionResult::failure(FailureKind::Search, error),
        },
        Err(error) => ExecutionResult::failure(FailureKind::Search, error),
    }
}
