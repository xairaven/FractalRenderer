use crate::context::Context;
use crate::ui::app::App;
use crate::ui::windows::Window;
use egui::{CentralPanel, SidePanel};

pub fn show(app: &mut App, ui: &mut egui::Ui, _ctx: &egui::Context) {
    SidePanel::right("CANVAS_PANEL")
        .resizable(false)
        .default_width(app.settings.panel_width)
        .show_separator_line(true)
        .show_inside(ui, |ui| {
            app.settings
                .show_panel(&mut app.canvas, &mut app.context, ui);
        });

    CentralPanel::default().show_inside(ui, |ui| {
        app.canvas.show_content(&mut app.context, ui);
    });

    show_opened_windows(ui, &mut app.context, &mut app.canvas.inner_windows);
    show_opened_windows(ui, &mut app.context, &mut app.settings.inner_windows);
}

fn show_opened_windows(
    ui: &mut egui::Ui, context: &mut Context, windows: &mut Vec<Box<dyn Window>>,
) {
    let mut closed_windows: Vec<usize> = Vec::with_capacity(2);

    for (index, window) in windows.iter_mut().enumerate() {
        window.show(ui, context);

        if window.is_closed() {
            closed_windows.push(index);
        }
    }

    if !closed_windows.is_empty() {
        for index in closed_windows {
            windows.remove(index);
        }
    }
}
