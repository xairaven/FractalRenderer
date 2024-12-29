use crate::context::Context;
use crate::fractals::ifs::examples::Example;
use crate::fractals::ifs::serialization;
use crate::fractals::ifs::state::IfsState;
use crate::fractals::ifs::ui::parameters::IfsParametersWindow;
use crate::io;
use crate::io::filter::FileFilter;
use crate::ui::components::settings::SettingsBlock;
use crate::ui::styles::colors;
use crate::ui::windows::message::MessageWindow;
use crate::ui::windows::{SubWindowProvider, Window};
use egui::{Button, DragValue, Grid, RichText};
use indoc::indoc;

#[derive(Default)]
pub struct IfsSettingsBlock {
    sub_window: Option<Box<dyn Window>>,
}

impl SettingsBlock for IfsSettingsBlock {
    fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        Grid::new("StatusGrid").num_columns(2).show(ui, |ui| {
            ui.label("Status: ");
            if context.ifs_state.is_initialized() {
                ui.label(RichText::new("Initialized!").color(colors::LIME));
            } else {
                ui.label(RichText::new("Not initialized.").color(colors::RED));
            }
            ui.end_row();
        });

        ui.add_space(10.0);

        Grid::new("SettingsGrid").num_columns(2).show(ui, |ui| {
            ui.label("Iterations: ");
            ui.add(
                DragValue::new(&mut context.ifs_state.iterations)
                    .speed(1)
                    .range(0..=u32::MAX),
            );
            ui.end_row();

            ui.label("Dot Radius: ");
            ui.add(
                DragValue::new(&mut context.ifs_state.radius_cm)
                    .speed(0.01)
                    .range(0.01..=5.0)
                    .suffix(" cm."),
            );
            ui.end_row();
        });

        ui.add_space(10.0);

        ui.vertical_centered_justified(|ui| {
            if ui.button("Parameters").clicked() {
                self.sub_window = Some(Box::new(IfsParametersWindow::default()));
            }
        });

        ui.add_space(10.0);

        ui.vertical_centered_justified(|ui| {
            if ui
                .add_enabled(context.ifs_state.is_initialized(), Button::new("Draw"))
                .clicked()
            {
                context.ifs_state.request_drawing();
            }
        });
        ui.vertical_centered_justified(|ui| {
            if ui.button("Reset Settings").clicked() {
                context.ifs_state = Default::default();
            }
        });

        ui.add_space(10.0);

        ui.collapsing("Load from Example", |ui| {
            ui.vertical_centered_justified(|ui| {
                for example in Example::iter() {
                    if ui.button(example.to_string()).clicked() {
                        let json = match io::operations::load_from_path(example.path()) {
                            Ok(json) => json,
                            Err(err) => {
                                context.ifs_state = Default::default();
                                let message = format!("File Error: {}", err);
                                self.sub_window =
                                    Some(Box::new(MessageWindow::error(&message)));
                                return;
                            },
                        };

                        self.load_state_from_json(&mut context.ifs_state, json);
                    }
                }
            });
        });

        ui.add_space(10.0);

        ui.collapsing("Load from File", |ui| {
            ui.vertical_centered_justified(|ui| {
                if ui.button("Open File...").clicked() {
                    let json = match io::operations::load_with_file_pick(FileFilter::json()) {
                        Some(Ok(json)) => json,
                        Some(Err(err)) => {
                            let message = format!("File Error: {}", err);
                            self.sub_window = Some(Box::new(MessageWindow::error(&message)));
                            return;
                        }
                        None => { return },
                    };

                    self.load_state_from_json(&mut context.ifs_state, json);
                }
                if ui.button("Help").clicked() {
                    let message = indoc! {"
                            File format: JSON, arrays with 7 numbers.
                            Numbers: A, B, D, E, C, F, Probability.

                            Example:
                            {
                                \"systems\": [
                                    [0, 0, 0, 0.16, 0, 0, 0.01],
                                    [0.85, 0.04, -0.04, 0.85, 0, 1.6, 0.85],
                                    [0.2, -0.26, 0.23, 0.22, 0, 1.6, 0.07],
                                    [-0.15, 0.28, 0.26, 0.24, 0, 0.44, 0.07]
                                ]
                            }

                            You can find other examples in the 'assets/fractals/ifs' folder.
                        "};
                    self.sub_window = Some(Box::new(MessageWindow::help(message)));
                }
            });
        });
    }
}

impl SubWindowProvider for IfsSettingsBlock {
    fn sub_window(&mut self) -> Option<Box<dyn Window>> {
        self.sub_window.take()
    }
}

impl IfsSettingsBlock {
    fn load_state_from_json(&mut self, state: &mut IfsState, json: String) {
        let dto = match serialization::deserialize(json) {
            Ok(value) => value,
            Err(err) => {
                let message = format!("JSON Error: {}", err);
                self.sub_window = Some(Box::new(MessageWindow::error(&message)));
                return;
            },
        };

        if let Err(err) = dto.load(state) {
            self.sub_window = Some(Box::new(err.window()));
        };
    }
}
