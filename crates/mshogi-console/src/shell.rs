//! Shared human command loop and atomic native-game sessions.
use crate::{Game, record_io, session};
use mshogi_core::{chu, native::CanonicalRecord, shogi, tenjiku, tori, xiangqi};
use std::{
    fmt::{Display, Write as _},
    io::{self, BufRead, Write},
    process::ExitCode,
};

pub enum Next {
    Continue,
    Quit,
}

pub fn diagnostic(error: impl Display) -> String {
    record_io::diagnostic(&error.to_string())
}

pub fn number<T: std::str::FromStr>(text: &str) -> Option<T> {
    text.bytes()
        .all(|b| b.is_ascii_digit())
        .then(|| text.parse().ok())
        .flatten()
}

pub fn empty(command: &str, args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        Ok(())
    } else {
        Err(format!("'{command}' takes no arguments"))
    }
}

pub fn separator(c: char) -> bool {
    matches!(c, ' ' | '\t' | '\n' | '\r' | '\x0b' | '\x0c')
}

pub fn words(text: &str) -> (&str, &str) {
    text.split_once(separator)
        .map_or((text, ""), |(head, tail)| {
            (head, tail.trim_matches(separator))
        })
}

pub trait HumanSession {
    fn help(&self) -> &'static str;
    fn ascii_commands_only(&self) -> bool {
        false
    }
    fn command(
        &mut self,
        command: &str,
        rest: &str,
        args: &[&str],
        out: &mut String,
    ) -> Result<(), String>;
    fn record(&self) -> Result<CanonicalRecord, String>;
    fn load_record(&mut self, record: CanonicalRecord, out: &mut String) -> Result<(), String>;
    fn record_json(&self) -> Result<String, String> {
        self.record()?.to_json().map_err(diagnostic)
    }

    fn check_export(&self) -> Result<(), String> {
        self.record_json().map(drop)
    }

    fn replace(&mut self, candidate: Self) -> Result<(), String>
    where
        Self: Sized,
    {
        candidate.check_export()?;
        *self = candidate;
        Ok(())
    }

    fn execute(&mut self, line: &str, out: &mut String) -> Result<Next, String> {
        let line = line.trim_matches(separator);
        if line.is_empty() || line.starts_with('#') {
            return Ok(Next::Continue);
        }
        let (command, rest) = words(line);
        if self.ascii_commands_only() && command != "record" && !line.is_ascii() {
            return Err("input must be ASCII".into());
        }
        let args: Vec<_> = rest.split(separator).filter(|s| !s.is_empty()).collect();
        match command {
            "quit" => {
                empty(command, &args)?;
                return Ok(Next::Quit);
            }
            "help" => {
                empty(command, &args)?;
                writeln!(out, "{}", self.help()).unwrap();
            }
            "record" => {
                use record_io::Operation;
                let record = match record_io::parse(rest)? {
                    Operation::Print => {
                        writeln!(out, "record {}", self.record_json()?).unwrap();
                        return Ok(Next::Continue);
                    }
                    Operation::Save(path) => {
                        record_io::save(path, &self.record_json()?, out)?;
                        return Ok(Next::Continue);
                    }
                    Operation::LoadFile(path) => {
                        CanonicalRecord::from_json(&record_io::read(path)?)
                    }
                    Operation::LoadJson(text) => CanonicalRecord::from_json(text),
                }
                .map_err(diagnostic)?;
                self.load_record(record, out)?;
            }
            _ => self.command(command, rest, &args, out)?,
        }
        Ok(Next::Continue)
    }
}

/// Keep undo tokens with the native game instance that issued them.
pub struct Session<G: Variant> {
    game: G,
    undo: Vec<G::Undo>,
}
impl<G: Variant> Session<G> {
    pub fn new(game: G) -> Self {
        Self {
            game,
            undo: Vec::new(),
        }
    }

    fn transaction(
        &mut self,
        out: &mut String,
        apply: impl FnOnce(&mut Self, &mut String) -> Result<(), String>,
    ) -> Result<(), String> {
        // Native undo restores the exact command prefix, including the validity
        // of older undo tokens. Keep the batch private until export succeeds.
        let mark = self.undo.len();
        let mut pending = String::new();
        let result = apply(self, &mut pending).and_then(|()| {
            self.check_export()?;
            if !pending.is_empty() && !pending.ends_with('\n') {
                pending.push('\n');
            }
            self.game.write_status(&mut pending)
        });
        if let Err(error) = result {
            self.undo_to(mark)?;
            return Err(error);
        }
        out.push_str(&pending);
        Ok(())
    }

    fn undo_to(&mut self, mark: usize) -> Result<(), String> {
        while self.undo.len() > mark {
            self.game
                .undo_event(self.undo.pop().expect("checked length"))?;
        }
        Ok(())
    }
}

pub trait Variant: Sized {
    type Undo;
    type Command;
    const NAME: &'static str;
    const HELP: &'static str;
    const MOVE_SYNTAX: &'static str;
    const LOADED: &'static str;
    fn position_command(&self, rest: &str) -> Result<Self, String>;
    fn after_position(&self, out: &mut String) -> Result<(), String> {
        self.write_status(out)
    }
    fn record(&self) -> Result<CanonicalRecord, String>;
    fn restore(record: CanonicalRecord) -> Result<(Self, Vec<Self::Undo>), String>;
    fn write_status(&self, out: &mut String) -> Result<(), String>;
    fn board(&self, out: &mut String);
    fn legal(&self, out: &mut String) -> Result<(), String>;
    fn history(&self, out: &mut String) -> Result<(), String>;
    fn perft(&self, command: &str, args: &[&str], out: &mut String) -> Result<(), String>;
    fn play(&mut self, label: &str, out: &mut String) -> Result<Self::Undo, String>;
    fn actor_command(command: &str) -> Option<Self::Command>;
    fn submit_actor(
        &mut self,
        args: &[&str],
        command: Self::Command,
        undo: &mut Vec<Self::Undo>,
        out: &mut String,
    ) -> Result<(), String>;
    fn undo_event(&mut self, undo: Self::Undo) -> Result<(), String>;
    fn extra(&self, command: &str, args: &[&str], out: &mut String) -> Result<(), String>;
}

impl<G: Variant> HumanSession for Session<G> {
    fn help(&self) -> &'static str {
        G::HELP
    }
    fn record(&self) -> Result<CanonicalRecord, String> {
        self.game.record()
    }
    fn load_record(&mut self, record: CanonicalRecord, out: &mut String) -> Result<(), String> {
        let (game, undo) = G::restore(record)?;
        self.replace(Self { game, undo })?;
        writeln!(out, "{}", G::LOADED).unwrap();
        self.game.write_status(out)
    }
    fn command(
        &mut self,
        command: &str,
        rest: &str,
        args: &[&str],
        out: &mut String,
    ) -> Result<(), String> {
        match command {
            "position" => {
                let candidate = Self::new(self.game.position_command(rest)?);
                self.replace(candidate)?;
                self.game.after_position(out)
            }
            "show" | "status" | "legal" | "history" => {
                empty(command, args)?;
                match command {
                    "show" => {
                        self.game.board(out);
                        self.game.write_status(out)
                    }
                    "status" => self.game.write_status(out),
                    "legal" => self.game.legal(out),
                    _ => self.game.history(out),
                }
            }
            "move" => {
                if args.is_empty() {
                    return Err(format!("usage: move <{}>...", G::MOVE_SYNTAX));
                }
                self.transaction(out, |session, pending| {
                    for label in args {
                        session.undo.push(
                            session
                                .game
                                .play(label, pending)
                                .map_err(|e| format!("{e}; no actions applied"))?,
                        );
                    }
                    Ok(())
                })
            }
            "undo" => {
                let count = match args {
                    [] => 1,
                    [n] => number::<usize>(n)
                        .filter(|&n| n > 0)
                        .ok_or("undo requires a positive ASCII count")?,
                    _ => return Err("usage: undo [n]".into()),
                };
                if count > self.undo.len() {
                    return Err(format!(
                        "cannot undo {count}: only {} events",
                        self.undo.len()
                    ));
                }
                self.undo_to(self.undo.len() - count)?;
                self.game.write_status(out)
            }
            "perft" | "divide" => self.game.perft(command, args, out),
            _ => {
                if let Some(cmd) = G::actor_command(command) {
                    self.transaction(out, |session, pending| {
                        session
                            .game
                            .submit_actor(args, cmd, &mut session.undo, pending)
                    })
                } else {
                    self.game.extra(command, args, out)
                }
            }
        }
    }
}

pub fn unknown<G: Variant>(command: &str) -> String {
    format!("unknown {} command '{}'", G::NAME, diagnostic(command))
}

pub fn startpos<G>(rest: &str, standard: impl FnOnce() -> G) -> Result<G, String> {
    if rest == "startpos" {
        Ok(standard())
    } else {
        Err("usage: position startpos".into())
    }
}

pub fn shogi_board(out: &mut String, size: u8, width: usize, cell: impl Fn(u8, u8) -> String) {
    for rank in 1..=size {
        write!(out, "{}", char::from(b'a' + rank - 1)).unwrap();
        for file in (1..=size).rev() {
            write!(out, " {:>width$}", cell(file, rank)).unwrap();
        }
        writeln!(out).unwrap();
    }
    out.push(' ');
    for file in (1..=size).rev() {
        write!(out, " {file:>width$}").unwrap();
    }
    writeln!(out).unwrap();
}

// The native modules expose distinct request, record and result types with the
// same operations. Keep that wiring here, next to the shared session.
macro_rules! sides {
    ($first:ident => $a:literal, $second:ident => $b:literal) => {
        fn side(args: &[&str]) -> Result<Side, String> {
            match args {
                [$a] => Ok(Side::$first),
                [$b] => Ok(Side::$second),
                _ => Err(concat!("actor must be ", $a, " or ", $b).into()),
            }
        }
        fn side_text(side: Side) -> &'static str {
            match side {
                Side::$first => $a,
                Side::$second => $b,
            }
        }
    };
}
pub(crate) use sides;

macro_rules! draw_command {
    ($command:expr) => {
        match $command {
            "offer-draw" => Some(GameCommand::OfferDraw),
            "accept-draw" => Some(GameCommand::AcceptDraw),
            "decline-draw" => Some(GameCommand::DeclineDraw),
            _ => None,
        }
    };
}
pub(crate) use draw_command;

macro_rules! native_record {
    ($game:ident, $kind:ident) => {
        fn record(&self) -> Result<CanonicalRecord, String> {
            Ok(CanonicalRecord::$game(
                GameRecord::from_game(self).map_err(diagnostic)?,
            ))
        }
        fn restore(record: CanonicalRecord) -> Result<(Self, Vec<GameUndo>), String> {
            let CanonicalRecord::$kind(record) = record else {
                return Err(format!("record is not {}; session preserved", Self::NAME));
            };
            record.to_game_with_undo().map_err(diagnostic)
        }
    };
}
pub(crate) use native_record;

macro_rules! variant_actions {
    ($parse:expr, $rule:literal $(, $effects:ident)?) => {
        fn legal(&self, out: &mut String) -> Result<(), String> {
            let actions = self.legal_actions();
            writeln!(out, "legal-game {} (complete native choices; {})", actions.len(), $rule).unwrap();
            for action in actions {
                let json = serde_json::to_string(&action).expect("native action");
                let label = notation::format_action(self.position(), action).map_err(diagnostic)?;
                writeln!(out, "{label} {json}").unwrap();
            }
            Ok(())
        }

        fn play(&mut self, label: &str, out: &mut String) -> Result<GameUndo, String> {
            let action = $parse(self, label).map_err(diagnostic)?;
            let token = self
                .submit(GameRequest {
                    expected: self.context(),
                    actor: self.position().side_to_move(),
                    command: GameCommand::Play { action },
                })
                .map_err(diagnostic)?;
            writeln!(out, "played {label}").unwrap();
            $( $effects(self, out); )?
            Ok(token)
        }
        fn submit_actor(
            &mut self,
            args: &[&str],
            command: GameCommand,
            undo: &mut Vec<GameUndo>,
            _out: &mut String,
        ) -> Result<(), String> {
            let token = self.submit(GameRequest {
                expected: self.context(),
                actor: side(args)?,
                command,
            }).map_err(diagnostic)?;
            undo.push(token);
            Ok(())
        }
        fn undo_event(&mut self, mut undo: GameUndo) -> Result<(), String> {
            self.undo(&mut undo).map_err(diagnostic)
        }
    };
}
pub(crate) use variant_actions;
// Search crates use game-specific types for their limits, decisions and PVs.
macro_rules! analyze {
    ($search:ident, $game:expr, $args:expr, $out:expr) => {{
        use mshogi_search::$search as search;
        let limits = match $args {
            [] => search::DEFAULT_LIMITS,
            [depth, nodes] => search::Limits {
                depth: crate::shell::number(depth).ok_or("invalid ASCII depth")?,
                nodes: crate::shell::number(nodes).ok_or("invalid ASCII nodes")?,
            },
            _ => return Err("usage: analyze [depth nodes]".into()),
        };
        let result = search::search($game, limits).map_err(diagnostic)?;
        writeln!(
            $out,
            "analysis-read-only perspective {:?} status {:?} decision {:?} score {:?} completed-depth {} visited {} pv {:?}",
            result.perspective, result.status, result.decision, result.score,
            result.completed_depth, result.nodes, result.principal_variation
        ).unwrap();
        Ok(())
    }}
}
pub(crate) use analyze;

pub fn run(game: Game, profile: Option<tenjiku::Profile>) -> ExitCode {
    let mut session: Box<dyn HumanSession> = match game {
        Game::Chess => Box::new(session::Session::new()),
        Game::Shogi => Box::new(Session::new(shogi::Game::new())),
        Game::Xiangqi => match xiangqi::Game::standard(xiangqi::ClaimPolicy::Explicit) {
            Ok(game) => Box::new(Session::new(game)),
            Err(e) => {
                eprintln!("error: {}", diagnostic(e));
                return ExitCode::FAILURE;
            }
        },
        Game::Tori => Box::new(Session::new(tori::Game::standard())),
        Game::Chu => Box::new(Session::new(chu::Game::standard())),
        Game::Tenjiku => Box::new(Session::new(tenjiku::Game::standard(
            profile.expect("validated profile"),
        ))),
    };
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();
    let mut failed = false;
    let mut input = stdin.lock();
    for index in 0usize.. {
        let number = index + 1;
        let line = match read_line(&mut input) {
            Ok(Some(bytes)) => bytes,
            Ok(None) => break,
            Err(error) => {
                let _ = writeln!(stderr, "error: line {number}: cannot read input: {error}");
                return ExitCode::FAILURE;
            }
        };
        let Ok(text) = String::from_utf8(line) else {
            let _ = writeln!(stderr, "error: line {number}: input is not valid UTF-8");
            failed = true;
            continue;
        };
        let mut out = String::new();
        let result = session.execute(&text, &mut out);
        // Report output before errors so the streams stay in command order.
        if stdout
            .write_all(out.as_bytes())
            .and_then(|()| stdout.flush())
            .is_err()
        {
            return ExitCode::FAILURE;
        }
        match result {
            Ok(Next::Continue) => {}
            Ok(Next::Quit) => break,
            Err(message) => {
                let _ = writeln!(stderr, "error: line {number}: {message}");
                failed = true;
            }
        }
    }
    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

/// Read one line without its LF; a possible CR stays in the line.
fn read_line(input: &mut impl BufRead) -> io::Result<Option<Vec<u8>>> {
    let mut line = Vec::new();
    if input.read_until(b'\n', &mut line)? == 0 {
        return Ok(None);
    }
    if line.last() == Some(&b'\n') {
        line.pop();
    }
    Ok(Some(line))
}
