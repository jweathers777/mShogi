//! Snapshot and valid7x7 data transformations; no game legality or state transitions.
use serde_json::{Value, json};

/// Exercise actual native apply/undo closures against a complete state projection.
/// Project logical state/caches; test never-rewound runtime lineage separately.
pub fn round_trip<S, V: PartialEq, U, E: std::fmt::Debug>(
    state: &mut S,
    project: impl Fn(&S) -> V,
    apply: impl FnOnce(&mut S) -> Result<U, E>,
    undo: impl FnOnce(&mut S, U) -> Result<(), E>,
) -> Result<(), String> {
    let before = project(state);
    let token = apply(state).map_err(|e| format!("apply failed: {e:?}"))?;
    undo(state, token).map_err(|e| format!("undo failed: {e:?}"))?;
    if project(state) == before {
        Ok(())
    } else {
        Err("apply/undo changed authoritative state".into())
    }
}

/// Exercise a real rejected operation and check that its state projection is unchanged.
pub fn rejection<S, V: PartialEq, T, E>(
    state: &mut S,
    project: impl Fn(&S) -> V,
    submit: impl FnOnce(&mut S) -> Result<T, E>,
) -> Result<(), String> {
    let before = project(state);
    if submit(state).is_ok() {
        return Err("operation was accepted instead of rejected".into());
    }
    if project(state) == before {
        Ok(())
    } else {
        Err("rejected operation mutated state".into())
    }
}

/// Supply actual complete before/undo, applied/redo, or source/replayed snapshots.
/// Native callers include authoritative caches for internal restoration checks.
pub fn snapshots(before: &Value, after: &Value) -> Value {
    json!({"before":before,"after":after})
}
/// Exact authoritative-state equivalence (also rejection immutability/base identity).
pub fn equivalent(before: &Value, after: &Value) -> Result<(), String> {
    if before == after {
        Ok(())
    } else {
        Err("authoritative snapshots differ".into())
    }
}
/// Compare a native-provided normalized material inventory; never demote pieces here.
pub fn preserves_material(before: &Value, after: &Value) -> Result<(), String> {
    equivalent(before, after)
}
/// Rotate a valid7x7 coordinate180 degrees. Does not establish legal movement.
pub fn rotate_square(s: &str) -> Result<String, String> {
    let b = s.as_bytes();
    if b.len() != 2 || !(b'1'..=b'7').contains(&b[0]) || !(b'a'..=b'g').contains(&b[1]) {
        return Err("invalid7x7 square".into());
    }
    Ok(format!(
        "{}{}",
        char::from(b'8' - b[0] + b'0'),
        char::from(b'g' - b[1] + b'a')
    ))
}
fn rotate_text(s: &str) -> String {
    if let Ok(sq) = rotate_square(s) {
        return sq;
    }
    let bits: Vec<_> = s.split(':').collect();
    if bits.len() == 3
        && matches!(bits[0], "s" | "g")
        && let Ok(sq) = rotate_square(bits[2])
    {
        return format!(
            "{}:{}:{sq}",
            if bits[0] == "s" { "g" } else { "s" },
            bits[1]
        );
    }
    match s {
        "s" => "g".into(),
        "g" => "s".into(),
        "sente" => "gote".into(),
        "gote" => "sente".into(),
        _ => s.into(),
    }
}
/// Rotate fixture board/hand coordinate data and exchange owners, preserving event order.
/// Descriptive prose is untouched. Native adapters must rotate native state independently.
pub fn rotate_data(v: &Value) -> Value {
    match v {
        Value::String(s) => Value::String(rotate_text(s)),
        Value::Array(a) => Value::Array(a.iter().map(rotate_data).collect()),
        Value::Object(m) => Value::Object(
            m.iter()
                .map(|(k, v)| {
                    let key = if k == "s" {
                        "g".into()
                    } else if k == "g" {
                        "s".into()
                    } else if let Some(tail) = k.strip_prefix("s:") {
                        format!("g:{tail}")
                    } else if let Some(tail) = k.strip_prefix("g:") {
                        format!("s:{tail}")
                    } else {
                        k.clone()
                    };
                    (key, rotate_data(v))
                })
                .collect(),
        ),
        _ => v.clone(),
    }
}
