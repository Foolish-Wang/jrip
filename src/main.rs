// filepath: jrip/jrip/src/main.rs
mod app;
mod file_utils;
mod audio_ripper;
mod ui;

use app::{AppState, Message};
use ui::view::view;
use iced::{Task, window};

fn main() -> iced::Result {
    // 移除了设置窗口图标的 .window() 调用
    iced::application("Jrip", update, view)
        .run()
}

fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Exit => window::get_latest().and_then(window::close),
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