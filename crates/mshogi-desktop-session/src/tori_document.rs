//! Tori document: the generic native document (`game_document`) for Tori.
use crate::game_document::{ControlOf, GameSettings, ValidatedGameDocument};
pub use crate::game_document::{
    GameDocumentError as ToriDocumentError, GameDocumentUsage as ToriDocumentUsage, RewindKind,
};
use crate::native_game::{NativeGame, Tori};
use mshogi_core::tori as t;

/// Whole-document validation allowance, in modelled Tori replay units.
pub const MAX_TORI_DOCUMENT_WORK: u64 = Tori::MAX_DOCUMENT_WORK;
/// Supported Tori document orientation.
pub type ToriSettings = GameSettings<t::Side>;
/// Tori document journal operation.
pub type ToriControl = ControlOf<Tori>;
/// An immutable validated Tori candidate.
pub type ValidatedToriDocument = ValidatedGameDocument<Tori>;

#[cfg(test)]
use crate::{game_document::validate, *};
#[cfg(test)]
use mshogi_core::native::CanonicalRecord;
#[cfg(test)]
use mshogi_engine::{ExecutionObservation, ExecutionPurpose, ExecutionResult};

#[cfg(test)]
fn native(r: &CanonicalRecord) -> Result<&t::GameRecord, ToriDocumentError> {
    crate::game_document::Errors::<Tori>::record_of(r)
}

#[cfg(test)]
mod tests;
