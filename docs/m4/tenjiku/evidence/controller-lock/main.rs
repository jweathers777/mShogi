//! External read-only measurement; no Tauri runtime or product code changes.
use mshogi_desktop_session::{DesktopController, MovementInspection, MovementInspectionQuery};
use serde_json::{Value, json};
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
    time::Instant,
};

fn ns(start: Instant, end: Instant) -> u64 {
    u64::try_from(end.duration_since(start).as_nanos()).unwrap()
}
fn stats(mut values: Vec<u64>) -> Value {
    values.sort_unstable();
    json!({"samples":values.len(),"total_ns":values.iter().sum::<u64>(),
        "min_ns":values[0],"p50_ns":values[values.len()/2],
        "p95_ns":values[(values.len()-1)*95/100],"max_ns":values[values.len()-1]})
}
struct Measured {
    result: MovementInspection,
    wait_ns: u64,
    held_ns: u64,
    call_ns: u64,
}
fn inspect(
    owner: &Mutex<DesktopController>,
    shutdown: &AtomicBool,
    query: MovementInspectionQuery,
) -> Measured {
    // Same std::sync::Mutex and lifecycle checks as host with_controller.
    assert!(!shutdown.load(Ordering::Acquire));
    let waiting = Instant::now();
    let controller = owner.lock().unwrap();
    let acquired = Instant::now();
    assert!(!shutdown.load(Ordering::Acquire) && !controller.lifecycle().close_approved());
    let result = controller.shared_inspection(query).unwrap();
    let call_done = Instant::now();
    drop(controller);
    let released = Instant::now();
    Measured {
        result,
        wait_ns: ns(waiting, acquired),
        held_ns: ns(acquired, released),
        call_ns: ns(acquired, call_done),
    }
}
fn main() {
    println!(
        "{}",
        json!({"kind":"scope","source_revision":"3a290024df82d23f215165707c81de4928ad3a4a",
        "build":"offline release; current public DesktopController; desktop-verification inputs",
        "lock":"actual std::sync::Mutex<DesktopController>, host-style shutdown/lifecycle checks",
        "timer":"std::time::Instant nanoseconds; acquisition wait separate from acquired-to-drop duration",
        "excluded":["Tauri IPC and query JSON parsing","native WebView rendering","production deadlines and timing guarantees"],
        "repetitions":3,"contention":"one competing real inspection; no synthetic sleep or timing assertion"})
    );
    for profile in ["adams", "muller"] {
        for scenario in ["standard", "dense-fire", "dense-area"] {
            let controller = DesktopController::verification(
                "/unused/no-tenjiku-engine-execution".into(),
                &format!("tenjiku-{profile}-{scenario}"),
            )
            .unwrap();
            let owner = Arc::new(Mutex::new(controller));
            let shutdown = AtomicBool::new(false);
            let before = owner.lock().unwrap().shared_snapshot();
            assert_eq!(before.pieces.len(), 156);
            assert!(!before.capabilities.engine_turn && !before.capabilities.move_analysis);
            let token = before.token;
            let mut waits = vec![];
            let mut held = vec![];
            let mut calls = vec![];
            let mut encode = vec![];
            let mut bytes = 0usize;
            let mut choices = 0usize;
            let mut slowest = (0u64, String::new(), 0usize, 0usize);
            for _ in 0..3 {
                for piece in &before.pieces {
                    let measured = inspect(
                        &owner,
                        &shutdown,
                        MovementInspectionQuery {
                            expected: token,
                            origin: piece.square.clone(),
                        },
                    );
                    assert!(measured.result.read_only);
                    let count = measured.result.choices.len();
                    let start = Instant::now();
                    let encoded = serde_json::to_vec_pretty(&measured.result).unwrap();
                    encode.push(ns(start, Instant::now()));
                    if measured.held_ns > slowest.0 {
                        slowest = (measured.held_ns, piece.square.clone(), count, encoded.len());
                    }
                    bytes += encoded.len();
                    choices += count;
                    waits.push(measured.wait_ns);
                    held.push(measured.held_ns);
                    calls.push(measured.call_ns);
                }
            }
            let mut snapshot_wait = vec![];
            let mut snapshot_held = vec![];
            let mut snapshot_encode = vec![];
            let mut snapshot_bytes = 0;
            for _ in 0..3 {
                let waiting = Instant::now();
                let mut guard = owner.lock().unwrap();
                let acquired = Instant::now();
                assert!(!shutdown.load(Ordering::Acquire) && !guard.lifecycle().close_approved());
                let snapshot = guard.shared_snapshot();
                drop(guard);
                let released = Instant::now();
                let start = Instant::now();
                snapshot_bytes = serde_json::to_vec_pretty(&snapshot).unwrap().len();
                snapshot_encode.push(ns(start, Instant::now()));
                snapshot_wait.push(ns(waiting, acquired));
                snapshot_held.push(ns(acquired, released));
            }
            // The contender holds the actual controller mutex for one actual
            // native query. Sending readiness adds explicit synchronization
            // overhead; no delay is injected to force an artificial wait.
            let (ready_send, ready_recv) = mpsc::channel();
            let busy = Arc::new(AtomicBool::new(false));
            let worker_owner = owner.clone();
            let worker_busy = busy.clone();
            let origin = slowest.1.clone();
            let worker = thread::spawn(move || {
                let guard = worker_owner.lock().unwrap();
                let acquired = Instant::now();
                worker_busy.store(true, Ordering::Release);
                ready_send.send(()).unwrap();
                let result = guard
                    .shared_inspection(MovementInspectionQuery {
                        expected: token,
                        origin,
                    })
                    .unwrap();
                worker_busy.store(false, Ordering::Release);
                drop(guard);
                let released = Instant::now();
                (ns(acquired, released), result.choices.len())
            });
            ready_recv.recv().unwrap();
            let worker_busy_at_attempt = busy.load(Ordering::Acquire);
            let contended = inspect(
                &owner,
                &shutdown,
                MovementInspectionQuery {
                    expected: token,
                    origin: slowest.1.clone(),
                },
            );
            let (worker_held_ns, worker_choices) = worker.join().unwrap();
            let after = owner.lock().unwrap().shared_snapshot();
            assert_eq!(after.token, before.token);
            assert_eq!(after.context, before.context);
            assert_eq!(after.document.dirty, before.document.dirty);
            println!(
                "{}",
                json!({"kind":"controller_lock","profile":before.profile,"scenario":scenario,
                "pieces":before.pieces.len(),"origins_per_repetition":before.pieces.len(),
                "inspection_wait":stats(waits),"inspection_guard_held":stats(held),
                "inspection_call_with_lifecycle_checks":stats(calls),"inspection_serialization_outside_guard":stats(encode),
                "total_inspection_pretty_utf8_bytes":bytes,"total_inspection_choices":choices,
                "slowest_origin":slowest.1,"slowest_origin_choices":slowest.2,"slowest_origin_pretty_bytes":slowest.3,
                "snapshot_wait":stats(snapshot_wait),"snapshot_guard_held":stats(snapshot_held),
                "snapshot_serialization_outside_guard":stats(snapshot_encode),"snapshot_pretty_utf8_bytes":snapshot_bytes,
                "contention_sample":{"worker_busy_at_attempt":worker_busy_at_attempt,
                    "worker_guard_held_ns_includes_ready_send":worker_held_ns,"worker_choices":worker_choices,
                    "main_wait_ns":contended.wait_ns,"main_guard_held_ns":contended.held_ns,
                    "main_choices":contended.result.choices.len()},
                "accepted_token_context_dirty_unchanged":true})
            );
        }
    }
}
