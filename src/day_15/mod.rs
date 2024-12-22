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
    let input_split: Vec<Vec<String>> = input
        .split(String::is_empty)
        .map(<[String]>::to_vec)
        .collect();

    let (warehouse_input, moves_input) =
        (input_split.first().unwrap(), input_split.last().unwrap());

    let mut warehouse = Warehouse::from(warehouse_input.as_slice());
    let moves: Vec<Direction> = moves_input
        .iter()
        .flat_map(|line| line.chars())
        .map(Direction::from)
        .collect();

    warehouse.move_robot(&moves);

    warehouse.box_gps_coordinates().into_iter().sum()
}

fn part_2(_input: &[String]) -> usize {
    unimplemented!()
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
    #[should_panic(expected = "not implemented")]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_15.txt").unwrap();

        assert_eq!(part_2(&input), 666);
    }
}
