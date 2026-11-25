//! Authoritative hard states for separately labeled QA packages only.
use super::*;

fn submit(game: &mut x::Game, actor: x::Side, command: x::GameCommand) -> Result<(), String> {
    game.submit(x::GameRequest {
        expected: game.context(),
        actor,
        command,
    })
    .map_err(|e| e.to_string())?;
    Ok(())
}
fn play(game: &mut x::Game, action: &str) -> Result<(), String> {
    submit(
        game,
        game.position().side_to_move(),
        x::GameCommand::Play {
            action: action
                .parse()
                .map_err(|e: x::ActionParseError| e.to_string())?,
        },
    )
}
fn corpus(text: &str, id: &str, maximum: usize) -> Result<x::Game, String> {
    let corpus: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let fixture = corpus["fixtures"]
        .as_array()
        .and_then(|fixtures| fixtures.iter().find(|f| f["id"] == id))
        .ok_or("Missing independent native verification fixture.")?;
    let root = x::PositionRoot::from_fen(
        fixture["root_fen"]
            .as_str()
            .ok_or("Missing fixture root.")?,
    )
    .map_err(|e| e.to_string())?;
    let mut game = x::Game::from_root(root, x::ClaimPolicy::Explicit).map_err(|e| e.to_string())?;
    for action in fixture["moves"]
        .as_array()
        .ok_or("Missing fixture actions.")?
        .iter()
        .take(maximum)
    {
        play(&mut game, action.as_str().ok_or("Invalid fixture action.")?)?;
    }
    Ok(game)
}
impl DesktopController {
    fn finish_verification_admission(&mut self, receipt: AdmissionReceipt) -> Result<(), String> {
        let deadline = Instant::now() + std::time::Duration::from_secs(30);
        loop {
            self.poll_native_admission();
            if let Some(result) = receipt.completion() {
                return result;
            }
            if Instant::now() >= deadline {
                return Err("Xiangqi verification admission timed out.".into());
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
    pub(crate) fn verification_xiangqi(&mut self, fixture: &str) -> Result<(), String> {
        let mut game = match fixture {
            "xiangqi-standard"
            | "xiangqi-failure"
            | "xiangqi-capacity"
            | "xiangqi-control-capacity"
            | "xiangqi-offer"
            | "xiangqi-agreed"
            | "xiangqi-resigned" => {
                x::Game::standard(x::ClaimPolicy::Explicit).map_err(|e| e.to_string())?
            }
            "xiangqi-warning" | "xiangqi-fourth" => corpus(
                include_str!("../../../tests/fixtures/xiangqi/history-cycles.json"),
                if fixture == "xiangqi-warning" {
                    "F022-CC"
                } else {
                    "F023-CCC"
                },
                usize::MAX,
            )?,
            "xiangqi-count" => corpus(
                include_str!("../../../tests/fixtures/xiangqi/natural-count.json"),
                "NATURAL-10-CHECKS",
                100,
            )?,
            "xiangqi-fast-proof" => x::Game::from_root(
                x::PositionRoot::from_fen("5k3/9/9/9/9/9/9/9/9/3K5 w - - 0 1")
                    .map_err(|e| e.to_string())?,
                x::ClaimPolicy::Explicit,
            )
            .map_err(|e| e.to_string())?,
            _ => return Err("Unknown Xiangqi verification fixture.".into()),
        };
        if matches!(fixture, "xiangqi-offer" | "xiangqi-agreed") {
            play(&mut game, "a3a4")?;
            play(&mut game, "a6a5")?;
            submit(&mut game, x::Side::Black, x::GameCommand::OfferDraw)?;
            if fixture == "xiangqi-agreed" {
                submit(&mut game, x::Side::Red, x::GameCommand::AcceptDraw)?;
            }
        }
        if fixture == "xiangqi-resigned" {
            submit(&mut game, x::Side::Red, x::GameCommand::Resign)?;
        }
        let human = if fixture == "xiangqi-failure" {
            x::Side::Black
        } else {
            x::Side::Red
        };
        let record = x::GameRecord::from_game(&game).map_err(|e| e.to_string())?;
        let operation = game
            .conclusion()
            .is_none()
            .then_some(SessionOperation::Resume {});
        // There is no control journal any more; the control-capacity fixture
        // is the standard game (MSH-228).
        let seed = native_session::RuntimeSeed::Record {
            record: Box::new(record),
            human,
            play_mode: PlayMode::HumanEngine,
        };
        let receipt = self.queue_game_candidate::<Xiangqi>(seed, operation, None, true)?;
        self.finish_verification_admission(receipt)?;
        if fixture == "xiangqi-capacity" {
            // Real native results, not a forged count or fabricated observations.
            // Poll only admission here so the supervised child is not also submitted.
            for _ in 0..MAX_ARCHIVE_OBSERVATIONS - 1 {
                self.start_game_work::<Xiangqi>(
                    WorkLimits {
                        time_ms: None,
                        depth: 1,
                        nodes: 1,
                    },
                    true,
                )
                .map_err(|e| e.diagnostic)?;
                let request = self
                    .session
                    .xiangqi()
                    .unwrap()
                    .pending
                    .as_ref()
                    .ok_or("Missing verification analysis request.")?;
                let mut bytes = serde_json::to_vec(&execution_request(request.clone()))
                    .map_err(|e| e.to_string())?;
                bytes.push(b'\n');
                let mut response = Vec::new();
                mshogi_engine::serve_once(std::io::Cursor::new(bytes), &mut response)
                    .map_err(|e| e.to_string())?;
                let response =
                    ExecutionResponse::decode(&response, &execution_request(request.clone()))?;
                if !self.accept_shared(response) {
                    return Err("Verification observation failed native context admission.".into());
                }
                self.finish_verification_admission(
                    self.admission_receipt
                        .clone()
                        .ok_or("Missing verification response admission.")?,
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn labeled_capacity_fixture_retains_real_observations_and_one_remaining_slot() {
        let mut controller = DesktopController::verification(
            "/unused/verification-engine".into(),
            "xiangqi-capacity",
        )
        .unwrap();
        controller.shared.manual_engine = true;
        let state = controller.session.xiangqi().unwrap();
        let observations = &state.observations;
        assert_eq!(observations.len(), MAX_ARCHIVE_OBSERVATIONS - 1);
        assert!(observations.iter().all(|o| matches!(
            &o.result,
            ExecutionResult::Xiangqi {
                result: XiangqiResult::Incomplete { .. }
            }
        )));
        assert_eq!(state.game.events().len(), 0);
        crate::game_document::ValidatedGameDocument::<Xiangqi>::decode(
            state.document.bytes(),
            &mut || false,
        )
        .unwrap();
    }
    #[test]
    fn labeled_native_verification_roots_replay_actual_facts() {
        for name in [
            "xiangqi-standard",
            "xiangqi-warning",
            "xiangqi-fourth",
            "xiangqi-count",
            "xiangqi-offer",
            "xiangqi-agreed",
            "xiangqi-resigned",
            "xiangqi-fast-proof",
            "xiangqi-failure",
        ] {
            let mut controller =
                DesktopController::verification("/unused/verification-engine".into(), name)
                    .unwrap();
            controller.shared.manual_engine = true;
            let snapshot = controller.game_snapshot::<Xiangqi>();
            let facts = snapshot.xiangqi.unwrap();
            assert_eq!(snapshot.game, GameKind::Xiangqi);
            match name {
                "xiangqi-warning" => assert!(facts.warnings.iter().any(|w| w.active)),
                "xiangqi-count" => assert!(
                    facts
                        .count_advice
                        .iter()
                        .any(|c| c.claimant == x::Side::Red && c.eligible)
                ),
                "xiangqi-offer" => assert!(
                    facts.terminal_actions.accept_draw && facts.terminal_actions.decline_draw
                ),
                "xiangqi-fourth" | "xiangqi-agreed" | "xiangqi-resigned" | "xiangqi-fast-proof" => {
                    assert!(snapshot.status.concluded)
                }
                "xiangqi-failure" => {
                    assert_eq!(snapshot.human.id, NativeSide::Xiangqi(x::Side::Black))
                }
                _ => assert!(!snapshot.status.concluded),
            }
            let record = match snapshot.context {
                CanonicalRecord::Xiangqi(record) => record,
                _ => panic!("native Xiangqi context"),
            };
            assert_eq!(
                record.to_game().unwrap().events(),
                controller.session.xiangqi().unwrap().game.events()
            );
        }
    }
}
