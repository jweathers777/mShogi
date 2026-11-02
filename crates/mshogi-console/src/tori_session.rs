//! Human Tori adapter. Native Game alone owns rules, history and admission.
use crate::{
    record_io,
    session::{MAX_LINE_BYTES, Next},
};
use mshogi_core::{
    native::CanonicalRecord,
    tori::{self as t, notation},
};
use std::fmt::Write as _;

const MAX_DIAGNOSTIC_DEPTH: u8 = 3;
const MAX_DIAGNOSTIC_VISITS: u64 = 100_000;
const HELP: &str = "Untimed Tori Shogi diagnostic console (not USI/SFEN):
  position startpos                 Standard root with complete known history
  position record-file <path>       Strict canonical standard/custom game
  position record-json <JSON>       Strict inline canonical standard/custom game
  show | hands | status             Board, resources and native game state
  legal                            Complete history-permitted native choices
  move <Tori-action>...             Atomic batch; native Game admits each action
  resign sente|gote                 Native resignation, including out of turn
  stop cancelled|resource|engine|work  Recorded incomplete/unscored reason
  undo [n]                         Exact event undo, including Stop/resignation
  history                          Native event ordering, conclusions/obligations
  record print                     Complete canonical envelope, not display text
  record save <path>                Atomic new-file save; never overwrite
  record load-file <path>           Strict bounded replay before replacement
  record load-json <JSON>           Strict inline canonical envelope
  perft <0..3> | divide <1..3> [visits]  Local diagnostics, default100000 visits
  analyze [depth nodes]             Read-only bounded native history-aware search
  help | quit
Tori actions: Sw3d-3c, capture Fa7cx7b=Ea, drop RQ*5e. Source face and
coordinates are exact; =Ea/=Go records compulsory changed identity. LQ/RQ
are distinct. Parsing selects native local choices; Game enforces history.
Board: s/g owner prefixes, no colon; Ph Fa Ea Cr Pt LQ RQ Sw Go in fixed cells.
Perft/divide ignores history and conclusions, never changes the live game.";

pub struct Session {
    game: t::Game,
    undo: Vec<t::GameUndo>,
}
impl Session {
    pub fn new() -> Self {
        Self {
            game: t::Game::standard(),
            undo: Vec::new(),
        }
    }
    fn restored(record: &t::GameRecord) -> Result<Self, String> {
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
                self.hands(out);
                self.status(out);
            }
            "hands" => {
                empty(&args)?;
                self.hands(out);
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
                    "legal-game {} (complete native choices; T06 history applied)",
                    actions.len()
                )
                .unwrap();
                for action in actions {
                    writeln!(
                        out,
                        "{} {}",
                        notation::format_action(self.game.position(), action)
                            .map_err(diagnostic)?,
                        serde_json::to_string(&action).expect("native action")
                    )
                    .unwrap();
                }
            }
            "move" => {
                if args.is_empty() {
                    return Err("usage: move <Tori-action>...".into());
                }
                let mut candidate =
                    Self::restored(&t::GameRecord::from_game(&self.game).map_err(diagnostic)?)?;
                let mut labels = Vec::new();
                for label in args {
                    let action = notation::parse_action(candidate.game.position(), label)
                        .map_err(|e| format!("{e}; no actions applied"))?;
                    let undo = candidate
                        .game
                        .submit(t::GameRequest {
                            expected: candidate.game.context(),
                            actor: candidate.game.position().side_to_move(),
                            command: t::GameCommand::Play { action },
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
                self.submit(actor, t::GameCommand::Resign)?;
                self.status(out);
            }
            "stop" => {
                let reason = match args.as_slice() {
                    ["cancelled"] => t::StopReason::Cancelled,
                    ["resource"] => t::StopReason::ResourceLimit,
                    ["engine"] => t::StopReason::EngineFailure,
                    ["work"] => t::StopReason::WorkLimit,
                    _ => return Err("usage: stop cancelled|resource|engine|work".into()),
                };
                self.submit(
                    self.game.position().side_to_move(),
                    t::GameCommand::Stop { reason },
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
                    let label = match event.command {
                        t::GameCommand::Play { action } => notation::format_history_action(
                            &self.game.event_prefix(event.index).map_err(diagnostic)?,
                            action,
                        )
                        .map_err(diagnostic)?,
                        _ => format!("{:?}", event.command),
                    };
                    writeln!(out,"event {} board-ply {} actor {} action {} occurrences {:?} obligation {:?} conclusion {:?} incomplete {:?}",event.index+1,event.board_ply,side_text(event.actor),label,event.occurrences,event.obligation,event.conclusion,event.incomplete).unwrap();
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
                    Operation::LegacyJson(_) => unreachable!("Tori legacy load is file"),
                }
            }
            "perft" | "divide" => {
                let (depth, visits) = match args.as_slice() {
                    [depth] => (*depth, MAX_DIAGNOSTIC_VISITS),
                    [depth, visits] => (
                        *depth,
                        decimal(visits)
                            .filter(|n| *n > 0 && *n <= t::MAX_PERFT_VISITS as usize)
                            .ok_or("visits must be ASCII 1..1000000")?
                            as u64,
                    ),
                    _ => return Err("usage: perft <0..3> [visits] | divide <1..3> [visits]".into()),
                };
                let depth = decimal(depth)
                    .filter(|n| *n <= usize::from(MAX_DIAGNOSTIC_DEPTH))
                    .ok_or("Tori diagnostic depth must be ASCII 0..3 (divide1..3)")?
                    as u8;
                if command == "perft" {
                    let report = self
                        .game
                        .local_position()
                        .perft(depth, visits)
                        .map_err(diagnostic)?;
                    writeln!(
                        out,
                        "perft-local {} leaves {} visited {} (ignores history/outcomes)",
                        depth, report.leaves, report.visited
                    )
                    .unwrap();
                } else {
                    let report = self
                        .game
                        .local_position()
                        .divide(depth, visits)
                        .map_err(diagnostic)?;
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
                        "total-local {} visited {} (ignores history/outcomes)",
                        report.total.leaves, report.total.visited
                    )
                    .unwrap();
                }
            }
            "analyze" => {
                use mshogi_search::tori::{self as search, Limits};
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
            _ => return Err(format!("unknown Tori command '{}'", line.escape_default())),
        }
        Ok(Next::Continue)
    }
    fn submit(&mut self, actor: t::Side, command: t::GameCommand) -> Result<(), String> {
        self.undo.push(
            self.game
                .submit(t::GameRequest {
                    expected: self.game.context(),
                    actor,
                    command,
                })
                .map_err(diagnostic)?,
        );
        Ok(())
    }
    fn record_json(&self) -> Result<String, String> {
        CanonicalRecord::tori(t::GameRecord::from_game(&self.game).map_err(diagnostic)?)
            .to_json()
            .map_err(diagnostic)
    }
    fn restore_json(&mut self, text: &str, out: &mut String) -> Result<(), String> {
        let CanonicalRecord::Tori(record) = CanonicalRecord::from_json(text).map_err(diagnostic)?
        else {
            return Err("record is not Tori; session preserved".into());
        };
        let replacement = Self::restored(&record)?;
        *self = replacement;
        writeln!(out, "loaded complete native Tori record").unwrap();
        self.status(out);
        Ok(())
    }
    fn status(&self, out: &mut String) {
        let root = self.game.position().root();
        writeln!(
            out,
            "profile {} edition {}",
            t::PROFILE_ID,
            t::PROFILE_EDITION
        )
        .unwrap();
        writeln!(out,"root {:?} source {:?} declared-root-plies {} earlier-history-unknown {} reachability-proven {}",root.kind(),root.source(),root.declared_completed_plies(),root.earlier_history_unknown(),root.historical_reachability_proven()).unwrap();
        writeln!(out,"side {} known-events {} known-board-actions {} occurrences {:?} obligation {:?} conclusion {:?} incomplete {:?}",side_text(self.game.position().side_to_move()),self.game.events().len(),self.game.snapshot().board_ply,self.game.occurrences(),self.game.obligation(),self.game.conclusion(),self.game.snapshot().incomplete).unwrap();
        if self.game.snapshot().incomplete.is_some() {
            writeln!(
                out,
                "incomplete work is unscored; no game conclusion inferred"
            )
            .unwrap();
        }
    }
    fn hands(&self, out: &mut String) {
        for side in [t::Side::Sente, t::Side::Gote] {
            let counts: Vec<_> = t::Face::HANDS
                .into_iter()
                .map(|f| format!("{}={}", f.code(), self.game.position().hand(side).count(f)))
                .collect();
            writeln!(out, "hands {} {}", side_text(side), counts.join(" ")).unwrap();
        }
    }
    fn board(&self, out: &mut String) {
        for rank in 1..=7 {
            let cells: Vec<_> = (1..=7)
                .rev()
                .map(|file| {
                    self.game
                        .position()
                        .piece(t::Square::new(file, rank).unwrap())
                        .map_or(".".into(), |p| {
                            format!(
                                "{}{}",
                                if p.side == t::Side::Sente { "s" } else { "g" },
                                p.face.code()
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
                    .map(|s| format!("{s:>3}"))
                    .collect::<Vec<_>>()
                    .join(" ")
            )
            .unwrap();
        }
        writeln!(out, "    7   6   5   4   3   2   1").unwrap();
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
fn side(args: &[&str]) -> Result<t::Side, String> {
    match args {
        ["sente"] => Ok(t::Side::Sente),
        ["gote"] => Ok(t::Side::Gote),
        _ => Err("actor must be sente or gote".into()),
    }
}
fn side_text(side: t::Side) -> &'static str {
    match side {
        t::Side::Sente => "sente",
        t::Side::Gote => "gote",
    }
}
