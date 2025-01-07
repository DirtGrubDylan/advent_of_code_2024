mod locks;

use crate::util::file_reader::to_string_vector;

use locks::System;

pub fn run() {
    let input = to_string_vector("inputs/day_25.txt").expect("Something went wrong with Day 25!");

    println!("Day 25 Part 1: {:?}", part_1(&input));
}

fn part_1(input: &[String]) -> usize {
    let system = System::from(input);

    system.number_of_keys_that_fit_without_overlap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_25.txt").unwrap();

        assert_eq!(part_1(&input), 3);
    }
}
