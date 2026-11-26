//! Native Xiangqi diagnostic consumer. No local rules or evaluator are implemented here.
use crate::{
    diagnostics,
    shell::{self, Variant, diagnostic},
};
use mshogi_core::perft::PerftPosition;
use mshogi_core::{native::CanonicalRecord, xiangqi::*};
use std::fmt::Write as _;
const HELP: &str = "Untimed Xiangqi diagnostic console (not UCCI/UCI protocol):
  position [explicit|auto_valid_claims] startpos|fen <FEN>|json <root-JSON>
  fen | show | status | legal | history | warnings
  move <a0a1>...                     Atomic complete native actions
  undo [n]                          Native commands, including same-ply claims
  eligibility red|black             Advisory claimant-specific natural count
  claim|resign|offer-draw|accept-draw|decline-draw red|black
  record print|save <path>|load-file <path>|load-json <JSON>
  perft <0..3> [visits] | divide <1..3> [visits]  Local board counts, not match outcomes
  nowin fast                        Scoped native fast assessment
  help | quit
Native coordinate dialect: files a-i from Red left, ranks 0-9 from Red baseline;
Show uses uppercase Red/lowercase Black, K A B N R C P (B=Elephant), dots for empty.
FEN import starts known history; it never restores earlier warnings/claims.
Use the canonical envelope for full restoration. Saves require a new path.
The fast no-win assessment is Unknown when it finds no sound proof; Unknown and
failure are not game results.";
impl Variant for Game {
    type Undo = GameUndo;
    type Command = GameCommand;
    const NAME: &'static str = "xiangqi";
    const HELP: &'static str = HELP;
    const MOVE_SYNTAX: &'static str = "a0a1";
    const LOADED: &'static str = "loaded complete native record; fresh runtime";
    shell::native_record!(xiangqi, Xiangqi);
    fn position_command(&self, rest: &str) -> Result<Self, String> {
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
            PositionRoot::from_fen(fen).map_err(diagnostic)?
        } else if let Some(json) = body.strip_prefix("json ") {
            PositionRoot::from_json(json).map_err(diagnostic)?
        } else {
            return Err(
                "usage: position [explicit|auto_valid_claims] startpos|fen <FEN>|json <root-JSON>"
                    .into(),
            );
        };
        Game::from_root(root, policy).map_err(diagnostic)
    }
    fn after_position(&self, out: &mut String) -> Result<(), String> {
        self.write_status(out)?;
        writeln!(
            out,
            "history-segment starts-at-root; earlier imported history unknown"
        )
        .unwrap();
        Ok(())
    }
    fn play(&mut self, text: &str, out: &mut String) -> Result<GameUndo, String> {
        let action = text.parse::<Action>().map_err(diagnostic)?;
        let result = self
            .submit(GameRequest {
                expected: self.context(),
                actor: self.position().side_to_move(),
                command: GameCommand::Play { action },
            })
            .map_err(diagnostic)?;
        if out.is_empty() {
            out.push_str("played");
        }
        write!(out, " {text}").unwrap();
        Ok(result.undo)
    }
    fn submit_actor(
        &mut self,
        args: &[&str],
        command: GameCommand,
        out: &mut String,
    ) -> Result<GameUndo, String> {
        let result = self
            .submit(GameRequest {
                expected: self.context(),
                actor: side(args)?,
                command,
            })
            .map_err(diagnostic)?;
        writeln!(
            out,
            "command-result {}",
            serde_json::to_string(&result.result).map_err(diagnostic)?
        )
        .unwrap();
        Ok(result.undo)
    }
    fn undo_event(&mut self, undo: GameUndo) -> Result<(), String> {
        self.undo(undo).map_err(diagnostic)
    }
    fn actor_command(command: &str) -> Option<GameCommand> {
        match command {
            "resign" => Some(GameCommand::Resign),
            "offer-draw" => Some(GameCommand::OfferDraw),
            "accept-draw" => Some(GameCommand::AcceptDraw),
            "decline-draw" => Some(GameCommand::DeclineDraw),
            "claim" => Some(GameCommand::ClaimNaturalCount),
            _ => None,
        }
    }
    fn write_status(&self, out: &mut String) -> Result<(), String> {
        if let Ok(fen) = self.position().to_fen() {
            writeln!(out, "fen {fen}").unwrap();
        } else {
            writeln!(
                out,
                "position-json {}",
                PositionRoot::analysis(self.position().clone()).to_json()
            )
            .unwrap();
        }
        let r = GameRecord::from_game(&self).map_err(diagnostic)?;
        writeln!(
            out,
            "side {} policy {} known-board-plies {} known-events {} conclusion {} pending {}",
            side_text(self.position().side_to_move()),
            serde_json::to_string(&self.claim_policy()).map_err(diagnostic)?,
            r.known_board_actions(),
            r.event_count(),
            serde_json::to_string(&self.conclusion()).map_err(diagnostic)?,
            serde_json::to_string(&self.pending_adjudication()).map_err(diagnostic)?
        )
        .unwrap();
        writeln!(
            out,
            "no-win-fast {} (not an ongoing-game verdict)",
            serde_json::to_string(&self.no_win_assessment()).map_err(diagnostic)?
        )
        .unwrap();
        Ok(())
    }
    fn board(&self, out: &mut String) {
        for rank in (0..10).rev() {
            write!(out, "{rank} ").unwrap();
            for file in 0..9 {
                let token = self
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
    }
    fn legal(&self, out: &mut String) -> Result<(), String> {
        let choices = self.position().legal_actions().map_err(diagnostic)?;
        writeln!(
            out,
            "legal-local {} (actual command admission also checks history/conclusion)",
            choices.len()
        )
        .unwrap();
        for a in choices {
            writeln!(out, "{a}").unwrap();
        }
        Ok(())
    }
    fn history(&self, out: &mut String) -> Result<(), String> {
        {
            writeln!(out, "profile {PROFILE_ID} edition {PROFILE_EDITION}").unwrap();
            writeln!(out, "root {}", self.root().to_json()).unwrap();
            for event in self.events() {
                writeln!(
                    out,
                    "event {}",
                    serde_json::to_string(event).map_err(diagnostic)?
                )
                .unwrap();
            }
        }
        warnings(self, out)?;
        Ok(())
    }
    fn perft(&self, command: &str, args: &[&str], out: &mut String) -> Result<(), String> {
        diagnostics::execute(
            self.position(),
            command,
            args,
            Position::MAX_DEPTH,
            diagnostics::Style::Local,
            out,
            |action| Ok(action.to_string()),
        )
    }
    fn extra(&self, command: &str, args: &[&str], out: &mut String) -> Result<(), String> {
        match command {
            "fen" => {
                shell::empty(command, args)?;
                writeln!(out, "fen {}", self.position().to_fen().map_err(diagnostic)?).unwrap();
                Ok(())
            }
            "eligibility" => {
                let actor = side(args)?;

                writeln!(
                    out,
                    "eligibility {} {} (advisory)",
                    side_text(actor),
                    serde_json::to_string(&self.natural_count_eligibility(actor))
                        .map_err(diagnostic)?
                )
                .unwrap();
                Ok(())
            }
            "nowin" => nowin(self, args, out),
            "warnings" => {
                shell::empty(command, args)?;
                warnings(self, out)
            }
            _ => Err(shell::unknown::<Self>(command)),
        }
    }
}
fn nowin(game: &Game, args: &[&str], out: &mut String) -> Result<(), String> {
    if args != ["fast"] {
        return Err("usage: nowin fast".into());
    }
    writeln!(
        out,
        "no-win-fast {}",
        serde_json::to_string(&game.no_win_assessment()).map_err(diagnostic)?
    )
    .unwrap();
    Ok(())
}
fn warnings(game: &Game, out: &mut String) -> Result<(), String> {
    writeln!(
        out,
        "warnings {}",
        serde_json::to_string(game.warnings()).map_err(diagnostic)?
    )
    .unwrap();
    game.write_status(out)
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
