use crate::geometry::line2d::Line2D;
use crate::geometry::point2d::Point2D;
use crate::ui::components::canvas::CanvasParams;
use crate::ui::styles::{colors, strokes};
use egui::{Color32, Shape, Stroke};

pub const DEFAULT_UNIT_LENGTH: f32 = 1.0;

pub struct Grid {
    pub is_enabled: bool,

    pub axis_x_color: Color32,
    pub axis_y_color: Color32,
    pub grid_color: Color32,

    unit_x: Point2D,
    unit_y: Point2D,

    axis_x_stroke: Stroke,
    axis_y_stroke: Stroke,
    grid_stroke: Stroke,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            unit_x: Point2D::new(DEFAULT_UNIT_LENGTH, 0.0),
            unit_y: Point2D::new(0.0, DEFAULT_UNIT_LENGTH),

            is_enabled: false,

            axis_x_color: colors::RED,
            axis_y_color: colors::LIME,
            grid_color: colors::GRAY,
            axis_x_stroke: strokes::axis_red(),
            axis_y_stroke: strokes::axis_lime(),
            grid_stroke: strokes::grid_gray(),
        }
    }
}

impl Grid {
    pub fn shapes(&mut self, params: &CanvasParams) -> Vec<Shape> {
        self.sync_stroke_colors();
        if self.is_enabled {
            self.lines(params)
                .iter()
                .map(|line| line.to_screen(params).to_shape())
                .collect()
        } else {
            Vec::with_capacity(0)
        }
    }

    fn lines(&mut self, canvas_params: &CanvasParams) -> Vec<Line2D> {
        self.unit_x = Point2D::new(canvas_params.unit_length, 0.0);
        self.unit_y = Point2D::new(0.0, canvas_params.unit_length);

        let offset = (canvas_params.offset.0, canvas_params.offset.1);
        let resolution = &canvas_params.resolution;
        let canvas_center = &canvas_params.center;

        // Sides of grid: left and right
        let width = (
            canvas_params.value_px_to_cm(resolution.width - canvas_center.x + offset.0),
            canvas_params.value_px_to_cm(resolution.width - canvas_center.x - offset.0),
        );

        // Sides of grid: bottom and top
        let height = (
            canvas_params.value_px_to_cm(resolution.height - canvas_center.y - offset.1),
            canvas_params.value_px_to_cm(resolution.height - canvas_center.y + offset.1),
        );

        let ticks_x = (
            (width.0 - (width.0 % self.unit_x.x)) / self.unit_x.x,
            (width.1 - (width.1 % self.unit_x.x)) / self.unit_x.x,
        );
        let ticks_y = (
            (height.0 - (height.0 % self.unit_y.y)) / self.unit_y.y,
            (height.1 - (height.1 % self.unit_y.y)) / self.unit_y.y,
        );

        let axis_x = Line2D {
            start: Point2D::new(-width.0, self.unit_x.y),
            end: Point2D::new(width.1, self.unit_x.y),
            stroke: self.axis_x_stroke,
        };

        let axis_y = Line2D {
            start: Point2D::new(self.unit_y.x, -height.0),
            end: Point2D::new(self.unit_y.x, height.1),
            stroke: self.axis_y_stroke,
        };

        let mut lines: Vec<Line2D> = vec![];

        // OY Grid
        for i in (-ticks_x.0 as i32)..=(ticks_x.1 as i32) {
            if i == 0 {
                continue;
            }

            let x = self.unit_x.x * (i as f32);

            let start = Point2D::new(x, axis_y.start.y);
            let end = Point2D::new(x, axis_y.end.y);

            lines.push(Line2D::new(start, end, self.grid_stroke));
        }

        // OX Grid
        for i in (-ticks_y.0 as i32)..=(ticks_y.1 as i32) {
            if i == 0 {
                continue;
            }

            let y = self.unit_y.y * (i as f32);

            let start = Point2D::new(axis_x.start.x, y);
            let end = Point2D::new(axis_x.end.x, y);

            lines.push(Line2D::new(start, end, self.grid_stroke));
        }

        // Pushing main axes
        lines.push(axis_x);
        lines.push(axis_y);

        lines
    }

    fn sync_stroke_colors(&mut self) {
        self.axis_x_stroke.color = self.axis_x_color;
        self.axis_y_stroke.color = self.axis_y_color;
        self.grid_stroke.color = self.grid_color;
    }
}
