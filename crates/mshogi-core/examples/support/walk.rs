//! Deterministic nonterminal native histories for measurements, not rules oracles.
use std::error::Error;
pub type Failure = Box<dyn Error>;

pub fn prefix<G, C: Clone, U>(
    game: &mut G,
    requested: usize,
    mut choices: impl FnMut(&G, usize) -> Vec<C>,
    mut submit: impl FnMut(&mut G, C) -> Result<U, Failure>,
    mut undo: impl FnMut(&mut G, &mut U) -> Result<(), Failure>,
    ended: impl Fn(&G) -> bool,
) -> Result<(usize, C), Failure> {
    let mut accepted = 0;
    let mut next = None;
    for index in 0..=requested {
        let mut candidates = choices(game, index);
        if !candidates.is_empty() {
            let offset = index.wrapping_mul(7919).wrapping_add(17) % candidates.len();
            candidates.rotate_left(offset);
        }
        let mut found = false;
        for command in candidates {
            let Ok(mut token) = submit(game, command.clone()) else {
                continue;
            };
            if ended(game) || index == requested {
                let terminal = ended(game);
                undo(game, &mut token)?;
                if terminal {
                    continue;
                }
                next = Some(command);
            } else {
                accepted += 1;
            }
            found = true;
            break;
        }
        if !found || next.is_some() {
            break;
        }
    }
    Ok((
        accepted,
        next.ok_or("workload has no further nonterminal command")?,
    ))
}
