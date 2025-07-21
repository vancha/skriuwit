use cosmic::iced::Color;

#[derive(Debug, Clone, Hash, Eq)]
pub struct Tag {
    /// Because sqlite's "INTEGER PRIMARY KEY" is a signed 64 bit integer
    pub id: Option<i64>,
    pub title: String,
    /// Valid options: #rrggbb, #rrggbbaa, #rgb, and #rgba
    pub hex_color: String,
    /// Persists the selected state, assumes we want to remember what was filtered on
    pub selected: bool,
}


impl Tag {
    //used when all fields are known (like when retrieving from the database)
    pub fn new(id: i64, title: String, hex_color:String, selected:bool) -> Self {
        Tag { id: Some(id), title, hex_color, selected }
    }

    pub fn from_fields(title: String, hex_color: String, selected:bool ) -> Self {
        Self {
            id: None,
            title,
            hex_color,
            selected
        }
    }

    pub fn get_color(&self) -> Color {
        Color::parse(&self.hex_color).unwrap()
    }
}


/// Convenience function for Document, used to check if a Vec contains a Document
impl std::cmp::PartialEq for Tag {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
