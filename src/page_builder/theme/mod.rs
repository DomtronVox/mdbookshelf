//load in files so they are embeded into the binary.
//>Templates
pub static INDEX: &[u8] = include_bytes!("index.hbs");

//>files (CSS images etc)
pub static FUNCTIONAL_STYLESHEET: (&str, &[u8]) = ("functional.css", include_bytes!("functional.css") );
pub static DARK_STYLESHEET: (&str, &[u8]) = ("style_dark.css", include_bytes!("style_dark.css") );
