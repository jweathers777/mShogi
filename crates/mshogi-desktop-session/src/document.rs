//! The sole current desktop envelope. Native payloads retain their own replay.
//! Runtime-to-codec and validated-codec-to-install bridges end in MSH-118.
use mshogi_core::native::GameKind;
use serde::{
    Deserialize, Serialize,
    de::{self, MapAccess, SeqAccess, Visitor},
};
use serde_json::value::RawValue;
use std::{collections::BTreeSet, fmt};

pub(crate) const FORMAT: &str = "mshogi.desktop.document.v2";

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct DocumentEnvelope {
    pub(crate) format: String,
    pub(crate) game_kind: GameKind,
    pub(crate) payload: Box<RawValue>,
}
impl DocumentEnvelope {
    pub(crate) fn parse(bytes: &[u8]) -> Result<Self, String> {
        // No intermediate map: duplicate outer fields reject here and every
        // nested byte reaches the strict native document/record parser intact.
        let doc: Self = serde_json::from_slice(bytes)
            .map_err(|e| diagnostic(format!("Invalid current desktop document: {e}")))?;
        if doc.format != FORMAT {
            return Err("Unsupported desktop document format/version; development archives are not supported.".into());
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
pub(crate) fn encode<T: Serialize>(game_kind: GameKind, payload: &T) -> Result<Vec<u8>, String> {
    let payload = serde_json::value::to_raw_value(payload).map_err(|e| e.to_string())?;
    serde_json::to_vec_pretty(&DocumentEnvelope {
        format: FORMAT.into(),
        game_kind,
        payload,
    })
    .map_err(|e| e.to_string())
}

#[cfg(test)]
#[path = "document_tests.rs"]
mod tests;

// Raw duplicate rejection precedes every buffered native document payload.
pub(crate) fn reject_duplicates(bytes: &[u8]) -> Result<(), String> {
    struct Unique;
    impl<'de> Deserialize<'de> for Unique {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            d.deserialize_any(UVisitor)
        }
    }
    struct UVisitor;
    impl<'de> Visitor<'de> for UVisitor {
        type Value = Unique;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("duplicate-free JSON")
        }
        fn visit_map<M: MapAccess<'de>>(self, mut m: M) -> Result<Unique, M::Error> {
            let mut keys = BTreeSet::new();
            while let Some(k) = m.next_key::<String>()? {
                if !keys.insert(k) {
                    return Err(de::Error::custom("duplicate field"));
                }
                m.next_value::<Unique>()?;
            }
            Ok(Unique)
        }
        fn visit_seq<S: SeqAccess<'de>>(self, mut s: S) -> Result<Unique, S::Error> {
            while s.next_element::<Unique>()?.is_some() {}
            Ok(Unique)
        }
        fn visit_bool<E: de::Error>(self, _: bool) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_u64<E: de::Error>(self, _: u64) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_i64<E: de::Error>(self, _: i64) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_f64<E: de::Error>(self, _: f64) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_str<E: de::Error>(self, _: &str) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_unit<E: de::Error>(self) -> Result<Unique, E> {
            Ok(Unique)
        }
    }
    let mut d = serde_json::Deserializer::from_slice(bytes);
    Unique::deserialize(&mut d).map_err(|e| e.to_string())?;
    d.end().map_err(|e| e.to_string())
}
