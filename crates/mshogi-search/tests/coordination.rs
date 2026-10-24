//! Real native-worker tests, with event/predicate waits instead of race sleeps.
use mshogi_core::chess::{AdjudicatedGame, ClaimPolicy};
use mshogi_search::{coordination::*, search::*};
use std::time::{Duration, Instant};

fn limits(depth: u16) -> SearchLimits {
    SearchLimits { depth, nodes: None }
}
fn wait_for<T>(mut observe: impl FnMut() -> Option<T>) -> T {
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        if let Some(value) = observe() {
            return value;
        }
        assert!(
            Instant::now() < deadline,
            "native worker did not acknowledge within test deadline"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn complete(coordinator: &SearchCoordinator) -> SearchCompletion {
    wait_for(|| coordinator.poll())
}
fn wait_ack(coordinator: &SearchCoordinator, context: &RequestContext) {
    wait_for(|| coordinator.acknowledged(context).then_some(()));
}
#[test]
fn request_and_result_keep_exact_context_without_mutating_caller() {
    let coordinator = SearchCoordinator::new().unwrap();
    let game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let before = game.clone();
    let context = coordinator.submit(&game, 42, limits(1)).unwrap();
    assert_eq!(context.version(), 42);
    assert_eq!(context.base(), &game.record());
    assert_eq!(context.limits(), limits(1));
    let completion = complete(&coordinator);
    assert_eq!(completion.context(), &context);
    assert_eq!(
        completion.result().as_ref().unwrap(),
        &search(&game, limits(1)).unwrap()
    );
    assert_eq!(coordinator.accept_result(&completion, &game, 42), Ok(()));
    assert!(coordinator.poll().is_none(), "single result consumed once");
    assert_eq!(
        coordinator.accept_result(&completion, &game, 43),
        Err(StaleResult::VersionChanged)
    );
    assert_eq!(game, before);
    wait_ack(&coordinator, &context);
}
#[test]
fn recorded_adjudication_mode_is_part_of_request_identity() {
    use mshogi_core::chess::{AdjudicationMode, ReachabilityLimits, STANDARD_START_FEN};
    let coordinator = SearchCoordinator::new().unwrap();
    let game = AdjudicatedGame::from_fen_with_mode(
        STANDARD_START_FEN,
        ClaimPolicy::ExplicitOnly,
        AdjudicationMode::Full {
            limits: ReachabilityLimits {
                max_positions: Some(0),
                max_transitions: None,
            },
        },
    )
    .unwrap();
    let before = game.clone();
    let context = coordinator.submit(&game, 1, limits(1)).unwrap();
    assert_eq!(context.base().adjudication_mode(), game.adjudication_mode());
    let result = complete(&coordinator);
    assert_eq!(coordinator.accept_result(&result, &game, 1), Ok(()));
    let legacy = AdjudicatedGame::from_fen(STANDARD_START_FEN, ClaimPolicy::ExplicitOnly).unwrap();
    assert_eq!(
        coordinator.accept_result(&result, &legacy, 1),
        Err(StaleResult::ContextChanged)
    );
    assert_eq!(game, before);
}
#[test]
fn delayed_result_after_position_change_is_rejected_even_without_version_bump() {
    let coordinator = SearchCoordinator::new().unwrap();
    let mut game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    coordinator.submit(&game, 1, limits(1)).unwrap();
    let delayed = complete(&coordinator);
    game.apply("e2e4".parse().unwrap()).unwrap();
    let changed = game.clone();
    assert_eq!(
        coordinator.accept_result(&delayed, &game, 1),
        Err(StaleResult::ContextChanged)
    );
    assert_eq!(game, changed);
}
#[test]
fn identical_current_fen_with_different_history_rejects_delayed_result() {
    let coordinator = SearchCoordinator::new().unwrap();
    let mut repeated = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    for text in ["g1f3", "g8f6", "f3g1", "f6g8"] {
        repeated.apply(text.parse().unwrap()).unwrap();
    }
    let imported =
        AdjudicatedGame::from_fen(&repeated.position().to_fen(), ClaimPolicy::ExplicitOnly)
            .unwrap();
    assert_eq!(repeated.position(), imported.position());
    coordinator
        .submit(
            &repeated,
            7,
            SearchLimits {
                depth: 1,
                nodes: Some(0),
            },
        )
        .unwrap();
    let delayed = complete(&coordinator);
    assert_eq!(
        coordinator.accept_result(&delayed, &imported, 7),
        Err(StaleResult::ContextChanged)
    );
    let new_request = coordinator.submit(&imported, 7, limits(1)).unwrap();
    assert_ne!(delayed.context().id(), new_request.id());
    assert_eq!(
        coordinator.accept_result(&delayed, &repeated, 7),
        Err(StaleResult::Superseded)
    );
}
#[test]
fn request_ids_prevent_aba_and_are_unique_across_coordinators() {
    let coordinator = SearchCoordinator::new().unwrap();
    let other = SearchCoordinator::new().unwrap();
    let game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let a = coordinator.submit(&game, 7, limits(1)).unwrap();
    let old = complete(&coordinator);
    let b = coordinator.submit(&game, 8, limits(1)).unwrap();
    let restored = coordinator.submit(&game, 7, limits(1)).unwrap();
    let external = other.submit(&game, 7, limits(1)).unwrap();
    let ids = [a.id(), b.id(), restored.id(), external.id()];
    for i in 0..ids.len() {
        for j in i + 1..ids.len() {
            assert_ne!(ids[i], ids[j]);
        }
    }
    assert_eq!(
        coordinator.accept_result(&old, &game, 7),
        Err(StaleResult::Superseded)
    );
    let current = complete(&coordinator);
    assert_eq!(current.context(), &restored);
    assert_eq!(coordinator.accept_result(&current, &game, 7), Ok(()));
    assert_eq!(
        other.accept_result(&current, &game, 7),
        Err(StaleResult::Superseded)
    );
}
#[test]
fn cancellation_is_nonjoining_acknowledged_and_preserves_caller() {
    let coordinator = SearchCoordinator::new().unwrap();
    let game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let before = game.clone();
    assert_eq!(coordinator.cancel(), None);
    let request = coordinator.submit(&game, 1, limits(MAX_DEPTH)).unwrap();
    let started = Instant::now();
    assert_eq!(coordinator.cancel(), Some(request.id()));
    // Generous test deadline guards a blocking join, not a real-time promise.
    assert!(started.elapsed() < Duration::from_secs(5));
    let completion = complete(&coordinator);
    assert_eq!(
        completion.result().as_ref().unwrap().status,
        SearchStatus::Stopped
    );
    assert_eq!(coordinator.accept_result(&completion, &game, 1), Ok(()));
    wait_ack(&coordinator, &request);
    assert_eq!(game, before);
}
#[test]
fn repeated_supersession_keeps_only_latest_pending_request_and_result() {
    let coordinator = SearchCoordinator::new().unwrap();
    let game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let first = coordinator.submit(&game, 0, limits(MAX_DEPTH)).unwrap();
    let mut latest = first.clone();
    for version in 1..=200 {
        latest = coordinator
            .submit(
                &game,
                version,
                SearchLimits {
                    depth: 1,
                    nodes: Some(0),
                },
            )
            .unwrap();
    }
    let completion = complete(&coordinator);
    assert_eq!(completion.context(), &latest);
    assert_eq!(
        completion.result().as_ref().unwrap().status,
        SearchStatus::NodeLimit
    );
    assert!(coordinator.poll().is_none());
    wait_ack(&coordinator, &first);
    assert_eq!(coordinator.accept_result(&completion, &game, 200), Ok(()));
}
#[test]
fn shutdown_discards_work_and_rejects_new_submissions_without_joining() {
    let coordinator = SearchCoordinator::new().unwrap();
    let game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let request = coordinator.submit(&game, 1, limits(MAX_DEPTH)).unwrap();
    coordinator.shutdown();
    assert_eq!(
        coordinator.submit(&game, 2, limits(1)),
        Err(SubmitError::ShuttingDown)
    );
    wait_for(|| coordinator.shutdown_acknowledged().then_some(()));
    wait_ack(&coordinator, &request);
    assert!(coordinator.poll().is_none());
}
#[test]
fn invalid_submission_does_not_supersede_current_work() {
    let coordinator = SearchCoordinator::new().unwrap();
    let game = AdjudicatedGame::new(ClaimPolicy::ExplicitOnly);
    let request = coordinator.submit(&game, 1, limits(1)).unwrap();
    assert_eq!(
        coordinator.submit(&game, 2, limits(0)),
        Err(SubmitError::Search(SearchError::InvalidDepth(0)))
    );
    let completion = complete(&coordinator);
    assert_eq!(completion.context(), &request);
    assert_eq!(coordinator.accept_result(&completion, &game, 1), Ok(()));
}
