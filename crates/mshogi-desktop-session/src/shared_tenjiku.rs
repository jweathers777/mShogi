//! Tenjiku in the sole desktop runtime: the generic native session
//! (`native_session`) with the Tenjiku presentation and stationary captures.
use super::native_session::*;
use super::*;
pub(crate) use crate::native_game::Tenjiku;
use mshogi_core::tenjiku as c;

/// The Tenjiku session.
pub(crate) type TenjikuSession = NativeSession<Tenjiku>;

large_shogi_session!(Tenjiku, tenjiku,
state: tenjiku, tenjiku_mut, field: tenjiku,
    board: 16, "Tenjiku Shogi 16×16 board", "8i",
    score_meaning: "Transparent material evaluation in Pawns from the analyzed native side; native terminal wins use board-action distance. Operational failures and absence of board actions remain unscored.",
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

impl DesktopController {
    /// Verification entry into the real native Tenjiku request path.
    #[cfg(any(test, feature = "desktop-verification"))]
    pub fn verify_tenjiku_execution(
        &mut self,
        limits: WorkLimits,
        analysis: bool,
    ) -> Result<ExecutionContext, String> {
        self.verify_game_execution::<Tenjiku>(limits, analysis)
    }
}

#[path = "shared_tenjiku_presentation.rs"]
mod presentation;
pub use presentation::{Presentation as TenjikuPresentation, TenjikuChoiceShape};
pub(super) fn inspect(game: &c::Game, origin: &str) -> Result<inspection::Inspected, String> {
    presentation::inspect(game, origin)
}
