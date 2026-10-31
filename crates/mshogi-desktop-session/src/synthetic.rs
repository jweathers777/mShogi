//! Explicitly synthetic development infrastructure, compiled only for QA builds.
//! The invented strider fixture is neither chess nor any promised real game.
use crate::{BoardPiece, MAX_SAFE_ID};
use mshogi_core::chess::Color;
use serde::{Deserialize, Serialize};

// Keep the M1 boundary evidence and development fixture on one invented rules
// implementation. Undo and selection helpers remain exercised by the M1 tests.
#[allow(dead_code)]
#[path = "../../mshogi-core/tests/support/synthetic_strider.rs"]
mod fixture;
use fixture::{Board, CompoundAction, Kind, Piece, Sq};

/// Deliberately distinct from every real-game and desktop archive profile.
pub const SYNTHETIC_PROFILE: &str = "mshogi.synthetic.strider16.v1";
const INITIAL_FIXTURE: &str = "strider16-capture-and-return-v1";

/// Native geometry consumed by the shared board presentation.
#[derive(Clone, Debug, Serialize)]
pub struct SyntheticGeometry {
    /// Files in their native ascending order.
    pub files: Vec<String>,
    /// Ranks in their native ascending order.
    pub ranks: Vec<String>,
    /// Accessible board name, explicitly identifying invented development rules.
    pub label: &'static str,
    /// Coordinate initially focused by the shared board.
    pub initial_focus: &'static str,
}
/// One complete native player choice. The ordered route is part of its identity.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SyntheticChoice {
    /// Opaque canonical identity, carrying origin and complete route only.
    pub action: String,
    /// Coordinate of the moving strider.
    pub from: String,
    /// Ordered visited coordinates, including a possible return to the origin.
    pub steps: Vec<String>,
    /// No promotion or other extra completion choice exists in this fixture.
    pub completion: Option<String>,
}
/// Small distinct record; it cannot be restored as a chess desktop archive.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SyntheticRecord {
    /// Supported fixture record version.
    pub schema_version: u32,
    /// Exact invented rules profile.
    pub rules_profile: String,
    /// Canonical initial arrangement, never an externally supplied raw board.
    pub initial_fixture: String,
    /// Complete canonical action identities, with no caller-supplied effects.
    pub actions: Vec<String>,
}
/// Immutable native fixture view; one action completes the demonstration.
#[derive(Clone, Debug, Serialize)]
pub struct SyntheticSnapshot {
    /// Native completed route; partial selections never change it.
    pub last_action: Option<crate::CompletedActionGeometry>,
    /// Explicit invented profile, distinct from chess and actual variants.
    pub profile: &'static str,
    /// Monotonic session identity, preserved across Reset.
    pub version: u64,
    /// Native 16 by 16 geometry.
    pub geometry: SyntheticGeometry,
    /// Native pieces in the existing shared presentation shape.
    pub pieces: Vec<BoardPiece>,
    /// Complete native choices; prefixes alone are presentation state.
    pub choices: Vec<SyntheticChoice>,
    /// Exact distinct record of the demonstration.
    pub record: SyntheticRecord,
    /// True after one completed action; Reset begins another demonstration.
    pub complete: bool,
}
/// Only complete opaque choices and explicit Reset cross the native boundary.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SyntheticCommand {
    /// Submit one complete canonical native fixture choice.
    Play {
        /// Expected current native version.
        version: u64,
        /// Complete action identity supplied by the native snapshot.
        action: String,
    },
    /// Reset the invented arrangement without resetting version protection.
    Reset {
        /// Expected current native version.
        version: u64,
    },
}
/// Native development controller; no engine, chess outcome, or archive behavior.
pub struct SyntheticController {
    board: Board,
    version: u64,
    actions: Vec<String>,
}
impl Default for SyntheticController {
    fn default() -> Self {
        Self::new()
    }
}
impl SyntheticController {
    /// Construct the explicitly invented initial arrangement.
    pub fn new() -> Self {
        Self {
            board: initial_board(),
            version: 1,
            actions: Vec::new(),
        }
    }
    /// Return an immutable presentation of the authoritative fixture.
    pub fn snapshot(&self) -> SyntheticSnapshot {
        let pieces = (0..=u8::MAX)
            .filter_map(|index| {
                let square = Sq(index);
                let piece = self.board.at(square)?;
                let color = if piece.side == 0 {
                    Color::White
                } else {
                    Color::Black
                };
                let kind = match piece.kind {
                    Kind::Strider => "strider",
                    Kind::Stone => "stone",
                };
                let square = coordinate(square);
                Some(BoardPiece {
                    label: format!("{square}, synthetic {color:?} {kind}"),
                    square,
                    color,
                    kind: kind.into(),
                })
            })
            .collect();
        SyntheticSnapshot {
            last_action: self
                .actions
                .last()
                .and_then(|id| fixture::decode(id))
                .map(|action| crate::CompletedActionGeometry {
                    action: fixture::encode(&action),
                    event_index: self.actions.len(),
                    origin: crate::ChoiceOrigin::Board {
                        square: coordinate(action.from),
                    },
                    steps: action.route.into_iter().map(coordinate).collect(),
                    effects: vec![],
                }),
            profile: SYNTHETIC_PROFILE,
            version: self.version,
            geometry: SyntheticGeometry {
                files: (b'a'..=b'p')
                    .map(|file| char::from(file).to_string())
                    .collect(),
                ranks: (1..=16).map(|rank| rank.to_string()).collect(),
                label: "Synthetic development strider board, 16 by 16; invented rules",
                initial_focus: "i9",
            },
            pieces,
            choices: self
                .choices()
                .iter()
                .map(|action| SyntheticChoice {
                    action: fixture::encode(action),
                    from: coordinate(action.from),
                    steps: action.route.iter().copied().map(coordinate).collect(),
                    completion: None,
                })
                .collect(),
            record: self.record(),
            complete: !self.actions.is_empty(),
        }
    }
    /// Accept atomically only a current complete canonical choice or Reset.
    pub fn command(&mut self, command: SyntheticCommand) -> Result<SyntheticSnapshot, String> {
        let expected = match &command {
            SyntheticCommand::Play { version, .. } | SyntheticCommand::Reset { version } => {
                *version
            }
        };
        if expected != self.version {
            return Err("The synthetic fixture changed. Use its current native version.".into());
        }
        if self.version >= MAX_SAFE_ID {
            return Err(
                "Synthetic fixture identity space exhausted. Restart the verification application."
                    .into(),
            );
        }
        match command {
            SyntheticCommand::Play { action, .. } => {
                let choice = self.choices().into_iter().find(|choice| fixture::encode(choice) == action)
                    .ok_or("Choose one complete canonical native synthetic action. A route prefix or effect annotation is not a submitted action.")?;
                let mut candidate = self.board.clone();
                candidate.apply(&choice).map_err(|error| {
                    format!("Synthetic fixture rejected the complete choice: {error:?}")
                })?;
                self.board = candidate;
                self.actions.push(action);
            }
            SyntheticCommand::Reset { .. } => {
                self.board = initial_board();
                self.actions.clear();
            }
        }
        self.version += 1;
        Ok(self.snapshot())
    }
    /// Return the distinctly profiled record, without runtime/UI selection state.
    pub fn record(&self) -> SyntheticRecord {
        SyntheticRecord {
            schema_version: 1,
            rules_profile: SYNTHETIC_PROFILE.into(),
            initial_fixture: INITIAL_FIXTURE.into(),
            actions: self.actions.clone(),
        }
    }
    /// Replay only this exact invented fixture and its complete canonical choices.
    pub fn replay(record: &SyntheticRecord) -> Result<Self, String> {
        if record.schema_version != 1
            || record.rules_profile != SYNTHETIC_PROFILE
            || record.initial_fixture != INITIAL_FIXTURE
            || record.actions.len() > 1
        {
            return Err("Unsupported synthetic fixture record/profile/initial arrangement; no chess archive or real game was restored.".into());
        }
        let mut controller = Self::new();
        for action in &record.actions {
            controller.command(SyntheticCommand::Play {
                version: controller.version,
                action: action.clone(),
            })?;
        }
        Ok(controller)
    }
    fn choices(&self) -> Vec<CompoundAction> {
        if !self.actions.is_empty() {
            return Vec::new();
        }
        let mut choices = Vec::new();
        let from = Sq(136);
        for index in 0..=u8::MAX {
            let first = CompoundAction {
                from,
                route: vec![Sq(index)],
            };
            if self.board.clone().apply(&first).is_err() {
                continue;
            }
            choices.push(first.clone());
            for second in 0..=u8::MAX {
                let action = CompoundAction {
                    from,
                    route: vec![Sq(index), Sq(second)],
                };
                if self.board.clone().apply(&action).is_ok() {
                    choices.push(action);
                }
            }
        }
        choices
    }
}
fn coordinate(square: Sq) -> String {
    format!("{}{}", char::from(b'a' + square.0 % 16), square.0 / 16 + 1)
}
fn initial_board() -> Board {
    let mut board = Board::empty();
    for (index, side, kind) in [
        (136, 0, Kind::Strider),
        (153, 1, Kind::Stone),
        (154, 1, Kind::Stone),
        (119, 0, Kind::Stone),
        (0, 0, Kind::Stone),
        (255, 1, Kind::Stone),
    ] {
        board.put(Sq(index), Piece { side, kind });
    }
    board
}

#[cfg(test)]
mod tests {
    use super::*;
    fn view(controller: &SyntheticController) -> serde_json::Value {
        serde_json::to_value(controller.snapshot()).unwrap()
    }
    #[test]
    fn native_geometry_and_complete_choices_include_distinct_return_routes() {
        let controller = SyntheticController::new();
        let snapshot = controller.snapshot();
        assert_eq!(snapshot.profile, "mshogi.synthetic.strider16.v1");
        assert_eq!(
            snapshot.geometry.files,
            (b'a'..=b'p')
                .map(|file| char::from(file).to_string())
                .collect::<Vec<_>>()
        );
        assert_eq!(
            snapshot.geometry.ranks,
            (1..=16).map(|rank| rank.to_string()).collect::<Vec<_>>()
        );
        assert!(snapshot.pieces.iter().any(|piece| piece.square == "p16"));
        for (action, expected_steps) in [
            ("136>153", vec!["j10"]),
            ("136>153>136", vec!["j10", "i9"]),
            ("136>135>136", vec!["h9", "i9"]),
        ] {
            let choice = snapshot
                .choices
                .iter()
                .find(|choice| choice.action == action)
                .unwrap();
            assert_eq!(choice.from, "i9");
            assert_eq!(choice.steps, expected_steps);
            assert_eq!(choice.completion, None);
        }
        assert!(
            snapshot
                .choices
                .iter()
                .all(|choice| (1..=2).contains(&choice.steps.len()))
        );
    }
    #[test]
    fn exact_complete_routes_derive_different_effects_and_replay_without_chess() {
        let mut capture = SyntheticController::new();
        let mut quiet = SyntheticController::new();
        let captured = capture
            .command(SyntheticCommand::Play {
                version: 1,
                action: "136>153>136".into(),
            })
            .unwrap();
        let uncaptured = quiet
            .command(SyntheticCommand::Play {
                version: 1,
                action: "136>135>136".into(),
            })
            .unwrap();
        assert!(captured.complete);
        assert!(captured.choices.is_empty());
        assert!(!captured.pieces.iter().any(|piece| piece.square == "j10"));
        assert!(uncaptured.pieces.iter().any(|piece| piece.square == "j10"));
        assert_eq!(captured.record.actions, ["136>153>136"]);
        assert_ne!(captured.record, uncaptured.record);
        for controller in [&capture, &quiet] {
            let serialized = serde_json::to_string(&controller.record()).unwrap();
            let record = serde_json::from_str(&serialized).unwrap();
            let restored = SyntheticController::replay(&record).unwrap();
            assert_eq!(view(&restored), view(controller));
            assert!(mshogi_core::chess::GameRecord::from_json(&serialized).is_err());
            assert!(mshogi_core::chess::MatchRecord::from_json(&serialized).is_err());
        }
    }
    #[test]
    fn prefix_stale_invalid_and_effect_inputs_are_atomic_and_reset_is_monotonic() {
        let mut controller = SyntheticController::new();
        let before = view(&controller);
        for action in [
            "136",
            "136>",
            "0136>153>136",
            "136>136",
            "136>119",
            "136>153>136>153",
            "136>153>136 capture=153",
        ] {
            assert!(
                controller
                    .command(SyntheticCommand::Play {
                        version: 1,
                        action: action.into()
                    })
                    .is_err(),
                "{action}"
            );
            assert_eq!(view(&controller), before);
        }
        assert!(
            controller
                .command(SyntheticCommand::Play {
                    version: 0,
                    action: "136>153>136".into()
                })
                .is_err()
        );
        assert_eq!(view(&controller), before);
        controller
            .command(SyntheticCommand::Play {
                version: 1,
                action: "136>153>136".into(),
            })
            .unwrap();
        let complete = view(&controller);
        assert!(
            controller
                .command(SyntheticCommand::Play {
                    version: 2,
                    action: "136>135>136".into()
                })
                .is_err()
        );
        assert_eq!(view(&controller), complete);
        let reset = controller
            .command(SyntheticCommand::Reset { version: 2 })
            .unwrap();
        assert_eq!(reset.version, 3);
        assert!(reset.record.actions.is_empty());
        assert!(!reset.complete);
        assert!(
            controller
                .command(SyntheticCommand::Reset { version: 1 })
                .is_err()
        );
        assert!(
            controller
                .command(SyntheticCommand::Play {
                    version: 1,
                    action: "136>153>136".into()
                })
                .is_err()
        );
        assert_eq!(controller.snapshot().version, 3);
    }
    #[test]
    fn replay_and_command_decoders_reject_other_profiles_and_raw_board_effects() {
        let initial = SyntheticController::new().record();
        assert_eq!(
            SyntheticController::replay(&initial).unwrap().record(),
            initial
        );
        for field in [
            "schema_version",
            "rules_profile",
            "initial_fixture",
            "actions",
        ] {
            let mut invalid = serde_json::to_value(&initial).unwrap();
            invalid[field] = match field {
                "schema_version" => 2.into(),
                "actions" => serde_json::json!(["136>153>136", "136>135>136"]),
                _ => "chess".into(),
            };
            let record = serde_json::from_value(invalid).unwrap();
            assert!(SyntheticController::replay(&record).is_err());
        }
        assert!(
            serde_json::from_str::<SyntheticCommand>(
                r#"{"kind":"play","version":1,"action":"136>153>136","board":{"cells":[]}}"#
            )
            .is_err()
        );
        assert!(
            serde_json::from_str::<SyntheticCommand>(
                r#"{"kind":"play","version":1,"action":"136>153>136","effects":[]}"#
            )
            .is_err()
        );
        let mut raw = serde_json::to_value(initial).unwrap();
        raw["board"] = serde_json::json!({"cells": [], "side": 0});
        assert!(serde_json::from_value::<SyntheticRecord>(raw).is_err());
    }
}
