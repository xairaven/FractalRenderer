use crate::ui::styles::colors;
use egui::Stroke;

const AXIS_WIDTH_PX: f32 = 1.5;
const GRID_WIDTH_PX: f32 = 0.8;

pub fn axis_lime() -> Stroke {
    Stroke::new(AXIS_WIDTH_PX, colors::LIME)
}

pub fn axis_red() -> Stroke {
    Stroke::new(AXIS_WIDTH_PX, colors::DARK_RED)
}
pub fn grid_gray() -> Stroke {
    Stroke::new(GRID_WIDTH_PX, colors::GRAY)
}
