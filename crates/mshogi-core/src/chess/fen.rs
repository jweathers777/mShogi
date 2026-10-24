//! FEN is an adapter for the documented local-analysis profile, not a game record.

use std::{error::Error, fmt, str::FromStr};

use super::{CastlingRights, Color, Piece, PieceKind, Position, Square};

/// Ordered error categories specified by the profile's FEN boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FenErrorCategory {
    /// Bad field grammar, including unsupported FEN extensions.
    Syntax,
    /// Syntactically decimal counter outside its allowed numeric domain.
    Range,
    /// A locally inconsistent position, without a retrograde reachability proof.
    Structure,
}

impl fmt::Display for FenErrorCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Syntax => "syntax",
            Self::Range => "range",
            Self::Structure => "structure",
        })
    }
}

/// Specific rejection identity, independent of diagnostic wording.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FenErrorReason {
    /// input must be ASCII.
    NonAscii,
    /// expected exactly six fields.
    FieldCount,
    /// side must be w or b.
    Side,
    /// counters must contain ASCII digits only.
    CounterSyntax,
    /// halfmove clock exceeds u32.
    HalfmoveRange,
    /// fullmove number exceeds u32.
    FullmoveRange,
    /// fullmove number must be positive.
    FullmoveZero,
    /// pawn on a back rank.
    PawnBackRank,
    /// expected exactly one king of each color.
    KingCount,
    /// kings are adjacent.
    AdjacentKings,
    /// more than eight pawns or sixteen pieces for a color.
    PieceCount,
    /// castling right without its home king.
    CastlingKing,
    /// castling right without its home rook.
    CastlingRook,
    /// en passant target must be empty on the correct rank.
    EnPassantTarget,
    /// en passant pawn, origin or halfmove clock is inconsistent.
    EnPassantState,
    /// nonmoving king is attacked.
    NonmovingKingAttacked,
    /// expected eight board ranks.
    BoardRanks,
    /// adjacent empty-run digits.
    AdjacentRuns,
    /// board rank exceeds eight squares.
    RankOverflow,
    /// unknown board symbol.
    BoardSymbol,
    /// board rank must contain eight squares.
    RankWidth,
    /// unsupported castling symbol.
    CastlingSymbol,
    /// castling rights must be unique in KQkq order.
    CastlingOrder,
    /// target must be - or a square a1..h8.
    TargetSyntax,
}

impl fmt::Display for FenErrorReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::NonAscii => "input must be ASCII",
            Self::FieldCount => "expected exactly six fields",
            Self::Side => "side must be w or b",
            Self::CounterSyntax => "counters must contain ASCII digits only",
            Self::HalfmoveRange => "halfmove clock exceeds u32",
            Self::FullmoveRange => "fullmove number exceeds u32",
            Self::FullmoveZero => "fullmove number must be positive",
            Self::PawnBackRank => "pawn on a back rank",
            Self::KingCount => "expected exactly one king of each color",
            Self::AdjacentKings => "kings are adjacent",
            Self::PieceCount => "more than eight pawns or sixteen pieces for a color",
            Self::CastlingKing => "castling right without its home king",
            Self::CastlingRook => "castling right without its home rook",
            Self::EnPassantTarget => "en passant target must be empty on the correct rank",
            Self::EnPassantState => "en passant pawn, origin or halfmove clock is inconsistent",
            Self::NonmovingKingAttacked => "nonmoving king is attacked",
            Self::BoardRanks => "expected eight board ranks",
            Self::AdjacentRuns => "adjacent empty-run digits",
            Self::RankOverflow => "board rank exceeds eight squares",
            Self::BoardSymbol => "unknown board symbol",
            Self::RankWidth => "board rank must contain eight squares",
            Self::CastlingSymbol => "unsupported castling symbol",
            Self::CastlingOrder => "castling rights must be unique in KQkq order",
            Self::TargetSyntax => "target must be - or a square a1..h8",
        })
    }
}

/// Deterministic category plus a field/reason diagnostic; never a partial position.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FenError {
    category: FenErrorCategory,
    reason: FenErrorReason,
}

impl FenError {
    /// The earliest failed validation stage: syntax, then range, then structure.
    pub const fn category(self) -> FenErrorCategory {
        self.category
    }

    /// Stable machine-readable reason; Display wording is for people.
    pub const fn reason(self) -> FenErrorReason {
        self.reason
    }

    fn new(category: FenErrorCategory, reason: FenErrorReason) -> Self {
        Self { category, reason }
    }
}

impl fmt::Display for FenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FEN {}: {}", self.category, self.reason)
    }
}

impl Error for FenError {}

impl Position {
    /// Parse all six FEN fields under the named profile's strict validation policy.
    ///
    /// Syntax of **all** fields is checked before any numeric range or structure
    /// error. Success establishes local invariants, not historical reachability.
    pub fn from_fen(input: &str) -> Result<Self, FenError> {
        use FenErrorCategory::{Range, Syntax};
        if !input.is_ascii() {
            return Err(FenError::new(Syntax, FenErrorReason::NonAscii));
        }
        // Rust's split_ascii_whitespace excludes vertical tab; this profile
        // explicitly accepts it, along with the other five ASCII whitespace bytes.
        let mut fields = input
            .split([' ', '\t', '\r', '\n', '\x0b', '\x0c'])
            .filter(|field| !field.is_empty());
        let mut parts = [""; 6];
        for field in &mut parts {
            *field = fields
                .next()
                .ok_or(FenError::new(Syntax, FenErrorReason::FieldCount))?;
        }
        if fields.next().is_some() {
            return Err(FenError::new(Syntax, FenErrorReason::FieldCount));
        }

        let board = parse_board(parts[0])?;
        let side_to_move = match parts[1] {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(FenError::new(Syntax, FenErrorReason::Side)),
        };
        let castling_rights = parse_rights(parts[2])?;
        let en_passant_target = parse_target(parts[3])?;
        for counter in [parts[4], parts[5]] {
            if !counter.bytes().all(|byte| byte.is_ascii_digit()) {
                return Err(FenError::new(Syntax, FenErrorReason::CounterSyntax));
            }
        }
        let halfmove_clock = parts[4]
            .parse::<u32>()
            .map_err(|_| FenError::new(Range, FenErrorReason::HalfmoveRange))?;
        let fullmove_number = parts[5]
            .parse::<u32>()
            .map_err(|_| FenError::new(Range, FenErrorReason::FullmoveRange))?;
        if fullmove_number == 0 {
            return Err(FenError::new(Range, FenErrorReason::FullmoveZero));
        }

        let candidate = Self {
            board,
            side_to_move,
            castling_rights,
            en_passant_target,
            halfmove_clock,
            fullmove_number,
        };
        candidate.validate_structure()?;
        Ok(candidate)
    }

    /// Canonical six-field FEN, preserving raw target and numeric counter values.
    pub fn to_fen(&self) -> String {
        self.to_string()
    }

    /// Replace the whole position only after successful parsing and validation.
    ///
    /// On any error every field of `self` remains unchanged. This replaces a
    /// position only; it neither reconstructs nor preserves a match history.
    pub fn set_fen(&mut self, input: &str) -> Result<(), FenError> {
        let candidate = Self::from_fen(input)?;
        *self = candidate;
        Ok(())
    }

    pub(super) fn validate_structure(&self) -> Result<(), FenError> {
        use FenErrorCategory::Structure;
        let fail = |reason| FenError::new(Structure, reason);
        let mut kings = [None; 2];
        let mut pieces = [0; 2];
        let mut pawns = [0; 2];
        for (index, occupant) in self.board.iter().enumerate() {
            if let Some(piece) = occupant {
                let square = Square(index as u8);
                let color = piece.color.index();
                pieces[color] += 1;
                if piece.kind == PieceKind::Pawn {
                    pawns[color] += 1;
                    if matches!(square.rank(), 0 | 7) {
                        return Err(fail(FenErrorReason::PawnBackRank));
                    }
                }
                if piece.kind == PieceKind::King && kings[color].replace(square).is_some() {
                    return Err(fail(FenErrorReason::KingCount));
                }
            }
        }
        let [Some(white_king), Some(black_king)] = kings else {
            return Err(fail(FenErrorReason::KingCount));
        };
        if white_king.file().abs_diff(black_king.file()) <= 1
            && white_king.rank().abs_diff(black_king.rank()) <= 1
        {
            return Err(fail(FenErrorReason::AdjacentKings));
        }
        if pawns.iter().any(|&count| count > 8) || pieces.iter().any(|&count| count > 16) {
            return Err(fail(FenErrorReason::PieceCount));
        }
        for (color, rank) in [(Color::White, 0usize), (Color::Black, 7)] {
            let rights = self.castling_rights;
            let has_piece = |file, kind| self.board[rank * 8 + file] == Some(Piece { color, kind });
            if (rights.kingside(color) || rights.queenside(color)) && !has_piece(4, PieceKind::King)
            {
                return Err(fail(FenErrorReason::CastlingKing));
            }
            if (rights.kingside(color) && !has_piece(7, PieceKind::Rook))
                || (rights.queenside(color) && !has_piece(0, PieceKind::Rook))
            {
                return Err(fail(FenErrorReason::CastlingRook));
            }
        }
        if let Some(target) = self.en_passant_target {
            let (target_rank, pawn_rank, origin_rank) = match self.side_to_move {
                Color::White => (5, 4, 6),
                Color::Black => (2, 3, 1),
            };
            if target.rank() != target_rank || self.piece_at(target).is_some() {
                return Err(fail(FenErrorReason::EnPassantTarget));
            }
            let file = target.file() as usize;
            let pawn = Some(Piece {
                color: self.side_to_move.opponent(),
                kind: PieceKind::Pawn,
            });
            if self.board[pawn_rank * 8 + file] != pawn
                || self.board[origin_rank * 8 + file].is_some()
                || self.halfmove_clock != 0
            {
                return Err(fail(FenErrorReason::EnPassantState));
            }
        }
        let nonmoving_king = match self.side_to_move {
            Color::White => black_king,
            Color::Black => white_king,
        };
        if self.is_square_attacked(nonmoving_king, self.side_to_move) {
            return Err(fail(FenErrorReason::NonmovingKingAttacked));
        }
        Ok(())
    }
}

impl FromStr for Position {
    type Err = FenError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::from_fen(input)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            let mut empty = 0;
            for file in 0..8 {
                match self.board[rank * 8 + file] {
                    None => empty += 1,
                    Some(piece) => {
                        if empty > 0 {
                            write!(f, "{empty}")?;
                            empty = 0;
                        }
                        write!(f, "{}", piece_letter(piece))?;
                    }
                }
            }
            if empty > 0 {
                write!(f, "{empty}")?;
            }
            if rank > 0 {
                f.write_str("/")?;
            }
        }
        write!(
            f,
            " {} {} ",
            if self.side_to_move == Color::White {
                'w'
            } else {
                'b'
            },
            self.castling_rights
        )?;
        if let Some(target) = self.en_passant_target {
            write!(f, "{target}")?;
        } else {
            f.write_str("-")?;
        }
        write!(f, " {} {}", self.halfmove_clock, self.fullmove_number)
    }
}

fn parse_board(field: &str) -> Result<[Option<Piece>; 64], FenError> {
    use FenErrorCategory::Syntax;
    let fail = |reason| FenError::new(Syntax, reason);
    let mut board = [None; 64];
    let mut ranks = field.split('/');
    for rank in (0..8).rev() {
        let row = ranks.next().ok_or(fail(FenErrorReason::BoardRanks))?;
        let mut file = 0usize;
        let mut previous_digit = false;
        for byte in row.bytes() {
            if matches!(byte, b'1'..=b'8') {
                if previous_digit {
                    return Err(fail(FenErrorReason::AdjacentRuns));
                }
                file += (byte - b'0') as usize;
                previous_digit = true;
            } else {
                if file >= 8 {
                    return Err(fail(FenErrorReason::RankOverflow));
                }
                let kind = match byte.to_ascii_lowercase() {
                    b'p' => PieceKind::Pawn,
                    b'n' => PieceKind::Knight,
                    b'b' => PieceKind::Bishop,
                    b'r' => PieceKind::Rook,
                    b'q' => PieceKind::Queen,
                    b'k' => PieceKind::King,
                    _ => return Err(fail(FenErrorReason::BoardSymbol)),
                };
                board[rank * 8 + file] = Some(Piece {
                    color: if byte.is_ascii_uppercase() {
                        Color::White
                    } else {
                        Color::Black
                    },
                    kind,
                });
                file += 1;
                previous_digit = false;
            }
            if file > 8 {
                return Err(fail(FenErrorReason::RankOverflow));
            }
        }
        if file != 8 {
            return Err(fail(FenErrorReason::RankWidth));
        }
    }
    if ranks.next().is_some() {
        return Err(fail(FenErrorReason::BoardRanks));
    }
    Ok(board)
}

fn parse_rights(field: &str) -> Result<CastlingRights, FenError> {
    let mut rights = CastlingRights::default();
    if field == "-" {
        return Ok(rights);
    }
    let mut previous = None;
    for byte in field.bytes() {
        let order = match byte {
            b'K' => 0,
            b'Q' => 1,
            b'k' => 2,
            b'q' => 3,
            _ => {
                return Err(FenError::new(
                    FenErrorCategory::Syntax,
                    FenErrorReason::CastlingSymbol,
                ));
            }
        };
        if previous.is_some_and(|last| last >= order) {
            return Err(FenError::new(
                FenErrorCategory::Syntax,
                FenErrorReason::CastlingOrder,
            ));
        }
        previous = Some(order);
        match byte {
            b'K' => rights.kingside[Color::White.index()] = true,
            b'Q' => rights.queenside[Color::White.index()] = true,
            b'k' => rights.kingside[Color::Black.index()] = true,
            b'q' => rights.queenside[Color::Black.index()] = true,
            _ => unreachable!(),
        }
    }
    Ok(rights)
}

fn parse_target(field: &str) -> Result<Option<Square>, FenError> {
    if field == "-" {
        return Ok(None);
    }
    match field.as_bytes() {
        &[file @ b'a'..=b'h', rank @ b'1'..=b'8'] => Ok(Square::new(file - b'a', rank - b'1')),
        _ => Err(FenError::new(
            FenErrorCategory::Syntax,
            FenErrorReason::TargetSyntax,
        )),
    }
}

fn piece_letter(piece: Piece) -> char {
    let letter: char = match piece.kind {
        PieceKind::Pawn => 'P',
        PieceKind::Knight => 'N',
        PieceKind::Bishop => 'B',
        PieceKind::Rook => 'R',
        PieceKind::Queen => 'Q',
        PieceKind::King => 'K',
    };
    if piece.color == Color::White {
        letter
    } else {
        letter.to_ascii_lowercase()
    }
}
