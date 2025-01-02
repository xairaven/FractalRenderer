use include_dir::{include_dir, Dir};
use std::path::PathBuf;
use strum_macros::Display;
use thiserror::Error;

static LSYSTEM_EXAMPLES_DIR: Dir<'_> = include_dir!("./assets/fractals/l-system");

#[derive(Copy, Clone, Display)]
pub enum Example {
    #[strum(serialize = "Dragon Curve")]
    DragonCurve,

    #[strum(serialize = "Gosper Curve")]
    GosperCurve,

    #[strum(serialize = "Hilbert Curve")]
    HilbertCurve,

    #[strum(serialize = "Koch Curve")]
    KochCurve,

    #[strum(serialize = "Koch Quadratic Curve")]
    KochQuadraticCurve,

    #[strum(serialize = "Koch Quadratic Snowflake")]
    KochQuadraticSnowflake,

    #[strum(serialize = "Koch Snowflake")]
    KochSnowflake,

    #[strum(serialize = "L-System Bush 1")]
    LsystemBush1,

    #[strum(serialize = "L-System Bush 2")]
    LsystemBush2,

    #[strum(serialize = "L-System Bush 3")]
    LsystemBush3,

    #[strum(serialize = "L-System Sticks 1")]
    LsystemSticks1,

    #[strum(serialize = "L-System Sticks 2")]
    LsystemSticks2,

    #[strum(serialize = "Peano Curve")]
    PeanoFractal,

    #[strum(serialize = "Penrose Tiling")]
    PenroseTiling,

    #[strum(serialize = "Sierpiński Curve")]
    SierpinskiCurve,

    #[strum(serialize = "Sierpiński Rhombus")]
    SierpinskiRhombus,

    #[strum(serialize = "Sierpiński Triangle")]
    SierpinskiTriangle,
}

impl Example {
    pub fn path(&self) -> PathBuf {
        match self {
            Example::DragonCurve => PathBuf::from(r"DragonCurve.json"),
            Example::GosperCurve => PathBuf::from(r"GosperCurve.json"),
            Example::HilbertCurve => PathBuf::from(r"HilbertCurve.json"),
            Example::KochCurve => PathBuf::from(r"KochCurve.json"),
            Example::KochQuadraticCurve => PathBuf::from(r"KochQuadraticCurve.json"),
            Example::KochQuadraticSnowflake => {
                PathBuf::from(r"KochQuadraticSnowflake.json")
            },
            Example::KochSnowflake => PathBuf::from(r"KochSnowflake.json"),
            Example::LsystemBush1 => PathBuf::from(r"LsystemBush-1.json"),
            Example::LsystemBush2 => PathBuf::from(r"LsystemBush-2.json"),
            Example::LsystemBush3 => PathBuf::from(r"LsystemBush-3.json"),
            Example::LsystemSticks1 => PathBuf::from(r"LsystemSticks-1.json"),
            Example::LsystemSticks2 => PathBuf::from(r"LsystemSticks-2.json"),
            Example::PeanoFractal => PathBuf::from(r"PeanoFractal.json"),
            Example::PenroseTiling => PathBuf::from(r"PenroseTiling.json"),
            Example::SierpinskiCurve => PathBuf::from(r"SierpinskiCurve.json"),
            Example::SierpinskiRhombus => PathBuf::from(r"SierpinskiRhombus.json"),
            Example::SierpinskiTriangle => PathBuf::from(r"SierpinskiTriangle.json"),
        }
    }

    pub fn contents(&self) -> Result<String, ExampleLoadingError> {
        let file = LSYSTEM_EXAMPLES_DIR
            .get_file(self.path())
            .ok_or(ExampleLoadingError::FileNotFound)?;
        Ok(file
            .contents_utf8()
            .ok_or(ExampleLoadingError::NonValidUtf8)?
            .to_string())
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Example::DragonCurve,
            Example::GosperCurve,
            Example::HilbertCurve,
            Example::KochCurve,
            Example::KochQuadraticCurve,
            Example::KochQuadraticSnowflake,
            Example::KochSnowflake,
            Example::LsystemBush1,
            Example::LsystemBush2,
            Example::LsystemBush3,
            Example::LsystemSticks1,
            Example::LsystemSticks2,
            Example::PeanoFractal,
            Example::PenroseTiling,
            Example::SierpinskiCurve,
            Example::SierpinskiRhombus,
            Example::SierpinskiTriangle,
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
