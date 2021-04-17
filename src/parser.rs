use crate::method::Method;
use std::convert::TryFrom;
use std::str;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Version {
    OneDotOne,
}

impl TryFrom<&[u8]> for Version {
    type Error = ParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        match str::from_utf8(value) {
            Ok("1.1") => Ok(Version::OneDotOne),
            _ => Err(ParseError::InvalidVersion),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidMethod,
    InvalidUri,
    InvalidVersion,
}

#[derive(Debug)]
pub struct Parser<'a> {
    // Assume that Parser parses ASCII.
    state: &'a [u8],
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self { state: input }
    }

    /// Read a first character in the state and advance state.
    pub fn consume(&mut self) -> Option<u8> {
        self.state.split_first().map(|(&b, tail)| {
            self.state = tail;
            b
        })
    }

    /// Read a first character in the state, but the state is not modified.
    pub fn peek(&self) -> Option<u8> {
        self.state.split_first().map(|(&b, _)| b)
    }

    /// Read until `target` appears and return string composed of bytes read so far.
    /// It does not include `target`.
    pub fn read_until(&mut self, target: u8) -> Option<&[u8]> {
        let mut pos = 0;
        while pos < self.state.len() {
            if self.state[pos] == target {
                break;
            }
            pos += 1;
        }

        if pos == 0 {
            return None;
        }
        let (found, tail) = self.state.split_at(pos);
        self.state = tail;
        // First element of tail is `target`, so skip it.
        self.consume();
        Some(found)
    }

    /// Read until whitespace(' ') appears.
    pub fn read_until_whitespace(&mut self) -> Option<&[u8]> {
        self.read_until(b' ')
    }

    pub fn parse_method(&mut self) -> Result<Method, ParseError> {
        match self.read_until_whitespace() {
            Some(method) => Method::try_from(method),
            None => Err(ParseError::InvalidMethod),
        }
    }

    pub fn parse_uri(&mut self) -> Result<&[u8], ParseError> {
        let uri = self.read_until_whitespace().ok_or(ParseError::InvalidUri)?;
        if uri.starts_with(&[b'/']) {
            Ok(uri)
        } else {
            Err(ParseError::InvalidUri)
        }
    }

    pub fn parse_version(&mut self) -> Result<Version, ParseError> {
        let protocol = self.read_until(b'/').ok_or(ParseError::InvalidVersion)?;
        match str::from_utf8(protocol) {
            Ok("HTTP") => (),
            _ => return Err(ParseError::InvalidVersion),
        }
        let version = self.read_until(b'\r').ok_or(ParseError::InvalidVersion)?;
        Version::try_from(version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_char() {
        let mut p = Parser::new(&[42, 43]);
        assert_eq!(Some(b'*'), p.consume());
        assert_eq!(Some(b'+'), p.consume());
        assert_eq!(None, p.consume());
    }

    #[test]
    fn peek_char() {
        let p = Parser::new(&[42, 43]);
        assert_eq!(Some(b'*'), p.peek());
    }

    #[test]
    fn read_until_delim() {
        let bytes = "GET /index.html HTTP/1.1\r\n".as_bytes();
        let mut p = Parser::new(bytes);
        assert_eq!(Some("GET".as_bytes()), p.read_until(b' '));
        assert_eq!(Some("/index.html".as_bytes()), p.read_until(b' '));
        assert_eq!(Some("HTTP/1.1".as_bytes()), p.read_until(b'\r'));
    }

    #[test]
    fn read_until_empty() {
        let mut p = Parser::new(&[]);
        assert_eq!(None, p.read_until(b' '));
    }

    #[test]
    fn parse_method() {
        let bytes = "GET /index.html HTTP/1.1\r\n".as_bytes();
        let mut p = Parser::new(bytes);
        assert_eq!(Ok(Method::Get), p.parse_method());
    }

    #[test]
    fn parse_uri() {
        let bytes = "/index.html HTTP/1.1\r\n".as_bytes();
        let mut p = Parser::new(bytes);
        assert_eq!(Ok("/index.html".as_bytes()), p.parse_uri());
    }

    #[test]
    fn parse_version() {
        let bytes = "HTTP/1.1\r\n".as_bytes();
        let mut p = Parser::new(bytes);
        assert_eq!(Ok(Version::OneDotOne), p.parse_version());
    }
}
