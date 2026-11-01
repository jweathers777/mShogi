//! Actual local observations; history/records/execution remain pending.
use super::{
    conformance::{self, action, root, snapshot, sq},
    *,
};
use mshogi_conformance::{
    Corpus, Fixture,
    assertions::{self, Assertion, NativeAdapter, Status},
    properties,
};
use serde_json::{Value, json};

pub(super) fn action_value(action: Action) -> Value {
    match action {
        Action::Move {
            from,
            to,
            result_face,
        } => {
            json!({"kind":"move","from":from.to_string(),"to":to.to_string(),"result_face":result_face.code()})
        }
        Action::Drop { face, to } => json!({"kind":"drop","piece":face.code(),"to":to.to_string()}),
    }
}
pub(super) fn runtime(p: &Position) -> (u64, u64, u64, u64) {
    (p.node, p.next_node, p.branch, p.revision)
}
fn local_snapshot(p: &LocalPosition) -> Value {
    json!({"state":snapshot(p.position()),"runtime":runtime(p.position())})
}
fn rejection_reason(error: ActionError, source: Option<Piece>) -> &'static str {
    match error {
        ActionError::Geometry => "blocked_path",
        ActionError::SelfCheck if source.is_some_and(|p| p.face == Face::Phoenix) => {
            "own_phoenix_attacked"
        }
        ActionError::SelfCheck => "exposes_own_phoenix",
        ActionError::SwallowFile => "third_own_unpromoted_swallow_on_file",
        ActionError::SwallowLastRank => "immobile_swallow_drop",
        ActionError::SwallowDropMate => "direct_swallow_drop_mate",
        ActionError::ResultFace => "forced_promotion_required",
        ActionError::OccupiedDrop => "occupied_drop_square",
        ActionError::MissingHand => "piece_not_in_hand",
        ActionError::InvalidDropFace => "promoted_drop_not_admitted",
        _ => "unmapped_native_error",
    }
}
struct LocalAdapter;
impl NativeAdapter for LocalAdapter {
    fn observe(&self, f: &Fixture, a: &Assertion) -> Result<Option<Value>, String> {
        if a.owner == "MSH-127" {
            return conformance::StructuralAdapter.observe(f, a);
        }
        if a.owner != "MSH-128" {
            return Ok(None);
        }
        let p = root(&f.given["root"]).map_err(|e| e.to_string())?;
        if f.scope == "root_admission" {
            let result = LocalPosition::admit(p);
            return Ok(Some(match a.field.as_str() {
                "root_admitted" => json!(result.is_ok()),
                "reason" => json!(match result.err().ok_or("expected root rejection")? {
                    AdmissionError::AdjacentRoyals | AdmissionError::NonmovingRoyalAttacked =>
                        "royal_safety",
                    AdmissionError::StrandedSwallow => "immobile_unpromoted_swallow",
                    AdmissionError::SwallowFile => "third_own_swallow",
                }),
                _ => return Err("unexpected local root field".into()),
            }));
        }
        let mut p = LocalPosition::admit(p).map_err(|e| e.to_string())?;
        if f.scope == "terminal_position" {
            return Ok(Some(match a.field.as_str() {
                "in_check" => json!(p.is_in_check(p.position().side_to_move())),
                "complete_legal_actions" => json!(
                    p.legal_actions()
                        .into_iter()
                        .map(action_value)
                        .collect::<Vec<_>>()
                ),
                _ => return Err("unexpected terminal local field".into()),
            }));
        }
        let before = p.clone();
        let native_before = local_snapshot(&p);
        let command = action(&f.given["command"]);
        let source = match command {
            Action::Move { from, .. } => p.position().piece(from),
            _ => None,
        };
        let result = p.apply(command);
        if a.field == "accepted" {
            return Ok(Some(json!(result.is_ok())));
        }
        if a.field == "reason" {
            return Ok(Some(json!(match result {
                Ok(_) => "admitted",
                Err(e) => rejection_reason(e, source),
            })));
        }
        if a.field == "rejection/local_state" {
            return Ok(Some(properties::snapshots(
                &native_before,
                &local_snapshot(&p),
            )));
        }
        let (effects, mut undo) = result.map_err(|e| e.to_string())?;
        Ok(Some(match a.field.as_str() {
            "reversal_contract/apply_then_undo" => {
                let original = snapshot(before.position());
                p.undo(&mut undo).map_err(|e| e.to_string())?;
                properties::snapshots(&original, &snapshot(p.position()))
            }
            "reversal_contract/redo" => {
                let applied = snapshot(p.position());
                let mut redo = p.undo(&mut undo).map_err(|e| e.to_string())?;
                let (again, _) = p.redo(&mut redo).map_err(|e| e.to_string())?;
                if effects != again {
                    return Err("local redo effects differ".into());
                }
                properties::snapshots(&applied, &snapshot(p.position()))
            }
            "check" | "own_phoenix_in_check_after" => {
                let side = if a.field == "check" {
                    p.position().side_to_move()
                } else {
                    effects.actor
                };
                json!(p.is_in_check(side))
            }
            "own_phoenix_in_check_before" => json!(before.is_in_check(effects.actor)),
            "legal_defense" => action_value(*p.legal_actions().first().ok_or("no native defense")?),
            "native_movement_probes" => {
                // These dictionary keys are independently specified probe INPUTS;
                // values below are actual native inspection, not expected booleans.
                let mut probes = serde_json::Map::new();
                for text in a
                    .expected
                    .as_object()
                    .ok_or("invalid probe specification")?
                    .keys()
                {
                    let from = sq(&text[..2]);
                    let to = sq(&text[2..]);
                    let present = p
                        .inspect(InspectionOrigin::Board(from))
                        .iter()
                        .any(|choice| matches!(choice.action,Action::Move{to:t,..} if t==to));
                    probes.insert(text.clone(), json!(present));
                }
                Value::Object(probes)
            }
            field => {
                conformance::transition_field(before.position(), p.position(), effects, field)?
            }
        }))
    }
}
#[test]
fn actual_local_and_structural_fixture_observations() {
    let corpus = Corpus::embedded().unwrap();
    let report = assertions::run_adapter(&corpus, &LocalAdapter);
    for a in &report.assertions {
        if a.status == Status::Failed {
            eprintln!("{a:?}");
        }
    }
    assert_eq!(report.failed, 0);
    assert!(
        report
            .assertions
            .iter()
            .filter(|a| matches!(a.owner.as_str(), "MSH-127" | "MSH-128"))
            .all(|a| a.status == Status::Passed)
    );
    assert!(
        report
            .assertions
            .iter()
            .filter(|a| !matches!(a.owner.as_str(), "MSH-127" | "MSH-128"))
            .all(|a| a.status == Status::Pending)
    );
    assert_eq!(
        (
            report.executed,
            report.passed,
            report.pending,
            report.failed
        ),
        (483, 483, 244, 0)
    );
    eprintln!(
        "TORI-128 native assertions: {} executed / {} passed / {} pending / {} failed; {} later consumer requirements pending",
        report.executed,
        report.passed,
        report.pending,
        report.failed,
        report.later_consumer_requirements.len()
    );
    if let Ok(path) = std::env::var("MSH_TORI_128_REPORT") {
        std::fs::write(path, serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    }
}
