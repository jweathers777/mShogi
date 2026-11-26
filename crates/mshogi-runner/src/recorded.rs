//! Recorded finite-work games and replay against the authoritative native rules.
pub use crate::protocol::End as Stop;
use crate::{
    common::*,
    protocol::{self, Failure, Native, Process, Traffic, Turn},
    *,
};
use serde::{Deserialize, Serialize};
/// Explicit untimed, restart-per-turn experiment input, independent of chess config.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config<O, P> {
    /// Additional fields required by this native protocol.
    #[serde(flatten)]
    pub policy: P,
    /// Current report/config version, one.
    pub schema_version: u32,
    /// Exact immutable native identity.
    pub profile: String,
    /// Experiment label.
    pub id: String,
    /// Must be untimed; no clock adjudication is supplied here.
    pub time_control: String,
    /// Must be fresh_per_turn; no untagged reply crosses request generations.
    pub process_policy: String,
    /// Individual executable/option settings (existing bounded process configuration).
    pub engines: [EngineConfig; 2],
    /// Complete roots/prefixes, at most four.
    pub openings: Vec<O>,
    /// Reverse engine assignments for every opening.
    pub paired: bool,
    /// Per-engine finite work, no movetime.
    pub work: [WorkLimit; 2],
    /// Maximum engine commands after the opening; an experiment cutoff, not a rule.
    pub max_actions: usize,
    /// Overall per-game experiment watchdog; expiry is unfinished, not a chess clock.
    pub max_game_ms: u64,
    /// Independent process watchdogs.
    pub watchdogs: Watchdogs,
    /// Explicit stream/report bounds.
    pub io: IoLimits,
}
impl<O, P> Config<O, P> {
    pub(crate) fn settings(&self) -> protocol::Settings<'_> {
        protocol::Settings {
            engines: &self.engines,
            watchdogs: self.watchdogs,
            io: self.io,
            max_game_ms: self.max_game_ms,
            max_actions: self.max_actions,
            work: self.work,
        }
    }

    pub(crate) fn schedule(&self) -> Schedule<'_, O> {
        Schedule {
            settings: self.settings(),
            openings: &self.openings,
            paired: self.paired,
        }
    }
    pub(crate) fn validate_common(
        &self,
        profile: &str,
        depth: u16,
        nodes: u64,
        validate_open: impl Fn(&O) -> Result<(), RunnerError>,
    ) -> Result<(), RunnerError> {
        if self.schema_version != 1
            || self.profile != profile
            || self.time_control != "untimed"
            || self.process_policy != "fresh_per_turn"
        {
            return Err(RunnerError(
                "unsupported version/profile/time control/process policy".into(),
            ));
        }
        let settings = self.settings();
        settings.validate_bounds(&self.id, self.openings.len())?;
        settings.validate_work(profile, depth, nodes)?;
        for opening in &self.openings {
            validate_open(opening)?;
        }
        settings.validate_processes(profile)
    }
}

/// One complete accepted native game plus bounded external observations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameReport<S, O, P> {
    /// Opening index.
    pub opening: usize,
    /// Engine indices assigned to Red/Black.
    pub assignment: [usize; 2],
    /// Complete typed canonical envelope string preserves strict native raw parsing.
    pub game: String,
    /// Native conclusion independently checked on replay.
    pub conclusion: Option<O>,
    /// Native pending classification; no heuristic can settle it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<P>,
    /// Half-point competition units only for completed successful experiments.
    pub score: Option<[u8; 2]>,
    /// Experiment stop reason.
    pub stop: Stop,
    /// Unscored execution failure if present.
    pub failure: Option<Failure>,
    /// Every accepted engine proposal with exact context provenance.
    pub turns: Vec<Turn<S>>,
    /// Bounded complete traffic unless resource failure explicitly marks otherwise.
    pub traffic: Vec<Traffic>,
    /// Whether a hostile output exceeded retention capacity.
    pub traffic_complete: bool,
    /// Every fresh process's actual exit/diagnostic/cleanup evidence.
    pub processes: Vec<Process>,
    /// Actual overall elapsed experiment time, not a rules clock.
    pub elapsed_ns: u64,
}
/// Self-contained replay evidence. External engines are not launched on replay.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Report<C, S, O, P> {
    /// Current report schema, one.
    pub schema_version: u32,
    /// Effective configuration including finite limits and all options.
    pub config: C,
    /// Actual executable/model hashes (no PATH lookup).
    pub identities: [EngineIdentity; 2],
    /// Platform which actually ran these processes.
    pub platform: String,
    /// Opening/assignment-specific games.
    pub games: Vec<GameReport<S, O, P>>,
}

pub(crate) trait Opening {
    type Adapter: Native;
    fn open(&self) -> Result<<Self::Adapter as Native>::Game, RunnerError>;
}

pub(crate) struct Schedule<'a, O> {
    pub settings: protocol::Settings<'a>,
    pub openings: &'a [O],
    pub paired: bool,
}
impl<O> Schedule<'_, O> {
    fn games(&self) -> impl Iterator<Item = (usize, [usize; 2], &O)> {
        self.openings
            .iter()
            .enumerate()
            .flat_map(|(index, opening)| {
                [[0, 1], [1, 0]]
                    .into_iter()
                    .take(if self.paired { 2 } else { 1 })
                    .map(move |assignment| (index, assignment, opening))
            })
    }
}
type Evidence<C, G> =
    Report<C, <G as Native>::Side, <G as Native>::Conclusion, <G as Native>::Pending>;

pub(crate) fn replay<C, G: Native, O: Opening<Adapter = G>>(
    report: &Evidence<C, G>,
    schedule: Schedule<'_, O>,
) -> Result<Vec<G::Game>, RunnerError> {
    if report.schema_version != 1 || report.games.len() != schedule.games().count() {
        return Err(RunnerError("report version/game count mismatch".into()));
    }
    let mut generation = 0;
    let mut games = Vec::new();
    for ((index, assignment, opening), entry) in schedule.games().zip(&report.games) {
        if entry.opening != index || entry.assignment != assignment {
            return Err(RunnerError("opening/assignment mismatch".into()));
        }
        let mut game = opening.open()?;
        protocol::replay::<G>(
            schedule.settings,
            protocol::Replay {
                turns: &entry.turns,
                traffic: &entry.traffic,
                processes: &entry.processes,
                elapsed_ns: entry.elapsed_ns,
            },
            &mut game,
            assignment,
            &mut generation,
        )?;
        check_canonical::<G>(&entry.game, &game)?;
        let conclusion = G::conclusion(&game);
        let pending = G::pending(&game);
        let score = (entry.stop == Stop::NativeConclusion)
            .then(|| conclusion.map(G::score))
            .flatten();
        if entry.conclusion != conclusion
            || entry.pending != pending
            || entry.score != score
            || (!entry.traffic_complete && !matches!(entry.stop, Stop::Failure | Stop::TimeLimit))
            || (entry.stop != Stop::Failure
                && (entry.stop == Stop::NativeConclusion) != conclusion.is_some())
            || (entry.stop == Stop::Unresolved) != pending.is_some()
            || (entry.stop == Stop::Failure) != entry.failure.is_some()
            || (entry.stop == Stop::ActionLimit
                && entry.turns.len() != schedule.settings.max_actions)
        {
            return Err(RunnerError("experiment stop/score mismatch".into()));
        }
        games.push(game);
    }
    Ok(games)
}

pub(crate) fn check_canonical<G: Native>(record: &str, game: &G::Game) -> Result<(), RunnerError> {
    let actual = mshogi_core::native::CanonicalRecord::from_json(record)
        .and_then(|record| record.to_json())
        .map_err(|e| RunnerError(e.to_string()))?;
    if actual != G::canonical(game)? {
        return Err(RunnerError("canonical game mismatch".into()));
    }
    Ok(())
}

pub(crate) fn run<C: Clone, G: Native, O: Opening<Adapter = G>>(
    config: &C,
    schedule: Schedule<'_, O>,
) -> Result<Evidence<C, G>, RunnerError> {
    let mut report = Report {
        schema_version: 1,
        config: config.clone(),
        identities: identities(schedule.settings.engines),
        platform: platform(),
        games: Vec::new(),
    };
    let mut generation = 0;
    for (index, assignment, opening) in schedule.games() {
        let outcome = protocol::run_game::<G>(
            schedule.settings,
            opening.open()?,
            assignment,
            &mut generation,
        )?;
        let game = outcome.game;
        let conclusion = G::conclusion(&game);
        report.games.push(GameReport {
            opening: index,
            assignment,
            game: G::canonical(&game)?,
            conclusion,
            pending: G::pending(&game),
            score: (outcome.stop == Stop::NativeConclusion)
                .then(|| conclusion.map(G::score))
                .flatten(),
            stop: outcome.stop,
            failure: outcome.failure,
            turns: outcome.turns,
            traffic: outcome.traffic,
            traffic_complete: outcome.traffic_complete,
            processes: outcome.processes,
            elapsed_ns: outcome.elapsed_ns,
        });
    }
    replay::<C, G, O>(&report, schedule)?;
    Ok(report)
}
