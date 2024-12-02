mod locations;

use crate::util::file_reader::to_string_vector;

pub fn run() {
    let input = to_string_vector("inputs/day_1.txt").expect("Something went wrong with Day 1!");

    println!("Day 1 Part 1: {:?}", part_1(&input));
    println!("Day 1 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> u32 {
    locations::minimum_differences(input).into_iter().sum()
}

fn part_2(input: &[String]) -> u32 {
    locations::singularity_scores(input).into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        let input = to_string_vector("test_inputs/day_1.txt").unwrap();

        assert_eq!(part_1(&input), 11);
    }

    #[test]
    fn test_day_2() {
        let input = to_string_vector("test_inputs/day_1.txt").unwrap();

        assert_eq!(part_2(&input), 31);
    }
}
