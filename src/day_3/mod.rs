mod program;

use crate::util::file_reader::to_string_vector;

use program::Instruction;

pub fn run() {
    let input = to_string_vector("inputs/day_3.txt").expect("Something went wrong with Day 3!");

    let line = input.join("").to_string();

    println!("Day 3 Part 1: {:?}", part_1(&line));
    println!("Day 3 Part 2: {:?}", part_2(&line));
}

fn part_1(line: &str) -> u32 {
    Instruction::get_multiply_instructions(line, /*use_dont_filter=*/ false)
        .into_iter()
        .map(|instruction| instruction.apply())
        .sum()
}

fn part_2(line: &str) -> u32 {
    Instruction::get_multiply_instructions(line, /*use_dont_filter=*/ true)
        .into_iter()
        .map(|instruction| instruction.apply())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_3_part_1.txt").unwrap();

        let line = input.join("").to_string();

        assert_eq!(part_1(&line), 161);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_3_part_2.txt").unwrap();

        let line = input.join("").to_string();

        assert_eq!(part_2(&line), 48);
    }
}
