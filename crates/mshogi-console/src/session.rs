//! Human-oriented line command session over the chess rules library.
//!
//! This is a diagnostic console, not a machine protocol: it is not UCI and its
//! output is meant for people and simple scripts. All rules come from
//! `mshogi-core`; this module only parses text and formats results.

use std::fmt::Write as _;

use mshogi_core::chess::{
    Action, ClaimPolicy, Color, Game, GameRecord, HistoryOrigin, PieceKind, Promotion, Square,
};

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

pub const SHELL_HELP: &str =
    "Commands (one per line; blank lines and lines starting with # are ignored):
  position startpos        Load the standard starting position
  position fen <FEN>       Load a six-field FEN position
  fen                      Print the canonical FEN
  show                     Print the board and every position field
  legal                    List legal actions in divide order
  move <action>...         Apply actions in order; all or none
  undo [n]                 Undo the last n applied actions (default 1)
  history                  Show origin, policy and current repetition count
  record                   Print the canonical chess history record
  record load <JSON>       Restore a move-only record atomically
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

/// Adapter state delegates every rule-relevant history field to the library.
pub struct Session {
    game: Game,
}

impl Session {
    pub fn new() -> Self {
        Self {
            game: Game::new(ClaimPolicy::ExplicitOnly),
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
                writeln!(out, "fen {}", self.game.position()).unwrap();
            }
            "show" => {
                no_args(command, &args)?;
                self.show(out);
            }
            "legal" => {
                no_args(command, &args)?;
                let mut legal = self.game.position().legal_actions();
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
            "history" => {
                no_args(command, &args)?;
                writeln!(
                    out,
                    "history {} origin {} policy {} repetitions {}",
                    self.game.ply_count(),
                    match self.game.history_origin() {
                        HistoryOrigin::StandardStart => "standard_start",
                        HistoryOrigin::ImportedFen => "imported_fen",
                    },
                    match self.game.claim_policy() {
                        ClaimPolicy::ExplicitOnly => "explicit_only",
                        ClaimPolicy::AutoClaim => "auto_claim",
                    },
                    self.game.current_occurrences()
                )
                .unwrap();
            }
            "record" => self.record(rest, out)?,
            "status" => {
                no_args(command, &args)?;
                writeln!(out, "status {}", self.game.position().classify()).unwrap();
            }
            "perft" => {
                let depth = depth(command, &args)?;
                let nodes = self
                    .game
                    .position()
                    .perft(depth)
                    .map_err(|e| e.to_string())?;
                writeln!(out, "perft {depth} {nodes}").unwrap();
            }
            "divide" => {
                let depth = depth(command, &args)?;
                let entries = self
                    .game
                    .position()
                    .divide(depth)
                    .map_err(|e| e.to_string())?;
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
        let game = match rest.split_once(separator) {
            _ if rest == "startpos" => Game::new(ClaimPolicy::ExplicitOnly),
            Some(("fen", fen)) => {
                Game::from_fen(fen.trim_matches(separator), ClaimPolicy::ExplicitOnly)
                    .map_err(|e| e.to_string())?
            }
            _ => return Err("usage: position startpos | position fen <FEN>".into()),
        };
        self.game = game;
        writeln!(out, "fen {}", self.game.position()).unwrap();
        Ok(())
    }

    fn apply(&mut self, args: &[&str], out: &mut String) -> Result<(), String> {
        if args.is_empty() {
            return Err("usage: move <action>...".into());
        }
        // Apply to a copy so that a failure anywhere leaves the session intact.
        let mut game = self.game.clone();
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
            game.apply(action)
                .map_err(|error| failure(error.to_string()))?;
        }
        self.game = game;
        writeln!(out, "fen {}", self.game.position()).unwrap();
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
        if count > self.game.ply_count() {
            return Err(format!(
                "cannot undo {count}: only {} action(s) since the last load",
                self.game.ply_count()
            ));
        }
        for _ in 0..count {
            self.game
                .undo()
                .expect("count checked against owned history");
        }
        writeln!(out, "fen {}", self.game.position()).unwrap();
        Ok(())
    }

    fn record(&mut self, rest: &str, out: &mut String) -> Result<(), String> {
        if rest.is_empty() {
            writeln!(out, "record {}", self.game.record().to_json()).unwrap();
            return Ok(());
        }
        let Some(("load", json)) = rest.split_once(separator) else {
            return Err("usage: record | record load <JSON>".into());
        };
        let record = GameRecord::from_json(json.trim_matches(separator))
            .map_err(|e| escaped(&e.to_string()))?;
        if record.adjudication_mode() != mshogi_core::chess::AdjudicationMode::M2 {
            return Err(
                "record uses full adjudication; diagnostic shell cannot restore that mode".into(),
            );
        }
        let replay = record
            .replay_history()
            .map_err(|e| escaped(&e.to_string()))?;
        if !replay.observations.is_empty() {
            return Err(
                "record includes match observations; diagnostic shell cannot restore match state"
                    .into(),
            );
        }
        self.game = replay.game;
        writeln!(out, "fen {}", self.game.position()).unwrap();
        Ok(())
    }

    fn show(&self, out: &mut String) {
        let p = self.game.position();
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
        writeln!(out, "history {}", self.game.ply_count()).unwrap();
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

/// Parse syntax through the shared core adapter; keep human shell diagnostics.
pub fn parse_action(text: &str) -> Result<Action, String> {
    text.parse().map_err(|_| {
        format!(
            "invalid action '{}'; expected e.g. e2e4 or e7e8q",
            escaped(text)
        )
    })
}

pub fn action_text(action: Action) -> String {
    action.to_string()
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
