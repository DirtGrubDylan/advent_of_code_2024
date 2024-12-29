use std::collections::HashMap;
use std::string::ToString;

#[derive(Debug, PartialEq, Copy, Clone)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<&usize> for OpCode {
    fn from(input: &usize) -> Self {
        Self::from(*input)
    }
}

impl From<usize> for OpCode {
    fn from(input: usize) -> Self {
        match input {
            0 => OpCode::Adv,
            1 => OpCode::Bxl,
            2 => OpCode::Bst,
            3 => OpCode::Jnz,
            4 => OpCode::Bxc,
            5 => OpCode::Out,
            6 => OpCode::Bdv,
            7 => OpCode::Cdv,
            _ => panic!("Cannot convert `{input}` to an `OpCode`"),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct OpStackItem {
    opcode: OpCode,
    operand: usize,
}

impl OpStackItem {
    fn new(opcode: OpCode, operand: usize) -> Self {
        OpStackItem { opcode, operand }
    }
}

impl From<(&usize, &usize)> for OpStackItem {
    fn from(input: (&usize, &usize)) -> OpStackItem {
        Self::from((*input.0, *input.1))
    }
}

impl From<(usize, usize)> for OpStackItem {
    fn from(input: (usize, usize)) -> OpStackItem {
        OpStackItem::new(OpCode::from(input.0), input.1)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct OpStackItemExecutionResult {
    operation_result: usize,
    stack_pointer: usize,
}

impl OpStackItemExecutionResult {
    fn new(operation_result: usize, stack_pointer: usize) -> Self {
        OpStackItemExecutionResult {
            operation_result,
            stack_pointer,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Computer {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    stack_pointer: usize,
    stack: Vec<usize>,
    output: Vec<usize>,
}

impl Computer {
    pub fn execute_stack(&mut self) -> String {
        while self.execute_next_item() {}

        Self::to_comma_joined_string(&self.output)
    }

    pub fn lowest_register_a_value_to_produce_program_copy(&self) -> usize {
        let mut output_to_register_a_input = HashMap::new();
        let stack_without_jump = self.stack[0..(self.stack.len() - 2)].to_vec();

        let mut clone = self.clone();

        clone.stack = stack_without_jump;

        println!("Min Inputs to Outputs");
        for register_a_value in 0..64 {
            clone.reset_with_register_a(register_a_value);

            clone.execute_stack();

            if let Some(output) = clone.output.get(0) {
                if !output_to_register_a_input.contains_key(output) {
                    output_to_register_a_input.insert(*output, register_a_value);
                }
            }
        }

        for (output, input) in &output_to_register_a_input {}

        let min_inputs_rev: Vec<usize> = self
            .stack
            .iter()
            .rev()
            .filter_map(|output| output_to_register_a_input.get(output))
            .copied()
            .collect();

        println!("{min_inputs_rev:?}");

        let result = min_inputs_rev.iter().fold(0, |acc, min_input| {
            let res = acc * 8 + min_input;
            println!("{min_input} -> {res}");
            res
        });

        clone.reset_with_register_a(result);
        clone.stack = self.stack.clone();
        clone.execute_stack();

        // 29328

        // A = A / 2
        // O = A % 8

        // O = [0, ]
        // A = 29328 / 2 = 14664
        // O = 0
        // A = 14664 / 2 = 14664
        // O = 0

        println!("Ogn: {}", Self::to_comma_joined_string(&self.stack));
        println!("New: {}", Self::to_comma_joined_string(&clone.output));

        result
    }

    fn reset_with_register_a(&mut self, register_a: usize) {
        self.register_a = register_a;
        self.register_b = 0;
        self.register_c = 0;
        self.stack_pointer = 0;
        self.output = Vec::new();
    }

    fn execute_next_item(&mut self) -> bool {
        if let Some(current_stack_item) = self.get_stack_item(self.stack_pointer) {
            let execution_result = self.execute_item(current_stack_item);

            match current_stack_item.opcode {
                OpCode::Adv => {
                    self.register_a = execution_result.operation_result;
                }
                OpCode::Bxl | OpCode::Bst | OpCode::Bxc | OpCode::Bdv => {
                    self.register_b = execution_result.operation_result;
                }
                OpCode::Out => {
                    self.output.push(execution_result.operation_result);
                }
                OpCode::Cdv => {
                    self.register_c = execution_result.operation_result;
                }
                OpCode::Jnz => {}
            };

            self.stack_pointer = execution_result.stack_pointer;

            true
        } else {
            false
        }
    }

    fn execute_item(&self, item: OpStackItem) -> OpStackItemExecutionResult {
        match item.opcode {
            OpCode::Adv | OpCode::Bdv | OpCode::Cdv => {
                let denominator = 2_usize.pow(self.combo_operand(item.operand).try_into().unwrap());

                OpStackItemExecutionResult::new(
                    self.register_a / denominator,
                    self.stack_pointer + 2,
                )
            }
            OpCode::Bxl => OpStackItemExecutionResult::new(
                self.register_b ^ item.operand,
                self.stack_pointer + 2,
            ),
            OpCode::Bst | OpCode::Out => OpStackItemExecutionResult::new(
                self.combo_operand(item.operand) % 8,
                self.stack_pointer + 2,
            ),
            OpCode::Jnz if self.register_a != 0 => OpStackItemExecutionResult::new(0, item.operand),
            OpCode::Jnz => OpStackItemExecutionResult::new(0, self.stack_pointer + 2),
            OpCode::Bxc => OpStackItemExecutionResult::new(
                self.register_b ^ self.register_c,
                self.stack_pointer + 2,
            ),
        }
    }

    fn get_stack_item(&self, pointer: usize) -> Option<OpStackItem> {
        self.stack
            .get(pointer)
            .zip(self.stack.get(pointer + 1))
            .map(OpStackItem::from)
    }

    fn combo_operand(&self, combo_operand: usize) -> usize {
        match combo_operand {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("{combo_operand} is not a valid operand!"),
        }
    }

    fn parse_register_input(input: &str) -> usize {
        let (_, rhs_info) = input.split_once(": ").unwrap_or_default();

        rhs_info
            .parse()
            .unwrap_or_else(|_| panic!("{rhs_info} is not a digit!"))
    }

    fn parse_stack_input(input: &str) -> Vec<usize> {
        let (_, rhs_info) = input.split_once(": ").unwrap_or_default();

        rhs_info.split(',').filter_map(|s| s.parse().ok()).collect()
    }

    pub fn to_comma_joined_string(input: &[usize]) -> String {
        input
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }
}

impl<const N: usize> From<[&str; N]> for Computer {
    fn from(input: [&str; N]) -> Self {
        let input_strings: Vec<String> = input.iter().map(ToString::to_string).collect();

        Self::from(input_strings.as_slice())
    }
}

impl From<&[String]> for Computer {
    fn from(input: &[String]) -> Self {
        assert_eq!(input.len(), 5, "{input:?} must be have five lines!");

        let register_a = Computer::parse_register_input(&input[0]);
        let register_b = Computer::parse_register_input(&input[1]);
        let register_c = Computer::parse_register_input(&input[2]);

        let stack_pointer = 0;
        let stack = Computer::parse_stack_input(&input[4]);

        let output = Vec::new();

        Computer {
            register_a,
            register_b,
            register_c,
            stack_pointer,
            stack,
            output,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computer_from_str_array() {
        let expected = Computer {
            register_a: 729,
            register_b: 0,
            register_c: 0,
            stack_pointer: 0,
            stack: vec![0, 1, 5, 4, 3, 0],
            output: Vec::new(),
        };

        let result = Computer::from([
            "Register A: 729",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 0,1,5,4,3,0",
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_stack_item() {
        let computer = Computer::from([
            "Register A: 729",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 0,1,5,4,3,0",
        ]);

        let expected_1 = OpStackItem::new(OpCode::Adv, 1);
        let expected_2 = OpStackItem::new(OpCode::Bxc, 3);
        let expected_3 = OpStackItem::new(OpCode::Jnz, 0);

        let result_1 = computer.get_stack_item(0);
        let result_2 = computer.get_stack_item(3);
        let result_3 = computer.get_stack_item(4);
        let result_4 = computer.get_stack_item(5);

        assert_eq!(result_1, Some(expected_1));
        assert_eq!(result_2, Some(expected_2));
        assert_eq!(result_3, Some(expected_3));
        assert!(result_4.is_none());
    }

    #[test]
    fn test_combo_operand() {
        let computer = Computer::from([
            "Register A: 729",
            "Register B: 1012",
            "Register C: 9",
            "",
            "Program: 0,1,5,4,3,0",
        ]);

        let expected_0 = 0;
        let expected_1 = 1;
        let expected_2 = 2;
        let expected_3 = 3;
        let expected_4 = 729;
        let expected_5 = 1012;
        let expected_6 = 9;

        let result_0 = computer.combo_operand(0);
        let result_1 = computer.combo_operand(1);
        let result_2 = computer.combo_operand(2);
        let result_3 = computer.combo_operand(3);
        let result_4 = computer.combo_operand(4);
        let result_5 = computer.combo_operand(5);
        let result_6 = computer.combo_operand(6);

        assert_eq!(result_0, expected_0);
        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
        assert_eq!(result_5, expected_5);
        assert_eq!(result_6, expected_6);
    }

    #[test]
    #[should_panic(expected = "7 is not a valid operand!")]
    fn test_combo_operand_panics_at_7() {
        let computer = Computer::from([
            "Register A: 729",
            "Register B: 1012",
            "Register C: 9",
            "",
            "Program: 0,1,5,4,3,0",
        ]);

        computer.combo_operand(7);
    }

    #[test]
    fn test_execute_item_adv() {
        let mut computer = Computer::from([
            "Register A: 2024",
            "Register B: 0",
            "Register C: 2",
            "",
            "Program: 0,2",
        ]);

        computer.stack_pointer = 10;

        let item = OpStackItem::new(OpCode::Adv, 6);

        let expected = OpStackItemExecutionResult::new(506, 12);

        let result = computer.execute_item(item);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_execute_item_bxl() {
        let computer = Computer::from([
            "Register A: 0",
            "Register B: 29",
            "Register C: 0",
            "",
            "Program: 1,7",
        ]);

        let item = OpStackItem::new(OpCode::Bxl, 7);

        let expected = OpStackItemExecutionResult::new(26, 2);

        let result = computer.execute_item(item);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_execute_item_bst() {
        let computer = Computer::from([
            "Register A: 0",
            "Register B: 0",
            "Register C: 9",
            "",
            "Program: 2,6",
        ]);

        let item = OpStackItem::new(OpCode::Bst, 6);

        let expected = OpStackItemExecutionResult::new(1, 2);

        let result = computer.execute_item(item);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_execute_item_jnz_no_jump() {
        let computer = Computer::from([
            "Register A: 0",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 3,3",
        ]);

        let item = OpStackItem::new(OpCode::Jnz, 0);

        let expected = OpStackItemExecutionResult::new(0, 2);

        let result = computer.execute_item(item);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_execute_item_jnz_jump() {
        let mut computer = Computer::from([
            "Register A: 1",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 3,3,5,0,5,0",
        ]);

        computer.stack_pointer = 4;

        let item = OpStackItem::new(OpCode::Jnz, 3);

        let expected = OpStackItemExecutionResult::new(0, 3);

        let result = computer.execute_item(item);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_execute_item_bxc() {
        let computer = Computer::from([
            "Register A: 0",
            "Register B: 2024",
            "Register C: 43690",
            "",
            "Program: 4,0",
        ]);

        let item = OpStackItem::new(OpCode::Bxc, 0);

        let expected = OpStackItemExecutionResult::new(44_354, 2);

        let result = computer.execute_item(item);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_execute_item_out() {
        let computer = Computer::from([
            "Register A: 1012",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 5,4",
        ]);

        let item = OpStackItem::new(OpCode::Out, 4);

        let expected = OpStackItemExecutionResult::new(4, 2);

        let result = computer.execute_item(item);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_execute_item_bdv() {
        let computer = Computer::from([
            "Register A: 2024",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 6,3",
        ]);

        let item = OpStackItem::new(OpCode::Bdv, 3);

        let expected = OpStackItemExecutionResult::new(253, 2);

        let result = computer.execute_item(item);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_execute_item_cdv() {
        let computer = Computer::from([
            "Register A: 809",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 7,2",
        ]);

        let item = OpStackItem::new(OpCode::Cdv, 2);

        let expected = OpStackItemExecutionResult::new(202, 2);

        let result = computer.execute_item(item);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_execute_stack() {
        let mut computer_1 = Computer::from([
            "Register A: 0",
            "Register B: 0",
            "Register C: 9",
            "",
            "Program: 2,6",
        ]);
        let mut computer_2 = Computer::from([
            "Register A: 10",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 5,0,5,1,5,4",
        ]);
        let mut computer_3 = Computer::from([
            "Register A: 2024",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 0,1,5,4,3,0",
        ]);
        let mut computer_4 = Computer::from([
            "Register A: 0",
            "Register B: 29",
            "Register C: 0",
            "",
            "Program: 1,7",
        ]);
        let mut computer_5 = Computer::from([
            "Register A: 0",
            "Register B: 2024",
            "Register C: 43690",
            "",
            "Program: 4,0",
        ]);

        let expected_1 = String::new();
        let expected_2 = String::from("0,1,2");
        let expected_3 = String::from("4,2,5,6,7,7,7,7,3,1,0");
        let expected_4 = String::new();
        let expected_5 = String::new();

        let result_1 = computer_1.execute_stack();
        let result_2 = computer_2.execute_stack();
        let result_3 = computer_3.execute_stack();
        let result_4 = computer_4.execute_stack();
        let result_5 = computer_5.execute_stack();

        assert_eq!(result_1, expected_1);
        assert_eq!(computer_1.register_b, 1);

        assert_eq!(result_2, expected_2);
        assert_eq!(computer_2.register_a, 10);

        assert_eq!(result_3, expected_3);
        assert_eq!(computer_3.register_a, 0);

        assert_eq!(result_4, expected_4);
        assert_eq!(computer_4.register_b, 26);

        assert_eq!(result_5, expected_5);
        assert_eq!(computer_5.register_b, 44_354);
    }

    #[test]
    fn test_lowest_register_a_value_to_produce_program_copy() {
        let computer = Computer::from([
            "Register A: 2024",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 0,3,5,4,3,0",
        ]);

        // 0,     3,    5,   4,  3, 0
        // 0,    24,   40,  32, 24, 0
        //    14680, 1832, 224, 24, 0

        // 5 -> 0
        // 6 -> 3

        assert_eq!(
            computer.lowest_register_a_value_to_produce_program_copy(),
            117_440
        );
    }
}
