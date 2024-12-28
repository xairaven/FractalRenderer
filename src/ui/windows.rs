use crate::context::Context;

pub trait Window: SubWindowProvider {
    fn show(&mut self, ui: &egui::Ui, context: &mut Context);
    fn is_closed(&self) -> bool;
}

pub trait SubWindowProvider {
    fn sub_window(&mut self) -> Option<Box<dyn Window>>;
}

pub mod main;
pub mod message;
