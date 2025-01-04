extern crate computor;
use computor::error::ComputorError;
use computor::solution_builder::SolutionBuilder;

#[test]
#[should_panic]
fn test_empty_input() {
    SolutionBuilder::default().build_equation_terms("").unwrap();
}

#[test]
#[should_panic(
    expected = "Can't solve equations with polynomial degree grater then 2. max degree found 3"
)]
fn test_third_degree_input() {
    SolutionBuilder::default()
        .build_equation_terms("2 * X^3 + X^2 + 2 * X + 5 = 0")
        .unwrap()
        .build_coefficients()
        .unwrap()
        .build()
        .solve()
        .unwrap();
}

#[test]
fn test_invalid_input() {
    match SolutionBuilder::default().build_equation_terms("5 *X^2 + 2 * X^2 + 3 * X + 7 = 0") {
        Err(ComputorError::InputError(_)) => (),
        _ => panic!(),
    }

    match SolutionBuilder::default().build_equation_terms("5 * X^2 + 2 * X^2 = 3 * X + 7 = 0") {
        Err(ComputorError::InputError(_)) => (),
        _ => panic!(),
    }

    match SolutionBuilder::default().build_equation_terms("5 * X^-2 + 2 * X^2 + 3 * X + 7 = 0") {
        Err(ComputorError::InputError(_)) => (),
        _ => panic!(),
    }

    match SolutionBuilder::default().build_equation_terms("5 * X^-2 + 2 * X^2 + 3 * X + 7=0") {
        Err(ComputorError::InputError(_)) => (),
        _ => panic!(),
    }

    match SolutionBuilder::default()
        .build_equation_terms("5 * X^2 + 2 * X^99999999999999999999999 + 3 * X + 7 = 0")
    {
        Err(ComputorError::InputError(_)) => (),
        _ => panic!(),
    }

    match SolutionBuilder::default().build_equation_terms("5 * X^2 + + - + 2 * X^0 + 3 * X + 7 = 0")
    {
        Err(ComputorError::InputError(_)) => (),
        _ => panic!(),
    }

    match SolutionBuilder::default().build_equation_terms("5 * X^2 + 2 * X^X + 3 * X + 7 = 0") {
        Err(ComputorError::InputError(_)) => (),
        _ => panic!(),
    }
}

#[test]
fn test_valid_equations() {
    match SolutionBuilder::default()
        .build_equation_terms("X^2 - 5 * X + 6 = 0")
        .unwrap()
        .build_coefficients()
        .unwrap()
        .build()
        .solve()
    {
        Ok(_) => (),
        _ => panic!(),
    }
    match SolutionBuilder::default()
        .build_equation_terms("X^2 - 4 * X + 4 = 0")
        .unwrap()
        .build_coefficients()
        .unwrap()
        .build()
        .solve()
    {
        Ok(_) => (),
        _ => panic!(),
    }

    match SolutionBuilder::default()
        .build_equation_terms("2 * X + 3 = 2 * X - 5")
        .unwrap()
        .build_coefficients()
        .unwrap()
        .build()
        .solve()
    {
        Err(ComputorError::CalculationError(_)) => (),
        _ => panic!(),
    }
    match SolutionBuilder::default()
        .build_equation_terms("5 * X^0 = 5 * X^0")
        .unwrap()
        .build_coefficients()
        .unwrap()
        .build()
        .solve()
    {
        Ok(_) => (),
        _ => panic!(),
    }

    match SolutionBuilder::default()
        .build_equation_terms("6 * X^0 + 11 * X^1 + 5 * X^2 = 1 * X^0 + 0 * X^1")
        .unwrap()
        .build_coefficients()
        .unwrap()
        .build()
        .solve()
    {
        Ok(_) => (),
        _ => panic!(),
    }
    match SolutionBuilder::default()
        .build_equation_terms("5 + 3.111 * X + 3 * X^2 = 1 + 0.1 * X^2")
        .unwrap()
        .build_coefficients()
        .unwrap()
        .build()
        .solve()
    {
        Ok(_) => (),
        _ => panic!(),
    }
    match SolutionBuilder::default()
        .build_equation_terms("5 + 3 * X + 3 * X^2 = 1 + 0 * X^2")
        .unwrap()
        .build_coefficients()
        .unwrap()
        .build()
        .solve()
    {
        Ok(_) => (),
        _ => panic!(),
    }
}
