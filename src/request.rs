use crate::{
    body::Body,
    header::{HeaderName, HeaderValue},
    method::Method,
    parser::Parser,
    status::StatusCode,
    Uri, Version,
};
use std::{collections::HashMap, fmt, str};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct RequestBuilder {
    inner: Request,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            inner: Request::default(),
        }
    }

    pub fn set_method(mut self, method: Method) -> Self {
        self.inner.method = method;
        self
    }

    pub fn set_uri(mut self, uri: Uri) -> Self {
        self.inner.uri = uri;
        self
    }

    pub fn get_header(&self, name: HeaderName) -> Option<&HeaderValue> {
        self.inner.headers.get(&name)
    }

    pub fn set_header(mut self, name: HeaderName, value: impl Into<HeaderValue>) -> Self {
        self.inner.headers.insert(name, value.into());
        self
    }

    pub fn set_body(mut self, body: impl Into<Body>) -> Self {
        self.inner.set_body(body);
        self
    }

    fn parse_request_line(&mut self, bytes: &[u8]) -> crate::Result<()> {
        let mut p = Parser::new(bytes);
        let (method, uri, version) = p.parse_request_line()?;
        self.inner.method = method;
        self.inner.uri = uri;
        self.inner.version = version;
        Ok(())
    }

    fn parse_header(&mut self, bytes: &[u8]) -> crate::Result<()> {
        let mut p = Parser::new(bytes);
        let (name, value) = p.parse_header()?;
        self.inner.headers.insert(name, value);
        Ok(())
    }

    fn parse_body(&mut self, bytes: &[u8]) -> crate::Result<()> {
        let body_len = std::str::from_utf8(
            self.inner
                .get_header(HeaderName::ContentLength)
                .ok_or(StatusCode::LengthRequired)?,
        )
        .or(Err(StatusCode::LengthRequired))?
        .parse::<usize>()
        .or(Err(StatusCode::LengthRequired))?;
        let mut p = Parser::new(bytes);
        let body = p.parse_body(body_len)?;
        self.inner.set_body(body);
        Ok(())
    }

    pub fn build(self) -> Request {
        self.inner
    }
}

/// Represents HTTP request. This struct is built from `RequestBuffer` and passed to `Handler`.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Request {
    pub(crate) method: Method,
    pub(crate) uri: Uri,
    pub(crate) version: Version,
    pub(crate) headers: HashMap<HeaderName, HeaderValue>,
    pub(crate) body: Body,
}

impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::default()
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn headers(&self) -> &HashMap<HeaderName, HeaderValue> {
        &self.headers
    }

    pub fn get_header(&self, name: HeaderName) -> Option<&HeaderValue> {
        self.headers.get(&name)
    }

    pub fn set_header(&mut self, name: HeaderName, value: impl Into<HeaderValue>) {
        self.headers.insert(name, value.into());
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn set_body(&mut self, body: impl Into<Body>) {
        self.body = body.into();
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {} {}", self.method, self.uri, self.version)?;
        for (name, value) in self.headers.iter() {
            writeln!(f, "{}: {}", name, str::from_utf8(&value).unwrap())?;
        }
        writeln!(f, "{}", self.body())?;
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
/// * Parse line by line
///
/// Although the first option is easier, parsing request with message body is difficult
/// because it is hard to know when to finish reading from socket.
/// So this struct parse a request with second strategy to get body size from `Content-Length`
/// header while parsing.
pub struct RequestBuffer {
    buffer: Vec<u8>,
    state: ParseState,
    builder: RequestBuilder,
}

impl RequestBuffer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            state: ParseState::RequestLine,
            builder: RequestBuilder::new(),
        }
    }

    pub fn complete(self) -> Request {
        self.builder.build()
    }

    /// Extend buffer of this struct with `data` and try to parse given request data.
    pub fn try_parse(&mut self, data: &[u8]) -> crate::Result<ParseState> {
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
                            self.builder
                                .parse_request_line(&self.buffer[parse_start..parse_end])?;
                            self.state = ParseState::Headers;
                        }
                        ParseState::Headers => {
                            if dist_to_crlf == 0 {
                                // CRLF only
                                if self.builder.get_header(HeaderName::ContentLength).is_some() {
                                    self.builder.parse_body(&self.buffer[(parse_start + 2)..])?;
                                }
                                self.state = ParseState::Completed;
                            } else {
                                self.builder
                                    .parse_header(&self.buffer[parse_start..parse_end])?;
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
            Request::builder()
                .set_method(Method::Get)
                .set_uri(Uri::new(b"/~/index.html"))
                .build(),
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
            Request::builder()
                .set_method(Method::Get)
                .set_uri(Uri::new(b"/~/index.html"))
                .set_header(HeaderName::Accept, b"*/*".to_vec())
                .set_header(HeaderName::Host, b"localhost:8080".to_vec())
                .build(),
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
            Request::builder()
                .set_method(Method::Get)
                .set_uri(Uri::new(b"/~/index.html"))
                .set_header(HeaderName::Accept, b"*/*".to_vec())
                .set_header(HeaderName::Host, b"localhost:8080".to_vec())
                .set_header(HeaderName::UserAgent, b"curl".to_vec())
                .build(),
            request_buf.complete()
        );
    }

    #[test]
    fn build_request_with_body() {
        let data = b"GET /~/index.html HTTP/1.1\r\nContent-Length: 13\r\n\r\nHello, World!"
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
            Request::builder()
                .set_method(Method::Get)
                .set_uri(Uri::new(b"/~/index.html"))
                .set_header(HeaderName::ContentLength, b"13".to_vec())
                .set_body(b"Hello, World!".to_vec())
                .build(),
            request_buf.complete()
        );
    }
}
