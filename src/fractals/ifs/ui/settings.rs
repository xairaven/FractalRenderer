use crate::context::Context;
use crate::fractals::ifs::examples::Example;
use crate::fractals::ifs::serialization;
use crate::fractals::ifs::state::IfsState;
use crate::fractals::ifs::ui::parameters::IfsParametersWindow;
use crate::io;
use crate::io::filter::FileFilter;
use crate::ui::styles::colors;
use crate::ui::windows::message::MessageWindow;
use crossbeam::channel::{unbounded, Receiver, Sender};
use egui::{Button, DragValue, Grid, RichText};
use indoc::indoc;

pub struct IfsSettingsBlock {
    json_sender: Sender<String>,
    json_receiver: Receiver<String>,
}

impl Default for IfsSettingsBlock {
    fn default() -> Self {
        let (sender, receiver) = unbounded::<String>();
        Self {
            json_sender: sender,
            json_receiver: receiver,
        }
    }
}

impl IfsSettingsBlock {
    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
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
                let _ = context
                    .windows_sender
                    .send(Box::new(IfsParametersWindow::default()));
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

        // Deserializing from Json, if needed
        if let Ok(json) = self.json_receiver.try_recv() {
            let result = self.deserialize_state(&mut context.ifs_state, json);
            if let Err(err) = result {
                let _ = context
                    .windows_sender
                    .send(Box::new(MessageWindow::error(&err)));
            }
        }

        ui.collapsing("Load from Example", |ui| {
            ui.vertical_centered_justified(|ui| {
                for example in Example::iter() {
                    if ui.button(example.to_string()).clicked() {
                        let json = match example.contents() {
                            Ok(json) => json,
                            Err(err) => {
                                let message = format!("File Error: {}", err);
                                let _ = context
                                    .windows_sender
                                    .send(Box::new(MessageWindow::error(&message)));
                                return;
                            },
                        };

                        let _ = self.json_sender.send(json);
                    }
                }
            });
        });

        ui.add_space(10.0);

        ui.collapsing("Load from File", |ui| {
            ui.vertical_centered_justified(|ui| {
                if ui.button("Open File...").clicked() {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        let json = match io::ops_native::load_with_file_pick(FileFilter::json()) {
                            Some(Ok(json)) => json,
                            Some(Err(err)) => {
                                let message = format!("File Error: {}", err);
                                let _ = context.windows_sender.send(Box::new(MessageWindow::error(&message)));
                                return;
                            }
                            None => { return },
                        };

                        let _ = self.json_sender.send(json);
                    }

                    #[cfg(target_arch = "wasm32")]
                    {
                        let json_sender = self.json_sender.clone();
                        let windows_sender = context.windows_sender.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            let json = match io::ops_wasm::load_with_file_pick(FileFilter::json()).await {
                                Some(Ok(json)) => json,
                                Some(Err(err)) => {
                                    let message = format!("File Error: {}", err);
                                    let _ = windows_sender.send(Box::new(MessageWindow::error(&message)));
                                    return;
                                }
                                None => { return },
                            };
                            let _ = json_sender.send(json);
                        });
                    }
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
                    let _ = context.windows_sender.send(Box::new(MessageWindow::help(message)));
                }
            });
        });
    }

    fn deserialize_state(
        &mut self, state: &mut IfsState, json: String,
    ) -> Result<(), String> {
        let dto = match serialization::deserialize(json) {
            Ok(value) => value,
            Err(err) => {
                return Err(format!("JSON Error: {}", err));
            },
        };

        if let Err(err) = dto.load(state) {
            return Err(err.to_string());
        };

        Ok(())
    }
}
