use std::fs;
//use std::io::Read;
//use std::path::PathBuf;

use log;

use handlebars::Handlebars;
use serde_json::json;

mod theme;
use theme::*;

use crate::book::BookshelfMetadata;



//render the index.html file from data and the template
pub fn render_index(metadata: &BookshelfMetadata) {
    let mut handlebars = Handlebars::new();
    
    if let Result::Err(err) = handlebars.register_template_string("index", &String::from_utf8_lossy(INDEX)) {
        log::error!("{}", err);
        panic!("Error registering template. See log.");
    }
    
    let mut data = std::collections::HashMap::new();

    //insert metadata
    data.insert("source_directory",    json!(metadata.source_directory));
    data.insert("build_directory",     json!(metadata.build_directory));
    data.insert("bookshelf_directory", json!(metadata.bookshelf_directory));
    data.insert("hierarchy",    json!(metadata.book_hierarchy));
    
    log::debug!("Template Data report: {:#?}", data);
    let file_render = handlebars.render("index", &data)
                        .expect("Handlebars encountered an error rendering the template from our data.");
    
    if let Result::Err(err) = 
      fs::write( format!("{}/{}", metadata.build_directory.display(), "index.html").as_str(), file_render.as_str()) {
        log::error!("{}", err);
        panic!("Error building template. See log.");
    }

}


pub fn build_pages(data: BookshelfMetadata) {
    
    
    //copy files over
    for (filename, file_data) in vec!(FUNCTIONAL_STYLESHEET, DARK_STYLESHEET) {
        if let Result::Err(err) = 
          fs::write( format!("{}", data.build_directory.join(filename).display()).as_str(), file_data ) {
            log::error!("{}", err);
            panic!("Error copying files into build directory. See log.");
        }
    }
    
    //process template files to build pages
    render_index(&data);
}
