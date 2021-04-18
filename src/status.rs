macro_rules! define_status_codes {
    ($(($num:expr, $num_str:expr, $entry:ident, $phrase:expr),)+) => {
        #[derive(Clone, Debug, PartialEq, Eq)]
        pub enum StatusCode {
            $(
            $entry,
            )+
        }

        impl StatusCode {
            pub fn code(&self) -> u16 {
                match &self {
                    $(
                    StatusCode::$entry => $num,
                    )+
                }
            }

            pub fn reason_phrase(&self) -> &'static str {
                match &self {
                    $(
                    StatusCode::$entry => $phrase,
                    )+
                }
            }

            pub fn as_str(&self) -> &'static str {
                match &self {
                    $(
                    StatusCode::$entry => $num_str,
                    )+
                }
            }
        }
    }
}

define_status_codes!(
    (200, "200", Ok, "OK"),
    (404, "404", NotFound, "Not Found"),
);

impl StatusCode {
    pub fn as_bytes(&self) -> [u8; 3] {
        let code = self.code();
        [
            (code / 100) as u8,
            (code / 10 % 10) as u8,
            (code % 10) as u8,
        ]
    }
}

impl Default for StatusCode {
    fn default() -> Self {
        StatusCode::Ok
    }
}
