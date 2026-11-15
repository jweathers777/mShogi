//! Shared participant ownership. Native games still admit every complete command.
use serde::{Deserialize, Serialize};

/// Durable local desktop participant arrangement.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlayMode {
    /// One assigned human and an automatic engine opponent.
    #[default]
    HumanEngine,
    /// Both native sides are controlled locally by humans.
    TwoHumans,
}
impl PlayMode {
    /// The local actor for ordinary turn-based controls.
    pub fn actor<S: Copy>(self, human: S, turn: S) -> S {
        if self.is_two_humans() { turn } else { human }
    }
    /// Whether the side belongs to a local human.
    pub fn owns<S: PartialEq>(self, human: S, side: S) -> bool {
        self.is_two_humans() || human == side
    }
    /// Whether the engine owns the side to move.
    pub fn engine_turn<S: PartialEq>(self, human: S, turn: S) -> bool {
        !self.owns(human, turn)
    }
    /// Whether both players are local humans.
    pub fn is_two_humans(self) -> bool {
        matches!(self, Self::TwoHumans)
    }
}
