use crate::error::ComputorError;
use std::cmp::Ordering;

#[derive(Debug, Default)]
pub struct Solution {
    pub coefficients: Vec<f64>,
    pub max_power: usize,
    pub solution_logs: Vec<String>,
}

impl Solution {
    fn simplify_polynomial(&self, coefficients: &[f64]) -> String {
        if coefficients.is_empty() {
            return "The Polynomial is empty 0 = 0".to_string();
        }
        let mut acc = String::from("Equation reduced form: ");
        for (index, &c) in coefficients.iter().enumerate() {
            let p = index;
            if c >= 0.0 && index > 0 {
                acc.push_str("+ ");
            } else if c < 0.0 {
                acc.push_str("- ");
            }
            acc.push_str(format!("{} * X^{} ", c.abs(), p).as_str());
        }
        acc.push_str("= 0");
        acc
    }

    pub fn solve(mut self) -> Result<Self, ComputorError> {
        let mut max_power = self.max_power;
        let mut coefficients = self.coefficients.clone();

        while !coefficients.is_empty() && coefficients[max_power as usize] == 0.0 {
            if max_power == 0 {
                coefficients.clear()
            } else {
                coefficients.pop();
            }
            if max_power != 0 {
                max_power = max_power - 1;
            }
        }
        self.solution_logs
            .push(self.simplify_polynomial(&coefficients));
        self.solution_logs
            .push(format!("Polynomial Degree is {}", max_power));

        if max_power > 2 {
            return Err(ComputorError::CalculationError(format!(
                "Can't solve equations with polynomial degree grater then 2. max degree found {}",
                max_power
            )));
        } else if max_power == 2 {
            let x = coefficients[2];
            let y = coefficients[1];
            let z = coefficients[0];

            if x == 0.0 && y == 0.0 && z == 0.0 {
                self.solution_logs
                    .push("Every real number is a solution".to_string());
                return Ok(self);
            }

            let delta = (y * y) - ((4.0 * x) * z);

            match delta.partial_cmp(&0.0) {
                Some(Ordering::Less) => {
                    let x1 = -y / (2.0 * x);
                    let x2 = delta.abs().sqrt() / (2.0 * x);
                    self.solution_logs.push(format!("No real solutions exist, Discriminant is negative\n Complex solutions are:"));
                    self.solution_logs.push(format!("{} + {:.6}i", x2, x1));
                    self.solution_logs.push(format!("{} - {:.6}i", x2, x1));
                    return Ok(self);
                }
                Some(Ordering::Equal) => {
                    self.solution_logs.push(format!(
                        "Discriminant is 0.0. the solution is {}",
                        (-y / (2.0 * x))
                    ));
                }
                Some(Ordering::Greater) => {
                    let x1 = (-y + delta.sqrt()) / (2.0 * x);
                    let x2 = (-y - delta.sqrt()) / (2.0 * x);
                    self.solution_logs.push(format!(
                        "Discriminant is positive. the solutions are\n{:.6}\n{:.6}",
                        x2, x1
                    ));
                }
                None => {
                    return Err(ComputorError::CalculationError(format!(
                        "Error calculating Delta [{}]",
                        delta
                    )))
                }
            }
        } else if max_power == 1 {
            if coefficients[0] == 0.0 && coefficients[1] == 0.0 {
                self.solution_logs
                    .push(format!("Every real number is a solution"));
                return Ok(self);
            } else {
                self.solution_logs.push(format!(
                    "The solution is:\n{}",
                    -coefficients[0] / coefficients[1]
                ));
                return Ok(self);
            }
        } else {
            if coefficients.is_empty() {
                self.solution_logs
                    .push(format!("Every real number is a solution"));
                return Ok(self);
            } else {
                return Err(ComputorError::CalculationError(format!(
                    "Invalid equation. No solution found"
                )));
            }
        }

        Ok(self)
    }

    pub fn log(self) -> () {
        for log in self.solution_logs {
            println!("{log}");
        }
    }
}
