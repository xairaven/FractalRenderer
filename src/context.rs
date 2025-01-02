use crate::fractals::ifs::state::IfsState;
use crate::fractals::lsystem::state::LSystemState;
use crate::fractals::FractalType;
use crate::graphics::grid::Grid;
use crate::ui::windows::Window;
use crossbeam::channel::{unbounded, Receiver, Sender};

pub struct Context {
    pub grid: Grid,

    pub fractal_type: FractalType,
    pub ifs_state: IfsState,
    pub lsystem_state: LSystemState,

    pub windows_sender: Sender<Box<dyn Window>>,
    pub windows_receiver: Receiver<Box<dyn Window>>,
}

impl Default for Context {
    fn default() -> Self {
        let (sender, receiver) = unbounded::<Box<dyn Window>>();

        Self {
            grid: Default::default(),
            fractal_type: Default::default(),
            ifs_state: Default::default(),
            lsystem_state: Default::default(),

            windows_sender: sender,
            windows_receiver: receiver,
        }
    }
}
