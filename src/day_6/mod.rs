mod security;

use crate::util::file_reader::to_string_vector;

use security::{Guard, PatrolMap};

pub fn run() {
    let input = to_string_vector("inputs/day_6.txt").expect("Something went wrong with Day 6!");

    let (guard, map) = get_guard_and_map(&input);

    println!("Day 6 Part 1: {:?}", part_1(&guard, &map));
    println!("Day 6 Part 2: {:?}", part_2(&guard, &map));
}

fn part_1(guard: &Guard, map: &PatrolMap) -> usize {
    guard.number_of_unique_positions_to_walk(map)
}

fn part_2(guard: &Guard, map: &PatrolMap) -> usize {
    unimplemented!()
}

fn get_guard_and_map(input: &[String]) -> (Guard, PatrolMap) {
    let map = PatrolMap::new(input);

    (map.guard(), map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_6.txt").unwrap();

        let (guard, map) = get_guard_and_map(&input);

        assert_eq!(part_1(&guard, &map), 41);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_6.txt").unwrap();

        let (guard, map) = get_guard_and_map(&input);

        assert_eq!(part_2(&guard, &map), 41);
    }
}
