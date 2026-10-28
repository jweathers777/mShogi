//! Reproducible native UCI experiments over the authoritative chess match API.
mod config;
mod process;
mod report;
mod run;
/// Native shogi USI experiments, separate from chess clocks and outcomes.
pub mod shogi;
/// Native Xiangqi UCI0-9 experiments, explicit profile and unscored limits.
pub mod xiangqi;
pub use config::*;
pub use report::*;
pub use run::run_experiment;
use std::{error::Error, fmt};
/// Invalid configuration/report or host operation before a game can be recorded.
#[derive(Debug)]
pub struct RunnerError(pub String);
impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl Error for RunnerError {}
