//! Actual production-child experiment evidence, separate from playing strength.
use mshogi_core::{
    chess, chu,
    native::{CanonicalRecord, RulesProfile},
    shogi, tenjiku, tori, xiangqi,
};
use mshogi_runner::experiments::{
    Config, Opening, Report, Split, digest, fresh_root, run, run_with_stop, template,
};
use serde_json::{Value, json};
use std::sync::Mutex;

// These actual subprocess journeys share CPU and independent watchdogs.
static JOURNEY: Mutex<()> = Mutex::new(());

fn add(config: &mut Config, record: CanonicalRecord, source: &str) {
    let profile = record.profile();
    config.openings.push(Opening {
        id: format!("tactical-{}", profile.id()),
        group: format!("native-experiment-regression:{}:{source}", profile.id()),
        root: fresh_root(record).unwrap(),
        split: Split::Test,
        provenance: format!(
            "Project-authored native diagnostic from {source}; Test-only regression lineage. \
             Native ending is independently established by the cited existing fixture/test; \
             no external training corpus or inferred licensing grant."
        ),
    });
}

fn fixture(config: &mut Config, text: &str, source: &str, profile: RulesProfile) {
    let document: Value = serde_json::from_str(text).unwrap();
    let record = CanonicalRecord::from_json(&document["payload"]["initial"].to_string()).unwrap();
    assert_eq!(record.profile(), profile);
    add(config, record, source);
}

fn tactical_config() -> Config {
    let mut config = template(env!("CARGO_BIN_EXE_mshogi")).unwrap();
    config.id = "actual-seven-profile-native-experiment-regression".into();
    config.protocol = "Actual production internal-search executable; one native command from \
        seven independently established tactical predecessors, paired sides and two identical \
        embedded baselines. This checks native infrastructure and bounded execution only, \
        without a playing-strength claim. All diagnostic lineages remain Test-only."
        .into();
    config.players.truncate(2);
    for p in &mut config.players {
        p.build_provenance = "Actual CARGO_BIN_EXE_mshogi from this Cargo integration-test build; \
            observed executable SHA256 retained in report. Embedded native baseline; no model load."
            .into();
        for limits in &mut p.limits {
            limits.work.depth = 1;
            limits.work.nodes = match limits.profile {
                RulesProfile::ToriModernV1 => 10_000,
                RulesProfile::ChuJcsa2019V1
                | RulesProfile::TenjikuAdamsModernV1
                | RulesProfile::TenjikuMullerCompletedV1 => 2_000,
                _ => 512,
            };
        }
    }
    config.max_steps = 1;
    config.wall_budget_ms = 120_000;
    config.watchdog_ms = 30_000;
    config.cleanup_ms = 1_000;
    config.openings.clear();

    let chess = chess::AdjudicatedGame::from_fen(
        "7k/8/5KQ1/8/8/8/8/8 w - - 0 1",
        chess::ClaimPolicy::AutoClaim,
    )
    .unwrap();
    add(
        &mut config,
        CanonicalRecord::chess(chess.record()),
        "crates/mshogi-search/tests/search.rs::mate_in_one_for_both_colors_and_already_mated_root",
    );
    let shogi = shogi::Game::from_root(
        shogi::GameRoot::import(
            shogi::Position::from_sfen("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1").unwrap(),
            shogi::RootCategory::ReducedMaterialDiagram,
            shogi::CounterSource::Supplied,
        )
        .unwrap(),
    );
    add(
        &mut config,
        CanonicalRecord::shogi(shogi::GameRecord::from_game(&shogi)),
        "crates/mshogi-console/tests/shogi_engine.rs::actual_analysis_declaration_incomplete_and_history_contexts_remain_distinct",
    );
    let xiangqi = xiangqi::Game::from_root(
        xiangqi::PositionRoot::from_fen("5k3/9/4R4/9/9/9/9/9/9/3K5 w - - 0 1").unwrap(),
        xiangqi::ClaimPolicy::Explicit,
    )
    .unwrap();
    add(
        &mut config,
        CanonicalRecord::xiangqi(xiangqi::GameRecord::from_game(&xiangqi).unwrap()),
        "crates/mshogi-search/tests/xiangqi.rs::immediate_stalemate_is_a_native_win_for_each_side",
    );
    fixture(
        &mut config,
        include_str!("../../../desktop/scripts/fixtures/tori-mate-in-one.mshogi"),
        "desktop/scripts/fixtures/tori-mate-in-one.mshogi#payload.initial",
        RulesProfile::ToriModernV1,
    );
    fixture(
        &mut config,
        include_str!("../../../desktop/scripts/fixtures/chu-complete-royal-engine.mshogi"),
        "desktop/scripts/fixtures/chu-complete-royal-engine.mshogi#payload.initial",
        RulesProfile::ChuJcsa2019V1,
    );
    fixture(
        &mut config,
        include_str!("../../../desktop/scripts/fixtures/tenjiku-adams-royal-engine.mshogi"),
        "desktop/scripts/fixtures/tenjiku-adams-royal-engine.mshogi#payload.initial",
        RulesProfile::TenjikuAdamsModernV1,
    );
    fixture(
        &mut config,
        include_str!("../../../desktop/scripts/fixtures/tenjiku-muller-royal-engine.mshogi"),
        "desktop/scripts/fixtures/tenjiku-muller-royal-engine.mshogi#payload.initial",
        RulesProfile::TenjikuMullerCompletedV1,
    );
    config.validate().unwrap();
    config
}

fn expected_outcome(profile: RulesProfile) -> Value {
    // Native facts come from the authored positions cited above, independently
    // of the actual observed search scores/decisions in this experiment.
    match profile {
        RulesProfile::ChessMatchV1 => serde_json::to_value(chess::MatchConclusion::Chess(
            chess::ChessOutcome::Checkmate {
                winner: chess::Color::White,
            },
        ))
        .unwrap(),
        RulesProfile::ShogiCsa36V1 => serde_json::to_value(shogi::Conclusion::Win {
            winner: shogi::Side::Sente,
            reason: shogi::WinReason::EnteringKing,
        })
        .unwrap(),
        RulesProfile::XiangqiWxf2018V1 => serde_json::to_value(xiangqi::Conclusion::Win {
            winner: xiangqi::Side::Red,
            reason: xiangqi::WinReason::NoLegalAction {
                reason: xiangqi::NoActionReason::Stalemate,
            },
        })
        .unwrap(),
        RulesProfile::ToriModernV1 => serde_json::to_value(tori::Conclusion::Checkmate {
            winner: tori::Side::Sente,
        })
        .unwrap(),
        RulesProfile::ChuJcsa2019V1 => serde_json::to_value(chu::Conclusion::RoyalCapture {
            winner: chu::Side::Sente,
        })
        .unwrap(),
        RulesProfile::TenjikuAdamsModernV1 | RulesProfile::TenjikuMullerCompletedV1 => {
            serde_json::to_value(tenjiku::Conclusion::RoyalCapture {
                winner: tenjiku::Side::Sente,
            })
            .unwrap()
        }
    }
}

fn rejected(report: &Report, change: impl FnOnce(&mut Value)) {
    let mut value = serde_json::to_value(report).unwrap();
    change(&mut value);
    assert!(Report::from_json(&value.to_string()).is_err());
}

#[test]
fn actual_seven_profile_games_replay_export_native_labels_and_reject_corruption() {
    let _journey = JOURNEY.lock().unwrap_or_else(|e| e.into_inner());
    let config = tactical_config();
    let report = run(&config).unwrap();
    assert_eq!(report.games.len(), 14);
    assert!(report.identities.iter().all(|id| {
        id.executable.sha256.as_ref().is_some_and(|s| s.len() == 64)
            && id.executable.error.is_none()
            && id.model.is_none()
            && id.model_identity.as_deref() == Some("executable-embedded")
    }));
    for game in &report.games {
        let opening = &config.openings[game.pairing.opening];
        let profile = opening.root.record.profile();
        assert_eq!(game.steps.len(), 1);
        let step = &game.steps[0];
        assert!(
            step.applied,
            "profile={} actual stop={} result={:?}",
            profile.id(),
            game.stop,
            step.observation
        );
        assert!(step.failure.is_none());
        let observation = step.observation.as_ref().expect("actual production result");
        assert!(observation.matches(&step.request));
        assert_eq!(step.request.game, opening.root.record);
        assert_eq!(game.stop, "native_concluded");
        assert_eq!(game.native_outcome, Some(expected_outcome(profile)));
        assert_eq!(game.score_first_twice, Some(2));
        assert_eq!(game.final_root.record.profile(), profile);
        assert_eq!(
            game.final_root.record.known_board_actions(),
            usize::from(profile != RulesProfile::ShogiCsa36V1)
        );
        assert_eq!(step.player, usize::from(game.pairing.swapped));
    }
    let encoded = report.to_json().unwrap();
    let replayed = Report::from_json(&encoded).unwrap();
    assert_eq!(replayed.to_json().unwrap(), encoded);
    let summaries = replayed.summaries().unwrap();
    assert_eq!(summaries.len(), 7);
    for summary in &summaries {
        assert_eq!(summary.scheduled_pairs, 1);
        assert_eq!(summary.complete_played_pairs, 1);
        assert_eq!(summary.unscored_pairs, 0);
        assert_eq!(summary.diagnostic_pairs, 0);
        assert_eq!(summary.pentanomial, [0, 0, 1, 0, 0]);
        assert_eq!(summary.wins_draws_losses, [1, 0, 1]);
    }
    let rows: Vec<Value> = replayed
        .data_jsonl()
        .unwrap()
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();
    assert_eq!(rows.len(), 14);
    for row in &rows {
        let index = row["game"].as_u64().unwrap() as usize;
        let game = &report.games[index];
        let opening = &config.openings[game.pairing.opening];
        assert_eq!(row["split"], "test");
        assert_eq!(row["group"], opening.group);
        assert!(
            row["native_lineage"]
                .as_str()
                .unwrap()
                .starts_with("native-root-v1:")
        );
        assert_eq!(row["source_report_sha256"], digest(&encoded));
        assert_eq!(row["config_sha256"], report.config_sha256);
        assert_eq!(row["native_outcome"], game.native_outcome.clone().unwrap());
        assert_eq!(row["terminal_value_available"], true);
        assert_eq!(row["usable_action_target"], true);
        assert_eq!(row["score_first_twice"], 2);
        assert_eq!(row["before"], serde_json::to_value(&opening.root).unwrap());
        assert_eq!(
            row["after"],
            serde_json::to_value(&game.final_root).unwrap()
        );
    }

    // Each corruption changes one independently bound identity/fact; replay
    // must not relabel a failed/changed transition as successful evidence.
    rejected(&report, |r| r["unexpected"] = json!(true));
    rejected(&report, |r| {
        r["games"][0]["steps"][0]["request"]["game"]["rules_profile"] =
            json!(
                if report.games[0].final_root.record.profile() == RulesProfile::ChessMatchV1 {
                    "mshogi.shogi.csa36.v1"
                } else {
                    "mshogi.chess.match.v1"
                }
            )
    });
    rejected(&report, |r| {
        r["games"][0]["final_root"] =
            serde_json::to_value(&config.openings[report.games[0].pairing.opening].root).unwrap()
    });
    rejected(&report, |r| {
        r["games"][0]["steps"][0]["request"]["request_id"] = json!(0)
    });
    rejected(&report, |r| {
        r["games"][0]["steps"][0]["request"]["limits"]["nodes"] = json!(0)
    });
    rejected(&report, |r| r["games"][0]["score_first_twice"] = json!(0));
    rejected(&report, |r| r["games"][0]["native_outcome"] = Value::Null);
    let tori = report
        .games
        .iter()
        .position(|g| g.final_root.record.profile() == RulesProfile::ToriModernV1)
        .unwrap();
    rejected(&report, |r| {
        r["games"][tori]["steps"][0]["observation"]["result"]["result"]["telemetry"]["score"] =
            json!(0)
    });
    let mut leaked = config.clone();
    let mut duplicate = leaked.openings[0].clone();
    duplicate.id = "same-native-lineage-different-declared-group".into();
    duplicate.group = "different-declared-group".into();
    duplicate.split = Split::Train;
    leaked.openings.push(duplicate);
    assert!(
        leaked.validate().is_err(),
        "same native lineage cannot cross Test/Train labels"
    );
    println!(
        "EXPERIMENT-ACTUAL-TACTICAL {}",
        json!({"profiles":7,"paired_games":14,"native_concluded":14,"summaries":summaries})
    );
}

fn single_chess() -> Config {
    let mut config = template(env!("CARGO_BIN_EXE_mshogi")).unwrap();
    config.players.truncate(2);
    config.openings.truncate(1);
    config.max_steps = 1;
    config.cleanup_ms = 1_000;
    config
}

#[test]
fn actual_zero_work_has_no_fallback_outcome_or_training_label() {
    let _journey = JOURNEY.lock().unwrap_or_else(|e| e.into_inner());
    let mut config = single_chess();
    for player in &mut config.players {
        for limits in &mut player.limits {
            limits.work.nodes = 0;
        }
    }
    let report = run(&config).unwrap();
    for game in &report.games {
        assert!(game.stop.starts_with("incomplete:"), "{}", game.stop);
        assert_eq!(game.steps.len(), 1);
        assert!(!game.steps[0].applied);
        assert!(game.steps[0].observation.is_some());
        assert_eq!(game.final_root, config.openings[0].root);
        assert_eq!(game.native_outcome, None);
        assert_eq!(game.score_first_twice, None);
    }
    let summary = &report.summaries().unwrap()[0];
    assert_eq!(summary.unscored_pairs, 1);
    assert_eq!(summary.unscored_games, 2);
    assert_eq!(summary.wins_draws_losses, [0, 0, 0]);
    for line in report.data_jsonl().unwrap().lines() {
        let row: Value = serde_json::from_str(line).unwrap();
        assert_eq!(row["terminal_value_available"], false);
        assert_eq!(row["usable_action_target"], false);
        assert!(row["score_first_twice"].is_null());
        assert_eq!(row["before"], row["after"]);
    }
    Report::from_json(&report.to_json().unwrap()).unwrap();
}

#[test]
fn cancellation_after_submission_retains_request_and_never_invents_response() {
    let _journey = JOURNEY.lock().unwrap_or_else(|e| e.into_inner());
    let config = single_chess();
    let mut polls = 0;
    let report = run_with_stop(&config, &mut || {
        polls += 1;
        polls >= 2
    })
    .unwrap();
    assert_eq!(report.games.len(), 2);
    assert_eq!(report.games[0].steps.len(), 1);
    let step = &report.games[0].steps[0];
    assert!(step.request.request_id > 0);
    assert_eq!(step.request.game, config.openings[0].root.record);
    assert!(step.observation.is_none());
    assert_eq!(step.failure.as_deref(), Some("cancelled"));
    assert!(!step.applied);
    assert!(report.games[1].steps.is_empty());
    for game in &report.games {
        assert_eq!(game.stop, "cancelled");
        assert_eq!(game.final_root, config.openings[0].root);
        assert_eq!(game.native_outcome, None);
        assert_eq!(game.score_first_twice, None);
    }
    let lines = report.data_jsonl().unwrap();
    assert_eq!(lines.lines().count(), 1);
    let row: Value = serde_json::from_str(lines.lines().next().unwrap()).unwrap();
    assert_eq!(row["failure"], "cancelled");
    assert!(row["observation"].is_null());
    assert_eq!(row["terminal_value_available"], false);
    assert_eq!(row["usable_action_target"], false);
    assert_eq!(row["before"], row["after"]);
    Report::from_json(&report.to_json().unwrap()).unwrap();
}
