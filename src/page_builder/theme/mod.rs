//load in files so they are embeded into the binary.
//>Templates
pub static INDEX: &[u8] = include_bytes!("index.hbs");

//>files (CSS images etc)
pub static STYLESHEET: (&str, &[u8]) = ("functional.css", include_bytes!("functional.css") );
