use crate::*;
use mshogi_core::chess::{AdjudicatedGame, AdjudicationMode};
use mshogi_search::{coordination::SearchCoordinator, search::SearchLimits};
use std::{
    io::{self, BufRead, Read, Write},
    thread,
    time::Duration,
};

/// Engine state that lasts for the life of one engine process.
///
/// The desktop keeps one engine process and sends one request for each
/// search. The state lets a timed Chess search use the transposition table
/// and move statistics of the earlier moves of the same game.
#[derive(Default)]
pub(crate) struct Session {
    chess: Option<ChessSession>,
}
struct ChessSession {
    coordinator: SearchCoordinator,
    last: mshogi_core::chess::GameRecord,
}

/// Read one bounded request line. `Ok(None)` is a clean end of input.
fn read_request(input: &mut impl BufRead) -> io::Result<Option<ExecutionRequest>> {
    let mut bytes = Vec::new();
    input
        .by_ref()
        .take((MAX_REQUEST_BYTES + 2) as u64)
        .read_until(b'\n', &mut bytes)?;
    if bytes.is_empty() {
        return Ok(None);
    }
    if bytes.last() != Some(&b'\n') || bytes.len() > MAX_REQUEST_BYTES + 1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "missing newline or oversized internal request",
        ));
    }
    serde_json::from_slice(&bytes).map(Some).map_err(|error| {
        let ChessResult::Failure { failure } =
            ChessResult::failure(FailureKind::InvalidRequest, error)
        else {
            unreachable!()
        };
        io::Error::other(failure.message)
    })
}
fn write_response(output: &mut impl Write, response: &ExecutionResponse) -> io::Result<()> {
    let bytes = serde_json::to_vec(response).map_err(io::Error::other)?;
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

/// Read one bounded versioned JSON line and emit one lossless final response.
///
/// Malformed/oversized envelopes produce an I/O error (there is no trustworthy
/// context to echo). Valid envelopes with unsupported settings/history return a
/// typed failure. The request runs with a new session.
/// Full mode is rejected *before* any live replay can run reachability work.
pub fn serve_once(mut input: impl BufRead, mut output: impl Write) -> io::Result<()> {
    let Some(request) = read_request(&mut input)? else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "missing newline or oversized internal request",
        ));
    };
    write_response(&mut output, &request.execute(&mut Session::default()))
}

/// Serve request lines until the end of input, one response line for each.
///
/// This is the engine process of the desktop. One session lasts for all
/// requests. A malformed or oversized line is an I/O error and ends the
/// process, as in [`serve_once`]. Input that ends before the first request is
/// the same error; input that ends after a response is a normal end. The host
/// stops a search by stopping the process.
pub fn serve(mut input: impl BufRead, mut output: impl Write) -> io::Result<()> {
    let mut session = Session::default();
    let mut served = false;
    loop {
        let Some(request) = read_request(&mut input)? else {
            return if served {
                Ok(())
            } else {
                Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "missing newline or oversized internal request",
                ))
            };
        };
        write_response(&mut output, &request.execute(&mut session))?;
        served = true;
    }
}

/// The coordinator for this request, and whether the session keeps it.
///
/// A request with a time budget is not deterministic, so it can use the state
/// of the earlier moves. It keeps that state only when the last timed game is
/// an exact prefix of this game; a new, loaded or taken-back game starts
/// empty. A request without a time budget must give the same result each
/// time, so it always gets a new coordinator and leaves the session unchanged.
fn chess_coordinator<'a>(
    session: &'a mut Session,
    record: &mshogi_core::chess::GameRecord,
    timed: bool,
) -> Result<Coordinator<'a>, std::io::Error> {
    if !timed {
        return SearchCoordinator::new().map(Coordinator::Fresh);
    }
    match &mut session.chess {
        Some(chess) => {
            if record.prefix(chess.last.events().len()).as_ref() != Some(&chess.last) {
                chess.coordinator.new_game();
            }
            chess.last = record.clone();
        }
        None => {
            session.chess = Some(ChessSession {
                coordinator: SearchCoordinator::new()?,
                last: record.clone(),
            });
        }
    }
    Ok(Coordinator::Kept(
        &session.chess.as_ref().expect("chess session").coordinator,
    ))
}
enum Coordinator<'a> {
    Fresh(SearchCoordinator),
    Kept(&'a SearchCoordinator),
}
impl std::ops::Deref for Coordinator<'_> {
    type Target = SearchCoordinator;
    fn deref(&self) -> &SearchCoordinator {
        match self {
            Self::Fresh(coordinator) => coordinator,
            Self::Kept(coordinator) => coordinator,
        }
    }
}
fn execute_chess(context: &ExecutionContext, session: &mut Session) -> ChessResult {
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
    let coordinator = match chess_coordinator(session, record, context.limits.time_ms.is_some()) {
        Ok(coordinator) => coordinator,
        Err(error) => return ChessResult::failure(FailureKind::Search, error),
    };
    if let Err(error) = coordinator.submit_with_time(
        &game,
        context.session_version,
        SearchLimits {
            depth: context.limits.depth,
            nodes: Some(context.limits.nodes),
        },
        context
            .limits
            .time_ms
            .map(mshogi_search::chess_engine::TimeBudget::from_hard_ms),
    ) {
        return ChessResult::failure(FailureKind::Search, error);
    }
    loop {
        if let Some(completion) = coordinator.poll() {
            return match completion.result() {
                Ok(result) => ChessResult::searched(result.clone(), completion.selective_depth()),
                Err(error) => ChessResult::failure(FailureKind::Search, error),
            };
        }
        thread::sleep(Duration::from_millis(1));
    }
}

pub(crate) fn execute(context: &ExecutionContext, session: &mut Session) -> ExecutionResult {
    match context.game.game_kind() {
        mshogi_core::native::GameKind::Tenjiku => execute_tenjiku(context),
        mshogi_core::native::GameKind::Chu => execute_chu(context),
        mshogi_core::native::GameKind::Tori => execute_tori(context),
        mshogi_core::native::GameKind::Xiangqi => execute_xiangqi(context),
        mshogi_core::native::GameKind::Shogi => execute_shogi(context),
        mshogi_core::native::GameKind::Chess => ExecutionResult::Chess {
            result: execute_chess(context, session),
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

fn execute_tori(context: &ExecutionContext) -> ExecutionResult {
    if !matches!(
        context.purpose,
        ExecutionPurpose::EngineTurn {} | ExecutionPurpose::MoveAnalysis { .. }
    ) {
        return ExecutionResult::failure(
            FailureKind::UnsupportedMode,
            "Chess Full reachability is not a Tori operation; no declarations or exact analysis are advertised.",
        );
    }
    if !(mshogi_search::tori::Limits {
        depth: context.limits.depth,
        nodes: context.limits.nodes,
    })
    .valid()
    {
        return ExecutionResult::failure(
            FailureKind::InvalidRequest,
            "Tori execution limits exceed the bounded native search profile",
        );
    }
    let mshogi_core::native::CanonicalRecord::Tori(record) = &context.game else {
        unreachable!()
    };
    let game = match record.to_game() {
        Ok(game) => game,
        Err(e) => return ExecutionResult::failure(FailureKind::InvalidRequest, e),
    };
    match mshogi_search::tori::search(
        &game,
        mshogi_search::tori::Limits {
            depth: context.limits.depth,
            nodes: context.limits.nodes,
        },
    ) {
        Ok(result) => ExecutionResult::Tori {
            result: ToriResult::searched(result),
        },
        Err(e) => ExecutionResult::failure(FailureKind::Search, e),
    }
}

fn execute_chu(context: &ExecutionContext) -> ExecutionResult {
    if !matches!(
        context.purpose,
        ExecutionPurpose::EngineTurn {} | ExecutionPurpose::MoveAnalysis { .. }
    ) {
        return ExecutionResult::failure(
            FailureKind::UnsupportedMode,
            "Chess Full reachability is not a Chu operation; no declarations or exact analysis are advertised.",
        );
    }
    if !(mshogi_search::chu::Limits {
        depth: context.limits.depth,
        nodes: context.limits.nodes,
    })
    .valid()
    {
        return ExecutionResult::failure(
            FailureKind::InvalidRequest,
            "Chu execution limits exceed the bounded native search profile",
        );
    }
    let mshogi_core::native::CanonicalRecord::Chu(record) = &context.game else {
        unreachable!()
    };
    let game = match record.to_game() {
        Ok(game) => game,
        Err(e) => return ExecutionResult::failure(FailureKind::InvalidRequest, e),
    };
    match mshogi_search::chu::search(
        &game,
        mshogi_search::chu::Limits {
            depth: context.limits.depth,
            nodes: context.limits.nodes,
        },
    ) {
        Ok(result) => ExecutionResult::Chu {
            result: ChuResult::searched(result),
        },
        Err(e) => ExecutionResult::failure(FailureKind::Search, e),
    }
}

fn execute_tenjiku(context: &ExecutionContext) -> ExecutionResult {
    if !matches!(
        context.purpose,
        ExecutionPurpose::EngineTurn {} | ExecutionPurpose::MoveAnalysis { .. }
    ) {
        return ExecutionResult::failure(
            FailureKind::UnsupportedMode,
            "Chess Full reachability is not a Tenjiku operation; no declarations or exact analysis are advertised.",
        );
    }
    if !(mshogi_search::tenjiku::Limits {
        depth: context.limits.depth,
        nodes: context.limits.nodes,
    })
    .valid()
    {
        return ExecutionResult::failure(
            FailureKind::InvalidRequest,
            "Tenjiku execution limits exceed the bounded native search profile",
        );
    }
    let mshogi_core::native::CanonicalRecord::Tenjiku(record) = &context.game else {
        unreachable!()
    };
    let game = match record.to_game() {
        Ok(game) => game,
        Err(e) => return ExecutionResult::failure(FailureKind::InvalidRequest, e),
    };
    match mshogi_search::tenjiku::search(
        &game,
        mshogi_search::tenjiku::Limits {
            depth: context.limits.depth,
            nodes: context.limits.nodes,
        },
    ) {
        Ok(result) => ExecutionResult::Tenjiku {
            result: TenjikuResult::searched(result),
        },
        Err(e) => ExecutionResult::failure(FailureKind::Search, e),
    }
}
