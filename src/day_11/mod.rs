mod stones;

use crate::util::file_reader::to_string_vector;

use stones::Arrangement;

pub fn run() {
    let input = to_string_vector("inputs/day_11.txt").expect("Something went wrong with Day 11!");

    println!("Day 11 Part 1: {:?}", part_1(&input));
    println!("Day 11 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> usize {
    let mut stones: Arrangement = input.first().map(|line| line.parse().unwrap()).unwrap();

    stones.nth(24);

    stones.number_of_stones()
}

fn part_2(input: &[String]) -> usize {
    let mut stones: Arrangement = input.first().map(|line| line.parse().unwrap()).unwrap();

    stones.nth(74);

    stones.number_of_stones()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_11.txt").unwrap();

        assert_eq!(part_1(&input), 55_312);
    }
}
