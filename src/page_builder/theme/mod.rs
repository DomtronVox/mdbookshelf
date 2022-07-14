//load in files so they are embeded into the binary.
//>Templates
pub static INDEX: &[u8] = include_bytes!("index.hbs");

//>files (CSS images etc)
pub static STYLESHEET: (&str, &[u8]) = ("core_style.css", include_bytes!("core_style.css") );
