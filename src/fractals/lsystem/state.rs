use crate::fractals::lsystem::model::ModelBuilder;
use crate::fractals::lsystem::validation;
use crate::fractals::lsystem::validation::ValidationError;
use crate::geometry::line2d::Line2D;
use crate::ui::components::canvas::CanvasParams;
use crate::ui::styles::{colors, strokes};
use eframe::epaint::Shape;
use egui::{Color32, Stroke};
use std::collections::HashMap;

pub struct LSystemState {
    is_initialized: bool,
    is_drawing_requested: bool,

    pub angle: f32,
    pub initial_angle: f32,
    pub axiom: String,
    pub rules: Vec<String>,
    pub iterations: usize,
    pub length: f32,

    pub color: Color32,
    stroke: Stroke,

    lines: Vec<Line2D>,

    rules_set: HashMap<char, String>,
}

impl Default for LSystemState {
    fn default() -> Self {
        Self {
            is_initialized: false,
            is_drawing_requested: false,

            angle: 0.0,
            initial_angle: 0.0,
            axiom: String::new(),
            rules: vec![String::new()],
            iterations: 1,
            length: 0.5,

            color: colors::BLACK,
            stroke: strokes::model_black(1.0),

            rules_set: HashMap::new(),

            lines: Vec::new(),
        }
    }
}

impl LSystemState {
    pub fn shapes(&mut self, params: &CanvasParams) -> Vec<Shape> {
        self.sync_stroke();

        if self.is_drawing_requested() {
            self.is_drawing_requested = false;
            self.lines = ModelBuilder::default()
                .with_axiom(self.axiom.clone())
                .with_angle(self.angle)
                .with_initial_angle(self.initial_angle)
                .with_rules(self.rules_set.clone())
                .with_iterations(self.iterations)
                .with_length(self.length)
                .with_stroke(self.stroke)
                .build()
                .lines()
        }

        self.lines
            .clone()
            .into_iter()
            .map(|line| line.to_screen(params).to_shape())
            .collect()
    }

    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    pub fn initialize(&mut self) -> Result<(), ValidationError> {
        validation::axiom_is_not_empty(&self.axiom)?;
        validation::angle_range(&self.angle)?;
        validation::angle_range(&self.initial_angle)?;
        validation::length_range(&self.length)?;
        validation::iterations_range(self.iterations)?;

        let mut alphabet: Vec<char> = Vec::new();
        let mut conditions: Vec<String> = Vec::new();

        for (index, line) in self.rules.iter().enumerate() {
            validation::right_syntax(line, index)?;

            let (letter, condition) = (&line[0..1], &line[5usize..]);

            validation::rule_constant_is_not_empty(letter, index)?;
            let letter = validation::is_valid_char(letter, index)?;
            alphabet.push(letter);

            validation::rule_condition_is_not_empty(condition, index)?;
            conditions.push(condition.to_string());
        }

        validation::ensure_axiom_symbols_in_alphabet(&alphabet, &self.axiom)?;
        validation::ensure_condition_symbols_in_alphabet(&alphabet, &conditions)?;

        let mut rules: HashMap<char, String> = HashMap::new();
        for i in 0..alphabet.len() {
            rules.insert(alphabet[i], conditions[i].to_string());
        }
        self.rules_set = rules;

        self.is_initialized = true;

        Ok(())
    }

    pub fn reset_initialization(&mut self) {
        self.rules_set = HashMap::new();
        self.is_initialized = false;
    }

    pub fn request_drawing(&mut self) {
        self.is_drawing_requested = true;
    }

    fn is_drawing_requested(&self) -> bool {
        self.is_drawing_requested
    }

    pub fn push_empty_rule(&mut self) {
        self.reset_initialization();

        self.rules.push(String::new());
    }

    pub fn remove_rule(&mut self, index: usize) {
        debug_assert!(index < self.rules.len());

        self.reset_initialization();

        self.rules.remove(index);
    }

    pub fn reset_with_empty_rules(&mut self) {
        *self = Default::default();
        self.rules = Vec::with_capacity(3);
    }

    fn sync_stroke(&mut self) {
        self.stroke.color = self.color;
    }
}
