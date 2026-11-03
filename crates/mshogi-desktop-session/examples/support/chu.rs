use mshogi_core::chu as c;
use serde_json::Value;
pub fn fixture(id: &str) -> Value {
    let v: Value =
        serde_json::from_str(include_str!("../../../../docs/m4/chu/fixtures.json")).unwrap();
    v["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == id)
        .unwrap()
        .clone()
}
pub fn root(v: &Value) -> c::Game {
    if v == "standard_start" {
        return c::Game::standard();
    }
    let board: Vec<_> = v["board"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            (
                serde_json::from_value(p["at"].clone()).unwrap(),
                c::Piece::from_code(
                    serde_json::from_value(p["side"].clone()).unwrap(),
                    p["piece"].as_str().unwrap(),
                )
                .unwrap(),
            )
        })
        .collect();
    c::Game::from_root(
        c::Position::from_diagram(
            &board,
            serde_json::from_value(v["turn"].clone()).unwrap(),
            c::RootMetadata::diagram(v["declared_board_plies"].as_u64().unwrap()),
        )
        .unwrap(),
    )
    .unwrap()
}
#[allow(dead_code)] // Also compiled for QA roots, where no scripted action is needed.
pub fn action(v: &Value) -> c::Action {
    c::Action {
        from: serde_json::from_value(v["from"].clone()).unwrap(),
        route: serde_json::from_value(v["route"].clone()).unwrap(),
        promote: v["promote"].as_bool().unwrap(),
    }
}
pub fn contract_game() -> c::Game {
    root(&fixture("CHU-RETURN-Ln-enemy")["given"]["root"])
}
