// filepath: jrip/jrip/src/main.rs
mod app;
mod file_utils;
mod audio_ripper;
mod ui;

use app::{AppState, Message};
use ui::view::view;
use iced::Task;

fn main() -> iced::Result {
    iced::application("Jrip", update, view)
        .theme(|_s| iced::Theme::KanagawaDragon)
        .run()
}

fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Exit => Task::none(), // Handle exit logic
        Message::CD(path_buf) => {
            state.current_dir = path_buf;
            state.current_files = file_utils::get_files(&state.current_dir);
            Task::none()
        }
        Message::JRIP(path_buf) => {
            audio_ripper::rip_audio(path_buf, state);
            Task::none()
        }
        Message::ClosePopup => {
            state.popup = None;
            Task::none()
        }
    }
}