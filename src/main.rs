use std::{fs, path::PathBuf, process::Command};

use iced::{
    widget::{button, column, horizontal_rule, row, text}, window, Border, Element, Length::Fill, Shadow, Task
};

#[derive(Debug)]
struct AppState {
    // Define the state of your application here
    current_dir:PathBuf,
    current_files:Vec<(String, bool)>,
    popup:Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let current_files =  get_files(&current_dir);
        AppState {
            current_dir,
            current_files,
            popup: None,
        }
    }
}

#[derive(Debug,Clone)]
enum Message {
    Exit,
    CD(PathBuf), // Change Directory message
    JRIP(PathBuf),
    ClosePopup
}

fn update(state: &mut AppState, message:Message) -> Task<Message> {
    match message {
        Message::Exit => window::get_latest().and_then(window::close), // Close the window when the Exit message is received
        Message::CD(path_buf) => {
            // Change the current directory and update the files
            state.current_dir = path_buf;
            state.current_files = get_files(&state.current_dir);
            Task::none() // No task to run, just update the state
        }
        Message::JRIP(path_buf) => {
            if let Some(parent) = path_buf.parent() {
                // If the path has a parent, change to that directory
                let mut new_file = parent.to_path_buf();
                new_file.push("output.mp3");

                if let Ok(output) = Command::new("ffmpeg")
                    .args(["-i",
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
                // If no parent, stay in the current directory
                state.current_dir = path_buf.clone();
            }
            Task::none() // No task to run, just log the action
        }
        Message::ClosePopup => {
            // Close the popup if it exists
            state.popup = None;
            Task::none() // No task to run, just update the state
        }
    }
}
    

fn view(state: &AppState) ->Element<Message> {
    
    let mut content = column![
        row![text(state.current_dir.to_str().unwrap_or("unknown dir"))
            .size(32)
            .width(Fill),
        button(text("Up").size(24)).on_press(Message::CD(
            state.current_dir.parent().unwrap_or(&state.current_dir).to_path_buf()
        )), // Go up one directory
        button(text("Exit").size(24)).on_press(Message::Exit),
        ]
        .spacing(8)]
        .spacing(2)
        .padding(4); //return an Element containing the UI components

    content = content.push(horizontal_rule(2));

    if let Some(pat) = &state.popup{
        
        content = content.push(row![
            text(pat).width(Fill),
            button(text("close").size(24)).on_press(Message::ClosePopup)
        ]);
        content = content.push(horizontal_rule(2));
    }
    
    for file in &state.current_files{
        let file_name = text(&file.0).size(18);
        let mut file_path = state.current_dir.clone();
        file_path.push(&file.0);
        if file.1{
            // If it's a directory, make it bold
            content = content.push(
                button(file_name)
                    .style(dir_button_style())
                    .on_press(Message::CD(file_path))) // Change directory to the selected one
        } else {
            // If it's a file, just add the text
            content = content.push(row![file_name.width(Fill),
                button(text("Jrip"))
                .on_press(Message::JRIP(file_path))
                ]);
        }
    }
    content.into()
}

fn dir_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style
{
    |_t, _e| button::Style {
        background: None,
        text_color: iced::Color::from_rgb(
            3.0 / 255.0,
            161.0 / 255.0,
            252.0 / 255.0,
        ),
        border: Border::default(),
        shadow: Shadow::default(),
    }
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
                        if name.ends_with("mkv"){
                            files.push((name.to_string(), false));
                        }                  
                    }
                }        
            }
        }
    }

    
    dirs.append(&mut files);
    dirs
    
}