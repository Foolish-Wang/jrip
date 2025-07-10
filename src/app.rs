// filepath: jrip/jrip/src/app.rs
use std::path::PathBuf;

#[derive(Debug)]
pub struct AppState {
    pub current_dir: PathBuf,
    pub current_files: Vec<(String, bool)>,
    pub popup: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let current_files = crate::file_utils::get_files(&current_dir);
        AppState {
            current_dir,
            current_files,
            popup: None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Exit,
    CD(PathBuf),
    JRIP(PathBuf),
    ClosePopup,
}