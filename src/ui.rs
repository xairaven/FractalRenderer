use crate::app;
use app::App;
use eframe::NativeOptions;
use egui::ThemePreference;

pub const WINDOW_WIDTH: f32 = 900.0;
pub const WINDOW_HEIGHT: f32 = 550.0;

pub fn start(crate_name: String, theme: ThemePreference) -> eframe::Result {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(format!("Computer Graphics: {crate_name}"))
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_min_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_icon(
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../assets/icon-256.png")[..],
                )
                .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    eframe::run_native(
        &crate_name,
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, theme)))),
    )
}

pub mod components {
    pub mod canvas;
    pub mod settings;
}
pub mod styles {
    pub mod colors;
    pub mod strokes;
}
pub mod windows;
