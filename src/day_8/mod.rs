mod antenna;

use crate::util::file_reader::to_string_vector;

use antenna::Map;

pub fn run() {
    let input = to_string_vector("inputs/day_8.txt").expect("Something went wrong with Day 8!");

    println!("Day 8 Part 1: {:?}", part_1(&input));
    println!("Day 8 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> usize {
    let mut map = Map::from(input);

    map.populate_antinodes(/*use_extended=*/ false);

    map.number_of_antinodes()
}

fn part_2(input: &[String]) -> usize {
    let mut map = Map::from(input);

    map.populate_antinodes(/*use_extended=*/ true);

    map.number_of_antinodes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_8.txt").unwrap();

        assert_eq!(part_1(&input), 14);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_8.txt").unwrap();

        assert_eq!(part_2(&input), 34);
    }
}
