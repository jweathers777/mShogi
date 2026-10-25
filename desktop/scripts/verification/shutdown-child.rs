use std::{fs, io, thread, time::Duration};
fn main() {
    let mut request = String::new();
    io::stdin().read_line(&mut request).unwrap();
    fs::write(
        "/tmp/mshogi-shutdown-child.pid",
        std::process::id().to_string(),
    )
    .unwrap();
    thread::sleep(Duration::from_secs(30));
}
