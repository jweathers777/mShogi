use mshogi_core::game::CanonicalRecord;
use mshogi_engine::*;
use std::{
    io::{Read, Write},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

fn actual(request: &ExecutionRequest) -> ExecutionResponse {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mshogi"))
        .arg("internal-search")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    writeln!(
        child.stdin.take().unwrap(),
        "{}",
        serde_json::to_string(request).unwrap()
    )
    .unwrap();
    // Drain both pipes while the process runs (especially Windows' small pipe
    // buffers); retain only bounded diagnostics.
    let mut stdout = child.stdout.take().unwrap();
    let stdout = thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout.read_to_end(&mut bytes).unwrap();
        bytes
    });
    let stderr = child.stderr.take().unwrap();
    let stderr = thread::spawn(move || {
        let mut bytes = Vec::new();
        stderr
            .take((MAX_STDERR_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .unwrap();
        bytes
    });
    let deadline = Instant::now() + Duration::from_secs(20);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() >= deadline {
            child.kill().unwrap();
            let _ = child.wait();
            panic!("bounded self-play child exceeded independent 20-second watchdog");
        }
        thread::sleep(Duration::from_millis(1));
    }
    let status = child.wait().unwrap();
    let stdout = stdout.join().unwrap();
    let stderr = stderr.join().unwrap();
    assert!(status.success(), "{}", String::from_utf8_lossy(&stderr));
    assert!(stderr.is_empty());
    let response: ExecutionResponse = ExecutionResponse::decode(&stdout, request).unwrap();
    assert!(response.matches(request));
    response
}

#[derive(Debug, Eq, PartialEq)]
pub struct Trace<A> {
    pub actions: Vec<A>,
    pub record: String,
    pub observations: Vec<ExecutionObservation>,
}

pub trait SelfplayGame: EngineGame {
    fn envelope(game: &Self::Game) -> CanonicalRecord;
    fn legal(game: &Self::Game) -> Vec<Self::Choice>;
    fn conclusion(game: &Self::Game) -> Option<Self::Conclusion>;
    fn admit(game: &mut Self::Game, action: Self::Choice);
}

macro_rules! game_adapter {
    ($module:ident, $kind:ident) => {
        impl SelfplayGame for game::$kind {
            fn envelope(game: &Self::Game) -> CanonicalRecord {
                CanonicalRecord::$module(mshogi_core::$module::GameRecord::from_game(game).unwrap())
            }
            fn legal(game: &Self::Game) -> Vec<Self::Choice> {
                game.legal_actions()
            }
            fn conclusion(game: &Self::Game) -> Option<Self::Conclusion> {
                game.conclusion()
            }
            fn admit(game: &mut Self::Game, action: Self::Choice) {
                use mshogi_core::$module::{GameCommand, GameRequest};
                game.submit(GameRequest {
                    expected: game.context(),
                    actor: game.position().side_to_move(),
                    command: GameCommand::Play { action },
                })
                .unwrap();
            }
        }
    };
}
game_adapter!(tori, Tori);
game_adapter!(tenjiku, Tenjiku);

/// Check actual choices against legal actions and independently replay
/// the final, complete record. Each fixture supplies its own depth expectation.
pub fn bounded<G: SelfplayGame>(
    mut game: G::Game,
    maximum_plies: usize,
    limits: WorkLimits,
    check_depth: impl Fn(u16),
) -> Trace<G::Choice> {
    let run = |game: &G::Game, id: u64| {
        actual(&ExecutionRequest {
            protocol: PROTOCOL_VERSION,
            context: ExecutionContext {
                purpose: ExecutionPurpose::EngineTurn {},
                request_id: id,
                session_version: id,
                game: G::envelope(game),
                limits,
            },
        })
    };
    let mut actions = Vec::new();
    let mut observations = Vec::new();
    for id in 1..=maximum_plies as u64 {
        if G::conclusion(&game).is_some() {
            break;
        }
        let response = run(&game, id);
        let result = G::result(&response.observation.result).expect("expected engine result");
        result.validate(&game).unwrap();
        let EngineResult::Choice { choice, telemetry } = result else {
            panic!("expected a completed choice: {result:?}");
        };
        check_depth(telemetry.completed_depth);
        assert!(telemetry.score.is_some());
        assert!(telemetry.nodes <= limits.nodes);
        assert_eq!(telemetry.perspective, G::side_to_move(&game));
        assert!(G::legal(&game).contains(choice));
        actions.push(choice.clone());
        G::admit(&mut game, choice.clone());
        observations.push(response.observation);
    }
    if let Some(expected) = G::conclusion(&game) {
        let response = run(&game, maximum_plies as u64 + 1);
        let Some(EngineResult::Finished {
            conclusion,
            telemetry,
        }) = G::result(&response.observation.result)
        else {
            panic!("expected terminal root");
        };
        assert_eq!(*conclusion, expected);
        assert!(telemetry.principal_variation.is_empty());
        observations.push(response.observation);
    }
    let game = G::envelope(&game);
    let record = game.to_json().unwrap();
    let restored = CanonicalRecord::from_json(&record).unwrap();
    assert_eq!(restored, game);
    let replay = G::to_game(G::record(&restored).expect("expected game record")).unwrap();
    assert_eq!(G::envelope(&replay), game);
    Trace {
        actions,
        record,
        observations,
    }
}
