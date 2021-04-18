use crate::{
    method::Method,
    parser::{ParseError, Parser},
};
use std::str;
use std::{convert::TryFrom, ops::Range};

#[derive(Debug, PartialEq, Eq)]
pub struct Uri(Vec<u8>);

impl Uri {
    pub fn new(path: &[u8]) -> Self {
        Self(path.to_vec())
    }
}

impl Default for Uri {
    fn default() -> Self {
        Uri(b"/".to_vec())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Version {
    OneDotOne,
}

impl Default for Version {
    fn default() -> Self {
        Self::OneDotOne
    }
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

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Request {
    method: Method,
    uri: Uri,
    version: Version,
}

impl Request {
    fn new() -> Self {
        Self::default()
    }

    fn parse_request_line(&mut self, bytes: &[u8]) -> Result<(), ParseError> {
        let mut p = Parser::new(bytes);
        let (method, uri, version) = p.parse_request_line()?;
        self.method = method;
        self.uri = uri;
        self.version = version;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseState {
    RequestLine,
    Completed,
}

impl ParseState {
    fn next_state(self) -> Self {
        match self {
            ParseState::RequestLine => ParseState::Completed,
            ParseState::Completed => ParseState::Completed,
        }
    }
}

/// Construct `Request` from chunked data, which is mainly got from TCP stream.
/// Currently, tokio does not buffered IO for `TcpStream`, so I implemented this by my own.
///
/// There are two parsing strategies:
/// * Read whole request, then parse it
/// * Parse line by lien
///
/// Although the first option is easier, parsing request with message body is difficult
/// because it is hard to know when to finish reading from socket.
/// So this struct parse a request with second strategy to get body size from `Content-Length`
/// header while parsing.
pub struct RequestBuffer {
    buffer: Vec<u8>,
    parsing: ParseState,
    request: Request,
}

impl RequestBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            parsing: ParseState::RequestLine,
            request: Request::new(),
        }
    }

    pub fn complete(self) -> Request {
        self.request
    }

    /// Extend buffer of this sturct with `data` and try to parse given request data.
    pub fn try_parse(&mut self, data: &[u8]) -> Result<ParseState, ParseError> {
        self.buffer.extend_from_slice(&data);
        let mut buf_iter = self.buffer.iter();
        let mut end = 0;
        // self.buffer may contain two lines(two CRLFs).
        let state = loop {
            if let ParseState::Completed = self.parsing {
                return Ok(ParseState::Completed);
            }

            // Find "\r\n" to determine a line.
            if let Some(index) = buf_iter.position(|&b| b == b'\r') {
                if let Some(b'\n') = buf_iter.next() {
                    let start = 0;
                    end = index + 2;
                    self.request.parse_request_line(&self.buffer[start..end])?;
                    let next_state = self.parsing.next_state();
                    self.parsing = next_state;
                } else {
                    break self.parsing;
                }
            } else {
                break self.parsing;
            }
        };
        self.buffer = self.buffer.drain(end..).collect();
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_request() {
        let data = vec![
            b"GET /~/in".to_vec(),
            b"dex.html ".to_vec(),
            b"HTTP/1.1\r".to_vec(),
            b"\n".to_vec(),
        ];
        let mut request_buf = RequestBuffer::new();
        for message in data {
            match request_buf.try_parse(&message) {
                Ok(ParseState::Completed) => break,
                Ok(_) => continue,
                Err(_) => panic!(),
            }
        }
        assert_eq!(
            Request {
                method: Method::Get,
                uri: Uri::new(b"/~/index.html"),
                version: Version::OneDotOne
            },
            request_buf.complete()
        );
    }
}
