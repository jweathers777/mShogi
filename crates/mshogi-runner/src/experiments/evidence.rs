use super::*;
use std::collections::BTreeMap;

pub(super) fn lineage_key(root: &Root) -> Result<String, RunnerError> {
    // Provenance grouping, not a repetition/search identity or a legality rule.
    // Current native schemas retain immutable roots and their policies. Strip
    // only derived current state/event ledgers; profile/edition remain bound.
    let mut value = serde_json::to_value(&root.record).map_err(err)?;
    let payload = value
        .get_mut("payload")
        .and_then(|p| p.as_object_mut())
        .ok_or_else(|| err("native root payload missing"))?;
    for key in [
        "events",
        "commands",
        "current_position",
        "conclusion",
        "score_half_points",
        "pending",
        "no_win_assessment",
        "obligation",
        "obligations",
        "incomplete",
        "current_protection",
        "draw_offer",
        "pending_bare",
    ] {
        payload.remove(key);
    }
    Ok(format!(
        "native-root-v1:{}",
        digest(&serde_json::to_string(&value).map_err(err)?)
    ))
}
/// Descriptive finite-catalog accounting. No Elo, significance or improvement verdict.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Summary {
    /// Immutable native profile; incomparable heuristic units are never pooled.
    pub profile: String,
    /// Candidate comparison partner.
    pub baseline: String,
    /// Entire scheduled color pairs, including failed/capped pairs.
    pub scheduled_pairs: u64,
    /// Both games played and genuinely native-scored.
    pub complete_played_pairs: u64,
    /// One or both games lack a native score.
    pub unscored_pairs: u64,
    /// Pair includes a preconcluded root; not strength evidence.
    pub diagnostic_pairs: u64,
    /// Paired candidate score0,0.5,1,1.5,2, only complete played pairs.
    pub pentanomial: [u64; 5],
    /// Candidate native wins/draws/losses among played games only.
    pub wins_draws_losses: [u64; 3],
    /// Missing outcomes, never counted as draws.
    pub unscored_games: u64,
    /// Already-concluded roots, separate from actual played results.
    pub preconcluded_games: u64,
    /// Exact experimental/native stop counts, including attrition.
    pub stops: BTreeMap<String, u64>,
}
impl Report {
    /// Derive paired counts only after authoritative evidence replay.
    pub fn summaries(&self) -> Result<Vec<Summary>, RunnerError> {
        self.validate()?;
        let mut groups: BTreeMap<(String, String), Summary> = BTreeMap::new();
        for pair in self.games.as_chunks::<2>().0 {
            let a = &pair[0];
            let profile = self.config.openings[a.pairing.opening]
                .root
                .record
                .profile()
                .id()
                .to_owned();
            let baseline = self.config.players[a.pairing.baseline].id.clone();
            let s = groups
                .entry((profile.clone(), baseline.clone()))
                .or_insert_with(|| Summary {
                    profile,
                    baseline,
                    ..Summary::default()
                });
            s.scheduled_pairs += 1;
            let mut scores = vec![];
            let mut diagnostic = false;
            for g in pair {
                *s.stops.entry(g.stop.clone()).or_default() += 1;
                let played = g.steps.iter().any(|step| step.applied);
                if !played && g.score_first_twice.is_some() {
                    s.preconcluded_games += 1;
                    diagnostic = true;
                }
                if let Some(n) = g.score_first_twice {
                    let score = if g.pairing.swapped { 2 - n } else { n };
                    scores.push(score);
                    if played {
                        s.wins_draws_losses[match score {
                            2 => 0,
                            1 => 1,
                            _ => 2,
                        }] += 1;
                    }
                } else {
                    s.unscored_games += 1;
                }
            }
            if scores.len() != 2 {
                s.unscored_pairs += 1;
            } else if diagnostic {
                s.diagnostic_pairs += 1;
            } else {
                s.complete_played_pairs += 1;
                s.pentanomial[usize::from(scores[0] + scores[1])] += 1;
            }
        }
        Ok(groups.into_values().collect())
    }
    /// Replayable native JSONL, with explicit missing labels for unsuccessful games.
    /// Every descendant of an opening inherits its split/lineage group.
    pub fn data_jsonl(&self) -> Result<String, RunnerError> {
        self.validate()?;
        let report_sha256 = digest(&self.to_json()?);
        let mut output = String::new();
        for (game_index, g) in self.games.iter().enumerate() {
            let opening = &self.config.openings[g.pairing.opening];
            let mut state = native::NativeState::from_root(&opening.root).map_err(err)?;
            for (step_index, step) in g.steps.iter().enumerate() {
                let before = state.root().map_err(err)?;
                if step.applied {
                    state
                        .admit(
                            &step
                                .observation
                                .as_ref()
                                .ok_or_else(|| err("applied without observation"))?
                                .result,
                        )
                        .map_err(err)?;
                }
                let after = state.root().map_err(err)?;
                let row = serde_json::json!({"schema":SCHEMA,"source_report_sha256":report_sha256,"config_sha256":self.config_sha256,"game":game_index,"step":step_index,"pair":g.pairing.pair,"opening":opening.id,"group":opening.group,"native_lineage":lineage_key(&opening.root)?,"split":opening.split,"participant":self.config.players[step.player].id,"executable":self.identities[step.player],"before":before,"request":step.request,"observation":step.observation,"applied":step.applied,"failure":step.failure,"after":after,"game_stop":g.stop,"native_outcome":g.native_outcome,"score_first_twice":g.score_first_twice,"terminal_value_available":g.score_first_twice.is_some(),"usable_action_target":step.applied});
                let line = serde_json::to_string(&row).map_err(err)?;
                if output.len() + line.len() + 1 > MAX_BYTES {
                    return Err(err("data export exceeds32MiB; no output committed"));
                }
                output.push_str(&line);
                output.push('\n');
            }
        }
        Ok(output)
    }
}
/// Existing actual external runner families; no new opponent wire protocol.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalProtocol {
    /// Orthodox Chess UCI.
    Uci,
    /// Standard Shogi USI.
    Usi,
    /// Actual Xiangqi UCI0–9 family.
    XiangqiUci09,
}
/// Shared provenance around a complete existing external-runner report.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalReport {
    /// Evidence wrapper version.
    pub schema: u32,
    /// Actual adapter family.
    pub protocol: ExternalProtocol,
    /// Observed harness build/runtime.
    pub environment: Environment,
    /// Exact existing effective report; native decoder/replay remains authoritative.
    pub report_json: String,
    /// Integrity fingerprint of those exact bytes.
    pub report_sha256: String,
}
impl ExternalReport {
    /// Strict bounded shared wrapper; the nested adapter retains its own decoder.
    pub fn from_json(text: &str) -> Result<Self, RunnerError> {
        if text.len() > MAX_BYTES {
            return Err(err("external wrapper exceeds32MiB"));
        }
        let report: Self = serde_json::from_str(text).map_err(err)?;
        report.validate()?;
        Ok(report)
    }
    /// Encode a replay-checked bounded wrapper.
    pub fn to_json(&self) -> Result<String, RunnerError> {
        self.validate()?;
        let text = serde_json::to_string(self).map_err(err)?;
        if text.len() > MAX_BYTES {
            return Err(err("external wrapper exceeds32MiB"));
        }
        Ok(text)
    }
    /// Recheck native reports, without pretending a historical observation reruns an engine.
    pub fn validate(&self) -> Result<(), RunnerError> {
        if self.schema != SCHEMA
            || self.report_json.len() > MAX_BYTES
            || serde_json::to_vec(self).map_err(err)?.len() > MAX_BYTES
            || digest(&self.report_json) != self.report_sha256
        {
            return Err(err("external report identity/size mismatch"));
        }
        match self.protocol {
            ExternalProtocol::Uci => {
                crate::ExperimentReport::from_json(&self.report_json)?.replay()?;
            }
            ExternalProtocol::Usi => {
                crate::shogi::Report::from_json(&self.report_json)?.replay()?;
            }
            ExternalProtocol::XiangqiUci09 => {
                crate::xiangqi::Report::from_json(&self.report_json)?.replay()?;
            }
        }
        Ok(())
    }
}
/// Execute the already implemented actual opponent adapter, preserving its complete evidence.
pub fn run_external(
    protocol: ExternalProtocol,
    config_json: &str,
) -> Result<ExternalReport, RunnerError> {
    if config_json.len() > 1024 * 1024 {
        return Err(err("external config exceeds1MiB"));
    }
    let report_json = match protocol {
        ExternalProtocol::Uci => {
            crate::run_experiment(&crate::ExperimentConfig::from_json(config_json)?)?.to_json()
        }
        ExternalProtocol::Usi => {
            crate::shogi::run(&crate::shogi::Config::from_json(config_json)?)?.to_json()?
        }
        ExternalProtocol::XiangqiUci09 => {
            crate::xiangqi::run(&crate::xiangqi::Config::from_json(config_json)?)?.to_json()?
        }
    };
    let r = ExternalReport {
        schema: SCHEMA,
        protocol,
        environment: Environment::observed(),
        report_sha256: digest(&report_json),
        report_json,
    };
    r.validate()?;
    Ok(r)
}
