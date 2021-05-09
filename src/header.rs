use std::{convert::From, fmt};

pub type HeaderValue = Vec<u8>;

macro_rules! define_headers {
    ($(($name:ident, $upper_str:expr, $lower_str:expr),)+) => {
        #[derive(Clone, Debug, Hash, PartialEq, Eq)]
        pub enum HeaderName {
            $($name,)+
            Unknown,
        }

        impl AsRef<[u8]> for HeaderName {
            fn as_ref(&self) -> &[u8] {
                match self {
                    $(HeaderName::$name => $upper_str,)+
                    HeaderName::Unknown => b"Unknown",
                }
            }
        }

        impl From<Vec<u8>> for HeaderName {
            fn from(mut name: Vec<u8>) -> Self {
                // RFC2616 says a header name is case insensitive.
                // https://www.w3.org/Protocols/rfc2616/rfc2616-sec4.html#sec4.2
                name.make_ascii_lowercase();
                match &name[..] {
                    $($lower_str => HeaderName::$name,)+
                    _ => HeaderName::Unknown
                }
            }
        }
    }
}

define_headers!(
    (Accept, b"Accept", b"accept"),
    (
        AccessControlAllowHeaders,
        b"Access-Control-Allow-Headers",
        b"access-control-allow-headers"
    ),
    (
        AccessControlAllowMethods,
        b"Access-Control-Allow-Methods",
        b"access-control-allow-methods"
    ),
    (
        AccessControlAllowOrigin,
        b"Access-Control-Allow-Origin",
        b"access-control-allow-origin"
    ),
    (
        AccessControlAllowMaxAge,
        b"Access-Control-Max-Age",
        b"access-control-max-age"
    ),
    (Authorization, b"Authorization", b"authorization"),
    (ContentLength, b"Content-Length", b"content-length"),
    (ContentType, b"Content-Type", b"content-type"),
    (Host, b"Host", b"host"),
    (Location, b"Location", b"location"),
    (Origin, b"Origin", b"origin"),
    (UserAgent, b"User-Agent", b"user-agent"),
    (WwwAuthenticate, b"WWW-Authenticate", b"www-authenticate"),
);

impl fmt::Display for HeaderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", std::str::from_utf8(self.as_ref()).unwrap())
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
