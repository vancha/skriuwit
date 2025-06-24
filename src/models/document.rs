use cosmic::widget::icon;
use std::path::PathBuf;
#[derive(Debug, Clone)]
pub struct Document {
    pub icon: icon::Handle,
    pub title: String,
    pub added_date: String,
    pub path: PathBuf,
}

impl Document {
    pub fn new() -> Self {
        Self {
            icon: icon::from_name("text-x-generic").into(),
            title: String::from("invoice_2025.pdf"),
            added_date: String::from("2025-06-14"),
            path: PathBuf::from("/"),
        }
    }

    pub fn from_fields(title: String, added_date: String, path: PathBuf) -> Self {
        Self {
            icon: icon::from_name("text-x-generic").into(),
            title,
            added_date,
            path,
        }
    }
}
