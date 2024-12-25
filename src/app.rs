use crate::context::Context;
use crate::ui::components::canvas::Canvas;
use crate::ui::components::settings::Settings;
use crate::ui::windows;
use egui::ThemePreference;

#[derive(Default)]
pub struct App {
    pub canvas: Canvas,
    pub context: Context,
    pub settings: Settings,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, theme: ThemePreference) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx
            .options_mut(|options| options.theme_preference = theme);
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            windows::main::show(self, ui, ctx);
        });
    }
}
