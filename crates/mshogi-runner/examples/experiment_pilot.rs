//! Generate bounded seven-profile diagnostic inputs; this does not run engines.
use mshogi_core::{
    chess,
    native::{CanonicalRecord, RulesProfile},
    shogi, xiangqi,
};
use mshogi_runner::experiments::{Config, Opening, Split, fresh_root, template};
use serde_json::Value;
use std::{error::Error, fs::OpenOptions, io::Write};

fn add(
    config: &mut Config,
    record: CanonicalRecord,
    source: &str,
    facts: &str,
) -> Result<(), String> {
    let profile = record.profile();
    config.openings.push(Opening {
        id: format!("tactical-{}", profile.id()),
        group: format!("msh48-pilot:{}:{source}", profile.id()),
        root: fresh_root(record)?,
        split: Split::Test,
        provenance: format!(
            "Project-authored native diagnostic from {source}. {facts} \
             Selected as a live predecessor to a genuine native ending. \
             Test-only diagnostic lineage; no external training corpus or inferred licensing grant."
        ),
    });
    Ok(())
}

fn fixture(
    config: &mut Config,
    source: &str,
    text: &str,
    expected: RulesProfile,
    facts: &str,
) -> Result<(), String> {
    let document: Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let initial = document
        .get("payload")
        .and_then(|payload| payload.get("initial"))
        .ok_or_else(|| format!("{source} lacks its initial canonical native record"))?;
    // Only trusted committed fixture JSON is selected here. The extracted
    // canonical record still goes through its authoritative strict decoder.
    let record = CanonicalRecord::from_json(&initial.to_string()).map_err(|e| e.to_string())?;
    if record.profile() != expected {
        return Err(format!("{source} has an unexpected native profile"));
    }
    add(config, record, source, facts)
}

fn generate(executable: &str) -> Result<Config, String> {
    let mut config = template(executable).map_err(|e| e.to_string())?;
    config.id = "msh48-seven-profile-bounded-pilot".into();
    config.protocol = "MSH-48 native infrastructure pilot: seven exact standard roots and seven \
        authored live tactical predecessors, one seeded catalog cycle, paired native sides, \
        candidate versus two unequal-work baselines. Every lineage is Test-only. \
        Candidate depth1/nodes512, baseline-low depth1/nodes128, baseline-deeper depth2/nodes512; \
        these are finite diagnostic defaults to measure before choosing actual pilot budgets, \
        not strength targets or fair equal-resource comparisons. At most two accepted commands \
        per game and 120 seconds whole-run wall budget; unfinished work stays unscored. \
        The current source Cargo build receipt and supplied executable identity must be verified \
        separately before using results. Actual host/build/elapsed observations belong in the \
        generated report. No trained model, external training corpus, strength verdict or \
        substantial training is supplied by this pilot."
        .into();
    let build = format!(
        "Declared MSH-48 pilot participant from current source; independently verify the supplied \
         executable against its Cargo build receipt and observed SHA256. Generator build: \
         revision={}, dirty-diff-sha256={}, rustc={}, target={}, profile={}, flags={}. \
         The generator receipt does not authenticate a separately supplied executable. \
         Embedded native baseline evaluator; different-work diagnostic, not a strength claim.",
        env!("EXPERIMENT_BUILD_REVISION"),
        env!("EXPERIMENT_BUILD_DIRTY_SHA256"),
        env!("EXPERIMENT_BUILD_RUSTC"),
        env!("EXPERIMENT_BUILD_TARGET"),
        env!("EXPERIMENT_BUILD_PROFILE"),
        env!("EXPERIMENT_BUILD_FLAGS"),
    );
    for player in &mut config.players {
        player.build_provenance = build.clone();
    }

    let game = chess::AdjudicatedGame::from_fen(
        "7k/8/5KQ1/8/8/8/8/8 w - - 0 1",
        chess::ClaimPolicy::AutoClaim,
    )
    .map_err(|e| e.to_string())?;
    add(
        &mut config,
        CanonicalRecord::chess(game.record()),
        "crates/mshogi-search/tests/search.rs::mate_in_one_for_both_colors_and_already_mated_root",
        "Authored Chess mate-in-one: White Kf6/Qg6 versus Kh8, g6g7 checkmates; imported FEN begins known history, M2 AutoClaim and a fresh untimed full match ledger.",
    )?;

    let root = shogi::GameRoot::import(
        shogi::Position::from_sfen("K8/1RB6/1+P+P+P+P+P+P+P+P/9/9/9/9/9/8k b B5P 1")
            .map_err(|e| e.to_string())?,
        shogi::RootCategory::ReducedMaterialDiagram,
        shogi::CounterSource::Supplied,
    )
    .map_err(|e| format!("native Shogi root: {e:?}"))?;
    let game = shogi::Game::from_root(root);
    add(
        &mut config,
        CanonicalRecord::shogi(shogi::GameRecord::from_game(&game)),
        "crates/mshogi-console/tests/shogi_engine.rs::actual_analysis_declaration_incomplete_and_history_contexts_remain_distinct",
        "Authored reduced-material CSA declaration diagram with supplied counter; native entering-king eligibility permits Sente's formal declaration. The actual declaration command, not a board move or advisory score, establishes the native win.",
    )?;

    let game = xiangqi::Game::from_root(
        xiangqi::PositionRoot::from_fen("5k3/9/4R4/9/9/9/9/9/9/3K5 w - - 0 1")
            .map_err(|e| e.to_string())?,
        xiangqi::ClaimPolicy::Explicit,
    )
    .map_err(|e| e.to_string())?;
    add(
        &mut config,
        CanonicalRecord::xiangqi(xiangqi::GameRecord::from_game(&game).map_err(|e| e.to_string())?),
        "crates/mshogi-search/tests/xiangqi.rs::immediate_stalemate_is_a_native_win_for_each_side",
        "Authored MSH-105 P1 diagram: Red rook e7e8 controls e9 and f8, leaving Black f9 unchecked with no legal action; the selected native profile awards Red a stalemate win. Explicit native claim policy is preserved.",
    )?;

    fixture(
        &mut config,
        "desktop/scripts/fixtures/tori-mate-in-one.mshogi#payload.initial",
        include_str!("../../../desktop/scripts/fixtures/tori-mate-in-one.mshogi"),
        RulesProfile::ToriModernV1,
        "Accepted native TORI-O-MATE predecessor: Sente Falcon 4c-4b promoting to Eagle, protected by Pheasant 4d, checks the blocked Gote Phoenix at 4a and establishes native checkmate.",
    )?;
    fixture(
        &mut config,
        "desktop/scripts/fixtures/chu-complete-royal-engine.mshogi#payload.initial",
        include_str!("../../../desktop/scripts/fixtures/chu-complete-royal-engine.mshogi"),
        RulesProfile::ChuJcsa2019V1,
        "Accepted native live Lion predecessor: the complete ordered 6f-6e-5e route captures Gote King and promoted Drunk Elephant/Prince in one turn, establishing RoyalCapture. The committed document also retains the genuine engine choice and Finished observation.",
    )?;
    fixture(
        &mut config,
        "desktop/scripts/fixtures/tenjiku-adams-royal-engine.mshogi#payload.initial",
        include_str!("../../../desktop/scripts/fixtures/tenjiku-adams-royal-engine.mshogi"),
        RulesProfile::TenjikuAdamsModernV1,
        "Accepted native live royal-capture predecessor under immutable Adams1999 Modern semantics; actual complete action and ordered native effects must establish the ending, with no resignation injection.",
    )?;
    fixture(
        &mut config,
        "desktop/scripts/fixtures/tenjiku-muller-royal-engine.mshogi#payload.initial",
        include_str!("../../../desktop/scripts/fixtures/tenjiku-muller-royal-engine.mshogi"),
        RulesProfile::TenjikuMullerCompletedV1,
        "Accepted native live royal-capture predecessor under immutable Muller Completed semantics; actual complete action and ordered native effects must establish the ending, with no resignation injection.",
    )?;
    config.validate().map_err(|e| e.to_string())?;
    Ok(config)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let executable = args
        .next()
        .ok_or("usage: experiment_pilot EXE CONFIG_PATH")?;
    let output = args
        .next()
        .ok_or("usage: experiment_pilot EXE CONFIG_PATH")?;
    if args.next().is_some() {
        return Err("usage: experiment_pilot EXE CONFIG_PATH".into());
    }
    let config = generate(executable.to_str().ok_or("executable path must be UTF8")?)?;
    let json = serde_json::to_vec_pretty(&config)?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(output)?;
    file.write_all(&json)?;
    file.write_all(b"\n")?;
    println!(
        "wrote {} Test-only native roots, {} participants, paired sides, {} command cap, {} ms wall budget",
        config.openings.len(),
        config.players.len(),
        config.max_steps,
        config.wall_budget_ms
    );
    Ok(())
}
