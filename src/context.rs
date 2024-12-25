use crate::fractals::FractalType;
use crate::graphics::grid::Grid;

#[derive(Default)]
pub struct Context {
    pub grid: Grid,

    pub fractal_type: FractalType,
}
