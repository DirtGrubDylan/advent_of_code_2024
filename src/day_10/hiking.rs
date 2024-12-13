use crate::util::point_2d::Point2d;
use std::collections::{HashMap, HashSet};

const UP: Point2d<i32> = Point2d { x: 0, y: -1 };
const RIGHT: Point2d<i32> = Point2d { x: 1, y: 0 };
const DOWN: Point2d<i32> = Point2d { x: 0, y: 1 };
const LEFT: Point2d<i32> = Point2d { x: -1, y: 0 };

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn as_offset(self) -> Point2d<i32> {
        match self {
            Direction::Up => UP,
            Direction::Right => RIGHT,
            Direction::Down => DOWN,
            Direction::Left => LEFT,
        }
    }

    fn turn_90_degrees_clockwise(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Map {
    heights: HashMap<Point2d<i32>, u32>,
}

impl Map {
    pub fn new(input: &[String]) -> Self {
        let mut heights = HashMap::new();

        for (row, line) in input.iter().enumerate() {
            for (col, c) in line.char_indices() {
                let point = Point2d::new(col.try_into().unwrap(), row.try_into().unwrap());

                if let Some(digit) = c.to_digit(10) {
                    heights.insert(point, digit);
                }
            }
        }

        Map { heights }
    }

    fn number_of_peak_paths_from(&self, point: Point2d<i32>) -> u32 {
        // match self.heights.get(&point) {
        //     None | Some(height) if height
        // }

        unimplemented!()
    }

    fn number_of_valid_paths_from(
        &self,
        current_point: Point2d<i32>,
        next_point: Point2d<i32>,
    ) -> u32 {
        let current_height = self
            .heights
            .get(&current_point)
            .expect(&format!("Map does not have {current_point:?}"));

        match self.heights.get(&next_point) {
            Some(9) => 1,
            Some(next_height) if next_height == current_height + 1 => {
                self.number_of_valid_paths_from(next_point, next_point + UP)
                    + self.number_of_valid_paths_from(next_point, next_point + RIGHT)
                    + self.number_of_valid_paths_from(next_point, next_point + DOWN)
                    + self.number_of_valid_paths_from(next_point, next_point + LEFT)
            }
            _ => 0,
        }
    }

    fn bases(&self) -> HashSet<Point2d<i32>> {
        self.points_with_height(0)
    }

    fn peaks(&self) -> HashSet<Point2d<i32>> {
        self.points_with_height(9)
    }

    fn points_with_height(&self, height: u32) -> HashSet<Point2d<i32>> {
        self.heights
            .iter()
            .filter(|(_, value)| **value == height)
            .map(|(point, _)| *point)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_new() {
        let input = [
            String::from("0..."),
            String::from("1234"),
            String::from("2..5"),
            String::from("9876"),
        ];

        let expected_heights = HashMap::from([
            (Point2d::new(0, 0), 0),
            (Point2d::new(0, 1), 1),
            (Point2d::new(1, 1), 2),
            (Point2d::new(2, 1), 3),
            (Point2d::new(3, 1), 4),
            (Point2d::new(0, 2), 2),
            (Point2d::new(3, 2), 5),
            (Point2d::new(3, 3), 6),
            (Point2d::new(2, 3), 7),
            (Point2d::new(1, 3), 8),
            (Point2d::new(0, 3), 9),
        ]);

        let expected = Map {
            heights: expected_heights,
        };

        let result = Map::new(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_map_bases() {
        let input = [
            String::from("10..9.."),
            String::from("2...8.."),
            String::from("3...7.."),
            String::from("4567654"),
            String::from("...8..3"),
            String::from("...9..2"),
            String::from(".....01"),
        ];

        let map = Map::new(&input);

        let expected = HashSet::from([Point2d::new(1, 0), Point2d::new(5, 6)]);

        let result = map.bases();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_map_peaks() {
        let input = [
            String::from("10..9.."),
            String::from("2...8.."),
            String::from("3...7.."),
            String::from("4567654"),
            String::from("...8..3"),
            String::from("...9..2"),
            String::from(".....01"),
        ];

        let map = Map::new(&input);

        let expected = HashSet::from([Point2d::new(4, 0), Point2d::new(3, 5)]);

        let result = map.peaks();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_map_number_of_peak_paths_from() {
        let input = [
            String::from("..90..9"),
            String::from("...1.98"),
            String::from("...2..7"),
            String::from("6543456"),
            String::from("765.987"),
            String::from("876...."),
            String::from("987...."),
        ];

        let map = Map::new(&input);

        assert_eq!(map.number_of_peak_paths_from(Point2d::new(3, 0)), 4);
    }
}
