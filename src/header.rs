use std::convert::From;

pub type HeaderValue = Vec<u8>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum HeaderName {
    Accept,
    Host,
    UserAgent,
    Unknown,
}

impl From<Vec<u8>> for HeaderName {
    fn from(mut name: Vec<u8>) -> Self {
        // RFC2616 says header name is case insensitive.
        // https://www.w3.org/Protocols/rfc2616/rfc2616-sec4.html#sec4.2
        name.make_ascii_lowercase();
        match &name[..] {
            b"accept" => HeaderName::Accept,
            b"host" => HeaderName::Host,
            b"user-agent" => HeaderName::UserAgent,
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
