//! Console line framing and protocol text validation.
use std::io::{self, BufRead};

pub enum Ending {
    Shell,
    Protocol,
}

/// Shell records end at LF. Protocol commands end at CR or LF.
/// A CRLF protocol separator produces an extra empty command, which is ignored.
pub fn line(input: &mut impl BufRead, ending: Ending) -> io::Result<Option<Vec<u8>>> {
    let mut line = Vec::new();
    loop {
        let chunk = input.fill_buf()?;
        if chunk.is_empty() {
            return Ok((!line.is_empty()).then_some(line));
        }
        let end = chunk
            .iter()
            .position(|&byte| byte == b'\n' || matches!(ending, Ending::Protocol) && byte == b'\r');
        let take = end.unwrap_or(chunk.len());
        line.extend_from_slice(&chunk[..take]);
        input.consume(take + usize::from(end.is_some()));
        if end.is_some() {
            return Ok(Some(line));
        }
    }
}

pub fn protocol_text(bytes: &[u8]) -> Option<&str> {
    let line = std::str::from_utf8(bytes).ok()?;
    (line.is_ascii()
        && !bytes
            .iter()
            .any(|b| b.is_ascii_control() && !b.is_ascii_whitespace()))
    .then_some(line)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cr_and_lf_both_end_a_line() {
        let mut input = std::io::Cursor::new(b"uci\risready\r\nquit\n");
        for expected in ["uci", "isready", "", "quit"] {
            assert_eq!(
                line(&mut input, Ending::Protocol).unwrap().unwrap(),
                expected.as_bytes()
            );
        }
        assert!(line(&mut input, Ending::Protocol).unwrap().is_none());
    }
}
