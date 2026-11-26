//! Test-only protocol engine.
mod protocol_fixture;
fn main() {
    protocol_fixture::serve(
        "usi",
        "id name mshogi-USI-verification-fixture\noption name Nodes type spin default 5000 min 0 max 20000\noption name RulesProfile type combo default mshogi.shogi.csa36.v1 var mshogi.shogi.csa36.v1\nusiok",
        "7g7f",
        "2g2f",
        "9a9b",
    );
}
