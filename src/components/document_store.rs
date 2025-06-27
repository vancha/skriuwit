use rusqlite::{Connection, params};
use std::path::{Path, PathBuf};
use whoami;

use crate::models::{document::Document, tag::Tag};

pub struct DocumentStore {
    conn: Connection,
}

impl DocumentStore {
    pub fn new() -> Self {
        let username = whoami::username();
        let db_path = format!("/home/{username}/.local/share/skriuwit/documents.db");
        std::fs::create_dir_all(Path::new(&db_path).parent().unwrap()).unwrap();

        let conn = Connection::open(&db_path).unwrap();

        conn.execute(
            // In a future this will save more stuff, icon maybe?, some extra properties?, whatever, save it here.
            "CREATE TABLE IF NOT EXISTS documents (
                title TEXT NOT NULL,
                added_date BIGINT NOT NULL,
                path TEXT PRIMARY KEY
            )",
            (),
        )
        .unwrap();

        conn.execute(
            // In a future this will save more stuff, icon maybe?, some extra properties?, whatever, save it here.
            "CREATE TABLE IF NOT EXISTS tags (
                id  INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                color TEXT NOT NULL
            )",
            (),
        )
        .unwrap();

        DocumentStore { conn }
    }
    pub fn document_exists(&self, document: &Document) -> bool {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM documents WHERE path = ?1")
            .unwrap();
        stmt.exists(params![document.path.to_str()])
            .expect("Could not perform a query for some reason")
    }

    pub fn get_all_documents(&self) -> Vec<Document> {
        let mut stmt = self
            .conn
            .prepare("SELECT title, added_date, path FROM documents")
            .unwrap();
        let doc_iter = stmt
            .query_map([], |row| {
                Ok(Document::from_fields(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    PathBuf::from(row.get::<_, String>(2).unwrap()),
                ))
            })
            .unwrap();

        let mut docs = Vec::new();
        for doc in doc_iter {
            docs.push(doc.unwrap());
        }

        docs
    }

    pub fn upload_document(&self, doc: &Document) {
        if !self.document_exists(doc) {
            self.conn
                .execute(
                    "INSERT OR IGNORE INTO documents (title, added_date, path) VALUES (?1, ?2, ?3)",
                    params![doc.title, doc.added_date, doc.path.to_string_lossy()],
                )
                .unwrap();
        }
    }

    pub fn remove_document(&self, path: &Path) {
        self.conn
            .execute(
                "DELETE FROM documents WHERE path = ?1",
                params![path.to_string_lossy()],
            )
            .unwrap();
    }

    pub fn get_all_tags(&self) -> Vec<Tag> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, color FROM tags")
            .unwrap();

        let tag_iter = stmt
            .query_map([], |row| {
                Ok(Tag::from_fields(
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                ))
            })
            .unwrap();

        let mut tags = Vec::new();
        for tag in tag_iter {
            tags.push(tag.unwrap());
        }

        tags
    }
    pub fn upload_tag(&self, tag: &Tag) {
        self.conn
            .execute(
                "INSERT OR IGNORE INTO tags (id, title, color) VALUES (?1, ?2, ?3)",
                params![tag.id, tag.title, tag.hex_color],
            )
            .unwrap();
    }
}
