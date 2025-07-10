// src/audio_ripper.rs
use std::path::PathBuf;
use std::process::Command;
use crate::app::AppState;

pub fn rip_audio(path_buf: PathBuf, state: &mut AppState) {
    if let Some(parent) = path_buf.parent() {
        let mut new_file = parent.to_path_buf();
        new_file.push("output.mp3");

        if let Ok(output) = Command::new("ffmpeg")
            .args([
                "-i",
                path_buf.to_str().unwrap_or("/home"),
                "-q:a", "2", // sound quality
                "-y",
                new_file.to_str().unwrap_or("/home"),
            ]).status() {
            if output.success() {
                state.popup = Some(String::from("Audio Has Been Ripped!"));
            } else {
                state.popup = Some(String::from("Error Ripping!"));
            }
        }
    } else {
        state.current_dir = path_buf.clone();
    }
}