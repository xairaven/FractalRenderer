#[derive(Debug, Default, Clone, Copy)]
pub struct Angle {
    degree: f32,
    radian: f32,
}

impl Angle {
    pub fn from_degree(degree: f32) -> Self {
        let mut local_degree = degree;
        if degree < 0.0 {
            local_degree = 360.0 + degree;
        }
        if degree > 360.0 {
            local_degree -= 360.0;
        }

        Self {
            degree: local_degree,
            radian: local_degree * (std::f64::consts::PI as f32) / 180.0,
        }
    }

    pub fn degree(&self) -> f32 {
        self.degree
    }

    pub fn radian(&self) -> f32 {
        self.radian
    }
}
