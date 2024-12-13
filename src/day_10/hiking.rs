use crate::util::point_2d::Point2d;
use std::collections::{HashMap, HashSet};

const UP: Point2d<i32> = Point2d { x: 0, y: -1 };
const RIGHT: Point2d<i32> = Point2d { x: 1, y: 0 };
const DOWN: Point2d<i32> = Point2d { x: 0, y: 1 };
const LEFT: Point2d<i32> = Point2d { x: -1, y: 0 };

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

    pub fn number_of_reachable_peaks(&self) -> usize {
        self.bases()
            .into_iter()
            .map(|base| self.number_of_reachable_peaks_from(base))
            .sum()
    }

    pub fn number_of_distinct_valid_paths(&self) -> usize {
        self.bases()
            .into_iter()
            .map(|base| self.number_of_distinct_valid_paths_from(base))
            .sum()
    }

    fn number_of_reachable_peaks_from(&self, point: Point2d<i32>) -> usize {
        let mut valid_reachable_peaks = HashSet::new();

        valid_reachable_peaks.extend(self.valid_reachable_peaks_from(point, point + UP));
        valid_reachable_peaks.extend(self.valid_reachable_peaks_from(point, point + RIGHT));
        valid_reachable_peaks.extend(self.valid_reachable_peaks_from(point, point + DOWN));
        valid_reachable_peaks.extend(self.valid_reachable_peaks_from(point, point + LEFT));

        valid_reachable_peaks.len()
    }

    fn number_of_distinct_valid_paths_from(&self, point: Point2d<i32>) -> usize {
        let mut peaks_with_distinct_path = Vec::new();

        peaks_with_distinct_path.extend(self.valid_reachable_peaks_from(point, point + UP));
        peaks_with_distinct_path.extend(self.valid_reachable_peaks_from(point, point + RIGHT));
        peaks_with_distinct_path.extend(self.valid_reachable_peaks_from(point, point + DOWN));
        peaks_with_distinct_path.extend(self.valid_reachable_peaks_from(point, point + LEFT));

        peaks_with_distinct_path.len()
    }

    fn valid_reachable_peaks_from(
        &self,
        current_point: Point2d<i32>,
        next_point: Point2d<i32>,
    ) -> Vec<Point2d<i32>> {
        let mut result = Vec::new();

        let current_height = self
            .heights
            .get(&current_point)
            .unwrap_or_else(|| panic!("Map does not have {current_point:?}"));

        match self.heights.get(&next_point) {
            None => {}
            Some(&next_height) if next_height != current_height + 1 => {}
            Some(9) => {
                result.push(next_point);
            }
            _ => {
                result.extend(self.valid_reachable_peaks_from(next_point, next_point + UP));
                result.extend(self.valid_reachable_peaks_from(next_point, next_point + RIGHT));
                result.extend(self.valid_reachable_peaks_from(next_point, next_point + DOWN));
                result.extend(self.valid_reachable_peaks_from(next_point, next_point + LEFT));
            }
        };

        result
    }

    fn bases(&self) -> HashSet<Point2d<i32>> {
        self.points_with_height(0)
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
    fn test_map_number_of_reachable_peaks() {
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

        assert_eq!(map.number_of_reachable_peaks(), 3);
    }

    #[test]
    fn test_map_number_of_distinct_valid_paths() {
        let input = [
            String::from("012345"),
            String::from("123456"),
            String::from("234567"),
            String::from("345678"),
            String::from("4.6789"),
            String::from("56789."),
        ];

        let map = Map::new(&input);

        assert_eq!(map.number_of_distinct_valid_paths(), 227);
    }

    #[test]
    fn test_map_number_of_reachable_peaks_from() {
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
        let base_point = Point2d::new(3, 0);

        assert_eq!(map.number_of_reachable_peaks_from(base_point), 4);
    }
    #[test]
    fn test_map_number_of_distinct_valid_paths_from() {
        let input = [
            String::from(".....0."),
            String::from("..4321."),
            String::from("..5..2."),
            String::from("..6543."),
            String::from("..7..4."),
            String::from("..8765."),
            String::from("..9...."),
        ];

        let map = Map::new(&input);
        let base_point = Point2d::new(5, 0);

        assert_eq!(map.number_of_distinct_valid_paths_from(base_point), 3);
    }

    #[test]
    fn test_map_valid_reachable_peaks_from_empty() {
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
        let base_point = Point2d::new(3, 0);

        let result = map.valid_reachable_peaks_from(base_point, base_point + LEFT);

        assert!(result.is_empty());
    }
    #[test]
    fn test_map_valid_reachable_peaks_from_non_empty() {
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
        let base_point = Point2d::new(3, 0);

        let expected = vec![
            Point2d::new(6, 0),
            Point2d::new(5, 1),
            Point2d::new(4, 4),
            Point2d::new(0, 6),
            Point2d::new(0, 6),
            Point2d::new(0, 6),
            Point2d::new(0, 6),
            Point2d::new(0, 6),
            Point2d::new(0, 6),
            Point2d::new(0, 6),
            Point2d::new(0, 6),
            Point2d::new(0, 6),
            Point2d::new(0, 6),
        ];

        let result = map.valid_reachable_peaks_from(base_point, base_point + DOWN);

        assert_eq!(result, expected);
    }
}
