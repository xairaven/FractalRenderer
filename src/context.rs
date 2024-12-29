use crate::fractals::ifs::state::IfsState;
use crate::fractals::lsystem::state::LSystemState;
use crate::fractals::FractalType;
use crate::graphics::grid::Grid;

#[derive(Default)]
pub struct Context {
    pub grid: Grid,

    pub fractal_type: FractalType,
    pub ifs_state: IfsState,
    pub lsystem_state: LSystemState,
}
