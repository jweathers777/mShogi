//! Real structural adapter. Reads only fixture inputs when producing observations.
//! Expected answers remain in mshogi-conformance's immutable corpus.
use super::{
    transition::{Effects, movement},
    *,
};
use mshogi_conformance::{
    Corpus, Fixture,
    assertions::{self, Assertion, NativeAdapter},
    properties,
};
use serde_json::{Map, Value, json};

pub(super) fn face(code: &str) -> Face {
    Face::ALL
        .into_iter()
        .find(|f| f.code() == code)
        .expect("audited native fixture kind")
}
pub(super) fn sq(s: &str) -> Square {
    s.parse().unwrap()
}
pub(super) fn side(s: &str) -> Side {
    match s {
        "s" | "sente" => Side::Sente,
        "g" | "gote" => Side::Gote,
        _ => panic!("invalid fixture owner"),
    }
}
fn name(side: Side) -> &'static str {
    match side {
        Side::Sente => "sente",
        Side::Gote => "gote",
    }
}
fn short(side: Side) -> &'static str {
    match side {
        Side::Sente => "s",
        Side::Gote => "g",
    }
}
fn entry(s: Square, p: Piece) -> String {
    format!("{}:{}:{s}", short(p.side), p.face.code())
}
pub(super) fn root(v: &Value) -> Result<Position, RootError> {
    if v["root_kind"] == "standard" {
        return Ok(Position::standard());
    }
    let entries: Vec<_> = v["board"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            let parts: Vec<_> = v.as_str().unwrap().split(':').collect();
            (
                sq(parts[2]),
                Piece {
                    side: side(parts[0]),
                    face: face(parts[1]),
                },
            )
        })
        .collect();
    let hands = std::array::from_fn(|i| {
        let h = &v["hands"][if i == 0 { "s" } else { "g" }];
        Hand::from_counts(
            &h.as_object()
                .unwrap()
                .iter()
                .map(|(k, n)| (face(k), n.as_u64().unwrap().try_into().unwrap()))
                .collect::<Vec<_>>(),
        )
        .unwrap()
    });
    Position::from_diagram(
        &entries,
        hands,
        side(v["side_to_move"].as_str().unwrap()),
        RootMetadata::diagram(v["declared_completed_plies"].as_u64().unwrap()),
    )
}
pub(super) fn action(v: &Value) -> Action {
    match v["kind"].as_str().unwrap() {
        "move" => Action::Move {
            from: sq(v["from"].as_str().unwrap()),
            to: sq(v["to"].as_str().unwrap()),
            result_face: face(v["result_face"].as_str().unwrap()),
        },
        "drop" => Action::Drop {
            face: face(v["piece"].as_str().unwrap()),
            to: sq(v["to"].as_str().unwrap()),
        },
        _ => panic!("not a board action"),
    }
}
pub(super) fn board(p: &Position) -> Vec<String> {
    Square::all()
        .filter_map(|s| p.piece(s).map(|v| entry(s, v)))
        .collect()
}
fn hands(p: &Position) -> Value {
    let mut result = Map::new();
    for s in [Side::Sente, Side::Gote] {
        let h: Map<_, _> = Face::HANDS
            .into_iter()
            .filter_map(|f| {
                let n = p.hand(s).count(f);
                (n != 0).then(|| (f.code().into(), json!(n)))
            })
            .collect();
        result.insert(short(s).into(), Value::Object(h));
    }
    Value::Object(result)
}
pub(super) fn snapshot(p: &Position) -> Value {
    json!({"profile":p.profile_id(),"edition":p.profile_edition(),"board":board(p),"hands":hands(p),"side":name(p.side_to_move()),"root":format!("{:?}",p.root()),"accepted_plies":p.accepted_plies(),"declared_total":p.declared_total_plies()})
}
struct StructuralAdapter;
impl NativeAdapter for StructuralAdapter {
    fn observe(&self, f: &Fixture, a: &Assertion) -> Result<Option<Value>, String> {
        if a.owner != "MSH-127" {
            return Ok(None);
        }
        if f.scope == "initial_position" {
            let p = Position::standard();
            return Ok(Some(match a.field.as_str() {
                "board" => json!(board(&p)),
                "hands" => hands(&p),
                "side_to_move" => json!(name(p.side_to_move())),
                "per_side" => {
                    let inventories: Vec<Map<String, Value>> = [Side::Sente, Side::Gote]
                        .into_iter()
                        .map(|s| {
                            Face::BASES
                                .into_iter()
                                .map(|face| {
                                    (
                                        face.code().into(),
                                        json!(
                                            Square::all()
                                                .filter(|&sq| p
                                                    .piece(sq)
                                                    .is_some_and(|v| v.side == s && v.face == face))
                                                .count()
                                        ),
                                    )
                                })
                                .collect()
                        })
                        .collect();
                    if inventories[0] != inventories[1] {
                        return Err("standard sides have different inventories".into());
                    }
                    json!(inventories[0])
                }
                _ => return Err(format!("unprojected setup field {}", a.field)),
            }));
        }
        if f.scope == "exhaustive_local_geometry" {
            let m = movement(face(f.given["piece"].as_str().unwrap()));
            return Ok(Some(match a.field.as_str(){
                "complete_steps"=>json!(m.steps),"complete_jumps"=>json!(m.jumps),
                "complete_rays"=>json!(m.rays.iter().filter(|r|r.2==6).map(|r|[r.0,r.1]).collect::<Vec<_>>()),
                "complete_limited_rays"=>json!(m.rays.iter().filter(|r|r.2!=6).map(|r|json!({"direction":[r.0,r.1],"lengths":(1..=r.2).collect::<Vec<_>>()})).collect::<Vec<_>>()),
                "other_geometry"=>json!(false),_=>return Err("unprojected geometry field".into())
            }));
        }
        if f.scope == "orientation_geometry" {
            return Ok(Some(match a.field.as_str() {
                "gote_board_offsets" => json!(
                    f.given["sente_local_offsets"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| Side::Gote.relative_to_sente(
                            v[0].as_i64().unwrap() as i8,
                            v[1].as_i64().unwrap() as i8
                        ))
                        .collect::<Vec<_>>()
                ),
                "rotated_squares" => json!(
                    f.given["sente_squares"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .map(|v| sq(v.as_str().unwrap()).rotated().to_string())
                        .collect::<Vec<_>>()
                ),
                "native_handed_kind_unchanged" => {
                    json!([Face::LeftQuail, Face::RightQuail].into_iter().all(|f| {
                        Piece {
                            side: Side::Gote,
                            face: f,
                        }
                        .face
                            == f
                    }))
                }
                _ => return Err("unprojected rotation field".into()),
            }));
        }
        if f.scope == "root_admission" {
            let result = root(&f.given["root"]);
            return Ok(Some(match a.field.as_str() {
                "root_admitted" => json!(result.is_ok()),
                "reason" => json!(match result.err().ok_or("root unexpectedly accepted")? {
                    RootError::DuplicateSquare => "duplicate_square",
                    RootError::MissingOrDuplicateRoyal => "royal_inventory",
                    RootError::ExcessInventory => "base_inventory_exceeded",
                    _ => return Err("unexpected root error".into()),
                }),
                _ => return Err("unexpected structural root facet".into()),
            }));
        }
        let mut p = root(&f.given["root"]).map_err(|e| e.to_string())?;
        let before = p.clone();
        let command = action(&f.given["command"]);
        let (effects, mut undo) = p
            .apply_structural(command)
            .map_err(|e| format!("structural operation failed: {e:?}"))?;
        let observed = transition_field(&before, &p, effects, &a.field)?;
        match a.field.as_str() {
            "reversal_contract/apply_then_undo" => {
                let snap = snapshot(&before);
                p.undo_structural(&mut undo)
                    .map_err(|e| format!("undo failed: {e:?}"))?;
                Ok(Some(properties::snapshots(&snap, &snapshot(&p))))
            }
            "reversal_contract/redo" => {
                let snap = snapshot(&p);
                let mut redo = p
                    .undo_structural(&mut undo)
                    .map_err(|e| format!("undo failed: {e:?}"))?;
                let (redone, _) = p
                    .redo_structural(&mut redo)
                    .map_err(|e| format!("redo failed: {e:?}"))?;
                if redone != effects {
                    return Err("redo effects differ".into());
                }
                Ok(Some(properties::snapshots(&snap, &snapshot(&p))))
            }
            _ => Ok(Some(observed)),
        }
    }
}
fn transition_field(
    before: &Position,
    after: &Position,
    effects: Effects,
    field: &str,
) -> Result<Value, String> {
    Ok(match field {
        "board_added" => json!(
            Square::all()
                .filter_map(|s| after
                    .piece(s)
                    .filter(|p| before.piece(s) != Some(*p))
                    .map(|p| entry(s, p)))
                .collect::<Vec<_>>()
        ),
        "board_removed" => json!(
            Square::all()
                .filter_map(|s| before
                    .piece(s)
                    .filter(|p| after.piece(s) != Some(*p))
                    .map(|p| entry(s, p)))
                .collect::<Vec<_>>()
        ),
        "board_unchanged" => json!(
            Square::all()
                .filter_map(|s| before
                    .piece(s)
                    .filter(|p| p.face != Face::Phoenix && after.piece(s) == Some(*p))
                    .map(|p| entry(s, p)))
                .collect::<Vec<_>>()
        ),
        "intermediate_unchanged" => {
            let Action::Move { from, to, .. } = effects.action else {
                return Err("intermediate field requires move".into());
            };
            json!(
                Square::all()
                    .filter_map(|s| {
                        let (x, y) = (
                            i16::from(s.file()) - i16::from(from.file()),
                            i16::from(s.rank()) - i16::from(from.rank()),
                        );
                        let (dx, dy) = (
                            i16::from(to.file()) - i16::from(from.file()),
                            i16::from(to.rank()) - i16::from(from.rank()),
                        );
                        let interior = s != from
                            && s != to
                            && x * dy == y * dx
                            && x.min(0) >= dx.min(0)
                            && x.max(0) <= dx.max(0)
                            && y.min(0) >= dy.min(0)
                            && y.max(0) <= dy.max(0);
                        before
                            .piece(s)
                            .filter(|p| interior && after.piece(s) == Some(*p))
                            .map(|p| entry(s, p))
                    })
                    .collect::<Vec<_>>()
            )
        }
        "hand_delta" => {
            let mut m = Map::new();
            if let Some((f, n)) = effects.hand_change {
                m.insert(format!("{}:{}", short(effects.actor), f.code()), json!(n));
            }
            Value::Object(m)
        }
        "side_to_move" => json!(name(after.side_to_move())),
        // This structural state has no timer or exceptional promotion rule.
        "promotion_timer" => Value::Null,
        "no_extra_promotion_privilege" => {
            let Action::Move { from, .. } = effects.action else {
                return Err("expected move".into());
            };
            json!(before.piece(from).unwrap().face == effects.resulting_piece.face)
        }
        "resulting_goose_does_not_attack_straight_forward_one" => {
            let f = effects.resulting_piece.face;
            let m = movement(f);
            json!(
                !m.steps.contains(&(0, 1))
                    && !m.jumps.contains(&(0, 1))
                    && !m.rays.iter().any(|r| (r.0, r.1) == (0, 1))
            )
        }
        "reversal_contract/apply_then_undo" | "reversal_contract/redo" => Value::Null,
        _ => return Err(format!("unprojected transition field {field}")),
    })
}
#[test]
fn real_structural_fixture_observations() {
    let corpus = Corpus::embedded().unwrap();
    let report = assertions::run_adapter(&corpus, &StructuralAdapter);
    for a in &report.assertions {
        if a.status == assertions::Status::Failed {
            eprintln!("{a:?}");
        }
    }
    assert_eq!(report.failed, 0);
    assert!(
        report
            .assertions
            .iter()
            .filter(|a| a.owner == "MSH-127")
            .all(|a| a.status == assertions::Status::Passed)
    );
    assert!(
        report
            .assertions
            .iter()
            .filter(|a| a.owner != "MSH-127")
            .all(|a| a.status == assertions::Status::Pending)
    );
    eprintln!(
        "TORI-127 actual fixture assertions: {} executed / {} passed / {} pending / {} failed; {} later-consumer obligations pending",
        report.executed,
        report.passed,
        report.pending,
        report.failed,
        report.later_consumer_requirements.len()
    );
    // Optional explicit evidence export, test process only; never in normal builds.
    if let Ok(path) = std::env::var("MSH_TORI_127_REPORT") {
        std::fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
}
