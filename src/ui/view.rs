// This file defines the `view` function which constructs the UI layout for the application.
// It creates buttons, displays the current directory, and shows any popups.

use iced::{
    widget::{button, column, horizontal_rule, row, text}, 
    Element, Length::Fill
};
use crate::app::{AppState, Message};
use crate::ui::styles::dir_button_style;


pub fn view(state: &AppState) -> Element<Message> {
    let mut content = column![
        row![
            text(state.current_dir.to_str().unwrap_or("unknown dir"))
                .size(32)
                .width(Fill),
            button(text("Up").size(24)).on_press(Message::CD(
                state.current_dir.parent().unwrap_or(&state.current_dir).to_path_buf()
            )), // Go up one directory
            button(text("Exit").size(24)).on_press(Message::Exit),
        ]
        .spacing(8)]
        .spacing(2)
        .padding(4); // Return an Element containing the UI components

    content = content.push(horizontal_rule(2));

    if let Some(pat) = &state.popup {
        content = content.push(row![
            text(pat).width(Fill),
            button(text("close").size(24)).on_press(Message::ClosePopup)
        ]);
        content = content.push(horizontal_rule(2));
    }

    for file in &state.current_files {
        let file_name = text(&file.0).size(18);
        let mut file_path = state.current_dir.clone();
        file_path.push(&file.0);
        if file.1 {
            // If it's a directory, make it bold
            content = content.push(
                button(file_name)
                    .style(dir_button_style())
                    .on_press(Message::CD(file_path)) // Change directory to the selected one
            );
        } else {
            // If it's a file, just add the text
            content = content.push(row![
                file_name.width(Fill),
                button(text("Jrip"))
                    .on_press(Message::JRIP(file_path))
            ]);
        }
    }
    content.into()
}