use cosmic::widget::button;
use crate::styles::button::Style;
use cosmic::style::Button;
use cosmic::iced::{Color, Vector, Radius, Background};

pub fn custom_button_style() -> Button {
    Button::Custom {
        active: Box::new(|_focused, _theme| Style {
            shadow_offset: Vector::new(2.0, 2.0),
            background: Some(Background::Color(Color::from_rgb(0.8, 0.8, 0.8))),
            overlay: None,
            border_radius: Radius::from(5.0),
            border_width: 2.0,
            border_color: Color::BLACK,
            outline_width: 0.0,
            outline_color: Color::TRANSPARENT,
            icon_color: None,
            text_color: Some(Color::BLACK),
        }),
        disabled: Box::new(|_theme| Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
            overlay: None,
            border_radius: Radius::from(5.0),
            border_width: 1.0,
            border_color: Color::from_rgb(0.6, 0.6, 0.6),
            outline_width: 0.0,
            outline_color: Color::TRANSPARENT,
            icon_color: None,
            text_color: Some(Color::from_rgb(0.5, 0.5, 0.5)),
        }),
        hovered: Box::new(|_focused, _theme| Style {
            shadow_offset: Vector::new(4.0, 4.0),
            background: Some(Background::Color(Color::from_rgb(0.7, 0.7, 0.9))),
            overlay: None,
            border_radius: Radius::from(5.0),
            border_width: 2.0,
            border_color: Color::from_rgb(0.3, 0.3, 0.7),
            outline_width: 0.0,
            outline_color: Color::TRANSPARENT,
            icon_color: None,
            text_color: Some(Color::BLACK),
        }),
        pressed: Box::new(|_focused, _theme| Style {
            shadow_offset: Vector::new(1.0, 1.0),
            background: Some(Background::Color(Color::from_rgb(0.6, 0.6, 0.6))),
            overlay: None,
            border_radius: Radius::from(5.0),
            border_width: 2.0,
            border_color: Color::BLACK,
            outline_width: 0.0,
            outline_color: Color::TRANSPARENT,
            icon_color: None,
            text_color: Some(Color::BLACK),
        }),
    }
}

pub fn selected_button_style() -> Button {
    Button::Custom {
        active: Box::new(|_focused, _theme| Style {
            shadow_offset: Vector::new(2.0, 2.0),
            background: Some(Background::Color(Color::from_rgb8(0x3A, 0x99, 0xF8))),
            overlay: None,
            border_radius: Radius::from(4.0),
            border_width: 2.0,
            border_color: Color::from_rgb8(0x1A, 0x70, 0xD9),
            outline_width: 0.0,
            outline_color: Color::TRANSPARENT,
            icon_color: None,
            text_color: Some(Color::WHITE),
        }),
        disabled: Box::new(|_theme| Style {
            shadow_offset: Vector::new(0.0, 0.0),
            background: Some(Background::Color(Color::from_rgb8(0x6A, 0xB9, 0xF8))),
            overlay: None,
            border_radius: Radius::from(4.0),
            border_width: 1.0,
            border_color: Color::from_rgb8(0x1A, 0x70, 0xD9),
            outline_width: 0.0,
            outline_color: Color::TRANSPARENT,
            icon_color: None,
            text_color: Some(Color::from_rgb(0.7, 0.7, 0.7)),
        }),
        hovered: Box::new(|_focused, _theme| Style {
            shadow_offset: Vector::new(3.0, 3.0),
            background: Some(Background::Color(Color::from_rgb8(0x2A, 0x79, 0xD8))),
            overlay: None,
            border_radius: Radius::from(4.0),
            border_width: 2.0,
            border_color: Color::from_rgb8(0x14, 0x50, 0xA9),
            outline_width: 0.0,
            outline_color: Color::TRANSPARENT,
            icon_color: None,
            text_color: Some(Color::WHITE),
        }),
        pressed: Box::new(|_focused, _theme| Style {
            shadow_offset: Vector::new(1.0, 1.0),
            background: Some(Background::Color(Color::from_rgb8(0x1A, 0x59, 0xB8))),
            overlay: None,
            border_radius: Radius::from(4.0),
            border_width: 2.0,
            border_color: Color::from_rgb8(0x11, 0x40, 0x98),
            outline_width: 0.0,
            outline_color: Color::TRANSPARENT,
            icon_color: None,
            text_color: Some(Color::WHITE),
        }),
    }
}