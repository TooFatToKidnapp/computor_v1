extern crate computor;

#[test]
#[should_panic]
fn test_empty_input() {
    computor::solution_builder::SolutionBuilder::default()
        .build_equation_terms("")
        .unwrap()
        .build_coefficients()
        .unwrap()
        .build()
        .solve()
        .unwrap()
        .log();
}

#[test]
#[should_panic(
    expected = "Can't solve equations with polynomial degree grater then 2. max degree found 3"
)]
fn test_third_degree_input() {
    computor::solution_builder::SolutionBuilder::default()
        .build_equation_terms("5 * X^3 + 2 * X^2 + 3 * X + 7 = 0")
        .unwrap()
        .build_coefficients()
        .unwrap()
        .build()
        .solve()
        .unwrap()
        .log();
}

#[test]
fn test_invalid_input() {
    match computor::solution_builder::SolutionBuilder::default()
        .build_equation_terms("5 *X^2 + 2 * X^2 + 3 * X + 7 = 0")
    {
        Err(computor::error::ComputorError::InputError(_)) => (),
        _ => panic!(),
    }

    match computor::solution_builder::SolutionBuilder::default()
        .build_equation_terms("5 * X^-2 + 2 * X^2 + 3 * X + 7 = 0")
    {
        Err(computor::error::ComputorError::InputError(_)) => (),
        _ => panic!(),
    }

    match computor::solution_builder::SolutionBuilder::default()
        .build_equation_terms("5 * X^-2 + 2 * X^2 + 3 * X + 7=0")
    {
        Err(computor::error::ComputorError::InputError(_)) => (),
        _ => panic!(),
    }

    match computor::solution_builder::SolutionBuilder::default()
        .build_equation_terms("5 * X^2 + 2 * X^99999999999999999999999 + 3 * X + 7 = 0")
    {
        Err(computor::error::ComputorError::InputError(_)) => (),
        _ => panic!(),
    }

    match computor::solution_builder::SolutionBuilder::default()
        .build_equation_terms("5 * X^2 + + - + 2 * X^0 + 3 * X + 7 = 0")
    {
        Err(computor::error::ComputorError::InputError(_)) => (),
        _ => panic!(),
    }

    match computor::solution_builder::SolutionBuilder::default()
        .build_equation_terms("5 * X^2 + 2 * X^X + 3 * X + 7 = 0")
    {
        Err(computor::error::ComputorError::InputError(_)) => (),
        _ => panic!(),
    }
}
