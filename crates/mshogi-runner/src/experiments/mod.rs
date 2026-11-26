//! Offline developer experiments over the existing native engines and rules.
mod evidence;
mod native;
mod run;
use crate::{EngineIdentity, RunnerError};
pub use evidence::*;
use mshogi_core::native::RulesProfile;
use mshogi_engine::{
    EngineGame, ExecutionContext, ExecutionObservation, ExecutionPurpose, WorkLimits,
};
pub use native::{Root, fresh_root, standard_root};
pub use run::{run, run_with_stop};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, path::Path};

/// Aggregate config/report/data bound for one experiment batch.
// Encoded steps omit the native record that native replay restores (MSH-204),
// so a report grows linearly with game length; the runner reserves 8 MiB of
// this budget per transition and retains all schedule caps.
pub const MAX_BYTES: usize = 128 * 1024 * 1024;
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
    let players=[("candidate",1,512),("baseline-low",1,128),("baseline-deeper",2,512)].into_iter().map(|(id,depth,nodes)|Player{id:id.into(),executable:executable.clone(),build_provenance:"unverified declaration: replace with actual source/compiler/build receipt; executable SHA is observed automatically".into(),limits:PROFILES.iter().map(|p|ProfileLimits{profile:*p,work:WorkLimits{ time_ms: None,depth,nodes}}).collect(),clock:None}).collect();
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
    let config=Config{schema:SCHEMA,id:"native-diagnostic-template".into(),protocol:"Finite standard-root diagnostic only; unequal-work baseline ablation, not competitive strength. Supply hardware/build/rights/target-distribution protocol before inference.".into(),seed:48,rounds:1,max_steps:2,wall_budget_ms:120_000,watchdog_ms:15_000,cleanup_ms:250,clock:None,players,openings};
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
    /// Effective values submitted to the internal protocol.
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
    /// This player's own clock, for time odds (MSH-204). Requires the shared
    /// configuration clock; absent means the shared clock.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock: Option<FischerClock>,
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
/// Bounded preregistered experiment. Player 0 is compared to each baseline.
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
    /// Optional wall-clock control; absent keeps deterministic fixed-work mode.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock: Option<FischerClock>,
    /// Candidate and one or more actual baselines.
    pub players: Vec<Player>,
    /// Explicit finite distribution: uniform catalog, seeded permutation per cycle.
    pub openings: Vec<Opening>,
}
pub use crate::FischerClock;
/// Replayable clock receipt, charged for transport and process overhead too.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClockReceipt {
    /// Time before the submitted request.
    pub before_ms: u64,
    /// Ceiling of actual request elapsed time.
    pub charged_ms: u64,
    /// Time after charge and any accepted-command increment.
    pub after_ms: u64,
}
/// The normal share of the remaining clock: the soft budget of the shared
/// allocation, at most 60 seconds.
fn clock_share_ms(clock: FischerClock, remaining: u64) -> u64 {
    mshogi_search::alphabeta::timing::TimeBudget::from_clock_ms(remaining, clock.increment_ms, 30)
        .soft
        .as_millis()
        .min(60_000) as u64
}
/// The engine treats its share, less a transport reserve, as a fixed move time.
fn clock_work(
    mut work: WorkLimits,
    clock: Option<FischerClock>,
    remaining: u64,
    first: bool,
) -> WorkLimits {
    if let Some(clock) = clock {
        let allotted = clock_share_ms(clock, remaining);
        let reserve = (if first { 300 } else { 50 }).min(allotted / 2);
        work.time_ms = Some(allotted.saturating_sub(reserve));
    }
    work
}
fn clock_receipt(
    clock: FischerClock,
    before_ms: u64,
    elapsed_ns: u64,
    applied: bool,
) -> ClockReceipt {
    let charged_ms = elapsed_ns.div_ceil(1_000_000);
    ClockReceipt {
        before_ms,
        charged_ms,
        after_ms: crate::common::balance_after(
            before_ms,
            charged_ms,
            if applied { clock.increment_ms } else { 0 },
        ),
    }
}
fn bounded(s: &str) -> bool {
    !s.is_empty() && s.len() <= 2048
}
fn err(s: impl ToString) -> RunnerError {
    RunnerError(s.to_string())
}
/// Lowercase SHA256 of exact UTF8 evidence bytes.
pub fn digest(text: &str) -> String {
    crate::common::sha256(text)
}
impl Config {
    /// Strict bounded decoding retains native-record duplicate-field admission.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        if text.len() > 1024 * 1024 {
            return Err(err("experiment config exceeds 1MiB"));
        }
        let value: Self = serde_json::from_str(text).map_err(err)?;
        value.validate()?;
        Ok(value)
    }
    /// The clock of one participant: its own clock (time odds) or the shared one.
    pub fn player_clock(&self, player: usize) -> Option<FischerClock> {
        self.clock
            .map(|shared| self.players[player].clock.unwrap_or(shared))
    }
    /// Validate resource limits and concrete native profiles before launching.
    pub fn validate(&self) -> Result<(), RunnerError> {
        if self.clock.is_none() && self.players.iter().any(|p| p.clock.is_some()) {
            return Err(err("a participant clock requires the shared clock"));
        }
        for clock in (0..self.players.len()).filter_map(|p| self.player_clock(p)) {
            if !clock.valid() {
                return Err(err("invalid finite Fischer clock"));
            }
        }
        for clock in (0..self.players.len()).filter_map(|p| self.player_clock(p)) {
            // Upper bound even if every command earns an increment without time charge.
            let remaining = clock
                .base_ms
                .saturating_add(clock.increment_ms.saturating_mul(u64::from(self.max_steps)));
            if self.watchdog_ms < clock_share_ms(clock, remaining) {
                return Err(err("engine watchdog is below possible clock allocation"));
            }
        }
        if self.schema != SCHEMA
            || !bounded(&self.id)
            || !bounded(&self.protocol)
            || !(2..=4).contains(&self.players.len())
            || self.openings.is_empty()
            || self.openings.len() > 16
            || !(1..=16).contains(&self.rounds)
            || self.openings.len() * usize::from(self.rounds) * (self.players.len() - 1) * 2 > 128
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
            return Err(err("config encoding exceeds 1MiB"));
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
    // This existing experiment format promises deterministic depth/node work.
    // Timed Chess execution is available through the child and UCI contracts;
    // it must not silently become a deterministic native experiment.
    if work.time_ms.is_some() {
        return false;
    }
    fn bounds<G: EngineGame>() -> (u16, u64) {
        (G::MAX_DEPTH, G::MAX_NODES)
    }
    use mshogi_engine::game;
    let (depth, nodes) = match profile.game() {
        mshogi_core::native::GameKind::Xiangqi => bounds::<game::Xiangqi>(),
        mshogi_core::native::GameKind::Shogi => bounds::<game::Shogi>(),
        mshogi_core::native::GameKind::Tori => bounds::<game::Tori>(),
        mshogi_core::native::GameKind::Chu => bounds::<game::Chu>(),
        mshogi_core::native::GameKind::Tenjiku => bounds::<game::Tenjiku>(),
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
    /// Optional wall-clock ledger, independently reconstructed on replay.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clock: Option<ClockReceipt>,
    /// Actual request/launch/search/receipt duration, not pure search time.
    pub elapsed_ns: u64,
    /// Exact actual submitted request, even if cancelled before a response.
    pub request: ExecutionContext,
    /// Actual the internal protocol observation, never fabricated on cancellation.
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
    let mut candidate = state.clone();
    let decision = candidate.admit(result)?;
    let after = candidate
        .root()
        .map_err(|e| format!("transition evidence: {e}"))?;
    let context = ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {},
        request_id: u64::MAX,
        session_version: u64::MAX,
        game: after.record,
        limits,
    };
    if !context.valid_limits() {
        return Err("transition evidence has unsupported work limits".into());
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
/// An owned report whose complete evidence has passed native validation.
///
/// Read access cannot change the report. To edit it, consume it with
/// [`Self::into_report`]; the resulting report must be validated again.
///
/// ```compile_fail
/// use mshogi_runner::experiments::ValidatedReport;
/// fn change_score(mut report: ValidatedReport) {
///     report.games[0].score_first_twice = Some(2);
/// }
/// ```
#[derive(Clone, Debug)]
pub struct ValidatedReport(Report);
impl ValidatedReport {
    /// Strict external parsing and complete native validation, once.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        Report::from_json(text).map(Self)
    }
    /// Immutable access to the evidence that was validated.
    pub fn as_report(&self) -> &Report {
        &self.0
    }
    /// Discard validated status and return editable evidence.
    pub fn into_report(self) -> Report {
        self.0
    }
    /// Encode this unchanged report without repeating native replay.
    pub fn to_json(&self) -> Result<String, RunnerError> {
        self.0.encode()
    }
}
impl std::ops::Deref for ValidatedReport {
    type Target = Report;
    fn deref(&self) -> &Report {
        self.as_report()
    }
}

impl Report {
    /// Bound, strict parsing followed by authoritative consistency replay.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        if text.len() > MAX_BYTES {
            return Err(err("experiment report exceeds 128MiB"));
        }
        let report = Self::decode(serde_json::from_str(text).map_err(err)?)?;
        report.validate()?;
        Ok(report)
    }
    /// Encoded form: each step's native record (request and observation
    /// context) is omitted, because it equals the replayed native history
    /// before that step (MSH-204, Tori Strength Harness). Without this, report
    /// size grew with the square of game length.
    fn encoded(&self) -> Result<serde_json::Value, RunnerError> {
        let mut value = serde_json::to_value(self).map_err(err)?;
        for game in value["games"].as_array_mut().into_iter().flatten() {
            for step in game["steps"].as_array_mut().into_iter().flatten() {
                elide(step);
            }
        }
        Ok(value)
    }
    /// Restore each omitted step record from native replay of the recorded
    /// observations, without decoding that record again. A step that carries
    /// its record is decoded strictly; validation compares it with replay.
    fn decode(mut value: serde_json::Value) -> Result<Self, RunnerError> {
        let config: Config = serde_json::from_value(value["config"].clone()).map_err(err)?;
        config.validate()?;
        let mut games = vec![];
        for game in value["games"].as_array_mut().into_iter().flatten() {
            let pairing: Pairing = serde_json::from_value(game["pairing"].clone()).map_err(err)?;
            let opening = config
                .openings
                .get(pairing.opening)
                .ok_or_else(|| err("pairing opening outside the configuration"))?;
            let mut state = native::NativeState::from_root(&opening.root).map_err(err)?;
            let raw = std::mem::take(&mut game["steps"]);
            let mut steps = vec![];
            for raw in raw.as_array().into_iter().flatten() {
                let step = decode_step(raw, &state)?;
                if let (true, Some(o)) = (step.applied, &step.observation) {
                    admit_bounded(&mut state, &o.result, step.request.limits).map_err(err)?;
                }
                steps.push(step);
            }
            game["steps"] = serde_json::Value::Array(vec![]);
            games.push(steps);
        }
        let mut report: Self = serde_json::from_value(value).map_err(err)?;
        if report.games.len() != games.len() {
            return Err(err("report game count changed while decoding"));
        }
        for (game, steps) in report.games.iter_mut().zip(games) {
            game.steps = steps;
        }
        Ok(report)
    }
    /// Consume editable evidence after one complete native validation.
    pub fn into_validated(self) -> Result<ValidatedReport, RunnerError> {
        self.validate()?;
        Ok(ValidatedReport(self))
    }
    /// Encode after checking native replay and complete evidence bounds.
    pub fn to_json(&self) -> Result<String, RunnerError> {
        self.validate()?;
        self.encode()
    }
    fn encode(&self) -> Result<String, RunnerError> {
        let s = self.encoded()?.to_string();
        if s.len() > MAX_BYTES {
            return Err(err("report encoding exceeds 128MiB"));
        }
        Ok(s)
    }
    /// Re-admit recorded observations without rerunning engines or trusting scores.
    pub fn validate(&self) -> Result<(), RunnerError> {
        self.config.validate()?;
        if self.encoded()?.to_string().len() > MAX_BYTES {
            return Err(err("typed report exceeds 128MiB"));
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
            let mut remaining: Vec<u64> = (0..self.config.players.len())
                .map(|p| self.config.player_clock(p).map_or(0, |c| c.base_ms))
                .collect();
            let mut moved = vec![false; self.config.players.len()];
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
                    || c.limits
                        != clock_work(
                            self.config.players[player].work(c.game.profile())?,
                            self.config.player_clock(player),
                            remaining[player],
                            !moved[player],
                        )
                    || c.request_id <= last_id
                    || c.session_version != c.request_id
                    || c.purpose != (ExecutionPurpose::EngineTurn {})
                {
                    return Err(err("stale/wrong participant, history, purpose or limits"));
                }
                let receipt = self.config.player_clock(player).map(|clock| {
                    clock_receipt(clock, remaining[player], step.elapsed_ns, step.applied)
                });
                if step.clock != receipt {
                    return Err(err("clock ledger mismatch"));
                }
                let exhausted = receipt
                    .as_ref()
                    .is_some_and(|r| r.charged_ms >= r.before_ms);
                if let Some(r) = receipt {
                    remaining[player] = r.after_ms;
                }
                moved[player] = true;
                last_id = c.request_id;
                let Some(observation) = &step.observation else {
                    let reason = step
                        .failure
                        .as_deref()
                        .ok_or_else(|| err("missing response without recorded interruption"))?;
                    if !["cancelled", "wall_budget", "clock_limit"].contains(&reason)
                        || step.applied
                        || (reason == "clock_limit" && !exhausted)
                    {
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
                let refusal = if exhausted {
                    Some("clock_limit")
                } else if !observation.matches(c) {
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
/// One encoded step. Without a request record, the record is the replayed
/// native history before the step, supplied to both request and observation.
fn decode_step(raw: &serde_json::Value, state: &native::NativeState) -> Result<Step, RunnerError> {
    if raw["request"].get("game").is_some() {
        return serde_json::from_str(&raw.to_string()).map_err(err);
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct ContextWire {
        purpose: ExecutionPurpose,
        request_id: u64,
        session_version: u64,
        limits: WorkLimits,
    }
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct StepWire {
        player: usize,
        #[serde(default)]
        clock: Option<ClockReceipt>,
        elapsed_ns: u64,
        request: ContextWire,
        observation: Option<Box<serde_json::value::RawValue>>,
        failure: Option<String>,
        applied: bool,
    }
    let wire: StepWire = serde_json::from_str(&raw.to_string()).map_err(err)?;
    let game = state.root().map_err(err)?.record;
    let observation = wire
        .observation
        .map(|o| {
            ExecutionObservation::from_json_with_record(o.get(), game.clone(), &mut || false)
                .map_err(|e| err(e.to_string()))
        })
        .transpose()?;
    Ok(Step {
        player: wire.player,
        clock: wire.clock,
        elapsed_ns: wire.elapsed_ns,
        request: ExecutionContext {
            purpose: wire.request.purpose,
            request_id: wire.request.request_id,
            session_version: wire.request.session_version,
            game,
            limits: wire.request.limits,
        },
        observation,
        failure: wire.failure,
        applied: wire.applied,
    })
}
/// Remove the replay-determined native record from one encoded step.
fn elide(step: &mut serde_json::Value) {
    if let Some(c) = step.get_mut("request").and_then(|c| c.as_object_mut()) {
        c.remove("game");
    }
    if let Some(c) = step
        .get_mut("observation")
        .and_then(|o| o.get_mut("context"))
        .and_then(|c| c.as_object_mut())
    {
        c.remove("game");
    }
}
/// Encoded size basis of one step (record omitted).
pub(super) fn encoded_step(step: &Step) -> Result<serde_json::Value, RunnerError> {
    let mut value = serde_json::to_value(step).map_err(err)?;
    elide(&mut value);
    Ok(value)
}
fn stop_for(a: native::Admission) -> Option<String> {
    match a {
        native::Admission::Applied => None,
        native::Admission::Finished => Some("native_concluded".into()),
        native::Admission::Incomplete(s) => Some(format!("incomplete:{s}")),
        native::Admission::Failed(s)
            if s == "cancelled" || s == "wall_budget" || s == "clock_limit" =>
        {
            Some(s)
        }
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
