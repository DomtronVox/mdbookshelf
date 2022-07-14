use std::path::PathBuf;


mod book;

mod book_builder;
use book_builder::build_bookshelf;

mod page_builder;
use page_builder::build_pages;

mod serve;
use serve::spawn_server;


//const VERSION: &str = concat!("v", crate_version!());

//const DEFAULT_SOURCE_DIR: &str = "bookshelf";
//const DEFAULT_BUILD_DIR: &str = "build";



fn main() {
    //figure out the root source and build paths and define the bookshelf directory
    let source_path = std::env::current_dir().unwrap().join("bookshelf");
    let build_path = std::env::current_dir().unwrap().join("build");
    let bookshelf_directory = PathBuf::from("bookshelf");
    
    let bookshelf_metadata = build_bookshelf(source_path, build_path, bookshelf_directory);
    
    build_pages(bookshelf_metadata);
    
    spawn_server("./build".to_string(), "127.0.0.1", "3000");
    println!("Serving at http://127.0.0.1:3000/ ");
    loop{}
}


