use crate::error::ComputorError;
use std::io;

pub fn get_equation_from_stdin() -> Result<String, ComputorError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(ComputorError::IoError)?;
    Ok(input.trim().to_string())
}
