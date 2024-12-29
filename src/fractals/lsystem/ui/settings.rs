use crate::context::Context;
use crate::ui::components::settings::SettingsBlock;
use crate::ui::styles::colors;
use crate::ui::windows::{SubWindowProvider, Window};
use egui::{vec2, Button, DragValue, Grid, RichText, Ui};

#[derive(Default)]
pub struct LSystemSettingsBlock {
    sub_window: Option<Box<dyn Window>>,
}

impl SettingsBlock for LSystemSettingsBlock {
    fn show(&mut self, ui: &mut Ui, context: &mut Context) {
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

            ui.label("Color:");
            if egui::color_picker::color_edit_button_srgba(
                ui,
                &mut context.lsystem_state.color,
                egui::color_picker::Alpha::Opaque,
            )
            .changed()
            {
                context.lsystem_state.reset_initialization();
            };
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
                    self.sub_window = Some(Box::new(err.window()));
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
    }
}

impl LSystemSettingsBlock {}

impl SubWindowProvider for LSystemSettingsBlock {
    fn sub_window(&mut self) -> Option<Box<dyn Window>> {
        self.sub_window.take()
    }
}
