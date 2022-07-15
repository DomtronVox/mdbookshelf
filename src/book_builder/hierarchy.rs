
use std::collections::HashMap;



use crate::book::{BookMetadata, HierarchyType};



pub fn compile_hierarchy(books_metadata: Vec<BookMetadata>) -> HierarchyType {
    
    let mut book_sort = HierarchyType::Section( HashMap::new() );
    
    //process each book
    for metadata in books_metadata {
        //we start at the highest level and move down from there based on the localized partial path
        let mut current_container = &mut book_sort;
        
        //build list of String components, each is considered a Hierarchy level
        let mut components: Vec<String> = 
            metadata.partial_path
                .components()
                .map(|comp| comp.as_os_str().to_str().unwrap().to_string() )
                .collect();
                
        //we need to remove the last component as it is always related to the book itself (eith filename or unique folder)
        components.pop();
        
        for level in components {
            //access hashmap
            if let HierarchyType::Section(hashmap) = current_container {
                
                //create shelf entry if this is the first time we have seen this shelf
                if ! hashmap.contains_key(&level) {
                    hashmap.insert( level.clone(), HierarchyType::Section( HashMap::new() ) );
                }
                
                
                //shift the level down one
                current_container = hashmap.get_mut(&level).unwrap(); //the above check ensures this will work
                
            } else {
                println!("Error: Bug has occured. This prompt should not have been possible to print.");
            }
        }
        
        //set book metadata at the lowest level
        if let HierarchyType::Section(hashmap) = current_container {
            hashmap.insert( metadata.title.clone(), HierarchyType::Book(metadata) );
        }
    }
    
    book_sort
}
