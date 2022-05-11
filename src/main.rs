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



///Enume indicating the type of book and path to it's source info
enum BookType {
    MDBook(PathBuf),
    PDF(PathBuf),
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
        Option::Some( BookType::MDBook(
            entry.path()
                 .parent() 
                 .expect("Failed to aquire parent directory of book.toml. This should not have happened!")
                 .to_path_buf()
        ))
    
    // PDF file detection
    } else if file_name.ends_with(".pdf") {
        Option::Some( BookType::PDF( entry.path().to_path_buf() ) )

    //catch all for all other files and directories
    } else {
        Option::None
    }
}


///Walks a given path looking for either PDFs or book.tomel files
fn find_books(path: &str) -> Vec<BookType> {
    let mut book_list = vec!();

    //build the iterator so we can start the dir walk
    let mut it = WalkDir::new(path).into_iter();
    
    //we need to control the iterator a little more closely then a simple for loop allows
    loop {
        //aquire next entry
        let entry = match it.next() {
            Some(Ok(entry)) => entry,
            None => break, //if there is no next we end the loop
            _ => continue, //we just skip error files and 
        };
        
        //test if entry denotes a valid book
        let book_data = match is_book(&entry) {
            //if not we continue loop to the next iteration
            None => continue,            
            //if so we return it to be processed
            Some(book_type) => book_type
        };
        
        //we skip the whole directory if this is an mdbook as it uses whole directories
        if let BookType::MDBook(_) = book_data  {
            it.skip_current_dir();
        }
        
        book_list.push(book_data);
    }
    
    book_list
}

///Inserts the given string as the first directory into the given path. 
fn move_root_directory(path: &PathBuf, new_root: &str) -> PathBuf {
    //we need to strip out the first folder level
    let mut comps = path.components();
    comps.next(); // skip ./
    
    //now we add in cwd then the new_root and finally the rest of the path
    let mut new_path = PathBuf::from(".");
    new_path.push(new_root);
    new_path.push(comps.as_path());
     
    new_path
}

///because of the way mdbook works, this function reworks the source path so it points to a **relative**
//  path from the mdbook source folder to the mdbookshelf build directory. it's a bit convoluted.
fn move_mdbook_path_to_build(mdbook_source: &PathBuf, build_root: &str) -> PathBuf {
    //first we create a relative path to the same location as the source data but under the build directory
    let cwd_relative = move_root_directory(mdbook_source, build_root);
    
    //Because running build on mdbook struct works from the source data's folder NOT CWD, 
    //  we have to adjust the relative path up to cwd then back down
    let mut new_path = PathBuf::from(".");
    
    //essentially replace each directory component with "go up a level"
    for comp in mdbook_source.components() {
        if let Component::RootDir = comp { continue; } 
        if let Component::CurDir = comp { continue; } 
        new_path.push("..");
    }
    
    //mangle everything together and return
    new_path.join({ 
        let mut comps = cwd_relative.components();
        comps.next(); //strip out root
        comps.as_path()
    })
}


fn main() {

    let book_list = find_books("./bookshelf");
    
    for book in book_list {
        if let BookType::MDBook(path) = book {
            let build_path = move_mdbook_path_to_build(&path, "build");
            
            println!("MDBook at {}, into (relative from previous path) {}", path.display(), build_path.display());
            
            //create book object from path which has the book.tomel and all needed info
            let mut md = MDBook::load(&path)
                .expect("Unable to load the book");
                
            //we need to set the output to be inside the bookshelf output 
            md.config.build.build_dir = build_path;

            //Try to build the book
            md.build().expect("Building failed");
            
        } else if let BookType::PDF(path) = book {
            let build_path = move_root_directory(&path, "build");
            
            println!("PDF at {}, into {}", path.display(), build_path.display());
            
            let mut directory_only = build_path.clone();
            directory_only.pop();
            println!("{}", directory_only.display());
            println!("{:?}", fs::create_dir_all(directory_only));
            println!("{:?}", fs::copy(&path, &build_path));
        }
    }
    
    spawn_server("./build".to_string(), "127.0.0.1", "3000");
    loop{}
}


