mod race;

use crate::util::file_reader::to_string_vector;

use race::Race;

pub fn run() {
    let input = to_string_vector("inputs/day_20.txt").expect("Something went wrong with Day 20!");

    println!("Day 20 Part 1: {:?}", part_1(&input, 100));
    println!("Day 20 Part 2: {:?}", part_2(&input, 100));
}

fn part_1(input: &[String], time_to_save_min: usize) -> usize {
    number_of_cheats_to_save_at_least(input, 2, time_to_save_min)
}

fn part_2(input: &[String], time_to_save_min: usize) -> usize {
    number_of_cheats_to_save_at_least(input, 20, time_to_save_min)
}

fn number_of_cheats_to_save_at_least(
    input: &[String],
    cheat_duration: usize,
    time_to_save_min: usize,
) -> usize {
    Race::from(input)
        .cheats_to_save(cheat_duration)
        .iter()
        .filter(|(time_saved, _)| time_to_save_min <= **time_saved)
        .map(|(_, cheats)| cheats.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_20.txt").unwrap();

        assert_eq!(part_1(&input, 1), 44);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_20.txt").unwrap();

        assert_eq!(part_2(&input, 50), 285);
    }
}
