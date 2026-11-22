//! Tori document: the generic saved game document (`game_document`) for Tori.
pub use crate::game_document::GameDocumentError as ToriDocumentError;
use crate::game_document::{GameSettings, ValidatedGameDocument};
use crate::native_game::Tori;
use mshogi_core::tori as t;

/// Saved Tori settings.
pub type ToriSettings = GameSettings<t::Side>;
/// An accepted Tori document.
pub type ValidatedToriDocument = ValidatedGameDocument<Tori>;
