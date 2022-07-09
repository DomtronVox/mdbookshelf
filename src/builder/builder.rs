use std::{
    path::{PathBuf, MAIN_SEPARATOR},
    fs,
};


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
        let book_source_path = book.1;
        
        //get the part of the path unique to the source directory. We use this same relative
        // path when placing stuff in the build dir.
        let partial_path = isolate_partial_path(&book_source_path, &source_path).unwrap();

        //location to place the book.
        let book_build_path = build_path.join(&bookshelf_directory).join(&partial_path);
        
        //some book metadata
        let mut title = book_source_path.file_name().unwrap().to_os_string().into_string().unwrap();
        let mut description = "".to_string();
        
        match book_type {
        
            BookType::MDBook => {
            
                println!("> MDBook source {}, building into {}\n", book_source_path.display(), book_build_path.display());
            
                //create book object from path which has the book.tomel and all needed info
                let mut md = MDBook::load(&book_source_path)
                    .expect("Unable to load the book");
                
                //we need to set the output to be inside the books individual build directory 
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
