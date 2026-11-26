//! One request token for the native games that admit context-bound commands.
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_INSTANCE: AtomicU64 = AtomicU64::new(1);

/// Runtime request token of one native game instance; never saved in a record.
///
/// The instance is new for each game, copy, fork and replay. The revision
/// advances at each accepted command, undo and retirement, and never goes back.
/// A request is admitted only with the current token. Thus a request made
/// before any of these changes, or on another instance, is refused, also when
/// that instance has the same history.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub struct GameContext {
    instance: u64,
    revision: u64,
}
impl GameContext {
    /// Token of a new game instance. A 64-bit instance count cannot run out.
    pub(crate) fn fresh() -> Self {
        Self {
            instance: NEXT_INSTANCE.fetch_add(1, Ordering::Relaxed),
            revision: 0,
        }
    }
    /// Advance after a command, undo or retirement. A native runtime cannot
    /// perform 2^64 operations in the lifetime of the process.
    pub(crate) fn advance(&mut self) {
        self.revision += 1;
    }
    /// Identity of the game instance, which binds its undo tokens.
    pub(crate) fn instance(self) -> u64 {
        self.instance
    }
    /// Current revision; unique in its instance, so it can name a history node.
    /// A copy starts its node at 0, which no command revision can be.
    pub(crate) fn revision(self) -> u64 {
        self.revision
    }
}
