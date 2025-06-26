use cosmic::iced::Color;

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: u32,
    pub title: String,
    pub hex_color: String
}


impl Tag {
    pub fn new() -> Self {
        Tag { id:0, title: String::from("Example tag"), hex_color: String::from("#0f0")}
    }

    pub fn from_fields(id: u32, title: String, hex_color: String ) -> Self {
        Self {
            id,
            title,
            hex_color
        }
    }

    pub fn get_color(&self) -> Color {
        Color::parse(&self.hex_color).unwrap()
    }
}
