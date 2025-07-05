use chrono::DateTime;
use cosmic::widget::icon;
use std::path::{Path, PathBuf};
use crate::models::tag::Tag;
use cosmic::{Element, widget::{Row, Column, button, text}};
use crate::app::Message;
use cosmic::iced::{Alignment, Length, Padding, Pixels, Subscription};


#[derive(Debug, Clone)]
pub struct Document {
    pub icon: icon::Handle,
    pub title: String,
    ///date as a timestamp
    pub added_date: i64,
    pub path: PathBuf,
    pub tags: Vec<Tag>,
}

impl Document {
    pub fn new(path: &Path) -> Self {
        let icon = icon::from_path(path.to_path_buf()).into();
        let title = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown")
            .to_string();
        let added_date = chrono::Local::now().timestamp();
        let path = path.to_path_buf();
        let tags = vec![];

        Self {
            icon,
            title,
            added_date,
            path,
            tags,
        }
    }

    pub fn from_fields(title: String, added_date: i64, path: PathBuf) -> Self {
        let icon = icon::from_path(path.clone()).into();
        println!("{:?}", &icon);
        Self {
            icon: icon,
            title,
            added_date,
            path,
            tags:vec![],
        }
    }

    pub fn get_tags(&self) -> Vec<&Tag> {
        self.tags.iter().collect::<Vec<_>>()
    }

    pub fn pretty_print_added_date(&self) -> String {
        if let Some(added_date) = DateTime::from_timestamp(self.added_date, 0)
        {
            if  let Some(current_date) =
                DateTime::from_timestamp(chrono::Local::now().timestamp(), 0) {
            match current_date - added_date {
                difference if difference.num_minutes() < 1 => {
                    format!("{} seconds ago", difference.num_seconds())
                }
                difference if difference.num_hours() < 1 => {
                    format!("{} minutes ago", difference.num_minutes())
                }
                difference if difference.num_days() < 1 => {
                    format!("{} hours ago", difference.num_hours())
                }
                difference if difference.num_weeks() < 1 => {
                    format!("{} days ago", difference.num_days())
                }
                difference if difference.num_weeks() < 4 => {
                    format!("{} weeks ago", difference.num_weeks())
                }
                difference if difference.num_weeks() < 52 => {
                    format!("{} months ago", difference.num_weeks() / 4)
                }
                _ => {
                    "Today somewhere or something".to_string()
                }
            }
                }
                else {
                    "Unknown date".to_string()
                }
        } else {
            "Unknown date".to_string()
        }
    }

    pub fn view(&self) -> Element<Message> {
        Row::with_children(vec![
            icon(self.icon.clone())
                .width(Length::Fixed(100.0))
                .height(Length::Fixed(100.0))
                .into(),
            Column::from_vec(vec![
                text::heading(&self.title).into(),
                Row::from_vec(
                    self.get_tags()
                        .into_iter()
                        .map(|tag| button::text(tag.title.as_str()))
                        .map(Into::<Element<Message>>::into)
                        .collect::<Vec<_>>(),
                )
                .into(),
                text::body(format!("Added: {}", self.pretty_print_added_date())).into(),
            ])
            .into(),
        ])
        .width(Length::Fill)
        .into()
    }
}

/// Convenience function for Document, used to check if a Vec contains a Document
impl std::cmp::PartialEq for Document {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}
