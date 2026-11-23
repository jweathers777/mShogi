//! Measure canonical record size and native replay on caller-supplied records.
use mshogi_core::native::CanonicalRecord;
use std::{hint::black_box, time::Instant};

fn main() {
    println!("file,bytes,replays,microseconds_per_replay");
    for path in std::env::args().skip(1) {
        let text = std::fs::read_to_string(&path).expect("read record");
        let record = CanonicalRecord::from_json(&text).expect("validate record");
        let text = record.to_json().expect("encode record");
        let start = Instant::now();
        let mut count = 0;
        loop {
            black_box(CanonicalRecord::from_json(black_box(&text)).expect("replay"));
            count += 1;
            if count >= 5 && start.elapsed().as_millis() >= 100 {
                break;
            }
        }
        println!(
            "{},{},{},{:.1}",
            path,
            text.len(),
            count,
            start.elapsed().as_secs_f64() * 1_000_000.0 / f64::from(count)
        );
    }
}
