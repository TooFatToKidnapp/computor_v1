use crate::error::ComputorError;
use crate::utils::{my_abs, my_sqrt};
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
            acc.push_str(format!("{} * X^{} ", my_abs(c), p).as_str());
        }
        acc.push_str("= 0");
        acc
    }

    pub fn solve(mut self) -> Result<Self, ComputorError> {
        let mut max_power = self.max_power;
        let mut coefficients = self.coefficients.clone();
        while !coefficients.is_empty() && coefficients[max_power] == 0.0 {
            if max_power == 0 {
                coefficients.clear()
            } else {
                coefficients.pop();
            }
            if max_power != 0 {
                max_power = max_power.saturating_sub(1);
            }
        }
        self.solution_logs
            .push(self.simplify_polynomial(&coefficients));
        self.solution_logs
            .push(format!("Polynomial Degree is {}", max_power));

        if max_power > 2 {
            return Err(ComputorError::Calculation(format!(
                "Can't solve equations with polynomial degree grater then 2. max degree found {}",
                max_power
            )));
        } else if max_power == 2 {
            let a = coefficients[2];
            let b = coefficients[1];
            let c = coefficients[0];

            if a == 0.0 && b == 0.0 && c == 0.0 {
                self.solution_logs
                    .push("Every real number is a solution".to_string());
                return Ok(self);
            }
            self.solution_logs
                .push(format!("a = {} | b = {} | c = {}", a, b, c));
            let delta = (b * b) - ((4.0 * a) * c);
            self.solution_logs.push(format!(
                "delta = b^2 - 4ac = {}^2 - 4 * {} * {} = {}",
                b, a, c, delta
            ));
            match delta.partial_cmp(&0.0) {
                Some(Ordering::Less) => {
                    self.solution_logs.push("Delta is negative, applying the following formula: X = (-b +/- √delta) / 2a".to_string());
                    let x1 = if (-b / (2.0 * a)) != (-b / (2.0 * a)).floor() {
                        format!("({} / {:.6})", -b, (2.0 * a))
                    } else {
                        format!("{:.6}", -b / (2.0 * a))
                    };
                    let x2 = format!("{:.6}", my_sqrt(my_abs(delta))? / (2.0 * a));

                    self.solution_logs
                        .push("No real solutions exist\nComplex solutions are:".to_string());
                    self.solution_logs.push(format!("{} + {}i", x1, x2));
                    self.solution_logs.push(format!("{} - {}i", x1, x2));
                    return Ok(self);
                }
                Some(Ordering::Equal) => {
                    self.solution_logs
                        .push("Delta = 0, applying the following formula: X = -b / 2a".to_string());
                    let solution = if (-b / (2.0 * a)) != (-b / (2.0 * a)).floor() {
                        format!("{} / {:.6}", -b, (2.0 * a))
                    } else {
                        format!("{:.6}", (-b / (2.0 * a)))
                    };
                    self.solution_logs
                        .push(format!("The solution is:\n {}", solution));
                }
                Some(Ordering::Greater) => {
                    self.solution_logs.push(
                        "Delta positive, applying the following formula: X = (-b -/+ √delta) / 2a"
                            .to_string(),
                    );
                    let x1 = if (-b + my_sqrt(delta)?) / (2.0 * a)
                        != ((-b + my_sqrt(delta)?) / (2.0 * a)).floor()
                    {
                        format!("{:.6} / {}", (-b + my_sqrt(delta)?), (2.0 * a))
                    } else {
                        format!("{:.6}", (-b + my_sqrt(delta)?) / (2.0 * a))
                    };
                    let x2 = if (-b - my_sqrt(delta)?) / (2.0 * a)
                        != ((-b - my_sqrt(delta)?) / (2.0 * a)).floor()
                    {
                        format!("{:.6} / {}", (-b - my_sqrt(delta)?), (2.0 * a))
                    } else {
                        format!("{:.6}", (-b - my_sqrt(delta)?) / (2.0 * a))
                    };
                    self.solution_logs
                        .push(format!("The solutions are\n{}\n{}", x2, x1));
                }
                None => {
                    return Err(ComputorError::Calculation(format!(
                        "Error calculating Delta [{}]",
                        delta
                    )))
                }
            }
        } else if max_power == 1 {
            if coefficients[0] == 0.0 && coefficients[1] == 0.0 {
                self.solution_logs
                    .push("Every real number is a solution".to_string());
                return Ok(self);
            } else {
                dbg!(-coefficients[0] / coefficients[1]);
                dbg!((-coefficients[0] / coefficients[1]).floor());
                let solution = if (-coefficients[0] / coefficients[1])
                    != (-coefficients[0] / coefficients[1]).floor()
                {
                    format!("{} / {}", -coefficients[0], coefficients[1])
                } else {
                    format!("{}", (-coefficients[0] / coefficients[1]))
                };
                self.solution_logs
                    .push(format!("The solution is:\n{}", solution));
                return Ok(self);
            }
        } else if coefficients.is_empty() {
            self.solution_logs
                .push("Every real number is a solution".to_string());
            return Ok(self);
        } else {
            return Err(ComputorError::Calculation(
                "Invalid equation. No solution found".to_string(),
            ));
        }

        Ok(self)
    }

    pub fn log(self) {
        for log in self.solution_logs {
            println!("{log}");
        }
    }
}
