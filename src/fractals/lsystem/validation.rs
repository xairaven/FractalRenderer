use crate::fractals::lsystem::model::{RULE_DELIMITER, TERMINAL_SYMBOLS};
use crate::ui::windows::message::MessageWindow;
use thiserror::Error;

pub fn axiom_is_not_empty(axiom: &str) -> Result<(), ValidationError> {
    if axiom.is_empty() {
        return Err(ValidationError::AxiomIsEmpty);
    }

    Ok(())
}

pub fn angle_range(angle: &f32) -> Result<(), ValidationError> {
    if !(0.0..=360.0).contains(angle) {
        return Err(ValidationError::BadAngleValue);
    }

    Ok(())
}

pub fn length_range(length: &f32) -> Result<(), ValidationError> {
    if length.is_sign_negative() {
        return Err(ValidationError::BadLengthValue);
    }

    Ok(())
}

pub fn iterations_range(iterations: usize) -> Result<(), ValidationError> {
    if iterations < 1 {
        return Err(ValidationError::BadIterationsValue);
    }

    Ok(())
}

pub fn right_syntax(raw_rule: &str, index: usize) -> Result<(), ValidationError> {
    if raw_rule.len() < 5 || !raw_rule[1..=4].eq(RULE_DELIMITER) {
        return Err(ValidationError::WrongRuleSyntax(format!(
            "Rule: {}",
            index + 1
        )));
    }

    Ok(())
}

pub fn rule_constant_is_not_empty(
    letter: &str, index: usize,
) -> Result<(), ValidationError> {
    if letter.trim().is_empty() {
        return Err(ValidationError::RuleConstantIsEmpty(format!(
            "Rule: {}",
            index + 1
        )));
    }

    Ok(())
}

pub fn is_valid_char(letter: &str, index: usize) -> Result<char, ValidationError> {
    letter
        .chars()
        .next()
        .ok_or(ValidationError::RuleConstantIsNotValidChar(format!(
            "Rule: {}",
            index + 1
        )))
}

pub fn rule_condition_is_not_empty(
    condition: &str, index: usize,
) -> Result<(), ValidationError> {
    if condition.trim().is_empty() {
        return Err(ValidationError::RuleConditionIsEmpty(format!(
            "Rule: {}",
            index + 1
        )));
    }

    Ok(())
}

pub fn ensure_condition_symbols_in_alphabet(
    alphabet: &[char], conditions: &[String],
) -> Result<(), ValidationError> {
    for (index, condition) in conditions.iter().enumerate() {
        for symbol in condition.chars() {
            if !alphabet.contains(&symbol) && !TERMINAL_SYMBOLS.contains(&symbol) {
                return Err(ValidationError::NonAlphabetSymbolCondition(format!(
                    "Rule: {}\nSymbol: {}",
                    index + 1,
                    symbol
                )));
            }
        }
    }

    Ok(())
}

pub fn ensure_axiom_symbols_in_alphabet(
    alphabet: &[char], axiom: &str,
) -> Result<(), ValidationError> {
    for symbol in axiom.chars() {
        if !alphabet.contains(&symbol) && !TERMINAL_SYMBOLS.contains(&symbol) {
            return Err(ValidationError::NonAlphabetSymbolAxiom(format!(
                "Symbol: {}",
                symbol
            )));
        }
    }

    Ok(())
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Axiom value is empty.")]
    AxiomIsEmpty,

    #[error(
        "The angle value is either greater than 360 degrees or less than 0 degrees."
    )]
    BadAngleValue,

    #[error("The 'length' value is lower than 0.")]
    BadLengthValue,

    #[error("The 'iterations' value is lower than 1.")]
    BadIterationsValue,

    #[error("Wrong rule syntax. There have to be constant, delimiter, and condition")]
    WrongRuleSyntax(String),

    #[error("Rule constant is a whitespace")]
    RuleConstantIsEmpty(String),

    #[error("Rule constant is not a valid UTF-8 symbol.")]
    RuleConstantIsNotValidChar(String),

    #[error("Rule condition consists of less than 1 symbol.")]
    RuleConditionIsEmpty(String),

    #[error("There's symbol in a rule that is not from an alphabet.")]
    NonAlphabetSymbolCondition(String),

    #[error("There's symbol in the axiom that is not from an alphabet.")]
    NonAlphabetSymbolAxiom(String),
}

impl ValidationError {
    pub fn additional_info(&self) -> Option<String> {
        match self {
            Self::WrongRuleSyntax(value)
            | Self::RuleConstantIsEmpty(value)
            | Self::RuleConstantIsNotValidChar(value)
            | Self::RuleConditionIsEmpty(value)
            | Self::NonAlphabetSymbolCondition(value)
            | Self::NonAlphabetSymbolAxiom(value) => Some(value.clone()),
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
    use crate::fractals::lsystem::state::LSystemState;

    #[test]
    fn valid() {
        let mut state = LSystemState::default();
        state.axiom = "FX".to_string();
        state.rules = vec![
            String::from("X -> X+YF++YF-FX--FXFX-YF+"),
            String::from("Y -> -FX+YFYF++YF+FX--FX-Y"),
        ];

        let result = state.initialize();

        assert!(result.is_ok());
    }

    #[test]
    fn empty_axiom() {
        let mut state = LSystemState::default();
        state.axiom = "".to_string();
        state.rules = vec![
            String::from("X -> X+YF++YF-FX--FXFX-YF+"),
            String::from("Y -> -FX+YFYF++YF+FX--FX-Y"),
        ];

        let result = state.initialize();

        assert!(matches!(result, Err(ValidationError::AxiomIsEmpty)));
    }

    #[test]
    fn wrong_angle_range() {
        let mut state = LSystemState::default();
        state.axiom = "FX".to_string();
        state.angle = -1.0;
        state.initial_angle = 1.0;
        state.rules = vec![
            String::from("X -> X+YF++YF-FX--FXFX-YF+"),
            String::from("Y -> -FX+YFYF++YF+FX--FX-Y"),
        ];

        let result = state.initialize();

        assert!(matches!(result, Err(ValidationError::BadAngleValue)));
    }

    #[test]
    fn wrong_initial_angle_range() {
        let mut state = LSystemState::default();
        state.axiom = "FX".to_string();
        state.angle = 1.0;
        state.initial_angle = -1.0;
        state.rules = vec![
            String::from("X -> X+YF++YF-FX--FXFX-YF+"),
            String::from("Y -> -FX+YFYF++YF+FX--FX-Y"),
        ];

        let result = state.initialize();

        assert!(matches!(result, Err(ValidationError::BadAngleValue)));
    }

    #[test]
    fn wrong_length_range() {
        let mut state = LSystemState::default();
        state.axiom = "FX".to_string();
        state.length = -1.0;
        state.rules = vec![
            String::from("X -> X+YF++YF-FX--FXFX-YF+"),
            String::from("Y -> -FX+YFYF++YF+FX--FX-Y"),
        ];

        let result = state.initialize();

        assert!(matches!(result, Err(ValidationError::BadLengthValue)));
    }

    #[test]
    fn wrong_iterations_range() {
        let mut state = LSystemState::default();
        state.axiom = "FX".to_string();
        state.iterations = 0;
        state.rules = vec![
            String::from("X -> X+YF++YF-FX--FXFX-YF+"),
            String::from("Y -> -FX+YFYF++YF+FX--FX-Y"),
        ];

        let result = state.initialize();

        assert!(matches!(result, Err(ValidationError::BadIterationsValue)));
    }

    #[test]
    fn wrong_rule_syntax() {
        let mut state = LSystemState::default();
        state.axiom = "FX".to_string();
        state.rules = vec![
            String::from("X-> X+YF++YF-FX--FXFX-YF+"),
            String::from("-> -FX+YFYF++YF+FX--FX-Y"),
        ];

        let result = state.initialize();

        assert!(matches!(result, Err(ValidationError::WrongRuleSyntax(_))));
    }

    #[test]
    fn empty_rule_constant() {
        let mut state = LSystemState::default();
        state.axiom = "FX".to_string();
        state.rules = vec![
            String::from("X -> X+YF++YF-FX--FXFX-YF+"),
            String::from("  -> -FX+YFYF++YF+FX--FX-Y"),
        ];

        let result = state.initialize();

        assert!(matches!(
            result,
            Err(ValidationError::RuleConstantIsEmpty(_))
        ));
    }

    #[test]
    fn empty_rule_condition() {
        let mut state = LSystemState::default();
        state.axiom = "FX".to_string();
        state.rules = vec![
            String::from("X -> X+YF++YF-FX--FXFX-YF+"),
            String::from("Y ->  "),
        ];

        let result = state.initialize();

        assert!(matches!(
            result,
            Err(ValidationError::RuleConditionIsEmpty(_))
        ));
    }

    #[test]
    fn non_alphabet_symbol_axiom() {
        let mut state = LSystemState::default();
        state.axiom = "FDX".to_string();
        state.rules = vec![
            String::from("X -> X+YF++YF-FX--FXFX-YF+"),
            String::from("Y -> -FX+YFYF++YF+FX--FX-Y"),
        ];

        let result = state.initialize();

        assert!(matches!(
            result,
            Err(ValidationError::NonAlphabetSymbolAxiom(_))
        ));
    }

    #[test]
    fn non_alphabet_symbol_condition() {
        let mut state = LSystemState::default();
        state.axiom = "FX".to_string();
        state.rules = vec![
            String::from("X -> X+YF++YF-FX--FXFX-YF+"),
            String::from("Y -> -FX+YFYDF++YF+FX--FX-Y"),
        ];

        let result = state.initialize();

        assert!(matches!(
            result,
            Err(ValidationError::NonAlphabetSymbolCondition(_))
        ));
    }
}
