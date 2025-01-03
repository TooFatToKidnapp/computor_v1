use crate::error::ComputorError;
use std::io;
use std::ops::Neg;

#[allow(dead_code)]
pub fn get_equation_from_stdin() -> Result<String, ComputorError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(ComputorError::IoError)?;
    Ok(input.trim().to_string())
}

pub fn my_abs<T>(x: T) -> T
where
    T: PartialOrd + Copy + Default + Neg<Output = T>,
{
    if x < T::default() {
        -x
    } else {
        x
    }
}

pub fn my_sqrt(x: f64) -> Result<f64, ComputorError> {
    if x < 0.0 {
        return Err(ComputorError::CalculationError(format!(
            "Failed compute the square root of a negative number [{}]",
            x
        )));
    }

    let mut guess = x / 2.0;
    let tolerance = 1e-10;
    let max_iterations = 1000;

    for _ in 0..max_iterations {
        let next_guess = (guess + x / guess) / 2.0;
        if my_abs(next_guess - guess) < tolerance {
            return Ok(next_guess);
        }
        guess = next_guess;
    }

    Err(ComputorError::CalculationError(format!(
        "Failed to converge to a solution for the square root of [{}]",
        x
    )))
}
