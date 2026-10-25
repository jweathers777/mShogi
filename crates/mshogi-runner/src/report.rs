//! Typed experiment observations; chess results are recomputed by core replay.
use crate::{ExperimentConfig, RunnerError};
use mshogi_core::chess::{ChessMatch, MatchConclusion, MatchRecord, MatchRequest};
use serde::{Deserialize, Serialize};

/// SHA-256 identity of a filesystem artifact, or an explicit acquisition error.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FileIdentity {
    /// Configured filesystem path.
    pub path: String,
    /// Canonical path when available.
    pub canonical_path: Option<String>,
    /// File length observed before hashing.
    pub bytes: Option<u64>,
    /// Lowercase SHA-256 when the file could be read completely.
    pub sha256: Option<String>,
    /// Failure to establish identity; never silently represented as a hash.
    pub error: Option<String>,
}
/// Executable and optional external/embedded model identity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineIdentity {
    /// Executable content identity.
    pub executable: FileIdentity,
    /// Optional external model content identity.
    pub model: Option<FileIdentity>,
    /// User-supplied embedded/model/evaluator label.
    pub model_identity: Option<String>,
}
/// Stable unscored experiment failure category.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureKind {
    /// Process could not be started.
    ProcessStartFailure,
    /// Nonzero exit, signal, or an idle engine exiting unexpectedly.
    UnexpectedExit,
    /// Bestmove syntax or rules-owned legality rejected the action.
    InvalidMove,
    /// Required reply was absent at clean EOF/closed output.
    MissingResponse,
    /// Live process missed the configured phase watchdog.
    ResponseTimeout,
    /// Malformed/oversized output or command/I/O resource exhaustion.
    InvalidResponse,
}
/// Portable process exit details, including Unix signal when applicable.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProcessExit {
    /// Native exit code when supplied by the platform.
    pub code: Option<i32>,
    /// Terminating signal on Unix, otherwise None.
    pub signal: Option<i32>,
    /// Whether the native status was successful.
    pub success: bool,
    /// Platform-provided descriptive status.
    pub description: String,
}
/// Failure evidence, kept distinct from chess outcomes and clock expiry.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunnerFailure {
    /// Stable category.
    pub kind: FailureKind,
    /// Configuration engine index (0 or 1).
    pub engine: usize,
    /// Protocol phase, such as handshake/readiness/bestmove/write.
    pub phase: String,
    /// Monotonic controller observation time since this game's origin.
    pub at_ns: u64,
    /// Configured watchdog when relevant.
    pub watchdog_ms: Option<u64>,
    /// Established watchdog deadline, distinct from later observation time.
    pub deadline_ns: Option<u64>,
    /// Bounded human evidence; never interpreted as game rules.
    pub detail: String,
    /// Observed process status, if available.
    pub exit: Option<ProcessExit>,
    /// Last complete bounded stdout observation.
    pub last_output: Option<String>,
}
/// Direction of a recorded UCI transport observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransportDirection {
    /// Attempted complete command, with write completion recorded separately.
    Command,
    /// Complete stdout line observed by the controller.
    Response,
}
/// One retained transport observation. Sequence covers dropped entries too.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TranscriptEntry {
    /// One-based total controller observation sequence.
    pub sequence: u64,
    /// Configured engine index.
    pub engine: usize,
    /// Direction.
    pub direction: TransportDirection,
    /// Controller observation/dispatch timestamp.
    pub at_ns: u64,
    /// Match event version at this observation.
    pub game_version: u64,
    /// Protocol phase.
    pub phase: String,
    /// Bounded escaped text, preserving arbitrary stdout bytes.
    pub text: String,
    /// Whether full text exceeded the remaining transcript budget.
    pub truncated: bool,
    /// For commands, whether write_all and flush completed.
    pub write_completed: Option<bool>,
}
/// Retained protocol observations with explicit truncation accounting.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Transcript {
    /// Retained ordered entries.
    pub entries: Vec<TranscriptEntry>,
    /// Total attempted entry count, including entries discarded after cap.
    pub observed_entries: u64,
    /// Bytes not retained due to the cap.
    pub dropped_bytes: u64,
    /// Whether any event/text was discarded.
    pub truncated: bool,
}
impl Transcript {
    pub(crate) fn new() -> Self {
        Self {
            entries: Vec::new(),
            observed_entries: 0,
            dropped_bytes: 0,
            truncated: false,
        }
    }
}
/// Bounded stderr evidence, separate from protocol stdout.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Diagnostic {
    /// Escaped retained bytes.
    pub text: String,
    /// Total bytes read (saturates only at the observation counter limit).
    pub observed_bytes: u64,
    /// Whether retained bytes omit later output.
    pub truncated: bool,
}
/// Child identity/diagnostics and bounded cleanup outcome for one game.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineObservation {
    /// Configuration engine index.
    pub engine: usize,
    /// Native process ID, not a reproducibility identity.
    pub process_id: u32,
    /// Advertised UCI id lines.
    pub uci_identity: Vec<String>,
    /// Identity lines exceeded the independent metadata count/byte cap.
    pub identity_truncated: bool,
    /// Bounded stderr capture.
    pub stderr: Diagnostic,
    /// Final observed status if reaped.
    pub exit: Option<ProcessExit>,
    /// Cleanup problems; these never replace a chess conclusion.
    pub cleanup_failures: Vec<String>,
}
/// Why the experiment stopped requesting moves.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GameStop {
    /// A rules-owned conclusion exists, possibly explicitly unscored uncertainty.
    ChessConclusion,
    /// Protocol/process failure, with no invented chess outcome.
    Failure,
    /// Configured maximum plies reached while the game remains ongoing.
    PlyLimit,
}
/// One game's complete reusable evidence.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameReport {
    /// Index in configured openings.
    pub opening_index: usize,
    /// Stable opening label.
    pub opening_id: String,
    /// First game is zero; reversed pair is one.
    pub pairing: u8,
    /// Configuration engine indices, White then Black.
    pub engines_by_color: [usize; 2],
    /// Typed experiment termination category.
    pub stop: GameStop,
    /// Core conclusion, never fabricated from process failure.
    pub conclusion: Option<MatchConclusion>,
    /// Primary unscored experiment failure when no earlier chess conclusion exists.
    pub failure: Option<RunnerFailure>,
    /// Later failures are diagnostic and cannot replace a prior result.
    pub diagnostic_failures: Vec<RunnerFailure>,
    /// Strict core match record, independently replayed from observations.
    pub match_record_json: String,
    /// Bounded ordered protocol transcript.
    pub transcript: Transcript,
    /// Each successfully spawned engine's observation/cleanup.
    pub engines: Vec<EngineObservation>,
}
/// Counts per opening; no rating/strength inference is made.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpeningSummary {
    /// Configured opening label.
    pub opening_id: String,
    /// Games recorded.
    pub games: u64,
    /// Scored wins for each configured engine.
    pub wins: [u64; 2],
    /// Exact chess draws.
    pub draws: u64,
    /// Explicit unresolved core terminations, failures and ply caps.
    pub unscored: u64,
    /// Primary experiment failures.
    pub failures: u64,
}
/// Versioned experiment report. Replay never invokes an engine or measures time.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExperimentReport {
    /// Encoding version (1).
    pub schema_version: u32,
    /// Effective validated configuration, including defaulted limits.
    pub config: ExperimentConfig,
    /// SHA-256 of canonical effective configuration, detecting accidental drift.
    /// This checksum is consistency evidence, not record authentication.
    pub config_sha256: String,
    /// Executable/model fingerprints from the actual configured paths.
    pub engine_identities: [EngineIdentity; 2],
    /// Runner package version.
    pub runner_version: String,
    /// Native OS name; this is evidence for this platform only.
    pub os: String,
    /// Native architecture.
    pub arch: String,
    /// Conditional eligibility; never a proof of an engine's deterministic behavior.
    pub deterministic_rerun_eligible: bool,
    /// Per-game records in opening/pair order.
    pub games: Vec<GameReport>,
    /// Recomputed aggregate counts.
    pub summaries: Vec<OpeningSummary>,
}
pub(crate) fn summaries(config: &ExperimentConfig, games: &[GameReport]) -> Vec<OpeningSummary> {
    config
        .openings
        .iter()
        .enumerate()
        .map(|(index, o)| {
            let mut s = OpeningSummary {
                opening_id: o.id.clone(),
                games: 0,
                wins: [0, 0],
                draws: 0,
                unscored: 0,
                failures: 0,
            };
            for g in games.iter().filter(|g| g.opening_index == index) {
                s.games += 1;
                if g.failure.is_some() {
                    s.failures += 1;
                }
                match g.conclusion.and_then(MatchConclusion::white_score_twice) {
                    Some(2) => s.wins[g.engines_by_color[0]] += 1,
                    Some(0) => s.wins[g.engines_by_color[1]] += 1,
                    Some(1) => s.draws += 1,
                    _ => s.unscored += 1,
                }
            }
            s
        })
        .collect()
}
impl ExperimentReport {
    /// Canonical compact JSON, not an engine rerun.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("finite report data")
    }
    /// Decode a bounded strict report; call replay to validate derived evidence.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        if text.len() > 134_217_728 {
            return Err(RunnerError("report exceeds 128 MiB".into()));
        }
        serde_json::from_str(text).map_err(|e| RunnerError(format!("report JSON: {e}")))
    }
    /// Rebuild all core match events and check configurations/results/aggregates.
    /// Process observations remain historical evidence, not re-executed claims.
    pub fn replay(&self) -> Result<Vec<ChessMatch>, RunnerError> {
        self.config.validate()?;
        if self.schema_version != 1 {
            return Err(RunnerError("unsupported report schema".into()));
        }
        if self.config_sha256 != crate::run::config_digest(&self.config) {
            return Err(RunnerError(
                "effective configuration checksum mismatch".into(),
            ));
        }
        for (config, identity) in self.config.engines.iter().zip(&self.engine_identities) {
            if config.path != identity.executable.path
                || config.model_identity != identity.model_identity
                || config.model_path.as_ref() != identity.model.as_ref().map(|m| &m.path)
            {
                return Err(RunnerError(
                    "configured executable/model identity mismatch".into(),
                ));
            }
        }
        let per = if self.config.paired { 2 } else { 1 };
        if self.games.len() != self.config.openings.len() * per {
            return Err(RunnerError("incomplete game report set".into()));
        }
        let mut matches = Vec::new();
        for (i, g) in self.games.iter().enumerate() {
            let index = i / per;
            let pairing = (i % per) as u8;
            let colors = if pairing == 0 { [0, 1] } else { [1, 0] };
            if g.opening_index != index
                || g.opening_id != self.config.openings[index].id
                || g.pairing != pairing
                || g.engines_by_color != colors
            {
                return Err(RunnerError("opening/color assignment mismatch".into()));
            }
            let record = MatchRecord::from_json(&g.match_record_json)
                .map_err(|e| RunnerError(e.to_string()))?;
            let m = record.replay().map_err(|e| RunnerError(e.to_string()))?;
            if m.conclusion() != g.conclusion
                || (g.stop == GameStop::ChessConclusion) != g.conclusion.is_some()
                || (g.stop == GameStop::Failure) != g.failure.is_some()
            {
                return Err(RunnerError("game termination mismatch".into()));
            }
            // Reconstruct the declared opening as the expected initial match.
            let initial = ChessMatch::new(
                self.config.opening_game(&self.config.openings[index])?,
                self.config.time_control,
            )
            .map_err(|e| RunnerError(e.to_string()))?;
            let raw: serde_json::Value = serde_json::from_str(&g.match_record_json)
                .map_err(|e| RunnerError(e.to_string()))?;
            let expected: serde_json::Value =
                serde_json::from_str(&initial.record().to_json()).expect("generated record");
            for field in [
                "schema_version",
                "rules_profile",
                "initial_record_json",
                "time_control",
            ] {
                if raw[field] != expected[field] {
                    return Err(RunnerError(format!("match initial {field} mismatch")));
                }
            }
            // Replay each event once more to retain the exact context of each
            // transport version. This binds retained generated commands to the
            // declared work/options/opening/clock configuration without an engine.
            let mut context_game = initial;
            let mut contexts = vec![(
                crate::run::position_command(context_game.game().history()),
                context_game.remaining_ns(),
                context_game.active(),
                0u64,
            )];
            for event in raw["events"]
                .as_array()
                .ok_or_else(|| RunnerError("match events missing".into()))?
            {
                let request: MatchRequest = serde_json::from_value(event["request"].clone())
                    .map_err(|e| RunnerError(e.to_string()))?;
                let at = request.at_ns;
                context_game.submit(request);
                contexts.push((
                    crate::run::position_command(context_game.game().history()),
                    context_game.remaining_ns(),
                    context_game.active(),
                    at,
                ));
            }
            for entry in &g.transcript.entries {
                if entry.engine > 1 {
                    return Err(RunnerError("transport engine index invalid".into()));
                }
                let context =
                    contexts
                        .get(usize::try_from(entry.game_version).map_err(|_| {
                            RunnerError("transport version exceeds platform".into())
                        })?)
                        .ok_or_else(|| RunnerError("transport version exceeds match".into()))?;
                if entry.direction == TransportDirection::Command {
                    if entry.write_completed.is_none() {
                        return Err(RunnerError("command completion evidence missing".into()));
                    }
                    let valid = match entry.phase.as_str() {
                        "handshake" => entry.text == "uci",
                        "new_game" => entry.text == "ucinewgame",
                        "readiness" => entry.text == "isready",
                        "position" => entry.text == context.0,
                        "go" => {
                            entry.text
                                == crate::run::go_command(
                                    &self.config,
                                    context.1,
                                    context.2,
                                    context.3,
                                    entry.at_ns,
                                )
                        }
                        "cleanup" => matches!(entry.text.as_str(), "stop" | "quit"),
                        "options" => {
                            self.config.engines[entry.engine].options.iter().any(|o| {
                                entry.text == format!("setoption name {} value {}", o.name, o.value)
                            }) || (g
                                .engines
                                .iter()
                                .find(|e| e.engine == entry.engine)
                                .is_some_and(|e| {
                                    e.uci_identity
                                        .iter()
                                        .any(|id| id.starts_with("id name mShogi "))
                                })
                                && entry.text == "setoption name ClaimPolicy value auto")
                        }
                        _ => false,
                    };
                    if !valid {
                        return Err(RunnerError(format!(
                            "generated {} command inconsistent with configuration/context",
                            entry.phase
                        )));
                    }
                } else if entry.write_completed.is_some() {
                    return Err(RunnerError(
                        "response cannot claim stdin write completion".into(),
                    ));
                }
            }
            if g.stop == GameStop::PlyLimit
                && m.game().history().ply_count() != self.config.max_plies as usize
            {
                return Err(RunnerError("ply-limit evidence mismatch".into()));
            }
            if g.failure.as_ref().is_some_and(|f| f.engine > 1)
                || g.diagnostic_failures.iter().any(|f| f.engine > 1)
            {
                return Err(RunnerError("failure engine index invalid".into()));
            }
            if g.transcript
                .entries
                .windows(2)
                .any(|w| w[0].sequence >= w[1].sequence || w[0].at_ns > w[1].at_ns)
                || g.transcript.entries.iter().any(|e| {
                    e.engine > 1 || e.sequence == 0 || e.sequence > g.transcript.observed_entries
                })
            {
                return Err(RunnerError("transcript ordering invalid".into()));
            }
            let retained_cost: usize = g
                .transcript
                .entries
                .iter()
                .map(|e| serde_json::to_vec(e).expect("finite event").len() + 1)
                .sum();
            if retained_cost > self.config.io.transcript_bytes
                || (!g.transcript.truncated
                    && (g.transcript.observed_entries != g.transcript.entries.len() as u64
                        || g.transcript.dropped_bytes != 0))
            {
                return Err(RunnerError(
                    "transcript bound/truncation evidence mismatch".into(),
                ));
            }
            let mut seen_engines = std::collections::HashSet::new();
            for engine in &g.engines {
                if engine.engine > 1
                    || !seen_engines.insert(engine.engine)
                    || engine.uci_identity.len() > 32
                    || engine.uci_identity.iter().map(String::len).sum::<usize>() > 4096
                    || engine.stderr.text.len() > self.config.io.stderr_bytes * 4
                {
                    return Err(RunnerError(
                        "engine metadata/diagnostic bounds invalid".into(),
                    ));
                }
            }
            matches.push(m);
        }
        if self.summaries != summaries(&self.config, &self.games) {
            return Err(RunnerError("opening aggregate mismatch".into()));
        }
        let expected = self.config.rerun_determinism_eligible()
            && self.engine_identities.iter().all(|e| {
                e.executable.sha256.is_some() && e.model.as_ref().is_none_or(|m| m.sha256.is_some())
            });
        if self.deterministic_rerun_eligible != expected {
            return Err(RunnerError("rerun eligibility mismatch".into()));
        }
        Ok(matches)
    }
}
