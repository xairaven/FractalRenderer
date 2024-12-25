use strum_macros::Display;

#[derive(Display, Default, PartialEq)]
pub enum FractalType {
    #[default]
    #[strum(serialize = "Iterated Function System (IFS)")]
    Ifs,

    #[strum(serialize = "L-System")]
    LSystem,
}
