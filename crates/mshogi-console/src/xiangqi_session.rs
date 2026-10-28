//! Native Xiangqi diagnostic consumer. No local rules or evaluator are implemented here.
use crate::{
    record_io,
    session::{MAX_LINE_BYTES, Next},
};
use mshogi_core::{native::CanonicalRecord, xiangqi::*};
use std::{
    fmt::Write as _,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver},
    },
    thread::{self, JoinHandle},
};
const HELP: &str = "Untimed Xiangqi diagnostic console (not UCCI/UCI protocol):
  position [explicit|auto_valid_claims] startpos|fen <FEN>|json <root-JSON>
  fen | show | status | legal | history | warnings
  move <a0a1>...                     Atomic complete native actions
  undo [n]                          Native commands, including same-ply claims
  eligibility red|black             Advisory claimant-specific natural count
  claim|resign|offer|accept|decline red|black
  record print|save <path>|load-file <path>|load-json <JSON>
  perft <0..3> | divide <1..3>        Local board counts, not match outcomes
  nowin fast                        Scoped native fast assessment
  nowin exact [states edges millis memory-bytes [bfs|dfs]]
                                    Explicit bounded read-only worker
  nowin poll|wait|cancel             Observe or retire exact work
  help | quit
Native coordinate dialect: files a-i from Red left, ranks0-9 from Red baseline;
Show uses uppercase Red/lowercase Black, K A B N R C P (B=Elephant), dots for empty.
FEN import starts known history; it never restores earlier warnings/claims.
Use the canonical envelope for full restoration. Saves require a new path.
Exact observations print their full source record, purpose, target, identity and
limits; they are diagnostic evidence, not pending work or part of GameRecord.
Exact acceptance never changes a game conclusion; required Unknown is unscored.
Ordinary queries/replay do not invoke the exact verifier. Changing the game/undo
retires pending exact work. Unknown/failure/cancellation are not game results.";
struct Worker {
    stop: Arc<AtomicBool>,
    receive: Receiver<ExactObservation>,
    thread: JoinHandle<()>,
}
pub struct Session {
    game: Game,
    undo: Vec<GameUndo>,
    analysis: ExactAnalysis,
    worker: Option<Worker>,
}
impl Drop for Session {
    fn drop(&mut self) {
        self.retire();
    }
}
impl Session {
    pub fn new() -> Result<Self, String> {
        Ok(Self {
            game: Game::standard(ClaimPolicy::Explicit).map_err(error)?,
            undo: vec![],
            analysis: ExactAnalysis::new().map_err(error)?,
            worker: None,
        })
    }
    fn record(&self) -> Result<CanonicalRecord, String> {
        Ok(CanonicalRecord::xiangqi(
            GameRecord::from_game(&self.game).map_err(error)?,
        ))
    }
    fn candidate(&self) -> Result<(Game, Vec<GameUndo>), String> {
        GameRecord::from_game(&self.game)
            .map_err(error)?
            .to_game_with_undo()
            .map_err(error)
    }
    fn retire(&mut self) {
        if let Some(worker) = self.worker.take() {
            worker.stop.store(true, Ordering::Relaxed);
            let _ = worker.thread.join();
        }
        let _ = self.analysis.retire();
    }
    fn replace(&mut self, game: Game, undo: Vec<GameUndo>) -> Result<(), String> {
        // Measure the actual accepted wrapper before committing; native limits alone
        // cannot promise space for the envelope overhead.
        CanonicalRecord::xiangqi(GameRecord::from_game(&game).map_err(error)?)
            .to_json()
            .map_err(error)?;
        self.retire();
        self.game = game;
        self.undo = undo;
        Ok(())
    }
    pub fn execute(&mut self, line: &str, out: &mut String) -> Result<Next, String> {
        if line.len() > MAX_LINE_BYTES {
            return Err(format!("line exceeds {MAX_LINE_BYTES} bytes"));
        }
        let line = line.trim_matches(|c: char| c.is_ascii_whitespace());
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
                self.retire();
                return Ok(Next::Quit);
            }
            "position" => {
                let (policy, body) = if let Some(body) = rest.strip_prefix("auto_valid_claims ") {
                    (ClaimPolicy::AutoValidClaims, body)
                } else {
                    (
                        ClaimPolicy::Explicit,
                        rest.strip_prefix("explicit ").unwrap_or(rest),
                    )
                };
                let root = if body == "startpos" {
                    PositionRoot::standard()
                } else if let Some(fen) = body.strip_prefix("fen ") {
                    PositionRoot::from_fen(fen).map_err(error)?
                } else if let Some(json) = body.strip_prefix("json ") {
                    PositionRoot::from_json(json).map_err(error)?
                } else {
                    return Err("usage: position [explicit|auto_valid_claims] startpos|fen <FEN>|json <root-JSON>".into());
                };
                self.replace(Game::from_root(root, policy).map_err(error)?, vec![])?;
                self.status(out)?;
                writeln!(
                    out,
                    "history-segment starts-at-root; earlier imported history unknown"
                )
                .unwrap();
            }
            "fen" => {
                empty(&args)?;
                writeln!(out, "fen {}", self.game.position().to_fen().map_err(error)?).unwrap();
            }
            "show" => {
                empty(&args)?;
                for rank in (0..10).rev() {
                    write!(out, "{rank} ").unwrap();
                    for file in 0..9 {
                        let token = self
                            .game
                            .position()
                            .piece(Intersection::new(file, rank).unwrap())
                            .map_or('.', |p| {
                                let code = letter(p.kind);
                                if p.side == Side::Red {
                                    code
                                } else {
                                    code.to_ascii_lowercase()
                                }
                            });
                        if file > 0 {
                            write!(out, " ").unwrap();
                        }
                        write!(out, "{token}").unwrap();
                    }
                    writeln!(out).unwrap();
                    if rank == 5 {
                        writeln!(out, "  ----- river -----").unwrap();
                    }
                }
                writeln!(out, "  a b c d e f g h i (intersections)").unwrap();
                self.status(out)?;
            }
            "status" => {
                empty(&args)?;
                self.status(out)?;
            }
            "legal" => {
                empty(&args)?;
                let choices = self.game.position().legal_actions().map_err(error)?;
                writeln!(
                    out,
                    "legal-local {} (actual command admission also checks history/conclusion)",
                    choices.len()
                )
                .unwrap();
                for a in choices {
                    writeln!(out, "{a}").unwrap();
                }
            }
            "move" => {
                if args.is_empty() {
                    return Err("usage: move <a0a1>...".into());
                }
                let (mut game, mut undo) = self.candidate()?;
                for text in &args {
                    let action = text.parse::<Action>().map_err(error)?;
                    let result = game
                        .submit(GameRequest {
                            expected: game.context(),
                            actor: game.position().side_to_move(),
                            command: GameCommand::Play { action },
                        })
                        .map_err(|e| {
                            format!("native action rejected: {e:?}; no actions applied")
                        })?;
                    undo.push(result.undo);
                }
                self.replace(game, undo)?;
                writeln!(out, "played {}", args.join(" ")).unwrap();
                self.status(out)?;
            }
            "eligibility" | "claim" | "resign" | "offer" | "accept" | "decline" => {
                let actor = side(&args)?;
                if command == "eligibility" {
                    writeln!(
                        out,
                        "eligibility {} {} (advisory)",
                        side_text(actor),
                        serde_json::to_string(&self.game.natural_count_eligibility(actor))
                            .map_err(error)?
                    )
                    .unwrap();
                } else {
                    let cmd = match command {
                        "claim" => GameCommand::ClaimNaturalCount,
                        "resign" => GameCommand::Resign,
                        "offer" => GameCommand::OfferDraw,
                        "accept" => GameCommand::AcceptDraw,
                        _ => GameCommand::DeclineDraw,
                    };
                    let (mut game, mut undo) = self.candidate()?;
                    let result = game
                        .submit(GameRequest {
                            expected: game.context(),
                            actor,
                            command: cmd,
                        })
                        .map_err(error)?;
                    let kind = result.result;
                    undo.push(result.undo);
                    self.replace(game, undo)?;
                    writeln!(
                        out,
                        "command-result {}",
                        serde_json::to_string(&kind).map_err(error)?
                    )
                    .unwrap();
                    self.status(out)?;
                }
            }
            "undo" => {
                let count = match args.as_slice() {
                    [] => 1,
                    [n] => number(n)
                        .filter(|v| *v > 0)
                        .ok_or("undo requires a positive ASCII command count")?,
                    _ => return Err("usage: undo [n]".into()),
                };
                if count > self.undo.len() {
                    return Err(format!(
                        "cannot undo {count}: only {} commands",
                        self.undo.len()
                    ));
                }
                // Native handles are owned and known LIFO; any invariant failure is
                // explicit, never an invented outcome. Ordinary bounds are checked first.
                self.retire();
                for _ in 0..count {
                    self.game.undo(self.undo.pop().unwrap()).map_err(error)?;
                }
                self.status(out)?;
            }
            "history" | "warnings" => {
                empty(&args)?;
                if command == "history" {
                    writeln!(out, "profile {PROFILE_ID} edition {PROFILE_EDITION}").unwrap();
                    writeln!(out, "root {}", self.game.root().to_json()).unwrap();
                    for event in self.game.events() {
                        writeln!(
                            out,
                            "event {}",
                            serde_json::to_string(event).map_err(error)?
                        )
                        .unwrap();
                    }
                }
                writeln!(
                    out,
                    "warnings {}",
                    serde_json::to_string(self.game.warnings()).map_err(error)?
                )
                .unwrap();
                self.status(out)?;
            }
            "record" => {
                use record_io::Operation;
                match record_io::parse(rest, record_io::LegacyLoad::File)? {
                    Operation::Print | Operation::LegacyPrint => {
                        writeln!(out, "record {}", self.record()?.to_json().map_err(error)?)
                            .unwrap();
                    }
                    Operation::Save(path) => {
                        record_io::save(path, &self.record()?.to_json().map_err(error)?, out)?
                    }
                    Operation::LoadFile(path) => self.load(&record_io::read(path)?, out)?,
                    Operation::LoadJson(json) => self.load(json, out)?,
                    Operation::LegacyJson(_) => unreachable!(),
                }
            }
            "perft" | "divide" => {
                let [value] = args.as_slice() else {
                    return Err("diagnostic requires one ASCII depth".into());
                };
                let depth = number(value)
                    .filter(|d| *d <= MAX_PERFT_DEPTH as usize)
                    .ok_or("xiangqi depth must be0..3 (divide1..3)")?
                    as u8;
                if command == "perft" {
                    writeln!(
                        out,
                        "perft-local {depth} {}",
                        self.game.position().perft(depth).map_err(error)?
                    )
                    .unwrap();
                } else {
                    let branches = self.game.position().divide(depth).map_err(error)?;
                    let mut total = 0;
                    for (a, n) in branches {
                        writeln!(out, "{a} {n}").unwrap();
                        total += n;
                    }
                    writeln!(out, "total-local {total}").unwrap();
                }
            }
            "nowin" => self.nowin(&args, out)?,
            _ => {
                return Err(format!(
                    "unknown xiangqi command '{}'",
                    record_io::diagnostic(line)
                ));
            }
        }
        Ok(Next::Continue)
    }
    fn load(&mut self, text: &str, out: &mut String) -> Result<(), String> {
        let CanonicalRecord::Xiangqi(record) = CanonicalRecord::from_json(text).map_err(error)?
        else {
            return Err("record is not xiangqi; session preserved".into());
        };
        let (game, undo) = record.to_game_with_undo().map_err(error)?;
        self.replace(game, undo)?;
        writeln!(
            out,
            "loaded complete native record; fresh runtime; exact work stopped"
        )
        .unwrap();
        self.status(out)
    }
    fn status(&self, out: &mut String) -> Result<(), String> {
        if let Ok(fen) = self.game.position().to_fen() {
            writeln!(out, "fen {fen}").unwrap();
        } else {
            writeln!(
                out,
                "position-json {}",
                PositionRoot::analysis(self.game.position().clone()).to_json()
            )
            .unwrap();
        }
        let r = GameRecord::from_game(&self.game).map_err(error)?;
        writeln!(
            out,
            "side {} policy {} known-board-plies {} known-events {} conclusion {} pending {}",
            side_text(self.game.position().side_to_move()),
            serde_json::to_string(&self.game.claim_policy()).map_err(error)?,
            r.known_board_actions(),
            r.event_count(),
            serde_json::to_string(&self.game.conclusion()).map_err(error)?,
            serde_json::to_string(&self.game.pending_adjudication()).map_err(error)?
        )
        .unwrap();
        writeln!(
            out,
            "no-win-fast {} (not an exact or ongoing-game verdict)",
            serde_json::to_string(&self.game.no_win_assessment()).map_err(error)?
        )
        .unwrap();
        Ok(())
    }
    fn nowin(&mut self, args: &[&str], out: &mut String) -> Result<(), String> {
        match args {
            ["fast"] => {
                writeln!(
                    out,
                    "no-win-fast {}",
                    serde_json::to_string(&self.game.no_win_assessment()).map_err(error)?
                )
                .unwrap();
            }
            ["cancel"] => {
                let active = self.worker.is_some();
                self.retire();
                writeln!(
                    out,
                    "exact retired {active}; no result accepted; game unchanged"
                )
                .unwrap();
            }
            ["poll"] | ["wait"] => {
                let worker = self.worker.as_ref().ok_or("no active exact request")?;
                let observation = if args[0] == "wait" {
                    Some(
                        worker
                            .receive
                            .recv()
                            .map_err(|_| "exact worker failed; unscored")?,
                    )
                } else {
                    match worker.receive.try_recv() {
                        Ok(o) => Some(o),
                        Err(mpsc::TryRecvError::Empty) => None,
                        Err(_) => return Err("exact worker failed; unscored".into()),
                    }
                };
                if let Some(observation) = observation {
                    let worker = self.worker.take().unwrap();
                    worker
                        .thread
                        .join()
                        .map_err(|_| "exact worker panicked; unscored")?;
                    let report = self
                        .analysis
                        .accept(&self.game, &observation)
                        .map_err(error)?;
                    let request = observation.request();
                    writeln!(out,"nowin-context purpose=natural_board_win goal={NATURAL_WIN_GOAL} identity={:?} target={:?} limits={}",request.identity(),request.target(),serde_json::to_string(&request.limits()).map_err(error)?).unwrap();
                    writeln!(
                        out,
                        "nowin-source {}",
                        CanonicalRecord::xiangqi(request.source_record().clone())
                            .to_json()
                            .map_err(error)?
                    )
                    .unwrap();
                    writeln!(
                        out,
                        "nowin-result {}",
                        serde_json::to_string(&report).map_err(error)?
                    )
                    .unwrap();
                    writeln!(out,"exact is read-only; required incomplete adjudication stays unresolved/unscored; GameRecord contains game semantics only").unwrap();
                } else {
                    writeln!(out, "exact pending").unwrap();
                }
            }
            ["exact", rest @ ..] => {
                let mut limits = ExactLimits::default();
                if !rest.is_empty() {
                    if !matches!(rest.len(), 4 | 5) {
                        return Err(
                            "usage: nowin exact [states edges millis memory-bytes [bfs|dfs]]"
                                .into(),
                        );
                    }
                    limits.states = number(rest[0])
                        .and_then(|n| n.try_into().ok())
                        .ok_or("invalid states")?;
                    limits.transitions =
                        number(rest[1]).map(|n| n as u64).ok_or("invalid edges")?;
                    limits.millis = number(rest[2])
                        .and_then(|n| n.try_into().ok())
                        .ok_or("invalid millis")?;
                    limits.memory_bytes = number(rest[3]).ok_or("invalid memory bytes")?;
                    if rest.len() == 5 {
                        limits.traversal = match rest[4] {
                            "bfs" => ExactTraversal::BreadthFirst,
                            "dfs" => ExactTraversal::DepthFirst,
                            _ => return Err("traversal must be bfs or dfs".into()),
                        };
                    }
                }
                self.retire();
                let request = self.analysis.begin(&self.game, limits).map_err(error)?;
                let stop = Arc::new(AtomicBool::new(false));
                let cancellation = stop.clone();
                let (send, receive) = mpsc::sync_channel(1);
                let thread = thread::Builder::new()
                    .name("xiangqi-no-win-diagnostic".into())
                    .spawn(move || {
                        let result = request.run(|| cancellation.load(Ordering::Relaxed));
                        let _ = send.send(result);
                    })
                    .map_err(error)?;
                self.worker = Some(Worker {
                    stop,
                    receive,
                    thread,
                });
                writeln!(out, "exact started; explicit bounded read-only work").unwrap();
            }
            _ => return Err("usage: nowin fast|exact [limits]|poll|wait|cancel".into()),
        }
        Ok(())
    }
}
fn error(e: impl std::fmt::Display) -> String {
    record_io::diagnostic(&e.to_string())
}
fn empty(args: &[&str]) -> Result<(), String> {
    if args.is_empty() {
        Ok(())
    } else {
        Err("command takes no arguments".into())
    }
}
fn number(s: &str) -> Option<usize> {
    (!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()))
        .then(|| s.parse().ok())
        .flatten()
}
fn side(args: &[&str]) -> Result<Side, String> {
    match args {
        ["red"] => Ok(Side::Red),
        ["black"] => Ok(Side::Black),
        _ => Err("actor must be red or black".into()),
    }
}
fn side_text(s: Side) -> &'static str {
    match s {
        Side::Red => "red",
        Side::Black => "black",
    }
}
fn letter(k: PieceKind) -> char {
    match k {
        PieceKind::General => 'K',
        PieceKind::Advisor => 'A',
        PieceKind::Elephant => 'B',
        PieceKind::Horse => 'N',
        PieceKind::Rook => 'R',
        PieceKind::Cannon => 'C',
        PieceKind::Soldier => 'P',
    }
}
