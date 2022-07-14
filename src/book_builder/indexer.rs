
use std::path::PathBuf;

use walkdir::{
    WalkDir, 
    DirEntry,
};


use crate::book::BookType;


///Checks if the given entry is a valid book or not
fn is_book(entry: &DirEntry) -> Option<BookType> {
    let file_name = entry.file_name()
                         .to_str()
                         .expect("Failed to convert path to string. This should not have happened!")
                         .to_lowercase();
    
    // mdbook config file detection    
    if file_name == "book.toml" {
        //note that mdbook wants the directory not the actual book.toml file so we use parent()
        Option::Some( BookType::MDBook )
    
    // PDF file detection
    } else if file_name.ends_with(".pdf") {
        Option::Some( BookType::PDF )

    //catch all for all other files and directories
    } else {
        Option::None
    }
}


///Walks a given path looking for either PDFs or book.tomel files
pub fn index_books(path: &PathBuf) -> Vec<(BookType, PathBuf)> {
    let mut search_results = vec!();

    //build the iterator so we can start the dir walk
    let mut it = WalkDir::new(path).into_iter();
    
    //we need to control the iterator a little more closely then a simple for loop allows
    loop {
        
        //acquire next entry
        let entry = match it.next() {
            Some(Ok(entry)) => entry,
            None => break, //if there is no next we end the loop
            _ => continue, //we just skip error files and 
        };
        
        
        //test if entry denotes a valid book we can recognize
        let book_type = match is_book(&entry) {
            //if not we continue loop to the next iteration
            None => continue,            
            //if so we return it to be processed
            Some(book_type) => book_type
        };
        
        
        //we tell the iterator to skip the whole directory if this is an mdbook as it uses the whole sub-tree
        if book_type == BookType::MDBook {
            it.skip_current_dir();
        }
        

        //figure out source path 
        let source_path = match book_type {
            BookType::MDBook => 
                entry.path()
                     .parent() 
                     .expect("Failed to acquire parent directory of book. This should not have happened!")
                     .to_path_buf(),
        
            BookType::PDF =>
                entry.path().to_path_buf(),
        };
        
        search_results.push( (book_type, source_path) );
    }
    
    search_results
}


