use std::fs::{self, File};
use std::io::Read;
use std::path::PathBuf;

use handlebars::Handlebars;
use serde_json::json;


use crate::BookMetadata;


//load in files so they are embeded into the binary.
pub static INDEX: &[u8] = include_bytes!("index.hbs");


//render the index.html file from data and the template
pub fn render_index(build_dir: &PathBuf, books: Vec<BookMetadata>) {
    let mut handlebars = Handlebars::new();
    
    handlebars.register_template_string("index", &String::from_utf8_lossy(INDEX));
    
    let mut data = std::collections::HashMap::new();

    //insert metadata
    data.insert("meta", json!({
        "book_dir": "bookshelf"
    }));

    //insert book list data
    data.insert("books", json!(books));
    
    let file_render = handlebars.render("index", &data)
                        .expect("Error parsing string.");
    
    fs::write( format!("{}/{}", build_dir.display(), "index.html").as_str(), file_render.as_str()).unwrap();

}



