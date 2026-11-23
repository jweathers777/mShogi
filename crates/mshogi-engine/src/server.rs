use crate::*;
use mshogi_core::chess::{AdjudicatedGame, AdjudicationMode};
use mshogi_search::{
    alphabeta::timing::{RequestTime, TimeBudget},
    coordination::SearchCoordinator,
    search::SearchLimits,
};
use std::{
    io::{self, BufRead, Read, Write},
    thread,
    time::Duration,
};

/// Engine state that lasts for the life of one engine process.
///
/// The desktop keeps one engine process and sends one request for each
/// search. The state lets a timed search use the transposition table
/// and move statistics of the earlier moves of the same game.
#[derive(Default)]
pub(crate) struct Session {
    kind: Option<mshogi_core::native::GameKind>,
    chess: Slot<SearchCoordinator>,
    shogi: Slot<mshogi_search::shogi::Engine>,
    xiangqi: Slot<mshogi_search::xiangqi::Engine>,
    tori: Slot<mshogi_search::tori::Engine>,
    chu: Slot<mshogi_search::chu::Engine>,
    tenjiku: Slot<mshogi_search::tenjiku::Engine>,
}
/// One lifecycle rule for every game. Fixed-limit requests do not touch this slot.
pub(crate) struct Slot<T> {
    kept: Option<T>,
    record: Option<mshogi_core::native::CanonicalRecord>,
}
impl<T> Default for Slot<T> {
    fn default() -> Self {
        Self {
            kept: None,
            record: None,
        }
    }
}
pub(crate) trait Reset {
    fn reset(&mut self);
}
impl Reset for SearchCoordinator {
    fn reset(&mut self) {
        self.new_game();
    }
}
impl<P: mshogi_search::alphabeta::SearchPosition> Reset for mshogi_search::alphabeta::Searcher<P> {
    fn reset(&mut self) {
        self.clear();
    }
}
impl<T: Reset> Slot<T> {
    fn acquire<E>(
        &mut self,
        context: &ExecutionContext,
        prefix: impl FnOnce(usize) -> Option<mshogi_core::native::CanonicalRecord>,
        create: impl FnOnce() -> Result<T, E>,
    ) -> Result<EngineState<'_, T>, E> {
        self.acquire_record(
            &context.game,
            context.limits.time_ms.is_some(),
            prefix,
            create,
        )
    }
    pub(crate) fn acquire_record<E>(
        &mut self,
        record: &mshogi_core::native::CanonicalRecord,
        timed: bool,
        prefix: impl FnOnce(usize) -> Option<mshogi_core::native::CanonicalRecord>,
        create: impl FnOnce() -> Result<T, E>,
    ) -> Result<EngineState<'_, T>, E> {
        self.acquire_relation(
            record,
            timed,
            |last| prefix(last.event_count()).as_ref() == Some(last),
            create,
        )
    }
    pub(crate) fn acquire_relation<E>(
        &mut self,
        record: &mshogi_core::native::CanonicalRecord,
        timed: bool,
        is_prefix: impl FnOnce(&mshogi_core::native::CanonicalRecord) -> bool,
        create: impl FnOnce() -> Result<T, E>,
    ) -> Result<EngineState<'_, T>, E> {
        if !timed {
            return create().map(EngineState::Fresh);
        }
        let reset = self.record.as_ref().is_none_or(|last| !is_prefix(last));
        self.record = Some(record.clone());
        self.acquire_state(reset, create)
    }
    /// Acquire an engine after the shared native session admitted its suffix.
    pub(crate) fn acquire_incremental<E>(
        &mut self,
        reset: bool,
        create: impl FnOnce() -> Result<T, E>,
    ) -> Result<EngineState<'_, T>, E> {
        self.record = None;
        self.acquire_state(reset, create)
    }
    fn acquire_state<E>(
        &mut self,
        reset: bool,
        create: impl FnOnce() -> Result<T, E>,
    ) -> Result<EngineState<'_, T>, E> {
        if let Some(state) = &mut self.kept {
            if reset {
                state.reset();
            }
        } else {
            self.kept = Some(create()?);
        }
        Ok(EngineState::Kept(self.kept.as_mut().expect("kept engine")))
    }
}
pub(crate) enum EngineState<'a, T> {
    Fresh(T),
    Kept(&'a mut T),
}
impl<T> std::ops::Deref for EngineState<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        match self {
            Self::Fresh(state) => state,
            Self::Kept(state) => state,
        }
    }
}
impl<T> std::ops::DerefMut for EngineState<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        match self {
            Self::Fresh(state) => state,
            Self::Kept(state) => state,
        }
    }
}

/// Read one bounded request line. `Ok(None)` is a clean end of input.
fn read_request(input: &mut impl BufRead) -> io::Result<Option<Vec<u8>>> {
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
    Ok(Some(bytes))
}
fn write_response(output: &mut impl Write, response: &impl serde::Serialize) -> io::Result<()> {
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
    dispatch_request(
        &request,
        &mut Session::default(),
        &mut crate::incremental::Server::default(),
        &mut output,
    )
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
    let mut native = crate::incremental::Server::default();
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
        dispatch_request(&request, &mut session, &mut native, &mut output)?;
        served = true;
    }
}

fn dispatch_request(
    bytes: &[u8],
    session: &mut Session,
    native: &mut crate::incremental::Server,
    output: &mut impl Write,
) -> io::Result<()> {
    if let Ok(request) = serde_json::from_slice::<crate::incremental::Request>(bytes) {
        let response = if request.game_kind() == mshogi_core::native::GameKind::Tori {
            native.tori.execute(request, |game, reset, limits, time| {
                execute_incremental_tori(session, game, reset, limits, time, None)
            })
        } else {
            native.shogi.execute(request, |game, reset, limits, time| {
                execute_incremental_shogi(session, game, reset, limits, time, None)
            })
        };
        return write_response(output, &response.bind(bytes));
    }
    let request: ExecutionRequest = serde_json::from_slice(bytes).map_err(io::Error::other)?;
    if crate::incremental::Client::supports(&request) {
        let record = Some(&request.context.game);
        let response = if matches!(
            request.context.game,
            mshogi_core::native::CanonicalRecord::Tori(_)
        ) {
            native.tori.begin(&request, |game, reset, limits, time| {
                execute_incremental_tori(session, game, reset, limits, time, record)
            })
        } else {
            native.shogi.begin(&request, |game, reset, limits, time| {
                execute_incremental_shogi(session, game, reset, limits, time, record)
            })
        };
        return write_response(output, &response.bind(bytes));
    }
    if request.context.limits.time_ms.is_some() {
        *native = crate::incremental::Server::default();
    }
    write_response(
        output,
        &crate::incremental::Response::new(bytes, request.execute(session)),
    )
}
fn execute_incremental_shogi(
    session: &mut Session,
    game: &mshogi_core::shogi::Game,
    reset: bool,
    limits: WorkLimits,
    time: &RequestTime,
    record: Option<&mshogi_core::native::CanonicalRecord>,
) -> ExecutionResult {
    if session.kind != Some(mshogi_core::native::GameKind::Shogi) {
        *session = Session::default();
        session.kind = Some(mshogi_core::native::GameKind::Shogi);
    }
    let create = || Ok::<_, std::convert::Infallible>(mshogi_search::shogi::new_engine());
    let mut engine = match record {
        Some(record) => session.shogi.acquire_relation(record, true,
            |last| matches!((record, last), (mshogi_core::native::CanonicalRecord::Shogi(current), mshogi_core::native::CanonicalRecord::Shogi(old)) if current.has_prefix(old)), create),
        None => session.shogi.acquire_incremental(reset, create),
    }.expect("infallible engine allocation");
    match mshogi_search::shogi::search_with_state(
        game,
        mshogi_search::shogi::Limits {
            depth: limits.depth,
            nodes: limits.nodes,
        },
        time.remaining(),
        &mut engine,
        || false,
    ) {
        Ok(found) => ExecutionResult::Shogi {
            result: ShogiResult::searched(found),
        },
        Err(error) => ExecutionResult::failure(FailureKind::Search, error),
    }
}

fn execute_incremental_tori(
    session: &mut Session,
    game: &mshogi_core::tori::Game,
    reset: bool,
    limits: WorkLimits,
    time: &RequestTime,
    record: Option<&mshogi_core::native::CanonicalRecord>,
) -> ExecutionResult {
    use mshogi_core::native::CanonicalRecord;
    if session.kind != Some(mshogi_core::native::GameKind::Tori) {
        *session = Session::default();
        session.kind = Some(mshogi_core::native::GameKind::Tori);
    }
    let create = || Ok::<_, std::convert::Infallible>(mshogi_search::tori::new_engine());
    let mut engine = match record {
        Some(record) => session.tori.acquire_relation(record, true,
            |last| matches!((record, last), (CanonicalRecord::Tori(current), CanonicalRecord::Tori(old)) if current.has_prefix(old)), create),
        None => session.tori.acquire_incremental(reset, create),
    }
    .expect("infallible engine allocation");
    match mshogi_search::tori::search_with_state(
        game,
        mshogi_search::tori::Limits {
            depth: limits.depth,
            nodes: limits.nodes,
        },
        time.remaining(),
        &mut engine,
        || false,
    ) {
        Ok(result) => ExecutionResult::Tori {
            result: ToriResult::searched(result),
        },
        Err(e) => ExecutionResult::failure(FailureKind::Search, e),
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
    let coordinator = match session.chess.acquire(
        context,
        |n| {
            record
                .prefix(n)
                .map(mshogi_core::native::CanonicalRecord::chess)
        },
        SearchCoordinator::new,
    ) {
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
    let time = RequestTime::new(context.limits.time_ms.map(TimeBudget::from_hard_ms));
    // Only one game's tables are retained. Untimed probes leave the timed session alone.
    if context.limits.time_ms.is_some() && session.kind != Some(context.game.game_kind()) {
        *session = Session::default();
        session.kind = Some(context.game.game_kind());
    }
    match context.game.game_kind() {
        mshogi_core::native::GameKind::Tenjiku => execute_tenjiku(context, session, &time),
        mshogi_core::native::GameKind::Chu => execute_chu(context, session, &time),
        mshogi_core::native::GameKind::Tori => execute_tori(context, session, &time),
        mshogi_core::native::GameKind::Xiangqi => execute_xiangqi(context, session, &time),
        mshogi_core::native::GameKind::Shogi => execute_shogi(context, session, &time),
        mshogi_core::native::GameKind::Chess => ExecutionResult::Chess {
            result: execute_chess(context, session),
        },
    }
}

fn execute_shogi(
    context: &ExecutionContext,
    session: &mut Session,
    time: &RequestTime,
) -> ExecutionResult {
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
    let mut engine = session
        .shogi
        .acquire_relation(
            &context.game,
            context.limits.time_ms.is_some(),
            |last| matches!(last, mshogi_core::native::CanonicalRecord::Shogi(previous) if record.has_prefix(previous)),
            || Ok::<_, std::convert::Infallible>(mshogi_search::shogi::new_engine()),
        )
        .expect("infallible engine allocation");
    match mshogi_search::shogi::search_with_state(
        &game,
        mshogi_search::shogi::Limits {
            depth: context.limits.depth,
            nodes: context.limits.nodes,
        },
        time.remaining(),
        &mut engine,
        || false,
    ) {
        Ok(result) => ExecutionResult::Shogi {
            result: ShogiResult::searched(result),
        },
        Err(error) => ExecutionResult::failure(FailureKind::Search, error),
    }
}

fn execute_xiangqi(
    context: &ExecutionContext,
    session: &mut Session,
    time: &RequestTime,
) -> ExecutionResult {
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
    let mut engine = session
        .xiangqi
        .acquire(
            context,
            |n| {
                game.committed_prefix(n)
                    .ok()
                    .and_then(|g| mshogi_core::xiangqi::GameRecord::from_game(&g).ok())
                    .map(mshogi_core::native::CanonicalRecord::xiangqi)
            },
            || Ok::<_, std::convert::Infallible>(mshogi_search::xiangqi::new_engine()),
        )
        .expect("infallible engine allocation");
    match mshogi_search::xiangqi::search_with_state(
        &game,
        mshogi_search::xiangqi::Limits {
            depth: context.limits.depth,
            nodes: context.limits.nodes,
        },
        time.remaining(),
        &mut engine,
        || false,
    ) {
        Ok(result) => match XiangqiResult::searched(result) {
            Ok(result) => ExecutionResult::Xiangqi { result },
            Err(error) => ExecutionResult::failure(FailureKind::Search, error),
        },
        Err(error) => ExecutionResult::failure(FailureKind::Search, error),
    }
}

fn execute_tori(
    context: &ExecutionContext,
    session: &mut Session,
    time: &RequestTime,
) -> ExecutionResult {
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
    let mut engine = session
        .tori
        .acquire(
            context,
            |n| {
                game.record_prefix(n)
                    .ok()
                    .map(mshogi_core::native::CanonicalRecord::tori)
            },
            || Ok::<_, std::convert::Infallible>(mshogi_search::tori::new_engine()),
        )
        .expect("infallible engine allocation");
    match mshogi_search::tori::search_with_state(
        &game,
        mshogi_search::tori::Limits {
            depth: context.limits.depth,
            nodes: context.limits.nodes,
        },
        time.remaining(),
        &mut engine,
        || false,
    ) {
        Ok(result) => ExecutionResult::Tori {
            result: ToriResult::searched(result),
        },
        Err(e) => ExecutionResult::failure(FailureKind::Search, e),
    }
}

fn execute_chu(
    context: &ExecutionContext,
    session: &mut Session,
    time: &RequestTime,
) -> ExecutionResult {
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
    let mut engine = session
        .chu
        .acquire(
            context,
            |n| {
                game.record_prefix(n)
                    .ok()
                    .map(mshogi_core::native::CanonicalRecord::chu)
            },
            || Ok::<_, std::convert::Infallible>(mshogi_search::chu::new_engine()),
        )
        .expect("infallible engine allocation");
    match mshogi_search::chu::search_with_state(
        &game,
        mshogi_search::chu::Limits {
            depth: context.limits.depth,
            nodes: context.limits.nodes,
        },
        time.remaining(),
        &mut engine,
        || false,
    ) {
        Ok(result) => ExecutionResult::Chu {
            result: ChuResult::searched(result),
        },
        Err(e) => ExecutionResult::failure(FailureKind::Search, e),
    }
}

fn execute_tenjiku(
    context: &ExecutionContext,
    session: &mut Session,
    time: &RequestTime,
) -> ExecutionResult {
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
    let mut engine = session
        .tenjiku
        .acquire(
            context,
            |n| {
                game.record_prefix(n)
                    .ok()
                    .map(mshogi_core::native::CanonicalRecord::tenjiku)
            },
            || Ok::<_, std::convert::Infallible>(mshogi_search::tenjiku::new_engine()),
        )
        .expect("infallible engine allocation");
    match mshogi_search::tenjiku::search_with_state(
        &game,
        mshogi_search::tenjiku::Limits {
            depth: context.limits.depth,
            nodes: context.limits.nodes,
        },
        time.remaining(),
        &mut engine,
        || false,
    ) {
        Ok(result) => ExecutionResult::Tenjiku {
            result: TenjikuResult::searched(result),
        },
        Err(e) => ExecutionResult::failure(FailureKind::Search, e),
    }
}

#[cfg(test)]
mod prefix_slot_tests {
    use super::*;
    #[derive(Debug)]
    struct Marker {
        value: u32,
        resets: u32,
    }
    impl Reset for Marker {
        fn reset(&mut self) {
            self.value = 0;
            self.resets += 1;
        }
    }
    fn acquire<'a>(
        slot: &'a mut Slot<Marker>,
        game: &mshogi_core::shogi::Game,
        timed: bool,
    ) -> EngineState<'a, Marker> {
        let record = mshogi_core::shogi::GameRecord::from_game(game);
        slot.acquire_relation(&mshogi_core::native::CanonicalRecord::shogi(record.clone()), timed,
            |last| matches!(last, mshogi_core::native::CanonicalRecord::Shogi(previous) if record.has_prefix(previous)),
            || Ok::<_, std::convert::Infallible>(Marker { value: 7, resets: 0 })).unwrap()
    }
    #[test]
    fn exact_shogi_relation_retains_only_timed_native_descendants() {
        use mshogi_core::shogi::*;
        let mut slot = Slot::default();
        let initial = Game::new();
        acquire(&mut slot, &initial, true).value = 42;
        let mut next = initial.clone();
        next.submit(GameRequest {
            expected: next.context(),
            actor: next.position().side_to_move(),
            command: GameCommand::Play("7g7f".parse().unwrap()),
        })
        .unwrap();
        assert_eq!(acquire(&mut slot, &next, true).value, 42);
        acquire(&mut slot, &initial, false).value = 99;
        assert_eq!(
            acquire(&mut slot, &next, true).value,
            42,
            "fixed work leaves timed state alone"
        );
        let reset = acquire(&mut slot, &initial, true);
        assert_eq!(
            (reset.value, reset.resets),
            (0, 1),
            "takeback clears retained state"
        );
        let imported = Game::from_root(
            GameRoot::import(
                Position::initial(),
                RootCategory::FullMaterialDiagram,
                CounterSource::Supplied,
            )
            .unwrap(),
        );
        let reset = acquire(&mut slot, &imported, true);
        assert_eq!(
            (reset.value, reset.resets),
            (0, 2),
            "root provenance cannot share state"
        );
    }
}
