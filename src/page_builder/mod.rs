use std::fs;
//use std::io::Read;
//use std::path::PathBuf;

use handlebars::Handlebars;
use serde_json::json;

mod theme;
use theme::*;

use crate::book::BookshelfMetadata;



//render the index.html file from data and the template
pub fn render_index(metadata: &BookshelfMetadata) {
    let mut handlebars = Handlebars::new();
    
    handlebars.register_template_string("index", &String::from_utf8_lossy(INDEX));
    
    let mut data = std::collections::HashMap::new();

    //insert metadata
    data.insert("source_directory",    json!(metadata.source_directory));
    data.insert("build_directory",     json!(metadata.build_directory));
    data.insert("bookshelf_directory", json!(metadata.bookshelf_directory));
    data.insert("hierarchy",    json!(metadata.book_hierarchy));
    
    println!("Template Data report: {:#?}", data);
    let file_render = handlebars.render("index", &data)
                        .expect("Error parsing string.");
    
    fs::write( format!("{}/{}", metadata.build_directory.display(), "index.html").as_str(), file_render.as_str()).unwrap();

}


pub fn build_pages(data: BookshelfMetadata) {
    
    
    //copy files over
    for (filename, file_data) in vec!(FUNCTIONAL_STYLESHEET, DARK_STYLESHEET) {
        fs::write( format!("{}", data.build_directory.join(filename).display()).as_str(), file_data );
    }
    
    //process template files to build pages
    render_index(&data);
}
