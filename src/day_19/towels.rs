use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Towels {
    patterns: HashSet<String>,
}

impl Towels {
    pub fn number_of_patterns_that_match_all(&self, desired_towel_stacks: &[String]) -> Vec<usize> {
        let mut result = Vec::new();
        let mut stack_pattern_count = HashMap::from([(String::new(), 1)]);

        for stack in desired_towel_stacks {
            result.push(self.number_of_patterns_that_match(stack, &mut stack_pattern_count));
        }

        result
    }

    fn number_of_patterns_that_match(
        &self,
        desired_towel_stack: &str,
        calculated: &mut HashMap<String, usize>,
    ) -> usize {
        let mut result = calculated.get(desired_towel_stack).copied().unwrap_or(0);

        if !calculated.contains_key(desired_towel_stack) {
            let mut temp_string = String::new();

            for c in desired_towel_stack.chars() {
                temp_string.push(c);

                if self.patterns.contains(&temp_string) {
                    let new_string_to_process = desired_towel_stack.replacen(&temp_string, "", 1);

                    result +=
                        self.number_of_patterns_that_match(&new_string_to_process, calculated);
                }
            }

            if desired_towel_stack.is_empty() {
                result = 1;
            }

            if result != 0 {
                calculated.insert(desired_towel_stack.to_string(), result);
            }
        }

        result
    }
}

#[derive(Debug, PartialEq)]
pub struct ParseError;

impl FromStr for Towels {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Towels {
            patterns: input.split(", ").map(ToString::to_string).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let expected = Towels {
            patterns: HashSet::from([
                String::from("r"),
                String::from("wr"),
                String::from("b"),
                String::from("g"),
                String::from("bwu"),
                String::from("rb"),
                String::from("gb"),
                String::from("br"),
            ]),
        };

        let result = "r, wr, b, g, bwu, rb, gb, br".parse();

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_number_of_patterns_that_match() {
        let towels = Towels {
            patterns: HashSet::from([
                String::from("r"),
                String::from("wr"),
                String::from("b"),
                String::from("g"),
                String::from("bwu"),
                String::from("rb"),
                String::from("gb"),
                String::from("br"),
            ]),
        };

        assert_eq!(
            towels.number_of_patterns_that_match("brwrr", &mut HashMap::new()),
            2
        );
        assert_eq!(
            towels.number_of_patterns_that_match("bgbr", &mut HashMap::new()),
            3
        );
        assert_eq!(
            towels.number_of_patterns_that_match(
                "rrbgbr",
                &mut HashMap::from([(String::from("bgbr"), 3)])
            ),
            6
        );
        assert_eq!(
            towels.number_of_patterns_that_match("bbrgwb", &mut HashMap::new()),
            0
        );
    }

    #[test]
    fn test_number_of_patterns_that_match_all() {
        let towels = Towels {
            patterns: HashSet::from([
                String::from("r"),
                String::from("wr"),
                String::from("b"),
                String::from("g"),
                String::from("bwu"),
                String::from("rb"),
                String::from("gb"),
                String::from("br"),
            ]),
        };

        let stacks = [
            String::from("brwrr"),
            String::from("bggr"),
            String::from("gbbr"),
            String::from("rrbgbr"),
            String::from("ubwu"),
            String::from("bwurrg"),
            String::from("brgr"),
            String::from("bbrgwb"),
        ];

        let expected = vec![2, 1, 4, 6, 0, 1, 2, 0];

        let result = towels.number_of_patterns_that_match_all(&stacks);

        assert_eq!(result, expected);
    }
}
