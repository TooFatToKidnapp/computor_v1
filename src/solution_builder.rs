use crate::{error::ComputorError, solution::Solution, term::Term};
use std::marker::PhantomData;

pub struct Locked;
pub struct Unlocked;
pub struct Done;

#[derive(Debug)]
pub struct SolutionBuilder<State = Locked> {
    pub term_vec_opt: Option<Vec<Term>>,
    pub coefficients_opt: Option<Vec<f64>>,
    pub max_power_opt: Option<usize>,
    pub state: PhantomData<State>,
}

impl Default for SolutionBuilder {
    fn default() -> Self {
        Self {
            term_vec_opt: None,
            coefficients_opt: None,
            max_power_opt: None,
            state: PhantomData::<Locked>,
        }
    }
}

impl SolutionBuilder<Locked> {
    pub fn build_equation_terms(
        mut self,
        formula: &str,
    ) -> Result<SolutionBuilder<Unlocked>, ComputorError> {
        let equation_elements = formula.split_whitespace().collect::<Vec<_>>();
        if equation_elements.is_empty() {
            return Err(ComputorError::InputError("empty input".to_string()));
        }
        let mut current_polarity = 1.0;
        let mut current_exponent = 0;
        let mut current_value = 1.0;
        let mut is_currently_right_side = false;
        let mut is_operator = false;
        let mut term_vec: Vec<Term> = equation_elements
            .iter()
            .filter_map(|str| {
                let mut term = Term::default();
                match *str {
                    s if s.starts_with("X") => {
                        if s.starts_with("X^") {
                            if let Some(exponent_as_str) = s.strip_prefix("X^") {
                                if let Ok(exponent) = exponent_as_str.parse() {
                                    current_exponent = exponent;
                                    is_operator = false;
                                    return None;
                                } else {
                                    return Some(Err(ComputorError::InputError(format!(
                                        "Unable to parse power value [{}]",
                                        exponent_as_str
                                    ))));
                                }
                            } else {
                                return Some(Err(ComputorError::InputError(format!(
                                    "Invalid format [{}]",
                                    s
                                ))));
                            }
                        } else if s.len() == 1 {
                            current_exponent = 1;
                            is_operator = false;
                            return None;
                        } else {
                            return Some(Err(ComputorError::InputError(format!(
                                "Invalid format [{}]",
                                s
                            ))));
                        }
                    }
                    "+" => {
                        if is_operator == true {
                            return Some(Err(ComputorError::InputError(format!(
                                "Out of place operator [{}]",
                                str
                            ))));
                        }
                        is_operator = true;
                        term.current_value = current_value;
                        term.polarity = current_polarity;
                        term.exponent = current_exponent;
                        current_polarity = 1.0;
                        current_value = 1.0;
                    }
                    "-" => {
                        if is_operator == true {
                            return Some(Err(ComputorError::InputError(format!(
                                "Out of place operator [{}]",
                                str
                            ))));
                        }
                        is_operator = true;
                        term.polarity = current_polarity;
                        term.current_value = current_value;
                        term.exponent = current_exponent;
                        current_value = 1.0;
                        current_polarity = -1.0;
                    }
                    "*" => {
                        if is_operator == true {
                            return Some(Err(ComputorError::InputError(format!(
                                "Out of place operator [{}]",
                                str
                            ))));
                        }
                        is_operator = true;
                        return None;
                    }
                    "=" => {
                        if is_operator == true {
                            return Some(Err(ComputorError::InputError(format!(
                                "Out of place operator [{}]",
                                str
                            ))));
                        }
                        is_operator = true;
                        term.current_value = current_value;
                        term.exponent = current_exponent;
                        term.polarity = current_polarity;
                        current_polarity = 1.0;
                        current_value = 1.0;
                        is_currently_right_side = true;
                    }
                    _ => {
                        if let Ok(val) = str.parse() {
                            current_value = val;
                            current_exponent = 0;
                            is_operator = false;
                            return None;
                        } else {
                            return Some(Err(ComputorError::InputError(format!(
                                "Invalid value [{}]",
                                str
                            ))));
                        }
                    }
                }
                Some(Ok(term))
            })
            .collect::<Result<Vec<_>, _>>()?;
        term_vec.push(Term::new(
            current_polarity,
            current_exponent,
            is_currently_right_side,
            current_value,
        ));
        let max_term_power = term_vec
            .iter()
            .map(|t| t.exponent)
            .max()
            .unwrap_or_default();

        self.max_power_opt = Some(max_term_power);
        self.term_vec_opt = Some(term_vec);
        self.max_power_opt = Some(max_term_power);
        Ok(SolutionBuilder {
            state: PhantomData::<Unlocked>,
            term_vec_opt: self.term_vec_opt,
            max_power_opt: self.max_power_opt,
            coefficients_opt: self.coefficients_opt,
        })
    }
}

impl SolutionBuilder<Unlocked> {
    pub fn build_coefficients(self) -> Result<SolutionBuilder<Done>, ComputorError> {
        let max_term_power = self.max_power_opt.unwrap();
        let mut coefficients: Vec<f64> = vec![0.0; max_term_power + 1];

        let term_vec = self.term_vec_opt.unwrap();

        for elem in term_vec.iter() {
            let index = elem.exponent;
            let value = elem.current_value * elem.polarity;
            if elem.is_right_side {
                coefficients[index] -= value;
            } else {
                coefficients[index] += value;
            }
        }

        Ok(SolutionBuilder {
            state: PhantomData::<Done>,
            term_vec_opt: Some(term_vec),
            max_power_opt: self.max_power_opt,
            coefficients_opt: Some(coefficients),
        })
    }
}

impl SolutionBuilder<Done> {
    pub fn build(self) -> Solution {
        Solution {
            max_power: self.max_power_opt.unwrap(),
            coefficients: self.coefficients_opt.unwrap(),
            solution_logs: vec![],
        }
    }
}
