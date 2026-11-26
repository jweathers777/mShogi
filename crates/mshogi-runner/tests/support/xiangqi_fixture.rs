//! Test-only protocol engine.
mod protocol_fixture;
fn main() {
    protocol_fixture::serve(
        "uci",
        "id name mshogi-Xiangqi-UCI0-9-verification-fixture\noption name Nodes type spin default 128 min 0 max 512\noption name RulesProfile type combo default mshogi.xiangqi.wxf2018.v1 var mshogi.xiangqi.wxf2018.v1\nuciok",
        "a3a4",
        "c3c4",
        "a0a9",
    );
}
