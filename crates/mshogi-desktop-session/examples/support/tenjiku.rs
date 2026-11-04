//! Source inputs for labeled native verification. No expected outputs are read.
use mshogi_core::tenjiku as c;
use serde_json::Value;

pub fn fixture_input(id: &str) -> Value {
    let corpus: Value =
        serde_json::from_str(include_str!("../../../../docs/m4/tenjiku/fixtures.json")).unwrap();
    corpus["fixtures"]
        .as_array()
        .unwrap()
        .iter()
        .find(|f| f["id"] == id)
        .unwrap()["given"]
        .clone()
}
pub fn selected(fixture: &str) -> Result<(c::Profile, &str), String> {
    let (profile, scenario) = fixture
        .strip_prefix("tenjiku-")
        .and_then(|name| name.split_once('-'))
        .ok_or("Tenjiku fixture requires an explicit adams or muller profile")?;
    let profile = match profile {
        "adams" => c::Profile::AdamsModern,
        "muller" => c::Profile::MullerCompleted,
        _ => return Err("Unknown Tenjiku verification profile".into()),
    };
    Ok((profile, scenario))
}
fn root(profile: c::Profile, input: &Value) -> Result<c::Game, String> {
    let entries = input["board"]
        .as_array()
        .ok_or("missing fixture board")?
        .iter()
        .map(|p| {
            let id = p["id"]
                .as_str()
                .ok_or("fixture piece identity")?
                .parse::<u16>()
                .map_err(|e| e.to_string())?;
            let mut piece = c::Piece::new(
                id,
                serde_json::from_value(p["side"].clone()).map_err(|e| e.to_string())?,
                serde_json::from_value(p["original"].clone()).map_err(|e| e.to_string())?,
                p["promoted"].as_bool().ok_or("fixture promotion bit")?,
            )
            .map_err(|e| e.to_string())?;
            if let Some(rights) = input["promotion_rights"].as_array() {
                for right in rights.iter().filter(|r| r["piece_id"] == p["id"]) {
                    piece.right = match right["state"].as_str() {
                        Some("unseen") => c::PromotionRight::Unseen,
                        Some("first_declined_next_move") => {
                            c::PromotionRight::FirstDeclinedNextMove
                        }
                        Some("restriction_consumed") => c::PromotionRight::Consumed,
                        _ => return Err("unknown fixture promotion right".into()),
                    };
                }
            }
            Ok((
                serde_json::from_value(p["at"].clone()).map_err(|e| e.to_string())?,
                piece,
            ))
        })
        .collect::<Result<Vec<_>, String>>()?;
    c::Game::from_root(
        c::Position::from_diagram(
            profile,
            &entries,
            serde_json::from_value(input["turn"].clone()).map_err(|e| e.to_string())?,
            c::RootMetadata::diagram(0),
        )
        .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}
pub fn game(profile: c::Profile, scenario: &str) -> Result<c::Game, String> {
    match scenario {
        "standard" | "gote" => Ok(c::Game::standard(profile)),
        "burn" | "burned" => root(profile, &fixture_input("TEN-BURN-DYING")["root"]),
        "igui" | "iguied" => root(profile, &fixture_input("TEN-TETRARCH-IGUI")["root"]),
        "promotion" | "promoted" | "declined" => root(
            profile,
            &fixture_input("TEN-PROMOTION-CONCRETE-P-YES")["root"],
        ),
        "hawk" | "hawk-played" => root(
            profile,
            &fixture_input("TEN-LIONHAWK-DOUBLE-CAPTURE")["root"],
        ),
        "promoted-hawk" => {
            let mut input = fixture_input("TEN-LIONHAWK-DOUBLE-CAPTURE")["root"].clone();
            input["board"][0]["original"] = serde_json::json!("Ln");
            input["board"][0]["promoted"] = serde_json::json!(true);
            root(profile, &input)
        }
        "area" | "area-played" => root(profile, &fixture_input("TEN-AREA-STOP-CAPTURE")["root"]),
        "ranked-jump" | "ranked-jump-played" => {
            root(profile, &fixture_input("TEN-GENERAL-VICTIM-P")["root"])
        }
        "ranked-blocked" => root(profile, &fixture_input("TEN-GENERAL-BLOCKER-BGn")["root"]),
        "dense-lion" | "dense-hawk" | "dense-fire" | "dense-area" => {
            let mut board = c::Game::standard(profile)
                .position()
                .pieces()
                .collect::<Vec<_>>();
            let (base, promoted) = match scenario {
                "dense-lion" => (c::Base::Ln, false),
                "dense-hawk" => (c::Base::LHk, false),
                "dense-fire" => (c::Base::FiD, false),
                _ => (c::Base::BGn, true),
            };
            // Relocate one actual standard physical identity. Adding pieces
            // beyond the native original inventory allotment is forbidden.
            let (at, piece) = board
                .iter_mut()
                .find(|(_, piece)| piece.side == c::Side::Sente && piece.base == base)
                .ok_or("dense source piece absent from standard inventory")?;
            *at = c::Square::new(8, 8).unwrap();
            *piece =
                c::Piece::new(piece.id, piece.side, base, promoted).map_err(|e| e.to_string())?;
            c::Game::from_root(
                c::Position::from_diagram(
                    profile,
                    &board,
                    c::Side::Sente,
                    c::RootMetadata::diagram(0),
                )
                .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())
        }
        "contract" | "compound" | "compound-played" | "unsupported" | "selected-root"
        | "ending-stopped" => {
            let board = [
                (16, 16, c::Side::Sente, c::Base::K),
                (15, 14, c::Side::Sente, c::Base::P),
                (1, 1, c::Side::Gote, c::Base::K),
                (2, 3, c::Side::Gote, c::Base::P),
                (8, 8, c::Side::Sente, c::Base::Ln),
                (8, 7, c::Side::Gote, c::Base::P),
                (9, 7, c::Side::Gote, c::Base::P),
            ]
            .into_iter()
            .enumerate()
            .map(|(id, (file, rank, side, base))| {
                (
                    c::Square::new(file, rank).unwrap(),
                    c::Piece::new(id as u16, side, base, false).unwrap(),
                )
            })
            .collect::<Vec<_>>();
            c::Game::from_root(
                c::Position::from_diagram(
                    profile,
                    &board,
                    c::Side::Sente,
                    c::RootMetadata::diagram(0),
                )
                .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())
        }
        _ => Err("Unknown Tenjiku verification scenario".into()),
    }
}
pub fn action(scenario: &str) -> Result<c::Action, String> {
    let sq = |f, r| c::Square::new(f, r).unwrap();
    match scenario {
        "burned" => Ok(c::Action::single(sq(8, 10), sq(8, 8))),
        "iguied" => Ok(c::Action::igui(sq(8, 8), sq(9, 8))),
        "promoted" | "declined" => {
            let input = fixture_input(if scenario == "promoted" {
                "TEN-PROMOTION-CONCRETE-P-YES"
            } else {
                "TEN-PROMOTION-CONCRETE-P-NO"
            });
            serde_json::from_value(input["commands"][0]["command"]["action"].clone())
                .map_err(|e| e.to_string())
        }
        "compound-played" | "selected-root" | "unsupported" => {
            let mut action = c::Action::single(sq(8, 8), sq(8, 7));
            action.route.push(sq(8, 8));
            Ok(action)
        }
        "hawk-played" => {
            let input = fixture_input("TEN-LIONHAWK-DOUBLE-CAPTURE");
            Ok(c::Action {
                kind: c::ActionKind::Travel,
                from: serde_json::from_value(input["actor"].clone()).map_err(|e| e.to_string())?,
                route: serde_json::from_value(input["route"].clone()).map_err(|e| e.to_string())?,
                promote: false,
                target: None,
            })
        }
        "area-played" => Ok(c::Action {
            kind: c::ActionKind::Travel,
            from: sq(8, 8),
            route: vec![sq(7, 8), sq(7, 7), sq(6, 7)],
            promote: false,
            target: None,
        }),
        "ranked-jump-played" => Ok(c::Action::single(sq(8, 8), sq(11, 5))),
        _ => Err("Fixture does not prescribe a played action".into()),
    }
}
