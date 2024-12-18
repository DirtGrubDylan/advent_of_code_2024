mod claw_machine;

use crate::util::file_reader::to_string_vector;

use claw_machine::ClawMachine;

pub fn run() {
    let input = to_string_vector("inputs/day_13.txt").expect("Something went wrong with Day 13!");

    println!("Day 13 Part 1: {:?}", part_1(&input));
    println!("Day 13 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> i64 {
    input
        .split(String::is_empty)
        .map(ClawMachine::from)
        .filter_map(|machine| machine.cost_to_get_prize(3, 1))
        .sum()
}

fn part_2(input: &[String]) -> i64 {
    input
        .split(String::is_empty)
        .map(ClawMachine::from)
        .filter_map(|mut machine| {
            machine.adjust_prize_location(10_000_000_000_000);

            machine.cost_to_get_prize(3, 1)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_13.txt").unwrap();

        assert_eq!(part_1(&input), 480);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_13.txt").unwrap();

        assert_eq!(part_2(&input), 875_318_608_908);
    }
}
