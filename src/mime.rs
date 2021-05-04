use std::ffi::OsStr;
use std::path::Path;

macro_rules! define_mime_types {
    ($(($entry:ident, $lit:expr),)+) => {
        $(
            pub const $entry: &'static [u8] = $lit.as_bytes();
        )+
    };
}

define_mime_types!(
    (TEXT_PLAIN, "text/plain"),
    (TEXT_HTML, "text/html"),
    (TEXT_CSS, "text/css"),
    (TEXT_JAVASCRIPT, "text/javascript"),
    (IMAGE_JPG, "image/jpg"),
    (IMAGE_PNG, "image/png"),
    (APPLICATION_WWW_FORM, "application/x-www-form-urlencoded"),
);

pub fn filename_to_mime<P: AsRef<Path>>(filename: P) -> &'static [u8] {
    match filename.as_ref().extension().and_then(OsStr::to_str) {
        Some("txt") => TEXT_PLAIN,
        Some("html") => TEXT_HTML,
        Some("css") => TEXT_CSS,
        Some("js") => TEXT_JAVASCRIPT,
        Some("jpg") => IMAGE_JPG,
        Some("png") => IMAGE_PNG,
        _ => b"application/octet-stream",
    }
}
