//! Checked chess transitions, including castling and en passant, and exact
//! local-state restoration.

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
/// action type. Partial UI selection is not an applied action. Castling names
/// the king's two-file move (e.g. e1 to g1); en passant names the capturing
/// pawn's diagonal move onto the raw target. The board determines the rook
/// move and the off-destination victim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Action {
    /// Starting square; the board determines the moving piece.
    pub from: Square,
    /// Destination; the board determines any capture.
    pub to: Square,
    /// Required on a last-rank pawn action, forbidden otherwise.
    pub promotion: Option<Promotion>,
}

/// A rejected action; rejection leaves every position field unchanged.
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
    /// A castling-shaped king action given to the ordinary-only boundary; use
    /// [`Position::apply`]. No legality is implied.
    CastlingNotOrdinary,
    /// An en passant capture given to the ordinary-only boundary; use
    /// [`Position::apply`]. No legality is implied.
    EnPassantNotOrdinary,
    /// The castling right is absent, its home rook is missing, or a square
    /// between the king and that rook is occupied.
    CastlingUnavailable,
    /// The castling king is in check or would cross an attacked square.
    /// An attacked destination is reported as [`ActionError::IllegalPosition`].
    CastlingKingAttacked,
    /// A counter increment would exceed the profile's u32 domain.
    ///
    /// This is a representation failure, not a finding that the action is
    /// illegal: a legal action (see [`Position::legal_actions`]) can be refused
    /// this way because its resulting state cannot be represented. The whole
    /// position, including board, side, rights, raw target and counters, is
    /// left unchanged.
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
            Self::CastlingNotOrdinary => "castling is not an ordinary action",
            Self::EnPassantNotOrdinary => "en passant is not an ordinary action",
            Self::CastlingUnavailable => "castling right, rook or empty path is missing",
            Self::CastlingKingAttacked => "castling king is in check or crosses an attacked square",
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

/// Opaque in-memory restoration record for one successful action.
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

// Only resolution constructs this private, trusted input. Captures, the placed
// piece, any castling rook move and counter/EP effects are derived from the
// current board and the checked action.
struct Resolved {
    action: Action,
    moving: Piece,
    placed: Piece,
    // Square and piece removed; differs from `action.to` only for en passant.
    captured: Option<(Square, Piece)>,
    // Castling rook origin and destination.
    rook: Option<(Square, Square)>,
    en_passant_target: Option<Square>,
    halfmove_clock: u32,
    fullmove_number: u32,
}

impl Position {
    /// Check and apply one complete action atomically: an ordinary move or
    /// capture, a promotion, castling or en passant.
    ///
    /// Validates ownership, movement/path, promotion, castling right/path and
    /// king origin/transit safety, en passant availability, counters, and the
    /// resulting king safety/local invariants before publishing any change. Own
    /// king safety is judged after **all** effects, including rook movement and
    /// removal of the en passant victim. No move generator or match-adjudication
    /// policy is invoked. Returns an exact local-state restoration record.
    ///
    /// An otherwise legal action fails with [`ActionError::CounterOverflow`]
    /// when a required counter increment cannot be represented in u32; that is
    /// a representation failure, not illegality, and changes nothing.
    ///
    /// ```
    /// use mshogi_core::chess::{Action, Position, Square};
    /// let mut position = Position::from_fen("r3k2r/8/8/8/8/8/8/R3K2R w KQkq - 0 1")?;
    /// let before = position.clone();
    /// let record = position.apply(Action {
    ///     from: Square::new(4, 0).unwrap(), // e1
    ///     to: Square::new(6, 0).unwrap(),   // g1
    ///     promotion: None,
    /// })?;
    /// assert_eq!(position.to_fen(), "r3k2r/8/8/8/8/8/8/R4RK1 b kq - 1 1");
    /// position.undo(&record)?;
    /// assert_eq!(position, before);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn apply(&mut self, action: Action) -> Result<Undo, ActionError> {
        let resolved = self.resolve(action, true)?;
        self.publish(resolved)
    }

    /// Check and apply one ordinary move/capture or explicit promotion atomically.
    ///
    /// The ordinary-only subset of [`Position::apply`]: castling-shaped king
    /// actions and en passant captures are rejected as
    /// [`ActionError::CastlingNotOrdinary`] and
    /// [`ActionError::EnPassantNotOrdinary`] without judging their legality.
    /// Everything else, including king safety, is checked exactly as by `apply`.
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
        let resolved = self.resolve(action, false)?;
        self.publish(resolved)
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

    fn publish(&mut self, resolved: Resolved) -> Result<Undo, ActionError> {
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

    fn resolve(&self, action: Action, special: bool) -> Result<Resolved, ActionError> {
        use ActionError::*;
        let moving = self.piece_at(action.from).ok_or(EmptySource)?;
        if moving.color != self.side_to_move {
            return Err(WrongSide);
        }
        if action.from == action.to {
            return Err(SameSquare);
        }
        let df = action.to.file() as i8 - action.from.file() as i8;
        let dr = action.to.rank() as i8 - action.from.rank() as i8;
        let home_rank = if moving.color == Color::White { 0 } else { 7 };
        if moving.kind == PieceKind::King
            && action.from.file() == 4
            && action.from.rank() == home_rank
            && dr == 0
            && df.abs() == 2
        {
            if !special {
                return Err(CastlingNotOrdinary);
            }
            return self.resolve_castling(action, moving, df > 0);
        }
        let mut captured = self.piece_at(action.to).map(|piece| (action.to, piece));
        if captured.is_some_and(|(_, piece)| piece.color == moving.color) {
            return Err(FriendlyDestination);
        }
        if captured.is_some_and(|(_, piece)| piece.kind == PieceKind::King) {
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
                        if !special {
                            return Err(EnPassantNotOrdinary);
                        }
                        // FEN and transition invariants place the opposing
                        // double-pushed pawn beside the mover; check anyway.
                        let victim_square = Square(action.from.rank() * 8 + action.to.file());
                        let victim = self.piece_at(victim_square).filter(|piece| {
                            piece.color != moving.color && piece.kind == PieceKind::Pawn
                        });
                        captured = victim.map(|piece| (victim_square, piece));
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
        let resets = moving.kind == PieceKind::Pawn || captured.is_some();
        let (halfmove_clock, fullmove_number) = self.next_counters(resets)?;
        Ok(Resolved {
            action,
            moving,
            placed: Piece {
                color: moving.color,
                kind: placed_kind,
            },
            captured,
            rook: None,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
        })
    }

    // The king moves two files toward its home rook, which jumps to the square
    // the king crossed (FIDE 3.8.a.ii, orthodox starting squares only).
    fn resolve_castling(
        &self,
        action: Action,
        moving: Piece,
        kingside: bool,
    ) -> Result<Resolved, ActionError> {
        use ActionError::*;
        let rights = self.castling_rights;
        let has_right = if kingside {
            rights.kingside(moving.color)
        } else {
            rights.queenside(moving.color)
        };
        let rank = action.from.rank();
        let at = |file: u8| Square(rank * 8 + file);
        let (rook_from, rook_to, between): (Square, Square, &[u8]) = if kingside {
            (at(7), at(5), &[5, 6])
        } else {
            (at(0), at(3), &[1, 2, 3])
        };
        let home_rook = Piece {
            color: moving.color,
            kind: PieceKind::Rook,
        };
        if !has_right
            || self.piece_at(rook_from) != Some(home_rook)
            || between
                .iter()
                .any(|&file| self.piece_at(at(file)).is_some())
        {
            return Err(CastlingUnavailable);
        }
        if action.promotion.is_some() {
            return Err(UnexpectedPromotion);
        }
        // Origin and transit are judged here; the destination is judged on the
        // resulting board by the same structural king-safety check as every action.
        // The square the king crosses is exactly the rook's destination.
        let opponent = moving.color.opponent();
        if self.is_square_attacked(action.from, opponent)
            || self.is_square_attacked(rook_to, opponent)
        {
            return Err(CastlingKingAttacked);
        }
        let (halfmove_clock, fullmove_number) = self.next_counters(false)?;
        Ok(Resolved {
            action,
            moving,
            placed: moving,
            captured: None,
            rook: Some((rook_from, rook_to)),
            en_passant_target: None,
            halfmove_clock,
            fullmove_number,
        })
    }

    fn next_counters(&self, resets_halfmove: bool) -> Result<(u32, u32), ActionError> {
        let halfmove_clock = if resets_halfmove {
            0
        } else {
            self.halfmove_clock
                .checked_add(1)
                .ok_or(ActionError::CounterOverflow)?
        };
        let fullmove_number = if self.side_to_move == Color::Black {
            self.fullmove_number
                .checked_add(1)
                .ok_or(ActionError::CounterOverflow)?
        } else {
            self.fullmove_number
        };
        Ok((halfmove_clock, fullmove_number))
    }

    // Trusted private primitive: resolution has established board effects,
    // geometry and counter bounds. Runs only on a candidate; the checked public
    // callers validate its resulting local invariants before publishing it.
    fn apply_resolved(&mut self, resolved: Resolved) {
        self.board[resolved.action.from.index()] = None;
        if let Some((square, piece)) = resolved.captured {
            self.board[square.index()] = None;
            if piece.kind == PieceKind::Rook {
                self.remove_home_rook_right(piece.color, square);
            }
        }
        self.board[resolved.action.to.index()] = Some(resolved.placed);
        if let Some((rook_from, rook_to)) = resolved.rook {
            self.board[rook_to.index()] = self.board[rook_from.index()].take();
        }
        if resolved.moving.kind == PieceKind::King {
            let color = resolved.moving.color.index();
            self.castling_rights.kingside[color] = false;
            self.castling_rights.queenside[color] = false;
        }
        if resolved.moving.kind == PieceKind::Rook {
            self.remove_home_rook_right(resolved.moving.color, resolved.action.from);
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
