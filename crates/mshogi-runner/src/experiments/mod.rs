//! Offline developer experiments over the existing native engines and rules.
mod evidence;
mod native;
mod run;
use crate::{EngineIdentity, RunnerError};
pub use evidence::*;
use mshogi_core::native::RulesProfile;
use mshogi_engine::{ExecutionContext, ExecutionObservation, ExecutionPurpose, WorkLimits};
pub use native::{Root, fresh_root, standard_root};
pub use run::{run, run_with_stop};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, path::Path};

/// Aggregate config/report/data resource bound, independent of product budgets.
pub const MAX_BYTES: usize = 32 * 1024 * 1024;
/// Versioned schedule and evidence family.
pub const SCHEMA: u32 = 1;
/// Deterministic SHA256 counter/rejection scheduling, not an engine search seed.
pub const SCHEDULE: &str = "sha256-counter-rejection-v1";
/// All accepted whole profiles; seven profiles are six games.
pub const PROFILES: [RulesProfile; 7] = [
    RulesProfile::ChessMatchV1,
    RulesProfile::ShogiCsa36V1,
    RulesProfile::XiangqiWxf2018V1,
    RulesProfile::ToriModernV1,
    RulesProfile::ChuJcsa2019V1,
    RulesProfile::TenjikuAdamsModernV1,
    RulesProfile::TenjikuMullerCompletedV1,
];

/// Editable bounded diagnostic template, not a strength target or training plan.
pub fn template(executable: &str) -> Result<Config, RunnerError> {
    let executable = std::fs::canonicalize(executable)
        .map_err(err)?
        .to_str()
        .ok_or_else(|| err("executable path must be UTF8"))?
        .to_owned();
    let players=[("candidate",1,512),("baseline-low",1,128),("baseline-deeper",2,512)].into_iter().map(|(id,depth,nodes)|Player{id:id.into(),executable:executable.clone(),build_provenance:"unverified declaration: replace with actual source/compiler/build receipt; executable SHA is observed automatically".into(),limits:PROFILES.iter().map(|p|ProfileLimits{profile:*p,work:WorkLimits{depth,nodes}}).collect()}).collect();
    let openings = PROFILES
        .iter()
        .map(|p| {
            Ok(Opening {
                id: p.id().into(),
                group: p.id().into(),
                root: standard_root(*p).map_err(err)?,
                split: Split::Test,
                provenance:
                    "native standard root; project-generated diagnostic, no external corpus".into(),
            })
        })
        .collect::<Result<_, RunnerError>>()?;
    let config=Config{schema:SCHEMA,id:"native-diagnostic-template".into(),protocol:"Finite standard-root diagnostic only; unequal-work baseline ablation, not competitive strength. Supply hardware/build/rights/target-distribution protocol before inference.".into(),seed:48,rounds:1,max_steps:2,wall_budget_ms:120_000,watchdog_ms:15_000,cleanup_ms:250,players,openings};
    config.validate()?;
    Ok(config)
}

/// Explicit dataset grouping; all descendants of one opening stay together.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Split {
    /// Training-only lineage.
    Train,
    /// Validation-only lineage.
    Validation,
    /// Held-out test lineage.
    Test,
}
/// Actual supported ordinary-search settings, scoped to a whole rules profile.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileLimits {
    /// Immutable whole profile.
    pub profile: RulesProfile,
    /// Effective values submitted to protocol4.
    pub work: WorkLimits,
}
/// Internal-search participant. Arbitrary model/options/arguments are unsupported.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Player {
    /// Stable comparison label.
    pub id: String,
    /// Exact existing internal-search executable; no shell/PATH search.
    pub executable: String,
    /// Caller-declared executable build provenance, not an authenticated identity.
    pub build_provenance: String,
    /// Per-profile effective search limits.
    pub limits: Vec<ProfileLimits>,
}
/// Complete native starting history and its authored/source provenance.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Opening {
    /// Stable sample/group identity.
    pub id: String,
    /// Authored parent trajectory/corpus grouping, shared by related imported roots.
    pub group: String,
    /// Native history, including a complete untimed Chess match where applicable.
    pub root: Root,
    /// Prevent color-pair/history-prefix split leakage.
    pub split: Split,
    /// Origin, license/rights and selection rationale.
    pub provenance: String,
}
/// Bounded preregistered experiment. Player0 is compared to each baseline.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// Format version.
    pub schema: u32,
    /// Stable experiment identifier.
    pub id: String,
    /// Caller declaration: question, conditions, rights and hardware description.
    pub protocol: String,
    /// Seed affects starting-position order only; native search has no seed input.
    pub seed: u64,
    /// Complete seeded catalog cycles; every opening is used in each cycle.
    pub rounds: u16,
    /// Accepted engine commands per game; exhaustion is unscored, never a draw.
    pub max_steps: u16,
    /// Whole-run soft wall budget, checked before/while native child work.
    pub wall_budget_ms: u64,
    /// Existing BridgeOptions watchdog per submitted request.
    pub watchdog_ms: u64,
    /// Existing bounded bridge cleanup observation interval.
    pub cleanup_ms: u64,
    /// Candidate and one or more actual baselines.
    pub players: Vec<Player>,
    /// Explicit finite distribution: uniform catalog, seeded permutation per cycle.
    pub openings: Vec<Opening>,
}
fn bounded(s: &str) -> bool {
    !s.is_empty() && s.len() <= 2048
}
fn err(s: impl ToString) -> RunnerError {
    RunnerError(s.to_string())
}
/// Lowercase SHA256 of exact UTF8 evidence bytes.
pub fn digest(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}
impl Config {
    /// Strict bounded decoding retains native-record duplicate-field admission.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        if text.len() > 1024 * 1024 {
            return Err(err("experiment config exceeds1MiB"));
        }
        let value: Self = serde_json::from_str(text).map_err(err)?;
        value.validate()?;
        Ok(value)
    }
    /// Validate resource limits and concrete native profiles before launching.
    pub fn validate(&self) -> Result<(), RunnerError> {
        if self.schema != SCHEMA
            || !bounded(&self.id)
            || !bounded(&self.protocol)
            || !(2..=4).contains(&self.players.len())
            || self.openings.is_empty()
            || self.openings.len() > 16
            || !(1..=16).contains(&self.rounds)
            || self.openings.len() * usize::from(self.rounds) * (self.players.len() - 1) * 2 > 128
            || self.max_steps > 512
            || self.openings.len()
                * usize::from(self.rounds)
                * (self.players.len() - 1)
                * 2
                * usize::from(self.max_steps)
                > 4096
            || !(1..=600_000).contains(&self.wall_budget_ms)
            || !(1..=60_000).contains(&self.watchdog_ms)
            || !(1..=2000).contains(&self.cleanup_ms)
        {
            return Err(err("invalid or excessive experiment configuration"));
        }
        let mut ids = BTreeSet::new();
        for p in &self.players {
            if !bounded(&p.id)
                || !bounded(&p.executable)
                || !Path::new(&p.executable).is_absolute()
                || !bounded(&p.build_provenance)
                || !ids.insert(p.id.clone())
            {
                return Err(err(
                    "invalid/duplicate player identity or nonabsolute executable",
                ));
            }
            let mut profiles = BTreeSet::new();
            for limits in &p.limits {
                if !profiles.insert(limits.profile.id()) || !valid_work(limits.profile, limits.work)
                {
                    return Err(err(
                        "duplicate profile or unsupported effective work limits",
                    ));
                }
            }
        }
        let mut ids = BTreeSet::new();
        let mut root_splits = std::collections::BTreeMap::new();
        let mut root_sizes = Vec::new();
        for o in &self.openings {
            if !bounded(&o.id)
                || !bounded(&o.group)
                || !bounded(&o.provenance)
                || !ids.insert(o.id.clone())
            {
                return Err(err("invalid/duplicate opening identity"));
            }
            let state = native::NativeState::from_root(&o.root).map_err(err)?;
            let normalized = serde_json::to_string(&state.root().map_err(err)?).map_err(err)?;
            root_sizes.push(normalized.len() + 8192);
            if let Some(split) = root_splits.insert(digest(&normalized), o.split)
                && split != o.split
            {
                return Err(err("same native root assigned to multiple dataset splits"));
            }
            for group in [format!("declared:{}", o.group), lineage_key(&o.root)?] {
                if let Some(split) = root_splits.insert(group, o.split)
                    && split != o.split
                {
                    return Err(err(
                        "related native prefixes/declared group cross dataset splits",
                    ));
                }
            }
            for p in &self.players {
                p.work(o.root.record.profile())?;
            }
        }
        if self
            .schedule()
            .iter()
            .map(|p| root_sizes[p.opening])
            .sum::<usize>()
            > MAX_BYTES / 2
        {
            return Err(err(
                "final-root schedule alone exceeds evidence budget; use smaller batches",
            ));
        }
        if serde_json::to_vec(self).map_err(err)?.len() > 1024 * 1024 {
            return Err(err("config encoding exceeds1MiB"));
        }
        Ok(())
    }
    /// Deterministic complete color pairs, with each cycle visiting every opening.
    pub fn schedule(&self) -> Vec<Pairing> {
        let mut result = Vec::new();
        for cycle in 0..self.rounds {
            let mut order: Vec<_> = (0..self.openings.len()).collect();
            let mut counter = 0u64;
            for i in (1..order.len()).rev() {
                let upper = (i + 1) as u64;
                let ceiling = u64::MAX - (u64::MAX % upper);
                let n = loop {
                    let hash =
                        Sha256::digest(format!("{SCHEDULE}:{}:{cycle}:{counter}", self.seed));
                    counter += 1;
                    let n = u64::from_le_bytes(hash[..8].try_into().unwrap());
                    if n < ceiling {
                        break n % upper;
                    }
                };
                order.swap(i, n as usize);
            }
            for opening in order {
                for baseline in 1..self.players.len() {
                    let pair = result.len() / 2;
                    for swapped in [false, true] {
                        result.push(Pairing {
                            pair,
                            cycle,
                            opening,
                            baseline,
                            swapped,
                        });
                    }
                }
            }
        }
        result
    }
}
impl Player {
    pub(super) fn work(&self, profile: RulesProfile) -> Result<WorkLimits, RunnerError> {
        self.limits
            .iter()
            .find(|l| l.profile == profile)
            .map(|l| l.work)
            .ok_or_else(|| err("participant has no effective limits for selected profile"))
    }
}
fn valid_work(profile: RulesProfile, work: WorkLimits) -> bool {
    let (depth, nodes) = match profile.game() {
        mshogi_core::native::GameKind::Tori => {
            (mshogi_engine::TORI_MAX_DEPTH, mshogi_engine::TORI_MAX_NODES)
        }
        mshogi_core::native::GameKind::Chu => {
            (mshogi_engine::CHU_MAX_DEPTH, mshogi_engine::CHU_MAX_NODES)
        }
        mshogi_core::native::GameKind::Tenjiku => (
            mshogi_engine::TENJIKU_MAX_DEPTH,
            mshogi_engine::TENJIKU_MAX_NODES,
        ),
        _ => (
            mshogi_engine::MAX_ANALYSIS_DEPTH,
            mshogi_engine::MAX_ANALYSIS_NODES,
        ),
    };
    if work.depth == 0 || work.depth > depth || work.nodes > nodes {
        return false;
    }
    match profile.game() {
        mshogi_core::native::GameKind::Shogi => mshogi_search::shogi::Limits {
            depth: work.depth,
            nodes: work.nodes,
        }
        .valid(),
        mshogi_core::native::GameKind::Xiangqi => mshogi_search::xiangqi::Limits {
            depth: work.depth,
            nodes: work.nodes,
        }
        .valid(),
        _ => true,
    }
}

/// Stable seed/cycle/opening/baseline/color identity, retained in every game.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pairing {
    /// Color pair identity.
    pub pair: usize,
    /// Catalog cycle.
    pub cycle: u16,
    /// Opening index.
    pub opening: usize,
    /// Baseline participant index.
    pub baseline: usize,
    /// Candidate plays second native side.
    pub swapped: bool,
}
/// Exact native observation before the authoritative accepted transition.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Step {
    /// Config participant index.
    pub player: usize,
    /// Actual request/launch/search/receipt duration, not pure search time.
    pub elapsed_ns: u64,
    /// Exact actual submitted request, even if cancelled before a response.
    pub request: ExecutionContext,
    /// Actual protocol4 observation, never fabricated on cancellation.
    #[serde(deserialize_with = "strict_observation")]
    pub observation: Option<ExecutionObservation>,
    /// Host cancellation or native-admission refusal.
    pub failure: Option<String>,
    /// Whether a native command was accepted.
    pub applied: bool,
}
fn strict_observation<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<ExecutionObservation>, D::Error> {
    let raw = Option::<Box<serde_json::value::RawValue>>::deserialize(d)?;
    raw.map(|r| {
        ExecutionObservation::from_json_with_stop(r.get(), &mut || false)
            .map_err(serde::de::Error::custom)
    })
    .transpose()
}

fn admit_bounded(
    state: &mut native::NativeState,
    result: &mshogi_engine::ExecutionResult,
    limits: WorkLimits,
) -> Result<native::Admission, String> {
    admit_with_match_capacity(
        state,
        result,
        limits,
        mshogi_core::native::MAX_RECORD_ENVELOPE_BYTES,
    )
}
fn admit_with_match_capacity(
    state: &mut native::NativeState,
    result: &mshogi_engine::ExecutionResult,
    limits: WorkLimits,
    match_capacity: usize,
) -> Result<native::Admission, String> {
    let before = state.root()?;
    let mut candidate = native::NativeState::from_root(&before)?;
    let decision = candidate.admit(result)?;
    let after = candidate
        .root()
        .map_err(|e| format!("transition evidence: {e}"))?;
    if after
        .chess_match
        .as_ref()
        .is_some_and(|r| r.len() > match_capacity)
    {
        return Err("transition evidence exceeds Chess match record capacity".into());
    }
    let request = mshogi_engine::ExecutionRequest {
        protocol: mshogi_engine::PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: u64::MAX,
            session_version: u64::MAX,
            process_generation: u64::MAX,
            game: after.record,
            limits,
        },
    };
    if !request.context.valid_limits()
        || serde_json::to_vec(&request)
            .map_err(|e| e.to_string())?
            .len()
            > mshogi_engine::MAX_REQUEST_BYTES
    {
        return Err("transition evidence exceeds existing request/history capacity".into());
    }
    *state = candidate;
    Ok(decision)
}
/// One native game and its honest ending, including unscored failures/caps.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Game {
    /// Exact schedule identity.
    pub pairing: Pairing,
    /// All observed work, including an incomplete/failed last turn.
    pub steps: Vec<Step>,
    /// Complete final native game/match history.
    pub final_root: Root,
    /// Native conclusion only; absent for experimental stops.
    pub native_outcome: Option<serde_json::Value>,
    /// Native first-side half-point score, absent unless genuinely concluded/scorable.
    pub score_first_twice: Option<u8>,
    /// Native/experiment stop reason.
    pub stop: String,
    /// Cleanup diagnostics never rewrite a native conclusion.
    pub cleanup: Vec<String>,
}
/// Observed harness build/runtime; participant build provenance remains declared.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Environment {
    /// Build-time harness checkout revision (unavailable outside Git).
    pub harness_revision: String,
    /// Build-time tracked source diff fingerprint; not authentication.
    pub harness_dirty_sha256: String,
    /// Actual harness compiler.
    pub rustc: String,
    /// Cargo build profile.
    pub build_profile: String,
    /// Cargo target.
    pub target: String,
    /// Cargo encoded Rust flags.
    pub rustflags: String,
    /// Observed OS.
    pub os: String,
    /// Observed architecture.
    pub arch: String,
    /// Observed logical parallelism, not proof of exclusive CPU availability.
    pub available_parallelism: usize,
}
impl Environment {
    fn observed() -> Self {
        Self {
            harness_revision: env!("EXPERIMENT_BUILD_REVISION").into(),
            harness_dirty_sha256: env!("EXPERIMENT_BUILD_DIRTY_SHA256").into(),
            rustc: env!("EXPERIMENT_BUILD_RUSTC").into(),
            build_profile: env!("EXPERIMENT_BUILD_PROFILE").into(),
            target: env!("EXPERIMENT_BUILD_TARGET").into(),
            rustflags: env!("EXPERIMENT_BUILD_FLAGS").into(),
            os: std::env::consts::OS.into(),
            arch: std::env::consts::ARCH.into(),
            available_parallelism: std::thread::available_parallelism().map_or(1, usize::from),
        }
    }
}
/// Bound artifact identity, complete games, settings and separately qualified observations.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Report {
    /// Schema version.
    pub schema: u32,
    /// Effective configuration, not just its path.
    pub config: Config,
    /// Integrity fingerprint, not a signature of authenticity.
    pub config_sha256: String,
    /// Seeded schedule implementation.
    pub schedule_algorithm: String,
    /// Automatically observed harness metadata.
    pub environment: Environment,
    /// Actual executable/embedded-model fingerprints.
    pub identities: Vec<EngineIdentity>,
    /// Entire scheduled sample, including unattempted budget stops.
    pub games: Vec<Game>,
}
impl Report {
    /// Bound, strict parsing followed by authoritative consistency replay.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        if text.len() > MAX_BYTES {
            return Err(err("experiment report exceeds32MiB"));
        }
        let report: Self = serde_json::from_str(text).map_err(err)?;
        report.validate()?;
        Ok(report)
    }
    /// Encode after checking native replay and complete evidence bounds.
    pub fn to_json(&self) -> Result<String, RunnerError> {
        self.validate()?;
        let s = serde_json::to_string(self).map_err(err)?;
        if s.len() > MAX_BYTES {
            return Err(err("report encoding exceeds32MiB"));
        }
        Ok(s)
    }
    /// Re-admit recorded observations without rerunning engines or trusting scores.
    pub fn validate(&self) -> Result<(), RunnerError> {
        self.config.validate()?;
        if serde_json::to_vec(self).map_err(err)?.len() > MAX_BYTES {
            return Err(err("typed report exceeds32MiB"));
        }
        if self.schema != SCHEMA
            || self.schedule_algorithm != SCHEDULE
            || self.config_sha256 != digest(&serde_json::to_string(&self.config).map_err(err)?)
            || self.identities.len() != self.config.players.len()
        {
            return Err(err("report/configuration identity mismatch"));
        }
        for (id, p) in self.identities.iter().zip(&self.config.players) {
            if id.executable.path != p.executable
                || id.executable.sha256.is_none()
                || id.executable.error.is_some()
                || id.model.is_some()
                || id.model_identity.as_deref() != Some("executable-embedded")
            {
                return Err(err("effective executable/model declaration mismatch"));
            }
        }
        let schedule = self.config.schedule();
        if self.games.len() != schedule.len() {
            return Err(err("incomplete schedule evidence"));
        }
        let mut last_id = 0;
        for (game, expected) in self.games.iter().zip(schedule) {
            if game.pairing != expected || game.steps.len() > usize::from(self.config.max_steps) {
                return Err(err("pairing/step count mismatch"));
            }
            let mut state =
                native::NativeState::from_root(&self.config.openings[expected.opening].root)
                    .map_err(err)?;
            let mut last_stop = None;
            for step in &game.steps {
                if state.stop_reason().is_some() {
                    return Err(err("observation after native stop"));
                }
                let first = first_side(state.turn());
                let player = if first != expected.swapped {
                    0
                } else {
                    expected.baseline
                };
                let c = &step.request;
                if step.player != player
                    || c.game != state.root().map_err(err)?.record
                    || c.limits != self.config.players[player].work(c.game.profile())?
                    || c.request_id <= last_id
                    || c.session_version != c.request_id
                    || c.process_generation != c.request_id
                    || c.purpose != (ExecutionPurpose::EngineTurn {})
                {
                    return Err(err("stale/wrong participant, history, purpose or limits"));
                }
                last_id = c.request_id;
                let Some(observation) = &step.observation else {
                    let reason = step
                        .failure
                        .as_deref()
                        .ok_or_else(|| err("missing response without recorded interruption"))?;
                    if !["cancelled", "wall_budget"].contains(&reason) || step.applied {
                        return Err(err("invalid interrupted request"));
                    }
                    last_stop = Some(native::Admission::Failed(reason.into()));
                    if game
                        .steps
                        .last()
                        .is_none_or(|s| s.request.request_id != c.request_id)
                    {
                        return Err(err("continued after interruption"));
                    }
                    continue;
                };
                if serde_json::to_vec(observation).map_err(err)?.len()
                    > mshogi_engine::MAX_RESPONSE_BYTES
                {
                    return Err(err("observation exceeds existing response capacity"));
                }
                let refusal = if !observation.matches(c) {
                    Some("observation:context")
                } else if !within_limits(observation) {
                    Some("observation:limits")
                } else {
                    None
                };
                if let Some(reason) = refusal {
                    if step.applied || step.failure.as_deref() != Some(reason) {
                        return Err(err("unrecorded observation refusal"));
                    }
                    last_stop = Some(native::Admission::Failed(reason.into()));
                    if game
                        .steps
                        .last()
                        .is_none_or(|s| s.request.request_id != c.request_id)
                    {
                        return Err(err("continued after refused observation"));
                    }
                    continue;
                }
                let before = state.root().map_err(err)?;
                let decision = match admit_bounded(&mut state, &observation.result, c.limits) {
                    Ok(d) => {
                        if step.failure.is_some() {
                            return Err(err("invented admission failure"));
                        }
                        d
                    }
                    Err(e) => {
                        if step.failure.as_deref() != Some(e.as_str())
                            || state.root().map_err(err)? != before
                        {
                            return Err(err("admission diagnostic/atomicity mismatch"));
                        }
                        native::Admission::Failed(format!("admission:{e}"))
                    }
                };
                if step.applied != matches!(decision, native::Admission::Applied) {
                    return Err(err("native applied marker mismatch"));
                }
                last_stop = Some(decision);
                if !step.applied
                    && game
                        .steps
                        .last()
                        .is_none_or(|last| last.request.request_id != c.request_id)
                {
                    return Err(err("continued after unusable result"));
                }
            }
            if state.root().map_err(err)? != game.final_root
                || state.outcome() != game.native_outcome
                || state.score_first_twice() != game.score_first_twice
            {
                return Err(err("native final history/outcome/score mismatch"));
            }
            let expected_stop = state.stop_reason().or_else(|| last_stop.and_then(stop_for));
            if let Some(stop) = expected_stop {
                if game.stop != stop {
                    return Err(err("native/observed stop mismatch"));
                }
            } else if !["step_limit", "wall_budget", "cancelled", "artifact_limit"]
                .contains(&game.stop.as_str())
                || (game.stop == "step_limit"
                    && game.steps.len() != usize::from(self.config.max_steps))
            {
                return Err(err("unsupported experiment stop"));
            }
        }
        Ok(())
    }
}
fn stop_for(a: native::Admission) -> Option<String> {
    match a {
        native::Admission::Applied => None,
        native::Admission::Finished => Some("native_concluded".into()),
        native::Admission::Incomplete(s) => Some(format!("incomplete:{s}")),
        native::Admission::Failed(s) if s == "cancelled" || s == "wall_budget" => Some(s),
        native::Admission::Failed(s) => Some(format!("failure:{s}")),
    }
}
fn within_limits(o: &ExecutionObservation) -> bool {
    // Structural resource consistency only; never a proof of optimal search.
    let Ok(v) = serde_json::to_value(&o.result) else {
        return false;
    };
    let Some(t) = v.get("result").and_then(|r| r.get("telemetry")) else {
        return true;
    };
    t.get("completed_depth")
        .and_then(|x| x.as_u64())
        .is_none_or(|d| d <= u64::from(o.context.limits.depth))
        && t.get("nodes")
            .and_then(|x| x.as_u64())
            .is_none_or(|n| n <= o.context.limits.nodes)
}
fn first_side(side: mshogi_core::native::NativeSide) -> bool {
    use mshogi_core::{chess, chu, native::NativeSide as N, shogi, tenjiku, tori, xiangqi};
    match side {
        N::Chess(s) => s == chess::Color::White,
        N::Shogi(s) => s == shogi::Side::Sente,
        N::Xiangqi(s) => s == xiangqi::Side::Red,
        N::Tori(s) => s == tori::Side::Sente,
        N::Chu(s) => s == chu::Side::Sente,
        N::Tenjiku(s) => s == tenjiku::Side::Sente,
    }
}

#[cfg(test)]
mod tests;
