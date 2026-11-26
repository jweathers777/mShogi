//! Reply timing for externally clocked matches.
use crate::{
    process::Engine,
    protocol::{Failure, Traffic},
    transport::Transport,
    *,
};
use std::time::{Duration, Instant};

pub(crate) enum Outcome {
    Accepted(String, u64, u64),
    Exhausted(u64),
    /// The overall game watchdog, not the mover's clock, ended the wait.
    Watchdog,
}

impl Transport<Vec<Traffic>> {
    /// Send the position and clock commands, then wait for `bestmove` until
    /// the mover's remaining time plus grace or the overall deadline.
    pub(crate) fn search(
        &mut self,
        engine: &mut Engine,
        position: &str,
        go: &str,
        remaining_ms: u64,
        grace_ms: u64,
        deadline: Instant,
    ) -> Result<Outcome, Failure> {
        let index = engine.index;
        self.send(engine, "position", position)?;
        self.send(engine, "search", go)?;
        let started = self.observed;
        let limit = remaining_ms.saturating_add(grace_ms);
        let clock_deadline =
            self.start + Duration::from_nanos(started) + Duration::from_millis(limit);

        let watchdog_capped = deadline < clock_deadline;
        let mut info = 0u64;
        loop {
            match self.line(engine, "search", clock_deadline, None)? {
                None => {
                    if watchdog_capped {
                        return Ok(Outcome::Watchdog);
                    }
                    let elapsed = self.observed.saturating_sub(started).div_ceil(1_000_000);
                    return Ok(Outcome::Exhausted(elapsed.max(limit + 1)));
                }
                Some(line) if line.trim_start().starts_with("bestmove") => {
                    let elapsed = self.observed.saturating_sub(started).div_ceil(1_000_000);
                    if elapsed > limit {
                        return Ok(Outcome::Exhausted(elapsed));
                    }
                    return Ok(Outcome::Accepted(line, elapsed, info));
                }
                Some(line) if line.trim_start().starts_with("info ") => info += 1,
                Some(_) => {
                    return Err(self.failure(
                        index,
                        FailureKind::MalformedReply,
                        "search",
                        "unexpected search output",
                    ));
                }
            }
        }
    }
}
