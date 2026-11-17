//! Bounded Texel fitting of the exact cached production evaluation.
//! fit TRAIN VALIDATION OUTPUT [SECONDS]; check FROZEN HELDOUT OUTPUT.
//! Held-out input is accepted only by the separate check command.
//! The fitting procedure is the shared `mshogi_search::texel` tool.
use mshogi_search::chess_eval::weights::ChessTexel;
#[path = "support/texel.rs"]
mod texel;
use texel::Result;

fn main() -> Result<()> {
    texel::main::<ChessTexel>(600)
}

#[cfg(test)]
mod tests {
    use mshogi_search::chess_eval::weights::{ChessTexel, INITIAL};
    use crate::texel::{penalty, probability};
    fn squared_error(score: i32, result: f64, k: f64) -> f64 {
        (probability(score, k) - result).powi(2)
    }
    #[test]
    fn sigmoid_and_error_have_independent_expected_values() {
        assert_eq!(probability(0, 1.0), 0.5);
        assert!((probability(400, 1.0) - 10.0 / 11.0).abs() < 1e-12);
        assert!((probability(-400, 1.0) - 1.0 / 11.0).abs() < 1e-12);
        assert_eq!(squared_error(0, 0.0, 1.0), 0.25);
        assert_eq!(squared_error(0, 0.5, 1.0), 0.0);
        assert!((probability(117, 0.8) + probability(-117, 0.8) - 1.0).abs() < 1e-12);
        assert_eq!(penalty::<ChessTexel>(&INITIAL.0), 0.0);
    }
}

#[cfg(test)]
mod input_tests {
    use mshogi_search::chess_eval::weights::{ChessTexel, INITIAL, NAMES, UPPER};
    use crate::texel::{frozen, load};
    use serde_json::json;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    fn temporary(label: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "msh168-texel-{label}-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ))
    }
    #[test]
    fn dataset_rejects_heldout_labels_bad_results_and_duplicate_boards() {
        let path = temporary("rows");
        let text = path.to_str().unwrap();
        let mut row = json!({"fen":"7k/8/8/8/3K4/8/8/8 w - - 0 1",
            "result":0.5,"split":"heldout","game_id":"g1"});
        std::fs::write(&path, format!("{row}\n")).unwrap();
        assert!(load::<ChessTexel>(text, "train").is_err());
        row["split"] = json!("train");
        row["result"] = json!(0.3);
        std::fs::write(&path, format!("{row}\n")).unwrap();
        assert!(load::<ChessTexel>(text, "train").is_err());
        row["result"] = json!(0.5);
        std::fs::write(&path, format!("{row}\n")).unwrap();
        assert_eq!(load::<ChessTexel>(text, "train").unwrap().len(), 1);
        let mut duplicate = row.clone();
        duplicate["fen"] = json!("7k/8/8/8/3K4/8/8/8 w - - 4 3");
        std::fs::write(&path, format!("{row}\n{duplicate}\n")).unwrap();
        assert!(load::<ChessTexel>(text, "train").is_err());
        row["fen"] = json!("not a FEN");
        std::fs::write(&path, format!("{row}\n")).unwrap();
        assert!(load::<ChessTexel>(text, "train").is_err());
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn frozen_weights_reject_wrong_names_bounds_and_calibration() {
        let path = temporary("weights");
        let text = path.to_str().unwrap();
        let valid = json!({"kind":"mshogi-chess-texel-fit","names":NAMES,
            "weights":INITIAL.0,"k":1.0});
        std::fs::write(&path, valid.to_string()).unwrap();
        assert_eq!(frozen::<ChessTexel>(text).unwrap().0, INITIAL.0.to_vec());
        let mut invalid = valid.clone();
        invalid["weights"][0] = json!(UPPER[0] + 1);
        std::fs::write(&path, invalid.to_string()).unwrap();
        assert!(frozen::<ChessTexel>(text).is_err());
        invalid = valid.clone();
        invalid["k"] = json!(0.0);
        std::fs::write(&path, invalid.to_string()).unwrap();
        assert!(frozen::<ChessTexel>(text).is_err());
        invalid = valid;
        invalid["names"][0] = json!("different");
        std::fs::write(&path, invalid.to_string()).unwrap();
        assert!(frozen::<ChessTexel>(text).is_err());
        std::fs::remove_file(path).unwrap();
    }
}
