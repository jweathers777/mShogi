//! Read the starting position of trusted committed desktop fixtures.
use mshogi_core::native::CanonicalRecord;

pub fn initial_record(text: &str) -> Result<CanonicalRecord, String> {
    let document: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let record = document
        .get("payload")
        .and_then(|payload| payload.get("record"))
        .ok_or("fixture lacks its canonical native record")?;
    let wrapper = serde_json::json!({
        "envelope_version": mshogi_core::native::RECORD_ENVELOPE_VERSION,
        "game_kind": document["game_kind"],
        "payload": record,
    });
    let record = CanonicalRecord::from_json(&wrapper.to_string()).map_err(|e| e.to_string())?;
    // A concluded fixture still carries its native root. Ask the native game
    // for that prefix instead of editing events or derived rule state in JSON.
    match record {
        CanonicalRecord::Chu(record) => record
            .to_game()
            .and_then(|game| game.record_prefix(0))
            .map(CanonicalRecord::chu)
            .map_err(|e| e.to_string()),
        record if record.event_count() == 0 => Ok(record),
        _ => Err("fixture must supply a live starting position".into()),
    }
}
