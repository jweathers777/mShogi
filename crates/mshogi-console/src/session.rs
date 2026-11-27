//! Human-oriented line command session over the chess rules library.
//!
//! This is a diagnostic console, not a machine protocol: it is not UCI and its
//! output is meant for people and simple scripts. All rules come from
//! `mshogi-core`; this module only parses text and formats results.

use crate::{
    diagnostics, record_io,
    shell::{HumanSession, diagnostic, empty as no_args, number, separator},
};
use mshogi_core::game::CanonicalRecord;
use std::fmt::Write as _;

use mshogi_core::chess::{
    Action, ClaimPolicy, Color, Game, HistoryOrigin, PieceKind, Square, sort_divide_actions,
};

/// Maximum perft and divide depth in the Chess shell.
pub const MAX_DIAGNOSTIC_DEPTH: u32 = 4;

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
  record print             Print the shared canonical envelope
  record save <path>       Atomic new-file envelope save; never overwrite
  record load-file <path>  Bounded strict envelope load before replacement
  record load-json <JSON>  Strict inline canonical envelope
  status                   Print quiet, check, checkmate or stalemate
  perft <depth> [visits]   Count legal action sequences of that depth
  divide <depth> [visits]  Per-root-action perft counts and total
  help                     Show this help
  quit                     End the session
Actions are lowercase coordinates: e2e4, e7e8q (q/r/b/n promotion),
castling as the king's move (e1g1), en passant as the pawn's move.";

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
}
impl HumanSession for Session {
    fn ascii_commands_only(&self) -> bool {
        true
    }

    fn help(&self) -> &'static str {
        SHELL_HELP
    }
    fn record(&self) -> Result<CanonicalRecord, String> {
        Ok(CanonicalRecord::chess(self.game.record()))
    }
    fn command(
        &mut self,
        command: &str,
        rest: &str,
        args: &[&str],
        out: &mut String,
    ) -> Result<(), String> {
        match command {
            "position" => self.load(rest, out)?,
            "fen" => {
                no_args(command, args)?;
                writeln!(out, "fen {}", self.game.position()).unwrap();
            }
            "show" => {
                no_args(command, args)?;
                self.show(out);
            }
            "legal" => {
                no_args(command, args)?;
                let mut legal = self.game.position().legal_actions();
                sort_divide_actions(&mut legal);
                let texts: Vec<String> = legal.into_iter().map(action_text).collect();
                if texts.is_empty() {
                    writeln!(out, "legal 0").unwrap();
                } else {
                    writeln!(out, "legal {} {}", texts.len(), texts.join(" ")).unwrap();
                }
            }
            "move" => self.apply(args, out)?,
            "undo" => self.undo(args, out)?,
            "history" => {
                no_args(command, args)?;
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
            "status" => {
                no_args(command, args)?;
                writeln!(out, "status {}", self.game.position().classify()).unwrap();
            }
            "perft" | "divide" => diagnostics::execute(
                self.game.position(),
                command,
                args,
                MAX_DIAGNOSTIC_DEPTH,
                diagnostics::Style::Chess,
                out,
                |action| Ok(action_text(action)),
            )?,
            other => {
                return Err(format!(
                    "unknown command '{}'; type 'help'",
                    record_io::diagnostic(other)
                ));
            }
        }
        Ok(())
    }

    fn load_record(&mut self, record: CanonicalRecord, out: &mut String) -> Result<(), String> {
        let CanonicalRecord::Chess(record) = record else {
            return Err("record is not chess; session preserved".into());
        };
        let replay = record.replay_history().map_err(diagnostic)?;
        if !replay.observations.is_empty() {
            return Err(
                "record includes match observations; diagnostic shell cannot restore match state"
                    .into(),
            );
        }
        self.replace(Self { game: replay.game })?;
        writeln!(out, "fen {}", self.game.position()).unwrap();
        Ok(())
    }
}
impl Session {
    fn load(&mut self, rest: &str, out: &mut String) -> Result<(), String> {
        let game = match rest.split_once(separator) {
            _ if rest == "startpos" => Game::new(ClaimPolicy::ExplicitOnly),
            Some(("fen", fen)) => {
                Game::from_fen(fen.trim_matches(separator), ClaimPolicy::ExplicitOnly)
                    .map_err(diagnostic)?
            }
            _ => return Err("usage: position startpos | position fen <FEN>".into()),
        };
        self.replace(Self { game })?;
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
                    record_io::diagnostic(text),
                    index + 1,
                    args.len()
                )
            };
            let action = parse_action(text).map_err(failure)?;
            game.apply(action)
                .map_err(|error| failure(error.to_string()))?;
        }
        self.replace(Self { game })?;
        writeln!(out, "fen {}", self.game.position()).unwrap();
        Ok(())
    }

    fn undo(&mut self, args: &[&str], out: &mut String) -> Result<(), String> {
        let count = match args {
            [] => 1,
            [n] => number::<usize>(n).filter(|&n| n > 0).ok_or_else(|| {
                format!(
                    "invalid undo count '{}'; expected a positive ASCII decimal integer",
                    record_io::diagnostic(n)
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

/// Parse syntax through the shared core adapter; keep human shell diagnostics.
pub fn parse_action(text: &str) -> Result<Action, String> {
    text.parse().map_err(|_| {
        format!(
            "invalid action '{}'; expected e.g. e2e4 or e7e8q",
            record_io::diagnostic(text)
        )
    })
}

pub fn action_text(action: Action) -> String {
    action.to_string()
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
