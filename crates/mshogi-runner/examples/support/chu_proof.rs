//! Full-width, finite native win proof, independent of alpha-beta and evaluation.
//! A cutoff is unknown, never a losing proof. Unresolved no-action is not a win.
use mshogi_core::chu::*;

pub fn play(game: &mut Game, action: Action) {
    game.submit(GameRequest {
        expected: game.context(),
        actor: game.position().side_to_move(),
        command: GameCommand::Play { action },
    })
    .unwrap();
}

pub fn wins(game: &Game, attacker: Side, plies: u8, visits: &mut u64) -> Option<bool> {
    if *visits == 0 {
        return None;
    }
    *visits -= 1;
    if let Some(c) = game.conclusion() {
        return Some(c.winner() == Some(attacker));
    }
    if plies == 0 {
        return Some(false);
    }
    let actions = game.legal_actions();
    if actions.is_empty() {
        return Some(false);
    }
    let attacking = game.position().side_to_move() == attacker;
    let mut unknown = false;
    for action in actions {
        let mut child = game.fork().unwrap();
        play(&mut child, action);
        match wins(&child, attacker, plies - 1, visits) {
            Some(value) if value == attacking => return Some(attacking),
            None => unknown = true,
            _ => (),
        }
    }
    if unknown { None } else { Some(!attacking) }
}

/// Enumerate every winning first action, or return unknown for the whole set.
pub fn answers(game: &Game, plies: u8, cap: u64) -> Option<Vec<Action>> {
    assert!(plies > 0);
    let attacker = game.position().side_to_move();
    let mut visits = cap;
    let mut result = vec![];
    for action in game.legal_actions() {
        let mut child = game.fork().unwrap();
        play(&mut child, action.clone());
        if wins(&child, attacker, plies - 1, &mut visits)? {
            result.push(action);
        }
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bound_does_not_invent_a_result() {
        let game = Game::standard();
        assert_eq!(wins(&game, Side::Sente, 1, &mut 0), None);
        assert_eq!(wins(&game, Side::Sente, 0, &mut 1), Some(false));
        assert_eq!(answers(&game, 1, 0), None);
    }
}
