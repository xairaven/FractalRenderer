use crate::context::Context;
use crate::graphics::resolution::Resolution;
use crate::ui::components::canvas::Canvas;
use crate::ui::components::settings::Settings;
use crate::ui::windows;
use crate::ui::windows::Window;
use egui::ThemePreference;

#[derive(Default)]
pub struct App {
    pub size: Resolution,

    pub canvas: Canvas,
    pub context: Context,
    pub settings: Settings,

    pub sub_windows: Vec<Box<dyn Window>>,
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
            let (width, height) = ui.response().rect.size().into();
            self.size = Resolution::from(width, height);

            let proportion = self.size.width / self.size.height;
            if !proportion.is_finite() || proportion > 1.0 {
                windows::main::show_normal(self, ui, ctx);
            } else {
                windows::main::show_phone(self, ui, ctx);
            }
        });
    }
}
