use crate::{adapter::for_game, *};
use mshogi_core::{
    chess::AdjudicatedGame,
    game::{CanonicalRecord, GameKind, TaggedSide},
};
use mshogi_search::{
    alphabeta::{
        Searcher,
        timing::{RequestTime, TimeBudget},
    },
    coordination::SearchCoordinator,
    search::SearchLimits,
};
use std::{
    any::Any,
    convert::Infallible,
    io::{self, BufRead, Write},
    thread,
    time::Duration,
};

/// Engine state that lasts for the life of one engine process.
///
/// The desktop keeps one engine process and sends one request for each
/// search. The state lets a timed search use the transposition table
/// and move statistics of the earlier moves of the same game. Only the
/// engine of one game is kept; a timed request for another game starts a
/// new session.
#[derive(Default)]
pub(crate) struct Session {
    kind: Option<GameKind>,
    engine: Slot<Box<dyn Reset>>,
}
impl Session {
    /// Keep only the engine of `kind`.
    fn select(&mut self, kind: GameKind) {
        if self.kind != Some(kind) {
            *self = Self::default();
            self.kind = Some(kind);
        }
    }
}
/// The engine of the session game.
fn session_engine<T: Reset>(engine: &mut Box<dyn Reset>) -> &mut T {
    let engine: &mut dyn Any = engine.as_mut();
    engine.downcast_mut().expect("engine of the session game")
}
/// New engine of a game, as the session keeps it.
fn new_engine<G: EngineGame>() -> Result<Box<dyn Reset>, Infallible> {
    Ok(Box::new(G::new_engine()))
}
/// `current` (the record of `game`) extends the record of the kept engine.
fn extends<G: EngineGame>(current: &G::Record, last: &CanonicalRecord) -> bool {
    G::record(last).is_some_and(|earlier| G::extends(current, earlier))
}
/// One lifecycle rule for every game. Fixed-limit requests do not touch this slot.
pub(crate) struct Slot<T> {
    kept: Option<T>,
    record: Option<CanonicalRecord>,
}
impl<T> Default for Slot<T> {
    fn default() -> Self {
        Self {
            kept: None,
            record: None,
        }
    }
}
pub(crate) trait Reset: Any {
    fn reset(&mut self);
}
impl Reset for Box<dyn Reset> {
    fn reset(&mut self) {
        self.as_mut().reset();
    }
}
impl Reset for SearchCoordinator {
    fn reset(&mut self) {
        self.new_game();
    }
}
impl<P: mshogi_search::alphabeta::SearchPosition + 'static> Reset for Searcher<P> {
    fn reset(&mut self) {
        self.clear();
    }
}
impl<T: Reset> Slot<T> {
    fn acquire<E>(
        &mut self,
        context: &ExecutionContext,
        prefix: impl FnOnce(usize) -> Option<CanonicalRecord>,
        create: impl FnOnce() -> Result<T, E>,
    ) -> Result<EngineState<'_, T>, E> {
        self.acquire_relation(
            &context.game,
            context.limits.time_ms.is_some(),
            |last| prefix(last.event_count()).as_ref() == Some(last),
            create,
        )
    }
    pub(crate) fn acquire_relation<E>(
        &mut self,
        record: &CanonicalRecord,
        timed: bool,
        is_prefix: impl FnOnce(&CanonicalRecord) -> bool,
        create: impl FnOnce() -> Result<T, E>,
    ) -> Result<EngineState<'_, T>, E> {
        if !timed {
            return create().map(EngineState::Fresh);
        }
        let reset = self.record.as_ref().is_none_or(|last| !is_prefix(last));
        self.record = Some(record.clone());
        self.acquire_state(reset, create)
    }
    /// Acquire an engine after the shared game session admitted its suffix.
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

/// Read one request line. `Ok(None)` is a clean end of input.
fn read_request(input: &mut impl BufRead) -> io::Result<Option<Vec<u8>>> {
    let mut bytes = Vec::new();
    input.read_until(b'\n', &mut bytes)?;
    if bytes.is_empty() {
        return Ok(None);
    }
    if bytes.last() != Some(&b'\n') {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "missing newline after internal request",
        ));
    }
    Ok(Some(bytes))
}
fn write_response(output: &mut impl Write, response: &impl serde::Serialize) -> io::Result<()> {
    let bytes = serde_json::to_vec(response).map_err(io::Error::other)?;
    output.write_all(&bytes)?;
    output.write_all(b"\n")?;
    output.flush()
}

/// Read one bounded versioned JSON line and emit one lossless final response.
///
/// Malformed/oversized envelopes produce an I/O error (there is no trustworthy
/// context to echo). Valid envelopes with unsupported settings/history return a
/// typed failure. The request runs with a new session.
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
    let mut incremental_server = crate::incremental::Server::default();
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
        dispatch_request(&request, &mut session, &mut incremental_server, &mut output)?;
        served = true;
    }
}

fn dispatch_request(
    bytes: &[u8],
    session: &mut Session,
    incremental_server: &mut crate::incremental::Server,
    output: &mut impl Write,
) -> io::Result<()> {
    if let Ok(request) = serde_json::from_slice::<crate::incremental::Request>(bytes) {
        let response = for_game!(request.game_kind(), G => {
            incremental_server.session::<G>().execute(request, |game, reset, limits, time| {
                execute_retained::<G>(session, game, reset, limits, time, None)
            })
        }, chess => crate::incremental::Response::new(bytes, ExecutionResult::failure(
            FailureKind::InvalidRequest, "Chess has no incremental session",
        )));
        return write_response(output, &response.bind(bytes));
    }
    let request: ExecutionRequest = serde_json::from_slice(bytes).map_err(io::Error::other)?;
    if crate::incremental::Client::supports(&request) {
        let record = Some(&request.context.game);
        let response = for_game!(request.context.game.game_kind(), G => {
            incremental_server.session::<G>().begin(&request, |game, reset, limits, time| {
                execute_retained::<G>(session, game, reset, limits, time, record)
            })
        }, chess => unreachable!("unsupported incremental request"));
        return write_response(output, &response.bind(bytes));
    }
    if request.context.limits.time_ms.is_some() {
        *incremental_server = crate::incremental::Server::default();
    }
    write_response(
        output,
        &crate::incremental::Response::new(bytes, request.execute(session)),
    )
}

/// Search a game that the incremental session admitted. A full first request
/// supplies its record; a suffix supplies only whether the session was reset.
fn execute_retained<G: EngineGame>(
    session: &mut Session,
    game: &G::Game,
    reset: bool,
    limits: WorkLimits,
    time: &RequestTime,
    record: Option<&CanonicalRecord>,
) -> ExecutionResult {
    session.select(G::KIND);
    let mut engine = match record {
        Some(record) => session.engine.acquire_relation(
            record,
            true,
            |last| G::record(record).is_some_and(|current| extends::<G>(current, last)),
            new_engine::<G>,
        ),
        None => session.engine.acquire_incremental(reset, new_engine::<G>),
    }
    .expect("infallible engine allocation");
    search::<G>(game, limits, time, session_engine(&mut engine))
}

pub(crate) fn execute(context: &ExecutionContext, session: &mut Session) -> ExecutionResult {
    let time = RequestTime::new(context.limits.time_ms.map(TimeBudget::from_hard_ms));
    // Untimed probes leave the timed session alone.
    let kind = context.game.game_kind();
    if context.limits.time_ms.is_some() {
        session.select(kind);
    }
    for_game!(kind, G => execute_game::<G>(context, session, &time),
        chess => ExecutionResult::Chess { result: execute_chess(context, session) })
}

/// Replay the full record and search it. A timed request keeps the engine
/// while each request extends the record of the previous one.
fn execute_game<G: EngineGame>(
    context: &ExecutionContext,
    session: &mut Session,
    time: &RequestTime,
) -> ExecutionResult {
    let record = G::record(&context.game).expect("request dispatched by game kind");
    let game = match G::to_game(record) {
        Ok(game) => game,
        Err(error) => return ExecutionResult::failure(FailureKind::InvalidRequest, error),
    };
    let mut engine = session
        .engine
        .acquire_relation(
            &context.game,
            context.limits.time_ms.is_some(),
            |last| extends::<G>(record, last),
            new_engine::<G>,
        )
        .expect("infallible engine allocation");
    search::<G>(&game, context.limits, time, session_engine(&mut engine))
}

fn search<G: EngineGame>(
    game: &G::Game,
    limits: WorkLimits,
    time: &RequestTime,
    engine: &mut Searcher<G::Position>,
) -> ExecutionResult {
    match G::search(game, limits, time.remaining(), engine, || false) {
        Ok(result) => G::wrap(result),
        Err(error) => ExecutionResult::failure(FailureKind::Search, error),
    }
}

fn execute_chess(context: &ExecutionContext, session: &mut Session) -> ChessResult {
    let CanonicalRecord::Chess(record) = &context.game else {
        unreachable!()
    };
    let game = match AdjudicatedGame::replay_record(record) {
        Ok(game) => game,
        Err(error) => return ChessResult::failure(FailureKind::InvalidRequest, error),
    };
    if let ExecutionPurpose::MoveAnalysis { target } = &context.purpose
        && (target.board_ply != game.history().ply_count() as u64
            || target.turn != TaggedSide::Chess(game.position().side_to_move()))
    {
        return ChessResult::failure(
            FailureKind::InvalidRequest,
            "analysis target provenance disagrees with its exact history",
        );
    }
    let mut coordinator = match session.engine.acquire(
        context,
        |n| record.prefix(n).map(CanonicalRecord::chess),
        || SearchCoordinator::new().map(|c| Box::new(c) as Box<dyn Reset>),
    ) {
        Ok(coordinator) => coordinator,
        Err(error) => return ChessResult::failure(FailureKind::Search, error),
    };
    let coordinator: &mut SearchCoordinator = session_engine(&mut coordinator);
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
                Ok(result) => ChessResult::searched(
                    result.clone(),
                    completion
                        .selective_depth()
                        .expect("a successful completion records its selective depth"),
                ),
                Err(error) => ChessResult::failure(FailureKind::Search, error),
            };
        }
        thread::sleep(Duration::from_millis(1));
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
        slot.acquire_relation(
            &CanonicalRecord::shogi(record.clone()),
            timed,
            |last| matches!(last, CanonicalRecord::Shogi(previous) if record.has_prefix(previous)),
            || {
                Ok::<_, Infallible>(Marker {
                    value: 7,
                    resets: 0,
                })
            },
        )
        .unwrap()
    }
    #[test]
    fn exact_shogi_relation_retains_only_timed_descendants() {
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
