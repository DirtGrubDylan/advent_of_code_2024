mod race;

use crate::util::file_reader::to_string_vector;

use race::Race;

pub fn run() {
    let input = to_string_vector("inputs/day_20.txt").expect("Something went wrong with Day 20!");

    println!("Day 20 Part 1: {:?}", part_1(&input, 100));
    println!("Day 20 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String], time_to_save: usize) -> usize {
    let race = Race::from(input);

    race.number_of_cheats_to_save(time_to_save)
}

fn part_2(_input: &[String]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_20.txt").unwrap();

        assert_eq!(part_1(&input, 12), 3);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_20.txt").unwrap();

        assert_eq!(part_2(&input), 666);
    }
}
