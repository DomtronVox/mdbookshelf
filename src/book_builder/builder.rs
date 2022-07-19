use std::{
    path::PathBuf,
    fs,
};

use log;

use mdbook::{
    MDBook,
    //config::Config,
};


use crate::book::{BookType, BookMetadata};


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


///Builds books and assembles a list of book metadata objects from the resulting info.
pub fn build_books(books_index: Vec<(BookType, PathBuf)>, 
             source_path: PathBuf, build_path: PathBuf, bookshelf_directory: PathBuf) -> Vec<BookMetadata> {
    
    let mut books_metadata = vec!();
    
    //process books by either copying files or triggering MDBook builds
    for book in books_index {
        //just to be clear what's what
        let book_type = book.0;
        let mut book_source_path = book.1;
        
        //get the part of the path unique to the source directory. We use this same relative
        // path when placing stuff in the build dir.
        let mut partial_path = isolate_partial_path(&book_source_path, &source_path).unwrap();

        //location to place the book.
        let mut book_build_path = build_path.join(&bookshelf_directory).join(&partial_path);
        
        //based on book type we build the book then return title and description metadata
        let (title, description) = match book_type {
        
            BookType::MDBook => {
                //Really dumb but this is the only way I could find to add a trailing slash easily
                //Need a trailing slash since the MDBook messes up the web template links otherwise
                //Add it to all 3 just to be consistant.
                book_source_path.push("");
                book_build_path.push("");
                partial_path.push("");
                
                log::info!("Bulding MDBook \"{}\"", partial_path.display());
                log::debug!("> MDBook source {}, building into {}\n", 
                            book_source_path.display(), book_build_path.display());
                
                //create book object from path which has the book.tomel and all needed info
                let mut md = MDBook::load(&book_source_path)
                    .expect("Unable to load the book");
                
                //we need to set the output to be inside the books individual build directory 
                md.config.build.build_dir = book_build_path.clone();
                
                //Try to build the book
                md.build().expect("Building failed");
                
                //pull some data from the mdbook config
                let title = md.config.book.title.expect("MDBook missing title somehow.");
                let description = md.config.book.description.unwrap_or("".to_string()).clone();
                
                (title, description)
            },
        
            BookType::PDF => {
                log::info!("Bulding PDF \"{}\"", partial_path.display());
                log::debug!("> PDF source {}, copying into {}\n", book_source_path.display(), book_build_path.display());
                
                //get a directory only path so we can make sure all directories up to the needed one exist
                let mut directory_only = book_build_path.clone();
                directory_only.pop();
                
                if let Result::Err(err) = fs::create_dir_all(directory_only) {
                    log::error!("{:#?}", err);
                }

                //copy the file over
                if let Result::Err(err) = fs::copy(&book_source_path, &book_build_path) {
                    log::error!("{:#?}", err);
                }
                    
                let title = book_source_path.file_stem().unwrap().to_os_string().into_string().unwrap();

                ( title, "".to_string() )
            },
        };
        
        //create metadata object we will need to populate the index template
        books_metadata.push(
            BookMetadata {
                book_type,
                title, description,
                
                source_path: book_source_path,
                partial_path,
                build_path: book_build_path,
            }
        );
    }
    
    books_metadata
}
