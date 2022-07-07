use std::{
    path::{PathBuf, Component},
    fs,
};

use walkdir::{
    WalkDir, 
    DirEntry,
};

use mdbook::{
    MDBook,
    //config::Config,
};

mod serve;
use serve::spawn_server;


//const VERSION: &str = concat!("v", crate_version!());

const DEFAULT_SOURCE_DIR: &str = "bookshelf";
const DEFAULT_BUILD_DIR: &str = "build";


///Enum indicating the type of book.
#[derive(PartialEq)]
enum BookType {
    MDBook,
    PDF,
}


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
fn find_books(path: &PathBuf) -> Vec<(BookType, PathBuf)> {
    let mut book_list = vec!();

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
        
        
        //we tell the iterator to skip the whole directory if this is an mdbook as it uses whole directories
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
        
        book_list.push( (book_type, source_path) );
    }
    
    book_list
}

///strips out everything from path before the source folder. 
///Note that the partial path is a string and should be joined into a path/pathbuf when used
fn isolate_partial_path(full_path: &PathBuf, source_path: &PathBuf) -> Result<PathBuf, String> {
    let mut partial_path = PathBuf::from("");
    
    let mut source_iter = source_path.components();
    
    //we need to strip out everything up to the given source_dir
    for component in full_path.components() {
        let src_comp = source_iter.next();
        
        //strip out everything not matching the source path.
        if src_comp != None {
            if component != src_comp.unwrap() {
                return Err(format!("Full path {:#?} does not match source path {:#?}. This should not be possible and is a bug. Terminating.", full_path, source_path).to_string());
            }
        
        //now that the source path is removed, everything else is added to our new path.
        } else if src_comp == None {
            partial_path.push(component);
        }
    }
    
    Ok(partial_path)
}



fn main() {
    //figure out the root source and build paths
    let source_path = std::env::current_dir().unwrap().join("bookshelf");
    let build_path = std::env::current_dir().unwrap().join("build");
    
    //compile list of books
    let book_list = find_books(&source_path);
    
    //process books by either copying files or triggering MDBook builds
    for book in book_list {
        //just to be clear
        let book_type = book.0;
        let book_source_path = book.1;
        
        //get the part of the path unique to the source directory. We use this same relative
        // path when placing stuff in the build dir.
        let path_part = isolate_partial_path(&book_source_path, &source_path).unwrap();

        //location to place the book.
        let book_build_path = build_path.join("bookshelf").join(path_part);
        
        match book_type {
        
            BookType::MDBook => {
            
                println!("MDBook source {}, building into {}", book_source_path.display(), book_build_path.display());
            
                //create book object from path which has the book.tomel and all needed info
                let mut md = MDBook::load(&book_source_path)
                    .expect("Unable to load the book");
                
                //we need to set the output to be inside the buildpath /bookshelf directory 
                md.config.build.build_dir = book_build_path;

                //Try to build the book
                md.build().expect("Building failed");
            
            },
        
            BookType::PDF => {
                println!("PDF source {}, copying into {}", book_source_path.display(), book_build_path.display());
            
                let mut directory_only = book_build_path.clone();
                directory_only.pop();
                println!("{}", directory_only.display());
                println!("{:?}", fs::create_dir_all(directory_only));
                println!("{:?}", fs::copy(&book_source_path, &book_build_path));
            },
        }
    }
    
    //spawn_server("./build".to_string(), "127.0.0.1", "3000");
    //loop{}
}


