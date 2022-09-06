use std::fmt::Display;

use validator::ValidationErrors;

#[derive(Debug)]
pub enum DomainError {
    ValidateError(String),
}

impl From<ValidationErrors> for DomainError {
    fn from(validation_errors: ValidationErrors) -> Self {
        Self::ValidateError(validation_errors.to_string())
    }
}

impl Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidateError(error) => write!(f, "{}", error),
        }
    }
}
