mod hiking;

use crate::util::file_reader::to_string_vector;

use hiking::Map;

pub fn run() {
    let input = to_string_vector("inputs/day_10.txt").expect("Something went wrong with Day 10!");

    println!("Day 10 Part 1: {:?}", part_1(&input));
    println!("Day 10 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> usize {
    let map = Map::new(input);

    map.number_of_reachable_peaks()
}

fn part_2(input: &[String]) -> usize {
    let map = Map::new(input);

    map.number_of_distinct_valid_paths()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_10.txt").unwrap();

        assert_eq!(part_1(&input), 36);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_10.txt").unwrap();

        assert_eq!(part_2(&input), 81);
    }
}
