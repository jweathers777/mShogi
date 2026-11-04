//! Bounded full-Game diagnostics: legal complete board actions with real history/endings.
use super::*;
use std::{error::Error, fmt};

/// Deliberately small full-game diagnostic horizon; not a search limit.
pub const MAX_PERFT_DEPTH: u8 = 3;
/// Absolute visited-node ceiling, further bounded by the caller.
pub const MAX_PERFT_VISITS: u64 = 100_000;
/// Exact result only when every requested branch completed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PerftReport {
    /// Requested complete-board-action depth.
    pub depth: u8,
    /// Leaves at that exact depth; depth0 includes the root.
    pub leaves: u64,
    /// All visited nodes, including root.
    pub visited: u64,
}
/// One actual complete root action and its exact subtree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DivideEntry {
    /// Native action, retaining ordered path and promotion.
    pub action: Action,
    /// Exact leaves below it.
    pub leaves: u64,
}
/// Exact deterministic root divide, with one shared visit budget.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DivideReport {
    /// Exact whole traversal.
    pub total: PerftReport,
    /// Native canonical legal-action order.
    pub entries: Vec<DivideEntry>,
}
/// Explicit incomplete/refused traversal; partial counts never masquerade as exact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PerftError {
    /// Unsupported caller limits, without traversal.
    Limits,
    /// Caller stopped work at this complete-action prefix.
    Stopped {
        /// Complete native action prefix.
        path: Vec<Action>,
    },
    /// Global visit allowance exhausted at the next unvisited prefix.
    WorkLimit {
        /// Complete native action prefix.
        path: Vec<Action>,
    },
    /// Native semantic reconstruction refused the private working root.
    Record(RecordError),
    /// An enumerated native action could not be committed.
    Action {
        /// Complete native prefix.
        path: Vec<Action>,
        /// Actual native refusal.
        error: GameError,
    },
    /// Native reversal refused a private diagnostic undo.
    Reversal {
        /// Complete native prefix.
        path: Vec<Action>,
    },
    /// Checked arithmetic could not represent the exact count.
    Capacity {
        /// Complete native prefix.
        path: Vec<Action>,
    },
}
impl fmt::Display for PerftError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tenjiku full-game perft incomplete: {self:?}")
    }
}
impl Error for PerftError {}
struct Budget {
    used: u64,
    limit: u64,
}
impl Budget {
    fn visit(
        &mut self,
        path: &[Action],
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), PerftError> {
        check(stop, path)?;
        if self.used >= self.limit {
            return Err(PerftError::WorkLimit {
                path: path.to_vec(),
            });
        }
        self.used += 1;
        Ok(())
    }
}
fn check(stop: &mut impl FnMut() -> bool, path: &[Action]) -> Result<(), PerftError> {
    if stop() {
        Err(PerftError::Stopped {
            path: path.to_vec(),
        })
    } else {
        Ok(())
    }
}
fn setup(
    game: &Game,
    depth: u8,
    visits: u64,
    stop: &mut impl FnMut() -> bool,
) -> Result<(Game, Budget), PerftError> {
    if depth > MAX_PERFT_DEPTH || visits == 0 || visits > MAX_PERFT_VISITS {
        return Err(PerftError::Limits);
    }
    check(stop, &[])?;
    let record = GameRecord::from_game(game).map_err(PerftError::Record)?;
    let copy = record.to_game_with_stop(stop).map_err(|error| {
        if error == RecordError::Stopped {
            PerftError::Stopped { path: vec![] }
        } else {
            PerftError::Record(error)
        }
    })?;
    Ok((
        copy,
        Budget {
            used: 0,
            limit: visits,
        },
    ))
}
fn walk(
    game: &mut Game,
    depth: u8,
    budget: &mut Budget,
    path: &mut Vec<Action>,
    stop: &mut impl FnMut() -> bool,
) -> Result<u64, PerftError> {
    budget.visit(path, stop)?;
    if depth == 0 {
        return Ok(1);
    }
    let actions = game
        .legal_actions_with_stop(stop)
        .map_err(|error| match error {
            GameError::Stopped => PerftError::Stopped {
                path: path.to_vec(),
            },
            error => PerftError::Action {
                path: path.to_vec(),
                error,
            },
        })?;
    check(stop, path)?;
    let mut leaves = 0u64;
    for action in actions {
        path.push(action.clone());
        check(stop, path)?;
        let mut undo = game
            .submit_with_stop(
                GameRequest {
                    expected: game.context(),
                    actor: game.position().side_to_move(),
                    command: GameCommand::Play { action },
                },
                stop,
            )
            .map_err(|error| match error {
                GameError::Stopped => PerftError::Stopped { path: path.clone() },
                error => PerftError::Action {
                    path: path.clone(),
                    error,
                },
            })?;
        let result = walk(game, depth - 1, budget, path, stop);
        game.undo(&mut undo)
            .map_err(|_| PerftError::Reversal { path: path.clone() })?;
        let count = result?;
        path.pop();
        leaves = leaves
            .checked_add(count)
            .ok_or_else(|| PerftError::Capacity { path: path.clone() })?;
    }
    Ok(leaves)
}
impl Game {
    /// Exact full-history count; the original game and runtime authority are untouched.
    pub fn perft(&self, depth: u8, max_visits: u64) -> Result<PerftReport, PerftError> {
        self.perft_with_stop(depth, max_visits, &mut || false)
    }
    /// Full-game count with explicit cooperative cancellation between native calls.
    pub fn perft_with_stop(
        &self,
        depth: u8,
        max_visits: u64,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<PerftReport, PerftError> {
        let (mut game, mut budget) = setup(self, depth, max_visits, stop)?;
        let leaves = walk(&mut game, depth, &mut budget, &mut vec![], stop)?;
        Ok(PerftReport {
            depth,
            leaves,
            visited: budget.used,
        })
    }
    /// Exact deterministic divide sharing one visit budget across all root actions.
    pub fn divide(&self, depth: u8, max_visits: u64) -> Result<DivideReport, PerftError> {
        self.divide_with_stop(depth, max_visits, &mut || false)
    }
    /// Cancellable full-game divide; no incomplete count is returned as exact.
    pub fn divide_with_stop(
        &self,
        depth: u8,
        max_visits: u64,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<DivideReport, PerftError> {
        if depth == 0 {
            return Err(PerftError::Limits);
        }
        let (mut game, mut budget) = setup(self, depth, max_visits, stop)?;
        budget.visit(&[], stop)?;
        let actions = game
            .legal_actions_with_stop(stop)
            .map_err(|error| match error {
                GameError::Stopped => PerftError::Stopped { path: vec![] },
                error => PerftError::Action {
                    path: vec![],
                    error,
                },
            })?;
        check(stop, &[])?;
        let mut entries = Vec::new();
        let mut leaves = 0u64;
        for action in actions {
            let mut path = vec![action.clone()];
            check(stop, &path)?;
            let mut undo = game
                .submit_with_stop(
                    GameRequest {
                        expected: game.context(),
                        actor: game.position().side_to_move(),
                        command: GameCommand::Play {
                            action: action.clone(),
                        },
                    },
                    stop,
                )
                .map_err(|error| match error {
                    GameError::Stopped => PerftError::Stopped { path: path.clone() },
                    error => PerftError::Action {
                        path: path.clone(),
                        error,
                    },
                })?;
            let result = walk(&mut game, depth - 1, &mut budget, &mut path, stop);
            game.undo(&mut undo)
                .map_err(|_| PerftError::Reversal { path: path.clone() })?;
            let count = result?;
            leaves = leaves
                .checked_add(count)
                .ok_or_else(|| PerftError::Capacity { path: path.clone() })?;
            entries.push(DivideEntry {
                action,
                leaves: count,
            });
        }
        Ok(DivideReport {
            total: PerftReport {
                depth,
                leaves,
                visited: budget.used,
            },
            entries,
        })
    }
}

#[cfg(test)]
#[path = "perft_tests.rs"]
mod tests;
