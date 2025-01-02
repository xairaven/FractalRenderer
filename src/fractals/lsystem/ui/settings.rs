use crate::context::Context;
use crate::fractals::lsystem::examples::Example;
use crate::fractals::lsystem::serialization;
use crate::fractals::lsystem::state::{ColorScheme, LSystemState};
use crate::io;
use crate::io::filter::FileFilter;
use crate::ui::styles::colors;
use crate::ui::windows::message::MessageWindow;
use crate::ui::windows::Window;
use crossbeam::channel::Sender;
use egui::{vec2, Button, Color32, DragValue, Grid, RichText, Ui};
use indoc::indoc;

#[derive(Default)]
pub struct LSystemSettingsBlock;

impl LSystemSettingsBlock {
    pub fn show(&mut self, ui: &mut Ui, context: &mut Context) {
        Grid::new("StatusGrid").num_columns(2).show(ui, |ui| {
            ui.label("Status: ");
            if context.lsystem_state.is_initialized() {
                ui.label(RichText::new("Initialized!").color(colors::LIME));
            } else {
                ui.label(RichText::new("Not initialized.").color(colors::RED));
            }
            ui.end_row();
        });

        ui.add_space(10.0);

        Grid::new("SettingsGrid").num_columns(2).show(ui, |ui| {
            ui.label("Axiom:");
            if ui
                .text_edit_singleline(&mut context.lsystem_state.axiom)
                .changed()
            {
                context.lsystem_state.reset_initialization();
            };
            ui.end_row();

            ui.label("Angle:");
            if ui
                .add(
                    DragValue::new(&mut context.lsystem_state.angle)
                        .speed(1)
                        .range(0..=360)
                        .suffix("°"),
                )
                .changed()
            {
                context.lsystem_state.reset_initialization();
            };
            ui.end_row();

            ui.label("Initial Angle:");
            if ui
                .add(
                    DragValue::new(&mut context.lsystem_state.initial_angle)
                        .speed(1)
                        .range(0..=360)
                        .suffix("°"),
                )
                .changed()
            {
                context.lsystem_state.reset_initialization();
            };
            ui.end_row();

            ui.label("Iterations:");
            if ui
                .add(
                    DragValue::new(&mut context.lsystem_state.iterations)
                        .speed(1)
                        .range(1..=usize::MAX),
                )
                .changed()
            {
                context.lsystem_state.reset_initialization();
            };
            ui.end_row();

            ui.label("Length:");
            if ui
                .add(
                    DragValue::new(&mut context.lsystem_state.length)
                        .speed(0.01)
                        .range(0.01..=f32::MAX)
                        .suffix(" cm."),
                )
                .changed()
            {
                context.lsystem_state.reset_initialization();
            };
            ui.end_row();

            ui.label("Color Scheme:");
            let color = match &context.lsystem_state.color_scheme {
                ColorScheme::Fixed(color) => *color,
                _ => colors::BLACK,
            };
            egui::ComboBox::from_id_salt("ColorScheme")
                .selected_text(format!("{}", context.lsystem_state.color_scheme))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut context.lsystem_state.color_scheme,
                        ColorScheme::Standard,
                        ColorScheme::Standard.to_string(),
                    );
                    ui.selectable_value(
                        &mut context.lsystem_state.color_scheme,
                        ColorScheme::Fixed(color),
                        ColorScheme::Fixed(Color32::default()).to_string(),
                    );
                    ui.selectable_value(
                        &mut context.lsystem_state.color_scheme,
                        ColorScheme::Random,
                        ColorScheme::Random.to_string(),
                    );
                });
            ui.end_row();

            if let ColorScheme::Fixed(color) = &mut context.lsystem_state.color_scheme {
                ui.label("Color:");
                if egui::color_picker::color_edit_button_srgba(
                    ui,
                    color,
                    egui::color_picker::Alpha::Opaque,
                )
                .changed()
                {
                    context.lsystem_state.reset_initialization();
                };
            }
            ui.end_row();
        });

        ui.add_space(10.0);

        ui.label("Rules:");
        ui.add_space(5.0);

        let mut rule_removed: (bool, usize) = (false, 0);
        let mut changed_line = false;
        for (rule_index, rule_line) in context.lsystem_state.rules.iter_mut().enumerate()
        {
            ui.horizontal(|ui| {
                if ui
                    .add_sized(vec2(200.0, 12.5), egui::TextEdit::singleline(rule_line))
                    .on_hover_text("Format:\nSymbol -> Rule\n\nFor Example:\nX -> X+YF+")
                    .changed()
                {
                    changed_line = true;
                };
                if ui.button("Remove").clicked() {
                    rule_removed = (true, rule_index);
                }
            });
        }
        if changed_line {
            context.lsystem_state.reset_initialization();
        }
        let (is_rule_removed, removed_rule_index) = rule_removed;
        if is_rule_removed {
            context.lsystem_state.remove_rule(removed_rule_index);
        }
        ui.vertical_centered_justified(|ui| {
            if ui.button("Add Rule").clicked() {
                context.lsystem_state.push_empty_rule();
            }
        });

        ui.add_space(10.0);

        ui.vertical_centered_justified(|ui| {
            if ui
                .add_enabled(
                    !context.lsystem_state.is_initialized(),
                    Button::new("Initialize"),
                )
                .clicked()
            {
                if let Err(err) = context.lsystem_state.initialize() {
                    let _ = context.windows_sender.send(Box::new(err.window()));
                }
            }
        });

        ui.add_space(2.0);

        ui.vertical_centered_justified(|ui| {
            if ui
                .add_enabled(context.lsystem_state.is_initialized(), Button::new("Draw"))
                .clicked()
            {
                context.lsystem_state.request_drawing();
            }
        });

        ui.add_space(10.0);

        ui.vertical_centered_justified(|ui| {
            if ui.button("Reset Settings").clicked() {
                context.lsystem_state = Default::default();
            }
        });

        ui.add_space(10.0);

        ui.collapsing("Load from Examples", |ui| {
            ui.vertical_centered_justified(|ui| {
                for example in Example::iter() {
                    if ui.button(example.to_string()).clicked() {
                        let json = match io::operations::load_from_path(example.path()) {
                            Ok(json) => json,
                            Err(err) => {
                                context.ifs_state = Default::default();
                                let message = format!("File Error: {}", err);
                                let _ = context
                                    .windows_sender
                                    .send(Box::new(MessageWindow::error(&message)));
                                return;
                            },
                        };

                        self.load_state_from_json(
                            &context.windows_sender,
                            &mut context.lsystem_state,
                            json,
                        );
                    }
                }
            });
        });

        ui.add_space(10.0);

        ui.collapsing("File Settings", |ui| {
            ui.vertical_centered_justified(|ui| {
                if ui.button("Load from File").clicked() {
                    let json = match io::operations::load_with_file_pick(FileFilter::json()) {
                        Some(Ok(json)) => json,
                        Some(Err(err)) => {
                            let message = format!("File Error: {}", err);
                            let _ = context.windows_sender.send(Box::new(MessageWindow::error(&message)));
                            return;
                        }
                        None => { return },
                    };

                    self.load_state_from_json(&context.windows_sender, &mut context.lsystem_state, json);
                }
                if ui.button("Save to File").clicked() {
                    let json = match serialization::serialize(&context.lsystem_state) {
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
                if ui.button("Help").clicked() {
                    let message = indoc! {"
                            File format: JSON.

                            Example:

                            {
                                \"Axiom\": \"FX\",
                                \"Angle\": 90,
                                \"Initial Angle\": 0,
                                \"Iterations\": 5,
                                \"Rules\": [
                                    \"X -> X+YF+\",
                                    \"Y -> -FX-Y\"
                                ]
                            }

                            You can find other examples in the 'assets/fractals/ifs' folder.
                        "};
                    let _ = context.windows_sender.send(Box::new(MessageWindow::help(message)));
                }
            });
        });
    }

    fn load_state_from_json(
        &mut self, sender: &Sender<Box<dyn Window>>, state: &mut LSystemState,
        json: String,
    ) {
        let dto = match serialization::deserialize(json) {
            Ok(value) => value,
            Err(err) => {
                let message = format!("JSON Error: {}", err);
                let _ = sender.send(Box::new(MessageWindow::error(&message)));
                return;
            },
        };

        if let Err(err) = dto.load(state) {
            let _ = sender.send(Box::new(err.window()));
        };
    }
}
