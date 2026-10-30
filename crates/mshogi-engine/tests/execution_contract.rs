//! Wire identity and durable semantic observation are separate strict boundaries.
use mshogi_core::{
    chess::{AdjudicatedGame, ClaimPolicy},
    native::CanonicalRecord,
};
use mshogi_engine::*;
use serde_json::json;
use std::io::Cursor;

fn request() -> ExecutionRequest {
    ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: 1,
            session_version: 7,
            process_generation: 3,
            game: CanonicalRecord::chess(AdjudicatedGame::new(ClaimPolicy::ExplicitOnly).record()),
            limits: WorkLimits { depth: 1, nodes: 0 },
        },
    }
}
fn execute(text: &str) -> std::io::Result<ExecutionResponse> {
    let mut output = Vec::new();
    serve_once(Cursor::new(format!("{text}\n")), &mut output)?;
    serde_json::from_slice(&output).map_err(std::io::Error::other)
}

#[test]
fn singular_wire_rejects_old_versions_without_interpreting_a_legacy_payload() {
    let request = request();
    let text = serde_json::to_string(&request).unwrap();
    let response = execute(&text).unwrap();
    assert!(response.matches(&request));
    for version in [0, 2, 3, PROTOCOL_VERSION + 1] {
        let mut changed = request.clone();
        changed.protocol = version;
        assert!(!changed.valid());
        let failure = execute(&serde_json::to_string(&changed).unwrap()).unwrap();
        assert!(matches!(
            failure.observation.result,
            ExecutionResult::Failure {
                failure: EngineFailure {
                    kind: FailureKind::InvalidRequest,
                    ..
                }
            }
        ));
        assert!(!failure.matches(&changed));
    }
    // The former flat v2/v3 request shape is never guessed or translated.
    let mut old = serde_json::to_value(&request.context).unwrap();
    old["protocol"] = json!(3);
    assert!(execute(&old.to_string()).is_err());
}

#[test]
fn outer_wrapper_context_native_and_observation_reject_duplicates_and_unknowns() {
    let request = request();
    let text = serde_json::to_string(&request).unwrap();
    for (needle, replacement) in [
        ("\"protocol\":4", "\"protocol\":4,\"protocol\":4"),
        ("\"context\":{", "\"context\":{},\"context\":{"),
        ("\"request_id\":1", "\"request_id\":1,\"request_id\":1"),
        (
            "\"game_kind\":\"chess\"",
            "\"game_kind\":\"chess\",\"game_kind\":\"chess\"",
        ),
        (
            "\"claim_policy\":\"explicit_only\"",
            "\"claim_policy\":\"explicit_only\",\"claim_policy\":\"explicit_only\"",
        ),
    ] {
        assert!(text.contains(needle));
        assert!(
            execute(&text.replacen(needle, replacement, 1)).is_err(),
            "{needle}"
        );
    }
    let value = serde_json::to_value(&request).unwrap();
    for pointer in ["", "/context", "/context/purpose"] {
        let mut unknown = value.clone();
        unknown.pointer_mut(pointer).unwrap()["extra"] = json!(true);
        assert!(execute(&unknown.to_string()).is_err(), "{pointer}");
    }
    for field in ["protocol", "context"] {
        let mut missing = value.clone();
        missing.as_object_mut().unwrap().remove(field);
        assert!(execute(&missing.to_string()).is_err(), "{field}");
    }
    let response = execute(&text).unwrap();
    let encoded = serde_json::to_string(&response).unwrap();
    for (needle, replacement) in [
        ("\"protocol\":4", "\"protocol\":4,\"protocol\":4"),
        ("\"observation\":{", "\"observation\":{},\"observation\":{"),
        (
            "\"schema_version\":1",
            "\"schema_version\":1,\"schema_version\":1",
        ),
        ("\"request_id\":1", "\"request_id\":1,\"request_id\":1"),
    ] {
        assert!(encoded.contains(needle));
        assert!(
            serde_json::from_str::<ExecutionResponse>(&encoded.replacen(needle, replacement, 1))
                .is_err(),
            "{needle}"
        );
    }
}

#[test]
fn saved_observation_has_semantic_schema_and_no_transport_identity_or_wrapper() {
    let request = request();
    let response = execute(&serde_json::to_string(&request).unwrap()).unwrap();
    let observation = response.observation.clone();
    let saved = serde_json::to_value(&observation).unwrap();
    assert_eq!(saved["schema_version"], OBSERVATION_SCHEMA_VERSION);
    assert!(saved.get("protocol").is_none());
    assert!(saved.get("observation").is_none());
    assert_eq!(saved.as_object().unwrap().len(), 3);
    let parsed: ExecutionObservation = serde_json::from_value(saved.clone()).unwrap();
    assert!(parsed.matches(&request.context));
    let mut unsupported = saved.clone();
    unsupported["schema_version"] = json!(OBSERVATION_SCHEMA_VERSION + 1);
    assert!(
        !serde_json::from_value::<ExecutionObservation>(unsupported)
            .unwrap()
            .matches(&request.context)
    );
    let mut missing = saved.clone();
    missing.as_object_mut().unwrap().remove("schema_version");
    assert!(serde_json::from_value::<ExecutionObservation>(missing).is_err());
    let mut wrapped = saved;
    wrapped["protocol"] = json!(PROTOCOL_VERSION);
    assert!(serde_json::from_value::<ExecutionObservation>(wrapped).is_err());
    for version in [2, 3, PROTOCOL_VERSION + 1] {
        let mut foreign = response.clone();
        foreign.protocol = version;
        assert!(!foreign.matches(&request));
    }
    for (field, changed) in [
        ("request_id", 2),
        ("session_version", 8),
        ("process_generation", 4),
    ] {
        let mut value = serde_json::to_value(&request.context).unwrap();
        value[field] = json!(changed);
        let foreign: ExecutionContext = serde_json::from_value(value).unwrap();
        assert!(!observation.matches(&foreign), "{field}");
    }
}
