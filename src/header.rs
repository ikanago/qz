use std::{convert::From, fmt};

pub type HeaderValue = Vec<u8>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum HeaderName {
    Accept,
    Authorization,
    ContentLength,
    ContentType,
    Host,
    Location,
    UserAgent,
    WwwAuthenticate,
    Unknown,
}

impl AsRef<[u8]> for HeaderName {
    fn as_ref(&self) -> &[u8] {
        match self {
            HeaderName::Accept => b"Accept",
            HeaderName::Authorization => b"Authorization",
            HeaderName::ContentLength => b"Content-Length",
            HeaderName::ContentType => b"Content-Type",
            HeaderName::Host => b"Host",
            HeaderName::Location => b"Location",
            HeaderName::UserAgent => b"User-Agent",
            HeaderName::WwwAuthenticate => b"WWW-Authenticate",
            HeaderName::Unknown => b"Unknown",
        }
    }
}

impl fmt::Display for HeaderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(self.as_ref()).unwrap())
    }
}

impl From<Vec<u8>> for HeaderName {
    fn from(mut name: Vec<u8>) -> Self {
        // RFC2616 says header name is case insensitive.
        // https://www.w3.org/Protocols/rfc2616/rfc2616-sec4.html#sec4.2
        name.make_ascii_lowercase();
        match &name[..] {
            b"accept" => HeaderName::Accept,
            b"authorization" => HeaderName::Authorization,
            b"content-length" => HeaderName::ContentLength,
            b"content-type" => HeaderName::ContentType,
            b"location" => HeaderName::Location,
            b"host" => HeaderName::Host,
            b"user-agent" => HeaderName::UserAgent,
            b"www-authenticate" => HeaderName::WwwAuthenticate,
            _ => HeaderName::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_name_case_insensitive() {
        assert_eq!(HeaderName::Accept, HeaderName::from(b"accept".to_vec()));
    }
}
