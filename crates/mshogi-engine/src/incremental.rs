//! Compact transport for a retained game session. Saved observations still
//! contain the complete original context; only process traffic is incremental.
use crate::{adapter::for_game, *};
use mshogi_core::{
    chu,
    game::{CanonicalRecord, GameKind},
    shogi, tenjiku, tori, xiangqi,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Exact acknowledged request endpoint. A ply count alone is never identity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stamp {
    request_id: u64,
}
/// All search semantics except the ledger, echoed without modification.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Metadata {
    purpose: ExecutionPurpose,
    request_id: u64,
    session_version: u64,
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
            limits: c.limits,
        }
    }
    fn stamp(&self) -> Stamp {
        Stamp {
            request_id: self.request_id,
        }
    }
}
/// Suffix families. Additional games use the same session protocol.
#[derive(Clone, Debug, Serialize)]
#[serde(
    tag = "kind",
    content = "delta",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum Delta {
    /// Exact Shogi commands and derived event expectations.
    Shogi(shogi::GameDelta),
    /// Exact Tori commands and derived event expectations (MSH-204).
    Tori(tori::GameDelta),
    /// Exact Chu suffix (MSH-213).
    Chu(chu::GameDelta),
    /// Complete Xiangqi command groups and their game events.
    Xiangqi(xiangqi::GameDelta),
    /// Exact Tenjiku events under the retained profile.
    Tenjiku(tenjiku::GameDelta),
}
// Capture the bytes before dispatch. Serde's adjacent-tag buffering
// cannot preserve a RawValue when the content field precedes the tag.
impl<'de> Deserialize<'de> for Delta {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct TaggedDelta {
            kind: GameKind,
            delta: Box<serde_json::value::RawValue>,
        }
        let wire = TaggedDelta::deserialize(d)?;
        let text = wire.delta.get();
        match wire.kind {
            GameKind::Shogi => serde_json::from_str(text).map(Self::Shogi),
            GameKind::Xiangqi => serde_json::from_str(text).map(Self::Xiangqi),
            GameKind::Tori => serde_json::from_str(text).map(Self::Tori),
            GameKind::Chu => serde_json::from_str(text).map(Self::Chu),
            GameKind::Tenjiku => serde_json::from_str(text).map(Self::Tenjiku),
            GameKind::Chess => {
                return Err(serde::de::Error::custom("Chess has no game record suffix"));
            }
        }
        .map_err(serde::de::Error::custom)
    }
}
/// A full validated reset or a suffix of one exact acknowledged endpoint.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum Update {
    /// Initial request, unrelated history, takeback or new process.
    Reset {
        /// Complete game record, validated by its strict decoder.
        game: CanonicalRecord,
    },
    /// Only events after a prior acknowledged request.
    Advance {
        /// Exact endpoint acknowledged by this child.
        base: Stamp,
        /// New events and their expected effects.
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
/// Compact process reply bound to the exact submitted request bytes.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Response {
    protocol: u32,
    request_digest: [u8; 32],
    endpoint: Option<Stamp>,
    result: ExecutionResult,
}
impl Response {
    /// Bind a result to one request line (without the framing newline).
    pub fn new(request: &[u8], result: ExecutionResult) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            request_digest: digest(request),
            endpoint: None,
            result,
        }
    }
    /// Inspect a child result without admitting it to a host session.
    /// Live consumers must use `Client::decode` to check request identity.
    pub fn into_result(self) -> ExecutionResult {
        self.result
    }
    pub(crate) fn bind(mut self, request: &[u8]) -> Self {
        self.request_digest = digest(request);
        self
    }
}
fn digest(bytes: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    Sha256::digest(bytes.strip_suffix(b"\n").unwrap_or(bytes)).into()
}

/// One client state per actual child process, discarded on process retirement.
#[derive(Default)]
pub struct Client {
    previous: Option<Arc<ExecutionRequest>>,
    pending: Option<(Arc<ExecutionRequest>, [u8; 32])>,
}
impl Client {
    /// Only adopted game families use incremental timed traffic.
    pub fn supports(request: &ExecutionRequest) -> bool {
        request.context.limits.time_ms.is_some()
            && request.context.game.game_kind() != GameKind::Chess
    }
    /// Prepare bytes without advancing the acknowledged endpoint.
    pub fn encode(
        &mut self,
        request: &Arc<ExecutionRequest>,
    ) -> Result<Vec<u8>, serde_json::Error> {
        let bytes = self.encode_request(request)?;
        self.pending = Some((Arc::clone(request), digest(&bytes)));
        Ok(bytes)
    }
    fn encode_request(&self, request: &ExecutionRequest) -> Result<Vec<u8>, serde_json::Error> {
        if !Self::supports(request) || self.previous.is_none() {
            return serde_json::to_vec(request);
        }
        let update = self
            .previous
            .as_ref()
            .and_then(|previous| {
                let current = &request.context.game;
                let old = &previous.context.game;
                for_game!(current.game_kind(), G => {
                    let current = G::record(current)?;
                    let earlier = G::record(old)?;
                    G::extends(current, earlier).then(|| Update::Advance {
                        base: Metadata::from_context(&previous.context).stamp(),
                        delta: G::suffix(current, old.event_count()),
                    })
                }, chess => None)
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
    pub fn decode(&mut self, bytes: &[u8]) -> Result<ExecutionResponse, (FailureKind, String)> {
        let malformed = |error: String| (FailureKind::MalformedResponse, error);
        let text = std::str::from_utf8(bytes).map_err(|e| malformed(e.to_string()))?;
        crate::shared::reject_observation_duplicates(text, &mut || false).map_err(malformed)?;
        let wire: Response = serde_json::from_str(text).map_err(|e| malformed(e.to_string()))?;
        let Some((submitted, expected_digest)) = &self.pending else {
            return Err((
                FailureKind::ContextMismatch,
                "no pending encoded request".into(),
            ));
        };
        let supports = Self::supports(submitted);
        let expected = Metadata::from_context(&submitted.context).stamp();
        if wire.protocol != PROTOCOL_VERSION
            || wire.request_digest != *expected_digest
            || wire
                .endpoint
                .as_ref()
                .is_some_and(|stamp| !supports || stamp != &expected)
            || (supports
                && wire.endpoint.is_none()
                && !matches!(wire.result, ExecutionResult::Failure { .. }))
        {
            return Err((
                FailureKind::ContextMismatch,
                "response differs from encoded request".into(),
            ));
        }
        let response = ExecutionResponse {
            protocol: wire.protocol,
            observation: ExecutionObservation::new(submitted.context.clone(), wire.result),
        };
        if !response.matches(submitted) {
            return Err((
                FailureKind::ContextMismatch,
                "result differs from submitted context".into(),
            ));
        }
        let (submitted, _) = self.pending.take().expect("checked pending request");
        if supports && wire.endpoint.is_some() {
            self.previous = Some(submitted);
        } else if submitted.context.limits.time_ms.is_some() {
            self.previous = None;
        }
        Ok(response)
    }
}

/// Shared transport lifecycle around any adopted game record adapter.
pub struct SessionServer<A: EngineGame> {
    game: mshogi_core::game::incremental::Session<A::Record>,
    endpoint: Option<Stamp>,
}
impl<A: EngineGame> Default for SessionServer<A> {
    fn default() -> Self {
        Self {
            game: Default::default(),
            endpoint: None,
        }
    }
}
/// A process retains just the active game's authenticated endpoint.
#[derive(Default)]
pub struct Server {
    active: Option<Box<dyn std::any::Any>>,
}
impl Server {
    pub(crate) fn session<G: EngineGame>(&mut self) -> &mut SessionServer<G> {
        if self
            .active
            .as_ref()
            .is_none_or(|s| !s.is::<SessionServer<G>>())
        {
            self.active = Some(Box::new(SessionServer::<G>::default()));
        }
        self.active
            .as_mut()
            .unwrap()
            .downcast_mut()
            .expect("selected game session")
    }
}
impl Request {
    /// The game family of this request, from its full record or suffix.
    pub fn game_kind(&self) -> mshogi_core::game::GameKind {
        match &self.update {
            Update::Reset { game } => game.game_kind(),
            Update::Advance { delta, .. } => match delta {
                Delta::Shogi(_) => GameKind::Shogi,
                Delta::Tori(_) => GameKind::Tori,
                Delta::Chu(_) => GameKind::Chu,
                Delta::Xiangqi(_) => GameKind::Xiangqi,
                Delta::Tenjiku(_) => GameKind::Tenjiku,
            },
        }
    }
}
impl<A: EngineGame> SessionServer<A> {
    /// Admit the initial full request and advertise only an exact prepared endpoint.
    pub fn begin(
        &mut self,
        request: &ExecutionRequest,
        run: impl FnOnce(
            &A::Game,
            bool,
            WorkLimits,
            &mshogi_search::alphabeta::timing::RequestTime,
        ) -> ExecutionResult,
    ) -> Response {
        if !request.valid() {
            return Response::new(
                &[],
                ExecutionResult::failure(FailureKind::InvalidRequest, "invalid initial request"),
            );
        }
        self.execute(
            Request {
                protocol: request.protocol,
                session: Metadata::from_context(&request.context),
                update: Update::Reset {
                    game: request.context.game.clone(),
                },
            },
            run,
        )
    }

    /// Admit a reset/suffix and execute against the retained authoritative game.
    /// The callback also permits measurement of admission without search work.
    pub fn execute(
        &mut self,
        request: Request,
        run: impl FnOnce(
            &A::Game,
            bool,
            WorkLimits,
            &mshogi_search::alphabeta::timing::RequestTime,
        ) -> ExecutionResult,
    ) -> Response {
        use mshogi_search::alphabeta::timing::{RequestTime, TimeBudget};
        let time = RequestTime::new(request.session.limits.time_ms.map(TimeBudget::from_hard_ms));
        let mut result = Response {
            protocol: PROTOCOL_VERSION,
            request_digest: [0; 32],
            endpoint: None,
            result: ExecutionResult::failure(
                FailureKind::InvalidRequest,
                "invalid incremental request",
            ),
        };
        let meta = &request.session;
        if request.protocol != PROTOCOL_VERSION
            || meta.request_id == 0
            || meta.limits.time_ms.is_none_or(|n| n > 60_000)
            || !A::valid_limits(meta.limits)
        {
            return result;
        }
        let reset = matches!(request.update, Update::Reset { .. });
        let admission = match request.update {
            Update::Reset { ref game } => match A::record(game) {
                Some(record) => self
                    .game
                    .reset(record)
                    .map(|_| ())
                    .map_err(|e| e.to_string()),
                None => Err("game differs from incremental adapter".into()),
            },
            Update::Advance { base, ref delta } => {
                if self.endpoint.as_ref() != Some(&base) || meta.request_id <= base.request_id {
                    Err("incremental base differs; full reset required".into())
                } else {
                    match A::delta(delta) {
                        Some(delta) => self
                            .game
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
        let game = self.game.game().expect("admitted game session");
        if let ExecutionPurpose::MoveAnalysis { target } = &meta.purpose
            && !target.matches(
                meta.session_version,
                A::progress(game),
                A::tagged_side(A::side_to_move(game)),
            )
        {
            self.endpoint = None;
            return result;
        }
        let stamp = meta.stamp();
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
            let response =
                server
                    .session::<game::Shogi>()
                    .execute(wire, |game, actual_reset, _, _| {
                        assert_eq!(game, expected);
                        assert_eq!(actual_reset, reset);
                        ExecutionResult::failure(FailureKind::Search, "admission-only test")
                    });
            assert!(response.endpoint.is_some());
            (reset, serde_json::to_vec(&response.bind(&bytes)).unwrap())
        } else {
            let full: ExecutionRequest = serde_json::from_slice(&bytes).unwrap();
            let response = server
                .session::<game::Shogi>()
                .begin(&full, |game, reset, _, _| {
                    assert_eq!(game, expected);
                    assert!(reset);
                    ExecutionResult::failure(FailureKind::Search, "admission-only test")
                });
            assert!(response.endpoint.is_some());
            (true, serde_json::to_vec(&response.bind(&bytes)).unwrap())
        };
        let restored = client.decode(&response_bytes).unwrap();
        assert!(restored.matches(&request));
        assert_eq!(restored.observation.context, request.context);
        reset
    }
    #[test]
    fn continuing_requests_send_only_suffix_and_reset_on_history_changes() {
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
        base.request_id += 1;
        let response = server
            .session::<game::Shogi>()
            .execute(wire, |_, _, _, _| panic!("invalid base must not search"));
        assert!(response.endpoint.is_none());
        assert_eq!(
            server.session::<game::Shogi>().game.game(),
            Some(&shogi::Game::new())
        );
        // Failure clears the client endpoint. The next request resynchronizes fully.
        client
            .decode(&serde_json::to_vec(&response.clone().bind(&encoded)).unwrap())
            .unwrap();
        assert!(cycle(&mut client, &mut server, next.clone(), &game));
        let mut forged = response;
        forged.endpoint = Some(Metadata::from_context(&old.context).stamp());
        assert!(
            client
                .decode(&serde_json::to_vec(&forged.bind(&encoded)).unwrap())
                .is_err()
        );
        assert_eq!(client.previous.as_ref().unwrap().context, next.context);
        // An illegal command with valid transport identity is still rejected.
        play(&mut game, "3c3d");
        let next = request(&game, 3);
        let mut value: serde_json::Value =
            serde_json::from_slice(&client.encode(&next).unwrap()).unwrap();
        value["update"]["advance"]["delta"]["delta"]["events"][0]["command"]["action"] =
            serde_json::json!("3c3e");
        let wire: Request = serde_json::from_value(value).unwrap();
        let prior = server.session::<game::Shogi>().game.game().unwrap().clone();
        let response = server
            .session::<game::Shogi>()
            .execute(wire, |_, _, _, _| panic!("illegal suffix must not search"));
        assert!(response.endpoint.is_none());
        assert_eq!(server.session::<game::Shogi>().game.game(), Some(&prior));
    }
    #[test]
    fn fixed_work_encoding_and_timed_endpoint_are_unchanged() {
        let mut client = Client::default();
        let mut server = Server::default();
        let mut game = shogi::Game::new();
        cycle(&mut client, &mut server, request(&game, 1), &game);
        let mut fixed = (*request(&game, 2)).clone();
        fixed.context.limits.time_ms = None;
        let fixed = Arc::new(fixed);
        assert_eq!(
            client.encode(&fixed).unwrap(),
            serde_json::to_vec(&*fixed).unwrap()
        );
        let response = Response::new(
            &client.encode(&fixed).unwrap(),
            ExecutionResult::failure(FailureKind::Search, "fixed test"),
        );
        client
            .decode(&serde_json::to_vec(&response).unwrap())
            .unwrap();
        play(&mut game, "7g7f");
        assert!(!cycle(&mut client, &mut server, request(&game, 3), &game));
    }
    #[test]
    fn response_requires_exact_pending_bytes_and_is_consumed_once() {
        let mut client = Client::default();
        let game = shogi::Game::new();
        let submitted = request(&game, 1);
        let bytes = client.encode(&submitted).unwrap();
        let mut wire = Response::new(
            &bytes,
            ExecutionResult::failure(FailureKind::Search, "test"),
        );
        wire.endpoint = Some(Stamp { request_id: 99 });
        assert!(client.decode(&serde_json::to_vec(&wire).unwrap()).is_err());
        assert!(client.previous.is_none());
        wire.endpoint = None;
        let mut value = serde_json::to_value(&wire).unwrap();
        value["unknown"] = serde_json::json!(true);
        assert!(client.decode(&serde_json::to_vec(&value).unwrap()).is_err());
        let mut other = (*submitted).clone();
        other.context.limits.nodes += 1;
        let wrong = Response::new(&serde_json::to_vec(&other).unwrap(), wire.result.clone());
        assert!(client.decode(&serde_json::to_vec(&wrong).unwrap()).is_err());
        let encoded = serde_json::to_vec(&wire).unwrap();
        client.decode(&encoded).unwrap();
        assert!(client.decode(&encoded).is_err());
        assert!(client.previous.is_none());
    }

    #[test]
    fn digest_binds_all_context_fields_and_rejects_duplicate_results() {
        let submitted = request(&shogi::Game::new(), 1);
        let mut client = Client::default();
        let bytes = client.encode(&submitted).unwrap();
        let failure = ExecutionResult::failure(FailureKind::Search, "test");
        let original = serde_json::to_value(&*submitted).unwrap();
        for (pointer, value) in [
            ("/context/request_id", serde_json::json!(2)),
            ("/context/session_version", serde_json::json!(2)),
            ("/context/limits/nodes", serde_json::json!(123)),
            ("/context/purpose", serde_json::json!({"kind":"other"})),
            ("/context/game", serde_json::json!({"game_kind":"other"})),
        ] {
            let mut other = original.clone();
            *other.pointer_mut(pointer).unwrap() = value;
            let forged = Response::new(&serde_json::to_vec(&other).unwrap(), failure.clone());
            assert!(
                client
                    .decode(&serde_json::to_vec(&forged).unwrap())
                    .is_err(),
                "{pointer}"
            );
        }
        // Two valid complete profiles with the same request ID cannot cross.
        for profile in mshogi_core::tenjiku::Profile::ALL {
            let mut other = (*submitted).clone();
            other.context.game = CanonicalRecord::tenjiku(
                mshogi_core::tenjiku::GameRecord::from_game(&mshogi_core::tenjiku::Game::standard(
                    profile,
                ))
                .unwrap(),
            );
            let forged = Response::new(&serde_json::to_vec(&other).unwrap(), failure.clone());
            assert!(
                client
                    .decode(&serde_json::to_vec(&forged).unwrap())
                    .is_err()
            );
        }
        let response = Response::new(&bytes, failure);
        let text = serde_json::to_string(&response).unwrap();
        let duplicate = text.replace(
            "\"kind\":\"failure\"",
            "\"kind\":\"failure\",\"kind\":\"failure\"",
        );
        assert_ne!(duplicate, text);
        assert!(client.decode(duplicate.as_bytes()).is_err());
        // Invalid replies leave the exact pending request available for its reply.
        client.decode(text.as_bytes()).unwrap();
        let next = request(&shogi::Game::new(), 2);
        client.encode(&next).unwrap();
        assert!(client.decode(text.as_bytes()).is_err());
    }

    fn tori_request(game: &tori::Game, id: u64) -> Arc<ExecutionRequest> {
        Arc::new(ExecutionRequest {
            protocol: PROTOCOL_VERSION,
            context: ExecutionContext {
                purpose: ExecutionPurpose::EngineTurn {},
                request_id: id,
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
    /// the server reset its game session, and checks the game it searched.
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
            let response =
                server
                    .session::<game::Tori>()
                    .execute(wire, |game, actual_reset, _, _| {
                        check(game);
                        assert_eq!(actual_reset, reset);
                        ExecutionResult::failure(FailureKind::Search, "admission-only test")
                    });
            (reset, serde_json::to_vec(&response.bind(&bytes)).unwrap())
        } else {
            let full: ExecutionRequest = serde_json::from_slice(&bytes).unwrap();
            let response = server
                .session::<game::Tori>()
                .begin(&full, |game, reset, _, _| {
                    check(game);
                    assert!(reset);
                    ExecutionResult::failure(FailureKind::Search, "admission-only test")
                });
            (true, serde_json::to_vec(&response.bind(&bytes)).unwrap())
        };
        let restored = client.decode(&response_bytes).unwrap();
        assert!(restored.matches(&request));
        reset
    }
    #[test]
    fn tori_requests_send_only_suffix_and_reset_on_history_changes() {
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
    #[test]
    fn chu_uses_exact_acknowledged_suffix_and_resets_for_takeback() {
        let mut game = chu::Game::standard();
        let mut client = Client::default();
        let mut server = Server::default();
        let root = chu::GameRecord::from_game(&game).unwrap();
        for id in 1..=4 {
            if id == 4 {
                game = root.to_game().unwrap();
            }
            let record = chu::GameRecord::from_game(&game).unwrap();
            let request = Arc::new(ExecutionRequest {
                protocol: PROTOCOL_VERSION,
                context: ExecutionContext {
                    purpose: ExecutionPurpose::EngineTurn {},
                    request_id: id,
                    session_version: id,
                    game: CanonicalRecord::chu(record.clone()),
                    limits: WorkLimits {
                        depth: 1,
                        nodes: 512,
                        time_ms: Some(100),
                    },
                },
            });
            let bytes = client.encode(&request).unwrap();
            let check =
                |g: &chu::Game,
                 reset: bool,
                 _: WorkLimits,
                 _: &mshogi_search::alphabeta::timing::RequestTime| {
                    assert_eq!(chu::GameRecord::from_game(g).unwrap(), record);
                    assert_eq!(reset, id == 1 || id == 4);
                    ExecutionResult::failure(FailureKind::Search, "admission-only test")
                };
            let response = if id == 1 {
                server.session::<game::Chu>().begin(&request, check)
            } else {
                let wire: Request = serde_json::from_slice(&bytes).unwrap();
                assert_eq!(matches!(wire.update, Update::Advance { .. }), id != 4);
                server.session::<game::Chu>().execute(wire, check)
            };
            let encoded = serde_json::to_vec(&response.bind(&bytes)).unwrap();
            assert!(client.decode(&encoded).is_ok());
            assert!(client.decode(&encoded).is_err());
            let action = game.legal_actions()[0].clone();
            game.submit(chu::GameRequest {
                expected: game.context(),
                actor: game.position().side_to_move(),
                command: chu::GameCommand::Play { action },
            })
            .unwrap();
        }
    }
    #[test]
    fn full_and_compact_replies_require_acknowledgement() {
        let game = chu::Game::standard();
        let mut client = Client::default();
        let result = ExecutionResult::Chu {
            result: ChuResult::Incomplete {
                telemetry: ChuTelemetry {
                    evaluator_id: mshogi_search::chu::EVALUATOR_ID.into(),
                    search_id: mshogi_search::chu::SEARCH_ID.into(),
                    perspective: chu::Side::Sente,
                    score: None,
                    completed_depth: 0,
                    nodes: 0,
                    principal_variation: vec![],
                    status: StopReason::Stopped,
                },
            },
        };
        for id in 1..=3 {
            let request = Arc::new(ExecutionRequest {
                protocol: PROTOCOL_VERSION,
                context: ExecutionContext {
                    purpose: ExecutionPurpose::EngineTurn {},
                    request_id: id,
                    session_version: id,
                    game: CanonicalRecord::chu(chu::GameRecord::from_game(&game).unwrap()),
                    limits: WorkLimits {
                        depth: 1,
                        nodes: 512,
                        time_ms: Some(100),
                    },
                },
            });
            let bytes = client.encode(&request).unwrap();
            assert_eq!(
                serde_json::from_slice::<ExecutionRequest>(&bytes).is_ok(),
                id == 1
            );
            let mut response = Response::new(&bytes, result.clone());
            {
                assert!(
                    client
                        .decode(&serde_json::to_vec(&response).unwrap())
                        .is_err(),
                    "successful request cannot lose its endpoint"
                );
            }
            response.endpoint = Some(Stamp { request_id: id });
            client
                .decode(&serde_json::to_vec(&response).unwrap())
                .unwrap();
            assert!(client.previous.is_some());
        }
    }
}
