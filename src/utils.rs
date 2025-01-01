use crate::error::ComputorError;
use std::io;

pub fn get_equation_from_stdin() -> Result<String, ComputorError> {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(ComputorError::IoError)?;
    Ok(input.trim().to_string())
}

// pub fn my_sqrt(x: f64) -> f64 {
// 	if x < 0.0 {
// 		panic!("Cannot compute the square root of a negative number");
// 	}

// 	// Initial guess for the square root
// 	let mut guess = x / 2.0;

// 	// The precision you desire
// 	let epsilon = 1e-10;

// 	// Loop until the guess is close enough to the actual square root
// 	while (guess * guess - x).abs() > epsilon {
// 		guess = (guess + x / guess) / 2.0;
// 	}

// 	guess
// }
