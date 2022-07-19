
use crate::book::{BookMetadata, HierarchySection};



pub fn compile_hierarchy(books_metadata: Vec<BookMetadata>) -> HierarchySection {
    
    let mut book_sort = HierarchySection::default();
    
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
                
            //create shelf entry if this is the first time we have seen this shelf
            if ! current_container.sub_sections.contains_key(&level) {
                let mut new_section = HierarchySection::default();
                new_section.name = level.clone();
                current_container.sub_sections.insert( level.clone(), new_section );
            }
                
                
            //shift the level down one
            //Note: the above check ensures this unwrap will work
            current_container = current_container.sub_sections.get_mut(&level).unwrap(); 
                
        }
        
        //set book metadata at the lowest level
        current_container.books.push( metadata );
    }
    
    book_sort
}
