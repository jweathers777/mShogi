//! Native root import, admissible board moves and evaluation inputs.
use mshogi_core::{
    chess, chu,
    native::{CanonicalRecord, RulesProfile},
    shogi, tenjiku, tori, xiangqi,
};
use mshogi_runner::experiments::{fresh_root, standard_root};
use mshogi_search::root::RootGame;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

trait Board: Clone {
    type Move: Clone + Serialize + DeserializeOwned;
    fn legal(&self) -> Result<Vec<Self::Move>>;
    fn play(&mut self, action: Self::Move) -> Result<()>;
    fn record(&self) -> Result<CanonicalRecord>;
    fn diagram(&self) -> Result<Value>;
    fn first(&self) -> bool;
    fn score(&self) -> Option<u8>;
    // Other evaluators apply their own quiet-position filter when loading a fit.
    fn export_position(&self) -> Result<Option<Value>> {
        Ok(Some(self.diagram()?))
    }
    fn parse(&self, value: &Value) -> Result<Self::Move> {
        Ok(serde_json::from_value(value.clone())?)
    }
}

impl Board for chess::AdjudicatedGame {
    type Move = chess::Action;
    fn legal(&self) -> Result<Vec<Self::Move>> {
        Ok(self.position().legal_actions())
    }
    fn play(&mut self, action: Self::Move) -> Result<()> {
        Ok(self.apply(action)?)
    }
    fn record(&self) -> Result<CanonicalRecord> {
        Ok(CanonicalRecord::chess(self.record()))
    }
    fn diagram(&self) -> Result<Value> {
        Ok(json!(self.position().to_fen()))
    }
    fn export_position(&self) -> Result<Option<Value>> {
        let p = self.position();
        let quiet = !p.in_check()
            && !p.legal_actions().iter().any(|m| {
                m.promotion.is_some()
                    || p.piece_at(m.to).is_some()
                    || (p
                        .piece_at(m.from)
                        .is_some_and(|piece| piece.kind == chess::PieceKind::Pawn)
                        && m.from.file() != m.to.file())
            })
            && mshogi_search::chess_eval::decompose(
                &mshogi_search::chess_bitboard::ChessPosition::from_position(p),
            )
            .specialized
            .is_none();
        if quiet {
            Ok(Some(self.diagram()?))
        } else {
            Ok(None)
        }
    }
    fn first(&self) -> bool {
        self.position().side_to_move() == chess::Color::White
    }
    fn score(&self) -> Option<u8> {
        self.outcome().map(|o| match o {
            chess::ChessOutcome::Checkmate {
                winner: chess::Color::White,
            } => 2,
            chess::ChessOutcome::Checkmate { .. } => 0,
            chess::ChessOutcome::Draw(_) => 1,
        })
    }
}

macro_rules! board {
    ($mod:ident, $first:ident, $legal:expr, $command:expr, $record:expr, $diagram:expr $(,$parse:item)?) => {
        impl Board for $mod::Game {
            type Move = $mod::Action;
            fn legal(&self) -> Result<Vec<Self::Move>> { ($legal)(self) }
            fn play(&mut self, action: Self::Move) -> Result<()> {
                self.submit($mod::GameRequest { expected: self.context(),
                    actor: self.position().side_to_move(), command: ($command)(action) }).map_err(|e| format!("{e:?}"))?;
                Ok(())
            }
            fn record(&self) -> Result<CanonicalRecord> {
                Ok(CanonicalRecord::$mod(($record)(self)?))
            }
            fn diagram(&self) -> Result<Value> { ($diagram)(self) }
            fn first(&self) -> bool { self.position().side_to_move() == $mod::Side::$first }
            fn score(&self) -> Option<u8> {
                self.conclusion().map(|c| Self::winner(c).map_or(1, |s| if s == $mod::Side::$first { 2 } else { 0 }))
            }
            $($parse)?
        }
    }
}
board!(
    shogi,
    Sente,
    |g: &shogi::Game| Ok(g.position().legal_actions()),
    shogi::GameCommand::Play,
    |g| Result::Ok(shogi::GameRecord::from_game(g)),
    |g: &shogi::Game| Ok(json!(g.position().to_sfen()))
);
board!(
    xiangqi,
    Red,
    |g: &xiangqi::Game| Ok(g.position().legal_actions()?),
    |action| xiangqi::GameCommand::Play { action },
    xiangqi::GameRecord::from_game,
    |g: &xiangqi::Game| Ok(json!(g.position().to_fen()?))
);
board!(
    tori,
    Sente,
    |g: &tori::Game| Ok(g.legal_actions()),
    |action| tori::GameCommand::Play { action },
    tori::GameRecord::from_game,
    |g: &tori::Game| Ok(g.position().compact_diagram()),
    fn parse(&self, value: &Value) -> Result<Self::Move> {
        match value.as_str() {
            Some(text) => Ok(tori::notation::parse_action(self.position(), text)?),
            None => Ok(serde_json::from_value(value.clone())?),
        }
    }
);
board!(
    chu,
    Sente,
    |g: &chu::Game| Ok(g.legal_actions()),
    |action| chu::GameCommand::Play { action },
    chu::GameRecord::from_game,
    |g: &chu::Game| Ok(json!({"pieces":g.position().pieces().collect::<Vec<_>>(),
        "side":g.position().side_to_move(),"protection":g.local_position().protection()}))
);
board!(
    tenjiku,
    Sente,
    |g: &tenjiku::Game| Ok(g.legal_actions()),
    |action| tenjiku::GameCommand::Play { action },
    tenjiku::GameRecord::from_game,
    |g: &tenjiku::Game| Ok(json!({"pieces":g.position().pieces().collect::<Vec<_>>(),
        "side":g.position().side_to_move(),"profile":g.profile(),"pending_bare":g.snapshot().pending_bare}))
);

fn import(profile: RulesProfile, row: &Value) -> Result<CanonicalRecord> {
    if let Some(root) = row.get("record") {
        let record = if root.get("game_kind").is_some() {
            serde_json::from_value(root.clone())?
        } else if profile == RulesProfile::ChuJcsa2019V1 {
            CanonicalRecord::chu(serde_json::from_value(root.clone())?)
        } else {
            return Err("expected a canonical record envelope".into());
        };
        if record.profile() != profile {
            return Err("record has another profile".into());
        }
        return Ok(record);
    }
    let record = match profile {
        RulesProfile::ChessMatchV1 if row["fen"].is_string() => CanonicalRecord::chess(
            chess::AdjudicatedGame::from_fen(
                row["fen"].as_str().unwrap(),
                chess::ClaimPolicy::AutoClaim,
            )?
            .record(),
        ),
        RulesProfile::ShogiCsa36V1 if row["sfen"].is_string() => {
            let pos = shogi::Position::from_sfen(row["sfen"].as_str().unwrap())?;
            let root = shogi::GameRoot::import(
                pos.clone(),
                shogi::RootCategory::FullMaterialDiagram,
                shogi::CounterSource::Supplied,
            )
            .or_else(|_| {
                shogi::GameRoot::import(
                    pos,
                    shogi::RootCategory::ReducedMaterialDiagram,
                    shogi::CounterSource::Supplied,
                )
            })
            .map_err(|e| format!("{e:?}"))?;
            CanonicalRecord::shogi(shogi::GameRecord::from_game(&shogi::Game::from_root(root)))
        }
        RulesProfile::XiangqiWxf2018V1 if row["fen"].is_string() => {
            CanonicalRecord::xiangqi(xiangqi::GameRecord::from_game(&xiangqi::Game::from_root(
                xiangqi::PositionRoot::from_fen(row["fen"].as_str().unwrap())?,
                xiangqi::ClaimPolicy::AutoValidClaims,
            )?)?)
        }
        RulesProfile::ToriModernV1 if row.get("diagram").is_some() => {
            CanonicalRecord::tori(tori::GameRecord::from_game(&tori::Game::from_root(
                tori::Position::from_compact_diagram(&row["diagram"])?,
            )?)?)
        }
        _ if ["fen", "sfen", "diagram"]
            .iter()
            .any(|field| !row[field].is_null()) =>
        {
            return Err("diagram does not match profile".into());
        }
        _ => standard_root(profile)?.record,
    };
    Ok(record)
}

macro_rules! with_board {
    ($record:expr, |$g:ident| $body:expr) => {{
        macro_rules! apply {
            ($game:expr) => {{
                let $g = $game;
                $body
            }};
        }
        match $record {
            CanonicalRecord::Chess(r) => apply!(chess::AdjudicatedGame::replay_record(r)?),
            CanonicalRecord::Shogi(r) => apply!(r.to_game()?),
            CanonicalRecord::Xiangqi(r) => apply!(r.to_game()?),
            CanonicalRecord::Tori(r) => apply!(r.to_game()?),
            CanonicalRecord::Chu(r) => apply!(r.to_game()?),
            CanonicalRecord::Tenjiku(r) => apply!(r.to_game()?),
        }
    }};
}

pub fn prepare(
    profile: RulesProfile,
    row: &Value,
    seed: u64,
    plies: usize,
    prove: bool,
) -> Result<Value> {
    if plies > 128 {
        return Err("at most 128 opening plies".into());
    }
    let record = import(profile, row)?;
    with_board!(&record, |g| prepare_board(g, row, seed, plies, prove))
}

pub fn export_position(record: &CanonicalRecord) -> Result<Option<Value>> {
    with_board!(record, |g| g.export_position())
}

fn prepare_board<G: Board>(
    mut game: G,
    row: &Value,
    seed: u64,
    plies: usize,
    prove: bool,
) -> Result<Value> {
    if let Some(moves) = row.get("moves") {
        for m in moves.as_array().ok_or("expected a move list")? {
            game.play(game.parse(m)?)?;
        }
    }
    let mut state = seed.wrapping_mul(0x9e3779b97f4a7c15) | 1;
    for _ in 0..plies {
        let legal = game.legal()?;
        if legal.is_empty() || game.score().is_some() {
            return Ok(json!({"stopped":true}));
        }
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        game.play(legal[(state % legal.len() as u64) as usize].clone())?;
    }
    let mut answers = Vec::new();
    if prove && game.score().is_none() {
        let win = if game.first() { 2 } else { 0 };
        for action in game.legal()? {
            let mut next = game.clone();
            next.play(action.clone())?;
            if next.score() == Some(win) {
                answers.push(action);
            }
        }
    } else if let Some(expected) = row.get("answers").or_else(|| row.get("expected_moves")) {
        for v in expected.as_array().ok_or("expected an answer list")? {
            let action = game.parse(v)?;
            game.clone().play(action.clone())?;
            answers.push(action);
        }
    }
    Ok(
        json!({"root":fresh_root(game.record()?)?, "position":game.diagram()?,
        "first":game.first(),"score_first_twice":game.score(),"answers":answers}),
    )
}
