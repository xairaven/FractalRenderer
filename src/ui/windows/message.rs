use crate::context::Context;
use crate::ui::windows::{SubWindowProvider, Window};
use egui::WidgetText;

pub struct MessageWindow {
    name: String,
    message: WidgetText,

    width: f32,
    height: f32,

    collapsible: bool,

    is_open: bool,
}

impl Default for MessageWindow {
    fn default() -> Self {
        Self {
            name: "Window".to_string(),
            message: WidgetText::default(),

            is_open: true,

            collapsible: true,

            width: 100.0,
            height: 100.0,
        }
    }
}

impl Window for MessageWindow {
    fn show(&mut self, ui: &egui::Ui, _ctx: &mut Context) {
        egui::Window::new(&self.name)
            .open(&mut self.is_open)
            .min_width(self.width)
            .min_height(self.height)
            .collapsible(self.collapsible)
            .show(ui.ctx(), |ui| {
                ui.label(self.message.clone());
            });
    }

    fn is_closed(&self) -> bool {
        !self.is_open
    }
}

impl SubWindowProvider for MessageWindow {
    fn sub_window(&mut self) -> Option<Box<dyn Window>> {
        None
    }
}

impl MessageWindow {
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_message(mut self, message: impl Into<WidgetText>) -> Self {
        self.message = message.into();
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    pub fn with_collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }
}
