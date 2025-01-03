use crate::fractals::ifs::model::ModelBuilder;
use crate::fractals::ifs::validation::ValidationError;
use crate::fractals::ifs::{model, validation};
use crate::geometry::dot::Dot;
use crate::ui::components::canvas::CanvasParams;
use crate::ui::styles::colors::ColorScheme;
use egui::Shape;

pub struct IfsState {
    is_initialized: bool,
    is_drawing_requested: bool,

    dots: Vec<Dot>,

    pub systems: Vec<[f32; 7]>,

    pub is_coloring_enabled: bool,
    pub color_schemas: Vec<ColorScheme>,

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
            color_schemas: vec![ColorScheme::Standard],

            iterations: model::DEFAULT_ITERATIONS,
            radius_cm: model::DEFAULT_RADIUS,
        }
    }
}

impl IfsState {
    pub fn shapes(&mut self, params: &CanvasParams) -> Vec<Shape> {
        if self.is_drawing_requested() {
            self.is_drawing_requested = false;
            self.dots = ModelBuilder::default()
                .with_systems(self.systems.clone())
                .with_color_schemas(self.color_schemas.clone())
                .with_iterations(self.iterations)
                .with_radius(self.radius_cm)
                .build()
                .dots();
        }

        self.dots
            .clone()
            .into_iter()
            .map(|dot| dot.to_screen(params).to_shape())
            .collect()
    }

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
        self.color_schemas.push(ColorScheme::Standard);
    }

    pub fn push_system(&mut self, system: [f32; 7]) {
        self.systems.push(system);
        self.color_schemas.push(ColorScheme::Standard);
    }

    pub fn remove_system(&mut self, index: usize) {
        debug_assert!(self.systems.len() == self.color_schemas.len());

        self.reset_initialization();

        self.systems.remove(index);
        self.color_schemas.remove(index);
    }

    pub fn empty_systems(&mut self) {
        self.systems = vec![];
        self.color_schemas = vec![];
    }
}
