use std::path::PathBuf;
use std::{fs, io};

pub fn load_with_file_pick() -> Option<Result<String, io::Error>> {
    if let Some(path) = rfd::FileDialog::new().pick_file() {
        return Some(load_from_path(path));
    }

    None
}

pub fn load_from_path(path: PathBuf) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
