//! Fixed-seed legal action sequences: exact reversal, serialization round
//! trips, independently restated transition effects, and replay of saved
//! regression sequences.
//!
//! Fast default: every start position, 2 playouts of up to 40 plies.
//! Deeper: `cargo test --release --locked -p mshogi-core --test sequences -- --ignored`.

use std::collections::BTreeSet;

use mshogi_core::{
    CHESS_POSITION_PROFILE_ID,
    chess::{
        Action, ActionError, Classification, Color, Piece, PieceKind, Position, Promotion, Square,
        Undo,
    },
};
use serde_json::Value;

// SplitMix64: a tiny deterministic generator so that seeds reproduce exactly
// on every platform without an external dependency.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

// Test-data adapter only; no production action notation is established here.
fn text(action: Action) -> String {
    let suffix = match action.promotion {
        None => "",
        Some(Promotion::Queen) => "q",
        Some(Promotion::Rook) => "r",
        Some(Promotion::Bishop) => "b",
        Some(Promotion::Knight) => "n",
    };
    format!("{}{}{suffix}", action.from, action.to)
}

fn parse(text: &str) -> Action {
    let bytes = text.as_bytes();
    let square = |f: u8, r: u8| Square::new(f - b'a', r - b'1').unwrap();
    Action {
        from: square(bytes[0], bytes[1]),
        to: square(bytes[2], bytes[3]),
        promotion: bytes.get(4).map(|choice| match choice {
            b'q' => Promotion::Queen,
            b'r' => Promotion::Rook,
            b'b' => Promotion::Bishop,
            b'n' => Promotion::Knight,
            _ => panic!("bad promotion in {text}"),
        }),
    }
}

fn sq(file: u8, rank: u8) -> Square {
    Square::new(file, rank).unwrap()
}

fn all_squares() -> impl Iterator<Item = Square> {
    (0..8).flat_map(|rank| (0..8).map(move |file| sq(file, rank)))
}

fn start_positions() -> Vec<String> {
    let mut fens = BTreeSet::new();
    let corpus: Value =
        serde_json::from_str(include_str!("../../../tests/fixtures/chess/corpus.json")).unwrap();
    for case in corpus["cases"].as_array().unwrap() {
        if case["operation"] == "position" {
            fens.insert(case["input"]["fen"].as_str().unwrap().to_owned());
        }
    }
    for (file, list) in [
        (
            include_str!("../../../tests/fixtures/chess/attacks.json"),
            "cases",
        ),
        (
            include_str!("../../../tests/fixtures/chess/special-actions.json"),
            "positions",
        ),
        (
            include_str!("../../../tests/fixtures/chess/classification.json"),
            "cases",
        ),
        (
            include_str!("../../../tests/fixtures/chess/ordinary-candidates.json"),
            "cases",
        ),
    ] {
        let data: Value = serde_json::from_str(file).unwrap();
        for case in data[list].as_array().unwrap() {
            fens.insert(case["fen"].as_str().unwrap().to_owned());
        }
    }
    // Positions at the u32 counter limit cannot advance (CounterOverflow is
    // covered by the legal_actions suite); play them with ordinary counters.
    fens.into_iter()
        .map(|fen| {
            let fields: Vec<&str> = fen.split(' ').collect();
            let near_limit = |field: &str| field.parse::<u32>().unwrap() > u32::MAX - 1000;
            if near_limit(fields[4]) || near_limit(fields[5]) {
                format!("{} 0 1", fields[..4].join(" "))
            } else {
                fen
            }
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Check every per-position invariant. `Err` describes the first violation.
fn check_position(position: &Position) -> Result<(), String> {
    let fen = position.to_fen();
    let reparsed = Position::from_fen(&fen).map_err(|e| format!("FEN round trip: {e}"))?;
    if &reparsed != position {
        return Err("state reached by transitions differs from its reparsed FEN".into());
    }
    let legal = position.legal_actions();
    let unique: BTreeSet<String> = legal.iter().copied().map(text).collect();
    if unique.len() != legal.len() {
        return Err("duplicate legal actions".into());
    }
    if position.perft(1) != Ok(legal.len() as u64) {
        return Err("perft(1) differs from the legal count".into());
    }
    // A candidate may be rejected only because the moving king would be in
    // check (or crosses an attacked square when castling); any other
    // structural reason would silently remove a legal action.
    for candidate in position.candidates() {
        match position.clone().apply(candidate) {
            Ok(_) | Err(ActionError::CastlingKingAttacked) => {}
            Err(ActionError::IllegalPosition(reason)) => {
                // FenError exposes only a category and message, so the reason
                // text is matched; a structured reason would replace this.
                let reason = reason.to_string();
                if !reason.contains("nonmoving king is attacked")
                    && !reason.contains("kings are adjacent")
                {
                    return Err(format!("candidate {} rejected: {reason}", text(candidate)));
                }
            }
            Err(error) => return Err(format!("candidate {} rejected: {error}", text(candidate))),
        }
    }
    let side = position.side_to_move();
    let king = all_squares()
        .find(|&s| {
            position.piece_at(s)
                == Some(Piece {
                    color: side,
                    kind: PieceKind::King,
                })
        })
        .ok_or("side to move has no king")?;
    let attacked = position.is_square_attacked(king, side.opponent());
    if position.in_check() != attacked {
        return Err("in_check differs from the king-square attack query".into());
    }
    let expected = match (attacked, legal.is_empty()) {
        (false, false) => Classification::Quiet,
        (true, false) => Classification::Check,
        (true, true) => Classification::Checkmate,
        (false, true) => Classification::Stalemate,
    };
    if position.classify() != expected {
        return Err("classification differs from (in_check, has_legal_action)".into());
    }
    Ok(())
}

/// Independently restate the effects of one applied legal action.
fn check_transition(before: &Position, action: Action, after: &Position) -> Result<(), String> {
    let mover = before.piece_at(action.from).ok_or("empty source")?;
    let df = action.to.file() as i8 - action.from.file() as i8;
    let dr = action.to.rank() as i8 - action.from.rank() as i8;
    let castling = mover.kind == PieceKind::King && df.abs() == 2;
    let en_passant =
        mover.kind == PieceKind::Pawn && df != 0 && before.piece_at(action.to).is_none();
    let capture = before.piece_at(action.to).is_some() || en_passant;

    // Board: exactly the documented squares change.
    let mut expected: Vec<Option<Piece>> = all_squares().map(|s| before.piece_at(s)).collect();
    let index = |s: Square| (s.rank() * 8 + s.file()) as usize;
    expected[index(action.from)] = None;
    let placed = match action.promotion {
        None => mover,
        Some(choice) => Piece {
            color: mover.color,
            kind: match choice {
                Promotion::Queen => PieceKind::Queen,
                Promotion::Rook => PieceKind::Rook,
                Promotion::Bishop => PieceKind::Bishop,
                Promotion::Knight => PieceKind::Knight,
            },
        },
    };
    expected[index(action.to)] = Some(placed);
    if en_passant {
        expected[index(sq(action.to.file(), action.from.rank()))] = None;
    }
    if castling {
        let rank = action.from.rank();
        let (rook_from, rook_to) = if df > 0 { (7, 5) } else { (0, 3) };
        expected[index(sq(rook_to, rank))] = expected[index(sq(rook_from, rank))].take();
    }
    for square in all_squares() {
        if after.piece_at(square) != expected[index(square)] {
            return Err(format!("unexpected board change at {square}"));
        }
    }

    if after.side_to_move() != before.side_to_move().opponent() {
        return Err("side to move did not alternate".into());
    }
    let halfmove = if mover.kind == PieceKind::Pawn || capture {
        0
    } else {
        before.halfmove_clock() + 1
    };
    if after.halfmove_clock() != halfmove {
        return Err("halfmove clock".into());
    }
    let fullmove = before.fullmove_number() + u32::from(mover.color == Color::Black);
    if after.fullmove_number() != fullmove {
        return Err("fullmove number".into());
    }
    let target = (mover.kind == PieceKind::Pawn && dr.abs() == 2).then(|| {
        sq(
            action.from.file(),
            (action.from.rank() + action.to.rank()) / 2,
        )
    });
    if after.en_passant_target() != target {
        return Err("raw en passant target".into());
    }
    // Rights: a right survives exactly when it was held, its king did not
    // move, and its home rook square was neither left nor captured on.
    let (b, a) = (before.castling_rights(), after.castling_rights());
    for color in [Color::White, Color::Black] {
        let rank = if color == Color::White { 0 } else { 7 };
        let king_moved = mover.kind == PieceKind::King && mover.color == color;
        for (held, kept, file) in [
            (b.kingside(color), a.kingside(color), 7),
            (b.queenside(color), a.queenside(color), 0),
        ] {
            let corner = sq(file, rank);
            let touched = action.from == corner || action.to == corner;
            if kept != (held && !king_moved && !touched) {
                return Err(format!("castling right for {color:?} on file {file}"));
            }
        }
    }
    // Ordinary actions are accepted identically by the ordinary-only boundary.
    let mut ordinary = before.clone();
    match ordinary.apply_ordinary(action) {
        Ok(_) if !castling && !en_passant => {
            if &ordinary != after {
                return Err("apply_ordinary and apply disagree".into());
            }
        }
        Err(ActionError::CastlingNotOrdinary) if castling => {}
        Err(ActionError::EnPassantNotOrdinary) if en_passant => {}
        other => return Err(format!("apply_ordinary returned {other:?}")),
    }
    Ok(())
}

/// Play `actions` from `start`, checking every invariant, then undo in
/// reverse. On failure return the minimal failing prefix length and reason.
fn replay(start: &Position, actions: &[Action]) -> Result<Position, (usize, String)> {
    let mut position = start.clone();
    let mut history: Vec<(Position, Undo)> = Vec::new();
    check_position(&position).map_err(|e| (0, e))?;
    for (ply, &action) in actions.iter().enumerate() {
        if !position.legal_actions().contains(&action) {
            return Err((ply + 1, format!("{} is not legal", text(action))));
        }
        let before = position.clone();
        let record = position
            .apply(action)
            .map_err(|e| (ply + 1, format!("{}: {e}", text(action))))?;
        check_transition(&before, action, &position).map_err(|e| (ply + 1, e))?;
        check_position(&position).map_err(|e| (ply + 1, e))?;
        history.push((before, record));
    }
    let end = position.clone();
    while let Some((before, record)) = history.pop() {
        let ply = history.len() + 1;
        position
            .undo(&record)
            .map_err(|e| (ply, format!("undo: {e}")))?;
        if position != before {
            return Err((ply, "undo did not restore every field".into()));
        }
    }
    if &position != start {
        return Err((0, "start position not restored".into()));
    }
    Ok(end)
}

fn playout(start: &Position, rng: &mut Rng, plies: usize) -> Vec<Action> {
    let mut position = start.clone();
    let mut actions = Vec::new();
    for _ in 0..plies {
        // Choose from a text-sorted list: seeds stay stable even if the
        // library's (documented as unspecified) legal order changes.
        let mut legal = position.legal_actions();
        if legal.is_empty() {
            break;
        }
        legal.sort_by_key(|&action| text(action));
        let action = legal[rng.below(legal.len())];
        position.apply(action).unwrap();
        actions.push(action);
    }
    actions
}

fn run_seeded(seed: u64, playouts_per_start: usize, plies: usize) -> usize {
    let mut rng = Rng(seed);
    let mut total = 0;
    for fen in start_positions() {
        let start = Position::from_fen(&fen).unwrap();
        for _ in 0..playouts_per_start {
            let actions = playout(&start, &mut rng, plies);
            if let Err((prefix, reason)) = replay(&start, &actions) {
                let record: Vec<String> = actions[..prefix].iter().copied().map(text).collect();
                panic!(
                    "sequence invariant failed: {reason}\nreplay record (add to \
                     tests/fixtures/chess/sequence-regressions.json):\n\
                     {{\"start_fen\": \"{fen}\", \"actions\": {record:?}}}"
                );
            }
            total += actions.len();
        }
    }
    total
}

#[test]
fn fixed_seed_sequences_restore_and_round_trip_every_ply() {
    let plies = run_seeded(0x6D53_686F_6769_0001, 2, 40);
    assert_eq!(
        plies, 6720,
        "the seeded trajectory changed: legal sets differ from the pinned run (a generation or ordering change); run tools/differential_chess.py to locate it"
    );
}

#[test]
#[ignore = "deep sequence run: use --release --ignored"]
fn deep_fixed_seed_sequences() {
    let mut plies = 0;
    for seed in 1..=8 {
        plies += run_seeded(seed, 4, 120);
    }
    println!("deep sequence plies checked: {plies}");
    assert_eq!(
        plies, 316_880,
        "the seeded trajectory changed: legal sets differ from the pinned run (a generation or ordering change); run tools/differential_chess.py to locate it"
    );
}

#[test]
fn saved_regression_sequences_replay_deterministically() {
    let data: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/chess/sequence-regressions.json"
    ))
    .unwrap();
    assert_eq!(data["format"], "mshogi.chess.sequence-regressions.v1");
    assert_eq!(data["schema_version"], 1);
    assert_eq!(data["rules_profile"], CHESS_POSITION_PROFILE_ID);
    let cases = data["cases"].as_array().unwrap();
    assert!(!cases.is_empty());
    for case in cases {
        let id = case["id"].as_str().unwrap();
        let start = Position::from_fen(case["start_fen"].as_str().unwrap()).unwrap();
        let actions: Vec<Action> = case["actions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| parse(a.as_str().unwrap()))
            .collect();
        let end = replay(&start, &actions)
            .unwrap_or_else(|(ply, reason)| panic!("{id}: ply {ply}: {reason}"));
        let Some(expected) = case.get("expected") else {
            continue;
        };
        let mut position = start.clone();
        let after_fens = expected["after_fens"].as_array().unwrap();
        assert_eq!(after_fens.len(), actions.len(), "{id}: after_fens length");
        for (action, fen) in actions.iter().zip(after_fens) {
            position.apply(*action).unwrap();
            assert_eq!(position.to_fen(), fen.as_str().unwrap(), "{id}");
        }
        let mut legal: Vec<String> = end.legal_actions().into_iter().map(text).collect();
        legal.sort();
        let oracle: Vec<&str> = expected["final_legal"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| a.as_str().unwrap())
            .collect();
        assert_eq!(legal, oracle, "{id}: final legal set");
        assert_eq!(
            end.classify().to_string(),
            expected["final_classification"],
            "{id}"
        );
    }
}

#[test]
fn the_checker_rejects_a_corrupted_transition() {
    // Guard against a vacuous checker: a wrong after-state must be reported.
    let start = Position::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1").unwrap();
    let castle = parse("e1g1");
    let mut after = start.clone();
    after.apply(castle).unwrap();
    let mut wrong = start.clone();
    wrong.apply(parse("e1f1")).unwrap();
    assert!(check_transition(&start, castle, &after).is_ok());
    assert!(check_transition(&start, castle, &wrong).is_err());
    assert!(check_transition(&start, parse("e1f1"), &after).is_err());
}
