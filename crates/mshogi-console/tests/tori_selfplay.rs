//! Reproducible bounded real-subprocess Tori self-play, not strength measurement.
mod support;
use mshogi_core::{native::CanonicalRecord, tori as t};
use mshogi_engine::*;
const NODES: u64 = 1_024;
fn bounded(game: t::Game, maximum_plies: usize) -> support::selfplay::Trace<t::Action> {
    support::selfplay::bounded::<game::Tori>(
        game,
        maximum_plies,
        WorkLimits {
            time_ms: None,
            depth: 1,
            nodes: NODES,
        },
        |depth| assert_eq!(depth, 1),
    )
}
#[test]
fn actual_standard_selfplay_is_deterministic_bounded_and_unscored_at_work_stop() {
    let first = bounded(t::Game::standard(), 8);
    let second = bounded(t::Game::standard(), 8);
    assert_eq!(first, second);
    assert_eq!(first.actions.len(), 8);
    let CanonicalRecord::Tori(record) = CanonicalRecord::from_json(&first.record).unwrap() else {
        panic!()
    };
    assert_eq!(record.known_board_actions(), 8);
    assert_eq!(record.event_count(), 8);
    assert!(record.conclusion().is_none());
    println!(
        "TORI-SELFPLAY-STANDARD {}",
        serde_json::json!({"command":"mshogi internal-search","limits":{"depth":1,"nodes":256},"maximum_plies":8,"record":first.record,"observations":first.observations})
    );
}

fn mate_root() -> t::Game {
    // TORI-O-MATE's independently reviewed terminal diagram, one predecessor
    // earlier. Ea4c-4b checks 4a; Pt4d protects 4b; native Gote 3a/5a/3b/5b
    // blockers leave no king escape or defense. No evaluator expectation oracle.
    let pieces = [
        ("7g", t::Side::Sente, t::Face::Phoenix),
        ("4a", t::Side::Gote, t::Face::Phoenix),
        ("4d", t::Side::Sente, t::Face::Pheasant),
        ("3a", t::Side::Gote, t::Face::Swallow),
        ("5a", t::Side::Gote, t::Face::Swallow),
        ("3b", t::Side::Gote, t::Face::Swallow),
        ("5b", t::Side::Gote, t::Face::Swallow),
        ("4c", t::Side::Sente, t::Face::Eagle),
    ];
    t::Game::from_root(
        t::Position::from_diagram(
            &pieces.map(|(s, side, face)| (s.parse().unwrap(), t::Piece { side, face })),
            [t::Hand::empty(); 2],
            t::Side::Sente,
            t::RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap()
}
#[test]
fn actual_complete_custom_game_reaches_independently_expected_native_checkmate() {
    let first = bounded(mate_root(), 4);
    let second = bounded(mate_root(), 4);
    assert_eq!(first, second);
    assert_eq!(
        first.actions,
        [t::Action::Move {
            from: "4c".parse().unwrap(),
            to: "4b".parse().unwrap(),
            result_face: t::Face::Eagle
        }]
    );
    let CanonicalRecord::Tori(record) = CanonicalRecord::from_json(&first.record).unwrap() else {
        panic!()
    };
    assert_eq!(
        record.conclusion(),
        Some(t::Conclusion::Checkmate {
            winner: t::Side::Sente
        })
    );
    assert_eq!(record.known_board_actions(), 1);
    assert_eq!(record.event_count(), 1);
    println!(
        "TORI-SELFPLAY-COMPLETE {}",
        serde_json::json!({"command":"mshogi internal-search","limits":{"depth":1,"nodes":256},"maximum_plies":4,"record":first.record,"observations":first.observations})
    );
}
