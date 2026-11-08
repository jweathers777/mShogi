//! Independently sourced native scenarios for ordinary packaged GUI Load.
//! Development-only example; neither this generator nor its archives is bundled.
//! Expected rules facts come from ratified MSH-82/86 fixtures, not engine output.
#[allow(dead_code)]
#[path = "support/chu.rs"]
mod roots;

use mshogi_core::{chu as c, native::CanonicalRecord};
use mshogi_desktop_session::chu_document::{ChuControl, ValidatedChuDocument};
use mshogi_engine::{
    CHU_PLAY_LIMITS, ChuResult, ExecutionContext, ExecutionPurpose, ExecutionRequest,
    ExecutionResponse, ExecutionResult, PROTOCOL_VERSION,
};
use serde_json::{Value, json};
use std::{io::Cursor, path::PathBuf};

fn play(g: &mut c::Game, action: c::Action) {
    command(
        g,
        g.position().side_to_move(),
        c::GameCommand::Play { action },
    );
}
fn command(g: &mut c::Game, actor: c::Side, command: c::GameCommand) {
    g.submit(c::GameRequest {
        expected: g.context(),
        actor,
        command,
    })
    .unwrap();
}
fn from_fixture(id: &str) -> c::Game {
    roots::root(&roots::fixture(id)["given"]["root"])
}
fn document(g: &c::Game, human: c::Side) -> ValidatedChuDocument {
    ValidatedChuDocument::new(c::GameRecord::from_game(g).unwrap(), human).unwrap()
}
fn replay(d: &ValidatedChuDocument) {
    let decoded = ValidatedChuDocument::decode(d.bytes(), &mut || false).unwrap();
    assert_eq!(decoded.bytes(), d.bytes());
    assert_eq!(decoded.record(), d.record());
    assert_eq!(decoded.human(), d.human());
    assert_eq!(
        decoded.record().to_game().unwrap().events(),
        d.record().to_game().unwrap().events()
    );
}
fn case(
    name: &str,
    source: &str,
    g: &c::Game,
    human: c::Side,
    expected: Value,
) -> (String, ValidatedChuDocument, Value) {
    let d = document(g, human);
    replay(&d);
    (
        name.into(),
        d,
        json!({ "source_fixture":source, "root":"independently_authored_diagram", "expected":expected }),
    )
}
fn scenarios() -> Vec<(String, ValidatedChuDocument, Value)> {
    let mut cases = vec![];
    let mut add = |name, id, human, expected| {
        cases.push(case(name, id, &from_fixture(id), human, expected));
    };
    add(
        "contract",
        "CHU-RETURN-Ln-enemy",
        c::Side::Sente,
        json!({
            "capture_return":"6fx7e-6f", "quiet_return":"6f-6e-6f", "capture_count":1,
            "same_endpoint_different_complete_actions":true, "partial_selection_commits":false
        }),
    );
    add(
        "captures",
        "CHU-LION-C2-1",
        c::Side::Sente,
        json!({
            "action":"3cx2bx1b", "ordered_captures":["2b","1b"], "concluded_after_first":false
        }),
    );
    add(
        "promotion",
        "CHU-PROMOTE-DE-YES",
        c::Side::Sente,
        json!({
        "action_geometry":{"from":"6e","route":["6d"],"promote":true}, "notation":"DE6e-6d^", "optional_nonpromotion":true, "result":"Prince", "King_survives":true
        }),
    );
    add(
        "royal-both",
        "CHU-ROYAL-BOTH-COMPOUND",
        c::Side::Sente,
        json!({
            "action":"6fx6ex5e", "ordered_captures":["6e","5e"], "conclusion":{"kind":"royal_capture","winner":"sente"}
        }),
    );
    add(
        "royal-survivor",
        "CHU-ROYAL-SURVIVOR",
        c::Side::Gote,
        json!({
            "action":"12jx12l", "remaining_sente_royals":["8j"], "concluded":false
        }),
    );
    add(
        "material-reply",
        "CHU-MATERIAL-DECLINE-COUNTERCAPTURE",
        c::Side::Gote,
        json!({
            "action":"6d-5c", "equalizer":"6dx6e", "conclusion":{"kind":"material","winner":"sente"}
        }),
    );
    add(
        "mate",
        "CHU-MATE",
        c::Side::Sente,
        json!({"conclusion":{"kind":"mate","winner":"gote"}}),
    );
    add(
        "agreement-root",
        "CHU-MATERIAL-AGREEMENT",
        c::Side::Sente,
        json!({"root_concluded":false,"offer_alone_concludes":false}),
    );

    // Exact independently reviewed first10 commands; never commit the forbidden
    // fourth occurrence or substitute a repetition-loss rule.
    let f = roots::fixture("CHU-REP-QUIET");
    let mut g = from_fixture("CHU-REP-QUIET");
    for (i, a) in f["given"]["commands"]
        .as_array()
        .unwrap()
        .iter()
        .take(10)
        .enumerate()
    {
        play(&mut g, roots::action(a));
        if i == 7 || i == 9 {
            assert!(g.conclusion().is_none());
            assert!(
                g.obligations()
                    .iter()
                    .any(|o| o.responsible_sides == [c::Side::Sente])
            );
            cases.push(case(if i == 7 { "repetition8" } else { "repetition10" }, "CHU-REP-QUIET", &g, c::Side::Sente,
                json!({"accepted_board_actions":i+1,"responsible_side":"sente","outcome":null,"forbidden_at_prefix10":"1k-1l"})));
        }
    }
    let before = c::GameRecord::from_game(&g).unwrap();
    let forbidden = roots::action(&f["given"]["commands"][10]);
    assert!(!g.legal_actions().contains(&forbidden));
    assert!(
        g.submit(c::GameRequest {
            expected: g.context(),
            actor: c::Side::Sente,
            command: c::GameCommand::Play { action: forbidden }
        })
        .is_err()
    );
    assert_eq!(c::GameRecord::from_game(&g).unwrap(), before);

    let mut agreement = from_fixture("CHU-MATERIAL-AGREEMENT");
    command(&mut agreement, c::Side::Sente, c::GameCommand::OfferDraw);
    assert!(agreement.conclusion().is_none());
    assert_eq!(agreement.position().side_to_move(), c::Side::Sente);
    cases.push(case("agreement", "CHU-MATERIAL-AGREEMENT", &agreement, c::Side::Gote,
        json!({"offer_by":"sente","human":"gote","board_turn":"sente","accept_draw_conclusion":{"kind":"agreed_draw"}})));
    command(&mut agreement, c::Side::Gote, c::GameCommand::AcceptDraw);
    assert_eq!(agreement.conclusion(), Some(c::Conclusion::AgreedDraw));
    assert_eq!(agreement.position().accepted_plies(), 0);
    assert_eq!(agreement.events().len(), 2);
    cases.push(case(
        "agreed-draw",
        "CHU-MATERIAL-AGREEMENT",
        &agreement,
        c::Side::Gote,
        json!({"conclusion":{"kind":"agreed_draw"},"board_plies":0,"native_events":2}),
    ));

    // These known final actions are independently expected fixture facts. Save
    // their actual complete native histories too, not duplicated claimed results.
    for (name, id, expected) in [
        (
            "royal-concluded",
            "CHU-ROYAL-BOTH-COMPOUND",
            c::Conclusion::RoyalCapture {
                winner: c::Side::Sente,
            },
        ),
        (
            "material-concluded",
            "CHU-MATERIAL-DECLINE-COUNTERCAPTURE",
            c::Conclusion::Material {
                winner: c::Side::Sente,
            },
        ),
    ] {
        let f = roots::fixture(id);
        let a = if f["given"]["action"].is_object() {
            &f["given"]["action"]
        } else {
            &f["given"]["commands"][0]
        };
        let mut g = from_fixture(id);
        assert!(g.conclusion().is_none());
        play(&mut g, roots::action(a));
        assert_eq!(g.conclusion(), Some(expected));
        cases.push(case(
            name,
            id,
            &g,
            c::Side::Sente,
            json!({"conclusion":expected,"known_board_actions":1}),
        ));
    }
    cases
}

fn actual_observation(d: &ValidatedChuDocument, id: u64) -> mshogi_engine::ExecutionObservation {
    actual_observation_at_limits(d, id, CHU_PLAY_LIMITS)
}
fn actual_observation_at_limits(
    d: &ValidatedChuDocument,
    id: u64,
    limits: mshogi_engine::WorkLimits,
) -> mshogi_engine::ExecutionObservation {
    let req = ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: id,
            session_version: id,
            process_generation: id,
            game: CanonicalRecord::chu(d.record().clone()),
            limits,
        },
    };
    let mut output = vec![];
    mshogi_engine::serve_once(
        Cursor::new(format!("{}\n", serde_json::to_string(&req).unwrap())),
        &mut output,
    )
    .unwrap();
    let response: ExecutionResponse = serde_json::from_slice(&output).unwrap();
    assert!(response.matches(&req));
    response.observation
}
fn capacity_document() -> (ValidatedChuDocument, Value) {
    let baseline = document(&c::Game::standard(), c::Side::Sente);
    // Assemble only our own trusted generated payload, not untrusted archive
    // input. One strict native decode admits the entire candidate. No expected
    // result, fake failure or copied telemetry replaces actual engine work.
    let mut wire: Value = serde_json::from_slice(baseline.bytes()).unwrap();
    let mut observations = vec![];
    let mut controls = vec![];
    for id in 1..=256 {
        let observation = actual_observation_at_limits(
            &baseline,
            id,
            mshogi_engine::WorkLimits {
                time_ms: None,
                depth: 1,
                nodes: 0,
            },
        );
        assert!(
            matches!(&observation.result,ExecutionResult::Chu {result:ChuResult::Incomplete {telemetry}} if telemetry.score.is_none() && telemetry.nodes==0 && telemetry.principal_variation.is_empty())
        );
        controls.push(
            serde_json::to_value(ChuControl::Observation {
                index: observations.len(),
            })
            .unwrap(),
        );
        observations.push(serde_json::to_value(observation).unwrap());
    }
    wire["payload"]["controls"] = controls.into();
    wire["payload"]["observations"] = observations.into();
    wire["payload"]["retired_execution_floor"] = 256.into();
    let d =
        ValidatedChuDocument::decode(&serde_json::to_vec(&wire).unwrap(), &mut || false).unwrap();
    assert_eq!(d.observations().len(), 256);
    assert!(d.record().to_game().unwrap().events().is_empty());
    replay(&d);
    let before = d.bytes().to_vec();
    let next = actual_observation_at_limits(
        &d,
        257,
        mshogi_engine::WorkLimits {
            time_ms: None,
            depth: 1,
            nodes: 0,
        },
    );
    let error = d.with_observation(next, &mut || false).unwrap_err();
    let fact = error.resource.expect("typed observation refusal");
    assert_eq!(
        fact.facts()[0].category,
        mshogi_desktop_session::ResourceCategory::OrdinaryObservations
    );
    assert_eq!(fact.facts()[0].count, Some(257));
    assert_eq!(fact.facts()[0].limit, Some(256));
    assert_eq!(d.bytes(), before);
    (
        d,
        json!({"root":"standard_start","observations":256,"native_events":0,
        "engine":"mshogi.chu.full-width.v1","work_limits":{"depth":1,"nodes":0},
        "expected":"A positive-budget normal analysis request refuses before committing a 257th observation, with typed observation capacity; Save remains available and Load does not reset the structural count.",
        "limitation":"Historical actual zero-work observations only; neither pending execution nor completed scores/actions."}),
    )
}
fn complete_standard_game() -> (ValidatedChuDocument, Value) {
    // Actual current depth1/512 deterministic engine on both sides, followed by
    // an explicit scripted participant resignation. This is a fully concluded
    // standard-root record, NOT natural self-play completion or strength proof.
    let mut d = document(&c::Game::standard(), c::Side::Sente);
    let mut turns = vec![];
    for id in 1..=12 {
        let observation = actual_observation(&d, id);
        let ExecutionResult::Chu {
            result: ChuResult::Choice { action, telemetry },
        } = &observation.result
        else {
            panic!("representative completed search required: {observation:?}")
        };
        assert_eq!(telemetry.completed_depth, 1);
        assert!(telemetry.nodes <= 512);
        let action = action.clone();
        let actor = d.record().to_game().unwrap().position().side_to_move();
        turns.push(json!({"actor":actor,"action":action,"nodes":telemetry.nodes}));
        d = d.with_observation(observation, &mut || false).unwrap();
        assert!(d.record().to_game().unwrap().conclusion().is_none());
    }
    d = d
        .with_control(
            ChuControl::Native {
                actor: c::Side::Sente,
                command: c::GameCommand::Resign,
            },
            &mut || false,
        )
        .unwrap();
    let g = d.record().to_game().unwrap();
    assert_eq!(
        g.conclusion(),
        Some(c::Conclusion::Resignation {
            winner: c::Side::Gote
        })
    );
    assert_eq!(g.position().accepted_plies(), 12);
    assert_eq!(g.events().len(), 13);
    assert_eq!(d.observations().len(), 12);
    replay(&d);
    let metadata = json!({"root":"standard_start","engine":"mshogi.chu.full-width.v1","work_limits":{"depth":1,"nodes":512},
        "observed_turns":turns,"ending":"explicit_scripted_sente_resignation","conclusion":{"kind":"resignation","winner":"gote"},
        "known_board_actions":12,"native_events":13,"observations":12,
        "limitation":"Complete standard-root recorded game with explicit resignation; not natural self-play completion or competitive-strength evidence."});
    (d, metadata)
}
fn complete_natural_engine_game() -> (ValidatedChuDocument, Value) {
    // This independently reviewed diagram has a complete two-leg capture of
    // both enemy royals. The actual engine must discover a terminal action;
    // neither a heuristic score nor the fixture's expected result is applied.
    let original = from_fixture("CHU-ROYAL-BOTH-COMPOUND");
    let mut board = original.position().pieces().collect::<Vec<_>>();
    // Retain a remote real nonroyal so a single royal removal does not also
    // satisfy the narrow material award. This is an explicitly derived diagram,
    // not a change to the immutable source fixture or an engine tie-break rule.
    board.push((
        "11a".parse().unwrap(),
        c::Piece::from_code(c::Side::Gote, "G").unwrap(),
    ));
    let g = c::Game::from_root(
        c::Position::from_diagram(&board, c::Side::Sente, c::RootMetadata::diagram(0)).unwrap(),
    )
    .unwrap();
    let mut d = document(&g, c::Side::Gote);
    assert!(d.record().to_game().unwrap().conclusion().is_none());
    let choice = actual_observation(&d, 1);
    let ExecutionResult::Chu {
        result: ChuResult::Choice { action, telemetry },
    } = &choice.result
    else {
        panic!("actual naturally finishing choice required: {choice:?}")
    };
    assert_eq!(telemetry.completed_depth, 1);
    assert_eq!(action.route.len(), 2);
    let actual_action = action.clone();
    d = d.with_observation(choice, &mut || false).unwrap();
    let g = d.record().to_game().unwrap();
    assert_eq!(
        g.conclusion(),
        Some(c::Conclusion::RoyalCapture {
            winner: c::Side::Sente
        })
    );
    assert_eq!(g.events().len(), 1);
    assert_eq!(g.royal_squares(c::Side::Gote).len(), 0);
    let effects = g.events()[0].effects.as_ref().unwrap();
    let mut captures = effects
        .captures
        .iter()
        .map(|v| v.square.to_string())
        .collect::<Vec<_>>();
    captures.sort();
    assert_eq!(captures, ["5e", "6e"]);
    let finished = actual_observation(&d, 2);
    let ExecutionResult::Chu {
        result: ChuResult::Finished {
            conclusion,
            telemetry,
        },
    } = &finished.result
    else {
        panic!("actual terminal-root response required: {finished:?}")
    };
    assert_eq!(
        *conclusion,
        c::Conclusion::RoyalCapture {
            winner: c::Side::Sente
        }
    );
    assert_eq!(telemetry.nodes, 1);
    assert!(telemetry.principal_variation.is_empty());
    d = d.with_observation(finished, &mut || false).unwrap();
    assert_eq!(d.record().to_game().unwrap().events().len(), 1);
    assert_eq!(d.observations().len(), 2);
    replay(&d);
    (
        d,
        json!({"root":"independently_authored_diagram","source_fixture":"CHU-ROYAL-BOTH-COMPOUND",
        "diagram_extension":"Remote Gote Gold11a prevents a single royal capture from also receiving a material award; original fixture remains unchanged.",
        "engine":"mshogi.chu.full-width.v1","work_limits":{"depth":1,"nodes":512},"actual_action":actual_action,
        "ending":"native_royal_capture_after_actual_engine_choice","conclusion":{"kind":"royal_capture","winner":"sente"},
        "known_board_actions":1,"native_events":1,"observations":2,"response_sequence":["choice","finished"],
        "limitation":"Naturally completed game from explicitly limited independently authored diagram, not a standard-root self-play completion or strength measurement."}),
    )
}
fn main() {
    let output = PathBuf::from(std::env::args_os().nth(1).expect("output directory"));
    std::fs::create_dir_all(&output).unwrap();
    let mut cases = scenarios();
    let (complete, metadata) = complete_standard_game();
    cases.push(("complete-resignation".into(), complete, metadata));
    let (complete, metadata) = complete_natural_engine_game();
    cases.push(("complete-royal-engine".into(), complete, metadata));
    let (capacity, metadata) = capacity_document();
    cases.push(("capacity256".into(), capacity, metadata));
    let mut manifest = vec![];
    for (name, d, metadata) in cases {
        let path = output.join(format!("chu-{name}.mshogi"));
        std::fs::write(&path, d.bytes()).unwrap();
        assert_eq!(std::fs::read(&path).unwrap(), d.bytes());
        manifest.push(json!({"file":path.file_name().unwrap().to_str().unwrap(),"provenance":metadata,"usage":d.usage()}));
        println!("{}", path.display());
    }
    std::fs::write(output.join("chu-native-fixtures.json"), serde_json::to_string_pretty(&json!({
        "profile":"mshogi.chu.jcsa2019.v1","generator":"write_chu_gui_documents",
        "expectation_source":"ratified MSH82/86 docs/m4/chu/fixtures.json; independent source transcription and hand-worked expectations, no engine oracle",
        "fixture_diagrams":"Not claimed historically reachable from standard start; known history begins at each diagram root.",
        "cases":manifest,
    })).unwrap()+"\n").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ratified_lion_routes_keep_effects_and_partial_rejection_atomic() {
        let f = roots::fixture("CHU-RETURN-Ln-enemy");
        let mut capture = from_fixture("CHU-RETURN-Ln-enemy");
        let capture_action = roots::action(&f["given"]["commands"][0]);
        let quiet_action = c::Action {
            from: "6f".parse().unwrap(),
            route: vec!["6e".parse().unwrap(), "6f".parse().unwrap()],
            promote: false,
        };
        assert_ne!(capture_action, quiet_action);
        assert!(capture.legal_actions().contains(&capture_action));
        assert!(capture.legal_actions().contains(&quiet_action));
        let record = c::GameRecord::from_game(&capture).unwrap();
        let partial = c::Action {
            from: capture_action.from,
            route: vec![],
            promote: false,
        };
        assert!(
            capture
                .submit(c::GameRequest {
                    expected: capture.context(),
                    actor: c::Side::Sente,
                    command: c::GameCommand::Play { action: partial }
                })
                .is_err()
        );
        assert_eq!(c::GameRecord::from_game(&capture).unwrap(), record);
        play(&mut capture, capture_action);
        let effects = capture.events()[0].effects.as_ref().unwrap();
        assert_eq!(
            effects
                .captures
                .iter()
                .map(|v| v.square.to_string())
                .collect::<Vec<_>>(),
            ["7e"]
        );
        assert!(!effects.quiet_return);
        assert_eq!(
            capture
                .position()
                .piece("6f".parse().unwrap())
                .unwrap()
                .face(),
            c::Face::Lion
        );
        let mut quiet = from_fixture("CHU-RETURN-Ln-enemy");
        play(&mut quiet, quiet_action);
        let effects = quiet.events()[0].effects.as_ref().unwrap();
        assert!(effects.captures.is_empty() && effects.quiet_return);
        assert!(quiet.position().piece("7e".parse().unwrap()).is_some());
    }
    #[test]
    fn independently_expected_capture_promotion_and_royal_facts_survive_native_replay() {
        for (id, captures, conclusion) in [
            ("CHU-LION-C2-1", vec!["2b", "1b"], None),
            (
                "CHU-ROYAL-BOTH-COMPOUND",
                vec!["6e", "5e"],
                Some(c::Conclusion::RoyalCapture {
                    winner: c::Side::Sente,
                }),
            ),
            ("CHU-ROYAL-SURVIVOR", vec!["12l"], None),
        ] {
            let f = roots::fixture(id);
            let mut g = from_fixture(id);
            play(&mut g, roots::action(&f["given"]["commands"][0]));
            assert_eq!(
                g.events()[0]
                    .effects
                    .as_ref()
                    .unwrap()
                    .captures
                    .iter()
                    .map(|v| v.square.to_string())
                    .collect::<Vec<_>>(),
                captures
            );
            assert_eq!(g.conclusion(), conclusion);
            if id == "CHU-ROYAL-SURVIVOR" {
                assert_eq!(g.royal_squares(c::Side::Sente), ["8j".parse().unwrap()]);
            }
            replay(&document(&g, c::Side::Sente));
        }
        let f = roots::fixture("CHU-PROMOTE-DE-YES");
        let mut g = from_fixture("CHU-PROMOTE-DE-YES");
        let promoted = roots::action(&f["given"]["action"]);
        let nonpromoted = c::Action {
            promote: false,
            ..promoted.clone()
        };
        assert!(g.legal_actions().contains(&promoted) && g.legal_actions().contains(&nonpromoted));
        play(&mut g, promoted);
        let prince = g.position().piece("6d".parse().unwrap()).unwrap();
        assert_eq!(
            (prince.base, prince.promoted, prince.face()),
            (c::Base::Elephant, true, c::Face::Prince)
        );
        assert_eq!(g.royal_squares(c::Side::Sente).len(), 2);
        assert!(g.conclusion().is_none());
        replay(&document(&g, c::Side::Sente));
    }
    #[test]
    fn narrow_material_and_finite_mate_follow_independent_fixtures_not_bare_royal_inference() {
        let f = roots::fixture("CHU-MATERIAL-DECLINE-COUNTERCAPTURE");
        let mut g = from_fixture("CHU-MATERIAL-DECLINE-COUNTERCAPTURE");
        assert!(g.conclusion().is_none());
        assert!(g.legal_actions().contains(&c::Action {
            from: "6d".parse().unwrap(),
            route: vec!["6e".parse().unwrap()],
            promote: false
        }));
        play(&mut g, roots::action(&f["given"]["action"]));
        assert_eq!(
            g.conclusion(),
            Some(c::Conclusion::Material {
                winner: c::Side::Sente
            })
        );
        replay(&document(&g, c::Side::Gote));
        assert_eq!(
            from_fixture("CHU-MATE").conclusion(),
            Some(c::Conclusion::Mate {
                winner: c::Side::Gote
            })
        );
        assert!(from_fixture("CHU-MATERIAL-NARROW").conclusion().is_none());
        assert!(
            from_fixture("CHU-MATERIAL-AGREEMENT")
                .conclusion()
                .is_none()
        );
    }
    #[test]
    fn gui_scenarios_have_authoritative_replayable_history_and_exact_archive_bytes() {
        let cases = scenarios();
        assert_eq!(cases.len(), 14);
        for (_, d, _) in cases {
            replay(&d);
        }
    }
    #[test]
    fn complete_standard_game_has_actual_observations_and_explicit_terminal_command() {
        let (d, m) = complete_standard_game();
        assert_eq!(m["ending"], "explicit_scripted_sente_resignation");
        assert_eq!(
            d.record().to_game().unwrap().conclusion(),
            Some(c::Conclusion::Resignation {
                winner: c::Side::Gote
            })
        );
    }
    #[test]
    fn actual_engine_natural_complete_action_and_finished_response_preserve_native_outcome() {
        let (d, m) = complete_natural_engine_game();
        assert_eq!(m["response_sequence"], json!(["choice", "finished"]));
        assert_eq!(
            d.record().to_game().unwrap().conclusion(),
            Some(c::Conclusion::RoyalCapture {
                winner: c::Side::Sente
            })
        );
    }
    #[test]
    fn capacity_fixture_retains256_actual_unscored_observations_and_atomic257_refusal() {
        let (d, _) = capacity_document();
        assert_eq!(d.observations().len(), 256);
        replay(&d);
    }
}
