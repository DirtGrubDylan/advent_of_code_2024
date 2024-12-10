mod antenna;

use crate::util::file_reader::to_string_vector;

use antenna::AntennaMap;

pub fn run() {
    let input = to_string_vector("inputs/day_8.txt").expect("Something went wrong with Day 8!");

    println!("Day 8 Part 1: {:?}", part_1(&input));
    println!("Day 8 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> usize {
    let mut map = AntennaMap::from(input);

    map.populate_antinodes();

    map.number_of_antinodes()
}

fn part_2(_input: &[String]) -> usize {
    unimplemented!()
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
    #[should_panic(expected = "not implemented")]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_8.txt").unwrap();

        assert_eq!(part_2(&input), 34);
    }
}
