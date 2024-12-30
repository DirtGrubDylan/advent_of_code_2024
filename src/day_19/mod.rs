mod towels;

use crate::util::file_reader::to_string_vector;

use towels::Towels;

pub fn run() {
    let input = to_string_vector("inputs/day_19.txt").expect("Something went wrong with Day 19!");

    println!("Day 19 Part 1: {:?}", part_1(&input));
    println!("Day 19 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> usize {
    let (towels, towel_stacks) = towels_and_stacks(input);

    towels
        .number_of_patterns_that_match_all(&towel_stacks)
        .iter()
        .filter(|number_of_patterns| **number_of_patterns != 0)
        .count()
}

fn part_2(input: &[String]) -> usize {
    let (towels, towel_stacks) = towels_and_stacks(input);

    towels
        .number_of_patterns_that_match_all(&towel_stacks)
        .iter()
        .sum()
}

fn towels_and_stacks(input: &[String]) -> (Towels, Vec<String>) {
    (
        input.first().unwrap().parse().unwrap(),
        input.iter().skip(2).map(ToString::to_string).collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_19.txt").unwrap();

        assert_eq!(part_1(&input), 6);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_19.txt").unwrap();

        assert_eq!(part_2(&input), 16);
    }
}
