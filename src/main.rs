use std::{fs, path::{self, PathBuf}};

use iced::{
    widget::{button, column, text, row}, 
    window, 
    Element, 
    Task, 
    Length::Fill
};

#[derive(Debug)]
struct AppState {
    // Define the state of your application here
    current_dir:PathBuf,
    current_files:Vec<(String, bool)>,
}

impl Default for AppState {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let current_files =  get_files(&current_dir);
        AppState {
            current_dir,
            current_files,
        }
    }
}

#[derive(Debug,Clone)]
enum Message {
    Exit
}

fn update(state: &mut AppState, message:Message) -> Task<Message> {
    match message {
        Message::Exit => window::get_latest().and_then(window::close), // Close the window when the Exit message is received
    }
}
    

fn view(state: &AppState) ->Element<Message> {
    
    let mut content = column![
        row![text(state.current_dir.to_str().unwrap_or("unknown dir")).size(32).width(Fill),
        button(text("Up").size(24)).on_press(Message::Exit),
        button(text("Exit").size(24)).on_press(Message::Exit),
        ].spacing(8)]; //return an Element containing the UI components

    for file in &state.current_files{
        content = content.push(text(&file.0))
    }
    
    content.into()
}

fn main() ->iced::Result {
    iced::application("Jrip", update, view)
    .theme(|_s| iced::Theme::KanagawaDragon)
    .run()
}

fn get_files(path: &PathBuf) -> Vec<(String, bool)> {
    let mut dirs = Vec::default();
    let mut files = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path){
        for read in read_dir {
            if let Ok(dir_entry) = read {
                if let Some(name) = dir_entry.file_name().to_str(){
                    if dir_entry.path().is_dir(){
                        dirs.push((name.to_string(), true));
                    } else {
                        files.push((name.to_string(), false));
                    }
                }        
            }
        }
    }

    
    dirs.append(&mut files);
    dirs
    
}