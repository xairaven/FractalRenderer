use crate::context::Context;
use crate::ui::components::canvas;
use crate::ui::components::canvas::Canvas;
use crate::ui::windows::Window;
use egui::{DragValue, Grid, RichText};

pub struct Settings {
    pub panel_width: f32,
    pub inner_windows: Vec<Box<dyn Window>>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            panel_width: 250.0,

            inner_windows: Vec::with_capacity(2),
        }
    }
}

impl Settings {
    pub fn show_panel(
        &mut self, canvas: &mut Canvas, context: &mut Context, ui: &mut egui::Ui,
    ) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Fractal Settings");
            });

            ui.add_space(10.0);

            ui.separator();

            ui.add_space(10.0);

            ui.collapsing("Canvas Settings", |ui| {
                ui.horizontal(|ui| {
                    ui.label("Pixels per Centimeter:");
                    ui.add(
                        DragValue::new(&mut canvas.params.px_per_cm)
                            .speed(1)
                            .range(canvas::MIN_PX_PER_CM..=canvas::MAX_PX_PER_CM)
                            .suffix(" cm."),
                    );
                });

                ui.add_space(10.0);

                ui.checkbox(
                    &mut canvas.params.is_dragging_enabled,
                    "Enable Drag & Offset",
                );

                ui.add_space(10.0);

                ui.label(RichText::new("Current Offset:").strong());
                ui.label(format!("X: {:.2} px.", -canvas.params.offset.0));
                ui.label(format!("Y: {:.2} px.", canvas.params.offset.1));

                ui.add_space(10.0);

                ui.vertical_centered(|ui| {
                    if ui.button("Reset Settings").clicked() {
                        canvas.params.px_per_cm = canvas::DEFAULT_PX_PER_CM;
                        canvas.params.is_dragging_enabled = false;
                        canvas.params.offset = (0.0, 0.0);
                    }
                });
            });

            ui.add_space(10.0);

            ui.collapsing("Grid Settings", |ui| {
                ui.checkbox(&mut context.grid.is_enabled, "Enable Grid");

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Unit Length:");
                    ui.add(
                        DragValue::new(&mut canvas.params.unit_length)
                            .speed(1)
                            .range(1.0..=f32::MAX)
                            .suffix(" cm."),
                    );
                });

                ui.add_space(10.0);

                ui.label(RichText::new("Colors:").strong());
                Grid::new("GridColors").num_columns(2).show(ui, |ui| {
                    ui.label("Axis X:");
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut context.grid.axis_x_color,
                        egui::color_picker::Alpha::Opaque,
                    );

                    ui.end_row();

                    ui.label("Axis Y:");
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut context.grid.axis_y_color,
                        egui::color_picker::Alpha::Opaque,
                    );

                    ui.end_row();

                    ui.label("Grid:");
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut context.grid.grid_color,
                        egui::color_picker::Alpha::Opaque,
                    );
                });

                ui.add_space(10.0);

                ui.vertical_centered(|ui| {
                    if ui.button("Reset Settings").clicked() {
                        context.grid = Default::default();
                        canvas.params.unit_length = 1.0;
                    }
                });
            });

            ui.add_space(10.0);

            ui.vertical_centered_justified(|ui| {
                if ui.button("Reset to Default Settings").clicked() {
                    self.reset_to_defaults(context, canvas);
                }
            });
        });
    }

    fn reset_to_defaults(&mut self, context: &mut Context, canvas: &mut Canvas) {
        canvas.params = Default::default();
        context.grid = Default::default();
    }
}
