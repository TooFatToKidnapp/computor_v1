use std::{env, io};
mod errors;
use errors::ComputorError;

fn get_equation_from_stdin() -> Result<String, ComputorError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(ComputorError::IoError)?;
    Ok(input.trim().to_string())
}

fn parse_formula(formula: &str) -> Result<(), ComputorError> {
    let equation_elements = formula.split_whitespace().collect::<Vec<_>>();
    if equation_elements.is_empty() {
        return Err(ComputorError::InputError("empty input".to_string()));
    }

    Ok(())
}

fn main() -> Result<(), ComputorError> {
    let input = match env::args().nth(1) {
        None => get_equation_from_stdin()?,
        Some(i) => i.trim().to_string(),
    };
    parse_formula(&input)?;
    println!("{input}");
    Ok(())
}
