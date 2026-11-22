//! Tenjiku document: the generic saved game document (`game_document`) for Tenjiku.
pub use crate::game_document::GameDocumentError as TenjikuDocumentError;
use crate::game_document::{GameSettings, ValidatedGameDocument};
use crate::native_game::Tenjiku;
use mshogi_core::tenjiku as c;

/// Saved Tenjiku settings.
pub type TenjikuSettings = GameSettings<c::Side>;
/// An accepted Tenjiku document.
pub type ValidatedTenjikuDocument = ValidatedGameDocument<Tenjiku>;
