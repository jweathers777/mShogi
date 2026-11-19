//! Texel fitting of the Xiangqi classical evaluation with the shared tool.
//! fit TRAIN VALIDATION OUTPUT [SECONDS]; check FROZEN HELDOUT OUTPUT;
//! stats FILE.... Rows use the `fen` field and Red's native outcome.
//! Positions that are not quiet (in check, or with a capture that wins
//! material by static exchange) are skipped and counted.
use mshogi_search::xiangqi::eval::XiangqiTexel;
#[path = "support/texel.rs"]
mod texel;

fn main() -> texel::Result<()> {
    texel::main::<XiangqiTexel>(3600)
}

#[cfg(test)]
mod tests {
    use mshogi_core::xiangqi::{PositionRoot, Side};
    use mshogi_search::alphabeta::SearchPosition;
    use mshogi_search::texel::TexelModel;
    use mshogi_search::xiangqi::eval::{DEFAULT, INITIAL, XiangqiTexel};
    use mshogi_search::xiangqi::position::XiangqiPosition;

    #[test]
    fn fitting_score_is_the_search_evaluation_from_reds_view() {
        for fen in [
            "rnbakabnr/9/1c5c1/p1p1p1p1p/9/9/P1P1P1P1P/1C5C1/9/RNBAKABNR w - - 0 1",
            "1rbakab1r/9/2n1c1nc1/p3p1pRp/2p6/9/P1P1P1P1P/1CN1C1N2/9/R1BAKAB2 b - - 0 1",
            "3ak4/4a4/9/9/9/9/9/2C6/R8/5K3 w - - 0 1",
        ] {
            let root = PositionRoot::from_fen(fen).unwrap();
            let p = XiangqiPosition::from_position(root.position());
            let Some(prepared) = XiangqiTexel::prepare(fen).unwrap() else {
                continue;
            };
            let sign = if p.side_to_move() == Side::Red { 1 } else { -1 };
            assert_eq!(
                XiangqiTexel::score(&prepared, &DEFAULT),
                sign * p.evaluate()
            );
            assert_eq!(
                XiangqiTexel::score(&prepared, &INITIAL),
                sign * mshogi_search::xiangqi::eval::score(&p, &INITIAL)
            );
        }
    }

    #[test]
    fn checks_and_winning_captures_are_not_quiet_and_keys_ignore_counters() {
        // Red's Chariot can take an undefended Horse.
        assert!(
            XiangqiTexel::prepare("4k4/9/9/9/n8/9/9/9/9/R2K5 w - - 0 1")
                .unwrap()
                .is_none()
        );
        // Black is in check from the Chariot on the e-file.
        assert!(
            XiangqiTexel::prepare("4k4/9/9/9/4R4/9/9/9/9/3K5 b - - 0 1")
                .unwrap()
                .is_none()
        );
        assert!(
            XiangqiTexel::prepare("4k4/9/9/9/9/9/9/9/9/3K1R3 b - - 0 1")
                .unwrap()
                .is_some()
        );
        assert_eq!(
            XiangqiTexel::key("4k4/9/9/9/9/9/9/9/9/3KR4 b - - 0 1"),
            XiangqiTexel::key("4k4/9/9/9/9/9/9/9/9/3KR4 b - - 12 40")
        );
        assert!(XiangqiTexel::prepare("not a fen").is_err());
    }
}
