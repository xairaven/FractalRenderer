use crate::fractals::ifs::state::IfsState;
use crate::fractals::ifs::ui::parameters::IfsParametersWindow;
use crate::ui::components::canvas::Canvas;
use crate::ui::styles::colors;
use crate::ui::windows::{SubWindowProvider, Window};
use egui::{Button, DragValue, Grid, RichText};

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
    }
}

impl SubWindowProvider for IfsSettingsComponent {
    fn sub_window(&mut self) -> Option<Box<dyn Window>> {
        self.sub_window.take()
    }
}
