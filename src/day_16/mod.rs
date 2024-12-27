mod maze;

use crate::util::file_reader::to_string_vector;

use maze::Maze;

pub fn run() {
    let input = to_string_vector("inputs/day_16.txt").expect("Something went wrong with Day 16!");

    println!("Day 16 Part 1: {:?}", part_1(&input));
    println!("Day 16 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> u32 {
    let maze = Maze::from(input);

    maze.lowest_path_score()
}

fn part_2(input: &[String]) -> usize {
    let maze = Maze::from(input);

    maze.number_of_optimal_sitting_spots()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_16.txt").unwrap();

        assert_eq!(part_1(&input), 11_048);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_16.txt").unwrap();

        assert_eq!(part_2(&input), 64);
    }
}
