mod computer;

use crate::util::file_reader::to_string_vector;

use computer::{Computer, FallingByte};

pub fn run() {
    let input = to_string_vector("inputs/day_18.txt").expect("Something went wrong with Day 18!");

    let computer = Computer::new(71, 71, &input);

    println!("Day 18 Part 1: {:?}", part_1(&computer, 1_024));
    println!("Day 18 Part 2: {}", part_2(&computer));
}

fn part_1(computer: &Computer, number_of_bytes_fallen: usize) -> usize {
    computer.shortest_path_length_after(number_of_bytes_fallen)
}

fn part_2(computer: &Computer) -> FallingByte {
    computer.first_byte_to_prevent_exit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_18.txt").unwrap();

        let computer = Computer::new(7, 7, &input);

        assert_eq!(part_1(&computer, 12), 22);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_18.txt").unwrap();

        let computer = Computer::new(7, 7, &input);

        assert_eq!(part_2(&computer), FallingByte::new(6, 1));
    }
}
