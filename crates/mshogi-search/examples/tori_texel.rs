//! Texel fitting of the Tori classical evaluation with the shared tool.
//! fit TRAIN VALIDATION OUTPUT [SECONDS]; check FROZEN HELDOUT OUTPUT;
//! stats FILE.... Rows use the compact `diagram` field and Sente's
//! outcome. Positions that are not quiet (in check, or with a capture or
//! promotion that wins material by static exchange) are skipped and counted.
use mshogi_search::tori::eval::ToriTexel;
#[path = "support/texel.rs"]
mod texel;

fn main() -> texel::Result<()> {
    texel::main::<ToriTexel>(3600)
}

#[cfg(test)]
mod tests {
    use mshogi_core::tori::{LocalPosition, Position, Side};
    use mshogi_search::alphabeta::SearchPosition;
    use mshogi_search::texel::TexelModel;
    use mshogi_search::tori::eval::{self, DEFAULT, INITIAL, ToriTexel};
    use mshogi_search::tori::position::ToriPosition;

    fn positions() -> Vec<Position> {
        let mut local = LocalPosition::admit(Position::standard()).unwrap();
        let mut out = vec![local.position().clone()];
        for ply in 0..400 {
            let actions = local.legal_actions();
            if actions.is_empty() {
                break;
            }
            local.apply(actions[(ply * 7) % actions.len()]).unwrap();
            out.push(local.position().clone());
        }
        out
    }

    #[test]
    fn fitting_score_is_the_search_evaluation_from_sentes_view() {
        let mut fitted = 0;
        for rules_position in positions() {
            let text = rules_position.compact_diagram().to_string();
            let Some(prepared) = ToriTexel::prepare(&text).unwrap() else {
                continue;
            };
            fitted += 1;
            let p = ToriPosition::from_position(&rules_position);
            let sign = if p.side_to_move() == Side::Sente {
                1
            } else {
                -1
            };
            assert_eq!(ToriTexel::score(&prepared, &DEFAULT), sign * p.evaluate());
            assert_eq!(
                ToriTexel::score(&prepared, &INITIAL),
                sign * eval::score(&p, &INITIAL)
            );
        }
        assert!(fitted > 10, "{fitted}");
    }

    #[test]
    fn checks_and_winning_captures_are_not_quiet() {
        let row = |board: &str, side: &str| {
            format!(
                r#"{{"board":[{board}],"hands":{{"s":{{}},"g":{{}}}},"side_to_move":"{side}"}}"#
            )
        };
        // A Sente Crane on 4d can take an undefended Gote Falcon on 4c.
        let capture = row(r#""g:Ph:1a","g:Fa:4c","s:Cr:4d","s:Ph:7g""#, "s");
        assert!(ToriTexel::prepare(&capture).unwrap().is_none());
        // Gote's Phoenix on 4a is in check from the Sente Crane on 4b.
        let check = row(r#""g:Ph:4a","s:Cr:4b","s:Ph:7g""#, "g");
        assert!(ToriTexel::prepare(&check).unwrap().is_none());
        let quiet = row(r#""g:Ph:1a","s:Cr:4d","s:Ph:7g""#, "g");
        assert!(ToriTexel::prepare(&quiet).unwrap().is_some());
        assert!(ToriTexel::prepare("not a diagram").is_err());
    }
}
