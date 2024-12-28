#[derive(Default)]
pub struct FileFilter {
    pub name: String,
    pub file_extensions: Vec<&'static str>,
}

impl FileFilter {
    pub fn json() -> Self {
        FileFilter {
            name: String::from("JSON"),
            file_extensions: vec!["json"],
        }
    }

    pub fn png() -> Self {
        FileFilter {
            name: String::from("PNG"),
            file_extensions: vec!["png"],
        }
    }
}
