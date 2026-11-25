//! Untimed USI experiments. A fresh child per turn gives USI's untagged replies
//! an unambiguous execution generation; retired output can never serve a later go.
use crate::{
    protocol::{self, End, Native, Rejection},
    *,
};
use mshogi_core::{native::CanonicalRecord, shogi::*};
use serde::{Deserialize, Serialize};

/// One known-history root and supplied legal board prefix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Opening {
    /// Stable experiment label.
    pub id: String,
    /// None is the exact standard root; Some is an imported diagram, never prior history.
    pub sfen: Option<String>,
    /// Complete known board-action suffix.
    pub moves: Vec<String>,
}
impl Opening {
    pub(crate) fn game(&self) -> Result<Game, RunnerError> {
        let mut game = if let Some(sfen) = &self.sfen {
            let position = Position::from_sfen(sfen).map_err(|e| RunnerError(e.to_string()))?;
            let root = match GameRoot::import(
                position.clone(),
                RootCategory::FullMaterialDiagram,
                CounterSource::Supplied,
            ) {
                Err(RootError::CategoryMismatch) => GameRoot::import(
                    position,
                    RootCategory::ReducedMaterialDiagram,
                    CounterSource::Supplied,
                ),
                r => r,
            }
            .map_err(|e| RunnerError(format!("native root: {e:?}")))?;
            Game::from_root(root)
        } else {
            Game::new()
        };
        for text in &self.moves {
            let action: Action = text
                .parse()
                .map_err(|e| RunnerError(format!("opening action: {e}")))?;
            submit(&mut game, GameCommand::Play(action)).map_err(RunnerError)?;
        }
        Ok(game)
    }
}
/// Explicit untimed, restart-per-turn experiment input, independent of chess config.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
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
    pub openings: Vec<Opening>,
    /// Reverse engine assignments for every opening.
    pub paired: bool,
    /// Per-engine finite work, no movetime.
    pub work: [WorkLimit; 2],
    /// Maximum engine commands after the opening; an experiment cutoff, not a rule.
    pub max_actions: u32,
    /// Overall per-game experiment watchdog; expiry is unfinished, not a chess clock.
    pub max_game_ms: u64,
    /// Independent process watchdogs.
    pub watchdogs: Watchdogs,
    /// Explicit stream/report bounds.
    pub io: IoLimits,
}
impl Config {
    fn settings(&self) -> protocol::Settings<'_> {
        protocol::Settings {
            engines: &self.engines,
            watchdogs: self.watchdogs,
            io: self.io,
            max_game_ms: self.max_game_ms,
            max_actions: self.max_actions,
            work: self.work,
        }
    }

    /// Strict, bounded input parsing.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        if text.len() > 1_048_576 {
            return Err(RunnerError("shogi config exceeds 1MiB".into()));
        }
        let config: Self = serde_json::from_str(text).map_err(|e| RunnerError(e.to_string()))?;
        config.validate()?;
        Ok(config)
    }
    /// Validate typed callers before starting children.
    pub fn validate(&self) -> Result<(), RunnerError> {
        if self.schema_version != 1
            || self.profile != PROFILE_ID
            || self.time_control != "untimed"
            || self.process_policy != "fresh_per_turn"
        {
            return Err(RunnerError(
                "unsupported shogi version/profile/time_control/process_policy".into(),
            ));
        }
        let settings = self.settings();
        settings.validate_bounds(&self.id, self.openings.len())?;
        settings.validate_work(
            "Shogi",
            mshogi_search::shogi::MAX_DEPTH,
            mshogi_search::shogi::MAX_NODES,
        )?;
        for o in &self.openings {
            if o.id.is_empty()
                || o.id.len() > 128
                || o.moves.len() > 320
                || o.sfen.as_ref().is_some_and(|s| s.len() > 4096)
            {
                return Err(RunnerError("invalid opening bounds".into()));
            }
            o.game()?;
        }
        settings.validate_processes("shogi")
    }
}
pub use crate::protocol::FailureKind;
/// Bounded protocol failure with this game's native categories.
pub type Failure = protocol::Failure<FailureKind>;
pub use crate::protocol::{Process, Traffic};
/// Accepted external reply with this game's native actor.
pub type Turn = protocol::Turn<Side>;
/// Why this experiment stopped; distinct from the native conclusion.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stop {
    /// Native authoritative conclusion.
    NativeConclusion,
    /// Engine command experiment cap.
    ActionLimit,
    /// Overall experiment watchdog, not a clock loss.
    TimeLimit,
    /// Unscored failure.
    Failure,
}
/// One complete accepted native game plus bounded external observations.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GameReport {
    /// Opening index.
    pub opening: usize,
    /// Engine indices assigned to Sente/Gote.
    pub assignment: [usize; 2],
    /// Complete typed canonical envelope string preserves strict native raw parsing.
    pub game: String,
    /// Native conclusion independently checked on replay.
    pub conclusion: Option<Conclusion>,
    /// Half-point competition units only for completed successful experiments.
    pub score: Option<[u8; 2]>,
    /// Experiment stop reason.
    pub stop: Stop,
    /// Unscored execution failure if present.
    pub failure: Option<Failure>,
    /// Every accepted engine proposal with exact context provenance.
    pub turns: Vec<Turn>,
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
pub struct Report {
    /// Current report schema, one.
    pub schema_version: u32,
    /// Effective configuration including finite limits and all options.
    pub config: Config,
    /// Actual executable/model hashes (no PATH lookup).
    pub identities: [EngineIdentity; 2],
    /// Platform which actually ran these processes.
    pub platform: String,
    /// Opening/assignment-specific games.
    pub games: Vec<GameReport>,
}
pub(crate) fn canonical(game: &Game) -> Result<String, RunnerError> {
    CanonicalRecord::shogi(GameRecord::from_game(game))
        .to_json()
        .map_err(|e| RunnerError(e.to_string()))
}
pub(crate) fn submit(game: &mut Game, command: GameCommand) -> Result<(), String> {
    game.submit(GameRequest {
        expected: game.context(),
        actor: game.position().side_to_move(),
        command,
    })
    .map(|_| ())
    .map_err(|e| format!("{e:?}"))
}
pub(crate) fn position(game: &Game) -> String {
    let mut text = if game.root().has_complete_initial_history() {
        "position startpos".into()
    } else {
        format!("position sfen {}", game.root().position().to_sfen())
    };
    if !game.events().is_empty() {
        text.push_str(" moves");
        for e in game.events() {
            if let GameCommand::Play(action) = e.command {
                text.push(' ');
                text.push_str(&action.to_string());
            }
        }
    }
    text
}
pub(crate) fn reply(text: &str) -> Result<GameCommand, String> {
    let words: Vec<_> = text.split_ascii_whitespace().collect();
    let token = match words.as_slice() {
        ["bestmove", token] => *token,
        ["bestmove", token, "ponder", ponder] => {
            ponder
                .parse::<Action>()
                .map_err(|e| format!("malformed ponder action: {e}"))?;
            if matches!(*token, "win" | "resign") {
                return Err("terminal bestmove cannot carry ponder".into());
            }
            *token
        }
        _ => return Err("expected bestmove <action|win|resign> [ponder <board-action>]".into()),
    };
    match token {
        "win" => Ok(GameCommand::DeclareEnteringKing),
        "resign" => Ok(GameCommand::Resign),
        token => token
            .parse::<Action>()
            .map(GameCommand::Play)
            .map_err(|e| e.to_string()),
    }
}
impl Report {
    /// Strict bounded report decoding followed by native replay.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        if text.len() > 67_108_864 {
            return Err(RunnerError("shogi report exceeds 64MiB".into()));
        }
        let report: Self = serde_json::from_str(text).map_err(|e| RunnerError(e.to_string()))?;
        report.replay()?;
        Ok(report)
    }
    /// Canonical report JSON; all accepted history remains in the native envelope.
    pub fn to_json(&self) -> Result<String, RunnerError> {
        let text = serde_json::to_string(self).map_err(|e| RunnerError(e.to_string()))?;
        if text.len() > 67_108_864 {
            return Err(RunnerError("report exceeds supported 64MiB bound".into()));
        }
        Ok(text)
    }
    /// Verify root, all accepted responses, context/limits/provenance and conclusions.
    pub fn replay(&self) -> Result<Vec<Game>, RunnerError> {
        self.config.validate()?;
        if self.schema_version != 1
            || self.games.len()
                != self.config.openings.len() * if self.config.paired { 2 } else { 1 }
        {
            return Err(RunnerError("report version/game count mismatch".into()));
        }
        let mut games = Vec::new();
        let mut generation = 0;
        for (index, entry) in self.games.iter().enumerate() {
            let opening = index / if self.config.paired { 2 } else { 1 };
            let assignment = if self.config.paired && index % 2 == 1 {
                [1, 0]
            } else {
                [0, 1]
            };
            if entry.opening != opening || entry.assignment != assignment {
                return Err(RunnerError("opening/assignment mismatch".into()));
            }
            let mut game = self.config.openings[opening].game()?;
            protocol::replay::<Adapter>(
                self.config.settings(),
                protocol::Replay {
                    turns: &entry.turns,
                    traffic: &entry.traffic,
                    processes: &entry.processes,
                    elapsed_ns: entry.elapsed_ns,
                    game: &entry.game,
                },
                &mut game,
                assignment,
                &mut generation,
            )?;
            let CanonicalRecord::Shogi(record) =
                CanonicalRecord::from_json(&entry.game).map_err(|e| RunnerError(e.to_string()))?
            else {
                return Err(RunnerError("report game is not shogi".into()));
            };
            if record.to_game().map_err(|e| RunnerError(e.to_string()))? != game
                || entry.conclusion != game.conclusion()
            {
                return Err(RunnerError("canonical game/conclusion mismatch".into()));
            }
            let score = if entry.stop == Stop::NativeConclusion && entry.failure.is_none() {
                game.conclusion().map(Conclusion::score_half_points)
            } else {
                None
            };
            if (!entry.traffic_complete && !matches!(entry.stop, Stop::Failure | Stop::TimeLimit))
                || entry.score != score
                || (entry.stop == Stop::NativeConclusion) != game.conclusion().is_some()
                    && entry.stop != Stop::Failure
                || (entry.stop == Stop::Failure) != entry.failure.is_some()
                || entry.turns.len() > self.config.max_actions as usize
                || (entry.stop == Stop::ActionLimit
                    && entry.turns.len() != self.config.max_actions as usize)
            {
                return Err(RunnerError("experiment stop/score mismatch".into()));
            }
            games.push(game);
        }
        Ok(games)
    }
}
/// Run each opening and engine assignment through the native protocol adapter.
pub fn run(config: &Config) -> Result<Report, RunnerError> {
    config.validate()?;
    let identities = protocol::identities(&config.engines);
    let mut report = Report {
        schema_version: 1,
        config: config.clone(),
        identities,
        platform: format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH),
        games: Vec::new(),
    };
    let mut generation = 0;
    for (opening_index, opening) in config.openings.iter().enumerate() {
        for reversed in 0..if config.paired { 2 } else { 1 } {
            let assignment = if reversed == 1 { [1, 0] } else { [0, 1] };
            let completed = protocol::run_game::<Adapter>(
                config.settings(),
                opening.game()?,
                assignment,
                &mut generation,
            )?;
            let game = completed.game;
            let stop = match completed.stop {
                End::NativeConclusion => Stop::NativeConclusion,
                End::ActionLimit => Stop::ActionLimit,
                End::TimeLimit => Stop::TimeLimit,
                End::Failure => Stop::Failure,
                End::Unresolved => unreachable!("Shogi has no pending adjudication"),
            };
            report.games.push(GameReport {
                opening: opening_index,
                assignment,
                game: canonical(&game)?,
                conclusion: game.conclusion(),
                score: if stop == Stop::NativeConclusion {
                    game.conclusion().map(Conclusion::score_half_points)
                } else {
                    None
                },
                stop,
                failure: completed.failure,
                turns: completed.turns,
                traffic: completed.traffic,
                traffic_complete: completed.traffic_complete,
                processes: completed.processes,
                elapsed_ns: completed.elapsed_ns,
            });
        }
    }
    report.replay()?;
    report.to_json()?;
    Ok(report)
}

pub(crate) struct Adapter;
impl Native for Adapter {
    type Game = Game;
    type Side = Side;
    type Kind = FailureKind;
    const HELLO: &'static str = "usi";
    const OK: &'static str = "usiok";
    const NEWGAME: &'static str = "usinewgame";
    const READY_AFTER_NEWGAME: bool = false;
    fn actor(game: &Game) -> Side {
        game.position().side_to_move()
    }
    fn side(side: Side) -> usize {
        usize::from(side == Side::Gote)
    }
    fn events(game: &Game) -> usize {
        game.events().len()
    }
    fn end(game: &Game) -> Option<End> {
        game.conclusion().map(|_| End::NativeConclusion)
    }
    fn canonical(game: &Game) -> Result<String, RunnerError> {
        canonical(game)
    }
    fn position(game: &Game) -> String {
        position(game)
    }
    fn apply_reply(game: &mut Game, text: &str) -> Result<(), RunnerError> {
        submit(game, reply(text).map_err(RunnerError)?).map_err(RunnerError)
    }
    fn admit_turn(game: &Game, text: &str) -> Result<Option<String>, Rejection<FailureKind>> {
        let command = reply(text).map_err(|detail| Rejection {
            kind: FailureKind::MalformedReply,
            phase: "search",
            detail,
        })?;
        let mut candidate = game.clone();
        submit(&mut candidate, command).map_err(|detail| Rejection {
            kind: if matches!(command, GameCommand::Play(_)) {
                FailureKind::IllegalAction
            } else {
                FailureKind::Native
            },
            phase: "admission",
            detail,
        })?;
        Ok(candidate.conclusion().map(|conclusion| {
            let notice = match conclusion {
                Conclusion::Draw(_) => "draw",
                Conclusion::Win { winner, .. } if winner == game.position().side_to_move() => "win",
                _ => "lose",
            };
            format!("gameover {notice}")
        }))
    }
}
