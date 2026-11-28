//! The core is tested here with a game that is not Chess: a subtraction game.
//! A player removes one, two or three counters; the player who removes the
//! last counter wins. The theory is known: a count divisible by four loses.
use super::*;

#[test]
fn timed_fallback_is_unscored_and_does_not_override_cancellation_or_fixed_work() {
    let root = Counters {
        left: 12,
        path: vec![],
        nulls: 0,
    };
    for (nodes, hard, expected) in [
        (Some(100), Duration::ZERO, Status::Stopped),
        (Some(0), Duration::from_secs(1), Status::NodeLimit),
        (Some(1), Duration::from_secs(1), Status::NodeLimit),
    ] {
        let mut p = root.clone();
        let mut s = Searcher::new(1);
        let mut bounds = limits(40);
        bounds.nodes = nodes;
        bounds.hard = Some(hard);
        let result = s.search_with_fallback(&mut p, bounds, &mut || false, 2, false);
        assert_eq!(
            (result.best, result.score, result.depth),
            (Some(2), None, 0)
        );
        assert_eq!(result.pv, vec![2]);
        assert_eq!(result.status, expected);
        assert!(result.nodes <= nodes.unwrap());
        assert_eq!(p.left, root.left);
        assert!(p.path.is_empty());
        let cancelled = s.search_with_fallback(&mut p, bounds, &mut || true, 2, false);
        assert!(cancelled.best.is_none());
        assert!(cancelled.pv.is_empty());
        bounds.hard = None;
        let original = Searcher::new(1).search(&mut p, bounds, &mut || false);
        let opted_in =
            Searcher::new(1).search_with_fallback(&mut p, bounds, &mut || false, 2, false);
        assert_eq!(original, opted_in);
    }
}

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
        clock: false,
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

#[test]
fn managed_forced_move_stops_after_one_iteration_and_restores() {
    let mut searcher = Searcher::<Counters>::new(1);
    let mut position = Counters {
        left: 1,
        path: Vec::new(),
        nulls: 0,
    };
    let mut budget = limits(20);
    budget.clock = true;
    budget.soft = Some(Duration::from_secs(1));
    budget.hard = Some(Duration::from_secs(3));
    let outcome = searcher.search(&mut position, budget, &mut || false);
    assert_eq!(outcome.best, Some(1));
    // At depth one this game reaches its zero static leaf evaluation.
    // The forced exit must preserve that completed result, not invent a mate.
    assert_eq!(outcome.score, Some(0));
    assert_eq!(outcome.depth, 1);
    assert_eq!(position.left, 1);
    assert!(position.path.is_empty());
    budget.clock = false;
    budget.soft = budget.hard;
    let fixed = searcher.search(&mut position, budget, &mut || false);
    assert_eq!(fixed.depth, 20);
    assert_eq!(fixed.best, Some(1));
    assert_eq!(fixed.score, Some(MATE_SCORE - 1));
}

#[test]
fn managed_two_moves_do_not_use_forced_exit() {
    let mut searcher = Searcher::<Counters>::new(1);
    let mut position = Counters {
        left: 2,
        path: Vec::new(),
        nulls: 0,
    };
    let mut budget = limits(4);
    budget.clock = true;
    budget.soft = Some(Duration::from_secs(1));
    budget.hard = Some(Duration::from_secs(3));
    let outcome = searcher.search(&mut position, budget, &mut || false);
    assert_eq!(outcome.depth, 4);
    assert_eq!(outcome.best, Some(2));
    assert_eq!(position.left, 2);
    assert!(position.path.is_empty());
}

#[test]
fn singular_exclusion_has_no_terminal_or_table_authority_and_restores_on_abort() {
    let mut s = Searcher::<Counters>::new(1).with_selectivity(Selectivity {
        singular_extension: true,
        ..Default::default()
    });
    let mut pos = Counters {
        left: 1,
        path: vec![],
        nulls: 0,
    };
    let hash = pos.hash();
    s.table.store(hash, 1, 1, 100, Some(0), 6, Bound::Lower);
    let before = s.table.probe(hash, 1).unwrap();
    s.excluded[1] = Some(1);
    let mut stop = || false;
    {
        let mut run = Run {
            s: &mut s,
            pos: &mut pos,
            stop: &mut stop,
            start: Instant::now(),
            limits: limits(8),
            nodes: 0,
            seldepth: 0,
            abort: false,
            status: Status::Complete,
            root_move: None,
            root_count: 0,
        };
        // A lower table score of 100 must not cut off the verification at 20.
        // With the sole legal move excluded, 10 is fail-low, not checkmate.
        assert_eq!(run.search(3, 1, 10, 20, false, false), 10);
    }
    let after = s.table.probe(hash, 1).unwrap();
    assert_eq!(
        (before.score, before.depth, before.move_bits),
        (after.score, after.depth, after.move_bits)
    );
    s.excluded[1] = None;
    {
        let mut bounded = limits(8);
        bounded.nodes = Some(1);
        let mut run = Run {
            s: &mut s,
            pos: &mut pos,
            stop: &mut stop,
            start: Instant::now(),
            limits: bounded,
            nodes: 0,
            seldepth: 0,
            abort: false,
            status: Status::Complete,
            root_move: None,
            root_count: 0,
        };
        // depth 7 cannot cut off from depth 6 TT. Verification hits the node cap.
        assert_eq!(run.search(7, 1, -1000, 1000, false, true), 0);
        assert!(run.abort);
    }
    assert!(s.excluded.iter().all(Option::is_none));
    assert_eq!(pos.left, 1);
    assert!(pos.path.is_empty());
    assert_eq!(pos.nulls, 0);
}

struct RazorEquality {
    state: u32,
}
impl SearchPosition for RazorEquality {
    type Move = u32;
    const HISTORY_SLOTS: usize = 3;
    fn hash(&self) -> u64 {
        112233 + self.state as u64
    }
    fn in_check(&self) -> bool {
        self.state == 2
    }
    fn generate(&self, stage: Stage, moves: &mut Vec<u32>) {
        if self.state == 0 {
            moves.push(if stage == Stage::Tactical { 1 } else { 2 });
        }
    }
    fn make(&mut self, m: u32) -> bool {
        if self.state == 0 && (1..=2).contains(&m) {
            self.state = m;
            true
        } else {
            false
        }
    }
    fn unmake(&mut self) {
        self.state = 0;
    }
    fn make_null(&mut self) {
        panic!("null unsafe")
    }
    fn unmake_null(&mut self) {
        panic!("null unsafe")
    }
    fn null_move_safe(&self) -> bool {
        false
    }
    fn rule_score(&self) -> Option<i32> {
        None
    }
    fn draw_claimable(&self) -> bool {
        false
    }
    fn no_move_score(&self, ply: i32) -> i32 {
        if self.state == 2 {
            -MATE_SCORE + ply
        } else {
            0
        }
    }
    fn evaluate(&self) -> i32 {
        if self.state == 0 { -1000 } else { 0 }
    }
    fn is_tactical(&self, m: u32) -> bool {
        m == 1
    }
    fn tactical_order(&self, _: u32) -> i32 {
        0
    }
    fn tactical_gain(&self, _: u32) -> i32 {
        2000
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
        self.state == 0 && (1..=2).contains(&m)
    }
}
#[test]
fn razor_threshold_equality_does_not_hide_quiet_mating_check() {
    // Quiescence sees only a drawing capture. Its score equals alpha 0, which
    // fails high in [-1,0]. Full search must still see the quiet mating check.
    let mut s = Searcher::<RazorEquality>::new(1).with_selectivity(Selectivity {
        razoring: true,
        ..Default::default()
    });
    let mut pos = RazorEquality { state: 0 };
    let mut stop = || false;
    let mut run = Run {
        s: &mut s,
        pos: &mut pos,
        stop: &mut stop,
        start: Instant::now(),
        limits: limits(1),
        nodes: 0,
        seldepth: 0,
        abort: false,
        status: Status::Complete,
        root_move: None,
        root_count: 0,
    };
    assert_eq!(run.search(1, 1, 0, 1, false, true), MATE_SCORE - 2);
    assert_eq!(run.pos.state, 0);
}

#[test]
fn improving_discards_previous_search_static_evaluations() {
    let mut s = Searcher::<Counters>::new(1).with_selectivity(Selectivity {
        improving: true,
        ..Default::default()
    });
    s.static_evals.fill(Some(-999_999));
    let mut pos = Counters {
        left: 1,
        path: vec![],
        nulls: 0,
    };
    let result = s.search(&mut pos, limits(2), &mut || false);
    assert_eq!(result.score, Some(MATE_SCORE - 1));
    assert_eq!(s.static_evals[MAX_PLY], None);
    assert!(pos.path.is_empty());
}

#[test]
fn table_entries_survive_search_requests_until_new_game() {
    let mut searcher = Searcher::<Counters>::new(1);
    let mut position = Counters {
        left: 8,
        path: vec![],
        nulls: 0,
    };
    let hash = position.hash();
    searcher.search(&mut position, limits(8), &mut || false);
    let entry = searcher
        .table
        .probe(hash, 0)
        .expect("completed root is stored");
    // A stopped request advances the generation but does not clear the table.
    searcher.search(&mut position, limits(8), &mut || true);
    let retained = searcher
        .table
        .probe(hash, 0)
        .expect("entry survives request");
    assert_eq!(
        (retained.score, retained.depth, retained.move_bits),
        (entry.score, entry.depth, entry.move_bits)
    );
    searcher.clear();
    assert!(searcher.table.probe(hash, 0).is_none());
}

/// A hand-solved tactical chain: a threatened side has one quiet escape to a
/// terminal loss for its opponent. A separate quiet leaf has static value 37.
struct HorizonGame<const Q: usize> {
    state: u32,
    path: Vec<u32>,
}
impl<const Q: usize> SearchPosition for HorizonGame<Q> {
    type Move = u32;
    const HISTORY_SLOTS: usize = 2;
    const QUIESCENCE_PLIES: usize = Q;
    fn hash(&self) -> u64 {
        (0x1820_0000 + u64::from(self.state)) << 32 | 17
    }
    fn in_check(&self) -> bool {
        self.state == 0
    }
    fn generate(&self, stage: Stage, moves: &mut Vec<u32>) {
        if self.state == 0 && stage == Stage::Quiet {
            moves.push(1);
        }
    }
    fn make(&mut self, m: u32) -> bool {
        if self.state != 0 || m != 1 {
            return false;
        }
        self.path.push(self.state);
        self.state = 1;
        true
    }
    fn unmake(&mut self) {
        self.state = self.path.pop().unwrap();
    }
    fn make_null(&mut self) {
        panic!("null disabled");
    }
    fn unmake_null(&mut self) {
        panic!("null disabled");
    }
    fn null_move_safe(&self) -> bool {
        false
    }
    fn rule_score(&self) -> Option<i32> {
        (self.state == 1).then_some(-73)
    }
    fn draw_claimable(&self) -> bool {
        false
    }
    fn no_move_score(&self, _: i32) -> i32 {
        -999
    }
    fn evaluate(&self) -> i32 {
        assert!(!self.in_check());
        37
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
        panic!("quiet escape needs no exchange test");
    }
    fn history_slot(&self, _: u32) -> usize {
        1
    }
    fn encode(m: u32) -> u32 {
        m
    }
    fn decode(&self, bits: u32) -> Option<u32> {
        self.is_pseudo_legal(bits).then_some(bits)
    }
    fn is_pseudo_legal(&self, m: u32) -> bool {
        self.state == 0 && m == 1
    }
}
fn horizon_probe<const Q: usize>(
    s: &mut Searcher<HorizonGame<Q>>,
    pos: &mut HorizonGame<Q>,
    qdepth: usize,
) -> i32 {
    let mut stop = || false;
    let mut run = Run {
        s,
        pos,
        stop: &mut stop,
        start: Instant::now(),
        limits: limits(1),
        nodes: 0,
        seldepth: 0,
        abort: false,
        status: Status::Complete,
        root_move: None,
        root_count: 0,
    };
    // Absolute ply is deliberately different: the tactical horizon is relative.
    run.quiesce(13, qdepth, -1000, 1000, false)
}
fn check_horizon<const Q: usize>() {
    let mut s = Searcher::<HorizonGame<Q>>::new(1);
    let mut p = HorizonGame {
        state: 0,
        path: vec![],
    };
    assert_eq!(horizon_probe(&mut s, &mut p, Q - 1), 73);
    assert_eq!(p.state, 0);
    assert!(p.path.is_empty());
    // At the stated horizon, threat uncertainty is zero, not stand-pat 37.
    assert_eq!(horizon_probe(&mut s, &mut p, Q), 0);
    p.state = 1;
    assert_eq!(horizon_probe(&mut s, &mut p, Q), -73);
    p.state = 2;
    assert_eq!(horizon_probe(&mut s, &mut p, Q), 37);
}
#[test]
fn bounded_quiescence_keeps_quiet_evasions_and_checks_terminal_before_horizon() {
    check_horizon::<4>();
    check_horizon::<8>();
}
fn check_horizon_cache<const Q: usize>() {
    let mut s = Searcher::<HorizonGame<Q>>::new(1);
    let mut p = HorizonGame {
        state: 2,
        path: vec![],
    };
    let hash = p.hash();
    let shallow = -((Q - 1) as i32);
    s.table
        .store(hash, 13, 0, 999, Some(37), shallow, Bound::Exact);
    assert_eq!(horizon_probe(&mut s, &mut p, 0), 37);
    assert_eq!(s.table.probe(hash, 13).unwrap().depth, 0);
    s.table.store(hash, 13, 0, 888, Some(37), 0, Bound::Exact);
    assert_eq!(horizon_probe(&mut s, &mut p, Q - 1), 888);
    s.table.clear();
    s.table
        .store(hash, 13, 0, 777, Some(37), shallow, Bound::Exact);
    assert_eq!(horizon_probe(&mut s, &mut p, Q - 1), 777);
    assert_eq!(p.state, 2);
    assert!(p.path.is_empty());
}
#[test]
fn bounded_quiescence_rejects_shallow_cache_for_fresh_tactical_work() {
    check_horizon_cache::<4>();
    check_horizon_cache::<8>();
}
