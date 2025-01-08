use std::fmt;

pub enum ComputorError {
    Input(String),
    Calculation(String),
    Io(std::io::Error),
}

impl fmt::Debug for ComputorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputorError::Input(msg) => write!(f, "Input -> {}", msg),
            ComputorError::Calculation(msg) => write!(f, "Calculation -> {}", msg),
            ComputorError::Io(err) => write!(f, "Io -> {}", err),
        }
    }
}
