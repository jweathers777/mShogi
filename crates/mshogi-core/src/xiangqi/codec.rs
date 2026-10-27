//! Strict position-only notation, not a game/archive/history restoration helper.
use super::{
    Counters, Intersection, POSITION_SCHEMA_ID, PROFILE_EDITION, PROFILE_ID, Piece, PieceKind,
    Position, PositionError, Side,
};
use serde::{Deserialize, Serialize};

/// Maximum native root-position JSON bytes. Valid documents are under512bytes;
/// the4KiB bound limits hostile parsing without imposing a future record limit.
pub const MAX_POSITION_BYTES: usize = 4096;
/// Maximum six-field position FEN bytes (no history or extra fields).
pub const MAX_FEN_BYTES: usize = 256;

/// How the immutable known root was supplied; not a reachability certificate.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootOrigin {
    /// Exact standard initial diagram/counters; known history starts there.
    StandardInitial,
    /// A six-field imported position; no earlier actions/checks are inferred.
    ImportedFen,
    /// A structural diagnostic diagram, possibly with unknown counters.
    AnalysisDiagram,
}
/// An immutable known root plus construction provenance, separate from a mutable
/// position. It contains no moves, warnings, claims, conclusions or execution IDs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PositionRoot {
    position: Position,
    origin: RootOrigin,
}
// Newtype rather than an Option field: an explicitly nullable field is required,
// so a missing counter cannot be silently normalized to unknown by Serde.
#[derive(Serialize)]
#[serde(transparent)]
struct NullableCounter(Option<u32>);
impl<'de> Deserialize<'de> for NullableCounter {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct CounterVisitor;
        impl serde::de::Visitor<'_> for CounterVisitor {
            type Value = NullableCounter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("an explicit null or unsigned32-bit declared counter")
            }
            fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
                Ok(NullableCounter(None))
            }
            fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                u32::try_from(value)
                    .map(|value| NullableCounter(Some(value)))
                    .map_err(|_| E::custom("declared counter exceeds u32"))
            }
        }
        // deserialize_any deliberately rejects Serde's missing_field deserializer;
        // deserialize_option would silently interpret a missing field as null.
        deserializer.deserialize_any(CounterVisitor)
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct RootWire {
    schema: String,
    profile: String,
    edition: String,
    history: String,
    origin: RootOrigin,
    board: String,
    side: Side,
    no_capture_plies: NullableCounter,
    fullmove_number: NullableCounter,
}
impl PositionRoot {
    /// Exact initial root. This starts known history, not a game record.
    pub fn standard() -> Self {
        Self {
            position: Position::initial(),
            origin: RootOrigin::StandardInitial,
        }
    }
    /// Strict imported root, preserving supplied counters but no earlier history.
    pub fn from_fen(text: &str) -> Result<Self, PositionError> {
        Ok(Self {
            position: Position::from_fen(text)?,
            origin: RootOrigin::ImportedFen,
        })
    }
    /// A diagnostic root; static live-domain validity is deliberately separate.
    pub fn analysis(position: Position) -> Self {
        Self {
            position,
            origin: RootOrigin::AnalysisDiagram,
        }
    }
    /// Root diagram. Clone it for local work while retaining the immutable root.
    pub const fn position(&self) -> &Position {
        &self.position
    }
    /// Supplied-root provenance. No prior repetition/check information is implied.
    pub const fn origin(&self) -> RootOrigin {
        self.origin
    }
    /// Position-only native JSON, preserving explicit unknown counters/provenance.
    /// Not an alternative canonical full-game schema or a desktop save file.
    pub fn to_json(&self) -> String {
        let wire = RootWire {
            schema: POSITION_SCHEMA_ID.into(),
            profile: PROFILE_ID.into(),
            edition: PROFILE_EDITION.into(),
            history: "starts_at_this_position".into(),
            origin: self.origin,
            board: self.position.board_fen(),
            side: self.position.side,
            no_capture_plies: NullableCounter(self.position.counters.no_capture_plies()),
            fullmove_number: NullableCounter(self.position.counters.fullmove_number()),
        };
        serde_json::to_string(&wire).expect("fixed native position fields serialize")
    }
    /// Strict bounded direct typed parsing: duplicate fields survive until Serde
    /// rejects them; unknown/missing/unsupported identities never fall through.
    pub fn from_json(text: &str) -> Result<Self, PositionError> {
        if text.len() > MAX_POSITION_BYTES {
            return Err(PositionError::Syntax);
        }
        let wire: RootWire = serde_json::from_str(text).map_err(|_| PositionError::Json)?;
        if wire.schema != POSITION_SCHEMA_ID
            || wire.profile != PROFILE_ID
            || wire.edition != PROFILE_EDITION
            || wire.history != "starts_at_this_position"
        {
            return Err(PositionError::Identity);
        }
        let counters = Counters::new(wire.no_capture_plies.0, wire.fullmove_number.0)?;
        let position = Position::from_board(parse_board(&wire.board)?, wire.side, counters)?;
        match wire.origin {
            RootOrigin::StandardInitial if position != Position::initial() => {
                return Err(PositionError::OriginMismatch);
            }
            RootOrigin::ImportedFen
                if counters.no_capture_plies().is_none()
                    || counters.fullmove_number().is_none() =>
            {
                return Err(PositionError::OriginMismatch);
            }
            _ => {}
        }
        Ok(Self {
            position,
            origin: wire.origin,
        })
    }
}
impl Position {
    /// Strict scoped FEN: rank9first, k/a/b/n/r/c/p, uppercase Red/lowercase Black,
    /// active `w` means Red and `b` Black, literal `- -`, capture-free plies and
    /// positive fullmove number. h/e aliases, noncanonical runs/spacing, unused
    /// field values and unknown counters are not silently normalized.
    /// Parsing admits a structural diagram, not live play or historical validity.
    pub fn from_fen(text: &str) -> Result<Self, PositionError> {
        if text.len() > MAX_FEN_BYTES || !text.is_ascii() {
            return Err(PositionError::Syntax);
        }
        let fields: Vec<_> = text.split(' ').collect();
        if fields.len() != 6
            || fields.iter().any(|field| field.is_empty())
            || fields[2] != "-"
            || fields[3] != "-"
        {
            return Err(PositionError::Syntax);
        }
        let side = match fields[1] {
            "w" => Side::Red,
            "b" => Side::Black,
            _ => return Err(PositionError::Syntax),
        };
        let counters = Counters::new(
            Some(parse_counter(fields[4], false)?),
            Some(parse_counter(fields[5], true)?),
        )?;
        Self::from_board(parse_board(fields[0])?, side, counters)
    }
    /// Canonical scoped position FEN. Missing declared counters are an explicit
    /// error; use the native position-root JSON to preserve unknown values.
    pub fn to_fen(&self) -> Result<String, PositionError> {
        let count = self
            .counters
            .no_capture_plies()
            .ok_or(PositionError::UnknownCounters)?;
        let fullmove = self
            .counters
            .fullmove_number()
            .ok_or(PositionError::UnknownCounters)?;
        Ok(format!(
            "{} {} - - {count} {fullmove}",
            self.board_fen(),
            if self.side == Side::Red { "w" } else { "b" }
        ))
    }
    fn board_fen(&self) -> String {
        let mut output = String::new();
        for rank in (0..10).rev() {
            if rank != 9 {
                output.push('/');
            }
            let mut empty = 0_u8;
            for file in 0..9 {
                let point = Intersection::new(file, rank).expect("bounded board loop");
                if let Some(piece) = self.piece(point) {
                    if empty != 0 {
                        output.push(char::from(b'0' + empty));
                        empty = 0;
                    }
                    let letter = piece.kind.letter();
                    output.push(if piece.side == Side::Red {
                        letter
                    } else {
                        letter.to_ascii_lowercase()
                    });
                } else {
                    empty += 1;
                }
            }
            if empty != 0 {
                output.push(char::from(b'0' + empty));
            }
        }
        output
    }
}
fn parse_counter(text: &str, positive: bool) -> Result<u32, PositionError> {
    if text.is_empty()
        || !text.bytes().all(|b| b.is_ascii_digit())
        || (text.len() > 1 && text.starts_with('0'))
    {
        return Err(PositionError::Counter);
    }
    let value = text.parse().map_err(|_| PositionError::Counter)?;
    if positive && value == 0 {
        return Err(PositionError::Counter);
    }
    Ok(value)
}
fn parse_board(text: &str) -> Result<[Option<Piece>; 90], PositionError> {
    if text.len() > 99 || !text.is_ascii() {
        return Err(PositionError::BoardSyntax);
    }
    let ranks: Vec<_> = text.split('/').collect();
    if ranks.len() != 10 {
        return Err(PositionError::BoardSyntax);
    }
    let mut board = [None; 90];
    for (row, rank_text) in ranks.into_iter().enumerate() {
        let mut file = 0_u8;
        let mut previous_digit = false;
        for letter in rank_text.chars() {
            if ('1'..='9').contains(&letter) {
                if previous_digit {
                    return Err(PositionError::BoardSyntax);
                }
                file += letter as u8 - b'0';
                previous_digit = true;
            } else {
                previous_digit = false;
                let kind = PieceKind::from_letter(letter).ok_or(PositionError::BoardSyntax)?;
                if file >= 9 {
                    return Err(PositionError::BoardSyntax);
                }
                let point =
                    Intersection::new(file, 9 - row as u8).ok_or(PositionError::BoardSyntax)?;
                board[point.index()] = Some(Piece::new(
                    if letter.is_ascii_uppercase() {
                        Side::Red
                    } else {
                        Side::Black
                    },
                    kind,
                ));
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
    Ok(board)
}
