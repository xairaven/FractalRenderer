#[derive(Debug, Default, Clone)]
pub struct Resolution {
    pub width: f32,
    pub height: f32,
}

impl PartialEq for Resolution {
    fn eq(&self, other: &Self) -> bool {
        self.width.eq(&other.width) && self.height.eq(&other.height)
    }
}

impl Resolution {
    pub fn from(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}
