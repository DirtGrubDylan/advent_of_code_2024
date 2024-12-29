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

fn part_2(_input: &[String]) -> usize {
    unimplemented!()
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
    #[should_panic(expected = "not implemented")]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_17.txt").unwrap();

        assert_eq!(part_2(&input), 666);
    }
}
