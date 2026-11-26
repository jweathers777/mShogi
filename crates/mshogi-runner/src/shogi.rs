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
        let config: Self = common::decode(text, common::CONFIG_BYTES)?;
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
pub use crate::{
    FailureKind,
    protocol::{Failure, Process, Traffic},
};
/// Accepted external reply with this game's native actor.
pub type Turn = protocol::Turn<Side>;
pub use crate::recorded::Stop;
/// One native game and its bounded process observations.
pub type GameReport = crate::recorded::GameReport<Side, Conclusion, ()>;
/// Recorded games with their effective configuration and executable identities.
pub type Report = crate::recorded::Report<Config, Side, Conclusion, ()>;
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
impl crate::recorded::Opening for Opening {
    type Adapter = Adapter;
    fn open(&self) -> Result<Game, RunnerError> {
        self.game()
    }
}
impl Config {
    fn schedule(&self) -> Result<crate::recorded::Schedule<'_, Opening>, RunnerError> {
        self.validate()?;
        Ok(crate::recorded::Schedule {
            settings: self.settings(),
            openings: &self.openings,
            paired: self.paired,
        })
    }
}
impl Report {
    /// Decode bounded evidence and verify the native history.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        let report: Self = common::decode(text, common::REPORT_BYTES)?;
        report.replay()?;
        Ok(report)
    }
    /// Encode bounded evidence.
    pub fn to_json(&self) -> Result<String, RunnerError> {
        common::encode_report(self)
    }
    /// Recreate each game from its opening and recorded replies.
    pub fn replay(&self) -> Result<Vec<Game>, RunnerError> {
        crate::recorded::replay(self, self.config.schedule()?)
    }
}
/// Run each opening and engine assignment through the native protocol adapter.
pub fn run(config: &Config) -> Result<Report, RunnerError> {
    let report = crate::recorded::run(config, config.schedule()?)?;
    report.to_json()?;
    Ok(report)
}

pub(crate) struct Adapter;
impl Native for Adapter {
    type Game = Game;
    type Side = Side;
    type Conclusion = Conclusion;
    type Pending = ();
    fn conclusion(game: &Game) -> Option<Conclusion> {
        game.conclusion()
    }
    fn pending(_game: &Game) -> Option<Self::Pending> {
        None
    }
    fn score(conclusion: Conclusion) -> [u8; 2] {
        conclusion.score_half_points()
    }
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
    fn admit_turn(game: &Game, text: &str) -> Result<Option<String>, Rejection> {
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
