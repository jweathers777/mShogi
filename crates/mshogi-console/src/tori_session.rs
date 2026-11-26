//! Human Tori adapter. Native Game alone owns rules, history and admission.
use crate::{
    diagnostics,
    shell::{self, Variant, diagnostic},
};
use mshogi_core::{native::CanonicalRecord, tori::*};
use std::fmt::Write as _;
const MAX_DIAGNOSTIC_DEPTH: u32 = 3;
const HELP: &str = "Untimed Tori Shogi diagnostic console (not USI/SFEN):
  position startpos                 Standard root with complete known history
  show | hands | status             Board, resources and native game state
  legal                            Complete history-permitted native choices
  move <Tori-action>...             Atomic batch; native Game admits each action
  resign sente|gote                 Native resignation, including out of turn
  undo [n]                         Exact event undo, including resignation
  history                          Native event ordering, conclusions, repetition limits
  record print                     Complete canonical envelope, not display text
  record save <path>                Atomic new-file save; never overwrite
  record load-file <path>           Strict bounded replay before replacement
  record load-json <JSON>           Strict inline canonical envelope
  perft <0..3> [visits] | divide <1..3> [visits]  Bounded local diagnostics
  analyze [depth nodes]             Read-only bounded native history-aware search
  help | quit
Tori actions: Sw3d-3c, capture Fa7cx7b=Ea, drop RQ*5e. Source face and
coordinates are exact; =Ea/=Go records compulsory changed identity. LQ/RQ
are distinct. Parsing selects native local choices; Game enforces history.
Board: s/g owner prefixes, no colon; Ph Fa Ea Cr Pt LQ RQ Sw Go in fixed cells.
Perft/divide ignores history and conclusions, never changes the live game.";

impl Variant for Game {
    type Undo = GameUndo;
    type Command = GameCommand;
    const NAME: &'static str = "Tori";
    const HELP: &'static str = HELP;
    const MOVE_SYNTAX: &'static str = "Tori-action";
    const LOADED: &'static str = "loaded complete native Tori record";
    shell::native_record!(tori, Tori);
    fn position_command(&self, rest: &str) -> Result<Self, String> {
        shell::startpos(rest, Game::standard)
    }
    shell::variant_actions!(
        |game: &Game, label| notation::parse_action(game.position(), label),
        "T06 history applied"
    );
    fn actor_command(command: &str) -> Option<GameCommand> {
        match command {
            "resign" => Some(GameCommand::Resign),
            _ => None,
        }
    }
    fn write_status(&self, out: &mut String) -> Result<(), String> {
        let root = self.position().root();
        writeln!(out, "profile {} edition {}", PROFILE_ID, PROFILE_EDITION).unwrap();
        writeln!(out,"root {:?} source {:?} declared-root-plies {} earlier-history-unknown {} reachability-proven {}",root.kind(),root.source(),root.declared_completed_plies(),root.earlier_history_unknown(),root.historical_reachability_proven()).unwrap();
        writeln!(out,"side {} known-events {} known-board-actions {} occurrences {:?} repetition {:?} conclusion {:?}",side_text(self.position().side_to_move()),self.events().len(),self.snapshot().board_ply,self.occurrences(),self.repetition(),self.conclusion()).unwrap();
        Ok(())
    }
    fn board(&self, out: &mut String) {
        shell::shogi_board(out, 7, 3, |file, rank| {
            self.position()
                .piece(Square::new(file, rank).unwrap())
                .map_or(".".into(), |p| {
                    format!(
                        "{}{}",
                        if p.side == Side::Sente { "s" } else { "g" },
                        p.face.code()
                    )
                })
        });
        hands(self, out);
    }
    fn history(&self, out: &mut String) -> Result<(), String> {
        self.write_status(out)?;
        for event in self.events() {
            let label = match event.command {
                GameCommand::Play { action } => notation::format_history_action(
                    &self.event_prefix(event.index).map_err(diagnostic)?,
                    action,
                )
                .map_err(diagnostic)?,
                _ => format!("{:?}", event.command),
            };
            writeln!(
                out,
                "event {} board-ply {} actor {} action {} occurrences {:?} conclusion {:?}",
                event.index + 1,
                event.board_ply,
                side_text(event.actor),
                label,
                event.occurrences,
                event.conclusion
            )
            .unwrap();
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
            "analyze" => shell::analyze!(tori, self, args, out),
            "hands" => {
                shell::empty(command, args)?;
                hands(self, out);
                Ok(())
            }
            _ => Err(shell::unknown::<Self>(command)),
        }
    }
}
fn hands(game: &Game, out: &mut String) {
    for side in [Side::Sente, Side::Gote] {
        let counts: Vec<_> = Face::HANDS
            .into_iter()
            .map(|f| format!("{}={}", f.code(), game.position().hand(side).count(f)))
            .collect();
        writeln!(out, "hands {} {}", side_text(side), counts.join(" ")).unwrap();
    }
}

shell::sides!(Sente => "sente", Gote => "gote");
