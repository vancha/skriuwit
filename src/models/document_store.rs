use crate::models::document::Document;

#[derive(Clone, Debug)]
pub struct DocumentStore {
	documents: Vec<Document>,
}

impl DocumentStore {

    pub async fn new() -> Self {
        let mut x = vec![];
        for i in 0..100 {
            let mut d = Document::new();
            d.title = format!("{}-{}",d.title,i.to_string());
            x.push(d);
        }
        DocumentStore { documents: x }
    }

    /*
        return all documents
    */
    pub fn get_all_documents(self) -> Vec<Document> {
       self.documents
    }

    /*
        returns all known tags
    */
    pub fn get_all_tags(self) -> Vec<Tag> {

    }

    /*
        registers a new tag, should be stored in the database
    */
    pub fn register_tag(&mut self) {P

    }
    /*
        these filter the documents based on what users type in the search field
    */
    pub fn filter_on_content(&mut self, content: String) {

    }

    /*
        these filter the documents based on which tags the users have selected
    */
    pub fn filter_on_tag(&mut self, tag: String) {

    }


}

