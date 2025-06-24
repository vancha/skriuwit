use crate::models::document::Document;
use std::path::Path;

pub struct DocumentSearchEngine {
    // Structures used to search faster
    // for now it will be only vector :)
    data: Vec<Document>
}

impl DocumentSearchEngine {
    pub fn new() -> Self {
        DocumentSearchEngine { data: Vec::new() }
    }

    pub fn get_all_documents(&self) -> Vec<Document> {
        self.data.clone()
    }

    pub fn add_document(&mut self, doc: Document) {
        self.data.push(doc);
    }
    
    pub fn remove_document(&mut self, path: &Path) {
        self.data.retain(|doc| doc.path != path);
    }
}