//! Shared clock allocation and iteration timing. No game rules live here.
use std::time::Duration;

/// Soft iteration target and hard search cutoff for one move.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeBudget {
    /// Nominal time; equal to hard for fixed move time.
    pub soft: Duration,
    /// Maximum search time.
    pub hard: Duration,
    /// True for allocation from a game clock; false for fixed search time.
    pub clock: bool,
}

impl TimeBudget {
    /// Fixed time for diagnostics and desktop callers.
    pub fn from_hard_ms(ms: u64) -> Self {
        Self {
            soft: Duration::from_millis(ms),
            hard: Duration::from_millis(ms),
            clock: false,
        }
    }

    /// Allocate only from time already on the clock. Keep a reserve for
    /// transport and scheduling, and save part of the increment for later moves.
    pub fn from_clock_ms(remaining: u64, increment: u64, moves: u64) -> Self {
        let reserve = 50.min(remaining / 2);
        let spendable = remaining.saturating_sub(reserve);
        let soft = (spendable / moves.max(1))
            .saturating_add(increment.saturating_sub(increment / 4))
            .min(spendable);
        Self {
            soft: Duration::from_millis(soft),
            hard: Duration::from_millis(soft.saturating_mul(3).min(spendable)),
            clock: true,
        }
    }

    /// Leave time for result delivery without consuming a short request.
    pub fn with_response_margin(self) -> Self {
        self.after_elapsed(Duration::from_millis(10).min(self.hard / 2))
    }

    /// Charge elapsed setup against the request budget.
    pub fn after_elapsed(self, elapsed: Duration) -> Self {
        if !self.clock {
            return self;
        }
        Self {
            soft: self.soft.saturating_sub(elapsed),
            hard: self.hard.saturating_sub(elapsed),
            clock: self.clock,
        }
    }
}

/// Stable choices need less time; a changed choice or falling score needs more.
/// Scores close to a win/loss are not treated as ordinary evaluation changes.
pub(super) fn iteration_target(
    soft: Duration,
    hard: Duration,
    stable: u32,
    previous: i32,
    score: i32,
) -> Duration {
    let falling = previous.abs() < super::MATE_BOUND
        && score.abs() < super::MATE_BOUND
        && previous.saturating_sub(score) >= 30;
    let percent = if falling || stable == 0 {
        150
    } else if stable >= 3 {
        60
    } else {
        100
    };
    (soft.saturating_mul(percent) / 100).min(hard)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn allocation_never_borrows_future_increment() {
        for remaining in [0, 1, 10, 50, 100, 800, 10_000, u64::MAX] {
            for increment in [0, 50, u64::MAX] {
                for moves in [0, 1, 30, u64::MAX] {
                    let t = TimeBudget::from_clock_ms(remaining, increment, moves);
                    assert!(t.soft <= t.hard);
                    assert!(t.hard <= Duration::from_millis(remaining));
                    if remaining >= 2 {
                        assert!(t.hard < Duration::from_millis(remaining));
                    }
                }
            }
        }
    }
    #[test]
    fn short_requests_keep_search_time() {
        for ms in [1, 2, 5, 10, 20, 100] {
            let t = TimeBudget {
                clock: true,
                ..TimeBudget::from_hard_ms(ms)
            }
            .with_response_margin();
            assert!(t.hard > Duration::ZERO);
            assert!(t.hard < Duration::from_millis(ms));
            assert_eq!(t.soft, t.hard);
        }
    }

    #[test]
    fn stability_saves_time_and_score_fall_restores_it() {
        let soft = Duration::from_millis(100);
        let hard = Duration::from_millis(300);
        assert!(iteration_target(soft, hard, 4, 10, 10) < soft);
        assert!(iteration_target(soft, hard, 0, 10, 10) > soft);
        assert!(iteration_target(soft, hard, 4, 10, -100) > soft);
        assert_eq!(iteration_target(soft, soft, 0, 10, 10), soft);
        assert_eq!(
            TimeBudget::from_hard_ms(100).after_elapsed(hard),
            TimeBudget::from_hard_ms(100)
        );
    }
}
