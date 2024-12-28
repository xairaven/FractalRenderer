use std::path::PathBuf;
use strum_macros::Display;

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
            Example::BarnsleyFern => {
                PathBuf::from(r"assets/fractals/ifs/Barnsleys-Fern.json")
            },
            Example::Binary => PathBuf::from(r"assets/fractals/ifs/Binary.json"),
            Example::Coral => PathBuf::from(r"assets/fractals/ifs/Coral.json"),
            Example::Crystal => PathBuf::from(r"assets/fractals/ifs/Crystal.json"),
            Example::Dragon => PathBuf::from(r"assets/fractals/ifs/Dragon.json"),
            Example::Floor => PathBuf::from(r"assets/fractals/ifs/Floor.json"),
            Example::Koch3 => PathBuf::from(r"assets/fractals/ifs/Koch-3.json"),
            Example::Spiral => PathBuf::from(r"assets/fractals/ifs/Spiral.json"),
            Example::Tree => PathBuf::from(r"assets/fractals/ifs/Tree.json"),
            Example::Triangle => PathBuf::from(r"assets/fractals/ifs/Triangle.json"),
            Example::Whirlpool => PathBuf::from(r"assets/fractals/ifs/Whirlpool.json"),
            Example::Zigzag => PathBuf::from(r"assets/fractals/ifs/Zigzag.json"),
        }
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
