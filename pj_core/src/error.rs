use validator::ValidationErrors;

pub enum DomainError {
    ValidateError(String),
}

impl From<ValidationErrors> for DomainError {
    fn from(validation_errors: ValidationErrors) -> Self {
        Self::ValidateError(validation_errors.to_string())
    }
}

pub trait AppErrorTrait {}
