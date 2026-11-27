//! Shared clock allocation and iteration timing. No game rules live here.
use std::time::{Duration, Instant};

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

/// One adapter request deadline, including setup and result validation.
/// Untimed requests do not read the clock after construction.
pub struct RequestTime {
    started: Instant,
    budget: Option<TimeBudget>,
}
impl RequestTime {
    /// Start accounting before reconstruction or other request setup.
    pub fn new(budget: Option<TimeBudget>) -> Self {
        Self {
            started: Instant::now(),
            budget,
        }
    }
    /// Whether the total request budget has expired.
    pub fn expired(&self) -> bool {
        self.budget
            .is_some_and(|time| self.started.elapsed() >= time.hard)
    }
    /// Remaining budget, charging setup for both fixed move and clock requests.
    /// This differs from `TimeBudget::after_elapsed`, whose historical fixed
    /// Chess move-time semantics start timing at the private search.
    pub fn remaining(&self) -> Option<TimeBudget> {
        self.budget.map(|time| {
            let elapsed = self.started.elapsed();
            TimeBudget {
                soft: time.soft.saturating_sub(elapsed),
                hard: time.hard.saturating_sub(elapsed),
                clock: time.clock,
            }
        })
    }
    /// Shared core limits, with a small reserve for engine result validation.
    pub fn limits(&self, depth: u16, nodes: Option<u64>) -> super::Limits {
        let time = self.remaining().map(|time| {
            let reserve = (time.hard / 10).min(Duration::from_millis(50));
            TimeBudget {
                soft: time.soft.saturating_sub(reserve),
                hard: time.hard.saturating_sub(reserve),
                clock: time.clock,
            }
        });
        super::Limits {
            depth,
            nodes,
            soft: time.map(|t| t.soft),
            hard: time.map(|t| t.hard),
            clock: time.is_some_and(|t| t.clock),
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
    fn request_deadline_charges_setup_and_reserves_validation_time() {
        for clock in [false, true] {
            let mut request = RequestTime::new(Some(TimeBudget {
                soft: Duration::from_millis(80),
                hard: Duration::from_millis(100),
                clock,
            }));
            request.started -= Duration::from_millis(20);
            let remaining = request.remaining().unwrap();
            assert!(remaining.hard <= Duration::from_millis(80));
            assert!(remaining.soft <= Duration::from_millis(60));
            assert_eq!(remaining.clock, clock);
            let limits = request.limits(4, Some(100));
            assert!(limits.hard.unwrap() < remaining.hard);
            assert!(limits.soft.unwrap() <= limits.hard.unwrap());
            assert_eq!(limits.depth, 4);
            assert_eq!(limits.nodes, Some(100));
            request.started -= Duration::from_secs(1);
            assert!(request.expired());
            assert_eq!(request.remaining().unwrap().hard, Duration::ZERO);
            assert_eq!(request.limits(4, None).hard, Some(Duration::ZERO));
        }
        let request = RequestTime::new(None);
        assert!(!request.expired());
        assert!(request.remaining().is_none());
        let limits = request.limits(3, Some(15));
        assert_eq!(
            (limits.soft, limits.hard, limits.clock),
            (None, None, false)
        );
    }

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
