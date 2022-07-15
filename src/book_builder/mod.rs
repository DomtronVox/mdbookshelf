mod indexer;
pub use indexer::index_books;

mod builder;
pub use builder::build_books;

mod hierarchy;
pub use hierarchy::compile_hierarchy;


use std::path::PathBuf;

use crate::book::BookshelfMetadata;

pub fn build_bookshelf(src: PathBuf, bld: PathBuf, bookshelf_directory: PathBuf) -> BookshelfMetadata {
    
    let books_index = index_books(&src);
    
    let books_metadata = build_books( books_index, src.clone(), bld.clone(), bookshelf_directory.clone() );
    
    let book_hierarchy = compile_hierarchy(books_metadata);
    
    BookshelfMetadata {
        source_directory: src,
        build_directory: bld,
        bookshelf_directory,
        
        book_hierarchy,
    }
}
