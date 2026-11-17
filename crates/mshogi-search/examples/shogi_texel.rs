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

#[cfg(test)]
mod tests {
    use mshogi_core::shogi::{Position, Side};
    use mshogi_search::alphabeta::SearchPosition;
    use mshogi_search::shogi::eval::{DEFAULT, INITIAL, ShogiTexel};
    use mshogi_search::shogi::position::ShogiPosition;
    use mshogi_search::texel::TexelModel;

    #[test]
    fn fitting_score_is_the_search_evaluation_from_sentes_view() {
        for sfen in [
            "lnsgkgsnl/1r5b1/ppppppppp/9/9/2P6/PP1PPPPPP/1B5R1/LNSGKGSNL w - 2",
            "2+P3snr/6lkl/4+P2p1/2p5P/3p2p+bS/7G1/1P4PPG/2g+n1+s1sK/7N1 w B5Prgn2l3p 216",
            "4k4/9/4P4/9/9/9/9/9/4K4 b Np 1",
        ] {
            let native = Position::from_sfen(sfen).unwrap();
            let p = ShogiPosition::from_position(&native);
            let Some(prepared) = ShogiTexel::prepare(sfen).unwrap() else {
                continue;
            };
            let sign = if native.side_to_move() == Side::Sente {
                1
            } else {
                -1
            };
            assert_eq!(ShogiTexel::score(&prepared, &DEFAULT), sign * p.evaluate());
            assert_eq!(
                ShogiTexel::score(&prepared, &INITIAL),
                sign * mshogi_search::shogi::eval::score(&p, &INITIAL)
            );
        }
    }

    #[test]
    fn checks_and_winning_captures_are_not_quiet_and_keys_ignore_the_counter() {
        // Sente's Rook can take an undefended Gold.
        assert!(
            ShogiTexel::prepare("4k4/9/9/9/4g4/9/9/4R4/K8 b - 1")
                .unwrap()
                .is_none()
        );
        // Gote is in check.
        assert!(
            ShogiTexel::prepare("4k4/9/9/9/4R4/9/9/9/K8 w - 1")
                .unwrap()
                .is_none()
        );
        assert!(
            ShogiTexel::prepare("4k4/9/9/9/9/9/9/9/K3G4 b - 1")
                .unwrap()
                .is_some()
        );
        assert_eq!(
            ShogiTexel::key("4k4/9/9/9/9/9/9/9/K3R4 b - 1"),
            ShogiTexel::key("4k4/9/9/9/9/9/9/9/K3R4 b - 41")
        );
        assert!(ShogiTexel::prepare("not an sfen").is_err());
    }
}
