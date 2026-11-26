//! Human Tenjiku adapter. Native Game alone owns rules, history and admission.
use crate::{
    diagnostics,
    shell::{self, PlayEffects, Variant, diagnostic},
};
use mshogi_core::perft::PerftPosition;
use mshogi_core::{native::CanonicalRecord, tenjiku::*};
use std::fmt::Write as _;
const MAX_DIAGNOSTIC_DEPTH: u32 = Game::MAX_DEPTH;
const HELP: &str = "Untimed Tenjiku Shogi diagnostic console (not USI/SFEN):
  position startpos                Standard root with complete known history
  show | status                    Board, native royal status and exact history
  legal                            Complete history-permitted native actions
  move <Tenjiku-action>...          Atomic batch, complete routes only
  resign sente|gote                 Actual resignation, independent of board turn
  offer-draw sente|gote             Actual draw offer; not an automatic result
  accept-draw sente|gote | decline-draw sente|gote
  undo [n] | history               Exact native events and event reversal
  record print | record save <path> | record load-file <path> | record load-json <JSON>
  perft <0..3> [visits] | divide <1..3> [visits]  Bounded full-history diagnostics
  analyze [depth nodes]             Read-only bounded history-aware search
  help | quit
Tenjiku actions: Ln8hx8gx8f ordered captures; Ln8hx8g-8h capture-return;
VGn4h-4g-5g-5f three actual landings; ChS+8h!x9h stationary igui;
P12f-12e^ requests promotion. Original code+ retains original identity.
Burn/death are native derived effects, never input choices. No captured hands.
Perft/divide uses full native history/endings, depth 0..3 (divide 1..3).
Saves require a new path; strict canonical records preserve the selected profile.";

impl Variant for Game {
    type Undo = GameUndo;
    type Command = GameCommand;
    const NAME: &'static str = "Tenjiku";
    const HELP: &'static str = HELP;
    const MOVE_SYNTAX: &'static str = "Tenjiku-action";
    const LOADED: &'static str = "loaded complete native Tenjiku record";
    shell::native_record!(tenjiku, Tenjiku);
    fn position_command(&self, rest: &str) -> Result<Self, String> {
        shell::startpos(rest, || Game::standard(self.profile()))
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
        writeln!(
            out,
            "profile {} edition {}",
            self.profile().id(),
            self.profile().edition()
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
        writeln!(
            out,
            "side {} known-events {} known-board-actions {} pending-bare {:?} conclusion {:?}",
            side_text(self.position().side_to_move()),
            self.events().len(),
            self.snapshot().board_ply,
            self.snapshot().pending_bare,
            self.conclusion()
        )
        .unwrap();
        for side in [Side::Sente, Side::Gote] {
            let royals: Vec<_> = self
                .royals(side)
                .into_iter()
                .map(|(square, piece)| format!("{}@{}", piece.face().code(), square))
                .collect();
            writeln!(out, "royals {} {}", side_text(side), royals.join(" ")).unwrap();
        }
        writeln!(
            out,
            "native-status {:?} draw-offer {:?}",
            self.status(),
            self.snapshot().draw_offer
        )
        .unwrap();
        Ok(())
    }
    fn board(&self, out: &mut String) {
        shell::shogi_board(out, 16, 5, |file, rank| {
            self.position()
                .piece(Square::new(file, rank).unwrap())
                .map_or(".".into(), |p| {
                    format!(
                        "{}{}{}",
                        if p.side == Side::Sente { "s" } else { "g" },
                        p.base.code(),
                        if p.promoted { "+" } else { "" }
                    )
                })
        });
    }
    fn legal(&self, out: &mut String) -> Result<(), String> {
        let actions = self.legal_actions();
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
            let effects = event
                .effects
                .as_ref()
                .map(notation::format_effects)
                .unwrap_or_else(|| "No board effects.".into());
            writeln!(out,"event {} board-ply {} actor {} action {} pending-bare {:?} conclusion {:?} effects {}",event.index+1,event.board_ply,side_text(event.actor),label,event.pending_bare,event.conclusion,effects).unwrap();
        }
        Ok(())
    }
    fn perft(&self, command: &str, args: &[&str], out: &mut String) -> Result<(), String> {
        diagnostics::execute(
            self,
            command,
            args,
            MAX_DIAGNOSTIC_DEPTH,
            diagnostics::Style::Game,
            out,
            |action| notation::format_action(self.position(), action).map_err(diagnostic),
        )
    }
    fn extra(&self, command: &str, args: &[&str], out: &mut String) -> Result<(), String> {
        match command {
            "analyze" => shell::analyze!(tenjiku, self, args, out),
            _ => Err(shell::unknown::<Self>(command)),
        }
    }
}
impl PlayEffects for Game {
    fn print_effects(&self, out: &mut String) {
        writeln!(
            out,
            "effects {}",
            notation::format_effects(
                self.events()
                    .last()
                    .and_then(|event| event.effects.as_ref())
                    .expect("admitted Play has native effects")
            )
        )
        .unwrap();
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
