// This file contains utility functions for file operations, such as retrieving files from a specified directory.

use std::{fs, path::PathBuf};

pub fn get_files(path: &PathBuf) -> Vec<(String, bool)> {
    let mut dirs = Vec::default();
    let mut files = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path) {
        for read in read_dir {
            if let Ok(dir_entry) = read {
                if let Some(name) = dir_entry.file_name().to_str() {
                    if dir_entry.path().is_dir() {
                        dirs.push((name.to_string(), true));
                    } else {
                        if name.ends_with("mkv") {
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