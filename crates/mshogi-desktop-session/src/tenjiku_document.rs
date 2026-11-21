//! Tenjiku document: the generic native document (`game_document`) for Tenjiku.
use crate::game_document::{ControlOf, GameSettings, ValidatedGameDocument};
pub use crate::game_document::{
    GameDocumentError as TenjikuDocumentError, GameDocumentUsage as TenjikuDocumentUsage,
    RewindKind,
};
use crate::native_game::{NativeGame, Tenjiku};
use mshogi_core::tenjiku as c;

/// Measured aggregate native replay allowance, including retained targets and installation.
/// Both profiles' eight retained 128-event contexts need approximately 153.23M units
/// with actual ending reserves and took approximately 5.5s for decode plus native
/// game restoration (the full file-worker/session path is measured separately).
/// The provisional 600M allowance admitted a 28-context document that stopped at 15s
/// on reopen. This delivery limit is independent of native per-record limits and
/// does not replace the file worker's cooperative 15-second elapsed deadline.
pub const MAX_TENJIKU_DOCUMENT_WORK: u64 = Tenjiku::MAX_DOCUMENT_WORK;
/// Supported Tenjiku document orientation.
pub type TenjikuSettings = GameSettings<c::Side>;
/// Tenjiku document journal operation.
pub type TenjikuControl = ControlOf<Tenjiku>;
/// An immutable validated Tenjiku candidate.
pub type ValidatedTenjikuDocument = ValidatedGameDocument<Tenjiku>;

#[cfg(test)]
use crate::{game_document::validate, *};
#[cfg(test)]
use mshogi_core::native::{CanonicalRecord, RulesProfile};
#[cfg(test)]
use mshogi_engine::{ExecutionObservation, ExecutionPurpose, ExecutionResult};

#[cfg(test)]
type Payload = crate::game_document::Payload<Tenjiku>;
#[cfg(test)]
fn charge(total: &mut u64, n: u64) -> Result<(), TenjikuDocumentError> {
    crate::game_document::Errors::<Tenjiku>::charge(total, n)
}

#[cfg(test)]
mod tests;

#[cfg(test)]
pub(crate) fn measurement_document(profile: c::Profile) -> ValidatedTenjikuDocument {
    tests::measurement_document(profile)
}

#[cfg(test)]
pub(crate) fn measured_real_engine_document(profile: c::Profile) -> ValidatedTenjikuDocument {
    tests::measured_real_engine_document(profile)
}
