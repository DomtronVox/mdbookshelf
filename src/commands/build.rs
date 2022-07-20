use std::path::PathBuf;


use clap::{Command, ArgMatches};


use crate::book_builder::build_bookshelf;
use crate::page_builder::build_pages;


pub fn build_bookshelf_cmd() {

    //figure out the root source and build paths and define the bookshelf directory
    //>Note pushing "" forces PathBuf to add a trailing /. only easy way I could find to do it.
    //TODO need to be pulling this from a config instead.
    let bookshelf_directory = PathBuf::from("bookshelf");
    let source_path = std::env::current_dir().unwrap().join(&bookshelf_directory).join("");
    let build_path = std::env::current_dir().unwrap().join("build").join("");
    
    //Compile book hierarchy and build all books into the build directory
    let bookshelf_metadata = build_bookshelf(source_path, build_path, bookshelf_directory);
    
    //Use book hierarchy data to build an index page that links to everything.
    build_pages(bookshelf_metadata);

}

// Create clap subcommand arguments for build
pub fn make_subcommand_build<'help>() -> Command<'help> {
    Command::new("build")
        .about("Builds a bookshelf from its source directory contents.")
}


// Build command implementation
pub fn execute_build(_args: &ArgMatches) -> Result<(), anyhow::Error> {

    build_bookshelf_cmd();

    Ok(())
}
