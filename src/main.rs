use std::{
    path::{PathBuf, MAIN_SEPARATOR},
    fs,
};


mod book;

mod builder;
use builder::build_bookshelf;

mod index_theme;
use index_theme::render_index;

mod serve;
use serve::spawn_server;


//const VERSION: &str = concat!("v", crate_version!());

const DEFAULT_SOURCE_DIR: &str = "bookshelf";
const DEFAULT_BUILD_DIR: &str = "build";



fn main() {
    //figure out the root source and build paths
    let source_path = std::env::current_dir().unwrap().join("bookshelf");
    let build_path = std::env::current_dir().unwrap().join("build");
    let bookshelf_directory = PathBuf::from("bookshelf");
    
    let bookshelf_metadata = build_bookshelf(source_path, build_path, bookshelf_directory);
    
    //render_index(&build_path, bookshelf_metadata);
    
    spawn_server("./build".to_string(), "127.0.0.1", "3000");
    println!("Serving at http://127.0.0.1:3000/ ");
    loop{}
}


