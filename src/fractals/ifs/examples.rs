use include_dir::{include_dir, Dir};
use std::path::PathBuf;
use strum_macros::Display;
use thiserror::Error;

static IFS_EXAMPLES_DIR: Dir<'_> = include_dir!("./assets/fractals/ifs/");

#[derive(Copy, Clone, Display)]
pub enum Example {
    #[strum(serialize = "Barnsley's Fern")]
    BarnsleyFern,

    #[strum(serialize = "Binary")]
    Binary,

    #[strum(serialize = "Coral")]
    Coral,

    #[strum(serialize = "Crystal")]
    Crystal,

    #[strum(serialize = "Dragon")]
    Dragon,

    #[strum(serialize = "Floor")]
    Floor,

    #[strum(serialize = "Koch-3")]
    Koch3,

    #[strum(serialize = "Spiral")]
    Spiral,

    #[strum(serialize = "Tree")]
    Tree,

    #[strum(serialize = "Triangle")]
    Triangle,

    #[strum(serialize = "Whirlpool")]
    Whirlpool,

    #[strum(serialize = "Zigzag")]
    Zigzag,
}

impl Example {
    pub fn path(&self) -> PathBuf {
        match self {
            Example::BarnsleyFern => PathBuf::from(r"Barnsleys-Fern.json"),
            Example::Binary => PathBuf::from(r"Binary.json"),
            Example::Coral => PathBuf::from(r"Coral.json"),
            Example::Crystal => PathBuf::from(r"Crystal.json"),
            Example::Dragon => PathBuf::from(r"Dragon.json"),
            Example::Floor => PathBuf::from(r"Floor.json"),
            Example::Koch3 => PathBuf::from(r"Koch-3.json"),
            Example::Spiral => PathBuf::from(r"Spiral.json"),
            Example::Tree => PathBuf::from(r"Tree.json"),
            Example::Triangle => PathBuf::from(r"Triangle.json"),
            Example::Whirlpool => PathBuf::from(r"Whirlpool.json"),
            Example::Zigzag => PathBuf::from(r"Zigzag.json"),
        }
    }

    pub fn contents(&self) -> Result<String, ExampleLoadingError> {
        let file = IFS_EXAMPLES_DIR
            .get_file(self.path())
            .ok_or(ExampleLoadingError::FileNotFound)?;

        Ok(file
            .contents_utf8()
            .ok_or(ExampleLoadingError::NonValidUtf8)?
            .to_string())
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Example::BarnsleyFern,
            Example::Binary,
            Example::Coral,
            Example::Crystal,
            Example::Dragon,
            Example::Floor,
            Example::Koch3,
            Example::Spiral,
            Example::Tree,
            Example::Triangle,
            Example::Whirlpool,
            Example::Zigzag,
        ]
        .into_iter()
    }
}

#[derive(Error, Debug)]
pub enum ExampleLoadingError {
    #[error("File not found.")]
    FileNotFound,

    #[error("Not valid UTF-8 (or file is empty)")]
    NonValidUtf8,
}
