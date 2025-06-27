use cosmic::iced::Color;

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: u32,
    pub title: String,
    pub hex_color: String,
    pub selected: bool,
}


impl Tag {

    pub fn from_fields(id: u32, title: String, hex_color: String ) -> Self {
        Self {
            id,
            title,
            hex_color,
            selected:false,
        }
    }

    pub fn get_color(&self) -> Color {
        Color::parse(&self.hex_color).unwrap()
    }
}
