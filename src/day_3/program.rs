use regex::Regex;

#[derive(Debug, PartialEq)]
pub struct InstructionParseError {}

impl InstructionParseError {
    fn new() -> Self {
        InstructionParseError {}
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Multiply(u32, u32),
}

impl Instruction {
    pub fn apply(&self) -> u32 {
        match self {
            Instruction::Multiply(x, y) => x * y,
        }
    }

    pub fn get_multiply_instructions(input: &str, use_dont_filter: bool) -> Vec<Self> {
        let mut line = input.to_string();

        if use_dont_filter {
            line = Regex::new(r"don't\(\).*?(do\(\)|$)")
                .expect("Could not build filter regex?")
                .replace_all(input, "don't()do()")
                .to_string();
        }

        Regex::new(r"mul\(\d{1,3},\d{1,3}\)")
            .expect("Could not build regex?")
            .find_iter(&line)
            .map(|instruction_match| instruction_match.as_str())
            .map(Instruction::create_multiply)
            .filter_map(Result::ok)
            .collect()
    }

    fn create_multiply(input: &str) -> Result<Self, InstructionParseError> {
        let mut is_valid = true;
        let mut on_first_value = true;
        let mut first_value = 0;
        let mut second_value = 0;

        for c in input.chars().skip(4) {
            match c {
                _ if !is_valid => break,
                ')' => break,
                ',' => {
                    on_first_value = false;
                }
                _ if c.is_ascii_digit() && on_first_value => {
                    first_value *= 10;
                    first_value += c.to_digit(10).unwrap();
                }
                _ if c.is_ascii_digit() && !on_first_value => {
                    second_value *= 10;
                    second_value += c.to_digit(10).unwrap();
                }
                _ => is_valid = false,
            }
        }

        if (first_value == 0) || (second_value == 0) || !is_valid {
            Err(InstructionParseError::new())
        } else {
            Ok(Instruction::Multiply(first_value, second_value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply() {
        let instructions = [
            Instruction::Multiply(2, 4),
            Instruction::Multiply(5, 5),
            Instruction::Multiply(11, 8),
            Instruction::Multiply(8, 5),
        ];

        let expected = [8, 25, 88, 40];

        let result: Vec<u32> = instructions
            .into_iter()
            .map(|instruction| instruction.apply())
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_multiply_instructions_without_filter() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        let expected = [
            Instruction::Multiply(2, 4),
            Instruction::Multiply(5, 5),
            Instruction::Multiply(11, 8),
            Instruction::Multiply(8, 5),
        ];

        let result = Instruction::get_multiply_instructions(input, /*use_dont_filter=*/ false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_multiply_instructions_with_filter() {
        let input =
            "xmul(2,4)&mul[3,7]!^mul(1,2don't()_mul(5,5)+mul(32,64](mul(11,8)undo())?mul(8,5))don't()mul(1,4)";

        let expected = [Instruction::Multiply(2, 4), Instruction::Multiply(8, 5)];

        let result = Instruction::get_multiply_instructions(input, /*use_dont_filter=*/ true);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_create_multiply_valid() {
        let inputs = [
            "mul(1,1)",
            "mul(12,1)",
            "mul(1,12)",
            "mul(12,12)",
            "mul(123,1)",
            "mul(123,12)",
            "mul(1,123)",
            "mul(12,123)",
            "mul(123,123)",
        ];

        let expected = [
            Ok(Instruction::Multiply(1, 1)),
            Ok(Instruction::Multiply(12, 1)),
            Ok(Instruction::Multiply(1, 12)),
            Ok(Instruction::Multiply(12, 12)),
            Ok(Instruction::Multiply(123, 1)),
            Ok(Instruction::Multiply(123, 12)),
            Ok(Instruction::Multiply(1, 123)),
            Ok(Instruction::Multiply(12, 123)),
            Ok(Instruction::Multiply(123, 123)),
        ];

        let result: Vec<Result<Instruction, InstructionParseError>> = inputs
            .into_iter()
            .map(Instruction::create_multiply)
            .collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_create_multiply_invalid() {
        let inputs = [
            "mul()",
            "mul(1, 1)",
            "mul( 1,1)",
            "mul(1,12]",
            "mul(1x,12)",
            "mul(12312)",
        ];

        assert!(inputs
            .into_iter()
            .all(|input| Instruction::create_multiply(input).is_err()));
    }
}
