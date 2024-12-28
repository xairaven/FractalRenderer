use crate::geometry::dot::{Dot, DotBuilder};
use crate::geometry::point2d::Point2D;
use crate::ui::styles::colors;
use egui::Color32;

pub struct EquationSystem {
    a: f32,
    b: f32,
    d: f32,
    e: f32,
    c: f32,
    f: f32,
    p: f32,

    color: Color32,
    radius: f32,
}

impl EquationSystem {
    pub fn new(coefficients: [f32; 7], radius: f32) -> Self {
        Self {
            a: coefficients[0],
            b: coefficients[1],
            d: coefficients[2],
            e: coefficients[3],
            c: coefficients[4],
            f: coefficients[5],
            p: coefficients[6],

            color: colors::BLACK,
            radius,
        }
    }

    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    pub fn probability(&self) -> f32 {
        self.p
    }

    pub fn next_dot(&self, dot: &Dot) -> Dot {
        let point = &dot.point;

        let x = self.a * point.x + self.b * point.y + self.c;
        let y = self.d * point.x + self.e * point.y + self.f;

        DotBuilder::default()
            .with_center(Point2D::new(x, y))
            .with_color(self.color)
            .with_radius_centimeters(self.radius)
            .build()
    }
}
