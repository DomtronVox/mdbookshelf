use std::{
    path::PathBuf,
    collections::HashMap,
};


use serde::Serialize;


///Enum indicating the type of book.
#[derive(Debug, Serialize, PartialEq)]
pub enum BookType {
    MDBook,
    PDF,
}


///Struct with data about a single book
#[derive(Debug, Serialize)]
pub struct BookMetadata {
    pub book_type: BookType,
    
    pub title: String,
    pub description: String,
    
    pub source_path: PathBuf,
    pub partial_path: PathBuf, //path isolated from src or target directory
    pub build_path: PathBuf,
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum HierarchyType {
    Section(HashMap<String, HierarchyType>),
    Book(BookMetadata),
}


#[derive(Debug, Serialize)]
pub struct BookshelfMetadata {
    pub source_directory: PathBuf,
    pub build_directory: PathBuf,
    pub bookshelf_directory: PathBuf,

    pub book_hierarchy: HierarchyType,
    
}

