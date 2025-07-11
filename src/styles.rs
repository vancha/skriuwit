use cosmic::widget::button;
use crate::styles::button::Style;
use cosmic::style::Button;
use cosmic::iced::{color, Color, Vector, Radius, Background};
use cosmic::cosmic_theme::palette::convert::TryIntoColor;

pub fn custom_button_style() -> Button {
    Button::Custom {
        active: Box::new(|_focused, _theme| Style {
        ..button::Style::default()
	     /* shadow_offset: Vector::new(2.0, 2.0),
            background: Some(Background::Color(Color::from_rgb(0.8, 0.8, 0.8))),
            overlay: None,
            border_radius: Radius::from(0.0),
            border_width: 2.0,
            border_color: Color::BLACK,
            outline_width: 0.0,
            outline_color: Color::TRANSPARENT,
            icon_color: None,
            text_color: Some(Color::BLACK),*/
        }),
        disabled: Box::new(|_theme| Style {
 	    ..button::Style::default()
        }),
        hovered: Box::new(|_focused, _theme| Style {
 	    ..button::Style::default()
        }),
        pressed: Box::new(|_focused, _theme| Style {
 	    ..button::Style::default()
        }),
    }
}

pub fn selected_button_style() -> Button {
    Button::Custom {
        active: Box::new(|_focused, _theme| Style {
            background: Some(Background::Color( color!(0xf, 0x0, 0x0) )),
	        ..button::Style::default()
        }),

        disabled: Box::new(|_theme| Style {
            background: Some(Background::Color( color!(0x2A, 0x79, 0xD8))),
     	    ..button::Style::default()
        }),
        hovered: Box::new(|_focused, _theme| Style {
            background: Some(Background::Color( color!(0x2A, 0x79, 0xD8))),
	        ..button::Style::default()
        }),
        pressed: Box::new(|_focused, _theme| Style {
            background: Some(Background::Color( color!(0x2A, 0x79, 0xD8))),
	        ..button::Style::default()
        }),
    }
}

pub fn tag_button_style(color: Color) -> Button {
    Button::Custom {
        active: Box::new(|_focused, _theme| Style {
            background: Some(Background::Color(color!(0x2A, 0x79, 0xD8))),
	        ..button::Style::default()
        }),

        disabled: Box::new(|_theme| Style {
            background: Some(Background::Color(color!(0x2A, 0x79, 0xD8))),
     	    ..button::Style::default()
        }),
        hovered: Box::new(|_focused, _theme| Style {
            background: Some(Background::Color(color!(0x2A, 0x79, 0xD8))),
	        ..button::Style::default()
        }),
        pressed: Box::new(|_focused, _theme| Style {
            background: Some(Background::Color(color!(0xAF, 0x00, 0xFF))),
            ..button::Style::default()
        }),
    }
}

