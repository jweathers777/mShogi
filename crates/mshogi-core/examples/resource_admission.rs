//! Native admission/replay measurements; generated walks are workloads, not rules oracles.
#[path = "support/walk.rs"]
mod walk;
use std::{error::Error, hint::black_box, time::Instant};

type Failure = Box<dyn Error>;

#[allow(clippy::too_many_arguments)]
fn measure<G, C: Clone, U>(
    mut game: G,
    requested: usize,
    mut choices: impl FnMut(&G, usize) -> Vec<C>,
    mut submit: impl FnMut(&mut G, C) -> Result<U, Failure>,
    mut undo: impl FnMut(&mut G, &mut U) -> Result<(), Failure>,
    ended: impl Fn(&G) -> bool,
    encode: impl Fn(&G) -> Result<String, Failure>,
    replay: impl Fn(&str) -> Result<(), Failure>,
) -> Result<(), Failure> {
    let start = Instant::now();
    let (accepted, command) = walk::prefix(
        &mut game,
        requested,
        &mut choices,
        &mut submit,
        &mut undo,
        ended,
    )?;
    let build_us = start.elapsed().as_micros();
    let json = encode(&game)?;
    let mut admissions = Vec::new();
    let mut undos = Vec::new();
    let mut command_bytes = 0;
    for _ in 0..11 {
        let start = Instant::now();
        let mut token = submit(&mut game, command.clone())?;
        admissions.push(start.elapsed().as_nanos());
        command_bytes = encode(&game)?.len() - json.len();
        let start = Instant::now();
        undo(&mut game, &mut token)?;
        undos.push(start.elapsed().as_nanos());
        assert_eq!(
            encode(&game)?,
            json,
            "undo must preserve the complete record"
        );
    }
    let mut replays = Vec::new();
    for _ in 0..5 {
        let start = Instant::now();
        replay(black_box(&json))?;
        replays.push(start.elapsed().as_nanos());
    }
    admissions.sort();
    undos.sort();
    replays.sort();
    println!(
        "{accepted},{},{command_bytes},{build_us},{:.3},{:.3},{:.3}",
        json.len(),
        admissions[5] as f64 / 1000.0,
        undos[5] as f64 / 1000.0,
        replays[2] as f64 / 1000.0
    );
    Ok(())
}

macro_rules! control {
    (tori, $enabled:expr, $index:expr) => {
        None::<(Side, GameCommand)>
    };
    ($module:ident, $enabled:expr, $index:expr) => {
        $enabled.then(|| {
            if $index % 2 == 0 {
                (Side::Sente, GameCommand::OfferDraw)
            } else {
                (Side::Gote, GameCommand::DeclineDraw)
            }
        })
    };
}
macro_rules! run {
    ($module:ident, $game:expr, $count:expr, $controls:expr) => {{
        use mshogi_core::$module::*;
        measure(
            $game,
            $count,
            |game: &Game, _index| {
                if let Some(command) = control!($module, $controls, _index) {
                    vec![command]
                } else {
                    game.legal_actions()
                        .into_iter()
                        .map(|action| {
                            (game.position().side_to_move(), GameCommand::Play { action })
                        })
                        .collect()
                }
            },
            |game, (actor, command)| {
                Ok(game.submit(GameRequest {
                    expected: game.context(),
                    actor,
                    command,
                })?)
            },
            |game, undo| {
                game.undo(undo)?;
                Ok(())
            },
            |game| game.conclusion().is_some(),
            |game| Ok(GameRecord::from_game(game)?.to_json().to_owned()),
            |text| {
                black_box(GameRecord::from_json(text)?);
                Ok(())
            },
        )
    }};
}
fn main() -> Result<(), Failure> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let kind = args
        .first()
        .ok_or("game: tori|chu|tenjiku-adams|tenjiku-muller")?;
    let count = args.get(1).ok_or("history length")?.parse::<usize>()?;
    if count > 1000 {
        return Err("history length must be <=1000".into());
    }
    let controls = args.get(2).is_some_and(|s| s == "controls");
    println!("events,record_bytes,command_bytes,build_us,admission_us,undo_us,replay_us");
    match kind.as_str() {
        "tori" if controls => Err("Tori has no draw-offer controls".into()),
        "tori" => run!(tori, Game::standard(), count, false),
        "chu" => run!(chu, Game::standard(), count, controls),
        "tenjiku-adams" => run!(
            tenjiku,
            Game::standard(Profile::AdamsModern),
            count,
            controls
        ),
        "tenjiku-muller" => run!(
            tenjiku,
            Game::standard(Profile::MullerCompleted),
            count,
            controls
        ),
        _ => Err("unknown game".into()),
    }
}
