use cosmic::widget::icon;

#[derive(Debug, Clone)]
pub struct Document {
    icon: icon::Handle,
    pub title: String,
    added_date: String,
}

impl Document {
    pub fn new() -> Self {
        Self {
            icon: icon::from_name("rich-text").into(),
            title: String::from("invoice_2025.pdf"),
            added_date: String::from("Today"),
        }
    }
}
