//! The core is tested here with a game that is not Chess: a subtraction game.
//! A player removes one, two or three counters; the player who removes the
//! last counter wins. The theory is known: a count divisible by four loses.
use super::*;

#[derive(Clone)]
struct Counters {
    left: u32,
    path: Vec<u32>,
    nulls: usize,
}
impl SearchPosition for Counters {
    type Move = u32;
    const HISTORY_SLOTS: usize = 4;
    fn hash(&self) -> u64 {
        u64::from(self.left).wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1 << 40
    }
    fn in_check(&self) -> bool {
        false
    }
    fn generate(&self, stage: Stage, moves: &mut Vec<u32>) {
        if stage == Stage::Quiet {
            moves.extend((1..=3).filter(|&n| n <= self.left));
        }
    }
    fn make(&mut self, m: u32) -> bool {
        if m == 0 || m > 3 || m > self.left {
            return false;
        }
        self.left -= m;
        self.path.push(m);
        true
    }
    fn unmake(&mut self) {
        self.left += self.path.pop().unwrap();
    }
    fn make_null(&mut self) {
        self.nulls += 1;
    }
    fn unmake_null(&mut self) {
        self.nulls -= 1;
    }
    fn null_move_safe(&self) -> bool {
        // A pass changes the result of this game, so the core must not pass.
        false
    }
    fn rule_score(&self) -> Option<i32> {
        None
    }
    fn draw_claimable(&self) -> bool {
        false
    }
    fn no_move_score(&self, ply: i32) -> i32 {
        // No counters left: the opponent removed the last one and won.
        -MATE_SCORE + ply
    }
    fn evaluate(&self) -> i32 {
        0
    }
    fn is_tactical(&self, _: u32) -> bool {
        false
    }
    fn tactical_order(&self, _: u32) -> i32 {
        0
    }
    fn tactical_gain(&self, _: u32) -> i32 {
        0
    }
    fn exchange_at_least(&self, _: u32, _: i32) -> bool {
        true
    }
    fn history_slot(&self, m: u32) -> usize {
        m as usize
    }
    fn encode(m: u32) -> u32 {
        m
    }
    fn decode(&self, bits: u32) -> Option<u32> {
        self.is_pseudo_legal(bits).then_some(bits)
    }
    fn is_pseudo_legal(&self, m: u32) -> bool {
        (1..=3).contains(&m) && m <= self.left
    }
}

fn limits(depth: u16) -> Limits {
    Limits {
        depth,
        nodes: None,
        soft: None,
        hard: None,
    }
}

#[test]
fn solves_a_game_that_is_not_chess() {
    let mut searcher = Searcher::<Counters>::new(1);
    for left in 1..=21u32 {
        let mut position = Counters {
            left,
            path: Vec::new(),
            nulls: 0,
        };
        let outcome = searcher.search(&mut position, limits(40), &mut || false);
        assert_eq!(position.left, left);
        assert!(position.path.is_empty());
        let score = outcome.score.unwrap();
        if left % 4 == 0 {
            // Lost: the opponent takes the last counter after 2 * left / 4 turns.
            assert_eq!(score, -MATE_SCORE + (left / 2) as i32, "{left}");
        } else {
            assert_eq!(outcome.best, Some(left % 4), "{left}");
            assert!(score >= MATE_BOUND, "{left}");
            assert_eq!(outcome.pv.first(), outcome.best.as_ref());
        }
        assert_eq!(outcome.status, Status::Complete);
        searcher.clear();
    }
}

#[test]
fn limits_stop_the_search_and_restore_the_position() {
    let mut searcher = Searcher::<Counters>::new(1);
    let start = Counters {
        left: 90,
        path: Vec::new(),
        nulls: 0,
    };
    let mut position = start.clone();
    let outcome = searcher.search(
        &mut position,
        Limits {
            nodes: Some(500),
            ..limits(60)
        },
        &mut || false,
    );
    assert_eq!(outcome.status, Status::NodeLimit);
    assert_eq!(outcome.nodes, 500);
    assert!(outcome.best.is_some());
    assert_eq!((position.left, position.path.len()), (90, 0));

    let outcome = searcher.search(&mut position, limits(60), &mut || true);
    assert_eq!(
        (outcome.status, outcome.best, outcome.nodes),
        (Status::Stopped, None, 0)
    );

    let outcome = searcher.search(
        &mut position,
        Limits {
            nodes: Some(0),
            ..limits(60)
        },
        &mut || false,
    );
    assert_eq!((outcome.status, outcome.best), (Status::NodeLimit, None));

    let mut polls = 0;
    let outcome = searcher.search(&mut position, limits(100), &mut || {
        polls += 1;
        polls > 3
    });
    assert_eq!(outcome.status, Status::Stopped);
    assert_eq!((position.left, position.path.len()), (90, 0));
    assert_eq!(position.nulls, 0);
}
