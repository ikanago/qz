use crate::{
    header::{HeaderName, HeaderValue},
    method::Method,
    status::StatusCode,
    Uri, Version,
};
use std::convert::TryFrom;
use std::str;

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

    /// Read until `target` appears and return string composed of bytes read so far.
    /// It does not include `target`.
    pub fn read_until(&mut self, target: u8) -> Option<&[u8]> {
        let index = self.state.iter().position(|&b| b == target)?;
        let (found, tail) = self.state.split_at(index);
        self.state = tail;
        // First element of tail is `target`, so skip it.
        self.consume();
        Some(found)
    }

    /// Read until whitespace(' ') appears.
    pub fn read_until_whitespace(&mut self) -> Option<&[u8]> {
        self.read_until(b' ')
    }

    /// Consume a first element and return error if it does not equal to `target`.
    pub fn expect(&mut self, target: u8, error: StatusCode) -> crate::Result<()> {
        match self.consume() {
            Some(b) if b == target => Ok(()),
            _ => Err(error),
        }
    }

    pub fn parse_request_line(&mut self) -> crate::Result<(Method, Uri, Version)> {
        let method = self.parse_method()?;
        let uri = self.parse_uri()?;
        let version = self.parse_version()?;
        self.expect(b'\n', StatusCode::BadRequest)?;
        Ok((method, uri, version))
    }

    fn parse_method(&mut self) -> crate::Result<Method> {
        match self.read_until_whitespace() {
            Some(method) => Method::try_from(method),
            None => Err(StatusCode::BadRequest),
        }
    }

    fn parse_uri(&mut self) -> crate::Result<Uri> {
        let uri = self.read_until_whitespace().ok_or(StatusCode::BadRequest)?;
        if uri.starts_with(&[b'/']) {
            Ok(Uri::new(uri))
        } else {
            Err(StatusCode::BadRequest)
        }
    }

    fn parse_version(&mut self) -> crate::Result<Version> {
        let protocol = self
            .read_until(b'/')
            .ok_or(StatusCode::HttpVersionNotSupported)?;
        match str::from_utf8(protocol) {
            Ok("HTTP") => (),
            _ => return Err(StatusCode::HttpVersionNotSupported),
        }
        let version = self
            .read_until(b'\r')
            .ok_or(StatusCode::HttpVersionNotSupported)?;
        Version::try_from(version)
    }

    pub fn parse_header(&mut self) -> crate::Result<(HeaderName, HeaderValue)> {
        let header_name = self
            .read_until(b':')
            .ok_or(StatusCode::BadRequest)?
            .to_vec();
        let header_name = HeaderName::from(header_name);
        self.expect(b' ', StatusCode::BadRequest)?;
        let header_value = self
            .read_until(b'\r')
            .ok_or(StatusCode::BadRequest)?
            .to_vec();
        self.expect(b'\n', StatusCode::BadRequest)?;
        Ok((header_name, header_value))
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
    fn parse_request_line() {
        let bytes = "GET /index.html HTTP/1.1\r\n".as_bytes();
        let mut p = Parser::new(bytes);
        let (method, uri, version) = p.parse_request_line().unwrap();
        assert_eq!(Method::Get, method);
        assert_eq!(Uri::new(b"/index.html"), uri);
        assert_eq!(Version::OneDotOne, version);
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
        assert_eq!(Ok(Uri::new(b"/index.html")), p.parse_uri());
    }

    #[test]
    fn parse_version() {
        let bytes = "HTTP/1.1\r\n".as_bytes();
        let mut p = Parser::new(bytes);
        assert_eq!(Ok(Version::OneDotOne), p.parse_version());
    }

    #[test]
    fn parse_header() {
        let bytes = b"Accept: */*\r\n";
        let mut p = Parser::new(bytes);
        assert_eq!(Ok((HeaderName::Accept, b"*/*".to_vec())), p.parse_header());
    }
}
