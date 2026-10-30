//! Native application operation and close ownership, independent of a GUI runtime.
use crate::SharedOpenToken;
use std::sync::{Arc, Mutex};

#[derive(Default)]
struct State {
    command: bool,
    file: bool,
    close_requested: bool,
    close_approved: bool,
}

/// Shared native lifecycle authority owned by the application controller.
/// Hosts hold handles, never independent copies of document-operation flags.
#[derive(Default)]
pub struct ApplicationLifecycle {
    state: Mutex<State>,
}

/// A bounded host waiter retains this native reservation until it completes.
/// Dropping a failed or cancelled operation releases exactly its own slot.
pub struct OperationReservation {
    owner: Arc<ApplicationLifecycle>,
    kind: OperationKind,
}

#[derive(Clone, Copy)]
enum OperationKind {
    Command,
    File,
}

impl ApplicationLifecycle {
    /// Reserve one complete command/admission waiter before releasing the host lock.
    pub fn reserve_command(self: &Arc<Self>) -> Result<OperationReservation, String> {
        self.reserve(OperationKind::Command)
    }

    /// Reserve the picker and complete atomic file operation. Saving is permitted
    /// during a close request; opening/replacement still needs current authority.
    pub fn reserve_file(self: &Arc<Self>) -> Result<OperationReservation, String> {
        self.reserve(OperationKind::File)
    }

    fn reserve(self: &Arc<Self>, kind: OperationKind) -> Result<OperationReservation, String> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| "Application lifecycle unavailable.")?;
        if state.close_approved {
            return Err("The application is closing; no further operations are accepted.".into());
        }
        if state.command || state.file {
            return Err("Wait for the current game or document operation.".into());
        }
        match kind {
            OperationKind::Command => {
                if state.close_requested {
                    return Err(
                        "Finish or cancel the close request before changing the game.".into(),
                    );
                }
                state.command = true;
            }
            OperationKind::File => state.file = true,
        }
        Ok(OperationReservation {
            owner: self.clone(),
            kind,
        })
    }

    /// Request close once. Repeated native menu/window requests can re-emit the
    /// current document guard without performing another semantic stop command.
    pub fn request_close(&self) -> Result<bool, String> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| "Application lifecycle unavailable.")?;
        if state.close_approved {
            return Err("Application close is already approved.".into());
        }
        let first = !state.close_requested;
        state.close_requested = true;
        Ok(first)
    }

    /// Cancel confirmation, leaving execution stopped until an explicit Resume.
    pub fn cancel_close(&self) -> Result<(), String> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| "Application lifecycle unavailable.")?;
        if state.close_approved {
            return Err("Application close is already approved.".into());
        }
        state.close_requested = false;
        Ok(())
    }

    /// Approve only the exact currently accepted native document after all
    /// command/file waiters release their reservations. Call under controller
    /// serialization so authority cannot change between comparison and approval.
    pub fn approve_close(
        &self,
        expected: SharedOpenToken,
        actual: SharedOpenToken,
    ) -> Result<(), String> {
        let mut state = self
            .state
            .lock()
            .map_err(|_| "Application lifecycle unavailable.")?;
        if !state.close_requested {
            return Err("Request close before confirming it.".into());
        }
        if state.command || state.file {
            return Err("Wait for the game or document operation before closing.".into());
        }
        if expected != actual {
            return Err("The document changed. Review unsaved changes before closing.".into());
        }
        state.close_approved = true;
        Ok(())
    }

    /// Whether a native close guard is pending, independently of game outcomes.
    pub fn close_requested(&self) -> bool {
        self.state
            .lock()
            .expect("application lifecycle")
            .close_requested
    }

    /// Whether final native close authority has been granted.
    pub fn close_approved(&self) -> bool {
        self.state
            .lock()
            .expect("application lifecycle")
            .close_approved
    }

    /// Whether a command or picker/file waiter currently owns a reservation.
    pub fn operation_active(&self) -> bool {
        let state = self.state.lock().expect("application lifecycle");
        state.command || state.file
    }
}

impl Drop for OperationReservation {
    fn drop(&mut self) {
        let mut state = self.owner.state.lock().expect("application lifecycle");
        match self.kind {
            OperationKind::Command => state.command = false,
            OperationKind::File => state.file = false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn token(document: u64, version: u64, document_revision: u64) -> SharedOpenToken {
        SharedOpenToken {
            document,
            version,
            document_revision,
        }
    }

    #[test]
    fn reservations_are_exclusive_release_on_failure_and_block_close_approval() {
        let lifecycle = Arc::new(ApplicationLifecycle::default());
        let command = lifecycle.reserve_command().unwrap();
        assert!(lifecycle.reserve_command().is_err());
        assert!(lifecycle.reserve_file().is_err());
        assert!(lifecycle.request_close().unwrap());
        assert!(
            lifecycle
                .approve_close(token(1, 1, 1), token(1, 1, 1))
                .is_err()
        );
        drop(command);
        let file = lifecycle.reserve_file().unwrap();
        assert!(lifecycle.reserve_command().is_err());
        assert!(
            lifecycle
                .approve_close(token(1, 1, 1), token(1, 1, 1))
                .is_err()
        );
        drop(file);
        assert!(!lifecycle.operation_active());
        lifecycle
            .approve_close(token(1, 1, 1), token(1, 1, 1))
            .unwrap();
        assert!(lifecycle.close_approved());
        assert!(lifecycle.reserve_file().is_err());
        assert!(lifecycle.cancel_close().is_err());
    }

    #[test]
    fn close_requires_exact_document_version_and_revision_without_aliasing_games() {
        let lifecycle = ApplicationLifecycle::default();
        let actual = token(3, 8, 4);
        assert!(lifecycle.approve_close(actual, actual).is_err());
        assert!(lifecycle.request_close().unwrap());
        assert!(!lifecycle.request_close().unwrap());
        for stale in [token(2, 8, 4), token(3, 7, 4), token(3, 8, 3)] {
            assert!(lifecycle.approve_close(stale, actual).is_err());
            assert!(!lifecycle.close_approved());
        }
        lifecycle.cancel_close().unwrap();
        assert!(!lifecycle.close_requested());
        assert!(lifecycle.approve_close(actual, actual).is_err());
    }
}
