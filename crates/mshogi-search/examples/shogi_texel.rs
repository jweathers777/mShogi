//! Texel fitting of the Shogi classical evaluation with the shared tool.
//! fit TRAIN VALIDATION OUTPUT [SECONDS]; check FROZEN HELDOUT OUTPUT;
//! stats FILE.... Rows use the `sfen` field and Sente's native outcome.
//! Positions that are not quiet (in check, or with a capture or promotion
//! that wins material by static exchange) are skipped and counted.
use mshogi_search::shogi::eval::ShogiTexel;
#[path = "support/texel.rs"]
mod texel;

fn main() -> texel::Result<()> {
    texel::main::<ShogiTexel>(3600)
}
