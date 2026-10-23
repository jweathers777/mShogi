//! Checked ordinary chess transitions and exact local-state restoration.

use std::{error::Error, fmt};

use super::{Color, FenError, Piece, PieceKind, Position, Square};

/// One of the four mandatory choices for a pawn reaching its last rank.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Promotion {
    /// Promote to a queen.
    Queen,
    /// Promote to a rook.
    Rook,
    /// Promote to a bishop.
    Bishop,
    /// Promote to a knight.
    Knight,
}

impl Promotion {
    const fn piece_kind(self) -> PieceKind {
        match self {
            Self::Queen => PieceKind::Queen,
            Self::Rook => PieceKind::Rook,
            Self::Bishop => PieceKind::Bishop,
            Self::Knight => PieceKind::Knight,
        }
    }
}

/// One complete chess player choice, not a client-supplied list of effects.
///
/// This is intentionally chess-specific, not a universal multi-route/variant
/// action type. Partial UI selection is not an applied action. The current
/// checked entry point supports ordinary moves/captures and promotions only.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Action {
    /// Starting square; the board determines the moving piece.
    pub from: Square,
    /// Destination; the board determines any ordinary capture.
    pub to: Square,
    /// Required on a last-rank pawn action, forbidden otherwise.
    pub promotion: Option<Promotion>,
}

/// A rejected ordinary action; rejection leaves every position field unchanged.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ActionError {
    /// No piece occupies the source square.
    EmptySource,
    /// The source piece does not belong to the side to move.
    WrongSide,
    /// Source and destination must differ; a null move is not a player action.
    SameSquare,
    /// A piece of the moving side occupies the destination.
    FriendlyDestination,
    /// A king cannot be captured.
    KingCapture,
    /// Movement shape, occupancy, pawn direction or intervening path is invalid.
    IllegalGeometry,
    /// A last-rank pawn action requires an explicit choice.
    MissingPromotion,
    /// A promotion choice was supplied for another piece or destination rank.
    UnexpectedPromotion,
    /// Standard castling execution is deferred to MSH-8; no legality is implied.
    UnsupportedCastling,
    /// En passant execution is deferred to MSH-8; no legality is implied.
    UnsupportedEnPassant,
    /// A counter increment would exceed the profile's u32 domain.
    CounterOverflow,
    /// The resulting local position is invalid, including an unsafe moving king.
    IllegalPosition(FenError),
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let reason = match self {
            Self::EmptySource => "empty source square",
            Self::WrongSide => "source piece belongs to the other side",
            Self::SameSquare => "source and destination are identical",
            Self::FriendlyDestination => "destination contains a friendly piece",
            Self::KingCapture => "king capture is forbidden",
            Self::IllegalGeometry => "invalid movement or blocked path",
            Self::MissingPromotion => "last-rank pawn action needs a promotion choice",
            Self::UnexpectedPromotion => "promotion is not allowed for this action",
            Self::UnsupportedCastling => "castling execution is not implemented",
            Self::UnsupportedEnPassant => "en passant execution is not implemented",
            Self::CounterOverflow => "position counter would overflow",
            Self::IllegalPosition(error) => return write!(f, "action rejected: {error}"),
        };
        write!(f, "action rejected: {reason}")
    }
}

impl Error for ActionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::IllegalPosition(error) => Some(error),
            _ => None,
        }
    }
}

/// Opaque in-memory restoration record for one successful ordinary action.
///
/// Full before/after snapshots deliberately favor clarity over compression.
/// There are no derived caches yet. This is not a serialized game record or a
/// proof of history/origin. Callers keep records in LIFO order and correct game
/// context. Restoration checks complete saved-state equality against `after`;
/// an identical post-state in another context is not distinguished by this API.
#[derive(Debug)]
pub struct Undo {
    before: Position,
    after: Position,
}

/// A restoration record does not match the current complete position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UndoError {
    /// Wrong, out-of-order or already-restored post-state; no fields were changed.
    PositionMismatch,
}

impl fmt::Display for UndoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("undo rejected: current position does not match the record's post-state")
    }
}

impl Error for UndoError {}

// Only resolution constructs this private, trusted input. Captures/placed piece
// and counter/EP effects are derived from the current board and checked action.
struct ResolvedOrdinary {
    action: Action,
    moving: Piece,
    captured: Option<Piece>,
    placed: Piece,
    en_passant_target: Option<Square>,
    halfmove_clock: u32,
    fullmove_number: u32,
}

impl Position {
    /// Check and apply one ordinary move/capture or explicit promotion atomically.
    ///
    /// Validates ownership, movement/path, promotion, counters and the resulting
    /// king safety/local invariants before publishing any change. Castling and
    /// en passant execution return explicit unsupported errors. No move generator
    /// or match-adjudication policy is invoked (e.g. bare-kings diagnostics remain
    /// usable). Returns an exact local-state restoration record on success.
    ///
    /// ```
    /// use mshogi_core::chess::{Action, Position, Square};
    /// let mut position = Position::from_fen(
    ///     "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    /// )?;
    /// let before = position.clone();
    /// let record = position.apply_ordinary(Action {
    ///     from: Square::new(4, 1).unwrap(), // e2
    ///     to: Square::new(4, 3).unwrap(),   // e4
    ///     promotion: None,
    /// })?;
    /// assert_eq!(position.en_passant_target(), Square::new(4, 2));
    /// position.undo(&record)?;
    /// assert_eq!(position, before);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn apply_ordinary(&mut self, action: Action) -> Result<Undo, ActionError> {
        let resolved = self.resolve_ordinary(action)?;
        let mut candidate = self.clone();
        candidate.apply_resolved(resolved);
        candidate
            .validate_structure()
            .map_err(ActionError::IllegalPosition)?;
        let undo = Undo {
            before: self.clone(),
            after: candidate.clone(),
        };
        *self = candidate;
        Ok(undo)
    }

    /// Restore an exact prior position only when its complete post-state matches.
    ///
    /// Failure is atomic. The borrowed token is reusable only if its matching
    /// post-state is reached again; immediate repeated use fails. Match-history
    /// bookkeeping and LIFO/context ownership remain the caller's responsibility.
    pub fn undo(&mut self, record: &Undo) -> Result<(), UndoError> {
        if *self != record.after {
            return Err(UndoError::PositionMismatch);
        }
        *self = record.before.clone();
        Ok(())
    }

    fn resolve_ordinary(&self, action: Action) -> Result<ResolvedOrdinary, ActionError> {
        use ActionError::*;
        let moving = self.piece_at(action.from).ok_or(EmptySource)?;
        if moving.color != self.side_to_move {
            return Err(WrongSide);
        }
        if action.from == action.to {
            return Err(SameSquare);
        }
        let captured = self.piece_at(action.to);
        let df = action.to.file() as i8 - action.from.file() as i8;
        let dr = action.to.rank() as i8 - action.from.rank() as i8;
        let home_rank = if moving.color == Color::White { 0 } else { 7 };
        if moving.kind == PieceKind::King
            && action.from.file() == 4
            && action.from.rank() == home_rank
            && dr == 0
            && df.abs() == 2
        {
            return Err(UnsupportedCastling);
        }
        if captured.is_some_and(|piece| piece.color == moving.color) {
            return Err(FriendlyDestination);
        }
        if captured.is_some_and(|piece| piece.kind == PieceKind::King) {
            return Err(KingCapture);
        }

        let mut en_passant_target = None;
        let diagonal = df != 0 && df.abs() == dr.abs();
        let orthogonal = (df == 0) != (dr == 0);
        let geometry_ok = match moving.kind {
            PieceKind::Pawn => {
                let forward = if moving.color == Color::White { 1 } else { -1 };
                let start_rank = if moving.color == Color::White { 1 } else { 6 };
                if df.abs() == 1 && dr == forward {
                    if captured.is_none() && self.en_passant_target == Some(action.to) {
                        return Err(UnsupportedEnPassant);
                    }
                    captured.is_some()
                } else if df == 0 && captured.is_none() {
                    if dr == forward {
                        true
                    } else if action.from.rank() == start_rank && dr == 2 * forward {
                        let middle_rank = (action.from.rank() + action.to.rank()) / 2;
                        let middle = Square(middle_rank * 8 + action.from.file());
                        en_passant_target = Some(middle);
                        self.piece_at(middle).is_none()
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            PieceKind::Knight => matches!((df.abs(), dr.abs()), (1, 2) | (2, 1)),
            PieceKind::Bishop => diagonal && self.ray_is_clear(action.from, action.to),
            PieceKind::Rook => orthogonal && self.ray_is_clear(action.from, action.to),
            PieceKind::Queen => {
                (diagonal || orthogonal) && self.ray_is_clear(action.from, action.to)
            }
            PieceKind::King => df.abs().max(dr.abs()) == 1,
        };
        if !geometry_ok {
            return Err(IllegalGeometry);
        }

        let last_rank = if moving.color == Color::White { 7 } else { 0 };
        let placed_kind = if moving.kind == PieceKind::Pawn && action.to.rank() == last_rank {
            action.promotion.ok_or(MissingPromotion)?.piece_kind()
        } else {
            if action.promotion.is_some() {
                return Err(UnexpectedPromotion);
            }
            moving.kind
        };
        let halfmove_clock = if moving.kind == PieceKind::Pawn || captured.is_some() {
            0
        } else {
            self.halfmove_clock.checked_add(1).ok_or(CounterOverflow)?
        };
        let fullmove_number = if moving.color == Color::Black {
            self.fullmove_number.checked_add(1).ok_or(CounterOverflow)?
        } else {
            self.fullmove_number
        };
        Ok(ResolvedOrdinary {
            action,
            moving,
            captured,
            placed: Piece {
                color: moving.color,
                kind: placed_kind,
            },
            en_passant_target,
            halfmove_clock,
            fullmove_number,
        })
    }

    // Trusted private primitive: resolve_ordinary has established board effects,
    // geometry and counter bounds. Runs only on a candidate; the checked public
    // caller validates its resulting local invariants before publishing it.
    fn apply_resolved(&mut self, resolved: ResolvedOrdinary) {
        self.board[resolved.action.from.index()] = None;
        self.board[resolved.action.to.index()] = Some(resolved.placed);
        if resolved.moving.kind == PieceKind::King {
            let color = resolved.moving.color.index();
            self.castling_rights.kingside[color] = false;
            self.castling_rights.queenside[color] = false;
        }
        if resolved.moving.kind == PieceKind::Rook {
            self.remove_home_rook_right(resolved.moving.color, resolved.action.from);
        }
        if let Some(Piece {
            color,
            kind: PieceKind::Rook,
        }) = resolved.captured
        {
            self.remove_home_rook_right(color, resolved.action.to);
        }
        self.en_passant_target = resolved.en_passant_target;
        self.halfmove_clock = resolved.halfmove_clock;
        self.fullmove_number = resolved.fullmove_number;
        self.side_to_move = self.side_to_move.opponent();
    }

    fn remove_home_rook_right(&mut self, color: Color, square: Square) {
        let home_rank = if color == Color::White { 0 } else { 7 };
        if square.rank() == home_rank {
            match square.file() {
                0 => self.castling_rights.queenside[color.index()] = false,
                7 => self.castling_rights.kingside[color.index()] = false,
                _ => {}
            }
        }
    }
}
