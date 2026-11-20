//! Compact transport for a retained native session. Saved observations still
//! contain the complete original context; only process traffic is incremental.
use crate::*;
use mshogi_core::{native::CanonicalRecord, shogi, tori};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Exact acknowledged request endpoint. A ply count alone is never identity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stamp {
    request_id: u64,
    generation: u64,
    session_version: u64,
    events: usize,
}
/// All search semantics except the native ledger, echoed without modification.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    purpose: ExecutionPurpose,
    request_id: u64,
    session_version: u64,
    process_generation: u64,
    limits: WorkLimits,
}
impl Metadata {
    /// Requested search bounds for the selected adapter.
    pub fn limits(&self) -> WorkLimits {
        self.limits
    }
    /// Search purpose and any exact target metadata.
    pub fn purpose(&self) -> &ExecutionPurpose {
        &self.purpose
    }
    /// Host-owned version for analysis target validation.
    pub fn session_version(&self) -> u64 {
        self.session_version
    }
    fn from_context(c: &ExecutionContext) -> Self {
        Self {
            purpose: c.purpose.clone(),
            request_id: c.request_id,
            session_version: c.session_version,
            process_generation: c.process_generation,
            limits: c.limits,
        }
    }
    fn stamp(&self, events: usize) -> Stamp {
        Stamp {
            request_id: self.request_id,
            generation: self.process_generation,
            session_version: self.session_version,
            events,
        }
    }
}
/// Native suffix families. Additional games use the same session protocol.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(
    tag = "kind",
    content = "delta",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum Delta {
    /// Exact native Shogi commands and derived event expectations.
    Shogi(shogi::GameDelta),
    /// Exact native Tori commands and derived event expectations (MSH-204).
    Tori(tori::GameDelta),
}
/// A full validated reset or a suffix of one exact acknowledged endpoint.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Update {
    /// Initial request, unrelated history, takeback or new process.
    Reset {
        /// Complete native record, validated by its strict decoder.
        game: CanonicalRecord,
    },
    /// Only events after a prior acknowledged request.
    Advance {
        /// Exact endpoint acknowledged by this child.
        base: Stamp,
        /// Native new events and their expected effects.
        delta: Delta,
    },
}
/// Compact request. The distinct session field separates it from full requests.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    protocol: u32,
    session: Metadata,
    update: Update,
}
/// Compact result, never an independently loadable saved observation.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Response {
    protocol: u32,
    session: Metadata,
    endpoint: Option<Stamp>,
    result: ExecutionResult,
}
/// Full response with an optional acknowledged compact-transport capability.
/// Old engines omit it and continue to receive ordinary full requests.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReadyResponse {
    /// Ordinary internal protocol identity.
    pub protocol: u32,
    /// Complete first observation, also understood by full-request consumers.
    pub observation: ExecutionObservation,
    /// Exact native endpoint prepared for subsequent suffixes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incremental: Option<Stamp>,
}
impl From<ReadyResponse> for ExecutionResponse {
    fn from(response: ReadyResponse) -> Self {
        Self {
            protocol: response.protocol,
            observation: response.observation,
        }
    }
}
impl<'de> Deserialize<'de> for ExecutionResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        ReadyResponse::deserialize(d).map(Into::into)
    }
}

/// One client state per actual child process, discarded on process retirement.
#[derive(Default)]
pub struct Client {
    previous: Option<Arc<ExecutionRequest>>,
}
impl Client {
    /// Only adopted game families use incremental timed traffic.
    pub fn supports(request: &ExecutionRequest) -> bool {
        request.context.limits.time_ms.is_some()
            && match request.context.game {
                CanonicalRecord::Shogi(_) => !matches!(
                    request.context.purpose,
                    ExecutionPurpose::ChessFullReachability { .. }
                ),
                // Tori adopts engine turns; analysis keeps full requests.
                CanonicalRecord::Tori(_) => {
                    matches!(request.context.purpose, ExecutionPurpose::EngineTurn {})
                }
                _ => false,
            }
    }
    /// Prepare bytes without advancing the acknowledged endpoint.
    pub fn encode(&self, request: &ExecutionRequest) -> Result<Vec<u8>, serde_json::Error> {
        if !Self::supports(request) || self.previous.is_none() {
            return serde_json::to_vec(request);
        }
        let update = self
            .previous
            .as_ref()
            .and_then(|previous| {
                let base = |events| Metadata::from_context(&previous.context).stamp(events);
                match (&request.context.game, &previous.context.game) {
                    (CanonicalRecord::Shogi(current), CanonicalRecord::Shogi(old))
                        if current.has_prefix(old) =>
                    {
                        Some(Update::Advance {
                            base: base(old.event_count()),
                            delta: Delta::Shogi(
                                current.suffix(old.event_count()).expect("validated prefix"),
                            ),
                        })
                    }
                    (CanonicalRecord::Tori(current), CanonicalRecord::Tori(old))
                        if current.has_prefix(old) =>
                    {
                        Some(Update::Advance {
                            base: base(old.event_count()),
                            delta: Delta::Tori(
                                current.suffix(old.event_count()).expect("validated prefix"),
                            ),
                        })
                    }
                    _ => None,
                }
            })
            .unwrap_or_else(|| Update::Reset {
                game: request.context.game.clone(),
            });
        serde_json::to_vec(&Request {
            protocol: PROTOCOL_VERSION,
            session: Metadata::from_context(&request.context),
            update,
        })
    }
    /// Check the compact echo, then attach the already owned immutable context.
    /// An invalid/stale response cannot change the acknowledged endpoint.
    pub fn decode(
        &mut self,
        bytes: &[u8],
        submitted: Arc<ExecutionRequest>,
    ) -> Result<ExecutionResponse, (FailureKind, String)> {
        if !Self::supports(&submitted) || self.previous.is_none() {
            let wire: ReadyResponse = serde_json::from_slice(bytes)
                .map_err(|e| (FailureKind::MalformedResponse, e.to_string()))?;
            let expected = Metadata::from_context(&submitted.context)
                .stamp(submitted.context.game.event_count());
            if wire
                .incremental
                .as_ref()
                .is_some_and(|stamp| stamp != &expected)
            {
                return Err((
                    FailureKind::ContextMismatch,
                    "incremental capability endpoint differs".into(),
                ));
            }
            let ready = wire.incremental.is_some();
            let response: ExecutionResponse = wire.into();
            if !response.matches(&submitted) {
                return Err((
                    FailureKind::ContextMismatch,
                    "full response context differs".into(),
                ));
            }
            if Self::supports(&submitted) && ready {
                self.previous = Some(submitted);
            } else if submitted.context.limits.time_ms.is_some() {
                self.previous = None;
            }
            return Ok(response);
        }
        let response: Response = serde_json::from_slice(bytes)
            .map_err(|e| (FailureKind::MalformedResponse, e.to_string()))?;
        let meta = Metadata::from_context(&submitted.context);
        let expected = meta.stamp(submitted.context.game.event_count());
        if response.protocol != PROTOCOL_VERSION
            || response.session != meta
            || response.endpoint.as_ref().is_some_and(|s| s != &expected)
            || (response.endpoint.is_none()
                && !matches!(response.result, ExecutionResult::Failure { .. }))
        {
            return Err((
                FailureKind::ContextMismatch,
                "incremental response identity differs".into(),
            ));
        }
        let result = ExecutionResponse {
            protocol: response.protocol,
            observation: ExecutionObservation::new(submitted.context.clone(), response.result),
        };
        if !result.matches(&submitted) {
            return Err((
                FailureKind::ContextMismatch,
                "incremental result differs from submitted context".into(),
            ));
        }
        if response.endpoint.is_some() {
            self.previous = Some(submitted);
        } else {
            self.previous = None;
        }
        Ok(result)
    }
}

/// Game adapter for the shared transport lifecycle. It supplies native types and
/// metadata checks, not another reset/advance/acknowledgement implementation.
pub trait Adapter {
    /// Native record adapter, also used by the game-independent native session.
    type Record: mshogi_core::native::incremental::IncrementalRecord;
    /// Select this game's record without reconstructing it.
    fn record(record: &CanonicalRecord) -> Option<&Self::Record>;
    /// Select this game's strictly decoded native suffix.
    fn delta(
        delta: &Delta,
    ) -> Option<&<Self::Record as mshogi_core::native::incremental::IncrementalRecord>::Delta>;
    /// Validate supported search limits and purpose before changing state.
    fn valid_limits(meta: &Metadata) -> bool;
    /// Check target identity against the admitted authoritative state.
    fn valid_target(
        meta: &Metadata,
        game: &<Self::Record as mshogi_core::native::incremental::IncrementalRecord>::Game,
    ) -> bool;
    /// Current native event count, not an identity on its own.
    fn events(
        game: &<Self::Record as mshogi_core::native::incremental::IncrementalRecord>::Game,
    ) -> usize;
}
/// First adapter; other games keep this exact session lifecycle.
pub struct ShogiAdapter;
impl Adapter for ShogiAdapter {
    type Record = shogi::GameRecord;
    fn record(record: &CanonicalRecord) -> Option<&Self::Record> {
        match record {
            CanonicalRecord::Shogi(r) => Some(r),
            _ => None,
        }
    }
    fn delta(delta: &Delta) -> Option<&shogi::GameDelta> {
        match delta {
            Delta::Shogi(d) => Some(d),
            _ => None,
        }
    }
    fn valid_limits(meta: &Metadata) -> bool {
        (mshogi_search::shogi::Limits {
            depth: meta.limits.depth,
            nodes: meta.limits.nodes,
        })
        .valid()
            && !matches!(meta.purpose, ExecutionPurpose::ChessFullReachability { .. })
    }
    fn valid_target(meta: &Metadata, game: &shogi::Game) -> bool {
        let ExecutionPurpose::MoveAnalysis { target } = &meta.purpose else {
            return true;
        };
        target.selection_id > 0
            && target.session_version == meta.session_version
            && target.event_index == game.events().len() as u64
            && target.board_ply
                == game
                    .events()
                    .iter()
                    .filter(|e| matches!(e.command, shogi::GameCommand::Play(_)))
                    .count() as u64
            && target.turn == mshogi_core::native::NativeSide::Shogi(game.position().side_to_move())
            && !target.label.is_empty()
            && target.label.len() <= 256
    }
    fn events(game: &shogi::Game) -> usize {
        game.events().len()
    }
}
/// Tori adoption of the shared session (MSH-204): engine turns only.
pub struct ToriAdapter;
impl Adapter for ToriAdapter {
    type Record = tori::GameRecord;
    fn record(record: &CanonicalRecord) -> Option<&Self::Record> {
        match record {
            CanonicalRecord::Tori(r) => Some(r),
            _ => None,
        }
    }
    fn delta(delta: &Delta) -> Option<&tori::GameDelta> {
        match delta {
            Delta::Tori(d) => Some(d),
            _ => None,
        }
    }
    fn valid_limits(meta: &Metadata) -> bool {
        (mshogi_search::tori::Limits {
            depth: meta.limits.depth,
            nodes: meta.limits.nodes,
        })
        .valid()
            && matches!(meta.purpose, ExecutionPurpose::EngineTurn {})
    }
    fn valid_target(_meta: &Metadata, _game: &tori::Game) -> bool {
        true
    }
    fn events(game: &tori::Game) -> usize {
        game.events().len()
    }
}
/// Shared transport lifecycle around any adopted native record adapter.
pub struct SessionServer<A: Adapter> {
    native: mshogi_core::native::incremental::Session<A::Record>,
    endpoint: Option<Stamp>,
}
impl<A: Adapter> Default for SessionServer<A> {
    fn default() -> Self {
        Self {
            native: Default::default(),
            endpoint: None,
        }
    }
}
/// The adopted game sessions of one engine process.
#[derive(Default)]
pub struct Server {
    /// Shogi session (MSH-189).
    pub shogi: SessionServer<ShogiAdapter>,
    /// Tori session (MSH-204).
    pub tori: SessionServer<ToriAdapter>,
}
impl Request {
    /// The game family of this request, from its full record or suffix.
    pub fn game_kind(&self) -> mshogi_core::native::GameKind {
        match &self.update {
            Update::Reset { game } => game.game_kind(),
            Update::Advance {
                delta: Delta::Shogi(_),
                ..
            } => mshogi_core::native::GameKind::Shogi,
            Update::Advance {
                delta: Delta::Tori(_),
                ..
            } => mshogi_core::native::GameKind::Tori,
        }
    }
}
impl<A: Adapter> SessionServer<A>
where
    <A::Record as mshogi_core::native::incremental::IncrementalRecord>::Error: std::fmt::Display,
{
    /// Admit the initial full request and advertise only an exact prepared endpoint.
    pub fn begin(
        &mut self,
        request: &ExecutionRequest,
        run: impl FnOnce(
            &<A::Record as mshogi_core::native::incremental::IncrementalRecord>::Game,
            bool,
            WorkLimits,
            &mshogi_search::alphabeta::timing::RequestTime,
        ) -> ExecutionResult,
    ) -> ReadyResponse {
        if !request.valid() {
            return ReadyResponse {
                protocol: PROTOCOL_VERSION,
                observation: request
                    .failure(FailureKind::InvalidRequest, "invalid initial request")
                    .observation,
                incremental: None,
            };
        }
        let response = self.execute(
            Request {
                protocol: request.protocol,
                session: Metadata::from_context(&request.context),
                update: Update::Reset {
                    game: request.context.game.clone(),
                },
            },
            run,
        );
        ReadyResponse {
            protocol: response.protocol,
            incremental: response.endpoint,
            observation: ExecutionObservation::new(request.context.clone(), response.result),
        }
    }
    /// Admit a reset/suffix and execute against the retained authoritative game.
    /// The callback also permits measurement of admission without search work.
    pub fn execute(
        &mut self,
        request: Request,
        run: impl FnOnce(
            &<A::Record as mshogi_core::native::incremental::IncrementalRecord>::Game,
            bool,
            WorkLimits,
            &mshogi_search::alphabeta::timing::RequestTime,
        ) -> ExecutionResult,
    ) -> Response {
        use mshogi_search::alphabeta::timing::{RequestTime, TimeBudget};
        let time = RequestTime::new(request.session.limits.time_ms.map(TimeBudget::from_hard_ms));
        let mut result = Response {
            protocol: PROTOCOL_VERSION,
            session: request.session.clone(),
            endpoint: None,
            result: ExecutionResult::failure(
                FailureKind::InvalidRequest,
                "invalid incremental request",
            ),
        };
        let meta = &request.session;
        if request.protocol != PROTOCOL_VERSION
            || meta.request_id == 0
            || meta.process_generation == 0
            || meta.limits.time_ms.is_none_or(|n| n > 60_000)
            || !A::valid_limits(meta)
        {
            return result;
        }
        let reset = matches!(request.update, Update::Reset { .. });
        let admission = match request.update {
            Update::Reset { ref game } => match A::record(game) {
                Some(record) => self
                    .native
                    .reset(record)
                    .map(|_| ())
                    .map_err(|e| e.to_string()),
                None => Err("game differs from incremental adapter".into()),
            },
            Update::Advance { base, ref delta } => {
                if self.endpoint.as_ref() != Some(&base)
                    || meta.request_id <= base.request_id
                    || meta.process_generation <= base.generation
                {
                    Err("incremental base differs; full reset required".into())
                } else {
                    match A::delta(delta) {
                        Some(delta) => self
                            .native
                            .advance(delta)
                            .map(|_| ())
                            .map_err(|e| e.to_string()),
                        None => Err("suffix differs from incremental adapter".into()),
                    }
                }
            }
        };
        if let Err(error) = admission {
            self.endpoint = None;
            result.result = ExecutionResult::failure(FailureKind::InvalidRequest, error);
            return result;
        }
        let game = self.native.game().expect("admitted native session");
        if !A::valid_target(meta, game) {
            self.endpoint = None;
            return result;
        }
        let stamp = meta.stamp(A::events(game));
        result.result = run(game, reset, meta.limits, &time);
        result.endpoint = Some(stamp.clone());
        self.endpoint = Some(stamp);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request(game: &shogi::Game, id: u64) -> Arc<ExecutionRequest> {
        Arc::new(ExecutionRequest {
            protocol: PROTOCOL_VERSION,
            context: ExecutionContext {
                purpose: ExecutionPurpose::EngineTurn {},
                request_id: id,
                process_generation: id,
                session_version: id,
                game: CanonicalRecord::shogi(shogi::GameRecord::from_game(game)),
                limits: WorkLimits {
                    depth: 1,
                    nodes: 0,
                    time_ms: Some(0),
                },
            },
        })
    }
    fn play(game: &mut shogi::Game, action: &str) {
        game.submit(shogi::GameRequest {
            expected: game.context(),
            actor: game.position().side_to_move(),
            command: shogi::GameCommand::Play(action.parse().unwrap()),
        })
        .unwrap();
    }
    fn cycle(
        client: &mut Client,
        server: &mut Server,
        request: Arc<ExecutionRequest>,
        expected: &shogi::Game,
    ) -> bool {
        let bytes = client.encode(&request).unwrap();
        let (reset, response_bytes) = if let Ok(wire) = serde_json::from_slice::<Request>(&bytes) {
            let reset = matches!(wire.update, Update::Reset { .. });
            let response = server.shogi.execute(wire, |game, actual_reset, _, _| {
                assert_eq!(game, expected);
                assert_eq!(actual_reset, reset);
                ExecutionResult::failure(FailureKind::Search, "admission-only test")
            });
            assert!(response.endpoint.is_some());
            (reset, serde_json::to_vec(&response).unwrap())
        } else {
            let full: ExecutionRequest = serde_json::from_slice(&bytes).unwrap();
            let response = server.shogi.begin(&full, |game, reset, _, _| {
                assert_eq!(game, expected);
                assert!(reset);
                ExecutionResult::failure(FailureKind::Search, "admission-only test")
            });
            assert!(response.incremental.is_some());
            (true, serde_json::to_vec(&response).unwrap())
        };
        let restored = client.decode(&response_bytes, request.clone()).unwrap();
        assert!(restored.matches(&request));
        assert_eq!(restored.observation.context, request.context);
        reset
    }
    #[test]
    fn continuing_requests_send_only_native_suffix_and_reset_on_history_changes() {
        let mut client = Client::default();
        let mut server = Server::default();
        let mut game = shogi::Game::new();
        assert!(cycle(&mut client, &mut server, request(&game, 1), &game));
        play(&mut game, "7g7f");
        play(&mut game, "3c3d");
        assert!(!cycle(&mut client, &mut server, request(&game, 2), &game));
        // An identical request may retain state; no command is applied twice.
        assert!(!cycle(&mut client, &mut server, request(&game, 3), &game));
        let mut takeback = shogi::Game::new();
        play(&mut takeback, "7g7f");
        assert!(cycle(
            &mut client,
            &mut server,
            request(&takeback, 4),
            &takeback
        ));
        let mut alternate = shogi::Game::new();
        play(&mut alternate, "2g2f");
        assert!(cycle(
            &mut client,
            &mut server,
            request(&alternate, 5),
            &alternate
        ));
        let imported = shogi::Game::from_root(
            shogi::GameRoot::import(
                game.position().clone(),
                shogi::RootCategory::FullMaterialDiagram,
                shogi::CounterSource::Supplied,
            )
            .unwrap(),
        );
        assert!(cycle(
            &mut client,
            &mut server,
            request(&imported, 6),
            &imported
        ));
        // A new process has no client acknowledgement, including an equal root.
        client = Client::default();
        server = Server::default();
        assert!(cycle(
            &mut client,
            &mut server,
            request(&imported, 7),
            &imported
        ));
    }
    #[test]
    fn wrong_base_illegal_suffix_and_replayed_response_cannot_advance_identity() {
        let mut client = Client::default();
        let mut server = Server::default();
        let mut game = shogi::Game::new();
        cycle(&mut client, &mut server, request(&game, 1), &game);
        let old = client.previous.clone().unwrap();
        play(&mut game, "7g7f");
        let next = request(&game, 2);
        let encoded = client.encode(&next).unwrap();
        let mut wire: Request = serde_json::from_slice(&encoded).unwrap();
        let Update::Advance { ref mut base, .. } = wire.update else {
            panic!("delta")
        };
        base.generation += 1;
        let response = server
            .shogi
            .execute(wire, |_, _, _, _| panic!("invalid base must not search"));
        assert!(response.endpoint.is_none());
        assert_eq!(server.shogi.native.game(), Some(&shogi::Game::new()));
        // Failure clears the client endpoint. The next request resynchronizes fully.
        client
            .decode(&serde_json::to_vec(&response).unwrap(), next.clone())
            .unwrap();
        assert!(cycle(&mut client, &mut server, next.clone(), &game));
        let mut forged = response;
        forged.endpoint = Some(Metadata::from_context(&old.context).stamp(0));
        assert!(
            client
                .decode(&serde_json::to_vec(&forged).unwrap(), next.clone())
                .is_err()
        );
        assert_eq!(client.previous.as_ref().unwrap().context, next.context);
        // An illegal command with valid transport identity is still native-rejected.
        play(&mut game, "3c3d");
        let next = request(&game, 3);
        let mut value: serde_json::Value =
            serde_json::from_slice(&client.encode(&next).unwrap()).unwrap();
        value["update"]["advance"]["delta"]["delta"]["events"][0]["command"]["action"] =
            serde_json::json!("3c3e");
        let wire: Request = serde_json::from_value(value).unwrap();
        let prior = server.shogi.native.game().unwrap().clone();
        let response = server
            .shogi
            .execute(wire, |_, _, _, _| panic!("illegal suffix must not search"));
        assert!(response.endpoint.is_none());
        assert_eq!(server.shogi.native.game(), Some(&prior));
    }
    #[test]
    fn fixed_work_encoding_and_timed_endpoint_are_unchanged() {
        let mut client = Client::default();
        let mut server = Server::default();
        let mut game = shogi::Game::new();
        cycle(&mut client, &mut server, request(&game, 1), &game);
        let mut fixed = (*request(&game, 2)).clone();
        fixed.context.limits.time_ms = None;
        assert_eq!(
            client.encode(&fixed).unwrap(),
            serde_json::to_vec(&fixed).unwrap()
        );
        let response = fixed.failure(FailureKind::Search, "fixed test");
        client
            .decode(&serde_json::to_vec(&response).unwrap(), Arc::new(fixed))
            .unwrap();
        play(&mut game, "7g7f");
        assert!(!cycle(&mut client, &mut server, request(&game, 3), &game));
    }
    #[test]
    fn old_full_response_without_capability_keeps_full_requests() {
        let mut client = Client::default();
        let mut game = shogi::Game::new();
        for id in 1..=3 {
            let submitted = request(&game, id);
            let bytes = client.encode(&submitted).unwrap();
            assert_eq!(bytes, serde_json::to_vec(&*submitted).unwrap());
            let old_response = submitted.failure(FailureKind::Search, "legacy peer");
            client
                .decode(&serde_json::to_vec(&old_response).unwrap(), submitted)
                .unwrap();
            assert!(client.previous.is_none());
            let action = game.position().legal_actions()[0];
            game.submit(shogi::GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: shogi::GameCommand::Play(action),
            })
            .unwrap();
        }
    }
    #[test]
    fn wrong_capability_or_unknown_response_field_never_enables_suffixes() {
        let mut client = Client::default();
        let game = shogi::Game::new();
        let submitted = request(&game, 1);
        let mut wire = ReadyResponse {
            protocol: PROTOCOL_VERSION,
            observation: submitted.failure(FailureKind::Search, "test").observation,
            incremental: Some(Metadata::from_context(&submitted.context).stamp(99)),
        };
        assert!(
            client
                .decode(&serde_json::to_vec(&wire).unwrap(), submitted.clone())
                .is_err()
        );
        assert!(client.previous.is_none());
        wire.incremental = None;
        let mut value = serde_json::to_value(wire).unwrap();
        value["unknown"] = serde_json::json!(true);
        assert!(serde_json::from_value::<ExecutionResponse>(value.clone()).is_err());
        assert!(
            client
                .decode(&serde_json::to_vec(&value).unwrap(), submitted)
                .is_err()
        );
        assert!(client.previous.is_none());
    }

    fn tori_request(game: &tori::Game, id: u64) -> Arc<ExecutionRequest> {
        Arc::new(ExecutionRequest {
            protocol: PROTOCOL_VERSION,
            context: ExecutionContext {
                purpose: ExecutionPurpose::EngineTurn {},
                request_id: id,
                process_generation: id,
                session_version: id,
                game: CanonicalRecord::tori(tori::GameRecord::from_game(game).unwrap()),
                limits: WorkLimits {
                    depth: 1,
                    nodes: 512,
                    time_ms: Some(0),
                },
            },
        })
    }
    /// One Tori request through client and server (MSH-204); returns whether
    /// the server reset its native session, and checks the game it searched.
    fn tori_cycle(
        client: &mut Client,
        server: &mut Server,
        request: Arc<ExecutionRequest>,
        expected: &tori::Game,
    ) -> bool {
        let bytes = client.encode(&request).unwrap();
        let check = |game: &tori::Game| {
            assert_eq!(
                tori::GameRecord::from_game(game).unwrap(),
                tori::GameRecord::from_game(expected).unwrap()
            );
        };
        let (reset, response_bytes) = if let Ok(wire) = serde_json::from_slice::<Request>(&bytes) {
            let reset = matches!(wire.update, Update::Reset { .. });
            let response = server.tori.execute(wire, |game, actual_reset, _, _| {
                check(game);
                assert_eq!(actual_reset, reset);
                ExecutionResult::failure(FailureKind::Search, "admission-only test")
            });
            (reset, serde_json::to_vec(&response).unwrap())
        } else {
            let full: ExecutionRequest = serde_json::from_slice(&bytes).unwrap();
            let response = server.tori.begin(&full, |game, reset, _, _| {
                check(game);
                assert!(reset);
                ExecutionResult::failure(FailureKind::Search, "admission-only test")
            });
            (true, serde_json::to_vec(&response).unwrap())
        };
        let restored = client.decode(&response_bytes, request.clone()).unwrap();
        assert!(restored.matches(&request));
        reset
    }
    #[test]
    fn tori_requests_send_only_native_suffix_and_reset_on_history_changes() {
        let play = |game: &mut tori::Game, index: usize| {
            let action = game.legal_actions()[index];
            game.submit(tori::GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: tori::GameCommand::Play { action },
            })
            .unwrap();
        };
        let mut client = Client::default();
        let mut server = Server::default();
        let mut game = tori::Game::standard();
        assert!(tori_cycle(
            &mut client,
            &mut server,
            tori_request(&game, 1),
            &game
        ));
        play(&mut game, 0);
        play(&mut game, 1);
        assert!(!tori_cycle(
            &mut client,
            &mut server,
            tori_request(&game, 2),
            &game
        ));
        assert!(!tori_cycle(
            &mut client,
            &mut server,
            tori_request(&game, 3),
            &game
        ));
        // A different history (not an extension) resets the session.
        let mut alternate = tori::Game::standard();
        play(&mut alternate, 2);
        assert!(tori_cycle(
            &mut client,
            &mut server,
            tori_request(&alternate, 4),
            &alternate
        ));
        play(&mut alternate, 0);
        assert!(!tori_cycle(
            &mut client,
            &mut server,
            tori_request(&alternate, 5),
            &alternate
        ));
    }
}
