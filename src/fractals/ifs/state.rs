use crate::fractals::ifs::validation::ValidationError;
use crate::fractals::ifs::{model, validation};
use crate::geometry::dot::Dot;
use crate::ui::styles::colors;
use egui::Color32;

pub struct IfsState {
    is_initialized: bool,
    is_drawing_requested: bool,

    dots: Vec<Dot>,

    pub systems: Vec<[f32; 7]>,

    pub is_coloring_enabled: bool,
    pub colors: Vec<Color32>,

    pub iterations: u32,
    pub radius_cm: f32,
}

const DEFAULT_SYSTEM: [f32; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0];

impl Default for IfsState {
    fn default() -> Self {
        Self {
            is_initialized: false,
            is_drawing_requested: false,

            dots: vec![],

            systems: vec![DEFAULT_SYSTEM],

            is_coloring_enabled: false,
            colors: vec![colors::BLACK],

            iterations: model::DEFAULT_ITERATIONS,
            radius_cm: model::DEFAULT_RADIUS,
        }
    }
}

impl IfsState {
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    pub fn initialize(&mut self) -> Result<(), ValidationError> {
        validation::systems_exist(&self.systems)?;
        validation::probability_range(&self.systems)?;
        validation::probability_sum(&self.systems)?;

        self.is_initialized = true;

        Ok(())
    }

    pub fn reset_initialization(&mut self) {
        self.is_initialized = false;
    }

    pub fn request_drawing(&mut self) {
        self.is_drawing_requested = true;
    }

    pub fn is_drawing_requested(&self) -> bool {
        self.is_drawing_requested
    }

    pub fn add_empty_system(&mut self) {
        self.reset_initialization();

        self.systems.push(DEFAULT_SYSTEM);
        self.colors.push(colors::BLACK);
    }

    pub fn push_system(&mut self, system: [f32; 7]) {
        self.systems.push(system);
        self.colors.push(colors::BLACK);
    }

    pub fn remove_system(&mut self, index: usize) {
        debug_assert!(self.systems.len() == self.colors.len());

        self.reset_initialization();

        self.systems.remove(index);
        self.colors.remove(index);
    }

    pub fn empty_systems(&mut self) {
        self.systems = vec![];
        self.colors = vec![];
    }
}
