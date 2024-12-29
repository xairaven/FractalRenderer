use crate::fractals::ifs::ui::settings::IfsSettingsBlock;
use crate::fractals::lsystem::ui::settings::LSystemSettingsBlock;
use crate::ui::components::settings::SettingsBlock;
use strum_macros::Display;

#[derive(Copy, Clone, Display, Default, PartialEq)]
pub enum FractalType {
    #[default]
    #[strum(serialize = "Iterated Function System (IFS)")]
    Ifs,

    #[strum(serialize = "L-System")]
    LSystem,
}

impl FractalType {
    pub fn ui(&self) -> Box<dyn SettingsBlock> {
        match self {
            FractalType::Ifs => Box::new(IfsSettingsBlock::default()),
            FractalType::LSystem => Box::new(LSystemSettingsBlock::default()),
        }
    }
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
    pub mod model;
    pub mod state;
    pub mod ui {
        pub mod settings;
    }
    pub mod validation;
}
