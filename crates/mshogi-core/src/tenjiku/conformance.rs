//! Test-only projections from production native APIs into the frozen independent corpus.
use super::*;
use mshogi_conformance::{
    Fixture,
    assertions::{Assertion, Boundary, Predicate, compare},
};
use serde_json::{Value, json};
#[path = "conformance_cases.rs"]
mod cases;
fn parse<T: serde::de::DeserializeOwned>(v: &Value) -> T {
    serde_json::from_value(v.clone()).expect("independent fixture input")
}
fn face_piece(side: Side, face: Face) -> Piece {
    if let Some(base) = Base::ALL.into_iter().find(|b| b.face() == face) {
        Piece::new(0, side, base, false).unwrap()
    } else {
        Piece::new(
            0,
            side,
            Base::ALL
                .into_iter()
                .find(|b| b.promotion() == Some(face))
                .unwrap(),
            true,
        )
        .unwrap()
    }
}
fn observe(f: &Fixture, profile: Profile) -> Result<Option<Value>, String> {
    let g = &f.given;
    if f.id.starts_with("TEN-MOVE-") {
        let origin: Square = parse(&g["actor"]);
        let side: Side = parse(&g["side"]);
        let face: Face = parse(&g["face"]);
        let p = Position::from_diagram(
            profile,
            &[(origin, face_piece(side, face))],
            side,
            RootMetadata::diagram(0),
        )
        .unwrap();
        let local = LocalPosition::admit(p).unwrap();
        let actions = local
            .geometry_actions_from(origin)
            .into_iter()
            .filter(|a| a.kind == ActionKind::Travel)
            .collect::<Vec<_>>();
        let mut targets = vec![];
        if face == Face::HT {
            let mut entries = vec![(origin, face_piece(side, face))];
            for (i, (df, dr)) in [
                (-1, -1),
                (0, -1),
                (1, -1),
                (-1, 0),
                (1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
            ]
            .into_iter()
            .enumerate()
            {
                if let Some(square) = origin.offset(side, df, dr) {
                    entries.push((
                        square,
                        Piece::new(i as u16 + 1, side.opponent(), Base::P, false).unwrap(),
                    ));
                }
            }
            let occupied = LocalPosition::admit(
                Position::from_diagram(profile, &entries, side, RootMetadata::diagram(0)).unwrap(),
            )
            .unwrap();
            targets = occupied
                .geometry_actions_from(origin)
                .into_iter()
                .filter_map(|a| {
                    if a.kind == ActionKind::Igui {
                        a.target
                    } else {
                        None
                    }
                })
                .collect();
        }
        return Ok(Some(
            json!({"complete_geometry_count":actions.len(),"complete_geometry_actions":actions,"potential_igui_targets":targets}),
        ));
    }
    if f.id == "TEN-SETUP" {
        let p = Position::standard(profile);
        let board = p
            .pieces()
            .map(|(at, piece)| json!({"side":piece.side,"piece":piece.base.code(),"at":at}))
            .collect::<Vec<_>>();
        return Ok(Some(
            json!({"board":board,"pieces_per_side":p.pieces().filter(|(_,p)|p.side==Side::Sente).count(),"original_kinds":Base::ALL.len(),"movement_faces":Face::ALL.len(),"first":p.side_to_move()}),
        ));
    }
    if f.id.starts_with("TEN-PROMOTION-CONCRETE-") {
        let position = cases::position_from_root(&g["root"], profile)?;
        let mut game = Game::from_root(position).map_err(|e| e.to_string())?;
        let input = &g["commands"][0];
        let actor = parse(&input["actor"]);
        let command: GameCommand = parse(&input["command"]);
        let before = GameRecord::from_game(&game).unwrap();
        let result = game.submit(GameRequest {
            expected: game.context(),
            actor,
            command,
        });
        if result.is_err() {
            return Ok(Some(
                json!({"accepted":false,"state_unchanged":before==GameRecord::from_game(&game).unwrap()}),
            ));
        }
        let effect = game.events().last().unwrap().effects.as_ref().unwrap();
        let piece = effect.mover_after.unwrap();
        return Ok(Some(
            json!({"accepted":true,"actor_original":piece.base.code(),"actor_promoted":piece.promoted,"movement_face":piece.face().code()}),
        ));
    }
    cases::observe(&f.id, g, profile)
}
#[test]
fn both_profiles_execute_every_applicable_frozen_native_expectation() {
    let corpus = mshogi_conformance::tenjiku::Corpus::embedded().unwrap();
    for profile in Profile::ALL {
        let mut rows = 0;
        let mut assertions = 0;
        let mut failures = vec![];
        for f in corpus.fixtures().iter().filter(|f| f.scope == "native") {
            rows += 1;
            let observed = match observe(f, profile) {
                Ok(Some(v)) => v,
                Ok(None) => {
                    failures.push(format!("{} UNIMPLEMENTED native adapter", f.id));
                    continue;
                }
                Err(e) => {
                    failures.push(format!("{} native error {e}", f.id));
                    continue;
                }
            };
            for (field, expected) in f.expected[profile.id()].as_object().unwrap() {
                assertions += 1;
                let predicate = if [
                    "complete_geometry_actions",
                    "potential_igui_targets",
                    "board",
                    "legal_all",
                    "divide_depth2",
                ]
                .contains(&field.as_str())
                {
                    Predicate::CompleteSet
                } else {
                    Predicate::Equal
                };
                let assertion = Assertion {
                    fixture_id: f.id.clone(),
                    field: field.clone(),
                    owner: "MSH-87".into(),
                    boundary: if f.id.starts_with("TEN-MOVE-") {
                        Boundary::Geometry
                    } else {
                        Boundary::History
                    },
                    predicate,
                    expected: expected.clone(),
                };
                if let Err(e) = compare(&assertion, &observed[field]) {
                    failures.push(format!(
                        "{}.{field}: {e}; expected={expected}; actual={}",
                        f.id, observed[field]
                    ));
                }
            }
        }
        eprintln!(
            "TENJIKU_CONFORMANCE profile={} native_rows={rows} executed={assertions} failed={} documentary_rows=129 later_consumer_rows=17",
            profile.id(),
            failures.len()
        );
        assert!(
            failures.is_empty(),
            "{}\n{}",
            profile.id(),
            failures.join("\n")
        );
        assert_eq!(rows, 204);
        assert_eq!(assertions, 621);
    }
}
#[test]
fn all_originals_once_promoted_never_gain_another_flip() {
    for profile in Profile::ALL {
        for base in Base::ALL {
            if base.promotion().is_none() {
                continue;
            }
            let origin = Square::new(8, 8).unwrap();
            let piece = Piece::new(0, Side::Sente, base, true).unwrap();
            let local = LocalPosition::admit(
                Position::from_diagram(
                    profile,
                    &[(origin, piece)],
                    Side::Sente,
                    RootMetadata::diagram(0),
                )
                .unwrap(),
            )
            .unwrap();
            let choices = local.choices_from(origin);
            assert!(!choices.is_empty());
            assert!(choices.iter().all(|a| !a.promote));
            let mut invalid = choices[0].clone();
            invalid.promote = true;
            let before = local.position().clone();
            assert_eq!(
                local.simulate(&invalid).unwrap_err(),
                ActionError::Promotion
            );
            assert_eq!(local.position(), &before);
        }
    }
}
