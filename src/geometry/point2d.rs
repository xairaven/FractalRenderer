use crate::ui::components::canvas::CanvasParams;
use eframe::emath::Pos2;
use eframe::epaint::{Color32, Shape};

#[derive(Debug, Default, Clone)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,

    // Debug fields:
    pub converted_to_screen: bool,
}

impl PartialEq for Point2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.converted_to_screen == other.converted_to_screen
    }
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            converted_to_screen: false,
        }
    }

    pub fn to_pos2(&self) -> Pos2 {
        Pos2::from([self.x, self.y])
    }

    pub fn from_pos2(pos: Pos2) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            converted_to_screen: false,
        }
    }

    pub fn to_screen(&self, canvas_params: &CanvasParams) -> Self {
        canvas_params.point_cm_to_px(self.clone())
    }

    pub fn to_shape(&self, radius: f32, color: Color32) -> Shape {
        Shape::circle_filled(self.to_pos2(), radius, color)
    }

    pub fn with_converted_checked(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            converted_to_screen: true,
        }
    }
}
