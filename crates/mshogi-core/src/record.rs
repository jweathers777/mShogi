//! Shared strict saved-record primitives. Game admission remains native-owned.
use serde::{
    Deserialize, Serialize,
    de::{self, MapAccess, SeqAccess, Visitor},
};
use std::{collections::BTreeSet, fmt};

/// One recorded input; effects and conclusions belong to authoritative replay.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct RecordedCommand<S, C> {
    pub actor: S,
    pub command: C,
}

/// Reject duplicate fields before tagged-enum buffering can erase them.
pub(crate) fn reject_duplicates(text: &str) -> serde_json::Result<()> {
    struct Unique;
    impl<'de> Deserialize<'de> for Unique {
        fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
            d.deserialize_any(UniqueVisitor)
        }
    }
    struct UniqueVisitor;
    impl<'de> Visitor<'de> for UniqueVisitor {
        type Value = Unique;
        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("duplicate-free bounded JSON")
        }
        fn visit_map<M: MapAccess<'de>>(self, mut map: M) -> Result<Unique, M::Error> {
            let mut keys = BTreeSet::new();
            while let Some(key) = map.next_key::<String>()? {
                if !keys.insert(key) {
                    return Err(de::Error::custom("duplicate field"));
                }
                map.next_value::<Unique>()?;
            }
            Ok(Unique)
        }
        fn visit_seq<S: SeqAccess<'de>>(self, mut seq: S) -> Result<Unique, S::Error> {
            while seq.next_element::<Unique>()?.is_some() {}
            Ok(Unique)
        }
        fn visit_bool<E: de::Error>(self, _: bool) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_i64<E: de::Error>(self, _: i64) -> Result<Unique, E> {
            Ok(Unique)
        }
        fn visit_u64<E: de::Error>(self, _: u64) -> Result<Unique, E> {
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
    let mut d = serde_json::Deserializer::from_str(text);
    Unique::deserialize(&mut d)?;
    d.end()
}
