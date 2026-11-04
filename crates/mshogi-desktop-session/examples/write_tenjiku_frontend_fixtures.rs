//! Actual two-profile native/controller DTOs. Development generator, never bundled.
use mshogi_core::{
    native::{NativeSide, RulesProfile},
    tenjiku as c,
};
use mshogi_desktop_session::{DesktopController, MovementInspectionQuery, SharedSnapshot};
use std::{
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

fn wait(
    controller: &mut DesktopController,
    receipt: mshogi_desktop_session::AdmissionReceipt,
) -> SharedSnapshot {
    let deadline = Instant::now() + Duration::from_secs(20);
    loop {
        let snapshot = controller.shared_snapshot();
        if let Some(result) = receipt.completion() {
            result.unwrap();
            return snapshot;
        }
        assert!(
            Instant::now() < deadline,
            "native fixture admission deadline"
        );
        std::thread::sleep(Duration::from_millis(1));
    }
}
fn write(dir: &Path, name: &str, value: &impl serde::Serialize) {
    std::fs::write(
        dir.join(format!("shared-tenjiku-{name}.json")),
        serde_json::to_string_pretty(value).unwrap() + "\n",
    )
    .unwrap();
}
fn measure(controller: &mut DesktopController, profile: &str, scenario: &str) {
    let begin = Instant::now();
    let snapshot = controller.shared_snapshot();
    let snapshot_us = begin.elapsed().as_micros();
    let begin = Instant::now();
    let encoded = serde_json::to_vec_pretty(&snapshot).unwrap();
    let encode_us = begin.elapsed().as_micros();
    let mut inspection_us = 0;
    let mut max_us = 0;
    let mut total_choices = 0;
    let mut max_choices = 0;
    let mut total_bytes = 0;
    let mut max_bytes = 0;
    let mut encode_total_us = 0;
    let token = controller.shared_token();
    for piece in &snapshot.pieces {
        let begin = Instant::now();
        let inspected = controller
            .shared_inspection(MovementInspectionQuery {
                expected: token,
                origin: piece.square.clone(),
            })
            .unwrap();
        let elapsed = begin.elapsed().as_micros();
        inspection_us += elapsed;
        max_us = max_us.max(elapsed);
        total_choices += inspected.choices.len();
        max_choices = max_choices.max(inspected.choices.len());
        let begin = Instant::now();
        let bytes = serde_json::to_vec_pretty(&inspected).unwrap().len();
        encode_total_us += begin.elapsed().as_micros();
        total_bytes += bytes;
        max_bytes = max_bytes.max(bytes);
    }
    assert_eq!(
        controller.shared_token(),
        token,
        "inspection changed native authority"
    );
    println!(
        "{}",
        serde_json::json!({"profile":profile,"scenario":scenario,"pieces":snapshot.pieces.len(),"snapshot_choices":snapshot.choices.len(),"snapshot_us":snapshot_us,"snapshot_encoded_bytes":encoded.len(),"snapshot_encode_us":encode_us,"inspection_origins":snapshot.pieces.len(),"inspection_total_us":inspection_us,"inspection_max_us":max_us,"inspection_total_choices":total_choices,"inspection_max_choices":max_choices,"inspection_total_bytes":total_bytes,"inspection_max_bytes":max_bytes,"inspection_encode_total_us":encode_total_us})
    );
}
fn main() {
    if std::env::args().nth(1).as_deref() == Some("internal-search") {
        use std::io::BufRead;
        let mut line = String::new();
        std::io::stdin().lock().read_line(&mut line).unwrap();
        mshogi_engine::serve_once(std::io::Cursor::new(line), std::io::stdout().lock()).unwrap();
        return;
    }
    let dir = PathBuf::from(std::env::args_os().nth(1).expect("output directory"));
    std::fs::create_dir_all(&dir).unwrap();
    // Positive fresh-document provenance comes from actual New on the same owner.
    let mut new = DesktopController::new(std::env::current_exe().unwrap()).unwrap();
    for (label, profile, filename) in [
        ("adams", c::Profile::AdamsModern, "standard"),
        ("muller", c::Profile::MullerCompleted, "standard"),
        ("adams", c::Profile::AdamsModern, "reset"),
    ] {
        let begin = Instant::now();
        let _native = c::Game::standard(profile);
        let native_construct_us = begin.elapsed().as_micros();
        let begin = Instant::now();
        let (_, receipt) = new
            .shared_new_game(
                new.shared_open_token(),
                RulesProfile::from(profile),
                NativeSide::Tenjiku(c::Side::Sente),
            )
            .unwrap();
        let new_call_us = begin.elapsed().as_micros();
        let snapshot = wait(&mut new, receipt);
        let new_admission_install_us = begin.elapsed().as_micros();
        write(&dir, &format!("{label}-{filename}"), &snapshot);
        println!(
            "{}",
            serde_json::json!({"profile":profile.id(),"scenario":filename,"native_construct_us":native_construct_us,"new_call_us":new_call_us,"new_admission_install_us":new_admission_install_us})
        );
        if filename == "standard" {
            measure(&mut new, label, filename);
        }
    }
    let primary = [
        "burn",
        "burned",
        "igui",
        "iguied",
        "promotion",
        "promoted",
        "declined",
        "compound",
        "compound-played",
        "selected-root",
        "unsupported",
        "ending-stopped",
        "hawk",
        "hawk-played",
        "promoted-hawk",
        "area",
        "area-played",
        "ranked-jump",
        "ranked-jump-played",
        "ranked-blocked",
    ];
    let dense = ["dense-lion", "dense-hawk", "dense-fire", "dense-area"];
    // Finish both profiles' consumer DTOs before auxiliary dense measurements.
    for scenarios in [primary.as_slice(), dense.as_slice()] {
        for label in ["adams", "muller"] {
            for &scenario in scenarios {
                let begin = Instant::now();
                let mut controller = DesktopController::verification(
                    std::env::current_exe().unwrap(),
                    &format!("tenjiku-{label}-{scenario}"),
                )
                .unwrap();
                let construct_admission_install_us = begin.elapsed().as_micros();
                let snapshot = controller.shared_snapshot();
                assert!(!snapshot.capabilities.engine_turn && !snapshot.capabilities.move_analysis);
                if scenario == "unsupported" {
                    assert_eq!(snapshot.observations.len(), 1);
                    let observation = &snapshot.observations[0].observation;
                    assert!(
                        matches!(&observation.result,mshogi_engine::ExecutionResult::Failure{failure} if failure.kind == mshogi_engine::FailureKind::UnsupportedMode)
                    );
                }
                write(&dir, &format!("{label}-{scenario}"), &snapshot);
                if scenario.starts_with("dense-")
                    || matches!(
                        scenario,
                        "burn" | "igui" | "compound" | "hawk" | "area" | "ranked-jump"
                    )
                {
                    println!(
                        "{}",
                        serde_json::json!({"profile":label,"scenario":scenario,"fixture_construct_admission_install_us":construct_admission_install_us})
                    );
                    measure(&mut controller, label, scenario);
                }
                if scenario == "selected-root" {
                    let inspection = controller
                        .shared_inspection(MovementInspectionQuery {
                            expected: controller.shared_token(),
                            origin: "8h".into(),
                        })
                        .unwrap();
                    write(&dir, &format!("{label}-inspection"), &inspection);
                    assert_eq!(controller.shared_snapshot().context, snapshot.context);
                }
            }
        }
    }
    // Make missing explicit profile selectors fail in this generator as in the host.
    assert!(
        DesktopController::verification(std::env::current_exe().unwrap(), "tenjiku-standard")
            .is_err()
    );
}
