use std::path::Path;
use whoami;
use crate::models::document::Document;

pub struct DocumentStore {
    
}

impl DocumentStore {

    pub fn new() -> Self{
        let username = whoami::username();
        let path = Path::new(&format!("/home/{}/.local/share/myapp/documents.db", username));
        // create db in this file, if already created, load the contents in the app
        DocumentStore {}
    }

    pub fn get_all() -> Vec<Document> {
        Vec::new()
    }

    pub fn upload_document(path: &Path) {

    }

    pub fn remove_document(path: &Path) {

    }
}