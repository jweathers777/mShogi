//! The sole current desktop envelope. Native payloads retain their own replay.
//! Runtime-to-codec and validated-codec-to-install bridges end in MSH-118.
use mshogi_core::native::{GameKind, RulesProfile};
use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

pub(crate) const FORMAT: &str = "mshogi.desktop.document.v1";
const VERSION: u32 = 1;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DocumentEnvelope {
    pub(crate) envelope_version: u32,
    pub(crate) format: String,
    pub(crate) game_kind: GameKind,
    pub(crate) rules_profile: RulesProfile,
    pub(crate) profile_edition: String,
    pub(crate) native_schema_version: u32,
    pub(crate) payload: Box<RawValue>,
}
impl DocumentEnvelope {
    pub(crate) fn parse(bytes: &[u8]) -> Result<Self, String> {
        if bytes.len() > crate::MAX_ARCHIVE_BYTES {
            return Err("Application capacity: desktop document exceeds16MiB.".into());
        }
        // No intermediate map: duplicate outer fields reject here and every
        // nested byte reaches the strict native document/record parser intact.
        let doc: Self = serde_json::from_slice(bytes)
            .map_err(|e| diagnostic(format!("Invalid current desktop document: {e}")))?;
        if doc.envelope_version != VERSION || doc.format != FORMAT {
            return Err("Unsupported desktop document format/version; development archives are not supported.".into());
        }
        if doc.game_kind != doc.rules_profile.game()
            || doc.profile_edition != doc.rules_profile.edition()
            || doc.native_schema_version != 1
        {
            return Err(
                "Conflicting or unsupported desktop game/profile/edition/native schema identity."
                    .into(),
            );
        }
        Ok(doc)
    }
}
pub(crate) fn diagnostic(mut message: String) -> String {
    let mut end = message.len().min(mshogi_engine::MAX_STDERR_BYTES);
    while !message.is_char_boundary(end) {
        end -= 1;
    }
    message.truncate(end);
    message
}
pub(crate) fn encode<T: Serialize>(profile: RulesProfile, payload: &T) -> Result<Vec<u8>, String> {
    let payload = serde_json::value::to_raw_value(payload).map_err(|e| e.to_string())?;
    serde_json::to_vec_pretty(&DocumentEnvelope {
        envelope_version: VERSION,
        format: FORMAT.into(),
        game_kind: profile.game(),
        rules_profile: profile,
        profile_edition: profile.edition().into(),
        native_schema_version: 1,
        payload,
    })
    .map_err(|e| e.to_string())
}

#[cfg(test)]
#[path = "document_tests.rs"]
mod tests;
