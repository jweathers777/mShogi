//! Non-search request-path probe. Run as an example against the frozen source.
#![allow(unexpected_cfgs)]
use mshogi_core::{native::CanonicalRecord, shogi::{Game, GameRecord}};
use mshogi_engine::*;
use mshogi_search::shogi::position::ShogiPosition;
use serde_json::json;
use std::{fs, hint::black_box, io::{self, BufRead, Write}, time::{Duration, Instant}};

fn request(game: &Game, id: u64) -> ExecutionRequest {
    ExecutionRequest { protocol: PROTOCOL_VERSION, context: ExecutionContext {
        purpose: ExecutionPurpose::EngineTurn {}, request_id:id, session_version:id,
        process_generation:id, game:CanonicalRecord::shogi(GameRecord::from_game(game)),
        limits:WorkLimits { depth:64, nodes:5_000_000, time_ms:Some(1_000) },
    }}
}
fn setup(game: &Game) {
    black_box(game.conclusion());
    black_box(game.declaration_eligibility(game.position().side_to_move()));
    let legal=game.position().legal_actions();
    let position=ShogiPosition::from_game(game);
    #[cfg(incremental_probe)]
    black_box(position.find(legal[0]).expect("native legal fallback maps to private root"));
    black_box(legal);black_box(position);
}
fn response(req: &ExecutionRequest) -> ExecutionResponse {
    ExecutionResponse { protocol:PROTOCOL_VERSION, observation:ExecutionObservation::new(
        req.context.clone(), ExecutionResult::Failure { failure:EngineFailure {kind:FailureKind::Search,message:"measurement: no search performed".into()} }) }
}
fn admit(req: &ExecutionRequest, kept: &mut Option<GameRecord>) {
    assert!(req.valid());
    let CanonicalRecord::Shogi(record)=&req.context.game else {panic!("Shogi fixture required")};
    let game=record.to_game().unwrap();
    if let Some(old)=kept { black_box(record.has_prefix(old)); }
    *kept=Some((**record).clone());
    setup(&game);
}
#[cfg(not(incremental_probe))]
fn server() {
    let stdin=io::stdin();let mut out=io::stdout().lock();let mut kept=None;
    for line in stdin.lock().lines() {
        let req:ExecutionRequest=serde_json::from_str(&line.unwrap()).unwrap();
        admit(&req,&mut kept);
        serde_json::to_writer(&mut out,&response(&req)).unwrap();out.write_all(b"\n").unwrap();out.flush().unwrap();
    }
}
#[cfg(incremental_probe)]
fn candidate_dispatch(bytes:&[u8],session:&mut incremental::Server)->Vec<u8> {
    let mut admitted=false;
    let run=|game:&Game,_,_,_:&mshogi_search::alphabeta::timing::RequestTime| {admitted=true;setup(game);probe_result()};
    let encoded=if let Ok(req)=serde_json::from_slice::<incremental::Request>(bytes) {
        serde_json::to_vec(&session.execute(req,run)).unwrap()
    } else {
        let req:ExecutionRequest=serde_json::from_slice(bytes).unwrap();
        serde_json::to_vec(&session.begin(&req,run)).unwrap()
    };
    assert!(admitted,"measurement request must reach native no-search callback");
    encoded
}
#[cfg(incremental_probe)]
fn server() {
    let stdin=io::stdin();let mut out=io::stdout().lock();let mut session=incremental::Server::default();
    for line in stdin.lock().lines() {
        let bytes=candidate_dispatch(line.unwrap().as_bytes(),&mut session);
        out.write_all(&bytes).unwrap();out.write_all(b"\n").unwrap();out.flush().unwrap();
    }
}
#[cfg(incremental_probe)]
fn probe_result()->ExecutionResult {
    ExecutionResult::Failure {failure:EngineFailure {kind:FailureKind::Search,message:"measurement: no search performed".into()}}
}
#[cfg(incremental_probe)]
fn exchange(client:&mut incremental::Client,server:&mut incremental::Server,host:std::sync::Arc<ExecutionRequest>)->(usize,usize) {
    let bytes=client.encode(&host).unwrap();let request_bytes=bytes.len();
    let reply=candidate_dispatch(&bytes,server);
    let response=client.decode(&reply,host.clone()).unwrap();assert!(response.matches(&host));assert_probe_result(&response);
    (request_bytes,reply.len())
}
#[cfg(incremental_probe)]
fn incremental_cycles(game:&Game,prior:&Game,count:usize)->serde_json::Value {
    let mut warm=vec![];let mut cold=vec![];let mut sizes=(0,0);
    for i in 0..count+2 {
        let mut client=incremental::Client::default();let mut server=incremental::Server::default();
        let begin=Instant::now();
        exchange(&mut client,&mut server,std::sync::Arc::new(request(prior,1)));
        let cold_ms=begin.elapsed().as_secs_f64()*1000.;
        let begin=Instant::now();
        sizes=exchange(&mut client,&mut server,std::sync::Arc::new(request(game,2)));
        let warm_ms=begin.elapsed().as_secs_f64()*1000.;
        if i>=2 {warm.push(warm_ms);cold.push(cold_ms);}
    }
    json!({"request_bytes":sizes.0,"response_bytes":sizes.1,"steady_two_action_nonsearch_ms":warm,
           "cold_prior_history_full_reset_ms":cold,"includes":"host record/request, client encode, strict wire decode, native admission, legal root/private history setup, response encode, client decode/exact match"})
}
fn measured(mut f:impl FnMut(), count:usize)->Vec<f64> {
    for _ in 0..2 {f();}
    (0..count).map(|_| {let now=Instant::now();f();now.elapsed().as_secs_f64()*1000.}).collect()
}
fn assert_probe_result(response:&ExecutionResponse) {
    assert!(matches!(&response.observation.result,ExecutionResult::Failure {failure}
        if failure.kind==FailureKind::Search && failure.message=="measurement: no search performed"),
        "only an admitted no-search callback is a valid timing result");
}
fn receive(bridge:&EngineBridge,req:&ExecutionRequest)->ExecutionResponse {
    let limit=Instant::now()+Duration::from_secs(5);
    loop {
        if let Some(result)=bridge.poll() {assert!(result.matches(req));return result;}
        assert!(Instant::now()<limit,"probe watchdog");
        std::thread::sleep(Duration::from_micros(50));
    }
}
fn wait(bridge:&EngineBridge,req:&ExecutionRequest) {assert_probe_result(&receive(bridge,req));}
fn compatibility(path:&str) {
    use mshogi_core::shogi::{GameCommand,GameRequest};
    let mut later=Game::new();
    for action in ["7g7f","3c3d"] {
        later.submit(GameRequest {expected:later.context(),actor:later.position().side_to_move(),command:GameCommand::Play(action.parse().unwrap())}).unwrap();
    }
    let bridge=EngineBridge::new(path.into()).unwrap();let mut results=vec![];
    for (i,game) in [Game::new(),later,Game::new()].iter().enumerate() {
        let mut req=request(game,i as u64+1);req.context.limits.depth=1;req.context.limits.nodes=5000;
        bridge.submit(req.clone()).unwrap();let reply=receive(&bridge,&req);
        let ExecutionResult::Shogi {result:ShogiResult::Choice {choice,telemetry}}=&reply.observation.result else {panic!("compatibility requires actual Shogi choice: {reply:?}")};
        assert_eq!(telemetry.completed_depth,1);assert!(telemetry.nodes>0);assert!(telemetry.score.is_some());
        let command=match choice {ShogiChoice::Move {action}=>GameCommand::Play(*action),ShogiChoice::Declare {actor:_}=>GameCommand::DeclareEnteringKing};
        let mut native=game.clone();native.submit(GameRequest {expected:native.context(),actor:native.position().side_to_move(),command}).unwrap();
        results.push(json!({"request_id":i+1,"events":game.events().len(),"choice":choice,"telemetry":telemetry,"native_applied":true}));
    }
    bridge.shutdown();let deadline=Instant::now()+Duration::from_secs(2);
    while !bridge.shutdown_acknowledged() {assert!(Instant::now()<deadline);std::thread::sleep(Duration::from_millis(1));}
    println!("{}",serde_json::to_string_pretty(&json!({"mode":"actual production EngineBridge peer compatibility","engine":path,"results":results,"shutdown_acknowledged":true})).unwrap());
}
fn main() {
    let args:Vec<_>=std::env::args().collect();
    if args.get(1).is_some_and(|a|a=="compatibility") {compatibility(&args[2]);return;}
    if args.get(1).is_some_and(|a|a=="internal-search") {server();return;}
    let text=fs::read_to_string(&args[1]).unwrap();
    let full=GameRecord::from_json(&text).unwrap().to_game().unwrap();
    let count=args.get(2).map_or(9,|s|s.parse().unwrap());
    let mut output=vec![];
    for n in [50,150,300] {
        let game=full.prefix(n).unwrap();let prior=full.prefix(n-2).unwrap();
        assert!(game.conclusion().is_none());
        let req=request(&game,2);let wire=serde_json::to_vec(&req).unwrap();
        let record=GameRecord::from_game(&game);let reply=response(&req);let reply_wire=serde_json::to_vec(&reply).unwrap();
        let mut phases=serde_json::Map::new();
        phases.insert("host_record_and_request_ms".into(),json!(measured(||{black_box(request(&game,2));},count)));
        phases.insert("request_encode_ms".into(),json!(measured(||{black_box(serde_json::to_vec(&req).unwrap());},count)));
        phases.insert("strict_request_decode_ms".into(),json!(measured(||{black_box(serde_json::from_slice::<ExecutionRequest>(&wire).unwrap());},count)));
        phases.insert("native_reconstruct_ms".into(),json!(measured(||{black_box(record.to_game().unwrap());},count)));
        phases.insert("root_setup_including_private_history_ms".into(),json!(measured(||setup(&game),count)));
        phases.insert("response_construct_encode_ms".into(),json!(measured(||{black_box(serde_json::to_vec(&response(&req)).unwrap());},count)));
        phases.insert("strict_response_decode_match_ms".into(),json!(measured(||{let r:ExecutionResponse=serde_json::from_slice(&reply_wire).unwrap();assert!(r.matches(&req));},count)));
        let mut combined=vec![];
        for i in 0..count+2 {
            let mut kept=Some(GameRecord::from_game(&prior));
            let start=Instant::now();
            let host=request(&game,2);let wire=serde_json::to_vec(&host).unwrap();
            let child:ExecutionRequest=serde_json::from_slice(&wire).unwrap();admit(&child,&mut kept);
            let wire=serde_json::to_vec(&response(&child)).unwrap();
            let reply:ExecutionResponse=serde_json::from_slice(&wire).unwrap();assert!(reply.matches(&host));assert_probe_result(&reply);
            if i>=2 {combined.push(start.elapsed().as_secs_f64()*1000.);}
        }
        phases.insert("combined_nonsearch_cycle_ms".into(),json!(combined));
        let bridge=EngineBridge::new(std::env::current_exe().unwrap()).unwrap();let mut id=0;
        let mut roundtrips=vec![];
        for i in 0..count+2 {
            id+=1;let warm=request(&prior,id);bridge.submit(warm.clone()).unwrap();wait(&bridge,&warm);
            id+=1;let begin=Instant::now();let host=request(&game,id);bridge.submit(host.clone()).unwrap();wait(&bridge,&host);
            let elapsed=begin.elapsed().as_secs_f64()*1000.;if i>=2 {roundtrips.push(elapsed);}
        }
        bridge.shutdown();let deadline=Instant::now()+Duration::from_secs(2);
        while !bridge.shutdown_acknowledged() {assert!(Instant::now()<deadline);std::thread::sleep(Duration::from_millis(1));}
        phases.insert("production_bridge_no_search_roundtrip_ms".into(),json!(roundtrips));
        #[cfg(incremental_probe)]
        let compact=incremental_cycles(&game,&prior,count);
        #[cfg(not(incremental_probe))]
        let compact=serde_json::Value::Null;
        output.push(json!({"incremental":compact,"events":n,"suffix_events":2,"request_bytes":wire.len(),"response_bytes":reply_wire.len(),"samples":phases}));
    }
    println!("{}",serde_json::to_string_pretty(&json!({"mode":if cfg!(incremental_probe) {"incremental transport; legacy components also measured; no search"} else {"baseline full-record protocol; no search"},"repetitions":count,"warmups":2,"measurements":output})).unwrap());
}
