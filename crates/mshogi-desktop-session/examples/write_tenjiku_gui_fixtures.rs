//! Independent native diagrams for ordinary packaged Tenjiku Load journeys.
//! Development-only writer; archives and writer are never application assets.
use mshogi_core::tenjiku as t;
use mshogi_desktop_session::tenjiku_document::ValidatedTenjikuDocument;
use serde_json::{Value, json};
use std::path::PathBuf;

fn corpus() -> Value {
    serde_json::from_str(include_str!("../../../docs/m4/tenjiku/fixtures.json")).unwrap()
}
fn given(id: &str) -> Value {
    corpus()["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == id)
        .unwrap()["given"]
        .clone()
}
fn fixture(profile: t::Profile, id: &str, royal_engine_extension: bool) -> t::Game {
    let input = given(id);
    let root = &input["root"];
    let mut pieces = root["board"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            let piece = t::Piece::new(
                p["id"].as_str().unwrap().parse().unwrap(),
                serde_json::from_value(p["side"].clone()).unwrap(),
                serde_json::from_value(p["original"].clone()).unwrap(),
                p["promoted"].as_bool().unwrap(),
            )
            .unwrap();
            (serde_json::from_value(p["at"].clone()).unwrap(), piece)
        })
        .collect::<Vec<_>>();
    if royal_engine_extension {
        // The frozen source demonstrates a specified royal-burning action. As
        // a free engine-choice diagram it also permits a tied bare-royal win:
        // FiD 8j-1c burns the only Gote Pawn at 2c, leaving King 8g alive.
        // This remote second Pawn cannot be removed together with 2c in one
        // Fire Demon action, so a one-ply win must remove the enemy King.
        assert!(
            pieces
                .iter()
                .all(|(at, piece)| *at != t::Square::new(1, 1).unwrap() && piece.id != 5)
        );
        pieces.push((
            t::Square::new(1, 1).unwrap(),
            t::Piece::new(5, t::Side::Gote, t::Base::P, false).unwrap(),
        ));
    }
    // Selected fixtures carry no non-default refusal rights. Do not silently
    // discard a future fixture's required history or promotion-right context.
    assert!(root["promotion_rights"].as_array().unwrap().is_empty());
    t::Game::from_root(
        t::Position::from_diagram(
            profile,
            &pieces,
            serde_json::from_value(root["turn"].clone()).unwrap(),
            t::RootMetadata::imported(
                format!("MSH44 packaged native source fixture {id}{}; earlier history unproved",if royal_engine_extension { "; explicit diagram extension: remote Gote Pawn id5 at 1a excludes the alternative one-ply bare-royal win" } else { "" }),
                0,
            )
            .unwrap(),
        )
        .unwrap(),
    )
    .unwrap()
}
fn play(game: &mut t::Game, actor: t::Side, command: t::GameCommand) {
    game.submit(t::GameRequest {
        expected: game.context(),
        actor,
        command,
    })
    .unwrap();
}
fn main() {
    let first = std::env::args_os()
        .nth(1)
        .expect("output directory or --verify-package-report");
    if first == "--verify-package-report" {
        verify_package_report(PathBuf::from(
            std::env::args_os().nth(2).expect("package report"),
        ));
        return;
    }
    let directory = PathBuf::from(first);
    std::fs::create_dir_all(&directory).unwrap();
    let mut cases = Vec::new();
    for profile in t::Profile::ALL {
        let key = if profile == t::Profile::AdamsModern {
            "adams"
        } else {
            "muller"
        };
        for (name, id, human, expected) in [
            (
                "compound",
                "TEN-LIONHAWK-DOUBLE-CAPTURE",
                t::Side::Sente,
                json!({"origin":"8h","steps":["9h","10h"],"capture_squares":["9h","10h"],"conclusion":null}),
            ),
            (
                "igui",
                "TEN-TETRARCH-IGUI",
                t::Side::Sente,
                json!({"origin":"8h","target":"9h","stationary":true,"conclusion":null}),
            ),
            (
                "promotion",
                "TEN-PROMOTION-CONCRETE-P-YES",
                t::Side::Sente,
                json!({"origin":"8f","steps":["8e"],"promote":true,"face":"G","conclusion":null}),
            ),
            (
                "burn",
                "TEN-BURN-DYING",
                t::Side::Sente,
                json!({"origin":"8j","steps":["7i","7h","8h"],"burned":if profile==t::Profile::AdamsModern {vec!["8g"]}else{vec![]},"mover_dies":"8h","conclusion":null}),
            ),
            (
                "royal-engine",
                "TEN-BURN-ENEMY-ROYAL",
                t::Side::Gote,
                json!({"engine_to_move":"sente","conclusion":{"kind":"royal_capture","winner":"sente"},"qualification":"Actual UI Resume must obtain a real engine choice and commit a native terminal game, not a fabricated observation."}),
            ),
            (
                "simultaneous",
                "TEN-BURN-SIMULTANEOUS-ROYALS",
                t::Side::Sente,
                json!({"origin":"8i","steps":["8h"],"conclusion":{"kind":"simultaneous_royal_loss"}}),
            ),
            (
                "bare-loss",
                "TEN-BARE-CONCRETE-LOSS",
                t::Side::Gote,
                json!({"origin":"5a","steps":["5j"],"conclusion":{"kind":"bare_royal_loss","winner":"gote"}}),
            ),
            (
                "no-actions",
                "TEN-NO-ACTIONS-CONCRETE",
                t::Side::Sente,
                json!({"legal_actions":[],"conclusion":null,"qualification":"No actions is unscored, never an invented draw/loss."}),
            ),
            (
                "agreement",
                "TEN-REPETITION-CONCRETE",
                t::Side::Gote,
                json!({"command_source":"TEN-COMMAND-OFFER_DRAW/TEN-COMMAND-ACCEPT_OPPOSING_OFFER/TEN-COMMAND-DECLINE_DRAW","known_events":1,"board_plies":0,"offered_by":"sente","accepting_actor":"gote","conclusion_after_accept":{"kind":"agreement"}}),
            ),
            (
                "repetition3",
                "TEN-REPETITION-CONCRETE",
                t::Side::Gote,
                json!({"known_events":3,"forbidden_origin":"2a","forbidden_destination":"1a","conclusion":null}),
            ),
        ] {
            let mut game = fixture(profile, id, name == "royal-engine");
            if name == "agreement" {
                play(&mut game, t::Side::Sente, t::GameCommand::OfferDraw);
                assert_eq!(game.snapshot().draw_offer, Some(t::Side::Sente));
                assert_eq!(game.position().side_to_move(), t::Side::Sente);
                assert_eq!(game.snapshot().board_ply, 0);
            }
            if name == "repetition3" {
                for entry in given(id)["commands"].as_array().unwrap().iter().take(3) {
                    play(
                        &mut game,
                        serde_json::from_value(entry["actor"].clone()).unwrap(),
                        serde_json::from_value(entry["command"].clone()).unwrap(),
                    );
                }
                assert_eq!(game.events().len(), 3);
                let forbidden: t::GameCommand =
                    serde_json::from_value(given(id)["commands"][3]["command"].clone()).unwrap();
                let before = t::GameRecord::from_game(&game).unwrap();
                assert!(matches!(
                    game.submit(t::GameRequest {
                        expected: game.context(),
                        actor: t::Side::Gote,
                        command: forbidden
                    }),
                    Err(t::GameError::Repetition)
                ));
                assert_eq!(t::GameRecord::from_game(&game).unwrap(), before);
            }
            assert!(
                game.conclusion().is_none(),
                "{name} root must be playable/unscored"
            );
            let doc =
                ValidatedTenjikuDocument::new(t::GameRecord::from_game(&game).unwrap(), human)
                    .unwrap();
            let replay = ValidatedTenjikuDocument::decode(doc.bytes(), &mut || false).unwrap();
            assert_eq!(replay.bytes(), doc.bytes());
            assert_eq!(replay.record(), doc.record());
            assert!(replay.observations().is_empty());
            let filename = format!("tenjiku-{key}-{name}.mshogi");
            let path = directory.join(&filename);
            std::fs::write(&path, doc.bytes()).unwrap();
            let mut case = json!({"file":filename,"profile":profile.id(),"source_fixture":id,"human":human,"expected":expected,"usage":doc.usage()});
            if name == "royal-engine" {
                case["diagram_extension"] = json!({"added":{"id":"5","side":"gote","original":"P","promoted":false,"at":[1,1]},"reason":"Exclude the independently identified alternative one-ply bare-royal win while retaining RoyalCapture expectation.","proof":"From 8j the Fire Demon's long diagonal can end at 1c or 2d and burn Pawn 2c, but neither endpoint burns remote Pawn 1a; its at-most-three-step area route cannot remove both Pawns. Any one-ply win must therefore remove King 8g.","development_observation":"Before this extension the actual normal macOS engine legally chose 8j-1c and won by bare_royal_loss; that was a fixture expectation gap, not a product defect.","qualification":"Explicit extension of the cited frozen root, not an exact corpus-root identity; the frozen corpus is unchanged."});
            }
            cases.push(case);
            println!("{}", path.display());
        }
    }
    std::fs::write(directory.join("tenjiku-native-fixtures.json"),serde_json::to_string_pretty(&json!({"schema":"mshogi.tenjiku.packaged-fixtures.v1","generator":"write_tenjiku_gui_fixtures","source_sha256":"2005e098b507f73248b8f9bd442f7ffe0c2c0c5ae205428d9a3d2371a7c08e82","source":"docs/m4/tenjiku/fixtures.json frozen MSH83/87 independently authored roots and expectations","qualification":"Current native desktop documents admitted/replayed by public Rust APIs. Unproved diagrams, no fabricated observations, no normal host fixture/IPC activation.","cases":cases})).unwrap()+"\n").unwrap();
}

fn verify_package_report(path: PathBuf) {
    let report: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let responses = report["tenjiku_responses"].as_array().unwrap();
    assert_eq!(responses.len(), 2);
    let mut replays = Vec::new();
    for (entry, profile) in responses.iter().zip(t::Profile::ALL) {
        assert_eq!(entry["profile"], profile.id());
        for purpose in ["engine_turn", "analysis"] {
            let observation = &entry[purpose]["observation"];
            let source = &observation["context"]["game"];
            assert_eq!(source["rules_profile"], profile.id());
            let record = t::GameRecord::from_json(&source["payload"].to_string()).unwrap();
            let result = &observation["result"]["result"];
            let variation = result["telemetry"]["principal_variation"]
                .as_array()
                .unwrap();
            assert_eq!(variation.first(), Some(&result["action"]));
            let mut game = record.to_game().unwrap();
            assert!(game.conclusion().is_none());
            for action in variation {
                let actor = game.position().side_to_move();
                play(
                    &mut game,
                    actor,
                    t::GameCommand::Play {
                        action: serde_json::from_value(action.clone()).unwrap(),
                    },
                );
            }
            assert_eq!(game.events().len(), record.event_count() + 1);
            assert_eq!(
                game.conclusion(),
                Some(t::Conclusion::RoyalCapture {
                    winner: t::Side::Sente
                })
            );
            replays.push(json!({"profile":profile.id(),"purpose":purpose,"actual_action":result["action"],"principal_variation":variation,"conclusion":game.conclusion(),"record":serde_json::from_str::<Value>(t::GameRecord::from_game(&game).unwrap().as_json()).unwrap()}));
        }
    }
    println!(
        "{}",
        json!({"verification":"Actual packaged engine action/PV admitted through public native Game APIs; independently expected RoyalCapture Sente", "replays":replays})
    );
}
