//! Native-generated Chu shared consumers, not a finished playable renderer.
#[path = "support/chu.rs"]
mod roots;
use mshogi_core::{
    chu as c,
    native::{CanonicalRecord, NativeSide},
};
use mshogi_desktop_session::{
    DesktopController, SessionOperation, SharedDesktopCommand, SharedSnapshot,
};
fn send(c: &mut DesktopController, operation: SessionOperation) -> SharedSnapshot {
    let game = c.shared_snapshot().game;
    let (_, r) = c
        .shared_command_with_receipt(SharedDesktopCommand {
            expected: c.shared_token(),
            game,
            operation,
        })
        .unwrap();
    let end = std::time::Instant::now() + std::time::Duration::from_secs(20);
    loop {
        let s = c.shared_snapshot();
        if let Some(done) = r.completion() {
            done.unwrap();
            return s;
        }
        assert!(std::time::Instant::now() < end);
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
fn main() {
    let dir = std::path::PathBuf::from(std::env::args_os().nth(1).expect("output directory"));
    std::fs::create_dir_all(&dir).unwrap();
    // Use the very same admitted controller snapshots as the live native adapter.
    for (name, id) in [
        ("routes", "CHU-RETURN-Ln-enemy"),
        ("captures", "CHU-LION-C2-1"),
        ("promotion", "CHU-PROMOTE-DE-YES"),
        ("royals", "CHU-ROYAL-BOTH-COMPOUND"),
    ] {
        let f = roots::fixture(id);
        let g = if name == "routes" {
            roots::contract_game()
        } else {
            roots::root(&f["given"]["root"])
        };
        let mut c = DesktopController::new("/unused/fixture-chu-engine".into()).unwrap();
        let s = send(
            &mut c,
            SessionOperation::Replace {
                record: CanonicalRecord::chu(c::GameRecord::from_game(&g).unwrap()),
                human: NativeSide::Chu(g.position().side_to_move()),
            },
        );
        let write = |name: &str, s: &SharedSnapshot| {
            std::fs::write(
                dir.join(format!("shared-chu-{name}.json")),
                serde_json::to_string_pretty(s).unwrap() + "\n",
            )
            .unwrap()
        };
        write(name, &s);
        if name == "routes" {
            let inspection = c
                .shared_inspection(mshogi_desktop_session::MovementInspectionQuery {
                    expected: c.shared_token(),
                    origin: "11a".into(),
                })
                .unwrap();
            std::fs::write(
                dir.join("shared-chu-inspection.json"),
                serde_json::to_string_pretty(&inspection).unwrap() + "\n",
            )
            .unwrap();
        }
        if name == "captures" {
            let s = send(
                &mut c,
                SessionOperation::Chu {
                    actor: c::Side::Sente,
                    command: c::GameCommand::Play {
                        action: roots::action(&f["given"]["commands"][0]),
                    },
                },
            );
            write("captured", &s);
        }
    }
}
