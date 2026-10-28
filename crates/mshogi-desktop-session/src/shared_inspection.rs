//! Read-only educational local movement, separate from executable session choices.
use super::*;

/// Strict, bounded read-only query bound to one exact native selection.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MovementInspectionQuery {
    /// Exact runtime and historical selection identity.
    pub expected: SessionToken,
    /// Opaque board or native hand origin from the selected snapshot.
    pub origin: String,
}
impl MovementInspectionQuery {
    /// Parse raw JSON without an intermediate map, preserving duplicate rejection.
    pub fn from_json(json: &str) -> Result<Self, String> {
        if json.len() > 1024 {
            return Err("Movement inspection query exceeds its encoded capacity.".into());
        }
        serde_json::from_str(json).map_err(|_| "Invalid movement inspection query.".into())
    }
}
/// Complete local choices for education only, never live command authorization.
#[derive(Clone, Debug, Serialize)]
pub struct MovementInspection {
    /// Exact selection for which these choices were computed.
    pub token: SessionToken,
    /// Native game family.
    pub game: GameKind,
    /// Queried opaque origin.
    pub origin: String,
    /// Native owner of the selected piece/resource.
    pub side: PresentedSide,
    /// Native semantic readable piece/resource description.
    pub label: String,
    /// Complete locally legal actions on the owner's hypothetical turn.
    pub choices: Vec<PresentedChoice>,
    /// Always true; these choices cannot authorize a session mutation.
    pub read_only: bool,
}
impl DesktopController {
    /// Inspect a selected native position without polling, accepting, retiring or
    /// starting execution. The immutable receiver protects all session state.
    pub fn shared_inspection(
        &self,
        query: MovementInspectionQuery,
    ) -> Result<MovementInspection, String> {
        if query.expected != self.shared_token() {
            return Err("The position changed. Select the piece again.".into());
        }
        let (game, owner, label, choices) = if let Some(state) = &self.shared.xiangqi {
            xiangqi::inspect(state.selected_position(), &query.origin)?
        } else if let Some(state) = &self.shared.shogi {
            inspect_shogi(state.selected().position(), &query.origin)?
        } else {
            inspect_chess(self.session.selected_game().position(), &query.origin)?
        };
        Ok(MovementInspection {
            token: query.expected,
            game,
            origin: query.origin,
            side: side(owner),
            label,
            choices,
            read_only: true,
        })
    }
}
pub(super) type Inspected = (GameKind, NativeSide, String, Vec<PresentedChoice>);
fn inspect_chess(position: &Position, origin: &str) -> Result<Inspected, String> {
    let bytes = origin.as_bytes();
    let square = if bytes.len() == 2
        && (b'a'..=b'h').contains(&bytes[0])
        && (b'1'..=b'8').contains(&bytes[1])
    {
        Square::new(bytes[0] - b'a', bytes[1] - b'1').unwrap()
    } else {
        return Err("Unknown inspection origin.".into());
    };
    let piece = position
        .piece_at(square)
        .ok_or("No piece at this origin.")?;
    let owner = NativeSide::Chess(piece.color);
    let label = format!(
        "{} {}",
        side(owner).label,
        format!("{:?}", piece.kind).to_lowercase()
    );
    let choices = position
        .inspection_actions(piece.color)
        .into_iter()
        .filter(|a| a.from == square)
        .map(|a| PresentedChoice {
            origin: ChoiceOrigin::Board {
                square: origin.into(),
            },
            action: a.to_string(),
            from: origin.into(),
            steps: vec![a.to.to_string()],
            completion: a.promotion.map(|p| Completion {
                label: format!("{p:?}").to_lowercase(),
                glyph: None,
            }),
        })
        .collect();
    Ok((GameKind::Chess, owner, label, choices))
}
fn inspect_shogi(position: &s::Position, origin: &str) -> Result<Inspected, String> {
    let (actor, kind, promoted, square) = if let Ok(square) = origin.parse::<s::Square>() {
        let piece = position.piece(square).ok_or("No piece at this origin.")?;
        (piece.side, piece.kind, piece.promoted, Some(square))
    } else {
        let resource = [s::Side::Sente, s::Side::Gote]
            .into_iter()
            .find_map(|actor| {
                s::PieceKind::HAND_KINDS.into_iter().find_map(|kind| {
                    (origin
                        == format!(
                            "hand:{}:{}",
                            side(NativeSide::Shogi(actor)).key,
                            piece_text(kind)
                        ))
                    .then_some((actor, kind))
                })
            })
            .ok_or("Unknown inspection origin.")?;
        if position.hand(resource.0).count(resource.1) == 0 {
            return Err("No piece in this hand resource.".into());
        }
        (resource.0, resource.1, false, None)
    };
    let owner = NativeSide::Shogi(actor);
    let label = format!(
        "{} {}{}{}",
        side(owner).label,
        if promoted { "promoted " } else { "" },
        format!("{kind:?}").to_lowercase(),
        if square.is_none() { " in hand" } else { "" }
    );
    let legal = position.inspection_actions(actor);
    let choices = legal
        .iter()
        .copied()
        .filter(|a| match (*a, square) {
            (s::Action::Move { from, .. }, Some(square)) => from == square,
            (s::Action::Drop { kind: k, .. }, None) => k == kind,
            _ => false,
        })
        .map(|a| {
            let (to, completion) = match a {
                s::Action::Move { from, to, promote } => (
                    to,
                    if promote
                        || legal.contains(&s::Action::Move {
                            from,
                            to,
                            promote: !promote,
                        })
                    {
                        Some(Completion {
                            label: if promote { "Promote" } else { "Do not promote" }.into(),
                            glyph: None,
                        })
                    } else {
                        None
                    },
                ),
                s::Action::Drop { to, .. } => (to, None),
            };
            PresentedChoice {
                origin: if let Some(square) = square {
                    ChoiceOrigin::Board {
                        square: square.to_string(),
                    }
                } else {
                    ChoiceOrigin::Hand {
                        owner: side(owner).key,
                        piece_kind: format!("{kind:?}").to_lowercase(),
                    }
                },
                action: a.to_string(),
                from: origin.into(),
                steps: vec![to.to_string()],
                completion,
            }
        })
        .collect();
    Ok((GameKind::Shogi, owner, label, choices))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn controller() -> DesktopController {
        let mut c = DesktopController::new(PathBuf::from("/unused/inspection-engine")).unwrap();
        c.shared.manual_engine = true;
        c
    }
    fn query(c: &DesktopController, origin: &str) -> Result<MovementInspection, String> {
        c.shared_inspection(MovementInspectionQuery {
            expected: c.shared_token(),
            origin: origin.into(),
        })
    }
    fn native_shogi(c: &mut DesktopController, sfen: &str) {
        let game = if sfen == s::INITIAL_SFEN {
            s::Game::new()
        } else {
            s::Game::from_root(
                s::GameRoot::import(
                    s::Position::from_sfen(sfen).unwrap(),
                    s::RootCategory::ReducedMaterialDiagram,
                    s::CounterSource::Supplied,
                )
                .unwrap(),
            )
        };
        c.shared.shogi =
            Some(ShogiSession::restored(&s::GameRecord::from_game(&game), s::Side::Sente).unwrap());
    }
    #[test]
    fn exact_native_chess_query_is_not_a_snapshot_poll_or_mutation() {
        let c = controller();
        let before = serde_json::to_string(&c.session.snapshot()).unwrap();
        let token = c.shared_token();
        let inspected = query(&c, "b8").unwrap();
        assert_eq!(inspected.label, "Black knight");
        let mut actions = inspected
            .choices
            .iter()
            .map(|a| a.action.as_str())
            .collect::<Vec<_>>();
        actions.sort();
        assert_eq!(actions, ["b8a6", "b8c6"]);
        assert!(inspected.read_only);
        assert_eq!(inspected.token, token);
        assert_eq!(c.shared_token(), token);
        assert_eq!(
            serde_json::to_string(&c.session.snapshot()).unwrap(),
            before
        );
        assert!(query(&c, "e4").is_err());
        assert!(query(&c, "e9").is_err());
    }
    #[test]
    fn chess_pending_engine_work_and_historical_prefix_are_not_touched() {
        let mut c = controller();
        c.session
            .command_unchecked(
                DesktopCommand::Play {
                    version: c.session.version,
                    action: "e2e4".parse().unwrap(),
                },
                0,
            )
            .unwrap();
        c.session.engine_request().unwrap();
        let pending = c.session.pending.clone();
        assert!(pending.is_some());
        let before = serde_json::to_string(&c.session.snapshot()).unwrap();
        query(&c, "g8").unwrap();
        query(&c, "g1").unwrap();
        assert_eq!(c.session.pending, pending);
        assert_eq!(
            serde_json::to_string(&c.session.snapshot()).unwrap(),
            before
        );
        let previous = c.shared_token();
        c.command(DesktopCommand::Inspect {
            version: c.session.version,
            event_index: 0,
        })
        .unwrap();
        let deadline = Instant::now() + std::time::Duration::from_secs(10);
        while !c.snapshot().view.historical {
            assert!(Instant::now() < deadline);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        assert!(query(&c, "e2").is_ok());
        assert!(query(&c, "e4").is_err());
        assert!(
            c.shared_inspection(MovementInspectionQuery {
                expected: previous,
                origin: "g8".into()
            })
            .is_err()
        );
    }
    #[test]
    fn selected_shogi_owner_promotions_hands_and_pending_work_stay_native() {
        let mut c = controller();
        native_shogi(&mut c, "4k4/9/9/4P4/9/9/9/9/4K4 b p 17");
        c.start_shogi(
            WorkLimits {
                depth: 1,
                nodes: 1000,
            },
            true,
        )
        .unwrap();
        let shogi = c.shared.shogi.as_ref().unwrap();
        let pending = shogi.pending.clone();
        let record = shogi.record();
        let token = c.shared_token();
        let inspected = query(&c, "5d").unwrap();
        assert_eq!(
            inspected
                .choices
                .iter()
                .map(|a| a.action.as_str())
                .collect::<Vec<_>>(),
            ["5d5c", "5d5c+"]
        );
        assert_eq!(
            inspected.choices[0].completion.as_ref().unwrap().label,
            "Do not promote"
        );
        let origin = format!("hand:{}:P", side(NativeSide::Shogi(s::Side::Gote)).key);
        let hand = query(&c, &origin).unwrap();
        assert_eq!(hand.label, "Gote pawn in hand");
        assert!(hand.choices.iter().any(|a| a.action == "P*4f"));
        assert!(!hand.choices.iter().any(|a| a.action.ends_with('i')));
        assert!(query(&c, &origin.replace(":P", ":R")).is_err());
        assert_eq!(c.shared.shogi.as_ref().unwrap().record(), record);
        assert_eq!(c.shared.shogi.as_ref().unwrap().pending, pending);
        assert_eq!(c.shared_token(), token);
    }
    #[test]
    fn exact_historical_selection_and_conclusion_are_read_only() {
        let mut c = controller();
        native_shogi(&mut c, s::INITIAL_SFEN);
        let state = c.shared.shogi.as_mut().unwrap();
        let prefix = state.game.clone();
        let request = s::GameRequest {
            expected: state.game.context(),
            actor: s::Side::Sente,
            command: s::GameCommand::Play("7g7f".parse().unwrap()),
        };
        state.game.submit(request).unwrap();
        state.viewed = Some(prefix);
        let before = state.record();
        assert!(query(&c, "7g").is_ok());
        assert!(query(&c, "7f").is_err());
        assert_eq!(c.shared.shogi.as_ref().unwrap().record(), before);
        let stale = MovementInspectionQuery {
            expected: c.shared_token(),
            origin: "7g".into(),
        };
        c.session.selection_id += 1;
        assert!(c.shared_inspection(stale).is_err());
        let state = c.shared.shogi.as_mut().unwrap();
        state.viewed = None;
        let resignation = s::GameRequest {
            expected: state.game.context(),
            actor: s::Side::Gote,
            command: s::GameCommand::Resign,
        };
        state.game.submit(resignation).unwrap();
        let ended = state.record();
        assert!(query(&c, "7f").unwrap().read_only);
        assert_eq!(c.shared.shogi.as_ref().unwrap().record(), ended);
    }
    #[test]
    fn strict_query_rejects_duplicate_nested_unknown_missing_and_oversize() {
        let valid =
            r#"{"expected":{"document":1,"version":1,"revision":1,"selection":1},"origin":"b8"}"#;
        assert!(MovementInspectionQuery::from_json(valid).is_ok());
        for invalid in [
            valid.replace("\"origin\":\"b8\"", "\"origin\":\"b8\",\"origin\":\"a8\""),
            valid.replace("\"version\":1", "\"version\":1,\"version\":1"),
            valid.replace("\"origin\":\"b8\"", "\"unexpected\":\"b8\""),
            valid.replace(",\"origin\":\"b8\"", ""),
            " ".repeat(1025),
        ] {
            assert!(MovementInspectionQuery::from_json(&invalid).is_err());
        }
    }
}
