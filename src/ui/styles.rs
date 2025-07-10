// This file contains styling functions for UI components, including button styles and other visual elements.

use iced::{
    widget::{button, container},
    Theme,
    Color,
    Border,
    Shadow,
    Background,
};

// 目录按钮样式
pub fn dir_button_style() -> impl Fn(&Theme, button::Status) -> button::Style {
    |_theme, status| button::Style {
        background: match status {
            button::Status::Hovered => Some(Background::Color(Color::from_rgb(0.2, 0.3, 0.4))),
            _ => Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))),
        },
        text_color: Color::from_rgb(0.3, 0.7, 1.0),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 8.0.into(),
        },
        shadow: Shadow::default(),
    }
}

// 普通按钮样式
pub fn primary_button_style() -> impl Fn(&Theme, button::Status) -> button::Style {
    |_theme, status| button::Style {
        background: match status {
            button::Status::Hovered => Some(Background::Color(Color::from_rgb(0.2, 0.5, 0.8))),
            button::Status::Pressed => Some(Background::Color(Color::from_rgb(0.1, 0.4, 0.7))),
            _ => Some(Background::Color(Color::from_rgb(0.15, 0.45, 0.75))),
        },
        text_color: Color::WHITE,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 4.0,
        },
    }
}

// Jrip 按钮样式
pub fn jrip_button_style() -> impl Fn(&Theme, button::Status) -> button::Style {
    |_theme, status| button::Style {
        background: match status {
            button::Status::Hovered => Some(Background::Color(Color::from_rgb(0.8, 0.3, 0.2))),
            button::Status::Pressed => Some(Background::Color(Color::from_rgb(0.7, 0.2, 0.1))),
            _ => Some(Background::Color(Color::from_rgb(0.9, 0.4, 0.3))),
        },
        text_color: Color::WHITE,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 6.0.into(),
        },
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            offset: iced::Vector::new(0.0, 2.0),
            blur_radius: 4.0,
        },
    }
}

// 弹出框样式
pub fn popup_container_style() -> impl Fn(&Theme) -> container::Style {
    |_theme| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.2, 0.3, 0.2))),
        border: Border {
            color: Color::from_rgb(0.4, 0.6, 0.4),
            width: 2.0,
            radius: 8.0.into(),
        },
        text_color: Some(Color::WHITE),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
            offset: iced::Vector::new(0.0, 4.0),
            blur_radius: 8.0,
        },
    }
}

// 主容器样式
pub fn main_container_style() -> impl Fn(&Theme) -> container::Style {
    |_theme| container::Style {
        background: Some(Background::Color(Color::from_rgb(0.05, 0.05, 0.08))),
        border: Border::default(),
        text_color: Some(Color::WHITE),
        shadow: Shadow::default(),
    }
}