use std::collections::{HashMap, HashSet};

use crate::util::point_2d::Point2d;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Antenna {
    frequency: char,
}

impl From<char> for Antenna {
    fn from(input: char) -> Self {
        Antenna { frequency: input }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Antinode {
    frequency: char,
}

impl Antinode {
    fn position_from(first: &Location, second: &Location) -> (Point2d<i32>, Point2d<i32>) {
        assert!(first.antenna.is_some() && second.antenna.is_some());
        assert_eq!(first.antenna, second.antenna);

        let position_before_first = first.position + first.position - second.position;
        let position_after_second = second.position + second.position - first.position;

        (position_before_first, position_after_second)
    }

    fn position_from_extended(
        first: &Location,
        second: &Location,
        max: Point2d<i32>,
    ) -> Vec<Point2d<i32>> {
        let mut result = Vec::new();
        let mut current_position = first.position;
        let slope = (second.position - first.position).reduce();

        let mut has_valid_x = 0 <= current_position.x && current_position.x <= max.x;
        let mut has_valid_y = 0 <= current_position.y && current_position.y <= max.y;

        while has_valid_x && has_valid_y {
            current_position -= slope;

            has_valid_x = 0 <= current_position.x && current_position.x <= max.x;
            has_valid_y = 0 <= current_position.y && current_position.y <= max.y;
        }

        current_position += slope;

        has_valid_x = 0 <= current_position.x && current_position.x <= max.x;
        has_valid_y = 0 <= current_position.y && current_position.y <= max.y;

        while has_valid_x && has_valid_y {
            result.push(current_position);

            current_position += slope;

            has_valid_x = 0 <= current_position.x && current_position.x <= max.x;
            has_valid_y = 0 <= current_position.y && current_position.y <= max.y;
        }

        result
    }
}

impl From<char> for Antinode {
    fn from(input: char) -> Self {
        Antinode { frequency: input }
    }
}

impl From<Antenna> for Antinode {
    fn from(antenna: Antenna) -> Self {
        Antinode {
            frequency: antenna.frequency,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Map {
    data: HashMap<Point2d<i32>, Location>,
    raw_data: Vec<Vec<char>>,
}

impl Map {
    #[allow(dead_code)]
    pub fn print(&self) {
        self.raw_data
            .iter()
            .map(|row| row.iter().collect::<String>())
            .for_each(|line| println!("{line}"));
    }

    pub fn number_of_antinodes(&self) -> usize {
        self.data
            .values()
            .filter(|location| !location.antinodes.is_empty())
            .count()
    }

    pub fn populate_antinodes(&mut self, use_extended: bool) {
        for (position, antinode) in self.find_antinodes(use_extended) {
            if !self.data.contains_key(&position) {
                continue;
            }

            self.data.entry(position).and_modify(|location| {
                location.add_antinode(antinode);
            });

            let element = self
                .raw_data
                .get_mut(usize::try_from(position.y).unwrap())
                .and_then(|row| row.get_mut(usize::try_from(position.x).unwrap()))
                .filter(|element| **element == '.');

            if let Some(data) = element {
                *data = '#';
            }
        }
    }

    fn find_antinodes(&self, use_extended: bool) -> Vec<(Point2d<i32>, Antinode)> {
        let mut result = Vec::new();

        for (frequency, locations) in self.frequency_to_locations() {
            if frequency == '.' || frequency == '#' {
                continue;
            }

            for (index, first_location) in locations.iter().enumerate() {
                for second_location in locations.iter().skip(index + 1) {
                    let antinode = Antinode::from(frequency);

                    if use_extended {
                        let antinode_locations = Antinode::position_from_extended(
                            first_location,
                            second_location,
                            self.max_point(),
                        );

                        for antinode_location in antinode_locations {
                            result.push((antinode_location, antinode));
                        }
                    } else {
                        let (first_antinode_position, second_antinode_position) =
                            Antinode::position_from(first_location, second_location);

                        result.push((first_antinode_position, antinode));
                        result.push((second_antinode_position, antinode));
                    }
                }
            }
        }

        result
    }

    fn frequency_to_locations(&self) -> HashMap<char, Vec<&Location>> {
        let mut result = HashMap::new();

        for location in self.data.values() {
            let frequency = location.antenna.map_or('.', |antenna| antenna.frequency);

            result.entry(frequency).or_insert(Vec::new()).push(location);
        }

        result
    }

    fn max_point(&self) -> Point2d<i32> {
        let max_row = self.raw_data.len();
        let max_col = self.raw_data.first().map_or(0, Vec::len);

        Point2d::new(
            i32::try_from(max_row).unwrap(),
            i32::try_from(max_col).unwrap(),
        )
    }
}

impl<'a> From<&'a [String]> for Map {
    fn from(input: &'a [String]) -> Self {
        let raw_data = input.iter().map(|line| line.chars().collect()).collect();

        let mut data = HashMap::new();

        for (row, line) in input.iter().enumerate() {
            for (col, item) in line.char_indices() {
                let location = Location::new(col, row, item);
                let position = location.position;

                data.insert(position, location);
            }
        }

        Map { data, raw_data }
    }
}

impl From<&Vec<String>> for Map {
    fn from(input: &Vec<String>) -> Self {
        Map::from(input.as_slice())
    }
}

#[derive(Debug, PartialEq)]
struct Location {
    position: Point2d<i32>,
    antenna: Option<Antenna>,
    antinodes: HashSet<Antinode>,
}

impl Location {
    fn new(col: usize, row: usize, data: char) -> Self {
        let position = Point2d::new(i32::try_from(col).unwrap(), i32::try_from(row).unwrap());
        let antenna = Self::get_possible_antenna(data);
        let antinodes = HashSet::new();

        Location {
            position,
            antenna,
            antinodes,
        }
    }

    fn add_antinode(&mut self, antinode: Antinode) -> bool {
        self.antinodes.insert(antinode)
    }

    fn get_possible_antenna(data: char) -> Option<Antenna> {
        match data {
            '.' => None,
            frequency => Some(Antenna::from(frequency)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antinode_position_from() {
        let antenna_1 = Location::new(8, 4, 'a');
        let antenna_2 = Location::new(5, 5, 'a');

        let expected = (Point2d::new(11, 3), Point2d::new(2, 6));

        let result = Antinode::position_from(&antenna_1, &antenna_2);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_antinode_position_from_extended() {
        let antenna_1 = Location::new(8, 8, 'A');
        let antenna_2 = Location::new(6, 6, 'A');
        let map_max_point = Point2d::new(9, 9);

        let expected = vec![
            Point2d::new(9, 9),
            Point2d::new(8, 8),
            Point2d::new(7, 7),
            Point2d::new(6, 6),
            Point2d::new(5, 5),
            Point2d::new(4, 4),
            Point2d::new(3, 3),
            Point2d::new(2, 2),
            Point2d::new(1, 1),
            Point2d::new(0, 0),
        ];

        let result = Antinode::position_from_extended(&antenna_1, &antenna_2, map_max_point);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_antenna_map_from_string_slice() {
        let input = vec![
            String::from("..."),
            String::from(".A."),
            String::from("..."),
        ];

        let expected_data = HashMap::from([
            (Point2d::new(0, 0), Location::new(0, 0, '.')),
            (Point2d::new(1, 0), Location::new(1, 0, '.')),
            (Point2d::new(2, 0), Location::new(2, 0, '.')),
            (Point2d::new(0, 1), Location::new(0, 1, '.')),
            (Point2d::new(1, 1), Location::new(1, 1, 'A')),
            (Point2d::new(2, 1), Location::new(2, 1, '.')),
            (Point2d::new(0, 2), Location::new(0, 2, '.')),
            (Point2d::new(1, 2), Location::new(1, 2, '.')),
            (Point2d::new(2, 2), Location::new(2, 2, '.')),
        ]);

        let result_data = Map::from(&input).data;

        assert_eq!(result_data, expected_data);
    }

    #[test]
    fn test_antenna_map_find_antinodes_not_extended() {
        let input = vec![
            String::from(".........."),
            String::from(".........."),
            String::from(".........."),
            String::from("....a....."),
            String::from("........a."),
            String::from(".....a...."),
            String::from(".........."),
            String::from("......A..."),
            String::from(".........."),
            String::from(".........."),
        ];

        let map = Map::from(&input);

        let expected = vec![
            (Point2d::new(3, 1), Antinode::from('a')),
            (Point2d::new(0, 2), Antinode::from('a')),
            (Point2d::new(2, 6), Antinode::from('a')),
            (Point2d::new(6, 7), Antinode::from('a')),
            (Point2d::new(11, 3), Antinode::from('a')),
            (Point2d::new(12, 5), Antinode::from('a')),
        ];

        let result = map.find_antinodes(/*use_extended=*/ false);

        assert_eq!(result.len(), expected.len());

        for expected_element in expected {
            assert!(result.contains(&expected_element));
        }
    }

    #[test]
    fn test_antenna_map_find_antinodes_extended() {
        // a1 = (3, 2)
        // a2 = (5, 6)
        // a2 - a1 = (2, 4) = (1, 2)
        let input = vec![
            String::from(".........."),
            String::from(".........."),
            String::from("...a......"),
            String::from(".........."),
            String::from(".........."),
            String::from(".........."),
            String::from(".....a...."),
            String::from(".........."),
            String::from("......A..."),
            String::from(".........."),
            String::from(".........."),
        ];

        let map = Map::from(&input);

        let expected = vec![
            (Point2d::new(2, 0), Antinode::from('a')),
            (Point2d::new(3, 2), Antinode::from('a')),
            (Point2d::new(4, 4), Antinode::from('a')),
            (Point2d::new(5, 6), Antinode::from('a')),
            (Point2d::new(6, 8), Antinode::from('a')),
            (Point2d::new(7, 10), Antinode::from('a')),
        ];

        let result = map.find_antinodes(/*use_extended=*/ true);

        println!("{result:?}");

        assert_eq!(result.len(), expected.len());

        for expected_element in expected {
            assert!(result.contains(&expected_element));
        }
    }

    #[test]
    fn test_antenna_map_frequency_to_locations() {
        let input = vec![
            String::from("..."),
            String::from(".A."),
            String::from("..."),
        ];

        let map = Map::from(&input);

        let location_0 = Location::new(0, 0, '.');
        let location_1 = Location::new(1, 0, '.');
        let location_2 = Location::new(2, 0, '.');
        let location_3 = Location::new(0, 1, '.');
        let location_4 = Location::new(2, 1, '.');
        let location_5 = Location::new(0, 2, '.');
        let location_6 = Location::new(1, 2, '.');
        let location_7 = Location::new(2, 2, '.');
        let location_8 = Location::new(1, 1, 'A');

        let expected = HashMap::from([
            (
                '.',
                vec![
                    &location_0,
                    &location_1,
                    &location_2,
                    &location_3,
                    &location_4,
                    &location_5,
                    &location_6,
                    &location_7,
                ],
            ),
            ('A', vec![&location_8]),
        ]);

        let result = map.frequency_to_locations();

        for (frequency, result_locations) in result {
            let expected_locations = expected.get(&frequency).unwrap();

            assert_eq!(result_locations.len(), expected_locations.len());

            for expected_location in expected_locations {
                assert!(result_locations.contains(expected_location));
            }
        }
    }

    #[test]
    fn test_location_new() {
        let expected_non_empty = Location {
            position: Point2d::new(1, 1),
            antenna: Some(Antenna::from('A')),
            antinodes: HashSet::new(),
        };
        let expected_empty = Location {
            position: Point2d::new(1, 2),
            antenna: None,
            antinodes: HashSet::new(),
        };

        let result_non_empty = Location::new(1, 1, 'A');
        let result_empty = Location::new(1, 2, '.');

        assert_eq!(result_non_empty, expected_non_empty);
        assert_eq!(result_empty, expected_empty);
    }
}
