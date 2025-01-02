use crate::fractals::lsystem::state::LSystemState;
use crate::fractals::lsystem::validation::ValidationError;
use serde::{Deserialize, Serialize};

pub fn deserialize(json: String) -> serde_json::Result<LSystemDto> {
    serde_json::from_str(&json)
}

pub fn serialize(state: &LSystemState) -> serde_json::Result<String> {
    let dto = LSystemDto {
        axiom: state.axiom.clone(),
        angle: state.angle,
        initial_angle: state.initial_angle,
        iterations: state.iterations,
        rules: state.rules.clone(),
    };

    serde_json::to_string_pretty(&dto)
}

#[derive(Serialize, Deserialize)]
pub struct LSystemDto {
    #[serde(rename = "Axiom")]
    pub axiom: String,

    #[serde(rename = "Angle")]
    pub angle: f32,

    #[serde(rename = "Initial Angle")]
    pub initial_angle: f32,

    #[serde(rename = "Iterations")]
    pub iterations: usize,

    #[serde(rename = "Rules")]
    pub rules: Vec<String>,
}

impl LSystemDto {
    pub fn load(self, state: &mut LSystemState) -> Result<(), ValidationError> {
        state.reset_with_empty_rules();

        state.axiom = self.axiom;
        state.angle = self.angle;
        state.initial_angle = self.initial_angle;
        state.iterations = self.iterations;
        state.rules = self.rules;

        let result = state.initialize();
        if result.is_err() {
            *state = Default::default();
        }
        result
    }
}
