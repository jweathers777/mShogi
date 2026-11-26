//! Human Chu adapter. Native Game alone owns rules, history and admission.
use crate::{
    diagnostics,
    shell::{self, PlayEffects, Variant, diagnostic},
};
use mshogi_core::{chu::*, native::CanonicalRecord};
use std::fmt::Write as _;
const MAX_DIAGNOSTIC_DEPTH: u32 = 2;
const HELP: &str = "Untimed Chu Shogi diagnostic console (not USI/SFEN):
  position startpos                Standard root with complete known history
  show | status                    Board, native royal status and exact history
  legal                            Complete history-permitted native actions
  move <Chu-action>...              Atomic batch, complete routes only
  resign sente|gote                 Actual resignation, independent of board turn
  offer-draw sente|gote             Actual draw offer; not an automatic result
  accept-draw sente|gote | decline-draw sente|gote
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

impl Variant for Game {
    type Undo = GameUndo;
    type Command = GameCommand;
    const NAME: &'static str = "Chu";
    const HELP: &'static str = HELP;
    const MOVE_SYNTAX: &'static str = "Chu-action";
    const LOADED: &'static str = "loaded complete native Chu record";
    shell::native_record!(chu, Chu);
    fn position_command(&self, rest: &str) -> Result<Self, String> {
        shell::startpos(rest, Game::standard)
    }
    shell::variant_actions!(|game: &Game, label| notation::parse_local_action(
        game.local_position(),
        label
    ));
    fn actor_command(command: &str) -> Option<GameCommand> {
        match command {
            "resign" => Some(GameCommand::Resign),
            "offer-draw" => Some(GameCommand::OfferDraw),
            "accept-draw" => Some(GameCommand::AcceptDraw),
            "decline-draw" => Some(GameCommand::DeclineDraw),
            _ => None,
        }
    }
    fn write_status(&self, out: &mut String) -> Result<(), String> {
        let root = self.position().root();
        writeln!(out, "profile {} edition {}", PROFILE_ID, PROFILE_EDITION).unwrap();
        writeln!(out,"root {:?} source {:?} declared-root-plies {} earlier-history-unknown {} reachability-proven {}",root.kind(),root.source(),root.declared_completed_plies(),root.earlier_history_unknown(),root.historical_reachability_proven()).unwrap();
        writeln!(out,"side {} known-events {} known-board-actions {} occurrences {:?} obligation {:?} conclusion {:?}",side_text(self.position().side_to_move()),self.events().len(),self.snapshot().board_ply,self.occurrences(),self.obligations(),self.conclusion()).unwrap();
        for side in [Side::Sente, Side::Gote] {
            let royals: Vec<_> = self
                .royal_squares(side)
                .into_iter()
                .map(|s| format!("{}@{}", self.position().piece(s).unwrap().face().code(), s))
                .collect();
            writeln!(out, "royals {} {}", side_text(side), royals.join(" ")).unwrap();
        }
        writeln!(
            out,
            "native-status {:?} draw-offer {:?} lion-protection {:?}",
            self.status(),
            self.snapshot().draw_offer,
            self.local_position().protection()
        )
        .unwrap();
        Ok(())
    }
    fn board(&self, out: &mut String) {
        shell::shogi_board(out, 12, 4, |file, rank| {
            self.position()
                .piece(Square::new(file, rank).unwrap())
                .map_or(".".into(), |p| {
                    format!(
                        "{}{}",
                        if p.side == Side::Sente { "s" } else { "g" },
                        p.code()
                    )
                })
        });
    }
    fn legal(&self, out: &mut String) -> Result<(), String> {
        let actions = self.legal_actions();
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
                notation::format_action(self.position(), action.clone()).map_err(diagnostic)?,
                serde_json::to_string(&action).expect("native action")
            )
            .unwrap();
        }
        Ok(())
    }
    fn history(&self, out: &mut String) -> Result<(), String> {
        self.write_status(out)?;
        for event in self.events() {
            let label = match &event.command {
                GameCommand::Play { action } => notation::format_history_action(
                    &self.event_prefix(event.index).map_err(diagnostic)?,
                    action.clone(),
                )
                .map_err(diagnostic)?,
                _ => format!("{:?}", event.command),
            };
            writeln!(out,"event {} board-ply {} actor {} action {} occurrences {:?} obligation {:?} conclusion {:?}",event.index+1,event.board_ply,side_text(event.actor),label,event.occurrences,event.obligations,event.conclusion).unwrap();
        }
        Ok(())
    }
    fn perft(&self, command: &str, args: &[&str], out: &mut String) -> Result<(), String> {
        diagnostics::execute(
            self.local_position(),
            command,
            args,
            MAX_DIAGNOSTIC_DEPTH,
            diagnostics::Style::LocalWithVisits,
            out,
            |action| notation::format_action(self.position(), action).map_err(diagnostic),
        )
    }
    fn extra(&self, command: &str, args: &[&str], out: &mut String) -> Result<(), String> {
        match command {
            "analyze" => shell::analyze!(chu, self, args, out),
            _ => Err(shell::unknown::<Self>(command)),
        }
    }
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
