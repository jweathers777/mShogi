//! Fit Chu material and placement using outcome JSON diagrams.
use mshogi_core::chu::{Face, LocalPosition, Position, RootMetadata};
use mshogi_search::{
    chu::{eval, position::ChuPosition},
    texel::TexelModel,
};
use std::sync::LazyLock;
#[path = "support/texel.rs"]
mod texel;

struct Prepared {
    features: Vec<(usize, i32)>,
    pieces: usize,
}
struct ChuTexel;
static NAMES: LazyLock<Vec<String>> = LazyLock::new(|| {
    Face::ALL
        .into_iter()
        .flat_map(|f| ["material", "advance", "central"].map(|term| format!("{}_{term}", f.code())))
        .chain(std::iter::once("tempo".to_string()))
        .collect()
});
static NAME_REFS: LazyLock<Vec<&str>> =
    LazyLock::new(|| NAMES.iter().map(String::as_str).collect());
fn local(text: &str) -> Result<LocalPosition, String> {
    let v: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let pieces: Vec<(mshogi_core::chu::Square, mshogi_core::chu::Piece)> =
        serde_json::from_value(v["pieces"].clone()).map_err(|e| e.to_string())?;
    let side = serde_json::from_value(v["side"].clone()).map_err(|e| e.to_string())?;
    let protection = serde_json::from_value(v["protection"].clone()).map_err(|e| e.to_string())?;
    let position = Position::from_diagram(&pieces, side, RootMetadata::diagram(0))
        .map_err(|e| e.to_string())?;
    LocalPosition::from_parts(position, protection).map_err(|e| e.to_string())
}
impl TexelModel for ChuTexel {
    type Prepared = Prepared;
    const KIND: &'static str = "chu";
    const POSITION_FIELD: &'static str = "diagram";
    const K_RANGE: (f64, f64) = (0.01, 0.71);
    const STEPS: &'static [i32] = &[64, 32, 16, 8, 4, 2, 1];
    fn names() -> &'static [&'static str] {
        &NAME_REFS
    }
    fn initial() -> Vec<i32> {
        eval::INITIAL.to_vec()
    }
    fn lower() -> Vec<i32> {
        bounds(false)
    }
    fn upper() -> Vec<i32> {
        bounds(true)
    }
    fn key(text: &str) -> String {
        // Sort the board so tuple order cannot disguise a duplicate.
        let Ok(mut v) = serde_json::from_str::<serde_json::Value>(text) else {
            return text.to_string();
        };
        if let Some(p) = v
            .get_mut("pieces")
            .and_then(serde_json::Value::as_array_mut)
        {
            p.sort_by_key(|p| p[0].to_string());
        }
        v.to_string()
    }
    fn prepare(text: &str) -> Result<Option<Prepared>, String> {
        let local = local(text)?;
        let p = ChuPosition::from_local(&local);
        if !eval::quiet(&p) {
            return Ok(None);
        }
        Ok(Some(Prepared {
            features: eval::features(&p)
                .into_iter()
                .enumerate()
                .filter(|(_, v)| *v != 0)
                .collect(),
            pieces: local.position().pieces().count(),
        }))
    }
    fn score(p: &Prepared, weights: &[i32]) -> i32 {
        p.features.iter().map(|&(i, v)| weights[i] * v).sum()
    }
    fn buckets() -> &'static [&'static str] {
        &["pieces_1_20", "pieces_21_60", "pieces_61_plus"]
    }
    fn bucket(p: &Prepared) -> usize {
        match p.pieces {
            0..=20 => 0,
            21..=60 => 1,
            _ => 2,
        }
    }
}
fn bounds(upper: bool) -> Vec<i32> {
    let mut out = vec![if upper { 32 } else { -32 }; eval::COUNT];
    for face in Face::ALL {
        let i = eval::index(face);
        let fixed = matches!(face, Face::Pawn | Face::King | Face::Prince);
        out[i] = if fixed {
            eval::INITIAL[i]
        } else if upper {
            eval::INITIAL[i] * 2
        } else {
            eval::INITIAL[i] / 2
        };
    }
    out[eval::COUNT - 1] = if upper { 64 } else { 0 };
    out
}
fn main() -> texel::Result<()> {
    texel::main::<ChuTexel>(1800)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mshogi_core::chu::Side;
    use mshogi_search::alphabeta::SearchPosition;
    #[test]
    fn diagrams_prepare_exact_scores_and_canonical_keys() {
        let local = LocalPosition::standard();
        let p = ChuPosition::from_local(&local);
        let mut diagram = serde_json::json!({
            "pieces":local.position().pieces().collect::<Vec<_>>(),
            "side":local.position().side_to_move(), "protection":local.protection()
        });
        let text = diagram.to_string();
        let prepared = ChuTexel::prepare(&text)
            .unwrap()
            .expect("quiet standard root");
        assert_eq!(ChuTexel::score(&prepared, &eval::DEFAULT), p.evaluate());
        diagram["pieces"].as_array_mut().unwrap().reverse();
        assert_eq!(ChuTexel::key(&text), ChuTexel::key(&diagram.to_string()));
        diagram["side"] = serde_json::to_value(Side::Gote).unwrap();
        let prepared = ChuTexel::prepare(&diagram.to_string()).unwrap().unwrap();
        assert_eq!(
            ChuTexel::score(&prepared, &eval::DEFAULT),
            -eval::DEFAULT[eval::COUNT - 1]
        );
    }
    #[test]
    fn features_match_incremental_score_and_undo() {
        let mut rules_position = LocalPosition::standard();
        let mut captures = 0;
        let mut promotions = 0;
        for ply in 0..200 {
            let mut p = ChuPosition::from_local(&rules_position);
            let score = |p: &ChuPosition| {
                eval::features(p)
                    .iter()
                    .zip(eval::DEFAULT)
                    .map(|(v, w)| v * w)
                    .sum::<i32>()
            };
            let sign = if p.side_to_move() == Side::Sente {
                1
            } else {
                -1
            };
            assert_eq!(score(&p), sign * p.evaluate());
            let before = score(&p);
            for m in p.legal_moves() {
                let action = p.to_action(m);
                promotions += usize::from(action.promote);
                captures += usize::from(action.route.iter().any(|&at| {
                    p.piece(at)
                        .is_some_and(|piece| piece.side != p.side_to_move())
                }));
                assert!(p.make(m));
                assert_eq!(score(&p), -sign * p.evaluate());
                p.unmake();
                assert_eq!(score(&p), before);
            }
            let actions = rules_position.legal_actions();
            if actions.is_empty() {
                break;
            }
            rules_position
                .apply(&actions[(ply * 17) % actions.len()])
                .unwrap();
        }
        assert!(captures > 0, "capture coverage");
        assert!(promotions > 0, "promotion coverage");
    }
    #[test]
    fn initial_placement_matches_previous_formula_for_every_piece_and_square() {
        use mshogi_core::chu::{Base, Piece, Square};
        for base in Base::ALL {
            for promoted in [false, true] {
                for side in [Side::Sente, Side::Gote] {
                    let Some(piece) = Piece::new(side, base, promoted) else {
                        continue;
                    };
                    for rank in 1..=12 {
                        for file in 1..=12 {
                            let at = Square::new(file, rank).unwrap();
                            let advance = i32::from(if side == Side::Sente {
                                12 - rank
                            } else {
                                rank - 1
                            });
                            let central =
                                i32::from(5 - (file - 1).abs_diff(5).min((file - 1).abs_diff(6)));
                            let old = match piece.face() {
                                Face::Pawn | Face::GoBetween => 4 * advance,
                                Face::Lion => 8 * central + 3 * advance.min(7),
                                Face::Kirin | Face::Phoenix => 4 * central + 2 * advance,
                                Face::King | Face::Prince => 0,
                                _ => central + advance,
                            };
                            let (a, c) = eval::coordinates(piece, at);
                            let i = eval::index(piece.face());
                            assert_eq!(eval::INITIAL[i + 1] * a + eval::INITIAL[i + 2] * c, old);
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn face_indices_and_initial_score() {
        for (i, face) in Face::ALL.into_iter().enumerate() {
            assert_eq!(eval::index(face), i * 3);
        }
        let p = ChuPosition::from_position(&Position::standard());
        assert_eq!(
            eval::features(&p)
                .iter()
                .zip(eval::INITIAL)
                .map(|(v, w)| v * w)
                .sum::<i32>(),
            10
        );
        assert!(ChuTexel::prepare("bad").is_err());
    }
}
