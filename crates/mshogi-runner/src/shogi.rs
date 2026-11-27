//! Untimed USI experiments. A fresh child per turn gives USI's untagged replies
//! an unambiguous execution generation; retired output can never serve a later go.
use crate::{
    protocol::{self, End, ProtocolGame, Rejection},
    *,
};
use mshogi_core::{game::CanonicalRecord, shogi::*};
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
            .map_err(|e| RunnerError(format!("game root: {e:?}")))?;
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
/// Protocol-specific configuration fields.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NoPolicy {}
/// Finite-work games using this adapter.
pub type Config = crate::recorded::Config<Opening, NoPolicy>;
impl Config {
    /// Strict, bounded input parsing.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        let config: Self = crate::common::decode_config(text)?;
        config.validate()?;
        Ok(config)
    }
    /// Validate typed callers before starting children.
    pub fn validate(&self) -> Result<(), RunnerError> {
        self.validate_common(
            PROFILE_ID,
            mshogi_search::shogi::MAX_DEPTH,
            mshogi_search::shogi::MAX_NODES,
            |o| {
                if o.id.is_empty()
                    || o.id.len() > 128
                    || o.sfen.as_ref().is_some_and(|s| s.len() > 4096)
                {
                    return Err(RunnerError("invalid opening bounds".into()));
                }
                o.game().map(|_| ())
            },
        )
    }
}
pub use crate::{
    FailureKind,
    protocol::{Failure, Process, Traffic},
};
/// Accepted external reply with this game's actor.
pub type Turn = protocol::Turn<Side>;
pub use crate::recorded::Stop;
/// One game and its bounded process observations.
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
impl Report {
    /// Decode bounded evidence and verify the game history.
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
        self.config.validate()?;
        crate::recorded::replay(self, self.config.schedule())
    }
}
/// Run each opening and engine assignment through the protocol adapter.
pub fn run(config: &Config) -> Result<Report, RunnerError> {
    config.validate()?;
    let report = crate::recorded::run(config, config.schedule())?;
    report.to_json()?;
    Ok(report)
}

pub(crate) struct Adapter;
impl ProtocolGame for Adapter {
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
    const DIALECT: crate::transport::Dialect = crate::transport::Dialect::Usi;
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
        game.conclusion().map(|_| End::GameConclusion)
    }
    fn canonical(game: &Game) -> Result<String, RunnerError> {
        canonical(game)
    }
    fn position(game: &Game) -> String {
        position(game)
    }
    fn admit_turn(game: &mut Game, text: &str) -> Result<Option<String>, Rejection> {
        let command = reply(text).map_err(|detail| Rejection {
            kind: FailureKind::MalformedReply,
            phase: "search",
            detail,
        })?;
        let actor = game.position().side_to_move();
        submit(game, command).map_err(|detail| Rejection {
            kind: if matches!(command, GameCommand::Play(_)) {
                FailureKind::IllegalAction
            } else {
                FailureKind::GameAdmission
            },
            phase: "admission",
            detail,
        })?;
        Ok(game.conclusion().map(|conclusion| {
            let notice = match conclusion {
                Conclusion::Draw(_) => "draw",
                Conclusion::Win { winner, .. } if winner == actor => "win",
                _ => "lose",
            };
            format!("gameover {notice}")
        }))
    }
}
