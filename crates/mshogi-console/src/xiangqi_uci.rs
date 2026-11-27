//! Pikafish UCI0-9 uses Xiangqi actions, with Red on White's clock.
use crate::protocol::Dialect;
use mshogi_core::xiangqi::*;
use mshogi_engine::{XiangqiChoice, game::Xiangqi};
use mshogi_search::xiangqi as search;
use std::io;

impl Dialect for Xiangqi {
    type Root = PositionRoot;
    type Action = Action;
    const PROTOCOL: &'static str = "uci";
    const GAME: &'static str = "xiangqi";
    const PROFILE: &'static str = PROFILE_ID;
    const DEFAULT_DEPTH: u16 = search::DEFAULT_LIMITS.depth;
    const DEFAULT_NODES: u64 = search::DEFAULT_LIMITS.nodes;
    const DEFAULT_HASH: usize = search::DEFAULT_TABLE_MB;
    const SCORE_DETAIL: &'static str = " units soldier_hundredths mode sound_fast_only";
    const USI: bool = false;
    fn start() -> Game {
        Game::standard(ClaimPolicy::AutoValidClaims).expect("standard game root")
    }
    fn root(game: &Game) -> &PositionRoot {
        game.root()
    }
    fn parse_root<'a>(words: &'a [&str]) -> Result<(PositionRoot, &'a [&'a str]), String> {
        match words {
            ["startpos", rest @ ..] => Ok((PositionRoot::standard(), rest)),
            ["fen", a, b, c, d, e, f, rest @ ..] => Ok((
                PositionRoot::from_fen(&[*a, *b, *c, *d, *e, *f].join(" "))
                    .map_err(|e| e.to_string())?,
                rest,
            )),
            _ => Err(
                "position requires startpos or six-field FEN (a-i, ranks 0-9, k/a/b/n/r/c/p)"
                    .into(),
            ),
        }
    }
    fn from_root(root: PositionRoot) -> Result<Game, String> {
        Game::from_root(root, ClaimPolicy::AutoValidClaims).map_err(|e| format!("game root: {e:?}"))
    }
    fn black_clock(side: Side) -> bool {
        side == Side::Black
    }
    fn play(action: Action) -> GameCommand {
        GameCommand::Play { action }
    }
    fn action(choice: &XiangqiChoice) -> Option<Action> {
        match *choice {
            XiangqiChoice::Move { action } => Some(action),
            _ => None,
        }
    }
    fn special_choice(_: &Game, _: &XiangqiChoice) -> io::Result<String> {
        Err(io::Error::other(
            "unsupported_adjudication: UCI0-9 has no claim channel; host must use auto_valid_claims",
        ))
    }
    fn terminal_move(_: &Game, _: Conclusion) -> Option<&'static str> {
        None
    }
}
