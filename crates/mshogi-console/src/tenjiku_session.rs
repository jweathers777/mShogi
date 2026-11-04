//! Human Tenjiku adapter. Native Game alone owns rules, history and admission.
use crate::{
    record_io,
    session::{MAX_LINE_BYTES, Next},
};
use mshogi_core::{
    native::CanonicalRecord,
    tenjiku::{self as c, notation},
};
use std::fmt::Write as _;

const MAX_DIAGNOSTIC_DEPTH: u8 = c::MAX_PERFT_DEPTH;
const MAX_DIAGNOSTIC_VISITS: u64 = c::MAX_PERFT_VISITS;
const HELP: &str = "Untimed Tenjiku Shogi diagnostic console (not USI/SFEN):
  position startpos | position record-file <path> | position record-json <JSON>
  show | status                    Board, native royal status and exact history
  legal                            Complete history-permitted native actions
  move <Tenjiku-action>...          Atomic batch, complete routes only
  resign sente|gote                 Actual resignation, independent of board turn
  offer-draw sente|gote             Actual draw offer; not an automatic result
  accept-draw sente|gote | decline-draw sente|gote
  stop cancelled|resource|engine|work  Recorded incomplete/unscored annotation
  undo [n] | history               Exact native events and event reversal
  record print | record save <path> | record load-file <path> | record load-json <JSON>
  perft <0..3> [visits] | divide <1..3> [visits]  Bounded full-history diagnostics
  analyze [depth nodes]             Read-only bounded history-aware search
  help | quit
Tenjiku actions: Ln8hx8gx8f ordered captures; Ln8hx8g-8h capture-return;
VGn4h-4g-5g-5f three actual landings; ChS+8h!x9h stationary igui;
P12f-12e^ requests promotion. Original code+ retains original identity.
Burn/death are native derived effects, never input choices. No captured hands.
Perft/divide uses full native history/endings, depth0..3 (divide1..3).
Saves require a new path; strict canonical records preserve the selected profile.";

pub struct Session {
    game: c::Game,
    undo: Vec<c::GameUndo>,
}
impl Session {
    pub fn new(profile: c::Profile) -> Self {
        Self {
            game: c::Game::standard(profile),
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
                    *self = Self::new(self.game.profile());
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
                    "legal-game {} (complete native choices; first recurrence forbidden by native history)",
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
                    return Err("usage: move <Tenjiku-action>...".into());
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
                    writeln!(out,"event {} board-ply {} actor {} action {} pending-bare {:?} conclusion {:?} incomplete {:?} effects {:?}",event.index+1,event.board_ply,side_text(event.actor),label,event.pending_bare,event.conclusion,event.incomplete,event.effects).unwrap();
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
                    Operation::LegacyJson(_) => unreachable!("Tenjiku legacy load is file"),
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
                    _ => return Err("usage: perft <0..3> [visits] | divide <1..3> [visits]".into()),
                };
                let depth = decimal(depth)
                    .filter(|n| *n <= usize::from(MAX_DIAGNOSTIC_DEPTH))
                    .ok_or("Tenjiku diagnostic depth must be ASCII 0..3")?
                    as u8;
                if command == "divide" && depth == 0 {
                    return Err("divide requires depth1..3".into());
                }
                if command == "perft" {
                    let report = self.game.perft(depth, visits).map_err(diagnostic)?;
                    writeln!(
                        out,
                        "perft-game {depth} leaves {} visited {} (full native history/endings)",
                        report.leaves, report.visited
                    )
                    .unwrap();
                } else {
                    let report = self.game.divide(depth, visits).map_err(diagnostic)?;
                    for row in report.entries {
                        writeln!(
                            out,
                            "{} {}",
                            notation::format_action(self.game.position(), row.action)
                                .map_err(diagnostic)?,
                            row.leaves
                        )
                        .unwrap();
                    }
                    writeln!(
                        out,
                        "total-game {} visited {} (full native history/endings)",
                        report.total.leaves, report.total.visited
                    )
                    .unwrap();
                }
            }
            "analyze" => {
                use mshogi_search::tenjiku::{self as search, Limits};
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
            _ => {
                return Err(format!(
                    "unknown Tenjiku command '{}'",
                    line.escape_default()
                ));
            }
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
        CanonicalRecord::tenjiku(c::GameRecord::from_game(&self.game).map_err(diagnostic)?)
            .to_json()
            .map_err(diagnostic)
    }
    fn restore_json(&mut self, text: &str, out: &mut String) -> Result<(), String> {
        let CanonicalRecord::Tenjiku(record) =
            CanonicalRecord::from_json(text).map_err(diagnostic)?
        else {
            return Err("record is not Tenjiku; session preserved".into());
        };
        let replacement = Self::restored(&record)?;
        *self = replacement;
        writeln!(out, "loaded complete native Tenjiku record").unwrap();
        self.status(out);
        Ok(())
    }
    fn status(&self, out: &mut String) {
        let root = self.game.position().root();
        writeln!(
            out,
            "profile {} edition {}",
            self.game.profile().id(),
            self.game.profile().edition()
        )
        .unwrap();
        writeln!(
            out,
            "root {:?} source {:?} declared-root-plies {} earlier-history-unknown {}",
            root.kind(),
            root.source(),
            root.declared_completed_plies(),
            root.earlier_history_unknown()
        )
        .unwrap();
        writeln!(out,"side {} known-events {} known-board-actions {} pending-bare {:?} conclusion {:?} incomplete {:?}",side_text(self.game.position().side_to_move()),self.game.events().len(),self.game.snapshot().board_ply,self.game.snapshot().pending_bare,self.game.conclusion(),self.game.snapshot().incomplete).unwrap();
        for side in [c::Side::Sente, c::Side::Gote] {
            let royals: Vec<_> = self
                .game
                .royals(side)
                .into_iter()
                .map(|(square, piece)| format!("{}@{}", piece.face().code(), square))
                .collect();
            writeln!(out, "royals {} {}", side_text(side), royals.join(" ")).unwrap();
        }
        writeln!(
            out,
            "native-status {:?} draw-offer {:?}",
            self.game.status(),
            self.game.snapshot().draw_offer
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
        for rank in 1..=16 {
            let cells: Vec<_> = (1..=16)
                .rev()
                .map(|file| {
                    self.game
                        .position()
                        .piece(c::Square::new(file, rank).unwrap())
                        .map_or(".".into(), |p| {
                            format!(
                                "{}{}{}",
                                if p.side == c::Side::Sente { "s" } else { "g" },
                                p.base.code(),
                                if p.promoted { "+" } else { "" }
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
                    .map(|s| format!("{s:>5}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
            .unwrap();
        }
        writeln!(
            out,
            "     16    15    14    13    12    11    10     9     8     7     6     5     4     3     2     1"
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
