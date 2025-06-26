use std::path::{Path, PathBuf};
use rusqlite::{params, Connection};
use whoami;

use crate::models::document::Document;

pub struct DocumentStore {
    conn: Connection,
}

impl DocumentStore {
    pub fn new() -> Self {
        let username = whoami::username();
        let db_path = format!("/home/{}/.local/share/skriuwit/documents.db", username);
        std::fs::create_dir_all(Path::new(&db_path).parent().unwrap()).unwrap();

        let conn = Connection::open(&db_path).unwrap();

        conn.execute( // In a future this will save more stuff, icon maybe?, some extra properties?, whatever, save it here.
            "CREATE TABLE IF NOT EXISTS documents (
                title TEXT NOT NULL,
                added_date TEXT NOT NULL,
                path TEXT PRIMARY KEY
            )",
            (),
        ).unwrap();

        DocumentStore { conn }
    }

    pub fn get_all_documents(&self) -> Vec<Document> {
        let mut stmt = self.conn.prepare("SELECT title, added_date, path FROM documents").unwrap();
        let doc_iter = stmt.query_map([], |row| {
            Ok(Document::from_fields(
                row.get(0).unwrap(),
                row.get(1).unwrap(),
                PathBuf::from(row.get::<_, String>(2).unwrap())))
        }).unwrap();

        let mut docs = Vec::new();
        for doc in doc_iter {
            docs.push(doc.unwrap());
        }

        docs
    }

    pub fn upload_document(&self, doc: &Document) {
        self.conn.execute(
            "INSERT OR IGNORE INTO documents (title, added_date, path) VALUES (?1, ?2, ?3)",
            params![doc.title, doc.added_date, doc.path.to_string_lossy()],
        ).unwrap();
    }

    pub fn remove_document(&self, path: &Path) {
        self.conn.execute(
            "DELETE FROM documents WHERE path = ?1",
            params![path.to_string_lossy()],
        ).unwrap();
    }
}
