//! Tenjiku in the sole desktop runtime: the generic native session
//! (`native_session`) with the Tenjiku presentation and stationary captures.
use super::native_session::*;
use super::*;
use crate::game_document::ValidatedGameDocument;
pub(crate) use crate::native_game::Tenjiku;
use mshogi_core::tenjiku as c;
use std::sync::Arc;

/// The Tenjiku session.
pub(crate) type TenjikuSession = NativeSession<Tenjiku>;

large_shogi_session!(Tenjiku, tenjiku, TenjikuResult, TENJIKU_PLAY_LIMITS, TENJIKU_MAX_DEPTH,
TENJIKU_MAX_NODES,
operation: Tenjiku, preferences: TenjikuPreferences, runtime: TenjikuRuntime,
save: TenjikuSave, state: tenjiku, tenjiku_mut, snapshot: tenjiku_snapshot,
pv: |a| if a.kind == c::ActionKind::Igui {
    format!(
        "{}!x{} (stationary capture)",
        a.from,
        a.target.expect("validated native igui target")
    )
} else {
    format!(
        "{}→{}{}",
        a.from,
        a.route.iter().map(ToString::to_string).collect::<Vec<_>>().join("→"),
        if a.promote { " (promote)" } else { "" }
    )
});

#[path = "shared_tenjiku_presentation.rs"]
mod presentation;
pub use presentation::{Presentation as TenjikuPresentation, TenjikuChoiceShape};
pub(super) fn inspect(game: &c::Game, origin: &str) -> Result<inspection::Inspected, String> {
    presentation::inspect(game, origin)
}
