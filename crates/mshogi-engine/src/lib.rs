//! Lossless internal engine protocol and bounded off-path process supervision.
//!
//! This is distinct from UCI: claims and incomplete searches are not board moves.
//! The authoritative host must check the entire echoed context immediately before
//! accepting a choice. Cancellation invalidates publication, even after completion.

mod bridge;
mod server;
pub use bridge::{BridgeOptions, EngineBridge, SubmitError};
use mshogi_core::chess::{Action, ChessOutcome, ClaimRequest, Color, DrawClaim, GameRecord};
use mshogi_search::search::{SearchChoice, SearchDecision, SearchResult, SearchStatus};
use serde::{Deserialize, Serialize};
pub use server::serve_once;

/// Current internal protocol version.
pub const PROTOCOL_VERSION: u32 = 1;
/// Maximum encoded request line, excluding its newline.
pub const MAX_REQUEST_BYTES: usize = 1024 * 1024;
/// Maximum encoded response line, excluding its newline.
pub const MAX_RESPONSE_BYTES: usize = 2 * 1024 * 1024;
/// Maximum retained stderr bytes per child.
pub const MAX_STDERR_BYTES: usize = 4096;

/// Explicit deterministic limits; a node limit is mandatory.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkLimits {
    /// Maximum completed depth in plies.
    pub depth: u16,
    /// Maximum attempted node visits, including discarded iterations.
    pub nodes: u64,
}
/// Exact context carried in both directions, including known history and modes.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineRequest {
    /// Protocol version, currently one.
    pub protocol: u32,
    /// Host-issued unique, increasing request identity.
    pub request_id: u64,
    /// Host session revision, independent of ply count.
    pub session_version: u64,
    /// Unique, increasing child process generation.
    pub process_generation: u64,
    /// Canonical full native game context, not just a FEN.
    #[serde(with = "record_json")]
    pub game: GameRecord,
    /// Recorded deterministic work limits.
    pub limits: WorkLimits,
}
/// Serializable native draw claim, including an optional unplayed intended move.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineClaim {
    /// Player making the claim.
    pub actor: Color,
    /// Claimable draw rule.
    pub reason: DrawClaim,
    /// Complete intended action; claiming does not play it.
    pub intended: Option<Action>,
}
impl From<ClaimRequest> for EngineClaim {
    fn from(value: ClaimRequest) -> Self {
        Self {
            actor: value.actor,
            reason: value.reason,
            intended: value.intended,
        }
    }
}
impl From<EngineClaim> for ClaimRequest {
    fn from(value: EngineClaim) -> Self {
        Self {
            actor: value.actor,
            reason: value.reason,
            intended: value.intended,
        }
    }
}
/// One complete continuation choice.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EngineChoice {
    /// Complete legal board action.
    Move {
        /// Native complete action.
        action: Action,
    },
    /// Claim remains separate from moving its intended action.
    Claim {
        /// Exact native claim request.
        claim: EngineClaim,
    },
}
impl From<SearchChoice> for EngineChoice {
    fn from(value: SearchChoice) -> Self {
        match value {
            SearchChoice::Move(action) => Self::Move { action },
            SearchChoice::Claim(claim) => Self::Claim {
                claim: claim.into(),
            },
        }
    }
}
/// Actual native execution stop reason, not a chess outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopReason {
    /// Requested depth completed or the root was concluded.
    Complete,
    /// Node budget exhausted.
    NodeLimit,
    /// External stop observed.
    Stopped,
}
/// Observations from the last fully completed native iteration.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Telemetry {
    /// Root side-to-move perspective, in material points (pawn=100).
    /// Mate scores use the native MATE_SCORE distance convention.
    pub score: Option<i32>,
    /// Last completed depth; zero means no completed iteration.
    pub completed_depth: u16,
    /// All attempted node visits, including discarded iterations.
    pub nodes: u64,
    /// Continuation from the completed iteration, potentially ending in a claim.
    pub principal_variation: Vec<EngineChoice>,
    /// Observed completion/limit/stop reason.
    pub status: StopReason,
}
/// Explicit process, transport, context or native-search failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureKind {
    /// The requested mode is deliberately unsupported by this play boundary.
    UnsupportedMode,
    /// Invalid version, identity, limits or game history.
    InvalidRequest,
    /// Native search failed without publishing a partial score.
    Search,
    /// Child executable could not be launched.
    Start,
    /// A pipe or native transport operation failed.
    Io,
    /// Child exited unsuccessfully.
    UnexpectedExit,
    /// Output was malformed, incomplete JSON, or contained extra data.
    MalformedResponse,
    /// Response exceeded its byte cap.
    OversizedResponse,
    /// Child exited successfully without a response.
    MissingResponse,
    /// Child failed to respond/exit within its watchdog.
    Timeout,
    /// Echoed complete context differed from the submitted request.
    ContextMismatch,
    /// Child could not be observed reaped within the cleanup watchdog.
    Cleanup,
}
/// No score or chess conclusion is attached to a failure.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineFailure {
    /// Machine-readable failure category.
    pub kind: FailureKind,
    /// Bounded diagnostic message.
    pub message: String,
}
/// Lossless native decision or explicit lack of a usable decision.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EngineResult {
    /// Apply this complete board action through the authoritative host.
    Move {
        /// Native action.
        action: Action,
        /// Real search observations.
        telemetry: Telemetry,
    },
    /// Apply this claim through the authoritative host, not its intended move.
    Claim {
        /// Native claim.
        claim: EngineClaim,
        /// Real search observations.
        telemetry: Telemetry,
    },
    /// Rules-owned root was already concluded.
    Finished {
        /// Established rules outcome.
        outcome: ChessOutcome,
        /// Real search observations.
        telemetry: Telemetry,
    },
    /// No full iteration completed; no fallback action or score is invented.
    Incomplete {
        /// Actual incomplete work observations.
        telemetry: Telemetry,
    },
    /// Process/search failure, never a chess win or draw.
    Failure {
        /// Categorized diagnostic.
        failure: EngineFailure,
    },
}
impl EngineResult {
    pub(crate) fn failure(kind: FailureKind, message: impl ToString) -> Self {
        let mut message = message.to_string();
        if message.len() > MAX_STDERR_BYTES {
            let mut end = MAX_STDERR_BYTES;
            while !message.is_char_boundary(end) {
                end -= 1;
            }
            message.truncate(end);
        }
        Self::Failure {
            failure: EngineFailure { kind, message },
        }
    }
    pub(crate) fn searched(result: SearchResult) -> Self {
        let telemetry = Telemetry {
            score: result.score,
            completed_depth: result.completed_depth,
            nodes: result.nodes,
            principal_variation: result
                .principal_variation
                .into_iter()
                .map(Into::into)
                .collect(),
            status: match result.status {
                SearchStatus::Complete => StopReason::Complete,
                SearchStatus::NodeLimit => StopReason::NodeLimit,
                SearchStatus::Stopped => StopReason::Stopped,
            },
        };
        match result.decision {
            Some(SearchDecision::Choose(SearchChoice::Move(action))) => {
                Self::Move { action, telemetry }
            }
            Some(SearchDecision::Choose(SearchChoice::Claim(claim))) => Self::Claim {
                claim: claim.into(),
                telemetry,
            },
            Some(SearchDecision::Finished(outcome)) => Self::Finished { outcome, telemetry },
            None => Self::Incomplete { telemetry },
        }
    }
}
/// One final result, permanently tied to its full submitted context.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineResponse {
    /// Exact submitted context echoed without downgrade.
    pub context: EngineRequest,
    /// Distinct native result.
    pub result: EngineResult,
}
mod record_json {
    use super::*;
    pub fn serialize<S: serde::Serializer>(
        record: &GameRecord,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let raw = serde_json::value::RawValue::from_string(record.to_json())
            .map_err(serde::ser::Error::custom)?;
        raw.serialize(serializer)
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<GameRecord, D::Error> {
        // Preserve duplicate fields for the strict native parser to reject.
        let raw = Box::<serde_json::value::RawValue>::deserialize(deserializer)?;
        GameRecord::from_json(raw.get()).map_err(serde::de::Error::custom)
    }
}
