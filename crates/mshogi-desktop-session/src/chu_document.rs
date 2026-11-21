//! Chu document: the generic native document (`game_document`) for Chu.
use crate::game_document::{ControlOf, GameSettings, ValidatedGameDocument};
pub use crate::game_document::{
    GameDocumentError as ChuDocumentError, GameDocumentUsage as ChuDocumentUsage, RewindKind,
};
use crate::native_game::{Chu, NativeGame};
use mshogi_core::chu as c;

/// Aggregate authoritative native replay units, including retained targets and installation.
/// This is independent of native per-record limits and the file worker's 15-second deadline.
pub const MAX_CHU_DOCUMENT_WORK: u64 = Chu::MAX_DOCUMENT_WORK;
/// Supported Chu document orientation.
pub type ChuSettings = GameSettings<c::Side>;
/// Chu document journal operation.
pub type ChuControl = ControlOf<Chu>;
/// An immutable validated Chu candidate.
pub type ValidatedChuDocument = ValidatedGameDocument<Chu>;

#[cfg(test)]
use crate::{game_document::validate, *};
#[cfg(test)]
use mshogi_core::native::CanonicalRecord;
#[cfg(test)]
use mshogi_engine::{ExecutionObservation, ExecutionPurpose, ExecutionResult};

#[cfg(test)]
fn native(r: &CanonicalRecord) -> Result<&c::GameRecord, ChuDocumentError> {
    crate::game_document::Errors::<Chu>::record_of(r)
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod execution_tests;
