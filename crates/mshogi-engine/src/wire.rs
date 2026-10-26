//! One supervisor owns both explicit transport versions. No untagged guessing.
use crate::*;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::value::RawValue;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WireRequest {
    Legacy(EngineRequest),
    Shared(SharedRequest),
}
impl WireRequest {
    pub fn valid(&self) -> bool {
        match self {
            Self::Legacy(r) => r.protocol == PROTOCOL_VERSION && r.valid_limits(),
            Self::Shared(r) => r.protocol == SHARED_PROTOCOL_VERSION && r.valid_limits(),
        }
    }
    pub fn request_id(&self) -> u64 {
        match self {
            Self::Legacy(r) => r.request_id,
            Self::Shared(r) => r.request_id,
        }
    }
    pub fn generation(&self) -> u64 {
        match self {
            Self::Legacy(r) => r.process_generation,
            Self::Shared(r) => r.process_generation,
        }
    }
    pub fn failure(&self, kind: FailureKind, message: impl ToString) -> WireResponse {
        match self {
            Self::Legacy(context) => WireResponse::Legacy(EngineResponse {
                context: context.clone(),
                result: EngineResult::failure(kind, message),
            }),
            Self::Shared(context) => WireResponse::Shared(SharedResponse {
                context: context.clone(),
                result: SharedResult::failure(kind, message),
            }),
        }
    }
    pub fn execute(&self) -> WireResponse {
        match self {
            Self::Legacy(context) => WireResponse::Legacy(EngineResponse {
                context: context.clone(),
                result: crate::server::execute(context),
            }),
            Self::Shared(context) => WireResponse::Shared(SharedResponse {
                context: context.clone(),
                result: crate::server::execute_shared(context),
            }),
        }
    }
}
impl Serialize for WireRequest {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Legacy(r) => r.serialize(s),
            Self::Shared(r) => r.serialize(s),
        }
    }
}
#[derive(Deserialize)]
struct Header {
    protocol: u32,
}
impl<'de> Deserialize<'de> for WireRequest {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(d)?;
        let header: Header = serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        match header.protocol {
            PROTOCOL_VERSION => serde_json::from_str(raw.get())
                .map(Self::Legacy)
                .map_err(serde::de::Error::custom),
            SHARED_PROTOCOL_VERSION => serde_json::from_str(raw.get())
                .map(Self::Shared)
                .map_err(serde::de::Error::custom),
            // Preserve the legacy diagnostic for a syntactically exact chess
            // request with an unsupported version. execute() rejects its version
            // before any replay/search. A shared envelope cannot decode as this
            // legacy shape, and known v3 above never falls back on parse failure.
            _ => serde_json::from_str(raw.get())
                .map(Self::Legacy)
                .map_err(serde::de::Error::custom),
        }
    }
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WireResponse {
    Legacy(EngineResponse),
    Shared(SharedResponse),
}
impl WireResponse {
    pub fn failure_message(&self) -> Option<&str> {
        match self {
            Self::Legacy(EngineResponse {
                result: EngineResult::Failure { failure },
                ..
            }) => Some(&failure.message),
            Self::Shared(SharedResponse {
                result: SharedResult::Failure { failure },
                ..
            }) => Some(&failure.message),
            _ => None,
        }
    }
    pub fn matches(&self, request: &WireRequest) -> bool {
        match (self, request) {
            (Self::Legacy(response), WireRequest::Legacy(context)) => {
                response.context == *context
                    && match &response.result {
                        EngineResult::Failure { failure } => {
                            failure.message.len() <= MAX_STDERR_BYTES
                        }
                        _ => true,
                    }
            }
            (Self::Shared(response), WireRequest::Shared(context)) => response.matches(context),
            _ => false,
        }
    }
}
impl Serialize for WireResponse {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Legacy(r) => r.serialize(s),
            Self::Shared(r) => r.serialize(s),
        }
    }
}
#[derive(Deserialize)]
struct ResponseHeader {
    context: Header,
}
impl<'de> Deserialize<'de> for WireResponse {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let raw = Box::<RawValue>::deserialize(d)?;
        let header: ResponseHeader =
            serde_json::from_str(raw.get()).map_err(serde::de::Error::custom)?;
        match header.context.protocol {
            PROTOCOL_VERSION => serde_json::from_str(raw.get())
                .map(Self::Legacy)
                .map_err(serde::de::Error::custom),
            SHARED_PROTOCOL_VERSION => serde_json::from_str(raw.get())
                .map(Self::Shared)
                .map_err(serde::de::Error::custom),
            // The exact legacy shape can retain an invalid version only for
            // context-mismatch diagnostics; no valid submitted request matches it.
            _ => serde_json::from_str(raw.get())
                .map(Self::Legacy)
                .map_err(serde::de::Error::custom),
        }
    }
}
