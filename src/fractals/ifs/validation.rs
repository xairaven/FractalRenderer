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
                "Value: {:.2}",
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
            "Sum is {:.2}",
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

    #[error("The provided list of systems is empty. At least one system is required.")]
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
        let mut message = format!("Validation Error: {}", self);
        if let Some(additional_info) = self.additional_info() {
            message += &format!("\n\nAdditional Info:\n{}", additional_info);
        }
        MessageWindow::error(&message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fractals::ifs::state::IfsState;

    #[test]
    fn valid_probability_range() {
        let mut state = IfsState::default();
        state.systems = vec![[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.5]];

        let result = state.initialize();

        assert!(result.is_ok());
    }

    #[test]
    fn negative_probability() {
        let mut state = IfsState::default();
        state.systems = vec![
            [
                0.307692, -0.531469, -0.461538, -0.293706, 5.401953, 8.655175, 0.40,
            ],
            [
                0.307692, -0.076923, 0.153846, -0.447552, -1.295248, 4.152990, -0.15,
            ],
            [
                0.000000, 0.545455, 0.692308, -0.195804, -4.893637, 7.269794, 0.45,
            ],
        ];

        let result = state.initialize();

        assert!(matches!(result, Err(ValidationError::BadProbability(_))));
    }

    #[test]
    fn probability_greater_than_one() {
        let mut state = IfsState::default();
        state.systems = vec![
            [
                0.307692, -0.531469, -0.461538, -0.293706, 5.401953, 8.655175, 0.40,
            ],
            [
                0.307692, -0.076923, 0.153846, -0.447552, -1.295248, 4.152990, 0.15,
            ],
            [
                0.307692, -0.076923, 0.153846, -0.447552, -1.295248, 4.152990, 1.01,
            ],
            [
                0.000000, 0.545455, 0.692308, -0.195804, -4.893637, 7.269794, 0.45,
            ],
        ];

        let result = state.initialize();

        assert!(matches!(result, Err(ValidationError::BadProbability(_))));
    }

    #[test]
    fn no_systems() {
        let mut state = IfsState::default();
        state.systems = vec![];

        let result = state.initialize();

        assert!(matches!(result, Err(ValidationError::NoSystems)));
    }

    #[test]
    fn sum_greater_than_one() {
        let mut state = IfsState::default();
        state.systems = vec![
            [
                0.787879, -0.424242, 0.242424, 0.859848, 1.758647, 1.408065, 0.895652,
            ],
            [
                -0.121212, 0.257576, 0.151515, 0.053030, -6.721654, 1.377236, 0.25,
            ],
            [
                0.181818, -0.136364, 0.090909, 0.181818, 6.086107, 1.568035, 0.052174,
            ],
        ];

        let result = state.initialize();

        assert!(matches!(result, Err(ValidationError::BadProbabilitySum(_))));
    }
}
