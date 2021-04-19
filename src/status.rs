macro_rules! define_status_codes {
    ($(($num:expr, $entry:ident, $phrase:expr),)+) => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum StatusCode {
            $(
            $entry,
            )+
        }

        impl StatusCode {
            /// Status code as an integer.
            pub const fn code(&self) -> u16 {
                match &self {
                    $(
                    StatusCode::$entry => $num,
                    )+
                }
            }

            /// Reason phrase corresponding to each status code.
            pub const fn reason_phrase(&self) -> &'static [u8] {
                match &self {
                    $(
                    StatusCode::$entry => $phrase.as_bytes(),
                    )+
                }
            }
        }
    }
}

define_status_codes!((200, Ok, "OK"), (404, NotFound, "Not Found"),);

impl StatusCode {
    const ASCII_ZERO: u8 = 48;

    /// Convert status code into 3 bytes of ASCII.
    pub const fn as_bytes(&self) -> [u8; 3] {
        let code = self.code();
        [
            (code / 100) as u8 + Self::ASCII_ZERO,
            (code / 10 % 10) as u8 + Self::ASCII_ZERO,
            (code % 10) as u8 + Self::ASCII_ZERO,
        ]
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::Ok
    }
}
