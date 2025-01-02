use strum_macros::Display;

#[derive(Copy, Clone, Display, Default, PartialEq)]
pub enum FractalType {
    #[default]
    #[strum(serialize = "Iterated Function System (IFS)")]
    Ifs,

    #[strum(serialize = "L-System")]
    LSystem,
}

pub mod ifs {
    pub mod examples;
    pub mod model;
    pub mod serialization;
    pub mod state;
    pub mod system;
    pub mod ui {
        pub mod parameters;
        pub mod settings;
    }
    pub mod validation;
}
pub mod lsystem {
    pub mod examples;
    pub mod model;
    pub mod serialization;
    pub mod state;
    pub mod ui {
        pub mod settings;
    }
    pub mod validation;
}
