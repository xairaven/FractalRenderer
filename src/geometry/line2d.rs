use crate::geometry::point2d::Point2D;
use crate::ui::components::canvas::CanvasParams;
use egui::{Shape, Stroke};

#[derive(Debug, Default, Clone)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,

    pub stroke: Stroke,
}

impl Line2D {
    pub fn new(start: Point2D, end: Point2D, stroke: Stroke) -> Self {
        Self { start, end, stroke }
    }

    pub fn to_shape(&self) -> Shape {
        Shape::line(vec![self.start.to_pos2(), self.end.to_pos2()], self.stroke)
    }

    pub fn to_screen(&self, canvas_params: &CanvasParams) -> Self {
        Self {
            start: self.start.to_screen(canvas_params),
            end: self.end.to_screen(canvas_params),
            stroke: self.stroke,
        }
    }
}
