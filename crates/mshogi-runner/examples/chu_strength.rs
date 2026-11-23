//! Chu harness: deterministic opening screening and native tactical proofs.
//! Run expensive generation in Cloud Batch. Artifacts belong under runs/.
//! chu_strength openings OUT COUNT DEPTH
//! chu_strength tactics OUT
//! chu_strength probe SUITE OUT MILLISECONDS
//! chu_strength export REPORT_PATHS OUT EVERY
use mshogi_core::chu::*;
use mshogi_search::{alphabeta::timing::TimeBudget, chu as search};
use serde_json::{Value, json};
use std::{collections::BTreeSet, fs};
#[path = "support/chu_proof.rs"]
mod chu_proof;
#[path = "support/chu_roots.rs"]
mod chu_roots;

fn write(path: &str, value: &Value) {
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .unwrap();
    file.write_all(serde_json::to_string_pretty(value).unwrap().as_bytes())
        .unwrap();
}

fn openings(count: u64, depth: u16) -> Value {
    let mut seen = BTreeSet::new();
    let mut accepted = vec![];
    let mut screened = vec![];
    for seed in 1..=count {
        let mut game = Game::standard();
        chu_roots::quiet_setup(&mut game, seed, 8, true);
        let key = format!("{:?}", game.repetition_key());
        if !seen.insert(key) {
            continue;
        }
        let result = search::search(
            &game,
            search::Limits {
                depth,
                nodes: 500_000,
            },
        )
        .unwrap();
        let pass = result.completed_depth == depth && result.score.is_some_and(|s| s.abs() <= 100);
        screened.push(json!({"seed":seed,"depth":result.completed_depth,"score":result.score,"nodes":result.nodes,"accepted":pass}));
        if pass {
            accepted.push(json!({"id":format!("chu-standard-s{seed}"),"record":GameRecord::from_game(&game).unwrap()}));
        }
        eprintln!(
            "seed {seed}: depth {} score {:?}; {} accepted",
            result.completed_depth,
            result.score,
            accepted.len()
        );
    }
    json!({"name":"chu-openings-v1","profile":PROFILE_ID,
        "provenance":"Project-generated standard Chu roots, eight seeded quiet rear-zone plies, no captures or promotions; xorshift seed 1..COUNT. BSD-LICENSE.",
        "balance_rule":format!("Fixed before screening: distinct repetition positions; depth {depth}, 500000 node cap; completed depth and absolute score <=100 Pawn hundredths"),
        "openings":accepted,"screened":screened})
}

/// Independently labelled construction corpus, not a sample of game difficulty.
/// Vary the two remote royals and the local tactical geometry; mirrored rows
/// stay in the same split. Full-width native terminal proofs supply all answers.
fn tactics() -> Value {
    let mut rows = vec![];
    let mut seen = BTreeSet::new();
    for family in 0..4 {
        for offset in 0..80u8 {
            let x = 4 + offset % 5;
            let y = 4 + (offset / 5) % 5;
            let sk = Square::new(1 + offset % 3, 12).unwrap();
            let gk = Square::new(10 + (offset / 3) % 3, 1).unwrap();
            let mut entries = vec![
                (sk, Piece::new(Side::Sente, Base::King, false).unwrap()),
                (gk, Piece::new(Side::Gote, Base::King, false).unwrap()),
            ];
            let local: Vec<(u8, u8, Side, Base, bool)> = match family {
                // Removing both nonroyals in one action wins by native exhaustion.
                0 => vec![
                    (x, y, Side::Sente, Base::Lion, false),
                    (x, y - 1, Side::Gote, Base::Pawn, false),
                    (x - 1, y - 1, Side::Gote, Base::Rook, false),
                ],
                // Capture-return is among the native winning answers.
                1 => vec![
                    (x, y, Side::Sente, Base::Lion, false),
                    (x, y - 1, Side::Gote, Base::Rook, false),
                ],
                // A non-Pawn intermediate victim permits a defended distance-two Lion capture.
                2 => vec![
                    (x, y, Side::Sente, Base::Lion, false),
                    (x, y - 1, Side::Gote, Base::Rook, false),
                    (x, y - 2, Side::Gote, Base::Lion, false),
                ],
                // Two royals require a complete double capture; no single-royal shortcut.
                _ => {
                    entries.pop();
                    vec![
                        (x, y, Side::Sente, Base::Lion, false),
                        (x, y - 1, Side::Gote, Base::King, false),
                        (x - 1, y - 1, Side::Gote, Base::Elephant, true),
                    ]
                }
            };
            entries.extend(
                local.into_iter().map(|(f, r, s, b, p)| {
                    (Square::new(f, r).unwrap(), Piece::new(s, b, p).unwrap())
                }),
            );
            let game = Game::from_root(
                Position::from_diagram(&entries, Side::Sente, RootMetadata::diagram(0)).unwrap(),
            )
            .unwrap();
            if game.conclusion().is_some() {
                continue;
            }
            let key = format!("{:?}", game.repetition_key());
            if !seen.insert(key) {
                continue;
            }
            let answers = chu_proof::answers(&game, 1, 100_000).expect("complete one-ply proof");
            if answers.is_empty() {
                continue;
            }
            let igui = (family == 1).then(|| Action {
                from: Square::new(x, y).unwrap(),
                route: vec![Square::new(x, y - 1).unwrap(), Square::new(x, y).unwrap()],
                promote: false,
            });
            rows.push(
                json!({"id":format!("chu-f{family}-{offset}"),"family":family,"igui_contract":igui,
                "split":if offset % 5 == 0 {"held-out"} else {"development"},
                "record":GameRecord::from_game(&game).unwrap(),"win_plies":1,"answers":answers}),
            );
        }
    }
    json!({"name":"chu-tactics-v1","profile":PROFILE_ID,"license":"BSD-LICENSE",
        "source":"Constructed native diagrams with varied royal and tactical placements; not claimed reachable from standard play",
        "answer_rule":"All native legal actions concluding an attacker win in one complete turn; exhaustive native admission, no search evaluation",
        "limitation":"Elementary one-turn Lion and royal tactics; not a measure of deep tactical strength. Construction templates are shared across splits; held-out measures new placements only.",
        "split_rule":"Construction offset modulo 5 equals zero is held-out; fixed before probing",
        "positions":rows})
}

/// Score exact igui support separately from best-move success. Ordinary capture
/// remains a valid winning answer, but cannot satisfy this route/effect check.
fn igui_contract(game: &Game, action: &Action) -> bool {
    use mshogi_search::alphabeta::SearchPosition as _;
    if action.route.len() != 2 || action.route[1] != action.from {
        return false;
    }
    let victim = action.route[0];
    let Some(lion) = game.position().piece(action.from) else {
        return false;
    };
    if !lion.is_lion()
        || game
            .position()
            .piece(victim)
            .is_none_or(|p| p.side == lion.side)
    {
        return false;
    }
    let mut position = search::position::ChuPosition::from_game(game);
    let Some(m) = position.find(action) else {
        return false;
    };
    if !position.make(m) {
        return false;
    }
    let mut native = game.fork().unwrap();
    if native
        .submit(GameRequest {
            expected: native.context(),
            actor: lion.side,
            command: GameCommand::Play {
                action: action.clone(),
            },
        })
        .is_err()
    {
        return false;
    }
    native.position().piece(action.from) == Some(lion)
        && native.position().piece(victim).is_none()
        && position.piece(action.from) == Some(lion)
        && position.piece(victim).is_none()
        && position.side_to_move() == lion.side.opponent()
}

fn export(paths: &Value, every: usize) -> String {
    assert!(every > 0);
    let mut lines = String::new();
    for path in paths.as_array().unwrap() {
        let text = fs::read_to_string(path.as_str().unwrap()).unwrap();
        let report = mshogi_runner::experiments::ValidatedReport::from_json(&text).unwrap();
        let digest = mshogi_runner::experiments::digest(&text);
        for (index, result) in report.games.iter().enumerate() {
            let mshogi_core::native::CanonicalRecord::Chu(record) = &result.final_root.record
            else {
                panic!("not Chu")
            };
            let game = record.to_game().unwrap();
            let outcome = game.conclusion().map(|c| match c.winner() {
                Some(Side::Sente) => 1.0,
                Some(Side::Gote) => 0.0,
                None => 0.5,
            });
            let opening = &report.config.openings[result.pairing.opening];
            let mshogi_core::native::CanonicalRecord::Chu(start) = &opening.root.record else {
                panic!("not Chu")
            };
            let mut samples = vec![];
            if let Some(label) = outcome {
                for (ply, event) in game.events()[start.event_count()..]
                    .iter()
                    .filter(|e| matches!(e.command, GameCommand::Play { .. }))
                    .enumerate()
                {
                    if ply % every != 0 {
                        continue;
                    }
                    let snap = game.event_prefix(event.index).unwrap();
                    let local = snap.local_position();
                    let pieces: Vec<_> = local.position().pieces().collect();
                    samples.push(
                        json!({"diagram":{"pieces":pieces,"side":local.position().side_to_move(),
                        "protection":snap.protection()},"sente_outcome":label,"ply":ply}),
                    );
                }
            }
            let row = json!({"game_id":format!("{digest}/{index}"),"source_report_sha256":digest,
                "opening_id":opening.id,"sente_outcome":outcome,"outcome":game.conclusion(),
                "stop":result.stop,"truncated":outcome.is_none(),"samples":samples});
            lines.push_str(&row.to_string());
            lines.push('\n');
        }
    }
    lines
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    match a[1].as_str() {
        "export" => {
            let paths: Value = serde_json::from_str(&fs::read_to_string(&a[2]).unwrap()).unwrap();
            use std::io::Write;
            fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&a[3])
                .unwrap()
                .write_all(export(&paths, a[4].parse().unwrap()).as_bytes())
                .unwrap();
        }
        "tactics" => write(&a[2], &tactics()),
        "openings" => write(
            &a[2],
            &openings(a[3].parse().unwrap(), a[4].parse().unwrap()),
        ),
        "probe" => {
            let suite: Value = serde_json::from_str(&fs::read_to_string(&a[2]).unwrap()).unwrap();
            let ms: u64 = a[4].parse().unwrap();
            let rows: Vec<_> = suite["positions"].as_array().unwrap().iter().map(|row| {
                let record: GameRecord = serde_json::from_value(row["record"].clone()).unwrap();
                let game = record.to_game().unwrap();
                let r = search::search_with_state(&game, search::Limits {depth:64,nodes:search::MAX_NODES},
                    Some(TimeBudget::from_hard_ms(ms)),&mut search::new_engine(),||false).unwrap();
                let action = match r.decision {Some(search::Decision::Choose(a)) => Some(a),_=>None};
                let passed = action.as_ref().is_some_and(|a| row["answers"].as_array().unwrap().contains(&serde_json::to_value(a).unwrap()));
                let contract = row.get("igui_contract").filter(|v| !v.is_null()).map(|v| {
                    let a: Action = serde_json::from_value(v.clone()).unwrap();
                    igui_contract(&game,&a)
                });
                json!({"id":row["id"],"split":row["split"],"passed":passed,"igui_contract_passed":contract,"action":action,"depth":r.completed_depth,"nodes":r.nodes})
            }).collect();
            write(&a[3], &json!({"ms":ms,"rows":rows}));
        }
        _ => panic!("usage: chu_strength openings OUT COUNT DEPTH | probe SUITE OUT MS"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lion_cases_have_complete_native_winning_answers() {
        let suite = tactics();
        let rows = suite["positions"].as_array().unwrap();
        assert!(rows.len() >= 200);
        assert!(rows.iter().any(|r| r["split"] == "held-out"));
        for family in 0..4 {
            let row = rows.iter().find(|r| r["family"] == family).unwrap();
            let record: GameRecord = serde_json::from_value(row["record"].clone()).unwrap();
            let game = record.to_game().unwrap();
            let answers: Vec<Action> = serde_json::from_value(row["answers"].clone()).unwrap();
            assert!(!answers.is_empty());
            if family == 1 {
                let action: Action = serde_json::from_value(row["igui_contract"].clone()).unwrap();
                assert!(igui_contract(&game, &action));
                let mut ordinary = action.clone();
                ordinary.route.pop();
                assert!(!igui_contract(&game, &ordinary));
                assert!(
                    answers.iter().any(|a| a.route.last() == Some(&a.from)),
                    "igui"
                );
            }
            if family == 2 {
                let lion = game
                    .position()
                    .pieces()
                    .find(|(_, p)| p.side == Side::Gote && p.is_lion())
                    .unwrap()
                    .0;
                assert!(
                    !game.legal_actions().iter().any(|a| a.route == vec![lion]),
                    "defended distance-two jump is forbidden"
                );
                assert!(
                    answers
                        .iter()
                        .any(|a| a.route.len() == 2 && a.route[1] == lion),
                    "qualifying intermediate capture permits Lion capture"
                );
            }
            if family == 3 {
                assert!(
                    answers.iter().all(|a| a.route.len() == 2),
                    "two royals need two landings"
                );
            }
        }
    }
}
