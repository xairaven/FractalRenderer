use crate::fractals::ifs::state::IfsState;
use crate::fractals::ifs::validation::ValidationError;
use serde::{Deserialize, Serialize};

pub fn parse(json: String) -> serde_json::Result<IfsDto> {
    serde_json::from_str(&json)
}

#[derive(Serialize, Deserialize)]
pub struct IfsDto {
    systems: Vec<[f32; 7]>,
}

impl IfsDto {
    pub fn load(self, state: &mut IfsState) -> Result<(), ValidationError> {
        *state = Default::default();
        state.empty_systems();

        for system in self.systems {
            state.push_system(system);
        }

        state.initialize()
    }
}
