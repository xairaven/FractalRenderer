use crate::context::Context;
use crate::ui::app::App;
use crate::ui::windows::Window;
use egui::{CentralPanel, SidePanel, TopBottomPanel};

pub fn show_normal(app: &mut App, ui: &mut egui::Ui, _ctx: &egui::Context) {
    SidePanel::right("SETTINGS_PANEL")
        .resizable(false)
        .min_width(app.settings.panel_width)
        .max_width(app.settings.panel_width)
        .show_separator_line(true)
        .show_inside(ui, |ui| {
            app.settings
                .show_panel(&mut app.canvas, &mut app.context, ui);
        });

    CentralPanel::default().show_inside(ui, |ui| {
        app.canvas.show_content(&mut app.context, ui);
    });

    if let Ok(sub_window) = app.context.windows_receiver.try_recv() {
        app.sub_windows.push(sub_window);
    }

    show_opened_sub_windows(ui, &mut app.context, &mut app.sub_windows);
}

pub fn show_phone(app: &mut App, ui: &mut egui::Ui, _ctx: &egui::Context) {
    let mut panel = TopBottomPanel::top("SETTINGS_PANEL");

    panel = if app.settings.is_displayed {
        panel
            .min_height(app.size.height * 0.5)
            .show_separator_line(true)
    } else {
        panel.show_separator_line(false)
    };

    panel.show_inside(ui, |ui| {
        if app.settings.is_displayed {
            ui.vertical_centered_justified(|ui| {
                if ui.button("Hide ⤴").clicked() {
                    app.settings.is_displayed = false;
                }
            });

            app.settings
                .show_panel(&mut app.canvas, &mut app.context, ui);
        } else {
            ui.vertical_centered_justified(|ui| {
                if ui.button("Display Settings ⤵").clicked() {
                    app.settings.is_displayed = true;
                }
            });
        }
    });

    CentralPanel::default().show_inside(ui, |ui| {
        app.canvas.show_content(&mut app.context, ui);
    });

    if let Ok(sub_window) = app.context.windows_receiver.try_recv() {
        app.sub_windows.push(sub_window);
    }

    show_opened_sub_windows(ui, &mut app.context, &mut app.sub_windows);
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
