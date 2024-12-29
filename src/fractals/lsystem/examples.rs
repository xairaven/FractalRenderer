use std::path::PathBuf;
use strum_macros::Display;

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
            Example::DragonCurve => {
                PathBuf::from(r"assets/fractals/l-system/DragonCurve.json")
            },
            Example::GosperCurve => {
                PathBuf::from(r"assets/fractals/l-system/GosperCurve.json")
            },
            Example::HilbertCurve => {
                PathBuf::from(r"assets/fractals/l-system/HilbertCurve.json")
            },
            Example::KochCurve => {
                PathBuf::from(r"assets/fractals/l-system/KochCurve.json")
            },
            Example::KochQuadraticCurve => {
                PathBuf::from(r"assets/fractals/l-system/KochQuadraticCurve.json")
            },
            Example::KochQuadraticSnowflake => {
                PathBuf::from(r"assets/fractals/l-system/KochQuadraticSnowflake.json")
            },
            Example::KochSnowflake => {
                PathBuf::from(r"assets/fractals/l-system/KochSnowflake.json")
            },
            Example::LsystemBush1 => {
                PathBuf::from(r"assets/fractals/l-system/LsystemBush-1.json")
            },
            Example::LsystemBush2 => {
                PathBuf::from(r"assets/fractals/l-system/LsystemBush-2.json")
            },
            Example::LsystemBush3 => {
                PathBuf::from(r"assets/fractals/l-system/LsystemBush-3.json")
            },
            Example::LsystemSticks1 => {
                PathBuf::from(r"assets/fractals/l-system/LsystemSticks-1.json")
            },
            Example::LsystemSticks2 => {
                PathBuf::from(r"assets/fractals/l-system/LsystemSticks-2.json")
            },
            Example::PeanoFractal => {
                PathBuf::from(r"assets/fractals/l-system/PeanoFractal.json")
            },
            Example::PenroseTiling => {
                PathBuf::from(r"assets/fractals/l-system/PenroseTiling.json")
            },
            Example::SierpinskiCurve => {
                PathBuf::from(r"assets/fractals/l-system/SierpinskiCurve.json")
            },
            Example::SierpinskiRhombus => {
                PathBuf::from(r"assets/fractals/l-system/SierpinskiRhombus.json")
            },
            Example::SierpinskiTriangle => {
                PathBuf::from(r"assets/fractals/l-system/SierpinskiTriangle.json")
            },
        }
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
