use std::convert::From;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Header {
    Accept(Vec<u8>),
    Host(Vec<u8>),
    UserAgent(Vec<u8>),
    Unknown,
}

impl From<(Vec<u8>, Vec<u8>)> for Header {
    fn from((mut name, value): (Vec<u8>, Vec<u8>)) -> Self {
        // RFC2616 says header name is case insensitive.
        // https://www.w3.org/Protocols/rfc2616/rfc2616-sec4.html#sec4.2
        name.make_ascii_lowercase();
        match &name[..] {
            b"accept" => Header::Accept(value),
            b"host" => Header::Host(value),
            b"user-agent" => Header::UserAgent(value),
            _ => Header::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_name_case_insensitive() {
        let name = b"accept".to_vec();
        let value = b"*/*".to_vec();
        assert_eq!(Header::Accept(b"*/*".to_vec()), Header::from((name, value)));
    }
}
