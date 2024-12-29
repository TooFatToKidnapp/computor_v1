use std::io;
use crate::errors::ComputorError;
use crate::term::Term;


pub fn get_equation_from_stdin() -> Result<String, ComputorError> {
  let mut input = String::new();
  io::stdin()
      .read_line(&mut input)
      .map_err(ComputorError::IoError)?;
  Ok(input.trim().to_string())
}

pub fn parse_formula(formula: &str) -> Result<Vec<Term>, ComputorError> {
  let equation_elements = formula.split_whitespace().collect::<Vec<_>>();
  if equation_elements.is_empty() {
      return Err(ComputorError::InputError("empty input".to_string()));
  }
  let mut current_polarity = 1.0;
  let mut current_exponent  = 0;
  let mut current_value = 1.0;
  let mut is_currently_right_side = false;
  let mut term_vec: Vec<Term> = equation_elements.iter()
    .filter_map(|str| {
      // println!("src = {}", str);
      let mut term = Term::default();
      match *str {
        s if s.starts_with("X") => {
          if s.starts_with("X^") {
            if let Some(exponent_as_str) = s.strip_prefix("X^") {
              if let Ok (exponent) = exponent_as_str.parse() {
                current_exponent = exponent;
                return None;
              } else {
                return Some(Err(ComputorError::InputError(format!("Unable to parse power value [{}]", exponent_as_str))));
              }
            } else {
              return Some(Err(ComputorError::InputError(format!("Invalid format [{}]", s))));
            }
          }
        },
        "+" => {
          term.current_value = current_value;
          term.polarity = 1.0 * current_polarity;
          term.exponent = current_exponent;
          current_polarity = 1.0;
          current_value = 1.0;
        },
        "-" => {
          term.polarity = 1.0 * current_polarity;
          term.current_value = current_value;
          term.exponent = current_exponent;
          current_value = 1.0;
          current_polarity = -1.0;
        },
        "*" => { return None },
        "=" => {
          term.current_value = current_value;
          term.exponent = current_exponent;
          term.polarity = 1.0 * current_polarity;
          current_polarity = 1.0;
          current_value = 1.0;
          is_currently_right_side = true;
        },
        _ => {
          if let Ok(val) = str.parse() {
            current_value = val;
            current_exponent = 0;
            return None;
          } else {
            return Some(Err(ComputorError::InputError(format!("Invalid value [{}]", str))));
          }
        }
    }
    // println!("term = {:#?}", term);
      Some(Ok(term))
    })
    .collect::<Result<Vec<_>, _>>()?;
    term_vec.push(Term::new(current_polarity, current_exponent, is_currently_right_side, current_value));
    Ok(term_vec)
}
