mod calibration;

use crate::util::file_reader::to_string_vector;

use calibration::Equation;

pub fn run() {
    let input = to_string_vector("inputs/day_7.txt").expect("Something went wrong with Day 7!");

    let equations = get_equations(&input);

    println!("Day 7 Part 1: {:?}", part_1(&equations));
    println!("Day 7 Part 2: {:?}", part_2(&equations));
}

fn part_1(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|equation| equation.has_valid_solutions(/*allow_concat=*/ false))
        .map(|equation| equation.test_value)
        .sum()
}

fn part_2(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|equation| equation.has_valid_solutions(/*allow_concat=*/ true))
        .map(|equation| equation.test_value)
        .sum()
}

fn get_equations(input: &[String]) -> Vec<Equation> {
    input.iter().filter_map(|line| line.parse().ok()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_7.txt").unwrap();

        let equations = get_equations(&input);

        assert_eq!(part_1(&equations), 3_749);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_7.txt").unwrap();

        let equations = get_equations(&input);

        assert_eq!(part_2(&equations), 11_387);
    }
}
