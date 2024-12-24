mod warehouse;

use crate::util::file_reader::to_string_vector;
use crate::util::grid::Direction;

use warehouse::Warehouse;

pub fn run() {
    let input = to_string_vector("inputs/day_15.txt").expect("Something went wrong with Day 15!");

    println!("Day 15 Part 1: {:?}", part_1(&input));
    println!("Day 15 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> i32 {
    let (warehouse_input, moves_input) = split_input(input);

    calculate_gps_coordinate_sum(&warehouse_input, &moves_input)
}

fn part_2(input: &[String]) -> i32 {
    let (warehouse_input, moves_input) = split_input(input);

    let widened_warehouse_input = widen(&warehouse_input);

    calculate_gps_coordinate_sum(&widened_warehouse_input, &moves_input)
}

fn calculate_gps_coordinate_sum(warehouse_input: &[String], moves_input: &[String]) -> i32 {
    let mut warehouse = Warehouse::from(warehouse_input);

    let moves: Vec<Direction> = moves_input
        .iter()
        .flat_map(|line| line.chars())
        .map(Direction::from)
        .collect();

    warehouse.move_robot(&moves);

    warehouse.box_gps_coordinates().into_iter().sum()
}

fn split_input(input: &[String]) -> (Vec<String>, Vec<String>) {
    let input_split: Vec<Vec<String>> = input
        .split(String::is_empty)
        .map(<[String]>::to_vec)
        .collect();

    (
        input_split.first().cloned().unwrap(),
        input_split.last().cloned().unwrap(),
    )
}

fn widen(input: &[String]) -> Vec<String> {
    input
        .iter()
        .map(|line| {
            line.replace('#', "##")
                .replace('O', "[]")
                .replace('.', "..")
                .replace('@', "@.")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_15.txt").unwrap();

        assert_eq!(part_1(&input), 10_092);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_15.txt").unwrap();

        assert_eq!(part_2(&input), 9_021);
    }

    #[test]
    fn test_widen() {
        let input = to_string_vector("test_inputs/day_15.txt").unwrap();

        let warehouse_input = split_input(&input).0;

        let expected = vec![
            String::from("####################"),
            String::from("##....[]....[]..[]##"),
            String::from("##............[]..##"),
            String::from("##..[][]....[]..[]##"),
            String::from("##....[]@.....[]..##"),
            String::from("##[]##....[]......##"),
            String::from("##[]....[]....[]..##"),
            String::from("##..[][]..[]..[][]##"),
            String::from("##........[]......##"),
            String::from("####################"),
        ];

        let result = widen(&warehouse_input);

        assert_eq!(result, expected);
    }
}
