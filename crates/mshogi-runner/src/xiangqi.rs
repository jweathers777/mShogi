//! Untimed UCI0-9 experiments. A fresh child per turn gives UCI0-9's untagged replies
//! an unambiguous execution generation; retired output can never serve a later go.
use crate::{
    protocol::{self, End, Native, Rejection},
    *,
};
use mshogi_core::{native::CanonicalRecord, xiangqi::*};
use serde::{Deserialize, Serialize};

/// One native known-history root and supplied board prefix. Canonical input is
/// carried as raw JSON text so native duplicate rejection is never normalized.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Opening {
    /// Stable experiment label.
    pub id: String,
    /// None is the exact standard root; supplied FEN begins known history.
    pub fen: Option<String>,
    /// Full native canonical envelope, mutually exclusive with FEN and moves.
    pub canonical: Option<String>,
    /// Complete native board-action suffix; supplied counters are not history.
    pub moves: Vec<String>,
}
impl Opening {
    pub(crate) fn game(&self) -> Result<Game, RunnerError> {
        let mut game = if let Some(text) = &self.canonical {
            if self.fen.is_some() || !self.moves.is_empty() {
                return Err(RunnerError(
                    "canonical opening cannot also specify FEN/moves".into(),
                ));
            }
            let CanonicalRecord::Xiangqi(record) =
                CanonicalRecord::from_json(text).map_err(|e| RunnerError(e.to_string()))?
            else {
                return Err(RunnerError("opening canonical game is not xiangqi".into()));
            };
            record.to_game().map_err(|e| RunnerError(e.to_string()))?
        } else {
            let root = match &self.fen {
                Some(fen) => PositionRoot::from_fen(fen).map_err(|e| RunnerError(e.to_string()))?,
                None => PositionRoot::standard(),
            };
            Game::from_root(root, ClaimPolicy::AutoValidClaims)
                .map_err(|e| RunnerError(format!("native root: {e:?}")))?
        };
        if game.claim_policy() != ClaimPolicy::AutoValidClaims {
            return Err(RunnerError("unsupported_adjudication_policy: UCI0-9 has no optional claim channel; auto_valid_claims required".into()));
        }
        for text in &self.moves {
            let action: Action = text
                .parse()
                .map_err(|e| RunnerError(format!("opening action: {e}")))?;
            submit(&mut game, GameCommand::Play { action }).map_err(RunnerError)?;
        }
        // Terminal native games need no conversion or child. Live external play
        // requires a lossless board-command prefix; never drop terminal-command history.
        if game.conclusion().is_none() && game.pending_adjudication().is_none() {
            external_prefix(&game).map_err(RunnerError)?;
        }
        Ok(game)
    }
}
/// Protocol-specific configuration fields.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Policy {
    /// Exactly pikafish_uci0_9; no Fairy generic or UCCI conversion is advertised.
    pub dialect: String,
    /// Exactly auto_valid_claims; retained independently of engine heuristics.
    pub claim_policy: ClaimPolicy,
}
/// Finite-work games using this native adapter.
pub type Config = crate::recorded::Config<Opening, Policy>;
impl Config {
    /// Strict, bounded input parsing.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        let config: Self = crate::common::decode_config(text)?;
        config.validate()?;
        Ok(config)
    }
    /// Validate typed callers before starting children.
    pub fn validate(&self) -> Result<(), RunnerError> {
        if self.policy.dialect != "pikafish_uci0_9"
            || self.policy.claim_policy != ClaimPolicy::AutoValidClaims
        {
            return Err(RunnerError(
                "unsupported dialect or adjudication policy (auto_valid_claims required)".into(),
            ));
        }
        self.validate_common(
            PROFILE_ID,
            mshogi_search::xiangqi::MAX_DEPTH,
            mshogi_search::xiangqi::MAX_NODES,
            |o| {
                if o.id.is_empty()
                    || o.id.len() > 128
                    || o.fen.as_ref().is_some_and(|s| s.len() > 256)
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
/// Accepted external reply with this game's native actor.
pub type Turn = protocol::Turn<Side>;
pub use crate::recorded::Stop;
/// One native game and its bounded process observations.
pub type GameReport = crate::recorded::GameReport<Side, Conclusion, PendingAdjudication>;
/// Recorded games with their effective configuration and executable identities.
pub type Report = crate::recorded::Report<Config, Side, Conclusion, PendingAdjudication>;
pub(crate) fn canonical(game: &Game) -> Result<String, RunnerError> {
    CanonicalRecord::xiangqi(GameRecord::from_game(game).map_err(|e| RunnerError(e.to_string()))?)
        .to_json()
        .map_err(|e| RunnerError(e.to_string()))
}
fn admit(game: &mut Game, command: GameCommand) -> Result<(), GameError> {
    game.submit(GameRequest {
        expected: game.context(),
        actor: game.position().side_to_move(),
        command,
    })
    .map(|_| ())
}
pub(crate) fn submit(game: &mut Game, command: GameCommand) -> Result<(), String> {
    admit(game, command).map_err(|e| e.to_string())
}
fn external_prefix(game: &Game) -> Result<(), String> {
    if game.root().origin() == RootOrigin::AnalysisDiagram {
        return Err(
            "unsupported root provenance: UCI0-9 cannot retain analysis-diagram identity".into(),
        );
    }
    game.root().position().to_fen().map_err(|e| e.to_string())?;
    let counters = game.root().position().counters();
    if counters.no_capture_plies().is_none_or(|n| n > 119)
        || counters.fullmove_number().is_none_or(|n| n > 100_000)
    {
        return Err("unsupported imported root counters for pinned Pikafish: no-capture 0..119, fullmove 1..100000; no normalization".into());
    }
    // Native parser already validated this payload. Inspect only command encoding,
    // never reimplement legality/adjudication. Board-only protocol cannot carry offers,
    // rejected claims or other same-ply participant commands.
    let record = GameRecord::from_game(game).map_err(|e| e.to_string())?;
    let wire: serde_json::Value =
        serde_json::from_str(record.to_json()).map_err(|e| e.to_string())?;
    if wire["commands"]
        .as_array()
        .ok_or("native command ledger absent")?
        .iter()
        .any(|c| c["command"]["kind"] != "play")
    {
        return Err("unsupported_adjudication: live UCI0-9 context contains non-board command history; use structured native execution".into());
    }
    Ok(())
}
pub(crate) fn position(game: &Game) -> String {
    let mut text = if game.root().origin() == RootOrigin::StandardInitial {
        "position startpos".into()
    } else {
        format!(
            "position fen {}",
            game.root()
                .position()
                .to_fen()
                .expect("validated representable root")
        )
    };
    let moves: Vec<_> = game
        .events()
        .iter()
        .filter_map(|e| match e.event {
            EventKind::BoardAction { action, .. } => Some(action.to_string()),
            _ => None,
        })
        .collect();
    if !moves.is_empty() {
        text.push_str(" moves ");
        text.push_str(&moves.join(" "));
    }
    text
}
pub(crate) fn score(conclusion: Conclusion) -> [u8; 2] {
    match conclusion {
        Conclusion::Draw { .. } => [1, 1],
        Conclusion::Win {
            winner: Side::Red, ..
        } => [2, 0],
        Conclusion::Win {
            winner: Side::Black,
            ..
        } => [0, 2],
    }
}

pub(crate) fn reply(text: &str) -> Result<GameCommand, String> {
    let words: Vec<_> = text.split_ascii_whitespace().collect();
    let token = match words.as_slice() {
        ["bestmove", token] => *token,
        ["bestmove", token, "ponder", ponder] => {
            ponder
                .parse::<Action>()
                .map_err(|e| format!("malformed ponder action: {e}"))?;
            if *token == "resign" {
                return Err("resignation cannot carry ponder".into());
            }
            *token
        }
        _ => return Err("expected bestmove <a0a1|resign> [ponder <a0a1>]".into()),
    };
    match token { "resign" => Ok(GameCommand::Resign),
        "0000" | "(none)" => Err("missing board action: native host owns terminal adjudication; null reply is not a draw or resignation".into()),
        token => token.parse::<Action>().map(|action| GameCommand::Play { action }).map_err(|e| e.to_string()), }
}
impl crate::recorded::Opening for Opening {
    type Adapter = Adapter;
    fn open(&self) -> Result<Game, RunnerError> {
        self.game()
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
        self.config.validate()?;
        crate::recorded::replay(self, self.config.schedule())
    }
}
/// Run each opening and engine assignment through the native protocol adapter.
pub fn run(config: &Config) -> Result<Report, RunnerError> {
    config.validate()?;
    let report = crate::recorded::run(config, config.schedule())?;
    report.to_json()?;
    Ok(report)
}

pub(crate) struct Adapter;
impl Native for Adapter {
    type Game = Game;
    type Side = Side;
    type Conclusion = Conclusion;
    type Pending = PendingAdjudication;
    fn conclusion(game: &Game) -> Option<Conclusion> {
        game.conclusion()
    }
    fn pending(game: &Game) -> Option<Self::Pending> {
        game.pending_adjudication().cloned()
    }
    fn score(conclusion: Conclusion) -> [u8; 2] {
        score(conclusion)
    }
    const DIALECT: crate::transport::Dialect = crate::transport::Dialect::Uci;
    fn search_failure(line: &str) -> Option<(FailureKind, &str)> {
        let detail = line.strip_prefix("info string mshogi execution_failure ")?;
        let kind = if detail.starts_with("incomplete:") {
            FailureKind::IncompleteWork
        } else if detail.starts_with("unresolved:") {
            FailureKind::UnresolvedWork
        } else if detail.starts_with("unsupported_adjudication:")
            || detail.starts_with("native_terminal:")
        {
            FailureKind::UnsupportedAdjudication
        } else {
            FailureKind::ExecutionFailure
        };
        Some((kind, detail))
    }
    fn actor(game: &Game) -> Side {
        game.position().side_to_move()
    }
    fn side(side: Side) -> usize {
        usize::from(side == Side::Black)
    }
    fn events(game: &Game) -> usize {
        game.events().len()
    }
    fn end(game: &Game) -> Option<End> {
        if game.conclusion().is_some() {
            Some(End::NativeConclusion)
        } else if game.pending_adjudication().is_some() {
            Some(End::Unresolved)
        } else {
            None
        }
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

        admit(game, command).map_err(|e| Rejection {
            kind: match e {
                GameError::Action(_) | GameError::WrongActor => FailureKind::IllegalAction,
                GameError::AdjudicationPending => FailureKind::UnresolvedWork,
                _ => FailureKind::Native,
            },
            phase: "admission",
            detail: e.to_string(),
        })?;
        canonical(game).map_err(|e| Rejection {
            kind: FailureKind::ResourceLimit,
            phase: "canonical_preflight",
            detail: e.to_string(),
        })?;
        Ok(None)
    }
}
