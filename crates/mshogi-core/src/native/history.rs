//! Append-only native history and the runtime authority for changes to it.
use super::GameContext;
use crate::repetition::RepetitionIndex;
use std::{fmt::Debug, hash::Hash};

/// Runtime binding of a single accepted command. Replaying a branch gives
/// its commands new revisions, so the same binding can never be used twice.
#[derive(Debug)]
pub struct GameUndo {
    after: GameContext,
    previous_node: u64,
}

#[derive(Debug)]
pub(crate) struct Authority {
    context: GameContext,
    node: u64,
}
impl Default for Authority {
    fn default() -> Self {
        Self {
            context: GameContext::fresh(),
            node: 0,
        }
    }
}
/// A copied game always starts a distinct runtime with the same logical history.
impl Clone for Authority {
    fn clone(&self) -> Self {
        Self::default()
    }
}
impl Authority {
    pub(crate) fn context(&self) -> GameContext {
        self.context
    }
    pub(crate) fn retire(&mut self) {
        self.context.advance();
    }
    fn commit(&mut self) -> GameUndo {
        self.retire();
        let previous_node = std::mem::replace(&mut self.node, self.context.revision());
        GameUndo {
            after: self.context,
            previous_node,
        }
    }
    pub(crate) fn undo(&mut self, token: &GameUndo) -> bool {
        if self.context.instance() != token.after.instance() || self.node != token.after.revision()
        {
            return false;
        }
        self.retire();
        self.node = token.previous_node;
        true
    }
}

/// Only the current mutable state and append cursors are needed to abort a command.
pub(crate) trait AppendGame: Sized {
    type Checkpoint;
    fn checkpoint(&mut self) -> Self::Checkpoint;
    fn restore(&mut self, before: Self::Checkpoint);
    fn authority(&mut self) -> &mut Authority;
    fn append_transaction<T, E>(
        &mut self,
        step: impl FnOnce(&mut Self) -> Result<T, E>,
    ) -> Result<(T, Self::Checkpoint, GameUndo), E> {
        let before = self.checkpoint();
        match step(self) {
            Ok(result) => Ok((result, before, self.authority().commit())),
            Err(error) => {
                self.restore(before);
                Err(error)
            }
        }
    }
}

/// A participant command bound to the live game that offered the choice.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Request<A, C> {
    /// Request token of the game when the command was chosen.
    pub expected: GameContext,
    /// Native participant submitting this command.
    pub actor: A,
    /// Complete native command.
    pub command: C,
}

/// Rules-owned data at a committed prefix. The owner keeps one snapshot per
/// event and the root. Each snapshot can restore its exact local position.
pub trait Snapshot: Clone + Debug {
    /// Admitted, mutable local board and rights.
    type Local: Clone + Debug;
    /// Board identity used by the repetition index.
    type Key: Clone + Eq + Hash + Debug;
    /// Additional per-board-action facts needed by the rules.
    type Ply: Clone + Debug;
    /// Committed native event.
    type Event: Clone + Debug;
    /// Native participant.
    type Side;
    /// Complete native command.
    type Command;
    /// A request came from a different or retired context.
    const STALE: Self::Error;
    /// Native root exceeds the compact root bound.
    const LARGE_ROOT: Self::Error;
    /// Apply the rules and append the resulting event and snapshot.
    fn step(
        game: &mut GameHistory<Self>,
        actor: Self::Side,
        command: Self::Command,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<(), Self::Error>;
    /// Native refusal.
    type Error;
    /// Report an invalid reversal.
    const INVALID_UNDO: Self::Error;
    /// Report an unknown prefix.
    const INVALID_PREFIX: Self::Error;
    /// Board-action count at this prefix.
    fn board_ply(&self) -> usize;
    /// Restore the admitted local state.
    fn restore_local(&self) -> Self::Local;
    /// Size of the native record in bytes, computed only on request.
    fn encoded_size(game: &GameHistory<Self>) -> Result<usize, Self::Error>;
}

/// Exact size and event count of a complete record.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CapacityUsage {
    /// Encoded native record bytes.
    pub encoded_bytes: usize,
    /// Committed native events.
    pub events: usize,
}

/// Authoritative game with one event and snapshot per committed command.
#[derive(Clone, Debug)]
pub struct GameHistory<S: Snapshot> {
    pub(crate) local: S::Local,
    pub(crate) keys: RepetitionIndex<S::Key>,
    pub(crate) plies: Vec<S::Ply>,
    pub(crate) events: Vec<S::Event>,
    pub(crate) snapshots: Vec<S>,
    pub(crate) root: S,
    authority: Authority,
}
impl<S: Snapshot> GameHistory<S> {
    pub(crate) fn from_snapshot(
        local: S::Local,
        keys: RepetitionIndex<S::Key>,
        root_snapshot: S,
    ) -> Self {
        Self {
            local,
            keys,
            root: root_snapshot,
            plies: vec![],
            events: vec![],
            snapshots: vec![],
            authority: Authority::default(),
        }
    }
    /// The admitted local position at the live tip.
    pub fn local_position(&self) -> &S::Local {
        &self.local
    }
    /// Accepted native events in order.
    pub fn events(&self) -> &[S::Event] {
        &self.events
    }
    /// Snapshot at the root, including any immediate conclusion.
    pub fn root_snapshot(&self) -> &S {
        &self.root
    }
    /// Snapshot at the current committed prefix.
    pub fn snapshot(&self) -> &S {
        self.snapshots.last().unwrap_or(&self.root)
    }
    /// Current repetition identity.
    pub fn repetition_key(&self) -> &S::Key {
        self.keys.current()
    }
    /// Token required for the next command.
    pub fn context(&self) -> GameContext {
        self.authority.context()
    }
    /// Return a fresh, independent game at the same prefix.
    pub fn fork(&self) -> Self {
        self.clone()
    }
    /// Exact immutable view at an event boundary.
    pub fn event_prefix(&self, count: usize) -> Result<S, S::Error> {
        match count.checked_sub(1) {
            None => Ok(self.root.clone()),
            Some(index) => self.snapshots.get(index).cloned().ok_or(S::INVALID_PREFIX),
        }
    }
    /// Select a historical view and retire outstanding requests.
    pub fn select_prefix(&mut self, count: usize) -> Result<S, S::Error> {
        let prefix = self.event_prefix(count)?;
        self.authority.retire();
        Ok(prefix)
    }
    /// Invalidate outstanding requests while preserving the committed prefix.
    pub fn retire_context(&mut self) -> Result<(), S::Error> {
        self.authority.retire();
        Ok(())
    }
    /// Compute exact native bytes only for an explicit diagnostic.
    pub fn capacity_usage(&self) -> Result<CapacityUsage, S::Error> {
        Ok(CapacityUsage {
            encoded_bytes: S::encoded_size(self)?,
            events: self.events.len(),
        })
    }
    pub(crate) fn check_root_size(&self) -> Result<(), S::Error> {
        if S::encoded_size(self)? > crate::record::MAX_COMPACT_ROOT_BYTES {
            return Err(S::LARGE_ROOT);
        }
        Ok(())
    }
    /// Submit a complete participant command against the expected live context.
    pub fn submit(&mut self, request: Request<S::Side, S::Command>) -> Result<GameUndo, S::Error> {
        self.admit(request, &mut || false)
    }
    pub(crate) fn admit(
        &mut self,
        request: Request<S::Side, S::Command>,
        stop: &mut impl FnMut() -> bool,
    ) -> Result<GameUndo, S::Error> {
        if request.expected != self.context() {
            return Err(S::STALE);
        }
        self.append_transaction(|game| S::step(game, request.actor, request.command, stop))
            .map(|(_, _, undo)| undo)
    }
    pub(crate) fn truncate_to(&mut self, count: usize) {
        self.events.truncate(count);
        self.snapshots.truncate(count);
        let snapshot = self.snapshot();
        let ply = snapshot.board_ply();
        self.local = snapshot.restore_local();
        self.plies.truncate(ply);
        self.keys.truncate(ply + 1);
    }
    /// Restore the previous committed prefix. Only the latest live command
    /// of the issuing instance can be reversed.
    pub fn undo(&mut self, token: &mut GameUndo) -> Result<(), S::Error> {
        if !self.authority.undo(token) {
            return Err(S::INVALID_UNDO);
        }
        self.truncate_to(self.events.len() - 1);
        Ok(())
    }
}

pub(crate) struct Checkpoint<L> {
    local: L,
    events: usize,
    snapshots: usize,
    plies: usize,
    keys: usize,
}
impl<S: Snapshot> AppendGame for GameHistory<S> {
    type Checkpoint = Checkpoint<S::Local>;
    fn authority(&mut self) -> &mut Authority {
        &mut self.authority
    }
    fn checkpoint(&mut self) -> Self::Checkpoint {
        // Local clones start with fresh local undo bindings. Save the original
        // owner so even those private bindings survive refusal or speculation.
        let working = self.local.clone();
        Checkpoint {
            local: std::mem::replace(&mut self.local, working),
            events: self.events.len(),
            snapshots: self.snapshots.len(),
            plies: self.plies.len(),
            keys: self.keys.len(),
        }
    }
    fn restore(&mut self, before: Self::Checkpoint) {
        self.local = before.local;
        self.events.truncate(before.events);
        self.snapshots.truncate(before.snapshots);
        self.plies.truncate(before.plies);
        self.keys.truncate(before.keys);
    }
}
