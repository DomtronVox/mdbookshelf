use std::{
    path::{PathBuf, MAIN_SEPARATOR},
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

use serde::Serialize;

mod index_theme;
use index_theme::render_index;

mod serve;
use serve::spawn_server;


//const VERSION: &str = concat!("v", crate_version!());

const DEFAULT_SOURCE_DIR: &str = "bookshelf";
const DEFAULT_BUILD_DIR: &str = "build";


///Enum indicating the type of book.
#[derive(Serialize, PartialEq)]
pub enum BookType {
    MDBook,
    PDF,
}


///Struct with data about a single book
#[derive(Serialize)]
pub struct BookMetadata {
    pub book_type: BookType,
    
    pub title: String,
    pub description: String,
    
    pub source_path: PathBuf,
    pub partial_path: PathBuf, //path isolated from src or target directory
    pub build_path: PathBuf,
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
        
        search_results.push( (book_type, source_path) );
    }
    
    search_results
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
    
    //holds metadata on each book for the library generator
    let mut book_list = vec!();
    
    //process books by either copying files or triggering MDBook builds
    for book in find_books(&source_path) {
        //just to be clear
        let book_type = book.0;
        let book_source_path = book.1;
        
        //get the part of the path unique to the source directory. We use this same relative
        // path when placing stuff in the build dir.
        let partial_path = isolate_partial_path(&book_source_path, &source_path).unwrap();

        //location to place the book.
        let book_build_path = build_path.join("bookshelf").join(&partial_path);
        
        //some book metadata
        let mut title = book_source_path.file_name().unwrap().to_os_string().into_string().unwrap();
        let mut description = "".to_string();
        
        match book_type {
        
            BookType::MDBook => {
            
                println!("> MDBook source {}, building into {}\n", book_source_path.display(), book_build_path.display());
            
                //create book object from path which has the book.tomel and all needed info
                let mut md = MDBook::load(&book_source_path)
                    .expect("Unable to load the book");
                
                //we need to set the output to be inside the buildpath /bookshelf directory 
                md.config.set( "build.build_dir", book_build_path.clone() );
                
                //Try to build the book
                md.build().expect("Building failed");
                
                //pull some data from the mdbook config
                title = md.config.book.title.unwrap_or(title);
                description = md.config.book.description.unwrap_or("".to_string()).clone();
            
            },
        
            BookType::PDF => {
                println!("> PDF source {}, copying into {}\n", book_source_path.display(), book_build_path.display());
                
                //get a directory only path so we can make sure all directories up to the needed one exist
                let mut directory_only = book_build_path.clone();
                directory_only.pop();
                fs::create_dir_all(directory_only)
                    .map_err(|err| println!("{:#?}", err));

                //copy the file over
                fs::copy(&book_source_path, &book_build_path)
                    .map_err(|err| println!("{:#?}", err));
            },
        }
        
        //create metadata object we will need to populate the index template
        book_list.push(
            BookMetadata {
                book_type,
                title, description,
                
                source_path: book_source_path,
                partial_path,
                build_path: book_build_path,
            }
        );
    }
    
    render_index(&build_path, book_list);
    
    spawn_server("./build".to_string(), "127.0.0.1", "3000");
    println!("Serving at http://127.0.0.1:3000/ ");
    loop{}
}


