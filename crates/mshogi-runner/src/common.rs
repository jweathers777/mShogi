//! Shared process evidence, clock arithmetic and bounded JSON.
use crate::{EngineConfig, EngineIdentity, ExperimentConfig, FileIdentity, RunnerError};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::{fs::File, io::Read, time::Instant};

/// Experiment failure, never a native foul or competition score.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureKind {
    /// Child launch failed.
    ProcessStart,
    /// Unexpected child exit.
    UnexpectedExit,
    /// Clean EOF without reply.
    MissingReply,
    /// Invalid protocol grammar/output.
    MalformedReply,
    /// Board action rejected by native admission.
    IllegalAction,
    /// Required reply missed process watchdog.
    Watchdog,
    /// Untagged reply arrived outside its request.
    LateReply,
    /// Stream/record resource cap reached.
    ResourceLimit,
    /// Advertised options/configuration mismatch.
    Configuration,
    /// Native invariant/admission failure unrelated to action legality.
    Native,
    /// Live context cannot be represented losslessly by this external dialect.
    UnsupportedAdjudication,
    /// No completed finite search decision.
    IncompleteWork,
    /// Required native classification is unresolved.
    UnresolvedWork,
    /// Actual engine error, without a game outcome.
    ExecutionFailure,
}
/// Equal Fischer clock for both sides, in milliseconds.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FischerClock {
    /// Initial time per side.
    pub base_ms: u64,
    /// Increment added after each accepted move.
    pub increment_ms: u64,
}

impl FischerClock {
    pub(crate) fn valid(self) -> bool {
        (1..=3_600_000).contains(&self.base_ms) && self.increment_ms <= 60_000
    }
}

pub(crate) fn balance_after(before: u64, elapsed: u64, increment: u64) -> u64 {
    before.saturating_sub(elapsed).saturating_add(increment)
}

pub(crate) fn platform() -> String {
    format!("{}-{}", std::env::consts::OS, std::env::consts::ARCH)
}

pub(crate) fn elapsed_ns(start: Instant) -> u64 {
    observed_ns(start, Instant::now())
}

pub(crate) fn observed_ns(start: Instant, receipt: Instant) -> u64 {
    u64::try_from(receipt.duration_since(start).as_nanos()).unwrap_or(u64::MAX)
}

pub(crate) fn sha256(bytes: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(crate) fn rerun_eligible(config: &ExperimentConfig, identities: &[EngineIdentity; 2]) -> bool {
    config.rerun_determinism_eligible()
        && identities.iter().all(|e| {
            e.executable.sha256.is_some() && e.model.as_ref().is_none_or(|m| m.sha256.is_some())
        })
}

/// Bound configuration text apart from the exact JSON spans of opening histories.
/// Read the spans only when the complete input exceeds the metadata allowance.
pub(crate) fn decode_config<T: DeserializeOwned>(text: &str) -> Result<T, RunnerError> {
    const METADATA_BYTES: usize = 1024 * 1024;
    if text.len() > METADATA_BYTES {
        use serde_json::value::RawValue;
        #[derive(Deserialize)]
        struct Opening<'a> {
            #[serde(borrow)]
            root: Option<&'a RawValue>,
            #[serde(borrow)]
            canonical: Option<&'a RawValue>,
            #[serde(borrow)]
            moves: Option<&'a RawValue>,
        }
        #[derive(Deserialize)]
        struct Config<'a> {
            #[serde(borrow, default)]
            openings: Vec<Opening<'a>>,
            #[serde(borrow)]
            opening: Option<Opening<'a>>,
        }
        let cfg: Config = serde_json::from_str(text).map_err(|e| RunnerError(e.to_string()))?;
        let history: usize = cfg
            .openings
            .into_iter()
            .chain(cfg.opening)
            .flat_map(|o| [o.root, o.canonical, o.moves].into_iter().flatten())
            .map(|raw| raw.get().len())
            .sum();
        if text.len() - history > METADATA_BYTES {
            return Err(RunnerError("configuration metadata exceeds 1 MiB".into()));
        }
    }
    serde_json::from_str(text).map_err(|e| RunnerError(e.to_string()))
}

pub(crate) fn decode<T: DeserializeOwned>(text: &str, cap: usize) -> Result<T, RunnerError> {
    if text.len() > cap {
        return Err(RunnerError(format!("JSON exceeds {cap} bytes")));
    }
    serde_json::from_str(text).map_err(|e| RunnerError(e.to_string()))
}

pub(crate) fn encode_report<T: Serialize>(value: &T) -> Result<String, RunnerError> {
    let text = serde_json::to_string(value).map_err(|e| RunnerError(e.to_string()))?;
    if text.len() > REPORT_BYTES {
        return Err(RunnerError("report exceeds 128 MiB".into()));
    }
    Ok(text)
}
pub(crate) const REPORT_BYTES: usize = crate::experiments::MAX_BYTES;

pub(crate) fn identity(path: &str) -> FileIdentity {
    let mut result = FileIdentity {
        path: path.into(),
        canonical_path: None,
        bytes: None,
        sha256: None,
        error: None,
    };
    let read = || -> Result<(String, u64, String), String> {
        let canonical = std::fs::canonicalize(path).map_err(|e| e.to_string())?;
        let metadata = std::fs::metadata(&canonical).map_err(|e| e.to_string())?;
        if !metadata.is_file() {
            return Err("identity path is not a regular file".into());
        }
        let mut file = File::open(&canonical).map_err(|e| e.to_string())?;
        let metadata = file.metadata().map_err(|e| e.to_string())?;
        if !metadata.is_file() {
            return Err("opened identity is not a regular file".into());
        }
        let size = metadata.len();
        if size > 2_147_483_648 {
            return Err("identity file exceeds 2 GiB hashing bound".into());
        }
        let mut digest = Sha256::new();
        let mut bytes = [0u8; 32768];
        let mut total = 0u64;
        loop {
            let n = file.read(&mut bytes).map_err(|e| e.to_string())?;
            if n == 0 {
                break;
            }
            total += n as u64;
            if total > 2_147_483_648 {
                return Err("identity file grew beyond hashing bound".into());
            }
            digest.update(&bytes[..n]);
        }
        if total != size {
            return Err("identity file size changed during hashing".into());
        }
        Ok((
            canonical.to_string_lossy().into_owned(),
            size,
            format!("{:x}", digest.finalize()),
        ))
    };
    match read() {
        Ok((canonical, size, sha)) => {
            result.canonical_path = Some(canonical);
            result.bytes = Some(size);
            result.sha256 = Some(sha)
        }
        Err(e) => result.error = Some(e),
    }
    result
}
pub(crate) fn identities(engines: &[EngineConfig; 2]) -> [EngineIdentity; 2] {
    engines.each_ref().map(|e| EngineIdentity {
        executable: identity(&e.path),
        model: e.model_path.as_ref().map(|p| identity(p)),
        model_identity: e.model_identity.clone(),
    })
}
