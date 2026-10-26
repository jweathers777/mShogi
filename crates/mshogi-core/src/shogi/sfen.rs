//! Strict four-field SFEN syntax, local validation and canonical output.

use super::{Hand, Piece, PieceKind, Position, PositionError, Side};
use std::fmt::Write;

impl Position {
    /// Parse exactly four ASCII SFEN fields with one space between fields.
    /// Counter is mandatory and positive. Hands accept any owner/kind order,
    /// but no duplicate kind for an owner; output uses canonical RBGSNLP order.
    /// No previous moves, checks, repetition or outcomes are reconstructed.
    pub fn from_sfen(text: &str) -> Result<Self, PositionError> {
        if text.len() > 512 || !text.is_ascii() {
            return Err(PositionError::Syntax);
        }
        let fields: Vec<_> = text.split(' ').collect();
        if fields.len() != 4 || fields.iter().any(|f| f.is_empty()) {
            return Err(PositionError::Syntax);
        }
        let ranks: Vec<_> = fields[0].split('/').collect();
        if ranks.len() != 9 {
            return Err(PositionError::BoardSyntax);
        }
        let mut board = [None; 81];
        for (rank, text) in ranks.iter().enumerate() {
            let mut chars = text.chars();
            let mut file = 0;
            let mut last_was_digit = false;
            while let Some(c) = chars.next() {
                if ('1'..='9').contains(&c) {
                    if last_was_digit {
                        return Err(PositionError::BoardSyntax);
                    }
                    file += (c as u8 - b'0') as usize;
                    last_was_digit = true;
                } else {
                    last_was_digit = false;
                    let promoted = c == '+';
                    let letter = if promoted {
                        chars.next().ok_or(PositionError::BoardSyntax)?
                    } else {
                        c
                    };
                    let kind = PieceKind::from_letter(letter).ok_or(PositionError::BoardSyntax)?;
                    let side = if letter.is_ascii_uppercase() {
                        Side::Sente
                    } else {
                        Side::Gote
                    };
                    let piece = Piece::new(side, kind, promoted).ok_or(PositionError::Promotion)?;
                    if file >= 9 {
                        return Err(PositionError::BoardSyntax);
                    }
                    board[rank * 9 + file] = Some(piece);
                    file += 1;
                }
                if file > 9 {
                    return Err(PositionError::BoardSyntax);
                }
            }
            if file != 9 {
                return Err(PositionError::BoardSyntax);
            }
        }
        let side = match fields[1] {
            "b" => Side::Sente,
            "w" => Side::Gote,
            _ => return Err(PositionError::SideSyntax),
        };
        let mut counts = [Vec::new(), Vec::new()];
        if fields[2] != "-" {
            let mut count: u16 = 0;
            let mut digits = false;
            for c in fields[2].chars() {
                if c.is_ascii_digit() {
                    if !digits && c == '0' {
                        return Err(PositionError::InvalidHand);
                    }
                    digits = true;
                    count = count
                        .checked_mul(10)
                        .and_then(|n| n.checked_add(u16::from(c as u8 - b'0')))
                        .ok_or(PositionError::InvalidHand)?;
                    if count > 18 {
                        return Err(PositionError::InvalidHand);
                    }
                } else {
                    let kind = PieceKind::from_letter(c).ok_or(PositionError::InvalidHand)?;
                    let owner = if c.is_ascii_uppercase() { 0 } else { 1 };
                    let n = if digits { count as u8 } else { 1 };
                    // Single resources have no numeric prefix in strict SFEN.
                    if digits && n == 1 {
                        return Err(PositionError::InvalidHand);
                    }
                    counts[owner].push((kind, n));
                    count = 0;
                    digits = false;
                }
            }
            if digits {
                return Err(PositionError::InvalidHand);
            }
        }
        let hands = [
            Hand::from_counts(&counts[0])?,
            Hand::from_counts(&counts[1])?,
        ];
        if fields[3].starts_with('0') || !fields[3].bytes().all(|c| c.is_ascii_digit()) {
            return Err(PositionError::Counter);
        }
        let next_ply = fields[3]
            .parse::<u32>()
            .map_err(|_| PositionError::Counter)?;
        Self::from_diagram(board, hands, side, Some(next_ply))
    }

    /// Canonical SFEN. Root provenance is intentionally not encoded in SFEN;
    /// preserve `history_origin` separately in a future canonical game record.
    pub fn to_sfen(&self) -> String {
        let mut out = String::new();
        for rank in 0..9 {
            if rank != 0 {
                out.push('/');
            }
            let mut empty = 0_u8;
            for file in 0..9 {
                if let Some(piece) = self.board[rank * 9 + file] {
                    if empty > 0 {
                        out.push(char::from(b'0' + empty));
                        empty = 0;
                    }
                    if piece.promoted {
                        out.push('+');
                    }
                    let letter = piece.kind.letter();
                    out.push(if piece.side == Side::Sente {
                        letter
                    } else {
                        letter.to_ascii_lowercase()
                    });
                } else {
                    empty += 1;
                }
            }
            if empty > 0 {
                out.push(char::from(b'0' + empty));
            }
        }
        out.push_str(if self.side == Side::Sente {
            " b "
        } else {
            " w "
        });
        let mut any = false;
        for side in [Side::Sente, Side::Gote] {
            for kind in PieceKind::HAND_KINDS {
                let count = self.hand(side).count(kind);
                if count == 0 {
                    continue;
                }
                any = true;
                if count > 1 {
                    write!(out, "{count}").expect("String formatting");
                }
                let letter = kind.letter();
                out.push(if side == Side::Sente {
                    letter
                } else {
                    letter.to_ascii_lowercase()
                });
            }
        }
        if !any {
            out.push('-');
        }
        write!(out, " {}", self.next_ply).expect("String formatting");
        out
    }
}
