use crate::context::Context;
use crate::geometry::line2d::Line2D;
use crate::geometry::point2d::Point2D;
use crate::ui::styles::colors;
use crate::ui::windows::Window;
use egui::{Frame, Painter, Response, Sense, Shape, Vec2};

pub struct Canvas {
    pub params: CanvasParams,
    pub inner_windows: Vec<Box<dyn Window>>,

    grid: Vec<Line2D>,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            params: Default::default(),

            grid: Vec::with_capacity(3),

            inner_windows: Vec::with_capacity(3),
        }
    }
}

impl Canvas {
    pub fn process(&mut self, ui: &egui::Ui, context: &mut Context, response: &Response) {
        // Check for dragging
        self.params.update_offset_on_drag(ui, response);

        self.grid = context.grid.process(&self.params);
    }

    pub fn draw(&mut self, painter: &Painter) {
        let grid_shapes: Vec<Shape> = self
            .grid
            .iter()
            .map(|line| line.to_screen(&self.params).to_shape())
            .collect();
        painter.extend(grid_shapes);
    }

    pub fn show_content(&mut self, context: &mut Context, ui: &mut egui::Ui) {
        Frame::canvas(ui.style())
            .fill(colors::WHITE)
            .show(ui, |ui| {
                let painter_size = ui.available_size_before_wrap();
                let (response, painter) =
                    ui.allocate_painter(painter_size, Sense::click_and_drag());
                self.params.center = Point2D::from_pos2(response.rect.center());
                self.params.resolution =
                    Resolution::from(response.rect.max.x, response.rect.max.y);

                // Pixels per centimeter updating
                ui.input(|i| {
                    let delta = i.smooth_scroll_delta.y;
                    self.params.px_per_cm = (self.params.px_per_cm + delta * 0.1)
                        .clamp(MIN_PX_PER_CM, MAX_PX_PER_CM);
                });

                self.process(ui, context, &response);
                self.draw(&painter);
            });
    }
}

pub const DEFAULT_PX_PER_CM: f32 = 20.0;
pub const MIN_PX_PER_CM: f32 = 5.0;
pub const MAX_PX_PER_CM: f32 = 100.0;

#[derive(Debug, Clone)]
pub struct CanvasParams {
    pub center: Point2D,
    pub resolution: Resolution,
    pub px_per_cm: f32,
    pub unit_length: f32,

    pub is_dragging_enabled: bool,
    pub offset: (f32, f32),
}

impl Default for CanvasParams {
    fn default() -> Self {
        Self {
            center: Default::default(),
            resolution: Default::default(),
            px_per_cm: DEFAULT_PX_PER_CM,
            unit_length: 1.0,

            is_dragging_enabled: true,
            offset: (0.0, 0.0),
        }
    }
}

impl CanvasParams {
    pub fn value_cm_to_px(&self, value: f32) -> f32 {
        value / self.unit_length * self.px_per_cm
    }

    pub fn point_cm_to_px(&self, point: Point2D) -> Point2D {
        debug_assert!(!point.converted_to_screen);

        let x =
            self.center.x + (point.x / self.unit_length * self.px_per_cm) + self.offset.0;
        let y =
            self.center.y - (point.y / self.unit_length * self.px_per_cm) + self.offset.1;

        Point2D::new(x, y).with_converted_checked()
    }

    pub fn value_px_to_cm(&self, value: f32) -> f32 {
        value / self.px_per_cm * self.unit_length
    }

    pub fn point_px_to_cm(&self, point: Point2D) -> Point2D {
        debug_assert!(point.converted_to_screen);

        let x =
            (point.x * self.unit_length / self.px_per_cm) - self.center.x + self.offset.0;
        let y =
            (point.y * self.unit_length / self.px_per_cm) + self.center.y + self.offset.1;

        Point2D::new(x, y).with_converted_unchecked()
    }

    pub fn vec2_px_to_cm(&self, vec: Vec2) -> Vec2 {
        Vec2::new(self.value_px_to_cm(vec.x), -self.value_px_to_cm(vec.y))
    }

    pub fn vec2_cm_to_px(&self, vec: Vec2) -> Vec2 {
        Vec2::new(self.value_cm_to_px(vec.x), -self.value_cm_to_px(vec.y))
    }

    pub fn update_offset_on_drag(&mut self, ui: &egui::Ui, response: &Response) {
        if self.is_dragging_enabled && response.dragged() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grab);

            let delta = response.drag_delta();
            let dragging_coefficient = 1.0;

            self.offset.0 += delta.x * dragging_coefficient;
            self.offset.1 += delta.y * dragging_coefficient;
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Resolution {
    pub width: f32,
    pub height: f32,
}

impl Resolution {
    pub fn from(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
