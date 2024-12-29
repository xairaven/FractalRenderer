use crate::fractals::ifs::system::EquationSystem;
use crate::geometry::dot::{Dot, DotBuilder};
use eframe::epaint::Color32;
use rand::distributions::{Distribution, WeightedIndex};
use rand::thread_rng;

pub const DEFAULT_ITERATIONS: u32 = 20000;
pub const DEFAULT_RADIUS: f32 = 0.025;

pub struct Model {
    systems: Vec<[f32; 7]>,
    colors: Vec<Color32>,

    iterations: u32,
    radius: f32,
}

impl Model {
    pub fn dots(&self) -> Vec<Dot> {
        debug_assert!(self.systems.len() == self.colors.len());

        let mut equations: Vec<EquationSystem> = Vec::new();
        for (index, parameters) in self.systems.iter().enumerate() {
            equations.push(
                EquationSystem::new(*parameters, self.radius)
                    .with_color(self.colors[index]),
            );
        }

        let mut dots: Vec<Dot> = Vec::new();

        let start_dot = DotBuilder::default().build();
        dots.push(start_dot);

        let probabilities: Vec<f32> = equations
            .iter()
            .map(|equation| equation.probability())
            .collect();
        let mut rng = thread_rng();

        let dist = match WeightedIndex::new(&probabilities) {
            Ok(value) => value,
            Err(err) => {
                log::error!(
                    "{}",
                    format!(
                        "Error occurred while creating weighted index. Additional Info: {}",
                        err
                    )
                );
                std::process::exit(1);
            },
        };

        for current_index in 0..self.iterations {
            let equation = &equations[dist.sample(&mut rng)];
            let new_dot = equation.next_dot(&dots[current_index as usize]);

            dots.push(new_dot);
        }

        dots
    }
}

pub struct ModelBuilder {
    systems: Vec<[f32; 7]>,
    colors: Vec<Color32>,

    iterations: u32,
    radius: f32,
}

impl Default for ModelBuilder {
    fn default() -> Self {
        Self {
            systems: vec![],
            colors: vec![],

            iterations: DEFAULT_ITERATIONS,
            radius: DEFAULT_RADIUS,
        }
    }
}

impl ModelBuilder {
    pub fn with_systems(mut self, systems: Vec<[f32; 7]>) -> Self {
        self.systems = systems;
        self
    }

    pub fn with_colors(mut self, colors: Vec<Color32>) -> Self {
        self.colors = colors;
        self
    }

    pub fn with_iterations(mut self, iterations: u32) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    pub fn build(self) -> Model {
        Model {
            systems: self.systems,
            colors: self.colors,
            iterations: self.iterations,
            radius: self.radius,
        }
    }
}
