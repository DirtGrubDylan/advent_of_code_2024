mod computer;

use crate::util::file_reader::to_string_vector;

use computer::Computer;

pub fn run() {
    let input = to_string_vector("inputs/day_17.txt").expect("Something went wrong with Day 17!");

    println!("Day 17 Part 1: {:?}", part_1(&input));
    println!("Day 17 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> String {
    let mut computer = Computer::from(input);

    computer.execute_stack()
}

fn part_2(input: &[String]) -> usize {
    let computer = Computer::from(input);

    computer.lowest_register_a_value_to_produce_program_copy()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_17.txt").unwrap();

        assert_eq!(part_1(&input), String::from("4,6,3,5,6,3,5,2,1,0"));
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_17.txt").unwrap();

        assert_eq!(part_2(&input), 29_327);
    }
}
