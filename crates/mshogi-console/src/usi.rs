//! USI notation, side names, declarations and terminal moves.
use crate::protocol::Dialect;
use mshogi_core::shogi::*;
use mshogi_engine::{ShogiChoice, game::Shogi};
use mshogi_search::shogi as search;
use std::io;

impl Dialect for Shogi {
    type Root = GameRoot;
    type Action = Action;
    const PROTOCOL: &'static str = "usi";
    const GAME: &'static str = "shogi";
    const PROFILE: &'static str = PROFILE_ID;
    const DEFAULT_DEPTH: u16 = search::DEFAULT_LIMITS.depth;
    const DEFAULT_NODES: u64 = search::DEFAULT_LIMITS.nodes;
    const DEFAULT_HASH: usize = search::DEFAULT_TABLE_MB;
    const SCORE_DETAIL: &'static str = "";
    const USI: bool = true;
    fn start() -> Game {
        Game::new()
    }
    fn root(game: &Game) -> &GameRoot {
        game.root()
    }
    fn parse_root<'a>(words: &'a [&str]) -> Result<(GameRoot, &'a [&'a str]), String> {
        match words {
            ["startpos", rest @ ..] => Ok((GameRoot::standard(), rest)),
            ["sfen", a, b, c, d, rest @ ..] => {
                let position =
                    Position::from_sfen(&[*a, *b, *c, *d].join(" ")).map_err(|e| e.to_string())?;
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
                    other => other,
                }
                .map_err(|e| format!("native root: {e:?}"))?;
                Ok((root, rest))
            }
            _ => Err("position requires startpos or four-field sfen".into()),
        }
    }
    fn from_root(root: GameRoot) -> Result<Game, String> {
        Ok(Game::from_root(root))
    }
    fn black_clock(side: Side) -> bool {
        side == Side::Sente
    }
    fn play(action: Action) -> GameCommand {
        GameCommand::Play(action)
    }
    fn action(choice: &ShogiChoice) -> Option<Action> {
        match *choice {
            ShogiChoice::Move { action } => Some(action),
            _ => None,
        }
    }
    fn special_choice(game: &Game, choice: &ShogiChoice) -> io::Result<String> {
        if let ShogiChoice::Declare { actor } = *choice
            && game.conclusion()
                == Some(Conclusion::Win {
                    winner: actor,
                    reason: WinReason::EnteringKing,
                })
        {
            Ok("win".into())
        } else {
            Err(io::Error::other(
                "searched declaration did not establish native win",
            ))
        }
    }
    fn terminal_move(game: &Game, conclusion: Conclusion) -> Option<&'static str> {
        matches!(conclusion, Conclusion::Win { winner, reason: WinReason::NoLegalAction { in_check: true }} if winner != game.position().side_to_move()).then_some("resign")
    }
}
