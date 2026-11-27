//! Fit the classical Tenjiku material, placement and tempo terms.
use mshogi_core::tenjiku::{Face, Piece, Position, Profile, RootMetadata, Side, Square};
use mshogi_search::{
    tenjiku::{eval, position::TenjikuPosition},
    texel::TexelModel,
};
use serde_json::Value;
use std::sync::LazyLock;
#[path = "support/texel.rs"]
mod texel;

struct Tenjiku;
struct Prepared {
    features: Vec<(usize, i32)>,
    profile: Profile,
}
static NAMES: LazyLock<Vec<String>> = LazyLock::new(|| {
    Face::ALL
        .into_iter()
        .map(|face| format!("{}_material", face.code()))
        .chain(
            [
                "pawn_dog_advance",
                "lion_hawk_demon_advance",
                "lion_hawk_demon_central",
                "other_central",
                "tempo",
            ]
            .map(str::to_owned),
        )
        .collect()
});
static NAME_REFS: LazyLock<Vec<&str>> =
    LazyLock::new(|| NAMES.iter().map(String::as_str).collect());
fn bounds(upper: bool) -> Vec<i32> {
    let mut out = eval::initial();
    for face in Face::ALL {
        if !matches!(face, Face::P | Face::K | Face::CP) {
            let v = &mut out[face as usize];
            *v = if upper { *v * 2 } else { *v / 2 };
        }
    }
    out[Face::ALL.len()..eval::COUNT - 1].fill(if upper { 32 } else { -32 });
    out[eval::COUNT - 1] = if upper { 64 } else { 0 };
    out.to_vec()
}
impl TexelModel for Tenjiku {
    type Prepared = Prepared;
    const KIND: &'static str = "tenjiku";
    const POSITION_FIELD: &'static str = "diagram";
    const K_RANGE: (f64, f64) = (0.01, 0.71);
    const STEPS: &'static [i32] = &[64, 32, 16, 8, 4, 2, 1];
    fn names() -> &'static [&'static str] {
        &NAME_REFS
    }
    fn initial() -> Vec<i32> {
        eval::initial().to_vec()
    }
    fn lower() -> Vec<i32> {
        bounds(false)
    }
    fn upper() -> Vec<i32> {
        bounds(true)
    }
    fn key(text: &str) -> String {
        let Ok(mut v) = serde_json::from_str::<Value>(text) else {
            return text.to_string();
        };
        if let Some(pieces) = v["pieces"].as_array_mut() {
            for piece in pieces.iter_mut() {
                if let Some(fields) = piece[1].as_object_mut() {
                    fields.remove("id");
                }
            }
            pieces.sort_by_cached_key(|p| p[0].to_string());
        }
        v.to_string()
    }
    fn prepare(text: &str) -> Result<Option<Prepared>, String> {
        let v: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
        let pending: Option<Side> =
            serde_json::from_value(v.get("pending_bare").ok_or("missing pending_bare")?.clone())
                .map_err(|e| e.to_string())?;
        let pieces: Vec<(Square, Piece)> =
            serde_json::from_value(v["pieces"].clone()).map_err(|e| e.to_string())?;
        let side = serde_json::from_value(v["side"].clone()).map_err(|e| e.to_string())?;
        let profile = serde_json::from_value(v["profile"].clone()).map_err(|e| e.to_string())?;
        let board = Position::from_diagram(profile, &pieces, side, RootMetadata::diagram(0))
            .map_err(|e| e.to_string())?;
        if pending.is_some() {
            return Ok(None);
        }
        let p = TenjikuPosition::from_position(&board);
        if !eval::quiet(&p) {
            return Ok(None);
        }
        Ok(Some(Prepared {
            features: eval::features(&p)
                .into_iter()
                .enumerate()
                .filter(|(_, v)| *v != 0)
                .collect(),
            profile,
        }))
    }
    fn score(p: &Prepared, weights: &[i32]) -> i32 {
        p.features.iter().map(|&(i, v)| weights[i] * v).sum()
    }
    fn buckets() -> &'static [&'static str] {
        &["adams", "muller"]
    }
    fn bucket(p: &Prepared) -> usize {
        match p.profile {
            Profile::AdamsModern => 0,
            Profile::MullerCompleted => 1,
        }
    }
}
fn main() -> texel::Result<()> {
    texel::main::<Tenjiku>(900)
}

#[cfg(test)]
mod tests {
    use super::*;
    use mshogi_core::tenjiku::Base;
    use serde_json::json;

    fn diagram(profile: Profile, side: Side) -> Value {
        let pieces = [
            (1, 16, Side::Sente, Base::K),
            (2, 13, Side::Sente, Base::P),
            (16, 1, Side::Gote, Base::K),
            (15, 4, Side::Gote, Base::P),
        ]
        .into_iter()
        .enumerate()
        .map(|(id, (file, rank, owner, base))| {
            (
                Square::new(file, rank).unwrap(),
                Piece::new(id as u16 + 1, owner, base, false).unwrap(),
            )
        })
        .collect::<Vec<_>>();
        json!({"pieces":pieces, "side":side,"pending_bare":null,"profile":profile})
    }
    #[test]
    fn one_quiet_diagram_per_profile_and_turn_matches_search_tempo() {
        for profile in Profile::ALL {
            for (side, tempo) in [(Side::Sente, 10), (Side::Gote, -10)] {
                let mut d = diagram(profile, side);
                let p = Tenjiku::prepare(&d.to_string()).unwrap().unwrap();
                assert_eq!(p.profile, profile);
                assert_eq!(Tenjiku::score(&p, &Tenjiku::initial()), tempo);
                let key = Tenjiku::key(&d.to_string());
                d["pieces"][0][1]["id"] = json!(99);
                d["pieces"].as_array_mut().unwrap().reverse();
                assert_eq!(key, Tenjiku::key(&d.to_string()));
                d["pending_bare"] = json!(side);
                assert!(Tenjiku::prepare(&d.to_string()).unwrap().is_none());
                assert_ne!(key, Tenjiku::key(&d.to_string()));
            }
        }
    }
    #[test]
    fn excludes_royal_threats_and_winning_captures() {
        for profile in Profile::ALL {
            let mut d = diagram(profile, Side::Sente);
            d["pieces"][3][0] = json!([2, 12]);
            assert!(Tenjiku::prepare(&d.to_string()).unwrap().is_none());
            d["pieces"][3][0] = json!([1, 15]);
            assert!(Tenjiku::prepare(&d.to_string()).unwrap().is_none());
        }
    }
}
