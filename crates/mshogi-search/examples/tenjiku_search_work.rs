//! Full authoritative search timing, distinct from playing strength or a latency promise.
use mshogi_core::tenjiku::*;
use mshogi_search::tenjiku::{DEFAULT_LIMITS, Limits, search};
use std::{error::Error, time::Instant};
fn tactical(profile: Profile) -> Result<Game, Box<dyn Error>> {
    let pieces = [
        (16, 16, Side::Sente, Base::K),
        (16, 14, Side::Sente, Base::P),
        (8, 11, Side::Sente, Base::FiD),
        (8, 7, Side::Gote, Base::K),
        (9, 8, Side::Gote, Base::FiD),
        (1, 3, Side::Gote, Base::P),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (f, r, side, base))| {
        Ok((
            Square::new(f, r).unwrap(),
            Piece::new(id as u16, side, base, false)?,
        ))
    })
    .collect::<Result<Vec<_>, RootError>>()?;
    Ok(Game::from_root(Position::from_diagram(
        profile,
        &pieces,
        Side::Sente,
        RootMetadata::diagram(0),
    )?)?)
}
fn wide(profile: Profile) -> Result<Game, Box<dyn Error>> {
    let entries = [
        (1, 16, Side::Sente, Base::K),
        (1, 14, Side::Sente, Base::P),
        (8, 8, Side::Sente, Base::FiD),
        (11, 11, Side::Sente, Base::LHk),
        (16, 1, Side::Gote, Base::K),
        (16, 3, Side::Gote, Base::P),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (f, r, side, base))| {
        Ok((
            Square::new(f, r).unwrap(),
            Piece::new(id as u16, side, base, false)?,
        ))
    })
    .collect::<Result<Vec<_>, RootError>>()?;
    Ok(Game::from_root(Position::from_diagram(
        profile,
        &entries,
        Side::Sente,
        RootMetadata::diagram(0),
    )?)?)
}
fn retained(profile: Profile) -> Result<Game, Box<dyn Error>> {
    // Same reproducible 156-piece quiet workload as the accepted native example.
    // Native Game still owns repetition and complete transition admission.
    let mut game = Game::standard(profile);
    let mut seed = 0x87_2026u64;
    for _ in 0..128 {
        let mut choices = game.local_position().legal_actions();
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        choices.sort_by_key(|a| {
            let p = game.position().piece(a.from).unwrap();
            let to = a.to().unwrap();
            let forward = match p.side {
                Side::Sente => to.rank() < a.from.rank(),
                Side::Gote => to.rank() > a.from.rank(),
            };
            let mix =
                (u64::from(a.from.index() as u16) * 257 + u64::from(to.index() as u16)) ^ seed;
            (
                !forward,
                a.promote,
                p.is_royal(),
                p.base != Base::P,
                mix,
                a.clone(),
            )
        });
        let mut accepted = false;
        for action in choices {
            if action.promote {
                continue;
            }
            let (_, e) = game.local_position().simulate(&action)?;
            if e.mover_after.is_none() || e.captures().next().is_some() {
                continue;
            }
            match game.submit(GameRequest {
                expected: game.context(),
                actor: e.actor,
                command: GameCommand::Play { action },
            }) {
                Ok(_) => {
                    accepted = true;
                    break;
                }
                Err(GameError::Repetition) => continue,
                Err(e) => return Err(e.into()),
            }
        }
        if !accepted {
            return Err("128-ply representative construction could not continue".into());
        }
    }
    assert_eq!(game.position().pieces().count(), 156);
    Ok(game)
}
fn main() -> Result<(), Box<dyn Error>> {
    let depth = std::env::args()
        .nth(1)
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(DEFAULT_LIMITS.depth);
    let nodes = std::env::args()
        .nth(2)
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(DEFAULT_LIMITS.nodes);
    let extended = std::env::args().nth(3).is_some_and(|s| s == "extended");
    for profile in Profile::ALL {
        let roots = if extended {
            vec![
                ("area_hawk", wide(profile)?),
                ("retained128", retained(profile)?),
            ]
        } else {
            vec![
                ("standard", Game::standard(profile)),
                ("demon_tactical", tactical(profile)?),
            ]
        };
        for (name, game) in roots {
            let before = GameRecord::from_game(&game)?;
            let context = game.context();
            let at = Instant::now();
            let actions = game.legal_actions().len();
            let generation_us = at.elapsed().as_micros();
            let at = Instant::now();
            let result = search(&game, Limits { depth, nodes });
            let elapsed_us = at.elapsed().as_micros();
            assert_eq!(GameRecord::from_game(&game)?, before);
            assert_eq!(game.context(), context);
            println!(
                "profile={} root={name} actions={actions} generation_us={generation_us} depth={depth} limit={nodes} elapsed_us={elapsed_us} result={result:?}",
                profile.id()
            );
        }
    }
    Ok(())
}
