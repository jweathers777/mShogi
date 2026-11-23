//! Measure current response traffic, decoding, context copies and native PV validation.
#[path = "../../mshogi-core/examples/support/walk.rs"]
mod walk;
use mshogi_core::native::CanonicalRecord;
use mshogi_engine::*;
use std::{hint::black_box, sync::Arc, time::Instant};
use walk::Failure;

fn median<T>(
    mut prepare: impl FnMut() -> Result<T, Failure>,
    mut run: impl FnMut(T) -> Result<(), Failure>,
) -> Result<f64, Failure> {
    let mut samples = Vec::new();
    for _ in 0..5 {
        let input = prepare()?;
        let started = Instant::now();
        run(input)?;
        samples.push(started.elapsed().as_nanos());
    }
    samples.sort();
    Ok(samples[2] as f64 / 1000.0)
}
fn measure(
    record: CanonicalRecord,
    result: ExecutionResult,
    mut validate: impl FnMut() -> Result<(), String>,
) -> Result<(), Failure> {
    let request = Arc::new(ExecutionRequest {
        protocol: PROTOCOL_VERSION,
        context: ExecutionContext {
            purpose: ExecutionPurpose::EngineTurn {},
            request_id: 1,
            session_version: 1,
            process_generation: 1,
            game: record,
            limits: WorkLimits {
                depth: 1,
                nodes: 1,
                time_ms: None,
            },
        },
    });
    let mut client = incremental::Client::default();
    let request_bytes = client.encode(&request)?;
    let response = incremental::Response::new(&request_bytes, result);
    let response_bytes = serde_json::to_vec(&response)?;
    let decode = median(
        || {
            let mut client = incremental::Client::default();
            client.encode(&request)?;
            Ok(client)
        },
        |mut client| {
            let decoded = client.decode(black_box(&response_bytes)).map_err(|e| e.1)?;
            assert!(decoded.matches(&request));
            Ok(())
        },
    )?;
    let clone = median(
        || Ok(()),
        |_| {
            black_box(request.context.clone());
            Ok(())
        },
    )?;
    let validation = median(
        || Ok(()),
        |_| {
            validate()?;
            Ok(())
        },
    )?;
    println!(
        "{},{},{},{decode:.3},{clone:.3},{validation:.3}",
        request.context.game.event_count(),
        request_bytes.len(),
        response_bytes.len()
    );
    Ok(())
}
macro_rules! run {
    ($module:ident, $family:ident, $result:ident, $telemetry:ident, $game:expr, $count:expr) => {{
        use mshogi_core::$module::*;
        use mshogi_search::$module as search;
        let mut game = $game;
        let (_, action) = walk::prefix(
            &mut game,
            $count,
            |game, _| game.legal_actions(),
            |game, action| {
                Ok(game.submit(GameRequest {
                    expected: game.context(),
                    actor: game.position().side_to_move(),
                    command: GameCommand::Play { action },
                })?)
            },
            |game, token| {
                game.undo(token)?;
                Ok(())
            },
            |game| game.conclusion().is_some(),
        )?;
        let result = mshogi_engine::$result::Choice {
            action: Clone::clone(&action),
            telemetry: mshogi_engine::$telemetry {
                evaluator_id: search::EVALUATOR_ID.into(),
                search_id: search::SEARCH_ID.into(),
                perspective: game.position().side_to_move(),
                score: Some(0),
                completed_depth: 1,
                nodes: 1,
                principal_variation: vec![action],
                status: StopReason::Complete,
            },
        };
        measure(
            CanonicalRecord::$family(Box::new(GameRecord::from_game(&game)?)),
            ExecutionResult::$family {
                result: result.clone(),
            },
            || result.validate(&game),
        )
    }};
}
fn main() -> Result<(), Failure> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let kind = args
        .first()
        .ok_or("game: tori|chu|tenjiku-adams|tenjiku-muller")?;
    let count: usize = args.get(1).ok_or("history length")?.parse()?;
    if count > 1000 {
        return Err("history length must be <=1000".into());
    }
    println!("events,request_bytes,response_bytes,decode_us,context_clone_us,validate_us");
    match kind.as_str() {
        "tori" => run!(
            tori,
            Tori,
            ToriResult,
            ToriTelemetry,
            Game::standard(),
            count
        ),
        "chu" => run!(chu, Chu, ChuResult, ChuTelemetry, Game::standard(), count),
        "tenjiku-adams" => run!(
            tenjiku,
            Tenjiku,
            TenjikuResult,
            TenjikuTelemetry,
            Game::standard(Profile::AdamsModern),
            count
        ),
        "tenjiku-muller" => run!(
            tenjiku,
            Tenjiku,
            TenjikuResult,
            TenjikuTelemetry,
            Game::standard(Profile::MullerCompleted),
            count
        ),
        _ => Err("unknown game".into()),
    }
}
