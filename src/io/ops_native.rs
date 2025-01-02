use crate::io::filter::FileFilter;
use std::path::PathBuf;
use std::{fs, io};

pub fn load_with_file_pick(file_filter: FileFilter) -> Option<Result<String, io::Error>> {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter(file_filter.name, &file_filter.file_extensions)
        .pick_file()
    {
        return Some(load_from_path(path));
    }

    None
}

pub fn save_with_file_pick(
    text: String, file_filter: FileFilter,
) -> Option<Result<(), io::Error>> {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter(file_filter.name, &file_filter.file_extensions)
        .save_file()
    {
        return Some(save_to_path(path, text));
    }

    None
}

pub fn load_from_path(path: PathBuf) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

pub fn save_to_path(path: PathBuf, text: String) -> Result<(), io::Error> {
    fs::write(path, text)
}
