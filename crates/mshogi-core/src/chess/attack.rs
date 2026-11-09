//! Geometric chess attack queries for `mshogi.chess.position.v1`.
//!
//! An attack is decided by piece kind, color, board edges and current
//! occupancy only. It is not legal-move membership: pinned pieces and kings
//! still attack, a target occupied by the attacker's own piece is attacked
//! (defended), and no castling or en passant effect is considered.

use super::{
    Action, Color, Piece, PieceKind, Position, Square,
    attack_cache::{DIRECTIONS, KINGS, KNIGHTS, PAWNS, RAYS, kind_index, nearest},
};

impl Position {
    pub(super) fn king_square(&self, side: Color) -> Square {
        let kings =
            self.attacks.kinds[kind_index(PieceKind::King)] & self.attacks.colors[side.index()];
        debug_assert_eq!(kings.count_ones(), 1);
        Square(kings.trailing_zeros() as u8)
    }
    fn queens_in_zone(&self, side: Color) -> u64 {
        let king = self.king_square(side.opponent());
        let mut queens =
            self.attacks.kinds[kind_index(PieceKind::Queen)] & self.attacks.colors[side.index()];
        let mut near = 0;
        while queens != 0 {
            let sq = Square(queens.trailing_zeros() as u8);
            queens &= queens - 1;
            if sq
                .file()
                .abs_diff(king.file())
                .max(sq.rank().abs_diff(king.rank()))
                <= 4
            {
                near |= 1 << sq.index();
            }
        }
        near
    }
    /// Geometric heuristic gate: a queen is within four king steps of the
    /// opposing king. This is neither an attack, legal move nor Game outcome.
    pub fn queen_in_attack_zone(&self, side: Color) -> bool {
        self.queens_in_zone(side) != 0
    }
    /// A counter-independent position-legal supported queen contact mate from
    /// a queen in the four-step attack zone, if found. Absence proves nothing
    /// about other mating patterns. Native resolution and replies confirm it;
    /// Game history/claims and actual counter representability still require
    /// ordinary Game admission. This deliberately finds only contact patterns.
    pub fn queen_contact_mate(&self) -> Option<Action> {
        self.contact_mate_for(self.side_to_move)
    }
    /// Whether passing this quiet turn would allow the opponent a supported
    /// queen contact mate from that attack zone. This query cannot conclude a
    /// Game; search may use it only to investigate real moves more carefully.
    pub fn has_queen_contact_mate_threat(&self) -> bool {
        self.queen_in_attack_zone(self.side_to_move.opponent())
            && !self.in_check()
            && self
                .contact_mate_for(self.side_to_move.opponent())
                .is_some()
    }
    fn contact_mate_for(&self, side: Color) -> Option<Action> {
        let queens = self.queens_in_zone(side);
        if queens == 0 {
            return None;
        }
        let king = self.king_square(side.opponent());
        let mut targets = KINGS[king.index()] & !self.attacks.colors[side.index()];
        let mut base = self.clone();
        if side != base.side_to_move {
            base.en_passant_target = None;
        }
        base.side_to_move = side;
        base.halfmove_clock = 0;
        base.fullmove_number = 1;
        while targets != 0 {
            let target = Square(targets.trailing_zeros() as u8);
            targets &= targets - 1;
            let attackers = base.attacker_mask(target, side);
            let mut origins = attackers & queens;
            while origins != 0 {
                let from = Square(origins.trailing_zeros() as u8);
                origins &= origins - 1;
                let action = Action {
                    from,
                    to: target,
                    promotion: None,
                };
                if let Ok(after) = base.checked_after(action)
                    // Moving the queen can uncover her supporting slider. The
                    // pre-move attack map cannot safely reject that support.
                    && after.attacker_mask(target, side) != 0
                    && after.in_check()
                    && !after.has_legal_action()
                {
                    return Some(action);
                }
            }
        }
        None
    }
    /// Geometric squares attacked by the piece on `from`, as an a1-bit0 map.
    ///
    /// This has exactly the attack/defence semantics of [`Position::attackers`]:
    /// slider rays include their first occupied square and stop there, friendly
    /// blockers are defended, and pins do not remove attacks. Empty squares
    /// return zero. Castling, pawn pushes and en passant are not attack effects.
    /// The map is read-only position data, never permission to play an action.
    pub fn attacks_from(&self, from: Square) -> u64 {
        let Some(piece) = self.piece_at(from) else {
            return 0;
        };
        self.attacks_as(piece, from)
    }

    /// Geometric attack map for a hypothetical `piece` at `from`, using this
    /// board's blockers. The origin occupant is ignored; every ray includes its
    /// first occupied target. This never changes the board, resolves a move or
    /// establishes legal admission. It shares all geometry with `attacks_from`.
    pub fn attacks_as(&self, piece: Piece, from: Square) -> u64 {
        match piece.kind {
            PieceKind::Pawn => PAWNS[piece.color.index()][from.index()],
            PieceKind::Knight => KNIGHTS[from.index()],
            PieceKind::King => KINGS[from.index()],
            kind => {
                let mut attacked = 0;
                for (dir, &(df, dr)) in DIRECTIONS.iter().enumerate() {
                    let diagonal = df != 0 && dr != 0;
                    if kind == PieceKind::Bishop && !diagonal || kind == PieceKind::Rook && diagonal
                    {
                        continue;
                    }
                    let ray = RAYS[from.index()][dir];
                    let blockers = ray & self.attacks.occupied();
                    attacked |= if blockers == 0 {
                        ray
                    } else {
                        ray & !RAYS[nearest(blockers, dir)][dir]
                    };
                }
                attacked
            }
        }
    }

    /// Squares holding a piece of color `by` that attacks `target`, a1 to h8.
    ///
    /// Pawns attack the two forward diagonals for their color whether or not
    /// the target is occupied; they never attack straight ahead. Knights and
    /// kings attack their fixed offsets. Bishops, rooks and queens attack along
    /// their lines up to and including the first occupied square in each
    /// direction. Moves never wrap across a board edge. Whether the attacker is
    /// pinned, whose piece stands on `target`, and whether moving the attacker
    /// would be legal are all irrelevant. A piece never attacks its own square.
    ///
    /// ```
    /// use mshogi_core::chess::{Color, Position, Square};
    /// let position = Position::from_fen("4k3/8/8/8/8/8/8/R3K3 w - - 0 1")?;
    /// let a8 = Square::new(0, 7).unwrap();
    /// let attackers: Vec<Square> = position.attackers(a8, Color::White).collect();
    /// assert_eq!(attackers, [Square::new(0, 0).unwrap()]);
    /// assert!(!position.is_square_attacked(a8, Color::Black));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn attackers(&self, target: Square, by: Color) -> impl Iterator<Item = Square> + '_ {
        (0..64u8)
            .map(Square)
            .filter(move |&from| self.piece_attacks(from, target, by))
    }

    /// Whether any piece of color `by` attacks `target`; see [`Position::attackers`].
    ///
    /// For example, the side to move is in check exactly when its king's square
    /// is attacked by the opponent. This query alone does not decide legality.
    pub fn is_square_attacked(&self, target: Square, by: Color) -> bool {
        self.attacker_bits(target, by, true) != 0
    }

    pub(super) fn attacker_mask(&self, target: Square, by: Color) -> u64 {
        self.attacker_bits(target, by, false)
    }

    fn attacker_bits(&self, target: Square, by: Color, first_only: bool) -> u64 {
        // Cached occupancy is derived by the same native effect-cell setter.
        // Leapers are reverse maps; a slider is visible iff it is the nearest
        // occupied square on its ray. Pins and target ownership do not enter.
        let ours = self.attacks.colors[by.index()];
        let pieces = |kind| ours & self.attacks.kinds[kind_index(kind)];
        let mut found = (PAWNS[by.opponent().index()][target.index()] & pieces(PieceKind::Pawn))
            | (KNIGHTS[target.index()] & pieces(PieceKind::Knight))
            | (KINGS[target.index()] & pieces(PieceKind::King));
        if first_only && found != 0 {
            return found;
        }
        let queens = pieces(PieceKind::Queen);
        let diagonal = queens | pieces(PieceKind::Bishop);
        let straight = queens | pieces(PieceKind::Rook);
        for (dir, &(df, dr)) in DIRECTIONS.iter().enumerate() {
            let attackers = if df != 0 && dr != 0 {
                diagonal
            } else {
                straight
            };
            if attackers == 0 {
                continue;
            }
            let blockers = RAYS[target.index()][dir] & self.attacks.occupied();
            if blockers != 0 && attackers & (1 << nearest(blockers, dir)) != 0 {
                found |= 1 << nearest(blockers, dir);
                if first_only {
                    return found;
                }
            }
        }
        found
    }

    fn piece_attacks(&self, from: Square, target: Square, by: Color) -> bool {
        let Some(piece) = self.piece_at(from) else {
            return false;
        };
        if piece.color != by {
            return false;
        }
        let df = target.file() as i8 - from.file() as i8;
        let dr = target.rank() as i8 - from.rank() as i8;
        let diagonal = df != 0 && df.abs() == dr.abs();
        let orthogonal = (df == 0) != (dr == 0);
        match piece.kind {
            PieceKind::Pawn => df.abs() == 1 && dr == if by == Color::White { 1 } else { -1 },
            PieceKind::Knight => matches!((df.abs(), dr.abs()), (1, 2) | (2, 1)),
            PieceKind::King => df.abs().max(dr.abs()) == 1,
            PieceKind::Bishop => diagonal && self.ray_is_clear(from, target),
            PieceKind::Rook => orthogonal && self.ray_is_clear(from, target),
            PieceKind::Queen => (diagonal || orthogonal) && self.ray_is_clear(from, target),
        }
    }

    // Called only for aligned, distinct, bounded squares. Every intervening
    // coordinate lies on the segment between them and is therefore in 0..8.
    pub(super) fn ray_is_clear(&self, from: Square, target: Square) -> bool {
        let step_file = (target.file() as i8 - from.file() as i8).signum();
        let step_rank = (target.rank() as i8 - from.rank() as i8).signum();
        let mut file = from.file() as i8 + step_file;
        let mut rank = from.rank() as i8 + step_rank;
        while (file, rank) != (target.file() as i8, target.rank() as i8) {
            if self.board[rank as usize * 8 + file as usize].is_some() {
                return false;
            }
            file += step_file;
            rank += step_rank;
        }
        true
    }
}
