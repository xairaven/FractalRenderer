use crate::context::Context;
use crate::ui::app::App;
use crate::ui::windows::{SubWindowProvider, Window};
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

    if let Some(sub_window) = app.canvas.sub_window() {
        app.sub_windows.push(sub_window);
    }
    if let Some(sub_window) = app.settings.sub_window() {
        app.sub_windows.push(sub_window);
    }

    gather_sub_windows(&mut app.sub_windows);
    show_opened_sub_windows(ui, &mut app.context, &mut app.sub_windows);
}

fn gather_sub_windows(windows: &mut Vec<Box<dyn Window>>) {
    let mut sub_windows: Vec<Box<dyn Window>> = vec![];

    for window in windows.iter_mut() {
        if let Some(sub_window) = window.sub_window() {
            sub_windows.push(sub_window);
        }
    }

    if !sub_windows.is_empty() {
        gather_sub_windows(&mut sub_windows);
    }

    for sub_window in sub_windows {
        windows.push(sub_window);
    }
}

fn show_opened_sub_windows(
    ui: &egui::Ui, context: &mut Context, windows: &mut Vec<Box<dyn Window>>,
) {
    let mut closed_windows: Vec<usize> = vec![];

    for (index, window) in windows.iter_mut().enumerate() {
        window.show(ui, context);

        if window.is_closed() {
            closed_windows.push(index);
        }
    }

    closed_windows.iter().for_each(|index| {
        windows.remove(*index);
    });
}
