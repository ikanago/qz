use crate::{
    header::{HeaderName, HeaderValue},
    method::Method,
    parser::{ParseError, Parser},
    Uri, Version,
};
use std::{collections::HashMap, fmt, str};

#[derive(Debug, Default, PartialEq)]
pub struct Request {
    method: Method,
    uri: Uri,
    version: Version,
    headers: HashMap<HeaderName, HeaderValue>,
}

impl Request {
    fn new() -> Self {
        Self::default()
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    fn parse_request_line(&mut self, bytes: &[u8]) -> Result<(), ParseError> {
        let mut p = Parser::new(bytes);
        let (method, uri, version) = p.parse_request_line()?;
        self.method = method;
        self.uri = uri;
        self.version = version;
        Ok(())
    }

    fn parse_header(&mut self, bytes: &[u8]) -> Result<(), ParseError> {
        let mut p = Parser::new(bytes);
        let (name, value) = p.parse_header()?;
        self.headers.insert(name, value);
        Ok(())
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}\n", self.method, self.uri, self.version)?;
        for (name, value) in self.headers.iter() {
            write!(f, "{}: {}\n", name, str::from_utf8(&value).unwrap())?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseState {
    RequestLine,
    Headers,
    Completed,
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
    state: ParseState,
    request: Request,
}

impl RequestBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            state: ParseState::RequestLine,
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
        let mut parse_start = 0;
        let mut parse_end = 0;
        // self.buffer may contain multiple lines(multiple CRLFs).
        loop {
            if let ParseState::Completed = self.state {
                return Ok(ParseState::Completed);
            }

            // Find "\r\n" to determine a line.
            if let Some(dist_to_crlf) = buf_iter.position(|&b| b == b'\r') {
                if let Some(b'\n') = buf_iter.next() {
                    parse_end += dist_to_crlf + 2;
                    match self.state {
                        ParseState::RequestLine => {
                            self.request
                                .parse_request_line(&self.buffer[parse_start..parse_end])?;
                            self.state = ParseState::Headers;
                        }
                        ParseState::Headers => {
                            if dist_to_crlf == 0 {
                                // CRLF only
                                self.state = ParseState::Completed;
                                break;
                            } else {
                                self.request
                                    .parse_header(&self.buffer[parse_start..parse_end])?
                            }
                        }
                        _ => unreachable!(),
                    }
                    parse_start = parse_end;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        self.buffer = self.buffer.drain(parse_end..).collect();
        Ok(self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_request() {
        let data = b"GET /~/index.html HTTP/1.1\r\n\r\n"
            .chunks(9)
            .map(|c| c.to_vec())
            .collect::<Vec<_>>();
        let mut request_buf = RequestBuffer::new();
        for message in data {
            match request_buf.try_parse(&message) {
                Ok(ParseState::Completed) => break,
                Ok(_) => continue,
                Err(err) => panic!("{:?}", err),
            }
        }
        assert_eq!(
            Request {
                method: Method::Get,
                uri: Uri::new(b"/~/index.html"),
                version: Version::OneDotOne,
                headers: HashMap::new(),
            },
            request_buf.complete()
        );
    }

    #[test]
    fn build_request_with_headers() {
        let data = b"GET /~/index.html HTTP/1.1\r\nAccept: */*\r\nHost: localhost:8080\r\n\r\n"
            .chunks(16)
            .map(|c| c.to_vec())
            .collect::<Vec<_>>();
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
                version: Version::OneDotOne,
                headers: vec![
                    (HeaderName::Accept, b"*/*".to_vec()),
                    (HeaderName::Host, b"localhost:8080".to_vec()),
                ]
                .into_iter()
                .collect(),
            },
            request_buf.complete()
        );
    }

    #[test]
    fn build_request_with_large_chunk() {
        let data = b"GET /~/index.html HTTP/1.1\r\nAccept: */*\r\nHost: localhost:8080\r\nUser-Agent: curl\r\n"
            .chunks(64)
            .map(|c| c.to_vec())
            .collect::<Vec<_>>();
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
                version: Version::OneDotOne,
                headers: vec![
                    (HeaderName::Accept, b"*/*".to_vec()),
                    (HeaderName::Host, b"localhost:8080".to_vec()),
                    (HeaderName::UserAgent, b"curl".to_vec()),
                ]
                .into_iter()
                .collect(),
            },
            request_buf.complete()
        );
    }
}
