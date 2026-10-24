//! Strict chess coordinate action identity, shared by records and adapters.
use super::{Action, Promotion, Square};
use std::{error::Error, fmt, str::FromStr};

/// Invalid coordinate action syntax (legality is a separate checked boundary).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ActionParseError;

impl fmt::Display for ActionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("expected lowercase coordinate action, e.g. e2e4 or e7e8q")
    }
}
impl Error for ActionParseError {}

impl FromStr for Action {
    type Err = ActionParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let b = text.as_bytes();
        if !matches!(b.len(), 4 | 5) {
            return Err(ActionParseError);
        }
        fn square(b: &[u8]) -> Result<Square, ActionParseError> {
            match b {
                [f @ b'a'..=b'h', r @ b'1'..=b'8'] => Ok(Square::new(f - b'a', r - b'1').unwrap()),
                _ => Err(ActionParseError),
            }
        }
        Ok(Self {
            from: square(&b[..2])?,
            to: square(&b[2..4])?,
            promotion: match b.get(4) {
                None => None,
                Some(b'q') => Some(Promotion::Queen),
                Some(b'r') => Some(Promotion::Rook),
                Some(b'b') => Some(Promotion::Bishop),
                Some(b'n') => Some(Promotion::Knight),
                _ => return Err(ActionParseError),
            },
        })
    }
}
impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.from,
            self.to,
            match self.promotion {
                None => "",
                Some(Promotion::Queen) => "q",
                Some(Promotion::Rook) => "r",
                Some(Promotion::Bishop) => "b",
                Some(Promotion::Knight) => "n",
            }
        )
    }
}
