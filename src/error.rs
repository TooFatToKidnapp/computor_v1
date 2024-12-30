use std::fmt;

pub enum ComputorError {
    InputError(String),
    CalculationError(String),
    IoError(std::io::Error),
}

impl fmt::Debug for ComputorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputorError::InputError(msg) => write!(f, "InputError -> {}", msg),
            ComputorError::CalculationError(msg) => write!(f, "CalculationError -> {}", msg),
            ComputorError::IoError(err) => write!(f, "IoError -> {}", err),
        }
    }
}
