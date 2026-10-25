//! Strict, bounded experiment inputs. Effective defaults are saved in reports.
use crate::RunnerError;
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy, TimeControl};
use serde::{Deserialize, Serialize};

/// One explicit UCI option, applied after handshake and before readiness.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UciOption {
    /// Exact advertised option name.
    pub name: String,
    /// Value sent without shell interpretation.
    pub value: String,
}
/// An executable and its declared deterministic settings/model identity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EngineConfig {
    /// Filesystem executable path; no shell or PATH lookup is used.
    pub path: String,
    /// Individual native process arguments.
    #[serde(default)]
    pub args: Vec<String>,
    /// Explicit UCI options, retained in their supplied order.
    #[serde(default)]
    pub options: Vec<UciOption>,
    /// Optional model file, independently SHA-256 hashed.
    #[serde(default)]
    pub model_path: Option<String>,
    /// Label for embedded/model/evaluator identity beyond the executable hash.
    #[serde(default)]
    pub model_identity: Option<String>,
    /// User declaration; this is not inferred from an engine's name.
    #[serde(default)]
    pub deterministic: bool,
}
/// Initial state and complete prefix actions for a paired opening.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpeningConfig {
    /// Stable label used in per-opening reports.
    pub id: String,
    /// None means known orthodox start; Some always preserves imported origin.
    #[serde(default)]
    pub fen: Option<String>,
    /// Legal complete actions before the experiment's first engine request.
    #[serde(default)]
    pub moves: Vec<String>,
}
/// Engine work/time request. Timed work never promises identical reruns.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkLimit {
    /// UCI maximum depth, if selected.
    pub depth: Option<u16>,
    /// UCI node budget, if selected.
    pub nodes: Option<u64>,
    /// Per-move wall-clock budget, in UCI milliseconds.
    pub movetime_ms: Option<u64>,
}
/// Independent protocol/cleanup watchdogs, in milliseconds.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Watchdogs {
    /// Await uciok.
    pub handshake_ms: u64,
    /// Await readyok.
    pub ready_ms: u64,
    /// Await bestmove, independent of any chess clock.
    pub response_ms: u64,
    /// Complete one bounded stdin write.
    pub write_ms: u64,
    /// Total graceful-quit/kill/reap/pipe cleanup budget per child.
    pub cleanup_ms: u64,
}
impl Default for Watchdogs {
    fn default() -> Self {
        Self {
            handshake_ms: 5000,
            ready_ms: 5000,
            response_ms: 10000,
            write_ms: 2000,
            cleanup_ms: 1000,
        }
    }
}
/// In-memory and transport limits. Exhaustion is an experiment failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IoLimits {
    /// Maximum bytes in one stdout line, before CR/LF.
    pub line_bytes: usize,
    /// Bounded pending stdout events per child.
    pub queued_lines: usize,
    /// Maximum retained protocol transcript bytes per game.
    pub transcript_bytes: usize,
    /// Maximum retained stderr bytes per child, with truncation evidence.
    pub stderr_bytes: usize,
    /// Maximum command bytes, including LF.
    pub command_bytes: usize,
}
impl Default for IoLimits {
    fn default() -> Self {
        Self {
            line_bytes: 4096,
            queued_lines: 32,
            transcript_bytes: 262144,
            stderr_bytes: 16384,
            command_bytes: 4096,
        }
    }
}
fn default_plies() -> u32 {
    512
}
/// Versioned complete experiment input. The runner uses the M2 software profile.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExperimentConfig {
    /// Supported encoding version (1).
    pub schema_version: u32,
    /// Human experiment label.
    pub id: String,
    /// Two engines; identical configurations perform self-play.
    pub engines: [EngineConfig; 2],
    /// At most 32 independently labelled initial positions/prefixes.
    pub openings: Vec<OpeningConfig>,
    /// Play a second game per opening with the engines' colors reversed.
    pub paired: bool,
    /// Controller-authoritative optional claim handling.
    pub claim_policy: ClaimPolicy,
    /// Untimed or injected monotonic Fischer clocks.
    pub time_control: TimeControl,
    /// Limits sent to each engine on each turn.
    pub work: WorkLimit,
    /// Total action-ply cap including the opening prefix, at most 512.
    #[serde(default = "default_plies")]
    pub max_plies: u32,
    /// Protocol watchdogs, separate from chess clocks.
    #[serde(default)]
    pub watchdogs: Watchdogs,
    /// Bounded process streams/transcript memory.
    #[serde(default)]
    pub io: IoLimits,
}
fn text(s: &str, cap: usize) -> bool {
    !s.is_empty() && s.len() <= cap && !s.chars().any(char::is_control)
}
fn uci_text(s: &str, cap: usize) -> bool {
    s.len() <= cap && s.is_ascii() && !s.bytes().any(|b| b.is_ascii_control())
}
impl ExperimentConfig {
    /// Strict decoding with one-megabyte input bound and semantic validation.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        if text.len() > 1_048_576 {
            return Err(RunnerError("configuration exceeds one MiB".into()));
        }
        let c: Self = serde_json::from_str(text)
            .map_err(|e| RunnerError(format!("configuration JSON: {e}")))?;
        c.validate()?;
        Ok(c)
    }
    /// Validate public typed construction as well as JSON inputs.
    pub fn validate(&self) -> Result<(), RunnerError> {
        let err = |s: &str| Err(RunnerError(s.into()));
        if self.schema_version != 1 {
            return err("unsupported experiment schema");
        }
        if self.claim_policy != ClaimPolicy::AutoClaim {
            return err(
                "runner v1 requires explicit recorded auto_claim policy; UCI has no optional-claim channel",
            );
        }
        if !text(&self.id, 128) || self.openings.is_empty() || self.openings.len() > 32 {
            return err("invalid label/opening count (1..=32)");
        }
        if self.max_plies == 0 || self.max_plies > 512 {
            return err("max_plies must be 1..=512");
        }
        for engine in &self.engines {
            if !text(&engine.path, 4096)
                || engine.args.len() > 64
                || engine.options.len() > 64
                || engine
                    .args
                    .iter()
                    .any(|s| s.len() > 4096 || s.contains('\0'))
                || engine.model_path.as_ref().is_some_and(|s| !text(s, 4096))
                || engine
                    .model_identity
                    .as_ref()
                    .is_some_and(|s| !text(s, 256))
            {
                return err("invalid engine configuration size/path");
            }
            let mut names = std::collections::HashSet::new();
            for o in &engine.options {
                if o.name.is_empty()
                    || !uci_text(&o.name, 128)
                    || !uci_text(&o.value, 2048)
                    || !names.insert(o.name.to_ascii_lowercase())
                {
                    return err("invalid or duplicate UCI option");
                }
            }
        }
        let w = self.watchdogs;
        if [
            w.handshake_ms,
            w.ready_ms,
            w.response_ms,
            w.write_ms,
            w.cleanup_ms,
        ]
        .iter()
        .any(|&n| n == 0 || n > 3_600_000)
        {
            return err("watchdogs must be 1..=3600000 milliseconds");
        }
        let io = self.io;
        if io.line_bytes == 0
            || io.line_bytes > 65536
            || io.queued_lines == 0
            || io.queued_lines > 256
            || io.transcript_bytes > 4_194_304
            || io.stderr_bytes > 1_048_576
            || io.command_bytes < 16
            || io.command_bytes > 4096
        {
            return err("invalid I/O bounds");
        }
        // Conservative JSON envelope bound, including escaped diagnostics,
        // bounded last-line failures, identities and per-game metadata. Protocol
        // entry accounting itself uses serialized JSON size, not payload only.
        let games = self.openings.len() * if self.paired { 2 } else { 1 };
        let per_game = io.transcript_bytes + io.stderr_bytes * 16 + io.line_bytes * 32 + 65536;
        if games * per_game > 120 * 1024 * 1024 {
            return err("aggregate report bound exceeds 120 MiB; reduce games or I/O caps");
        }
        if self.work.depth.is_some_and(|n| n == 0 || n > 128) {
            return err("depth must be 1..=128");
        }
        if self.work.movetime_ms.is_some_and(|n| n > 3_600_000) {
            return err("movetime exceeds one hour");
        }
        if self.time_control == TimeControl::Untimed
            && self.work.depth.is_none()
            && self.work.nodes.is_none()
            && self.work.movetime_ms.is_none()
        {
            return err("untimed experiments require a finite work/movetime limit");
        }
        if let TimeControl::Increment { initial_ns, .. } = self.time_control
            && initial_ns.contains(&0)
        {
            return err("initial clock balances must be positive");
        }
        let mut ids = std::collections::HashSet::new();
        for opening in &self.openings {
            if !text(&opening.id, 128)
                || !ids.insert(&opening.id)
                || opening.moves.len() >= self.max_plies as usize
                || opening.fen.as_ref().is_some_and(|s| !uci_text(s, 256))
            {
                return err("invalid/duplicate opening or prefix too long");
            }
            self.opening_game(opening)?;
        }
        Ok(())
    }
    /// The exact validated live opening, including origin and prefix history.
    pub fn opening_game(&self, opening: &OpeningConfig) -> Result<AdjudicatedGame, RunnerError> {
        let mut g = match &opening.fen {
            Some(f) => AdjudicatedGame::from_fen(f, self.claim_policy)
                .map_err(|e| RunnerError(format!("opening {}: {e}", opening.id)))?,
            None => AdjudicatedGame::new(self.claim_policy),
        };
        for (i, s) in opening.moves.iter().enumerate() {
            let a = s.parse().map_err(|e| {
                RunnerError(format!("opening {} action {}: {e}", opening.id, i + 1))
            })?;
            g.apply(a).map_err(|e| {
                RunnerError(format!("opening {} action {}: {e}", opening.id, i + 1))
            })?;
        }
        Ok(g)
    }
    /// Eligibility only: engines must additionally honor their declarations.
    pub fn rerun_determinism_eligible(&self) -> bool {
        self.time_control == TimeControl::Untimed
            && self.work.movetime_ms.is_none()
            && (self.work.depth.is_some() || self.work.nodes.is_some())
            && self.engines.iter().all(|e| e.deterministic)
    }
}
