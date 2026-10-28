//! Explicit human console adapter; native game owns all rules and admission.
use crate::{
    record_io,
    session::{MAX_LINE_BYTES, Next},
};
use mshogi_core::{native::CanonicalRecord, shogi::*};
use std::fmt::Write as _;

pub const MAX_DIAGNOSTIC_DEPTH: u32 = 2;
const HELP: &str = "Untimed shogi diagnostic console (not USI protocol):
  position startpos                  Complete standard-root history
  position sfen full|reduced <SFEN>   New known-history segment; supplied counter
  sfen | show | hands | status        Inspect native state and conclusions
  legal                              Complete local USI choices + readable labels
  move <USI-action>...                Native actions, all or none
  eligibility sente|gote              Harmless advisory declaration query
  declare sente|gote                  Formal declaration; false declaration loses
  resign sente|gote                   Native resignation (not restricted to turn)
  undo [n]                           Reverse n events, including terminal commands
  history                            Root provenance, complete event/check history
  record print                       Complete canonical envelope (not SFEN alone)
  record save <path>                  Atomic new-file save; never overwrite
  record load-file <path>             Bounded strict load/replay before replacement
  record load-json <JSON>             Strict inline canonical envelope
  record | record load <path>         Compatibility aliases for print/load-file
  perft <0..2> | divide <1..2>         Position-only local legality; ignores outcomes
  help | quit
USI actions: 7g7f, 5c5b+, P*5e. Readable long notation: S:Pawn[7g->7f;keep],
S:Pawn[5c->5b;promote], S:Pawn[hand->5e;drop]. Always include source; keep
retains existing promoted identity. Readable labels are output, USI is input.
Compact board: uppercase = Sente, lowercase = Gote; K R B G S N L P.
Prefix + marks promotion (+P/+p); two-character cells keep piece/file alignment.";

pub struct Session {
    game: Game,
    undo: Vec<GameUndo>,
}
impl Session {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            undo: Vec::new(),
        }
    }
    fn restored(game: Game) -> Result<Self, String> {
        let mut replay = Game::from_root(game.root().clone());
        let mut undo = Vec::new();
        for event in game.events() {
            undo.push(
                replay
                    .submit(GameRequest {
                        expected: replay.context(),
                        actor: event.actor,
                        command: event.command,
                    })
                    .map_err(|e| format!("native replay failed: {e:?}"))?,
            );
        }
        if replay != game {
            return Err("native replay disagrees with complete recorded game".into());
        }
        Ok(Self { game: replay, undo })
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
                let game = if rest == "startpos" {
                    Game::new()
                } else {
                    let Some(("sfen", rest)) = rest.split_once(' ') else {
                        return Err(
                            "usage: position startpos | position sfen full|reduced <SFEN>".into(),
                        );
                    };
                    let Some((category, text)) = rest.trim().split_once(' ') else {
                        return Err("SFEN requires full|reduced material category".into());
                    };
                    let category = match category {
                        "full" => RootCategory::FullMaterialDiagram,
                        "reduced" => RootCategory::ReducedMaterialDiagram,
                        _ => return Err("SFEN requires full|reduced material category".into()),
                    };
                    let position = Position::from_sfen(text.trim()).map_err(|e| e.to_string())?;
                    Game::from_root(
                        GameRoot::import(position, category, CounterSource::Supplied)
                            .map_err(|e| format!("native root rejected: {e:?}"))?,
                    )
                };
                *self = Self::restored(game)?;
                self.status(out);
                writeln!(
                    out,
                    "history-segment earlier-history {}",
                    if self.game.root().has_complete_initial_history() {
                        "complete"
                    } else {
                        "unknown"
                    }
                )
                .unwrap();
            }
            "sfen" => {
                empty(&args)?;
                writeln!(out, "sfen {}", self.game.position().to_sfen()).unwrap();
            }
            "show" => {
                empty(&args)?;
                compact_board(self.game.position(), out);
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
                let actions = self.game.position().legal_actions();
                writeln!(
                    out,
                    "legal-local {} (game admission also checks conclusion)",
                    actions.len()
                )
                .unwrap();
                for action in actions {
                    writeln!(out, "{action} {}", readable(self.game.position(), action)?).unwrap();
                }
            }
            "move" => {
                if args.is_empty() {
                    return Err("usage: move <USI-action>...".into());
                }
                let mut candidate = self.game.clone();
                let mut undo = Vec::new();
                let mut labels = Vec::new();
                for text in args {
                    let action: Action =
                        text.parse().map_err(|e: ActionParseError| e.to_string())?;
                    let label = readable(candidate.position(), action)?;
                    undo.push(
                        candidate
                            .submit(GameRequest {
                                expected: candidate.context(),
                                actor: candidate.position().side_to_move(),
                                command: GameCommand::Play(action),
                            })
                            .map_err(|e| {
                                format!("native action rejected: {e:?}; no actions applied")
                            })?,
                    );
                    labels.push(format!("played {action} {label}"));
                }
                self.game = candidate;
                self.undo.extend(undo);
                for label in labels {
                    writeln!(out, "{label}").unwrap();
                }
                self.status(out);
            }
            "eligibility" | "declare" | "resign" => {
                let actor = side(&args)?;
                if command == "eligibility" {
                    let evidence = self.game.declaration_eligibility(actor);
                    writeln!(
                        out,
                        "eligibility {} {} {evidence:?} (advisory; no command submitted)",
                        side_text(actor),
                        evidence.eligible()
                    )
                    .unwrap();
                } else {
                    let command = if command == "declare" {
                        GameCommand::DeclareEnteringKing
                    } else {
                        GameCommand::Resign
                    };
                    let undo = self
                        .game
                        .submit(GameRequest {
                            expected: self.game.context(),
                            actor,
                            command,
                        })
                        .map_err(|e| format!("native command rejected: {e:?}"))?;
                    self.undo.push(undo);
                    self.status(out);
                }
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
                        .undo(self.undo.pop().expect("count checked"))
                        .map_err(|e| format!("native undo failed: {e:?}"))?;
                }
                self.status(out);
            }
            "history" => {
                empty(&args)?;
                writeln!(out, "profile {PROFILE_ID} edition {PROFILE_EDITION}").unwrap();
                writeln!(
                    out,
                    "root {:?} counter {:?} complete-initial-history {} root-sfen {}",
                    self.game.root().category(),
                    self.game.root().counter_source(),
                    self.game.root().has_complete_initial_history(),
                    self.game.root().position().to_sfen()
                )
                .unwrap();
                writeln!(
                    out,
                    "events {} known-board-actions {} occurrences {:?}",
                    self.game.events().len(),
                    CanonicalRecord::shogi(GameRecord::from_game(&self.game)).known_board_actions(),
                    self.game.occurrences()
                )
                .unwrap();
                for event in self.game.events() {
                    writeln!(out,"event {} actor {} command {:?} check {:?} declaration {:?} conclusion {:?} after {}",event.index,side_text(event.actor),event.command,event.gave_check,event.declaration,event.conclusion,event.after_sfen).unwrap();
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
                    Operation::LegacyJson(_) => unreachable!("shogi legacy load is a file"),
                }
            }
            "perft" | "divide" => {
                let [n] = args.as_slice() else {
                    return Err("diagnostic requires one depth".into());
                };
                let depth = decimal(n)
                    .filter(|n| *n <= MAX_DIAGNOSTIC_DEPTH as usize)
                    .ok_or("shogi diagnostic depth must be ASCII 0..2 (divide 1..2)")?
                    as u32;
                if command == "perft" {
                    let nodes = self
                        .game
                        .position()
                        .perft(depth)
                        .map_err(|e| e.to_string())?;
                    writeln!(out, "perft-local {depth} {nodes}").unwrap();
                } else {
                    let entries = self
                        .game
                        .position()
                        .divide(depth)
                        .map_err(|e| e.to_string())?;
                    let total: u64 = entries.iter().map(|e| e.nodes).sum();
                    for entry in entries {
                        writeln!(out, "{} {}", entry.action, entry.nodes).unwrap();
                    }
                    writeln!(out, "total-local {total}").unwrap();
                }
            }
            _ => return Err(format!("unknown shogi command '{}'", line.escape_default())),
        }
        Ok(Next::Continue)
    }
    fn record_json(&self) -> Result<String, String> {
        CanonicalRecord::shogi(GameRecord::from_game(&self.game))
            .to_json()
            .map_err(|e| record_io::diagnostic(&e.to_string()))
    }
    fn restore_json(&mut self, json: &str, out: &mut String) -> Result<(), String> {
        let CanonicalRecord::Shogi(record) =
            CanonicalRecord::from_json(json).map_err(|e| record_io::diagnostic(&e.to_string()))?
        else {
            return Err("record is not shogi; session preserved".into());
        };
        let replacement = Self::restored(
            record
                .to_game()
                .map_err(|e| record_io::diagnostic(&e.to_string()))?,
        )?;
        *self = replacement;
        writeln!(out, "loaded complete native record").unwrap();
        self.status(out);
        Ok(())
    }
    fn status(&self, out: &mut String) {
        writeln!(out, "sfen {}", self.game.position().to_sfen()).unwrap();
        writeln!(
            out,
            "side {} declared-completed {} known-events {} conclusion {:?}",
            side_text(self.game.position().side_to_move()),
            self.game.position().completed_actions(),
            self.game.events().len(),
            self.game.conclusion()
        )
        .unwrap();
    }
    fn hands(&self, out: &mut String) {
        for side in [Side::Sente, Side::Gote] {
            let hand = self.game.position().hand(side);
            let kinds: Vec<_> = PieceKind::HAND_KINDS
                .into_iter()
                .map(|kind| format!("{}:{}", kind_text(kind), hand.count(kind)))
                .collect();
            writeln!(out, "hands {} {}", side_text(side), kinds.join(" ")).unwrap();
        }
    }
}
fn empty(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        Ok(())
    } else {
        Err("command takes no arguments".into())
    }
}
fn decimal(s: &str) -> Option<usize> {
    (!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))
        .then(|| s.parse().ok())
        .flatten()
}
fn side(args: &[&str]) -> Result<Side, String> {
    match args {
        ["sente"] => Ok(Side::Sente),
        ["gote"] => Ok(Side::Gote),
        _ => Err("actor must be sente or gote".into()),
    }
}
fn side_text(side: Side) -> &'static str {
    match side {
        Side::Sente => "sente",
        Side::Gote => "gote",
    }
}
fn kind_text(kind: PieceKind) -> &'static str {
    match kind {
        PieceKind::King => "King",
        PieceKind::Rook => "Rook",
        PieceKind::Bishop => "Bishop",
        PieceKind::Gold => "Gold",
        PieceKind::Silver => "Silver",
        PieceKind::Knight => "Knight",
        PieceKind::Lance => "Lance",
        PieceKind::Pawn => "Pawn",
    }
}
fn piece_text(piece: Piece) -> String {
    format!(
        "{}:{}{}",
        if piece.side == Side::Sente { "S" } else { "G" },
        if piece.promoted { "+" } else { "" },
        kind_text(piece.kind)
    )
}
fn readable(position: &Position, action: Action) -> Result<String, String> {
    match action {
        Action::Move { from, to, promote } => {
            let piece = position.piece(from).ok_or("empty source")?;
            Ok(format!(
                "{}[{from}->{to};{}]",
                piece_text(piece),
                if promote { "promote" } else { "keep" }
            ))
        }
        Action::Drop { kind, to } => Ok(format!(
            "{}[hand->{to};drop]",
            piece_text(Piece {
                side: position.side_to_move(),
                kind,
                promoted: false
            })
        )),
    }
}
/// Human board tokens are deliberately independent of long action labels.
fn compact_board(position: &Position, out: &mut String) {
    for rank in 1..=9 {
        let cells: Vec<_> = (1..=9)
            .rev()
            .map(|file| {
                position
                    .piece(Square::new(file, rank).unwrap())
                    .map_or(".".into(), |piece| {
                        let code = match piece.kind {
                            PieceKind::King => 'K',
                            PieceKind::Rook => 'R',
                            PieceKind::Bishop => 'B',
                            PieceKind::Gold => 'G',
                            PieceKind::Silver => 'S',
                            PieceKind::Knight => 'N',
                            PieceKind::Lance => 'L',
                            PieceKind::Pawn => 'P',
                        };
                        let code = if piece.side == Side::Sente {
                            code
                        } else {
                            code.to_ascii_lowercase()
                        };
                        format!("{}{code}", if piece.promoted { "+" } else { "" })
                    })
            })
            .collect();
        let row = cells
            .iter()
            .map(|token| format!("{token:>2}"))
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(out, "{} {}", char::from(b'a' + rank - 1), row.trim_end()).unwrap();
    }
    writeln!(out, "   9  8  7  6  5  4  3  2  1").unwrap();
}
