//! Fixed-input Chu search timing and coarse cost attribution. Run in the cloud.
//! chu_bench prepare OPENINGS TACTICS REPORT OUT
//! chu_bench bench INPUT OUT DEPTH REPEATS
use mshogi_core::{chu::GameRecord, native::CanonicalRecord};
use mshogi_search::{
    alphabeta::{Limits, Searcher, Status},
    chu::position::ChuPosition,
};
use serde_json::{Value, json};
use std::{fs, time::Instant};
#[path = "support/search_profile.rs"]
mod search_profile;

fn read(path: &str) -> Value {
    serde_json::from_str(&fs::read_to_string(path).unwrap()).unwrap()
}
fn write(path: &str, value: &Value) {
    use std::io::Write;
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap()
        .write_all(serde_json::to_string_pretty(value).unwrap().as_bytes())
        .unwrap();
}
fn prepare(openings: &str, tactics: &str, report: &str) -> Value {
    let mut rows = Vec::new();
    for row in read(openings)["openings"]
        .as_array()
        .unwrap()
        .iter()
        .take(8)
    {
        rows.push(json!({"id":row["id"],"record":row["record"]}));
    }
    let suite = read(tactics);
    for family in 0..4 {
        let row = suite["positions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["family"] == family)
            .unwrap();
        rows.push(json!({"id":row["id"],"record":row["record"]}));
    }
    for (name, base) in [
        ("horned-falcon", mshogi_core::chu::Base::DragonHorse),
        ("soaring-eagle", mshogi_core::chu::Base::DragonKing),
    ] {
        use mshogi_core::chu::{Base, Game, Piece, Position, RootMetadata, Side};
        let pieces: Vec<_> = [
            ("12l", Side::Sente, Base::King, false),
            ("1a", Side::Gote, Base::King, false),
            ("6h", Side::Sente, base, true),
            ("7e", Side::Gote, base, true),
            ("5g", Side::Sente, Base::Lion, false),
            ("8f", Side::Gote, Base::Lion, false),
            ("6g", Side::Gote, Base::Pawn, false),
            ("7f", Side::Sente, Base::Pawn, false),
        ]
        .into_iter()
        .map(|(at, side, base, promoted)| {
            (
                at.parse().unwrap(),
                Piece::new(side, base, promoted).unwrap(),
            )
        })
        .collect();
        let game = Game::from_root(
            Position::from_diagram(&pieces, Side::Sente, RootMetadata::diagram(0)).unwrap(),
        )
        .unwrap();
        assert!(game.conclusion().is_none());
        rows.push(json!({"id":name,"record":GameRecord::from_game(&game).unwrap()}));
    }
    let report = mshogi_runner::experiments::ValidatedReport::from_json(
        &fs::read_to_string(report).unwrap(),
    )
    .unwrap();
    let CanonicalRecord::Chu(record) = &report.games[0].final_root.record else {
        panic!("Chu report required")
    };
    let game = record.to_game().unwrap();
    for count in [80, 160, 240, 320] {
        let record = game.record_prefix(count).unwrap();
        assert!(record.to_game().unwrap().conclusion().is_none());
        rows.push(json!({"id":format!("native-game-prefix-{count}"),"record":record}));
    }
    json!({"name":"chu-speed-v1","selection":"first8 frozen openings, first case of each of4 tactical families, symmetric Horned Falcon/Soaring Eagle diagrams, first source game prefixes80/160/240/320; fixed before optimization", "positions":rows})
}
fn bench(input: &str, depth: u16, repeats: u32) -> Value {
    assert!(repeats > 0);
    let spec = read(input);
    let limits = Limits {
        depth,
        nodes: Some(5_000_000),
        soft: None,
        hard: None,
        clock: false,
    };
    let mut rows = Vec::new();
    for row in spec["positions"].as_array().unwrap() {
        let game = serde_json::from_value::<GameRecord>(row["record"].clone())
            .unwrap()
            .to_game()
            .unwrap();
        let at = Instant::now();
        let mut root = ChuPosition::from_game(&game);
        root.restrict_root(&game.legal_actions());
        let setup_ns = at.elapsed().as_nanos();
        let mut rounds = Vec::new();
        let mut expected = None;
        for _ in 0..repeats {
            let mut position = root.clone();
            let mut engine = Searcher::new(16);
            let at = Instant::now();
            let result = engine.search(&mut position, limits, &mut || false);
            let nanos = at.elapsed().as_nanos();
            assert_eq!(result.status, Status::Complete, "incomplete {}", row["id"]);
            assert_eq!(result.depth, depth);
            if let Some(previous) = &expected {
                assert_eq!(previous, &result);
            }
            rounds.push(json!({"nanos":nanos,"nps":result.nodes as f64 * 1e9 / nanos as f64}));
            expected = Some(result);
        }
        let mut profiled = search_profile::Profiled::new(root);
        let mut engine = Searcher::new(16);
        let at = Instant::now();
        let result = engine.search(&mut profiled, limits, &mut || false);
        let profile_ns = at.elapsed().as_nanos();
        assert_eq!(expected.as_ref(), Some(&result), "profiling changed search");
        let categories: Vec<_> = [
            "generation_and_route_admission",
            "make_and_threat",
            "unmake",
            "rule_and_capture_threat",
            "exchange",
        ]
        .iter()
        .zip(profiled.totals.into_inner())
        .map(|(name, (calls, nanos))| json!({"name":name,"calls":calls,"nanos":nanos}))
        .collect();
        rows.push(json!({"id":row["id"],"setup_ns":setup_ns,"rounds":rounds,"signature":format!("{result:?}"),"nodes":result.nodes,"profile_ns":profile_ns,"categories":categories}));
    }
    json!({"name":spec["name"],"depth":depth,"repeats":repeats,"rows":rows,"note":"Uninstrumented search timing excludes native setup; profile includes timer overhead and attributes whole trait calls, not nested subcalls."})
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "prepare" => write(&a[5], &prepare(&a[2], &a[3], &a[4])),
        "bench" => write(
            &a[3],
            &bench(&a[2], a[4].parse().unwrap(), a[5].parse().unwrap()),
        ),
        _ => panic!("use prepare OPENINGS TACTICS REPORT OUT or bench INPUT OUT DEPTH REPEATS"),
    }
}
