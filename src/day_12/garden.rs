use crate::util::point_2d::Point2d;
use std::collections::{HashMap, HashSet};

const UP: Point2d<i32> = Point2d { x: 0, y: -1 };
const RIGHT: Point2d<i32> = Point2d { x: 1, y: 0 };
const DOWN: Point2d<i32> = Point2d { x: 0, y: 1 };
const LEFT: Point2d<i32> = Point2d { x: -1, y: 0 };

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Plant {
    id: char,
}

impl From<char> for Plant {
    fn from(input: char) -> Self {
        Plant { id: input }
    }
}

#[derive(Debug, PartialEq)]
pub struct Region {
    plant: Plant,
    locations: HashSet<Point2d<i32>>,
}

impl Region {
    fn new(input: char) -> Self {
        Region {
            plant: Plant::from(input),
            locations: HashSet::new(),
        }
    }

    fn area(&self) -> usize {
        self.locations.len()
    }

    fn perimeter(&self) -> usize {
        self.locations
            .iter()
            .map(|location| 4 - self.number_of_locations_bordering(*location))
            .sum()
    }

    fn number_of_locations_bordering(&self, location: Point2d<i32>) -> usize {
        let border_locations = vec![
            location + UP,
            location + RIGHT,
            location + DOWN,
            location + LEFT,
        ];

        border_locations
            .into_iter()
            .filter(|location| self.locations.contains(location))
            .count()
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn add_location(&mut self, location: Point2d<i32>) -> bool {
        self.locations.insert(location)
    }
}

#[derive(Debug, PartialEq)]
pub struct Garden {
    regions: Vec<Region>,
}

impl Garden {
    fn new() -> Self {
        Garden {
            regions: Vec::new(),
        }
    }

    pub fn total_price(&self) -> usize {
        self.regions.iter().map(|region| region.price()).sum()
    }

    fn to_map(input: &[String]) -> HashMap<Point2d<i32>, Plant> {
        let mut result = HashMap::new();

        for (row, line) in input.iter().enumerate() {
            for (col, c) in line.char_indices() {
                let location = Point2d::new(col.try_into().unwrap(), row.try_into().unwrap());
                let plant = Plant::from(c);

                result.insert(location, plant);
            }
        }

        result
    }

    fn regions_from(map: &HashMap<Point2d<i32>, Plant>) -> Vec<Region> {
        unimplemented!()
    }
}

impl From<Vec<String>> for Garden {
    fn from(input: Vec<String>) -> Self {
        Self::from(input.as_slice())
    }
}

impl From<&Vec<String>> for Garden {
    fn from(input: &Vec<String>) -> Self {
        Self::from(input.as_slice())
    }
}

impl<const N: usize> From<[&str; N]> for Garden {
    fn from(input: [&str; N]) -> Self {
        Self::from(
            input
                .into_iter()
                .map(|line| line.to_string())
                .collect::<Vec<String>>(),
        )
    }
}

impl From<&[String]> for Garden {
    fn from(input: &[String]) -> Self {
        let map = Self::to_map(input);

        Garden {
            regions: Self::regions_from(&map),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_garden_from_str_array() {
        let a_region_1 = Region {
            plant: Plant::from('A'),
            locations: HashSet::from([
                Point2d::new(0, 0),
                Point2d::new(1, 0),
                Point2d::new(2, 0),
                Point2d::new(3, 0),
            ]),
        };
        let a_region_2 = Region {
            plant: Plant::from('A'),
            locations: HashSet::from([Point2d::new(0, 3), Point2d::new(1, 3), Point2d::new(2, 3)]),
        };
        let b_region = Region {
            plant: Plant::from('B'),
            locations: HashSet::from([
                Point2d::new(0, 1),
                Point2d::new(1, 1),
                Point2d::new(0, 2),
                Point2d::new(1, 2),
            ]),
        };
        let c_region = Region {
            plant: Plant::from('C'),
            locations: HashSet::from([
                Point2d::new(2, 1),
                Point2d::new(2, 2),
                Point2d::new(3, 2),
                Point2d::new(3, 3),
            ]),
        };
        let d_region = Region {
            plant: Plant::from('D'),
            locations: HashSet::from([Point2d::new(3, 2)]),
        };

        let expected = Garden {
            regions: vec![a_region_1, a_region_2, b_region, c_region, d_region],
        };

        let result = Garden::from(["AAAA", "BBCD", "BBCC", "AAAC"]);

        assert_eq!(result.regions.len(), expected.regions.len());

        for expected_region in expected.regions {
            assert!(result.regions.contains(&expected_region));
        }
    }

    #[test]
    fn test_garden_total_price() {
        let garden = Garden::from(["AAAA", "BBCD", "BBCC", "AAAC"]);

        assert_eq!(garden.total_price(), 140);
    }

    #[test]
    fn test_region_perimeter() {
        let locations = HashSet::from([
            Point2d::new(0, 0),
            Point2d::new(1, 0),
            Point2d::new(2, 0),
            Point2d::new(3, 0),
            Point2d::new(4, 0),
            Point2d::new(0, 1),
            Point2d::new(2, 1),
            Point2d::new(4, 1),
            Point2d::new(0, 2),
            Point2d::new(1, 2),
            Point2d::new(2, 2),
            Point2d::new(3, 2),
            Point2d::new(4, 2),
            Point2d::new(0, 3),
            Point2d::new(2, 3),
            Point2d::new(4, 3),
            Point2d::new(0, 4),
            Point2d::new(1, 4),
            Point2d::new(2, 4),
            Point2d::new(3, 4),
            Point2d::new(4, 4),
        ]);

        let region = Region {
            plant: Plant::from('O'),
            locations,
        };

        assert_eq!(region.perimeter(), 36);
    }
}
