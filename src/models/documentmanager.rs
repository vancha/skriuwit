use crate::models::document::Document;
use std::path::Path;

pub struct DocumentManager {
    documents: Vec<Document>,
}

impl DocumentManager {
    pub fn new() -> Self {
	Self { documents: vec![] }
    }
    ///"Reverse index search", lets you get only the documents that have a certain tag associated with them
    pub fn get_documents_by_tagname(tagname: String) -> Option<Vec<Document>> {
        // Mock data, this should filter the documents in self by the tagname (which should probably be a list of tagnames as well)
        Some(vec![Document::new(), Document::new(), Document::new()])
    }

    /*
        Loads all data, the documents, as well as the metadata that stores which tags are associated with which documents
    */
    pub fn load_data_from_disk(path: &Path) -> Vec<Document> {
        vec![]
    }
}
