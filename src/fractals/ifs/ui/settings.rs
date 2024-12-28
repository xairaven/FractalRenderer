use crate::common;
use crate::fractals::ifs::state::IfsState;
use crate::fractals::ifs::ui::parameters::IfsParametersWindow;
use crate::fractals::ifs::utilities;
use crate::ui::components::canvas::Canvas;
use crate::ui::styles::colors;
use crate::ui::windows::message::MessageWindow;
use crate::ui::windows::{SubWindowProvider, Window};
use egui::{Button, DragValue, Grid, RichText};
use indoc::indoc;

#[derive(Default)]
pub struct IfsSettingsComponent {
    sub_window: Option<Box<dyn Window>>,
}

impl IfsSettingsComponent {
    pub fn show(
        &mut self, state: &mut IfsState, _canvas: &mut Canvas, ui: &mut egui::Ui,
    ) {
        Grid::new("StatusGrid").num_columns(2).show(ui, |ui| {
            ui.label("Status: ");
            if state.is_initialized() {
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
                DragValue::new(&mut state.iterations)
                    .speed(1)
                    .range(0..=u32::MAX),
            );
            ui.end_row();

            ui.label("Dot Radius: ");
            ui.add(
                DragValue::new(&mut state.radius_cm)
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
                .add_enabled(state.is_initialized(), Button::new("Draw"))
                .clicked()
            {
                state.request_drawing();
            }
        });
        ui.vertical_centered_justified(|ui| {
            if ui.button("Reset Settings").clicked() {
                *state = Default::default();
            }
        });

        ui.add_space(10.0);

        ui.collapsing("Load from File", |ui| {
            ui.vertical_centered_justified(|ui| {
                if ui.button("Open File...").clicked() {
                    let json = match common::file_utils::load_with_file_pick() {
                        Some(Ok(json)) => json,
                        Some(Err(err)) => {
                            let message = format!("File Error: {}", err);
                            self.sub_window = Some(Box::new(MessageWindow::error(&message)));
                            return;
                        }
                        None => { return },
                    };

                    let dto = match utilities::json::parse(json) {
                        Ok(value) => value,
                        Err(err) => {
                            let message = format!("JSON Error: {}", err);
                            self.sub_window = Some(Box::new(MessageWindow::error(&message)));
                            return;
                        }
                    };

                    if let Err(err) = dto.load(state) {
                        self.sub_window = Some(Box::new(err.window()));
                    };
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

                            You can find other examples in the 'src/fractals/ifs/examples' folder.
                        "};
                    self.sub_window = Some(Box::new(MessageWindow::help(message)));
                }
            });
        });
    }
}

impl SubWindowProvider for IfsSettingsComponent {
    fn sub_window(&mut self) -> Option<Box<dyn Window>> {
        self.sub_window.take()
    }
}
