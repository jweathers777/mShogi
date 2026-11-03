//! Human Chu adapter. Native Game alone owns rules, history and admission.
use crate::{
    record_io,
    session::{MAX_LINE_BYTES, Next},
};
use mshogi_core::{
    chu::{self as c, notation},
    native::CanonicalRecord,
};
use std::fmt::Write as _;

const MAX_DIAGNOSTIC_DEPTH: u8 = 2;
const MAX_DIAGNOSTIC_VISITS: u64 = 100_000;
const HELP: &str = "Untimed Chu Shogi diagnostic console (not USI/SFEN):
  position startpos | position record-file <path> | position record-json <JSON>
  show | status                    Board, native royal status and exact history
  legal                            Complete history-permitted native actions
  move <Chu-action>...              Atomic batch, complete routes only
  resign sente|gote                 Actual resignation, independent of board turn
  offer-draw sente|gote             Actual draw offer; not an automatic result
  accept-draw sente|gote | decline-draw sente|gote
  stop cancelled|resource|engine|work  Recorded incomplete/unscored annotation
  undo [n] | history               Exact native events and event reversal
  record print | record save <path> | record load-file <path> | record load-json <JSON>
  perft <0..2> [visits] | divide <1..2> [visits]  Bounded local diagnostics
  analyze [depth nodes]             Read-only bounded history-aware search
  help | quit
Chu actions: Ln6fx6ex6d ordered double capture, Ln6fx6e-6f capture-return,
Ln6f-5e-6f quiet-return, P6e-6d^ promotion, P6e-6d no flip. Ki+ means an
already promoted original Kirin, not an original Lion. Each landing is exact.
Board: lowercase s/g owner prefix and original piece code, no colon or hands.
Perft/divide ignores history and conclusions. Search uses native full history.
Saves require a new path; strict canonical records preserve complete semantics.";

pub struct Session {
    game: c::Game,
    undo: Vec<c::GameUndo>,
}
impl Session {
    pub fn new() -> Self {
        Self {
            game: c::Game::standard(),
            undo: Vec::new(),
        }
    }
    fn restored(record: &c::GameRecord) -> Result<Self, String> {
        let (game, undo) = record.to_game_with_undo().map_err(diagnostic)?;
        Ok(Self { game, undo })
    }
    pub fn execute(&mut self, line: &str, out: &mut String) -> Result<Next, String> {
        if line.len() > MAX_LINE_BYTES {
            return Err(format!("line exceeds {MAX_LINE_BYTES} bytes"));
        }
        let line = line.trim_matches([' ', '\t', '\r', '\x0b', '\x0c']);
        if line.is_empty() || line.starts_with('#') {
            return Ok(Next::Continue);
        }
        let (command, rest) = line
            .split_once(|c: char| c.is_ascii_whitespace())
            .map_or((line, ""), |(a, b)| (a, b.trim()));
        let args: Vec<_> = rest.split_ascii_whitespace().collect();
        match command {
            "help" => {
                empty(&args)?;
                writeln!(out, "{HELP}").unwrap();
            }
            "quit" => {
                empty(&args)?;
                return Ok(Next::Quit);
            }
            "position" => {
                if rest == "startpos" {
                    *self = Self::new();
                    self.status(out);
                } else if let Some(path) = rest.strip_prefix("record-file ") {
                    self.restore_json(&record_io::read(path.trim())?, out)?;
                } else if let Some(json) = rest.strip_prefix("record-json ") {
                    self.restore_json(json.trim(), out)?;
                } else {
                    return Err("usage: position startpos | position record-file <path> | position record-json <canonical JSON>".into());
                }
            }
            "show" => {
                empty(&args)?;
                self.board(out);

                self.status(out);
            }
            "status" => {
                empty(&args)?;
                self.status(out);
            }
            "legal" => {
                empty(&args)?;
                let actions = self.game.legal_actions();
                writeln!(
                    out,
                    "legal-game {} (complete native choices; pre-fourth repetition history applied)",
                    actions.len()
                )
                .unwrap();
                for action in actions {
                    writeln!(
                        out,
                        "{} {}",
                        notation::format_action(self.game.position(), action.clone())
                            .map_err(diagnostic)?,
                        serde_json::to_string(&action).expect("native action")
                    )
                    .unwrap();
                }
            }
            "move" => {
                if args.is_empty() {
                    return Err("usage: move <Chu-action>...".into());
                }
                let mut candidate =
                    Self::restored(&c::GameRecord::from_game(&self.game).map_err(diagnostic)?)?;
                let mut labels = Vec::new();
                for label in args {
                    let action =
                        notation::parse_local_action(candidate.game.local_position(), label)
                            .map_err(|e| format!("{e}; no actions applied"))?;
                    let undo = candidate
                        .game
                        .submit(c::GameRequest {
                            expected: candidate.game.context(),
                            actor: candidate.game.position().side_to_move(),
                            command: c::GameCommand::Play { action },
                        })
                        .map_err(|e| format!("{}; no actions applied", diagnostic(e)))?;
                    candidate.undo.push(undo);
                    labels.push(label);
                }
                *self = candidate;
                for label in labels {
                    writeln!(out, "played {label}").unwrap();
                }
                self.status(out);
            }
            "resign" => {
                let actor = side(&args)?;
                self.submit(actor, c::GameCommand::Resign)?;
                self.status(out);
            }
            "offer-draw" | "accept-draw" | "decline-draw" => {
                let actor = side(&args)?;
                let command = match command {
                    "offer-draw" => c::GameCommand::OfferDraw,
                    "accept-draw" => c::GameCommand::AcceptDraw,
                    _ => c::GameCommand::DeclineDraw,
                };
                self.submit(actor, command)?;
                self.status(out);
            }
            "stop" => {
                let reason = match args.as_slice() {
                    ["cancelled"] => c::StopReason::Cancelled,
                    ["resource"] => c::StopReason::ResourceLimit,
                    ["engine"] => c::StopReason::EngineFailure,
                    ["work"] => c::StopReason::WorkLimit,
                    _ => return Err("usage: stop cancelled|resource|engine|work".into()),
                };
                self.submit(
                    self.game.position().side_to_move(),
                    c::GameCommand::Stop { reason },
                )?;
                self.status(out);
            }
            "undo" => {
                let count = match args.as_slice() {
                    [] => 1,
                    [n] => decimal(n)
                        .filter(|n| *n > 0)
                        .ok_or("undo requires a positive ASCII count")?,
                    _ => return Err("usage: undo [n]".into()),
                };
                if count > self.undo.len() {
                    return Err(format!(
                        "cannot undo {count}: only {} events",
                        self.undo.len()
                    ));
                }
                for _ in 0..count {
                    self.game
                        .undo(&mut self.undo.pop().expect("count checked"))
                        .map_err(diagnostic)?;
                }
                self.status(out);
            }
            "history" => {
                empty(&args)?;
                self.status(out);
                for event in self.game.events() {
                    let label = match &event.command {
                        c::GameCommand::Play { action } => notation::format_history_action(
                            &self.game.event_prefix(event.index).map_err(diagnostic)?,
                            action.clone(),
                        )
                        .map_err(diagnostic)?,
                        _ => format!("{:?}", event.command),
                    };
                    writeln!(out,"event {} board-ply {} actor {} action {} occurrences {:?} obligation {:?} conclusion {:?} incomplete {:?}",event.index+1,event.board_ply,side_text(event.actor),label,event.occurrences,event.obligations,event.conclusion,event.incomplete).unwrap();
                }
            }
            "record" => {
                use record_io::Operation;
                match record_io::parse(rest, record_io::LegacyLoad::File)? {
                    Operation::Print | Operation::LegacyPrint => {
                        writeln!(out, "record {}", self.record_json()?).unwrap();
                    }
                    Operation::Save(path) => record_io::save(path, &self.record_json()?, out)?,
                    Operation::LoadFile(path) => self.restore_json(&record_io::read(path)?, out)?,
                    Operation::LoadJson(json) => self.restore_json(json, out)?,
                    Operation::LegacyJson(_) => unreachable!("Chu legacy load is file"),
                }
            }
            "perft" | "divide" => {
                let (depth, visits) = match args.as_slice() {
                    [depth] => (*depth, MAX_DIAGNOSTIC_VISITS),
                    [depth, visits] => (
                        *depth,
                        decimal(visits)
                            .filter(|n| *n > 0 && *n <= 100_000)
                            .ok_or("visits must be ASCII 1..100000")?
                            as u64,
                    ),
                    _ => return Err("usage: perft <0..2> [visits] | divide <1..2> [visits]".into()),
                };
                let depth = decimal(depth)
                    .filter(|n| *n <= usize::from(MAX_DIAGNOSTIC_DEPTH))
                    .ok_or("Chu diagnostic depth must be ASCII 0..2")?
                    as u8;
                if command == "divide" && depth == 0 {
                    return Err("divide requires depth1..2".into());
                }
                let mut visited = 0;
                let mut candidate = self.game.local_position().clone();
                if command == "perft" {
                    let leaves = perft(&mut candidate, depth, visits, &mut visited)?;
                    writeln!(out,"perft-local {depth} leaves {leaves} visited {visited} (ignores history/outcomes)").unwrap();
                } else {
                    let mut rows = Vec::new();
                    let mut total = 0u64;
                    for action in candidate.legal_actions() {
                        let (_, mut token) = candidate.apply(&action).map_err(diagnostic)?;
                        let counted = perft(&mut candidate, depth - 1, visits, &mut visited);
                        candidate.undo(&mut token).map_err(diagnostic)?;
                        let leaves = counted?;
                        total = total
                            .checked_add(leaves)
                            .ok_or("diagnostic count overflow")?;
                        rows.push((
                            notation::format_action(self.game.position(), action)
                                .map_err(diagnostic)?,
                            leaves,
                        ));
                    }
                    for (label, leaves) in rows {
                        writeln!(out, "{label} {leaves}").unwrap();
                    }
                    writeln!(
                        out,
                        "total-local {total} visited {visited} (ignores history/outcomes)"
                    )
                    .unwrap();
                }
            }
            "analyze" => {
                use mshogi_search::chu::{self as search, Limits};
                let limits = match args.as_slice() {
                    [] => search::DEFAULT_LIMITS,
                    [depth, nodes] => Limits {
                        depth: decimal(depth)
                            .and_then(|n| n.try_into().ok())
                            .ok_or("invalid ASCII depth")?,
                        nodes: decimal(nodes)
                            .and_then(|n| n.try_into().ok())
                            .ok_or("invalid ASCII nodes")?,
                    },
                    _ => return Err("usage: analyze [depth nodes]".into()),
                };
                let result = search::search(&self.game, limits).map_err(diagnostic)?;
                writeln!(out,"analysis-read-only perspective {:?} status {:?} decision {:?} score {:?} completed-depth {} visited {} pv {:?}",result.perspective,result.status,result.decision,result.score,result.completed_depth,result.nodes,result.principal_variation).unwrap();
            }
            _ => return Err(format!("unknown Chu command '{}'", line.escape_default())),
        }
        Ok(Next::Continue)
    }
    fn submit(&mut self, actor: c::Side, command: c::GameCommand) -> Result<(), String> {
        self.undo.push(
            self.game
                .submit(c::GameRequest {
                    expected: self.game.context(),
                    actor,
                    command,
                })
                .map_err(diagnostic)?,
        );
        Ok(())
    }
    fn record_json(&self) -> Result<String, String> {
        CanonicalRecord::chu(c::GameRecord::from_game(&self.game).map_err(diagnostic)?)
            .to_json()
            .map_err(diagnostic)
    }
    fn restore_json(&mut self, text: &str, out: &mut String) -> Result<(), String> {
        let CanonicalRecord::Chu(record) = CanonicalRecord::from_json(text).map_err(diagnostic)?
        else {
            return Err("record is not Chu; session preserved".into());
        };
        let replacement = Self::restored(&record)?;
        *self = replacement;
        writeln!(out, "loaded complete native Chu record").unwrap();
        self.status(out);
        Ok(())
    }
    fn status(&self, out: &mut String) {
        let root = self.game.position().root();
        writeln!(
            out,
            "profile {} edition {}",
            c::PROFILE_ID,
            c::PROFILE_EDITION
        )
        .unwrap();
        writeln!(out,"root {:?} source {:?} declared-root-plies {} earlier-history-unknown {} reachability-proven {}",root.kind(),root.source(),root.declared_completed_plies(),root.earlier_history_unknown(),root.historical_reachability_proven()).unwrap();
        writeln!(out,"side {} known-events {} known-board-actions {} occurrences {:?} obligation {:?} conclusion {:?} incomplete {:?}",side_text(self.game.position().side_to_move()),self.game.events().len(),self.game.snapshot().board_ply,self.game.occurrences(),self.game.obligations(),self.game.conclusion(),self.game.snapshot().incomplete).unwrap();
        for side in [c::Side::Sente, c::Side::Gote] {
            let royals: Vec<_> = self
                .game
                .royal_squares(side)
                .into_iter()
                .map(|s| {
                    format!(
                        "{}@{}",
                        self.game.position().piece(s).unwrap().face().code(),
                        s
                    )
                })
                .collect();
            writeln!(out, "royals {} {}", side_text(side), royals.join(" ")).unwrap();
        }
        writeln!(
            out,
            "native-status {:?} draw-offer {:?} lion-protection {:?}",
            self.game.status(),
            self.game.snapshot().draw_offer,
            self.game.local_position().protection()
        )
        .unwrap();
        if self.game.snapshot().incomplete.is_some() {
            writeln!(
                out,
                "incomplete work is unscored; no game conclusion inferred"
            )
            .unwrap();
        }
    }
    fn board(&self, out: &mut String) {
        for rank in 1..=12 {
            let cells: Vec<_> = (1..=12)
                .rev()
                .map(|file| {
                    self.game
                        .position()
                        .piece(c::Square::new(file, rank).unwrap())
                        .map_or(".".into(), |p| {
                            format!(
                                "{}{}",
                                if p.side == c::Side::Sente { "s" } else { "g" },
                                p.code()
                            )
                        })
                })
                .collect();
            writeln!(
                out,
                "{} {}",
                char::from(b'a' + rank - 1),
                cells
                    .iter()
                    .map(|s| format!("{s:>4}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
            .unwrap();
        }
        writeln!(
            out,
            "    12   11   10    9    8    7    6    5    4    3    2    1"
        )
        .unwrap();
    }
}
fn diagnostic(error: impl std::fmt::Display) -> String {
    record_io::diagnostic(&error.to_string())
}
fn empty(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        Ok(())
    } else {
        Err("command takes no arguments".into())
    }
}
fn decimal(text: &str) -> Option<usize> {
    (!text.is_empty() && text.bytes().all(|b| b.is_ascii_digit()))
        .then(|| text.parse().ok())
        .flatten()
}
fn side(args: &[&str]) -> Result<c::Side, String> {
    match args {
        ["sente"] => Ok(c::Side::Sente),
        ["gote"] => Ok(c::Side::Gote),
        _ => Err("actor must be sente or gote".into()),
    }
}
fn side_text(side: c::Side) -> &'static str {
    match side {
        c::Side::Sente => "sente",
        c::Side::Gote => "gote",
    }
}

// Local-only diagnostic recursion. The private candidate is always reversed,
// including resource refusal; no logical or runtime live-game state is touched.
fn perft(
    local: &mut c::LocalPosition,
    depth: u8,
    limit: u64,
    visited: &mut u64,
) -> Result<u64, String> {
    if *visited >= limit {
        return Err(format!(
            "local diagnostic visit limit {limit} reached; incomplete, unscored; live session preserved"
        ));
    }
    *visited += 1;
    if depth == 0 {
        return Ok(1);
    }
    let mut total = 0u64;
    for action in local.legal_actions() {
        let (_, mut undo) = local.apply(&action).map_err(diagnostic)?;
        let counted = perft(local, depth - 1, limit, visited);
        local.undo(&mut undo).map_err(diagnostic)?;
        total = total
            .checked_add(counted?)
            .ok_or("diagnostic count overflow")?;
    }
    Ok(total)
}
