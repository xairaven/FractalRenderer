use crate::geometry::point2d::Point2D;
use crate::ui::components::canvas::CanvasParams;
use crate::ui::styles::colors;
use egui::{Color32, Shape};

#[derive(Clone)]
pub struct Dot {
    pub point: Point2D,
    pub color: Color32,
    pub radius: f32,
}

impl Dot {
    pub fn new(point: Point2D, color: Color32, radius: f32) -> Self {
        Self {
            point,
            color,
            radius,
        }
    }

    pub fn to_screen(&self, canvas_params: &CanvasParams) -> Self {
        Self {
            point: self.point.to_screen(canvas_params),
            color: self.color,
            radius: canvas_params.value_cm_to_px(self.radius),
        }
    }

    pub fn to_shape(&self) -> Shape {
        debug_assert!(self.point.converted_to_screen);

        self.point.to_shape(self.radius, self.color)
    }
}

pub struct DotBuilder {
    point: Point2D,
    color: Color32,
    radius: f32,
}

impl Default for DotBuilder {
    fn default() -> Self {
        Self {
            point: Point2D::new(0.0, 0.0),
            color: colors::BLACK,
            radius: 0.025,
        }
    }
}

impl DotBuilder {
    pub fn with_center(mut self, point: Point2D) -> Self {
        self.point = point;
        self
    }

    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    pub fn with_radius_centimeters(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn build(self) -> Dot {
        Dot::new(self.point, self.color, self.radius)
    }
}
