//! Human-oriented line command session over the chess rules library.
//!
//! This is a diagnostic console, not a machine protocol: it is not UCI and its
//! output is meant for people and simple scripts. All rules come from
//! `mshogi-core`; this module only parses text and formats results.

use std::fmt::Write as _;

use mshogi_core::chess::{Action, Color, PieceKind, Position, Promotion, Square, Undo};

/// Maximum bytes before LF, including a possible CR. Enforced while reading.
pub const MAX_LINE_BYTES: usize = 4096;
/// Interactive diagnostics are deliberately bounded; the library remains uncapped.
pub const MAX_DIAGNOSTIC_DEPTH: u32 = 4;

fn separator(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\r' | '\x0b' | '\x0c')
}

fn escaped(text: &str) -> String {
    text.escape_default().to_string()
}

fn decimal(text: &str) -> bool {
    !text.is_empty() && text.bytes().all(|b| b.is_ascii_digit())
}

pub const STARTPOS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub const SHELL_HELP: &str =
    "Commands (one per line; blank lines and lines starting with # are ignored):
  position startpos        Load the standard starting position
  position fen <FEN>       Load a six-field FEN position
  fen                      Print the canonical FEN
  show                     Print the board and every position field
  legal                    List legal actions in divide order
  move <action>...         Apply actions in order; all or none
  undo [n]                 Undo the last n applied actions (default 1)
  status                   Print quiet, check, checkmate or stalemate
  perft <depth>            Count legal action sequences of that depth
  divide <depth>           Per-root-action perft counts and total
  help                     Show this help
  quit                     End the session
Actions are lowercase coordinates: e2e4, e7e8q (q/r/b/n promotion),
castling as the king's move (e1g1), en passant as the pawn's move.";

/// What a command asks the session loop to do next.
pub enum Next {
    Continue,
    Quit,
}

/// Session state: the current position and the undo records applied since
/// the last load, newest last.
pub struct Session {
    position: Position,
    history: Vec<Undo>,
}

impl Session {
    pub fn new() -> Self {
        Self {
            position: Position::from_fen(STARTPOS).expect("standard position is valid"),
            history: Vec::new(),
        }
    }

    /// Run one input line. On error nothing in the session changes.
    pub fn execute(&mut self, line: &str, out: &mut String) -> Result<Next, String> {
        if line.len() > MAX_LINE_BYTES {
            return Err(format!("line exceeds {MAX_LINE_BYTES} bytes"));
        }
        if !line.is_ascii() {
            return Err("input must be ASCII".into());
        }
        let line = line.trim_matches(separator);
        if line.is_empty() || line.starts_with('#') {
            return Ok(Next::Continue);
        }
        let (command, rest) = line
            .split_once(separator)
            .map_or((line, ""), |(command, rest)| {
                (command, rest.trim_matches(separator))
            });
        let args: Vec<&str> = rest.split(separator).filter(|s| !s.is_empty()).collect();
        match command {
            "position" => self.load(rest, out)?,
            "fen" => {
                no_args(command, &args)?;
                writeln!(out, "fen {}", self.position).unwrap();
            }
            "show" => {
                no_args(command, &args)?;
                self.show(out);
            }
            "legal" => {
                no_args(command, &args)?;
                let mut legal = self.position.legal_actions();
                legal.sort_by_key(|action| order_key(*action));
                let texts: Vec<String> = legal.into_iter().map(action_text).collect();
                if texts.is_empty() {
                    writeln!(out, "legal 0").unwrap();
                } else {
                    writeln!(out, "legal {} {}", texts.len(), texts.join(" ")).unwrap();
                }
            }
            "move" => self.apply(&args, out)?,
            "undo" => self.undo(&args, out)?,
            "status" => {
                no_args(command, &args)?;
                writeln!(out, "status {}", self.position.classify()).unwrap();
            }
            "perft" => {
                let depth = depth(command, &args)?;
                let nodes = self.position.perft(depth).map_err(|e| e.to_string())?;
                writeln!(out, "perft {depth} {nodes}").unwrap();
            }
            "divide" => {
                let depth = depth(command, &args)?;
                let entries = self.position.divide(depth).map_err(|e| e.to_string())?;
                let mut total: u64 = 0;
                for entry in entries {
                    writeln!(out, "{} {}", action_text(entry.action), entry.nodes).unwrap();
                    total = total
                        .checked_add(entry.nodes)
                        .ok_or("perft: node count exceeds u64")?;
                }
                writeln!(out, "total {total}").unwrap();
            }
            "help" => {
                no_args(command, &args)?;
                writeln!(out, "{SHELL_HELP}").unwrap();
            }
            "quit" => {
                no_args(command, &args)?;
                return Ok(Next::Quit);
            }
            other => return Err(format!("unknown command '{}'; type 'help'", escaped(other))),
        }
        Ok(Next::Continue)
    }

    fn load(&mut self, rest: &str, out: &mut String) -> Result<(), String> {
        let position = match rest.split_once(separator) {
            _ if rest == "startpos" => Position::from_fen(STARTPOS).unwrap(),
            Some(("fen", fen)) => {
                Position::from_fen(fen.trim_matches(separator)).map_err(|e| e.to_string())?
            }
            _ => return Err("usage: position startpos | position fen <FEN>".into()),
        };
        self.position = position;
        self.history.clear();
        writeln!(out, "fen {}", self.position).unwrap();
        Ok(())
    }

    fn apply(&mut self, args: &[&str], out: &mut String) -> Result<(), String> {
        if args.is_empty() {
            return Err("usage: move <action>...".into());
        }
        // Apply to a copy so that a failure anywhere leaves the session intact.
        let mut position = self.position.clone();
        let mut records = Vec::with_capacity(args.len());
        for (index, text) in args.iter().enumerate() {
            let failure = |error: String| {
                format!(
                    "{} (action {} of {}): {error}; no actions applied",
                    escaped(text),
                    index + 1,
                    args.len()
                )
            };
            let action = parse_action(text).map_err(failure)?;
            let record = position
                .apply(action)
                .map_err(|error| failure(error.to_string()))?;
            records.push(record);
        }
        self.position = position;
        self.history.extend(records);
        writeln!(out, "fen {}", self.position).unwrap();
        Ok(())
    }

    fn undo(&mut self, args: &[&str], out: &mut String) -> Result<(), String> {
        let count = match args {
            [] => 1,
            [n] => decimal(n)
                .then(|| n.parse::<usize>().ok())
                .flatten()
                .filter(|&n| n > 0)
                .ok_or_else(|| {
                    format!(
                        "invalid undo count '{}'; expected a positive ASCII decimal integer",
                        escaped(n)
                    )
                })?,
            _ => return Err("usage: undo [n]".into()),
        };
        if count > self.history.len() {
            return Err(format!(
                "cannot undo {count}: only {} action(s) since the last load",
                self.history.len()
            ));
        }
        let mut position = self.position.clone();
        for record in self.history.iter().rev().take(count) {
            position.undo(record).map_err(|e| e.to_string())?;
        }
        self.position = position;
        self.history.truncate(self.history.len() - count);
        writeln!(out, "fen {}", self.position).unwrap();
        Ok(())
    }

    fn show(&self, out: &mut String) {
        let p = &self.position;
        for rank in (0..8).rev() {
            let mut row = String::new();
            for file in 0..8 {
                let square = Square::new(file, rank).unwrap();
                row.push(
                    p.piece_at(square)
                        .map_or('.', |piece| piece_char(piece.color, piece.kind)),
                );
                if file < 7 {
                    row.push(' ');
                }
            }
            writeln!(out, "{} {row}", rank + 1).unwrap();
        }
        writeln!(out, "  a b c d e f g h").unwrap();
        writeln!(out, "fen {p}").unwrap();
        let side = match p.side_to_move() {
            Color::White => "white",
            Color::Black => "black",
        };
        writeln!(out, "side {side}").unwrap();
        writeln!(out, "castling {}", p.castling_rights()).unwrap();
        let target = p
            .en_passant_target()
            .map_or("-".to_string(), |s| s.to_string());
        writeln!(out, "en-passant {target}").unwrap();
        writeln!(out, "halfmove {}", p.halfmove_clock()).unwrap();
        writeln!(out, "fullmove {}", p.fullmove_number()).unwrap();
        writeln!(out, "status {}", p.classify()).unwrap();
        writeln!(out, "history {}", self.history.len()).unwrap();
    }
}

fn no_args(command: &str, args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        Ok(())
    } else {
        Err(format!("'{command}' takes no arguments"))
    }
}

fn depth(command: &str, args: &[&str]) -> Result<u32, String> {
    match args {
        [depth] => decimal(depth)
            .then(|| depth.parse::<u32>().ok())
            .flatten()
            .filter(|&n| n <= MAX_DIAGNOSTIC_DEPTH)
            .ok_or_else(|| {
                format!(
                    "invalid depth '{}'; expected ASCII decimal from 0 to {MAX_DIAGNOSTIC_DEPTH}",
                    escaped(depth)
                )
            }),
        _ => Err(format!("usage: {command} <depth>")),
    }
}

fn parse_square(text: &[u8]) -> Option<Square> {
    match text {
        [file @ b'a'..=b'h', rank @ b'1'..=b'8'] => Square::new(file - b'a', rank - b'1'),
        _ => None,
    }
}

/// Parse lowercase coordinate notation; legality is decided by the library.
pub fn parse_action(text: &str) -> Result<Action, String> {
    let bytes = text.as_bytes();
    let invalid = || {
        format!(
            "invalid action '{}'; expected e.g. e2e4 or e7e8q",
            escaped(text)
        )
    };
    if !matches!(bytes.len(), 4 | 5) {
        return Err(invalid());
    }
    let from = parse_square(&bytes[0..2]).ok_or_else(invalid)?;
    let to = parse_square(&bytes[2..4]).ok_or_else(invalid)?;
    let promotion = match bytes.get(4) {
        None => None,
        Some(b'q') => Some(Promotion::Queen),
        Some(b'r') => Some(Promotion::Rook),
        Some(b'b') => Some(Promotion::Bishop),
        Some(b'n') => Some(Promotion::Knight),
        Some(_) => return Err(invalid()),
    };
    Ok(Action {
        from,
        to,
        promotion,
    })
}

pub fn action_text(action: Action) -> String {
    let suffix = match action.promotion {
        None => "",
        Some(Promotion::Queen) => "q",
        Some(Promotion::Rook) => "r",
        Some(Promotion::Bishop) => "b",
        Some(Promotion::Knight) => "n",
    };
    format!("{}{}{suffix}", action.from, action.to)
}

// The library's documented divide order: source, destination, promotion.
fn order_key(action: Action) -> (u8, u8, u8) {
    let promotion = match action.promotion {
        None => 0,
        Some(Promotion::Queen) => 1,
        Some(Promotion::Rook) => 2,
        Some(Promotion::Bishop) => 3,
        Some(Promotion::Knight) => 4,
    };
    (
        action.from.rank() * 8 + action.from.file(),
        action.to.rank() * 8 + action.to.file(),
        promotion,
    )
}

fn piece_char(color: Color, kind: PieceKind) -> char {
    let letter = match kind {
        PieceKind::Pawn => 'p',
        PieceKind::Knight => 'n',
        PieceKind::Bishop => 'b',
        PieceKind::Rook => 'r',
        PieceKind::Queen => 'q',
        PieceKind::King => 'k',
    };
    match color {
        Color::White => letter.to_ascii_uppercase(),
        Color::Black => letter,
    }
}
