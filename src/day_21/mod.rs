mod keypad;

use crate::util::file_reader::to_string_vector;

use keypad::{DirectionPad, NumberPadButton};

pub fn run() {
    let input = to_string_vector("inputs/day_21.txt").expect("Something went wrong with Day 21!");

    println!("Day 21 Part 1: {:?}", part_1(&input));
    println!("Day 21 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> u64 {
    let dpad = DirectionPad::lowest_pad_of(3);

    input.iter().map(|code| complexity_for(code, &dpad)).sum()
}

fn part_2(input: &[String]) -> u64 {
    let dpad = DirectionPad::lowest_pad_of(26);

    input.iter().map(|code| complexity_for(code, &dpad)).sum()
}

fn complexity_for(code: &str, dpad: &DirectionPad) -> u64 {
    let code_value: u64 = code.trim_end_matches('A').parse().unwrap();

    let mut presses_required_from_dpad = Vec::new();
    let mut current_number = NumberPadButton::A;

    for next_number in code.chars().map(NumberPadButton::from) {
        presses_required_from_dpad.extend(
            current_number
                .possible_presses_to_traverse_and_press(next_number)
                .iter()
                .min_by_key(|presses| dpad.cost_to_press(presses))
                .unwrap(),
        );

        current_number = next_number;
    }

    code_value * dpad.cost_to_press(&presses_required_from_dpad)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_21.txt").unwrap();

        assert_eq!(part_1(&input), 12_6384);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_21.txt").unwrap();

        assert_eq!(part_2(&input), 154_115_708_116_294);
    }
}
