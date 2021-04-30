use std::ffi::OsStr;
use std::path::Path;

macro_rules! define_mime_types {
    ($(($entry:ident, $lit:expr, $ext:expr),)+) => {
        $(
            pub const $entry: &'static [u8] = $lit.as_bytes();
        )+

        pub fn filename_to_mime<P: AsRef<Path>>(filename: P) -> &'static [u8] {
            match filename.as_ref().extension().and_then(OsStr::to_str) {
                $(
                Some($ext) => $entry,
                )+
                _ => b"application/octet-stream",
            }
        }
    };
}

define_mime_types!(
    (TEXT_PLAIN, "text/plain", "txt"),
    (TEXT_HTML, "text/html", "html"),
    (IMAGE_PNG, "image/png", "png"),
);
