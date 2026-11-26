use mshogi_engine::{BridgeOptions, EngineBridge, ExecutionContext, ExecutionResponse};
use std::{
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};
pub fn bridge(path: &str, watchdog: Duration) -> EngineBridge {
    EngineBridge::with_options(
        PathBuf::from(path),
        BridgeOptions {
            watchdog,
            cleanup: Duration::from_secs(1),
        },
    )
    .unwrap()
}
pub fn stop(bridge: &EngineBridge) {
    bridge.shutdown();
    let deadline = Instant::now() + Duration::from_secs(5);
    while !bridge.shutdown_acknowledged() {
        assert!(
            Instant::now() < deadline,
            "engine cleanup shutdown acknowledgement"
        );
        thread::sleep(Duration::from_millis(1));
    }
}
pub fn wait(bridge: &EngineBridge) -> ExecutionResponse {
    wait_for(bridge, Duration::from_secs(20))
}
pub fn wait_for(bridge: &EngineBridge, timeout: Duration) -> ExecutionResponse {
    let deadline = Instant::now() + timeout;
    loop {
        if let Some(response) = bridge.poll() {
            return response;
        }
        assert!(
            Instant::now() < deadline,
            "engine bridge completion deadline"
        );
        thread::sleep(Duration::from_millis(1));
    }
}
pub fn started(prefix: &str, context: &ExecutionContext) -> std::path::PathBuf {
    let marker = std::env::temp_dir().join(format!(
        "{prefix}-started-{}-{}.txt",
        std::process::id(),
        context.request_id
    ));
    let deadline = Instant::now() + Duration::from_secs(5);
    while !marker.is_file() {
        assert!(Instant::now() < deadline, "actual child did not start");
        thread::sleep(Duration::from_millis(1));
    }
    let child: u32 = std::fs::read_to_string(&marker).unwrap().parse().unwrap();
    assert_ne!(child, std::process::id());
    marker
}
