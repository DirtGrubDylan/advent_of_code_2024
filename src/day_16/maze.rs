use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fmt;

use crate::util::grid::{Direction, Grid};
use crate::util::point_2d::Point2d;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Item {
    Empty,
    Wall,
    Reindeer { facing: Direction },
    End,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_value = match self {
            Item::Empty => String::from("."),
            Item::Wall => String::from("#"),
            Item::Reindeer { facing: _ } => String::from("S"),
            Item::End => String::from("E"),
        };

        write!(f, "{string_value}")
    }
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '.' => Item::Empty,
            '#' => Item::Wall,
            'S' => Item::Reindeer {
                facing: Direction::Right,
            },
            'E' => Item::End,
            _ => panic!("Cannot parse {value} into an Item!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Maze {
    map: Grid<Item>,
    starting_location: Point2d<i32>,
}

impl Maze {
    fn path_with_lowest_score(&self) -> u32 {
        let mut score = 0;
        let mut heap = BinaryHeap::new();

        score
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.map)
    }
}

impl<const N: usize> From<&[&str; N]> for Maze {
    fn from(input: &[&str; N]) -> Self {
        Maze::from(input.as_slice())
    }
}

impl From<&[&str]> for Maze {
    fn from(input: &[&str]) -> Self {
        let input_strings: Vec<String> = input.iter().map(ToString::to_string).collect();

        Maze::from(input_strings.as_slice())
    }
}

impl From<&[String]> for Maze {
    fn from(input: &[String]) -> Self {
        let mut map = Grid::default();
        let mut starting_location = Point2d::default();

        for (row, line) in input.iter().enumerate() {
            for (col, c) in line.char_indices() {
                let point = Point2d::new(i32::try_from(col).unwrap(), i32::try_from(row).unwrap());

                let item = Item::from(c);

                if let &Item::Reindeer { facing: _ } = &item {
                    starting_location = point;
                }

                map.insert(point, &item);
            }
        }

        Maze {
            map,
            starting_location,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_from_str_array() {
        let expected_grid = Grid::from([
            (Point2d::new(0, 0), Item::Wall),
            (Point2d::new(1, 0), Item::Wall),
            (Point2d::new(2, 0), Item::Wall),
            (Point2d::new(0, 1), Item::Wall),
            (Point2d::new(1, 1), Item::End),
            (Point2d::new(2, 1), Item::Wall),
            (Point2d::new(0, 2), Item::Wall),
            (
                Point2d::new(1, 2),
                Item::Reindeer {
                    facing: Direction::Right,
                },
            ),
            (Point2d::new(2, 2), Item::Wall),
            (Point2d::new(0, 3), Item::Wall),
            (Point2d::new(1, 3), Item::Wall),
            (Point2d::new(2, 3), Item::Empty),
        ]);

        let expected = Maze {
            map: expected_grid,
            starting_location: Point2d::new(1, 2),
        };

        let result = Maze::from(&["###", "#E#", "#S#", "##."]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_maze_path_with_lowest_score() {
        let maze = Maze::from(&[
            "###############",
            "#.......#....E#",
            "#.#.###.#.###.#",
            "#.....#.#...#.#",
            "#.###.#####.#.#",
            "#.#.#.......#.#",
            "#.#.#####.###.#",
            "#...........#.#",
            "###.#.#####.#.#",
            "#...#.....#.#.#",
            "#.#.#.###.#.#.#",
            "#.....#...#.#.#",
            "#.###.#.#.#.#.#",
            "#S..#.....#...#",
            "###############",
        ]);

        assert_eq!(maze.path_with_lowest_score(), 7_036);
    }
}
