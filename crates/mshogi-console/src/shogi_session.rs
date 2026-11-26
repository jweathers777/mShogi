//! Explicit human console adapter; native game owns all rules and admission.
use crate::{
    diagnostics,
    shell::{self, Variant, diagnostic},
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
  perft <0..2> [visits] | divide <1..2> [visits]  Position-only local legality; ignores outcomes
  help | quit
USI actions: 7g7f, 5c5b+, P*5e. Readable long notation: S:Pawn[7g->7f;keep],
S:Pawn[5c->5b;promote], S:Pawn[hand->5e;drop]. Always include source; keep
retains existing promoted identity. Readable labels are output, USI is input.
Compact board: uppercase = Sente, lowercase = Gote; K R B G S N L P.
Prefix + marks promotion (+P/+p); two-character cells keep piece/file alignment.";

impl Variant for Game {
    type Undo = GameUndo;
    type Command = GameCommand;
    const NAME: &'static str = "shogi";
    const HELP: &'static str = HELP;
    const MOVE_SYNTAX: &'static str = "USI-action";
    const LOADED: &'static str = "loaded complete native record";
    fn position_command(&self, rest: &str) -> Result<Self, String> {
        let game = if rest == "startpos" {
            Game::new()
        } else {
            let ("sfen", rest) = shell::words(rest) else {
                return Err("usage: position startpos | position sfen full|reduced <SFEN>".into());
            };
            let (category, text) = shell::words(rest);
            let category = match category {
                "full" => RootCategory::FullMaterialDiagram,
                "reduced" => RootCategory::ReducedMaterialDiagram,
                _ => return Err("SFEN requires full|reduced material category".into()),
            };
            let position = Position::from_sfen(text).map_err(diagnostic)?;
            Game::from_root(
                GameRoot::import(position, category, CounterSource::Supplied)
                    .map_err(root_error)?,
            )
        };
        Ok(game)
    }
    fn after_position(&self, out: &mut String) -> Result<(), String> {
        self.write_status(out)?;
        writeln!(
            out,
            "history-segment earlier-history {}",
            if self.root().has_complete_initial_history() {
                "complete"
            } else {
                "unknown"
            }
        )
        .unwrap();
        Ok(())
    }
    fn record(&self) -> Result<CanonicalRecord, String> {
        Ok(CanonicalRecord::shogi(GameRecord::from_game(self)))
    }
    fn restore(record: CanonicalRecord) -> Result<(Self, Vec<GameUndo>), String> {
        let CanonicalRecord::Shogi(record) = record else {
            return Err("record is not shogi; session preserved".into());
        };
        let game = record.to_game().map_err(diagnostic)?;
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
                    .map_err(game_error)?,
            );
        }
        if replay != game {
            return Err("native replay disagrees with complete recorded game".into());
        }
        Ok((replay, undo))
    }
    fn play(&mut self, text: &str, out: &mut String) -> Result<GameUndo, String> {
        let action: Action = text.parse().map_err(diagnostic)?;
        let label = readable(self.position(), action)?;
        let undo = self
            .submit(GameRequest {
                expected: self.context(),
                actor: self.position().side_to_move(),
                command: GameCommand::Play(action),
            })
            .map_err(game_error)?;
        writeln!(out, "played {action} {label}").unwrap();
        Ok(undo)
    }
    fn submit_actor(
        &mut self,
        args: &[&str],
        command: GameCommand,
        _out: &mut String,
    ) -> Result<GameUndo, String> {
        self.submit(GameRequest {
            expected: self.context(),
            actor: side(args)?,
            command,
        })
        .map_err(game_error)
    }
    fn undo_event(&mut self, undo: GameUndo) -> Result<(), String> {
        self.undo(undo)
            .map_err(|_| "native undo token rejected".into())
    }
    fn actor_command(command: &str) -> Option<GameCommand> {
        match command {
            "resign" => Some(GameCommand::Resign),
            "declare" => Some(GameCommand::DeclareEnteringKing),
            _ => None,
        }
    }
    fn write_status(&self, out: &mut String) -> Result<(), String> {
        writeln!(out, "sfen {}", self.position().to_sfen()).unwrap();
        writeln!(
            out,
            "side {} declared-completed {} known-events {} conclusion {:?}",
            side_text(self.position().side_to_move()),
            self.position().completed_actions(),
            self.events().len(),
            self.conclusion()
        )
        .unwrap();
        Ok(())
    }
    fn board(&self, out: &mut String) {
        compact_board(self.position(), out);
        hands(self, out);
    }
    fn legal(&self, out: &mut String) -> Result<(), String> {
        let actions = self.position().legal_actions();
        writeln!(
            out,
            "legal-local {} (game admission also checks conclusion)",
            actions.len()
        )
        .unwrap();
        for action in actions {
            writeln!(out, "{action} {}", readable(self.position(), action)?).unwrap();
        }
        Ok(())
    }
    fn history(&self, out: &mut String) -> Result<(), String> {
        writeln!(out, "profile {PROFILE_ID} edition {PROFILE_EDITION}").unwrap();
        writeln!(
            out,
            "root {:?} counter {:?} complete-initial-history {} root-sfen {}",
            self.root().category(),
            self.root().counter_source(),
            self.root().has_complete_initial_history(),
            self.root().position().to_sfen()
        )
        .unwrap();
        writeln!(
            out,
            "events {} known-board-actions {} occurrences {:?}",
            self.events().len(),
            CanonicalRecord::shogi(GameRecord::from_game(self)).known_board_actions(),
            self.occurrences()
        )
        .unwrap();
        for event in self.events() {
            writeln!(out,"event {} actor {} command {:?} check {:?} declaration {:?} conclusion {:?} after {}",event.index,side_text(event.actor),event.command,event.gave_check,event.declaration,event.conclusion,event.after_sfen).unwrap();
        }
        Ok(())
    }
    fn perft(&self, command: &str, args: &[&str], out: &mut String) -> Result<(), String> {
        diagnostics::execute(
            self.position(),
            command,
            args,
            MAX_DIAGNOSTIC_DEPTH,
            diagnostics::Style::Local,
            out,
            |action| Ok(action.to_string()),
        )
    }
    fn extra(&self, command: &str, args: &[&str], out: &mut String) -> Result<(), String> {
        match command {
            "hands" => {
                shell::empty(command, args)?;
                hands(self, out);
                Ok(())
            }
            "sfen" => {
                shell::empty(command, args)?;
                writeln!(out, "sfen {}", self.position().to_sfen()).unwrap();
                Ok(())
            }
            "eligibility" => {
                let actor = side(args)?;

                let evidence = self.declaration_eligibility(actor);
                writeln!(
                    out,
                    "eligibility {} {} {evidence:?} (advisory; no command submitted)",
                    side_text(actor),
                    evidence.eligible()
                )
                .unwrap();
                Ok(())
            }
            _ => Err(shell::unknown::<Self>(command)),
        }
    }
}
fn hands(game: &Game, out: &mut String) {
    for side in [Side::Sente, Side::Gote] {
        let hand = game.position().hand(side);
        let kinds: Vec<_> = PieceKind::HAND_KINDS
            .into_iter()
            .map(|kind| format!("{}:{}", kind_text(kind), hand.count(kind)))
            .collect();
        writeln!(out, "hands {} {}", side_text(side), kinds.join(" ")).unwrap();
    }
}

fn root_error(error: RootError) -> String {
    match error {
        RootError::StandardRequiresInitialRoot => "standard root requires the initial position",
        RootError::CategoryMismatch => "material does not match the root category",
        RootError::CounterProvenance => "invalid root counter source",
        RootError::NonmovingKingAttacked => "the nonmoving king is attacked",
    }
    .into()
}
fn game_error(error: GameError) -> String {
    match error {
        GameError::StaleContext => "stale game context".into(),
        GameError::Concluded => "game has concluded".into(),
        GameError::WrongActor => "wrong actor for board action".into(),
        GameError::IllegalAction(error) => diagnostic(error),
        GameError::MutualCheckInvariant(_) => "native mutual-check invariant failed".into(),
    }
}

shell::sides!(Sente => "sente", Gote => "gote");
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
    shell::shogi_board(out, 9, 2, |file, rank| {
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
    });
}
