use egui::Color32;
use rand::Rng;
use strum_macros::Display;

pub const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
pub const DARK_RED: Color32 = Color32::from_rgb(198, 55, 57);
pub const GRAY: Color32 = Color32::from_rgb(200, 200, 200);
pub const LIME: Color32 = Color32::from_rgb(50, 205, 50);
pub const RED: Color32 = Color32::from_rgb(255, 0, 0);
pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);

#[derive(Copy, Clone, Display, Default, PartialEq)]
pub enum ColorScheme {
    #[strum(serialize = "Fixed")]
    Fixed(Color32),

    #[strum(serialize = "Random")]
    Random,

    #[strum(serialize = "Standard (Black)")]
    #[default]
    Standard,
}

impl ColorScheme {
    pub fn get_color(&self) -> Color32 {
        match self {
            ColorScheme::Fixed(color) => *color,
            ColorScheme::Random => {
                let mut rng = rand::thread_rng();
                Color32::from_rgb(
                    rng.gen_range(0..=255),
                    rng.gen_range(0..=255),
                    rng.gen_range(0..=255),
                )
            },
            ColorScheme::Standard => BLACK,
        }
    }
}
