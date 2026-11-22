//! Chu document: the generic saved game document (`game_document`) for Chu.
pub use crate::game_document::GameDocumentError as ChuDocumentError;
use crate::game_document::{GameSettings, ValidatedGameDocument};
use crate::native_game::Chu;
use mshogi_core::chu as c;

/// Saved Chu settings.
pub type ChuSettings = GameSettings<c::Side>;
/// An accepted Chu document.
pub type ValidatedChuDocument = ValidatedGameDocument<Chu>;
