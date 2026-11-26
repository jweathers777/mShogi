//! Reproducible bounded real-subprocess Tenjiku self-play, not strength measurement.
mod support;
use mshogi_core::{native::CanonicalRecord, tenjiku as t};
use mshogi_engine::*;
fn bounded(game: t::Game, maximum_plies: usize) -> support::selfplay::Trace<t::Action> {
    support::selfplay::bounded::<game::Tenjiku>(
        game,
        maximum_plies,
        game::Tenjiku::PLAY_LIMITS,
        |depth| assert!(depth >= 1),
    )
}
// Debug exercises actual subprocess/replay behavior on an independent corner
// diagram. Mandatory release execution on all three platforms exercises the
// full 156-piece standard root; neither path is skipped or called the other.
fn smoke_root(profile: t::Profile) -> t::Game {
    if !cfg!(debug_assertions) {
        return t::Game::standard(profile);
    }
    let pieces = [
        (16, 16, t::Side::Sente, t::Base::K),
        (16, 14, t::Side::Sente, t::Base::P),
        (1, 1, t::Side::Gote, t::Base::K),
        (1, 3, t::Side::Gote, t::Base::P),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (f, r, side, base))| {
        (
            square(f, r),
            t::Piece::new(id as u16, side, base, false).unwrap(),
        )
    })
    .collect::<Vec<_>>();
    t::Game::from_root(
        t::Position::from_diagram(
            profile,
            &pieces,
            t::Side::Sente,
            t::RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap()
}
#[test]
fn actual_selfplay_is_deterministic_bounded_and_unscored_at_work_stop() {
    for profile in t::Profile::ALL {
        let first = bounded(smoke_root(profile), 4);
        let second = bounded(smoke_root(profile), 4);
        assert_eq!(first, second);
        assert_eq!(first.actions.len(), 4);
        let CanonicalRecord::Tenjiku(record) = CanonicalRecord::from_json(&first.record).unwrap()
        else {
            panic!()
        };
        assert_eq!(record.profile(), profile);
        assert_eq!(
            record
                .to_game()
                .unwrap()
                .native_root()
                .board()
                .iter()
                .flatten()
                .count(),
            if cfg!(debug_assertions) { 4 } else { 156 }
        );
        assert_eq!(
            record.to_game().unwrap().position().root().kind(),
            if cfg!(debug_assertions) {
                t::RootKind::Diagram
            } else {
                t::RootKind::Standard
            }
        );
        assert_eq!(record.known_board_actions(), 4);
        assert_eq!(record.event_count(), 4);
        assert!(record.conclusion().is_none());
        println!(
            "TENJIKU-SELFPLAY-SMOKE {}",
            serde_json::json!({"profile":profile.id(),"root":if cfg!(debug_assertions){"independent_corner_diagram"}else{"standard"},"command":"mshogi internal-search","limits":game::Tenjiku::PLAY_LIMITS,"maximum_plies":4,"record":first.record,"observations":first.observations})
        );
    }
}
fn square(f: u8, r: u8) -> t::Square {
    t::Square::new(f, r).unwrap()
}
fn royal_root(profile: t::Profile) -> t::Game {
    // Source-derived Lion double capture: the two opponent royals are adjacent
    // consecutive landings. Capturing both ends the actual game in one action;
    // capturing just one cannot establish RoyalCapture while the other survives.
    let pieces = [
        (16, 16, t::Side::Sente, t::Base::K, false),
        (16, 14, t::Side::Sente, t::Base::P, false),
        (8, 8, t::Side::Sente, t::Base::Ln, false),
        (8, 7, t::Side::Gote, t::Base::K, false),
        (8, 6, t::Side::Gote, t::Base::DE, true),
        (1, 3, t::Side::Gote, t::Base::P, false),
    ]
    .into_iter()
    .enumerate()
    .map(|(id, (f, r, side, base, promoted))| {
        (
            square(f, r),
            t::Piece::new(id as u16, side, base, promoted).unwrap(),
        )
    })
    .collect::<Vec<_>>();
    t::Game::from_root(
        t::Position::from_diagram(
            profile,
            &pieces,
            t::Side::Sente,
            t::RootMetadata::diagram(0),
        )
        .unwrap(),
    )
    .unwrap()
}
#[test]
fn actual_complete_custom_game_captures_both_independently_expected_native_royals() {
    for profile in t::Profile::ALL {
        let first = bounded(royal_root(profile), 4);
        let second = bounded(royal_root(profile), 4);
        assert_eq!(first, second);
        assert_eq!(
            first.actions,
            vec![t::Action {
                kind: t::ActionKind::Travel,
                from: square(8, 8),
                route: vec![square(8, 7), square(8, 6)],
                promote: false,
                target: None
            }]
        );
        let CanonicalRecord::Tenjiku(record) = CanonicalRecord::from_json(&first.record).unwrap()
        else {
            panic!()
        };
        assert_eq!(record.profile(), profile);
        assert_eq!(
            record.conclusion(),
            Some(t::Conclusion::RoyalCapture {
                winner: t::Side::Sente
            })
        );
        assert_eq!(record.known_board_actions(), 1);
        assert_eq!(record.event_count(), 1);
        assert_eq!(first.observations.len(), 2);
        println!(
            "TENJIKU-SELFPLAY-COMPLETE {}",
            serde_json::json!({"profile":profile.id(),"command":"mshogi internal-search","limits":game::Tenjiku::PLAY_LIMITS,"maximum_plies":4,"record":first.record,"observations":first.observations})
        );
    }
}
