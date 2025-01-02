use crate::context::Context;

pub trait Window {
    fn show(&mut self, ui: &egui::Ui, context: &mut Context);
    fn is_closed(&self) -> bool;
}

pub mod main;
pub mod message;
