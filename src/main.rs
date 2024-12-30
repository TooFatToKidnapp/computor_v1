mod error;
mod solution;
mod solution_builder;
mod term;
mod utils;

use error::ComputorError;
use solution_builder::SolutionBuilder;
use std::env;
use utils::{get_equation_from_stdin, parse_formula};

fn main() -> Result<(), ComputorError> {
    if env::args().len() > 2 {
        return Err(ComputorError::InputError(
            "Program can only take up to one cli argument".to_string(),
        ));
    }
    let input = match env::args().nth(1) {
        None => get_equation_from_stdin()?,
        Some(i) => i.trim().to_string(),
    };
    // let equation_terms = parse_formula(&input)?;

    let builder = SolutionBuilder::default()
        .build_equation_terms(&input)?
        .build_coefficients()?
        .build();
    // println!("{input}");
    // for x in equation_terms {
    //     println!("{:?}", x);
    // }

    Ok(())
}
