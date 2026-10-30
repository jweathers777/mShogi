//! Full certificates are independently specified legal action transcripts, not
//! graph searches. Reuse is local to one complete native document validation.
use super::*;
use mshogi_engine::{ExecutionPurpose, ReachabilityTarget};

fn live(history_cycles: usize) -> AdjudicatedGame {
    let mut game =
        AdjudicatedGame::from_fen("7k/8/5KQ1/8/8/8/8/8 w - - 0 1", ClaimPolicy::ExplicitOnly)
            .unwrap();
    for _ in 0..history_cycles {
        for action in ["g6g5", "h8h7", "g5g6", "h7h8"] {
            game.apply(action.parse().unwrap()).unwrap();
        }
    }
    assert_eq!(game.position().side_to_move(), Color::White);
    game
}

fn proof(history_cycles: usize, witness_cycles: usize) -> (DesktopSession, ArchivedObservation) {
    let game = live(history_cycles);
    let mut session = DesktopSession::from_record(&game.record(), Color::White).unwrap();
    session
        .command(
            ChessMutationCommand::Analyze {
                version: session.version,
                operation: AnalysisOperation::FullReachability {
                    target: ReachabilityTarget::White,
                    max_positions: 100_000,
                    max_transitions: 1_000_000,
                },
            },
            1,
        )
        .unwrap();
    let context = session.analysis_request().unwrap().context;
    let cycle: Vec<Action> = ["g6g5", "h8h7", "g5g6", "h7h8"]
        .into_iter()
        .map(|text| text.parse().unwrap())
        .collect();
    let mut witness = cycle.repeat(witness_cycles);
    witness.push("g6g7".parse().unwrap());
    let work = ReachabilityWork {
        positions: witness.len() as u64 + 1,
        transitions: witness.len() as u64,
        expanded: witness.len() as u64,
        ..ReachabilityWork::default()
    };
    let observation = ArchivedObservation {
        sequence: 1,
        label: "Independent cooperative cycle followed by Qg7 mate".into(),
        observation: ExecutionObservation::new(
            context,
            ExecutionResult::Chess {
                result: ChessResult::Reachability {
                    result: MateReachability::Possible { witness, work },
                },
            },
        ),
        historical: false,
        analyzed_turn: Color::White,
        application_error: None,
    };
    (session, observation)
}

fn fresh_identity(observation: &ArchivedObservation) -> ArchivedObservation {
    let mut next = observation.clone();
    next.sequence += 1;
    let context = &mut next.observation.context;
    context.request_id += 10;
    context.process_generation += 20;
    context.session_version += 30;
    let ExecutionPurpose::ChessFullReachability { target, .. } = &mut context.purpose else {
        unreachable!()
    };
    target.selection_id += 40;
    target.session_version = context.session_version;
    target.label = "A different request at the same exact native history".into();
    next
}

fn witness_mut(observation: &mut ArchivedObservation) -> &mut Vec<Action> {
    let ExecutionResult::Chess {
        result:
            ChessResult::Reachability {
                result: MateReachability::Possible { witness, .. },
            },
    } = &mut observation.observation.result
    else {
        unreachable!()
    };
    witness
}

fn payload(session: &DesktopSession, observations: Vec<ArchivedObservation>) -> ChessDocument {
    let mut archive = session.archive_snapshot();
    archive.match_record_json = session.game.record().to_json();
    archive.game_record_json = session.game.game().record().to_json();
    archive.observations = observations;
    archive.version = 1000;
    archive.revision = 1000;
    archive.next_request = 1000;
    archive.document_revision = 1000;
    archive
}

fn open(archive: &ChessDocument) -> Result<Opened, String> {
    crate::shared::archive::decode_chess_document(&encode_chess(archive).unwrap(), &mut || false)
        .map_err(String::from)
}

#[test]
fn full_certificate_reuse_is_operation_local_cancellable_and_ignores_only_runtime_identity() {
    let (session, first) = proof(0, 10_000);
    let second = fresh_identity(&first);
    assert!(has_full_certificate(&first));
    assert!(same_full_certificate(&first, &second));
    let final_record = session.game.game().record();
    let mut cache = Vec::new();
    let mut first_polls = 0;
    validate_observation_lineage(&first, &final_record, &[], &mut cache, &mut || {
        first_polls += 1;
        false
    })
    .unwrap();
    assert!(
        first_polls > 40_000,
        "the first proof must actually verify the full legal transcript"
    );
    assert_eq!(cache.len(), 1);
    let mut duplicate_polls = 0;
    validate_observation_lineage(&second, &final_record, &[], &mut cache, &mut || {
        duplicate_polls += 1;
        false
    })
    .unwrap();
    assert!(
        duplicate_polls < 100,
        "identical native certificate must not repeat 40,001 action checks"
    );
    assert_eq!(cache.len(), 1);
    assert!(
        validate_observation_lineage(&second, &final_record, &[], &mut cache, &mut || true)
            .is_err()
    );
    let mut new_operation = Vec::new();
    let mut fresh_polls = 0;
    validate_observation_lineage(&second, &final_record, &[], &mut new_operation, &mut || {
        fresh_polls += 1;
        false
    })
    .unwrap();
    assert!(
        fresh_polls > 40_000,
        "another Open must validate its own first certificate"
    );
    let archive = payload(&session, vec![first, second]);
    assert_eq!(open(&archive).unwrap().archive.observations.len(), 2);
}

#[test]
fn distinct_witnesses_and_same_board_different_history_never_share_certificate_verification() {
    let (_, first) = proof(0, 1);
    let (_, distinct) = proof(0, 2);
    let (history_session, historical) = proof(1, 1);
    assert!(!same_full_certificate(&first, &distinct));
    assert!(!same_full_certificate(&first, &historical));
    assert_eq!(
        live(0).position().to_fen().split_whitespace().next(),
        live(1).position().to_fen().split_whitespace().next()
    );
    assert_ne!(
        first.observation.context.game,
        historical.observation.context.game
    );
    let final_record = history_session.game.game().record();
    let mut cache = Vec::new();
    for certificate in [&first, &distinct, &historical] {
        let before = cache.len();
        let mut checks = 0;
        validate_observation_lineage(certificate, &final_record, &[], &mut cache, &mut || {
            checks += 1;
            false
        })
        .unwrap();
        assert_eq!(cache.len(), before + 1);
        assert!(
            checks >= 5,
            "every distinct legal witness must be checked natively"
        );
    }
}

#[test]
fn later_duplicate_cannot_bypass_context_perspective_goal_limits_or_work_validation() {
    let (session, first) = proof(0, 1);
    let duplicate = fresh_identity(&first);
    for case in [
        "target",
        "turn",
        "session",
        "request",
        "process",
        "history",
        "purpose",
        "perspective",
        "goal",
        "limits",
        "work",
        "witness",
    ] {
        let mut bad = duplicate.clone();
        match case {
            "target" => {
                let ExecutionPurpose::ChessFullReachability { target, .. } =
                    &mut bad.observation.context.purpose
                else {
                    unreachable!()
                };
                target.event_index += 1;
            }
            "session" => {
                bad.observation.context.session_version += 1;
            }
            "turn" => {
                let ExecutionPurpose::ChessFullReachability { target, .. } =
                    &mut bad.observation.context.purpose
                else {
                    unreachable!()
                };
                target.turn = NativeSide::Chess(Color::Black);
            }
            "request" => {
                bad.observation.context.request_id = 0;
            }
            "process" => {
                bad.observation.context.process_generation = 0;
            }
            "history" => {
                bad.observation.context.game = CanonicalRecord::chess(live(1).record());
            }
            "purpose" => {
                bad.observation.context.purpose = ExecutionPurpose::EngineTurn {};
            }
            "perspective" => {
                bad.analyzed_turn = Color::Black;
            }
            "goal" => {
                let ExecutionPurpose::ChessFullReachability { mate, .. } =
                    &mut bad.observation.context.purpose
                else {
                    unreachable!()
                };
                *mate = ReachabilityTarget::Black;
            }
            "limits" => {
                let ExecutionPurpose::ChessFullReachability { limits, .. } =
                    &mut bad.observation.context.purpose
                else {
                    unreachable!()
                };
                limits.max_positions = 1;
            }
            "work" => {
                let ExecutionResult::Chess {
                    result:
                        ChessResult::Reachability {
                            result: MateReachability::Possible { work, .. },
                        },
                } = &mut bad.observation.result
                else {
                    unreachable!()
                };
                work.expanded = work.positions + 1;
            }
            "witness" => {
                *witness_mut(&mut bad) = vec!["g6g5".parse().unwrap()];
            }
            _ => unreachable!(),
        }
        assert!(
            open(&payload(&session, vec![first.clone(), bad])).is_err(),
            "accepted corrupt later {case}"
        );
    }
}

#[test]
fn failed_certificates_do_not_enter_operation_cache() {
    let (session, mut bad) = proof(0, 1);
    *witness_mut(&mut bad) = vec!["g6g5".parse().unwrap()];
    let mut cache = Vec::new();
    assert!(
        validate_observation_lineage(
            &bad,
            &session.game.game().record(),
            &[],
            &mut cache,
            &mut || false
        )
        .is_err()
    );
    assert!(cache.is_empty());
    let (_, good) = proof(0, 1);
    validate_observation_lineage(
        &good,
        &session.game.game().record(),
        &[],
        &mut cache,
        &mut || false,
    )
    .unwrap();
    assert_eq!(cache.len(), 1);
}

#[test]
fn complete_candidate_admission_rechecks_distinct_prior_full_proofs_past_the_high_watermark() {
    let (mut session, first) = proof(0, 1000);
    let (_, mut second) = proof(0, 1001);
    second.sequence = 2;
    session.observations = vec![first, second];
    // First certificate is immutable prior evidence; the second is new work.
    session.validated_observations = session.observations.len() - 1;
    session.next_request = 1000;
    session.version = 1000;
    session.archive_revision = 1000;
    session.document_revision = 1000;
    let mut checks = 0;
    session
        .check_archive_capacity(&mut || {
            checks += 1;
            false
        })
        .unwrap();
    assert!(
        checks > 8000,
        "complete admission must validate both distinct prior transcripts, even past the ordinary-observation high-water mark"
    );
    let before = serde_json::to_string(&session.observations).unwrap();
    let before_match = session.game.record().to_json();
    let before_revision = session.document_revision;
    let mut interrupted_checks = 0;
    assert!(
        session
            .check_archive_capacity(&mut || {
                interrupted_checks += 1;
                interrupted_checks >= 2000
            })
            .is_err()
    );
    assert!(
        interrupted_checks >= 2000,
        "cancellation must interrupt complete native certificate validation, not skip it"
    );
    assert_eq!(
        serde_json::to_string(&session.observations).unwrap(),
        before
    );
    assert_eq!(session.game.record().to_json(), before_match);
    assert_eq!(session.document_revision, before_revision);
    *witness_mut(&mut session.observations[0]) = vec!["g6g5".parse().unwrap()];
    let before = serde_json::to_string(&session.observations).unwrap();
    let error = session.check_archive_capacity(&mut || false).unwrap_err();
    assert!(error.contains("mate"), "{error}");
    assert_eq!(
        serde_json::to_string(&session.observations).unwrap(),
        before
    );
    assert!(session.check_archive_capacity(&mut || true).is_err());
}
