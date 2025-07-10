// This file contains styling functions for UI components, including button styles and other visual elements.

use iced::{
    widget::{button},
    Theme,
    Color,
    Border,
    Shadow,
};

pub fn dir_button_style() -> impl Fn(&Theme, button::Status) -> button::Style {
    |_theme, _status| button::Style {
        background: None,
        text_color: Color::from_rgb(3.0 / 255.0, 161.0 / 255.0, 252.0 / 255.0),
        border: Border::default(),
        shadow: Shadow::default(),
    }
}