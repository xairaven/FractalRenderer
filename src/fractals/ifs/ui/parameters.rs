use crate::context::Context;
use crate::fractals::ifs::serialization;
use crate::io;
use crate::io::filter::FileFilter;
use crate::ui::styles::colors;
use crate::ui::windows::message::MessageWindow;
use crate::ui::windows::Window;
use egui::{Button, DragValue, Grid, RichText};

pub struct IfsParametersWindow {
    name: String,
    is_open: bool,
    collapsible: bool,
    resizable: bool,

    width: f32,
    height: f32,
}

impl Default for IfsParametersWindow {
    fn default() -> Self {
        Self {
            name: "IFS Parameters".to_string(),
            is_open: true,
            collapsible: true,
            resizable: true,

            width: 450.0,
            height: 250.0,
        }
    }
}

impl Window for IfsParametersWindow {
    fn show(&mut self, ui: &egui::Ui, context: &mut Context) {
        let mut to_close = false;
        let mut reset_initialization = false;

        egui::Window::new(&self.name)
            .open(&mut self.is_open)
            .min_width(self.width)
            .min_height(self.height)
            .collapsible(self.collapsible)
            .resizable(self.resizable)
            .show(ui.ctx(), |ui| {
                ui.checkbox(
                    &mut context.ifs_state.is_coloring_enabled,
                    "With colors",
                );

                ui.add_space(10.0);

                egui::ScrollArea::vertical()
                    .max_height(self.height - 30.0)
                    .show(ui, |ui| {
                        let mut rule_removed: (bool, usize) = (false, 0);

                        let grid_columns = 8 + if context.ifs_state.is_coloring_enabled {
                            1
                        } else {
                            0
                        };
                        Grid::new("SystemGrid")
                            .num_columns(grid_columns)
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label(RichText::new("A").strong());
                                ui.label(RichText::new("B").strong());
                                ui.label(RichText::new("D").strong());
                                ui.label(RichText::new("E").strong());
                                ui.label(RichText::new("C").strong());
                                ui.label(RichText::new("F").strong());
                                ui.label(RichText::new("P").strong());
                                ui.end_row();

                                for (index_system, system) in
                                    context.ifs_state.systems.iter_mut().enumerate()
                                {
                                    for element in &mut system[0..=5] {
                                        if ui
                                            .add(
                                                DragValue::new(element)
                                                    .speed(0.01)
                                                    .range(-f32::MAX..=f32::MAX),
                                            )
                                            .changed()
                                        {
                                            reset_initialization = true;
                                        };
                                    }

                                    if ui
                                        .add(
                                            DragValue::new(&mut system[6])
                                                .speed(0.01)
                                                .range(0.01..=1.0),
                                        )
                                        .on_hover_text(
                                            "Hint: The sum of probabilities cannot exceed 1.",
                                        )
                                        .changed()
                                    {
                                        reset_initialization = true;
                                    };

                                    if context.ifs_state.is_coloring_enabled {
                                        egui::color_picker::color_edit_button_srgba(
                                            ui,
                                            &mut context.ifs_state.colors[index_system],
                                            egui::color_picker::Alpha::Opaque,
                                        );
                                    } else {
                                        for color in &mut context.ifs_state.colors {
                                            *color = colors::BLACK;
                                        }
                                    }

                                    if ui.button("Remove").clicked() {
                                        rule_removed = (true, index_system);
                                    }

                                    ui.end_row();
                                }
                            });
                        let (is_rule_removed, removed_rule_index) = rule_removed;
                        if is_rule_removed {
                            context.ifs_state.remove_system(removed_rule_index);
                        }
                    });

                ui.add_space(10.0);

                ui.vertical_centered_justified(|ui| {
                    if ui.button("Add System").clicked() {
                        context.ifs_state.add_empty_system();
                    }
                });

                ui.vertical_centered_justified(|ui| {
                    if ui.button("Save to File...").clicked() {
                        let json = match serialization::serialize(&context.ifs_state) {
                            Ok(value) => value,
                            Err(err) => {
                                let message = format!("JSON Error: {}", err);
                                let _ = context.windows_sender.send(Box::new(MessageWindow::error(&message)));
                                return;
                            }
                        };

                        if let Some(Err(err)) = io::operations::save_with_file_pick(json, FileFilter::json()) {
                            let message = format!("File Error: {}", err);
                            let _ = context.windows_sender.send(Box::new(MessageWindow::error(&message)));
                        }
                    }
                });

                ui.add_space(10.0);

                ui.columns(2, |columns| {
                    columns[0].vertical_centered(|ui| {
                        if ui
                            .add_sized([self.width / 2.0 - 15.0, 20.0], Button::new("Ok"))
                            .clicked()
                        {
                            let initialization_result = context.ifs_state.initialize();

                            match initialization_result {
                                Ok(_) => {
                                    to_close = true;
                                },
                                Err(error) => {
                                    let _ = context.windows_sender.send(Box::new(error.window()));
                                },
                            }
                        }
                    });
                    columns[1].vertical_centered(|ui| {
                        if ui
                            .add_sized([self.width / 2.0 - 15.0, 20.0], Button::new("Close"))
                            .clicked()
                        {
                            to_close = true;
                        }
                    });
                });
            });

        if reset_initialization {
            context.ifs_state.reset_initialization();
        }

        if to_close {
            self.close();
        }
    }

    fn is_closed(&self) -> bool {
        !self.is_open
    }
}

impl IfsParametersWindow {
    fn close(&mut self) {
        self.is_open = false;
    }
}
