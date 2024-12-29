mod errors;
mod utils;
mod term;

use std::env;
use errors::ComputorError;
use utils::{get_equation_from_stdin, parse_formula};


fn main() -> Result<(), ComputorError> {
    let input = match env::args().nth(1) {
        None => get_equation_from_stdin()?,
        Some(i) => i.trim().to_string(),
    };
    let equation_terms = parse_formula(&input)?;
    println!("{input}");
    for x in equation_terms {
        println!("{:?}", x);
    }

    Ok(())
}
