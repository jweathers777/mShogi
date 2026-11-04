//! Authoritative native Tenjiku rules for two immutable whole profiles.
//! Shared application dispatch, search and normal product activation remain separate consumers.
mod position;
pub use position::*;
mod legal;
pub use legal::*;
mod game;
pub use game::*;
mod record;
pub use record::*;
/// Complete human action display/input; native Game retains admission authority.
pub mod notation;
mod perft;
pub use perft::*;
#[cfg(test)]
mod conformance;
#[cfg(test)]
mod game_tests;
#[cfg(test)]
mod local_tests;
