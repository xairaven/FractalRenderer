use crate::context::Context;
use crate::fractals::ifs::ui::settings::IfsSettingsComponent;
use crate::ui::components::canvas::Canvas;
use crate::ui::windows::{SubWindowProvider, Window};
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
    pub fn ui(
        &self, canvas: &mut Canvas, context: &mut Context, ui: &mut egui::Ui,
    ) -> Option<Box<dyn Window>> {
        match self {
            FractalType::Ifs => {
                let mut component = IfsSettingsComponent::default();
                component.show(&mut context.ifs_state, canvas, ui);
                component.sub_window()
            },
            FractalType::LSystem => None,
        }
    }
}

pub mod ifs {
    pub mod examples;
    pub mod model;
    pub mod state;
    pub mod system;
    pub mod ui {
        pub mod parameters;
        pub mod settings;
    }
    pub mod utilities {
        pub mod json;
    }
    pub mod validation;
}
