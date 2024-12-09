use std::ops::{Add, BitOr, Mul};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Equation {
    pub test_value: usize,
    operator_values: Vec<usize>,
}

impl Equation {
    pub fn has_valid_solutions(&self, allow_concat: bool) -> bool {
        !self.valid_solutions(allow_concat).is_empty()
    }

    pub fn valid_solutions(&self, allow_concat: bool) -> Vec<Solution> {
        let mut result = Vec::new();
        let next_values = self.operator_values.get(1..).unwrap_or(&[]);

        if let Some(initial_solution) = self.operator_values.first().copied().map(Solution::from) {
            result.append(&mut Self::find_solutions(
                &initial_solution,
                next_values,
                allow_concat,
            ));
        }

        result
            .into_iter()
            .filter(|solution| solution.value == self.test_value)
            .collect()
    }

    fn find_solutions(
        solution_so_far: &Solution,
        values_left: &[usize],
        allow_concat: bool,
    ) -> Vec<Solution> {
        let mut result = Vec::new();
        let next_values = values_left.get(1..).unwrap_or(&[]);

        if let Some(value) = values_left.first().copied().map(Solution::from) {
            let add_solution = solution_so_far + &value;

            result.append(&mut Self::find_solutions(
                &add_solution,
                next_values,
                allow_concat,
            ));

            let mul_solution = solution_so_far * &value;

            result.append(&mut Self::find_solutions(
                &mul_solution,
                next_values,
                allow_concat,
            ));

            if allow_concat {
                let concat_solution = solution_so_far | &value;

                result.append(&mut Self::find_solutions(
                    &concat_solution,
                    next_values,
                    allow_concat,
                ));
            }
        } else {
            result.push(solution_so_far.clone());
        }

        result
    }
}

#[derive(Debug, PartialEq)]
pub struct EquationParseError;

impl FromStr for Equation {
    type Err = EquationParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (test_value_str, other_values_str) =
            input.split_once(": ").ok_or(EquationParseError)?;

        let test_value = test_value_str.parse().map_err(|_| EquationParseError)?;

        let operator_values = other_values_str
            .split(' ')
            .map(|value_str| value_str.parse().map_err(|_| EquationParseError))
            .collect::<Result<Vec<usize>, EquationParseError>>()?;

        Ok(Equation {
            test_value,
            operator_values,
        })
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct Solution {
    value: usize,
    source: String,
}

#[derive(Debug, PartialEq)]
pub struct SolutionParseError;

impl FromStr for Solution {
    type Err = SolutionParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let value = input.parse().map_err(|_| SolutionParseError)?;
        let source = String::from(input);

        Ok(Solution { value, source })
    }
}

impl From<usize> for Solution {
    fn from(value: usize) -> Self {
        Solution {
            value,
            source: value.to_string(),
        }
    }
}

impl Add for Solution {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let value = self.value + rhs.value;
        let source = self.source + " + " + &rhs.source;

        Solution { value, source }
    }
}

impl<'rhs> Add<&'rhs Solution> for Solution {
    type Output = Self;

    fn add(self, rhs: &'rhs Self) -> Self {
        self + rhs.clone()
    }
}

impl<'lhs> Add<Solution> for &'lhs Solution {
    type Output = Solution;

    fn add(self, rhs: Solution) -> Solution {
        self.clone() + rhs
    }
}

impl<'lhs, 'rhs> Add<&'rhs Solution> for &'lhs Solution {
    type Output = Solution;

    fn add(self, rhs: &'rhs Solution) -> Solution {
        self.clone() + rhs.clone()
    }
}

impl Mul for Solution {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let value = self.value * rhs.value;
        let source = self.source + " * " + &rhs.source;

        Solution { value, source }
    }
}

impl<'rhs> Mul<&'rhs Solution> for Solution {
    type Output = Self;

    fn mul(self, rhs: &'rhs Self) -> Self {
        self * rhs.clone()
    }
}

impl<'lhs> Mul<Solution> for &'lhs Solution {
    type Output = Solution;

    fn mul(self, rhs: Solution) -> Solution {
        self.clone() * rhs
    }
}

impl<'lhs, 'rhs> Mul<&'rhs Solution> for &'lhs Solution {
    type Output = Solution;

    fn mul(self, rhs: &'rhs Solution) -> Solution {
        self.clone() * rhs.clone()
    }
}

impl BitOr for Solution {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self {
        let mut temp = rhs.value;
        let mut modified_lhs_value = self.value;

        while temp != 0 {
            temp /= 10;
            modified_lhs_value *= 10;
        }

        let value = modified_lhs_value + rhs.value;
        let source = self.source + " || " + &rhs.source;

        Solution { value, source }
    }
}

impl<'rhs> BitOr<&'rhs Solution> for Solution {
    type Output = Self;

    fn bitor(self, rhs: &'rhs Self) -> Self {
        self | rhs.clone()
    }
}

impl<'lhs> BitOr<Solution> for &'lhs Solution {
    type Output = Solution;

    fn bitor(self, rhs: Solution) -> Solution {
        self.clone() | rhs
    }
}

impl<'lhs, 'rhs> BitOr<&'rhs Solution> for &'lhs Solution {
    type Output = Solution;

    fn bitor(self, rhs: &'rhs Solution) -> Solution {
        self.clone() | rhs.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equation_fromstr() {
        let expected_ok = Ok(Equation {
            test_value: 3_267,
            operator_values: vec![81, 400, 27],
        });
        let expected_err: Result<Equation, EquationParseError> = Err(EquationParseError);

        let result_ok = "3267: 81 400 27".parse();
        let result_err = "3267- 81 40 27".parse();

        assert_eq!(result_ok, expected_ok);
        assert_eq!(result_err, expected_err);
    }

    #[test]
    fn test_equation_find_solutions_allow_concat() {
        let expected = vec![
            Solution {
                value: 21,
                source: String::from("15 + 6"),
            },
            Solution {
                value: 90,
                source: String::from("15 * 6"),
            },
            Solution {
                value: 156,
                source: String::from("15 || 6"),
            },
        ];

        let result =
            Equation::find_solutions(&Solution::from(15), &[6], /*allow_concat=*/ true);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_equation_find_solutions_no_concat() {
        let expected = vec![
            Solution {
                value: 148,
                source: String::from("81 + 40 + 27"),
            },
            Solution {
                value: 3_267,
                source: String::from("81 + 40 * 27"),
            },
            Solution {
                value: 3_267,
                source: String::from("81 * 40 + 27"),
            },
            Solution {
                value: 87_480,
                source: String::from("81 * 40 * 27"),
            },
        ];

        let result =
            Equation::find_solutions(&Solution::from(81), &[40, 27], /*allow_concat=*/ false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_equation_valid_solutions_allow_concat_non_empty() {
        let equation: Equation = "7290: 6 8 6 15".parse().unwrap();

        let expected = vec![Solution {
            value: 7_290,
            source: String::from("6 * 8 || 6 * 15"),
        }];

        let result = equation.valid_solutions(/*allow_concat=*/ true);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_equation_valid_solutions_no_concat_non_empty() {
        let equation: Equation = "3267: 81 40 27".parse().unwrap();

        let expected = vec![
            Solution {
                value: 3_267,
                source: String::from("81 + 40 * 27"),
            },
            Solution {
                value: 3_267,
                source: String::from("81 * 40 + 27"),
            },
        ];

        let result = equation.valid_solutions(/*allow_concat=*/ false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_equation_valid_solutions_allow_concat_empty() {
        let equation: Equation = "7290: 6 8 6 15".parse().unwrap();

        assert!(equation.valid_solutions(/*allow_concat=*/ false).is_empty());
    }

    #[test]
    fn test_equation_valid_solutions_no_concat_empty() {
        let equation: Equation = "21037: 9 7 18 13".parse().unwrap();

        assert!(equation.valid_solutions(/*allow_concat=*/ false).is_empty());
    }

    #[test]
    fn test_solution_fromstr() {
        let expected_ok = Ok(Solution::from(3_267));
        let expected_err: Result<Solution, SolutionParseError> = Err(SolutionParseError);

        let result_ok = "3267".parse();
        let result_err = "3267-".parse();

        assert_eq!(result_ok, expected_ok);
        assert_eq!(result_err, expected_err);
    }

    #[test]
    fn test_solution_add() {
        let expected = Solution {
            value: 50,
            source: "25 + 25".to_string(),
        };

        let result = Solution::from(25) + Solution::from(25);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solution_add_by_ref_rhs() {
        let expected = Solution {
            value: 50,
            source: "25 + 25".to_string(),
        };

        let result = Solution::from(25) + &Solution::from(25);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solution_add_by_ref_lhs() {
        let expected = Solution {
            value: 50,
            source: "25 + 25".to_string(),
        };

        let result = &Solution::from(25) + Solution::from(25);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solution_add_by_ref_both() {
        let expected = Solution {
            value: 50,
            source: "25 + 25".to_string(),
        };

        let result = &Solution::from(25) + &Solution::from(25);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solution_mul() {
        let expected = Solution {
            value: 24,
            source: "2 * 3 * 4".to_string(),
        };

        let result = Solution::from(2) * Solution::from(3) * Solution::from(4);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_solution_bitor() {
        let expected = Solution {
            value: 12_345,
            source: "12 || 345".to_string(),
        };

        let result = &Solution::from(12) | &Solution::from(345);

        assert_eq!(result, expected);
    }
}
