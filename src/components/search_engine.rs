use crate::models::document::Document;
use std::path::Path;
use crate::models::tag::Tag;

pub struct DocumentSearchEngine {
    // Structures used to search faster
    // for now it will be only vector :)
    data: Vec<Document>,
}

impl DocumentSearchEngine {
    pub fn new() -> Self {
        DocumentSearchEngine { data: Vec::new() }
    }

    pub fn get_all_documents(&self) -> Vec<&Document> {
        self.data.iter().collect::<Vec<_>>()//clone()
    }

    pub fn add_document(&mut self, doc: Document) {
        if !self.data.contains(&doc) {
            self.data.push(doc);
        }
    }

    pub fn remove_document(&mut self, path: &Path) {
        self.data.retain(|doc| doc.path != path);
    }

    //should get a list of tags
    pub fn filter_by_tags(&self, tags: Vec<Tag>) -> Vec<&Document> {
        self.data
            .iter()
            .filter(|doc| doc.tags.iter().any(|doc_tag|tags.contains(doc_tag)) ).collect::<Vec<_>>()
    }
}
