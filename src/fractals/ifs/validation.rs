use crate::ui::windows::message::MessageWindow;
use thiserror::Error;

const EPSILON: f32 = 1e-6;

pub fn systems_exist(systems: &[[f32; 7]]) -> Result<(), ValidationError> {
    match !systems.is_empty() {
        true => Ok(()),
        false => Err(ValidationError::NoSystems),
    }
}

pub fn probability_range(systems: &[[f32; 7]]) -> Result<(), ValidationError> {
    for row in systems {
        let probability = &row[6];

        if !(0.0..=(1.0 + EPSILON)).contains(probability) {
            return Err(ValidationError::BadProbability(format!(
                "Value: {}",
                probability
            )));
        }
    }

    Ok(())
}

pub fn probability_sum(systems: &[[f32; 7]]) -> Result<(), ValidationError> {
    let sum: f32 = systems.iter().map(|row| row[6]).sum();

    if sum > 1.0 + EPSILON {
        return Err(ValidationError::BadProbabilitySum(format!(
            "Sum is {:.6}",
            sum
        )));
    }
    Ok(())
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Probability have to be in range 0..=1")]
    BadProbability(String),

    #[error("Probability sum have to be lower than 1")]
    BadProbabilitySum(String),

    #[error("Probability sum have to be lower than 1")]
    NoSystems,
}

impl ValidationError {
    pub fn additional_info(&self) -> Option<String> {
        match self {
            Self::BadProbability(value) | Self::BadProbabilitySum(value) => {
                Some(value.clone())
            },
            _ => None,
        }
    }

    pub fn window(&self) -> MessageWindow {
        let mut message = format!("Validation error: {}", self);
        if let Some(additional_info) = self.additional_info() {
            message += &format!("\n\nAdditional Info:\n{}", additional_info);
        }

        MessageWindow::default()
            .with_message(message)
            .with_name("Error ❎")
            .with_height(500.0)
            .with_width(300.0)
            .with_collapsible(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_probability_range() {
        let systems = vec![[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5]];
        assert!(probability_range(&systems).is_ok());
    }

    // TODO: MORE TESTS
}
