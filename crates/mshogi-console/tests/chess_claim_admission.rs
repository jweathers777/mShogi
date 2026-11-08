//! Actual child proof that a leaf draw claim is a complete-command PV tail.
use mshogi_core::{
    chess::{
        AdjudicatedGame, ChessOutcome, ClaimPolicy, Color, DrawClaim, DrawReason, MatchConclusion,
        MatchRecord,
    },
    native::{CanonicalRecord, RulesProfile},
};
use mshogi_engine::{ChessResult, EngineChoice, ExecutionResult};
use mshogi_runner::experiments::{Opening, Report, Split, fresh_root, run, template};

#[test]
fn actual_depth_one_leaf_claim_tail_is_admitted_without_playing_intended_move() {
    // This accepted native/search fixture has no claim for White at counter98.
    // Every quiet move lets Black claim by an intended quiet move at counter99.
    // Search depth counts board plies; that terminal claim adds a command, not
    // another played board ply. The final actual match must contain one move.
    let source =
        AdjudicatedGame::from_fen("7k/8/8/8/8/8/8/KR6 w - - 98 1", ClaimPolicy::AutoClaim).unwrap();
    let original = source.clone();
    assert!(source.available_claims().is_empty());
    let root = fresh_root(CanonicalRecord::chess(source.record())).unwrap();
    let mut config = template(env!("CARGO_BIN_EXE_mshogi")).unwrap();
    config.id = "actual-chess-leaf-claim-admission".into();
    config.protocol = "Actual production child, independently established native fifty-move \
        intended-claim fixture from search::auto_claim_is_respected_at_leaf_turn_boundaries. \
        Two identical participants with reversed native colors; not strength evidence."
        .into();
    config.players.truncate(2);
    for player in &mut config.players {
        player.build_provenance = "Actual CARGO_BIN_EXE_mshogi from this integration build; \
            executable SHA is retained by the report."
            .into();
        player
            .limits
            .retain(|l| l.profile == RulesProfile::ChessMatchV1);
        player.limits[0].work.depth = 1;
        player.limits[0].work.nodes = 1_000;
    }
    config.openings = vec![Opening {
        id: "counter98-intended-claim".into(),
        group: "accepted-native-chess-counter98".into(),
        root: root.clone(),
        split: Split::Test,
        provenance: "Project-authored accepted search regression; fifty-move count98, \
            no initial claim, a quiet White move enables Black's intended claim."
            .into(),
    }];
    config.max_steps = 2;
    config.wall_budget_ms = 30_000;
    config.watchdog_ms = 10_000;
    config.cleanup_ms = 1_000;
    let input_json = serde_json::to_string(&config).unwrap();
    let report = run(&config).unwrap();
    assert_eq!(
        source, original,
        "native source position/history/policy unchanged"
    );
    assert_eq!(serde_json::to_string(&config).unwrap(), input_json);
    assert_eq!(report.games.len(), 2);
    for game in &report.games {
        let first = &game.steps[0];
        assert!(
            first.applied,
            "valid depth1 move+leaf-claim refused: stop={}, failure={:?}, observation={:?}",
            game.stop, first.failure, first.observation
        );
        assert_eq!(first.request.game, root.record);
        assert_eq!(first.player, usize::from(game.pairing.swapped));
        let observation = first.observation.as_ref().unwrap();
        assert!(observation.matches(&first.request));
        let ExecutionResult::Chess {
            result: ChessResult::Move { action, telemetry },
        } = &observation.result
        else {
            panic!("expected actual native move observation: {observation:?}");
        };
        assert_eq!(telemetry.completed_depth, 1);
        assert_eq!(telemetry.score, Some(0));
        assert_eq!(telemetry.principal_variation.len(), 2);
        assert_eq!(
            telemetry.principal_variation[0],
            EngineChoice::Move { action: *action }
        );
        let EngineChoice::Claim { claim } = telemetry.principal_variation[1] else {
            panic!("expected terminal claim tail: {telemetry:?}");
        };
        assert_eq!(claim.actor, Color::Black);
        assert_eq!(claim.reason, DrawClaim::FiftyMove);
        assert!(
            claim.intended.is_some(),
            "intended action is not a played move"
        );
        assert_eq!(game.steps.len(), 2);
        assert!(game.steps.iter().all(|s| s.applied && s.failure.is_none()));
        let second = &game.steps[1];
        assert_eq!(second.request.game.known_board_actions(), 1);
        assert_eq!(second.request.game.profile(), RulesProfile::ChessMatchV1);
        assert!(matches!(
            &second.observation.as_ref().unwrap().result,
            ExecutionResult::Chess {
                result: ChessResult::Claim { .. }
            }
        ));
        assert_eq!(game.stop, "native_concluded");
        assert_eq!(game.score_first_twice, Some(1));
        assert_eq!(game.final_root.record.known_board_actions(), 1);
        assert_eq!(game.final_root.record.profile(), RulesProfile::ChessMatchV1);
        let native = MatchRecord::from_json(game.final_root.chess_match.as_ref().unwrap())
            .unwrap()
            .replay()
            .unwrap();
        assert_eq!(
            native.conclusion(),
            Some(MatchConclusion::Chess(ChessOutcome::Draw(
                DrawReason::Claimed(DrawClaim::FiftyMove)
            )))
        );
        assert_eq!(native.game().history().ply_count(), 1);
        assert!(game.cleanup.is_empty());
    }
    let json = report.to_json().unwrap();
    let replayed = Report::from_json(&json).unwrap();
    assert_eq!(replayed.to_json().unwrap(), json);
}
