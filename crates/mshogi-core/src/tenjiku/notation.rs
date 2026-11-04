//! Compact mShogi Tenjiku action notation, not historical notation or a record.
//!
//! Original code plus optional `+`, origin, each actual landing prefixed by `-`
//! or `x`, and optional final `^` for the requested flip. Stationary igui uses
//! `!x` and its exact target. Native derived burn/death phases remain separate.
use super::{
    Action, ActionKind, EffectPhase, Effects, Face, HistorySnapshot, LocalPosition, Piece,
    Position, Side,
};
use std::{error::Error, fmt};

/// Source-aligned English movement-face name for readable and spoken text.
pub fn face_name(face: Face) -> &'static str {
    match face {
        Face::K => "King",
        Face::FiD => "Fire Demon",
        Face::GGn => "Great General",
        Face::VGn => "Vice General",
        Face::RGn => "Rook General",
        Face::BGn => "Bishop General",
        Face::LHk => "Lion Hawk",
        Face::Ln => "Lion",
        Face::WBf => "Water Buffalo",
        Face::FEg => "Free Eagle",
        Face::FK => "Free King",
        Face::HT => "Heavenly Tetrarchs",
        Face::SE => "Soaring Eagle",
        Face::HF => "Horned Falcon",
        Face::ChS => "Chariot Soldier",
        Face::DK => "Dragon King",
        Face::DH => "Dragon Horse",
        Face::FO => "Flying Ox",
        Face::FB => "Free Boar",
        Face::R => "Rook",
        Face::B => "Bishop",
        Face::Ph => "Phoenix",
        Face::VSo => "Vertical Soldier",
        Face::VM => "Vertical Mover",
        Face::SSo => "Side Soldier",
        Face::SM => "Side Mover",
        Face::WH => "White Horse",
        Face::W => "Whale",
        Face::Ky => "Kirin",
        Face::CP => "Crown Prince",
        Face::FS => "Flying Stag",
        Face::RC => "Reverse Chariot",
        Face::L => "Lance",
        Face::MGn => "Multi General",
        Face::G => "Gold General",
        Face::DE => "Drunk Elephant",
        Face::BT => "Blind Tiger",
        Face::FL => "Ferocious Leopard",
        Face::S => "Silver General",
        Face::C => "Copper General",
        Face::I => "Iron General",
        Face::N => "Knight",
        Face::Dg => "Dog",
        Face::P => "Pawn",
    }
}

fn effect_piece(piece: Piece) -> String {
    let side = match piece.side {
        Side::Sente => "Sente",
        Side::Gote => "Gote",
    };
    if piece.promoted {
        format!(
            "{side} {} (promoted {})",
            face_name(piece.face()),
            face_name(piece.base.face())
        )
    } else {
        format!("{side} {}", face_name(piece.face()))
    }
}

/// Describe actual native effects in their supplied phase order, including
/// simultaneous burn batches and the original identity of promoted pieces.
/// This formatter neither resolves an action nor infers victims from a board.
pub fn format_effects(effects: &Effects) -> String {
    let mut parts = Vec::new();
    if let (ActionKind::Igui, Some(target)) = (effects.action.kind, effects.action.target) {
        parts.push(format!("Stationary capture at {target}"));
        parts.push(format!("stationary origin {}", effects.action.from));
    }
    for phase in &effects.phases {
        parts.push(match phase {
            EffectPhase::Capture { victim } => format!(
                "Captured {} at {}",
                effect_piece(victim.piece),
                victim.square
            ),
            EffectPhase::Burn { victims } => format!(
                "Burned together: {}",
                if victims.is_empty() {
                    "no pieces".into()
                } else {
                    victims
                        .iter()
                        .map(|v| format!("{} at {}", effect_piece(v.piece), v.square))
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            ),
            EffectPhase::MoverDies { square, piece } => {
                format!("{} destroyed at {square}", effect_piece(*piece))
            }
            EffectPhase::Promotion {
                square,
                before,
                after,
            } => format!(
                "Promoted {} to {} at {square}",
                effect_piece(*before),
                effect_piece(*after)
            ),
        });
    }
    if effects.phases.is_empty() {
        parts.push("No captures or other effects.".into());
    }
    parts.join("; ")
}

/// A human label cannot describe or identify a complete native action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NotationError {
    /// No current-owner original occupies the source.
    Source,
    /// Travel/igui fields do not form a complete native action shape.
    Shape,
    /// No complete native local choice has the exact supplied spelling.
    NoLocalChoice,
}
impl fmt::Display for NotationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tenjiku notation refused: {self:?}")
    }
}
impl Error for NotationError {}

/// Describe an admitted action against its exact preceding native prefix.
pub fn format_history_action(
    before: &HistorySnapshot,
    action: Action,
) -> Result<String, NotationError> {
    // Formatting an already admitted immutable snapshot requires its board,
    // not another LocalPosition admission or generation of legal continuations.
    let position = Position::install(before.state().clone()).map_err(|_| NotationError::Source)?;
    format_action(&position, action)
}

/// Describe the complete choice without granting native legality or permission.
pub fn format_action(position: &Position, action: Action) -> Result<String, NotationError> {
    let source = position
        .piece(action.from)
        .filter(|p| p.side == position.side_to_move())
        .ok_or(NotationError::Source)?;
    let mut label = format!(
        "{}{}{}",
        source.base.code(),
        if source.promoted { "+" } else { "" },
        action.from
    );
    match action.kind {
        ActionKind::Igui => {
            if !action.route.is_empty() || action.promote {
                return Err(NotationError::Shape);
            }
            let target = action.target.ok_or(NotationError::Shape)?;
            label.push_str(&format!("!x{target}"));
        }
        ActionKind::Travel => {
            if !(1..=3).contains(&action.route.len()) || action.target.is_some() {
                return Err(NotationError::Shape);
            }
            let mut removed = vec![action.from];
            for landing in action.route {
                let capture = !removed.contains(&landing) && position.piece(landing).is_some();
                label.push(if capture { 'x' } else { '-' });
                label.push_str(&landing.to_string());
                removed.push(landing);
            }
            if action.promote {
                label.push('^');
            }
        }
    }
    Ok(label)
}

/// Select a real complete native local choice; Game still owns history admission.
pub fn parse_local_action(local: &LocalPosition, text: &str) -> Result<Action, NotationError> {
    if text.is_empty() || text.len() > 32 || !text.is_ascii() {
        return Err(NotationError::NoLocalChoice);
    }
    local
        .legal_actions()
        .into_iter()
        .find(|a| format_action(local.position(), a.clone()).is_ok_and(|label| label == text))
        .ok_or(NotationError::NoLocalChoice)
}

/// Select a structural root's local choice, without inventing earlier game history.
pub fn parse_action(position: &Position, text: &str) -> Result<Action, NotationError> {
    let local = LocalPosition::admit(position.clone()).map_err(|_| NotationError::Source)?;
    parse_local_action(&local, text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tenjiku::{
        Base, Game, GameCommand, GameRequest, Piece, Profile, RootMetadata, Side, Square,
    };
    fn sq(f: u8, r: u8) -> Square {
        Square::new(f, r).unwrap()
    }
    fn local(profile: Profile, entries: &[(u8, u8, Side, Base, bool)]) -> LocalPosition {
        LocalPosition::admit(
            Position::from_diagram(
                profile,
                &entries
                    .iter()
                    .enumerate()
                    .map(|(id, &(f, r, side, base, promoted))| {
                        (
                            sq(f, r),
                            Piece::new(id as u16, side, base, promoted).unwrap(),
                        )
                    })
                    .collect::<Vec<_>>(),
                Side::Sente,
                RootMetadata::diagram(0),
            )
            .unwrap(),
        )
        .unwrap()
    }
    #[test]
    fn independently_specified_lion_routes_captures_and_returns_are_distinct() {
        for profile in Profile::ALL {
            let local = local(
                profile,
                &[
                    (8, 8, Side::Sente, Base::Ln, false),
                    (8, 7, Side::Gote, Base::S, false),
                    (8, 6, Side::Gote, Base::R, false),
                ],
            );
            for (label, route) in [
                ("Ln8hx8f", vec![sq(8, 6)]),
                ("Ln8hx8gx8f", vec![sq(8, 7), sq(8, 6)]),
                ("Ln8hx8g-8h", vec![sq(8, 7), sq(8, 8)]),
                ("Ln8h-7g-8h", vec![sq(7, 7), sq(8, 8)]),
            ] {
                let expected = Action {
                    kind: ActionKind::Travel,
                    from: sq(8, 8),
                    route,
                    promote: false,
                    target: None,
                };
                assert_eq!(
                    parse_local_action(&local, label),
                    Ok(expected.clone()),
                    "{profile:?} {label}"
                );
                assert_eq!(format_action(local.position(), expected).unwrap(), label);
            }
            for bad in [
                "Ln8h-8f",
                "Ln8h-8g-8h",
                "Ln8hx8gx8h",
                "ln8hx8f",
                "Ln8hx8f ",
                "Ln8h",
                "Ln8hx17f",
                "Ln8h!x8g",
            ] {
                assert!(parse_local_action(&local, bad).is_err(), "{bad}");
            }
        }
    }
    #[test]
    fn original_promoted_identity_igui_three_landings_and_flip_remain_explicit() {
        for profile in Profile::ALL {
            let local = local(
                profile,
                &[
                    (8, 8, Side::Sente, Base::ChS, true),
                    (9, 8, Side::Gote, Base::P, false),
                    (4, 8, Side::Sente, Base::VGn, false),
                    (12, 6, Side::Sente, Base::P, false),
                ],
            );
            let igui = Action::igui(sq(8, 8), sq(9, 8));
            assert_eq!(parse_local_action(&local, "ChS+8h!x9h"), Ok(igui.clone()));
            assert_eq!(format_action(local.position(), igui).unwrap(), "ChS+8h!x9h");
            let three = Action {
                kind: ActionKind::Travel,
                from: sq(4, 8),
                route: vec![sq(4, 7), sq(5, 7), sq(5, 6)],
                promote: false,
                target: None,
            };
            assert_eq!(parse_local_action(&local, "VGn4h-4g-5g-5f"), Ok(three));
            for label in ["P12f-12e", "P12f-12e^"] {
                let a = parse_local_action(&local, label).unwrap();
                assert_eq!(format_action(local.position(), a).unwrap(), label);
            }
            for bad in [
                "HT8h!x9h",
                "ChS8h!x9h",
                "ChS+8hx9h",
                "ChS+8h!x9h^",
                "VGn4hx4g-5g-5f",
                "P12f-12e+",
                "P12f-12e^^",
            ] {
                assert!(parse_local_action(&local, bad).is_err(), "{bad}");
            }
        }
    }
    #[test]
    fn admitted_history_preserves_capture_label_undo_and_fresh_permission() {
        for profile in Profile::ALL {
            let local = local(
                profile,
                &[
                    (16, 16, Side::Sente, Base::K, false),
                    (1, 1, Side::Gote, Base::K, false),
                    (8, 8, Side::Sente, Base::Ln, false),
                    (8, 7, Side::Gote, Base::S, false),
                    (3, 3, Side::Gote, Base::G, false),
                ],
            );
            let mut game = Game::from_root(local.position().clone()).unwrap();
            let before = game.context();
            let action = parse_local_action(game.local_position(), "Ln8hx8g-8h").unwrap();
            let mut undo = game
                .submit(GameRequest {
                    expected: before.clone(),
                    actor: Side::Sente,
                    command: GameCommand::Play {
                        action: action.clone(),
                    },
                })
                .unwrap();
            assert_eq!(
                format_history_action(&game.event_prefix(0).unwrap(), action.clone()).unwrap(),
                "Ln8hx8g-8h"
            );
            game.undo(&mut undo).unwrap();
            assert!(
                game.submit(GameRequest {
                    expected: before,
                    actor: Side::Sente,
                    command: GameCommand::Play { action }
                })
                .is_err()
            );
        }
    }
    #[test]
    fn malformed_action_shapes_reject_and_all_standard_choices_roundtrip_both_owners() {
        for profile in Profile::ALL {
            let mut local = LocalPosition::standard(profile);
            for _ in 0..2 {
                let actions = local.legal_actions();
                for action in &actions {
                    let label = format_action(local.position(), action.clone()).unwrap();
                    assert_eq!(parse_local_action(&local, &label), Ok(action.clone()));
                }
                local.apply(&actions[0]).unwrap();
            }
            let action = local.legal_actions()[0].clone();
            let mut malformed = action.clone();
            malformed.route.clear();
            assert_eq!(
                format_action(local.position(), malformed),
                Err(NotationError::Shape)
            );
            let mut malformed = action.clone();
            malformed.target = Some(sq(1, 1));
            assert_eq!(
                format_action(local.position(), malformed),
                Err(NotationError::Shape)
            );
            let mut malformed = Action::igui(action.from, sq(1, 1));
            malformed.route = vec![sq(1, 2)];
            assert_eq!(
                format_action(local.position(), malformed),
                Err(NotationError::Shape)
            );
            assert!(parse_local_action(&local, &"x".repeat(33)).is_err());
        }
    }
    #[test]
    fn english_face_names_match_the_independently_ratified_profile() {
        let profile: serde_json::Value =
            serde_json::from_str(include_str!("../../../../docs/m4/tenjiku/profile.json")).unwrap();
        let pieces = profile["pieces"].as_array().unwrap();
        assert_eq!(pieces.len(), Face::ALL.len());
        for face in Face::ALL {
            let expected = pieces
                .iter()
                .find(|p| p["face"].as_str() == Some(face.code()))
                .unwrap();
            assert_eq!(
                Some(face_name(face)),
                expected["name"].as_str(),
                "{}",
                face.code()
            );
        }
    }
    #[test]
    fn actual_three_landing_demon_effects_explain_both_profiles_without_invented_victims() {
        // TEN-BURN-DYING's independent root, with actual three-step route
        // 8j -> 7i -> 7h -> 8h. T04 burns the8g Pawn before mover death only
        // under Adams; both profiles leave the enemy Demon9h untouched.
        for profile in Profile::ALL {
            let local = local(
                profile,
                &[
                    (8, 10, Side::Sente, Base::FiD, false),
                    (9, 8, Side::Gote, Base::FiD, false),
                    (8, 7, Side::Gote, Base::P, false),
                    (16, 16, Side::Sente, Base::K, false),
                    (1, 1, Side::Gote, Base::K, false),
                    (15, 14, Side::Sente, Base::P, false),
                    (2, 3, Side::Gote, Base::P, false),
                ],
            );
            let action = Action {
                kind: ActionKind::Travel,
                from: sq(8, 10),
                route: vec![sq(7, 9), sq(7, 8), sq(8, 8)],
                promote: false,
                target: None,
            };
            let (_, effects) = local.simulate(&action).unwrap();
            let expected = match profile {
                Profile::AdamsModern => {
                    "Burned together: Gote Pawn at 8g; Sente Fire Demon destroyed at 8h"
                }
                Profile::MullerCompleted => "Sente Fire Demon destroyed at 8h",
            };
            assert_eq!(format_effects(&effects), expected);
            assert_eq!(effects.action, action);
        }
    }
    #[test]
    fn stationary_capture_and_original_promoted_capture_identities_are_spoken_exactly() {
        for profile in Profile::ALL {
            let igui = local(
                profile,
                &[
                    (8, 8, Side::Sente, Base::ChS, true),
                    (9, 8, Side::Gote, Base::P, false),
                ],
            );
            let (_, effects) = igui.simulate(&Action::igui(sq(8, 8), sq(9, 8))).unwrap();
            assert_eq!(
                format_effects(&effects),
                "Stationary capture at 9h; stationary origin 8h; Captured Gote Pawn at 9h"
            );
            let lion = local(
                profile,
                &[
                    (8, 8, Side::Sente, Base::Ln, false),
                    (8, 7, Side::Gote, Base::P, true),
                    (8, 6, Side::Gote, Base::G, false),
                ],
            );
            let action = Action {
                kind: ActionKind::Travel,
                from: sq(8, 8),
                route: vec![sq(8, 7), sq(8, 6)],
                promote: false,
                target: None,
            };
            let (_, effects) = lion.simulate(&action).unwrap();
            assert_eq!(
                format_effects(&effects),
                "Captured Gote Gold General (promoted Pawn) at 8g; Captured Gote Gold General at 8f"
            );
        }
    }
    #[test]
    fn stationary_igui_origin_does_not_claim_survival_beside_an_enemy_demon() {
        for profile in Profile::ALL {
            // The Tetrarch captures9h without moving. The other enemy Demon7h
            // survives; its final-position hazard destroys the mover at8h.
            let igui = local(
                profile,
                &[
                    (8, 8, Side::Sente, Base::ChS, true),
                    (9, 8, Side::Gote, Base::P, false),
                    (7, 8, Side::Gote, Base::FiD, false),
                ],
            );
            let action = Action::igui(sq(8, 8), sq(9, 8));
            let (after, effects) = igui.simulate(&action).unwrap();
            assert!(after.position().piece(sq(8, 8)).is_none());
            assert!(after.position().piece(sq(9, 8)).is_none());
            assert_eq!(after.position().piece(sq(7, 8)).unwrap().base, Base::FiD);
            assert_eq!(effects.mover_after, None);
            assert_eq!(
                format_effects(&effects),
                "Stationary capture at 9h; stationary origin 8h; Captured Gote Pawn at 9h; Sente Heavenly Tetrarchs (promoted Chariot Soldier) destroyed at 8h"
            );
        }
    }
    #[test]
    fn simultaneous_burn_order_promotion_and_quiet_effects_remain_native_facts() {
        for profile in Profile::ALL {
            let demon = local(
                profile,
                &[
                    (8, 10, Side::Sente, Base::FiD, false),
                    (8, 7, Side::Gote, Base::P, false),
                    (7, 8, Side::Gote, Base::P, true),
                ],
            );
            let (_, effects) = demon
                .simulate(&Action::single(sq(8, 10), sq(8, 8)))
                .unwrap();
            assert_eq!(
                format_effects(&effects),
                "Burned together: Gote Pawn at 8g, Gote Gold General (promoted Pawn) at 7h"
            );
            let pawn = local(
                profile,
                &[
                    (12, 6, Side::Sente, Base::P, false),
                    (12, 5, Side::Gote, Base::S, false),
                ],
            );
            let mut action = Action::single(sq(12, 6), sq(12, 5));
            action.promote = true;
            let (_, effects) = pawn.simulate(&action).unwrap();
            assert_eq!(
                format_effects(&effects),
                "Captured Gote Silver General at 12e; Promoted Sente Pawn to Sente Gold General (promoted Pawn) at 12e"
            );
            // Presentation must retain supplied native phase order, not regroup
            // captures/promotions or reconstruct them from occupancy.
            let mut reversed = effects.clone();
            reversed.phases.reverse();
            assert_eq!(
                format_effects(&reversed),
                "Promoted Sente Pawn to Sente Gold General (promoted Pawn) at 12e; Captured Gote Silver General at 12e"
            );
            let quiet = local(profile, &[(8, 8, Side::Sente, Base::G, false)]);
            let (_, effects) = quiet.simulate(&Action::single(sq(8, 8), sq(8, 7))).unwrap();
            assert_eq!(format_effects(&effects), "No captures or other effects.");
        }
    }
}
