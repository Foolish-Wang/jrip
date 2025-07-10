// This file defines the `view` function which constructs the UI layout for the application.
// It creates buttons, displays the current directory, and shows any popups.

use iced::{
    widget::{button, column, container, horizontal_rule, row, text, Space, Image, Stack}, 
    Element, Length, Alignment, Color, Background, Border, ContentFit
};
use crate::app::{AppState, Message};
use crate::ui::styles::{dir_button_style, primary_button_style, jrip_button_style, popup_container_style, main_container_style};


pub fn view(state: &AppState) -> Element<Message> {
    let header = row![
        text("[DIR] ")
            .size(20)
            .color(Color::from_rgb(0.8, 0.8, 0.8)),
        text(state.current_dir.to_str().unwrap_or("unknown dir"))
            .size(20)
            .width(Length::Fill)
            .color(Color::from_rgb(0.9, 0.9, 0.9)),
        button(text("↑ Up").size(16))
            .style(primary_button_style())
            .padding(8)
            .on_press(Message::CD(
                state.current_dir.parent().unwrap_or(&state.current_dir).to_path_buf()
            )),
        Space::with_width(8),
        button(text("Exit").size(16))
            .style(primary_button_style())
            .padding(8)
            .on_press(Message::Exit),
    ]
    .spacing(8)
    .padding(12)
    .align_y(Alignment::Center);

    let mut content_column = column![header]
        .spacing(8);

    content_column = content_column.push(horizontal_rule(1));

    // 弹出框
    if let Some(popup_text) = &state.popup {
        let popup = container(
            row![
                text(popup_text)
                    .size(16)
                    .width(Length::Fill)
                    .color(Color::WHITE),
                button(text("X").size(14))
                    .style(primary_button_style())
                    .padding(4)
                    .on_press(Message::ClosePopup)
            ]
            .spacing(8)
            .padding(12)
            .align_y(Alignment::Center)
        )
        .style(popup_container_style())
        .padding(4);
        
        content_column = content_column.push(popup);
        content_column = content_column.push(Space::with_height(8));
    }

    // 文件列表
    for file in &state.current_files {
        let mut file_path = state.current_dir.clone();
        file_path.push(&file.0);
        
        if file.1 {
            // 目录
            let dir_row = container(
                button(
                    row![
                        text("[DIR] ").size(16),
                        text(&file.0).size(16)
                    ]
                    .spacing(4)
                    .align_y(Alignment::Center)
                )
                .style(dir_button_style())
                .padding(8)
                .width(Length::Fill)
                .on_press(Message::CD(file_path))
            )
            .padding(2);
            
            content_column = content_column.push(dir_row);
        } else {
            // 文件
            let file_row = container(
                row![
                    text("[FILE] ").size(16),
                    text(&file.0)
                        .size(16)
                        .width(Length::Fill)
                        .color(Color::from_rgb(0.8, 0.8, 0.8)),
                    button(text("JRIP").size(14))
                        .style(jrip_button_style())
                        .padding(6)
                        .on_press(Message::JRIP(file_path))
                ]
                .spacing(8)
                .padding(8)
                .align_y(Alignment::Center)
            )
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::from_rgba(0.08, 0.08, 0.12, 0.75))),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: 4.0.into(),
                },
                text_color: None,
                shadow: iced::Shadow::default(),
            })
            .padding(2);
            
            content_column = content_column.push(file_row);
        }
    }

    let ui_content = container(content_column)
        .style(main_container_style())
        .padding(8)
        .width(Length::Fill)
        .height(Length::Fill);

    Stack::new()
        .push(
            Image::new("assets/background.jpg")
                .width(Length::Fill)
                .height(Length::Fill)
                .content_fit(ContentFit::Cover),
        )
        .push(ui_content)
        .into()
}