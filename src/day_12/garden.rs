use crate::util::point_2d::Point2d;
use std::collections::{HashMap, HashSet};
use std::string::ToString;

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
    fn new(plant: Plant) -> Self {
        Region {
            plant,
            locations: HashSet::new(),
        }
    }

    fn price(&self, with_discount: bool) -> usize {
        if with_discount {
            self.area() * self.number_of_sides()
        } else {
            self.area() * self.perimeter()
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

    fn number_of_sides(&self) -> usize {
        let up_edges = self.edges_facing(UP);
        let right_edges = self.edges_facing(RIGHT);
        let down_edges = self.edges_facing(DOWN);
        let left_edges = self.edges_facing(LEFT);

        Self::number_of_contiguous_lines_in(&up_edges, RIGHT)
            + Self::number_of_contiguous_lines_in(&right_edges, DOWN)
            + Self::number_of_contiguous_lines_in(&down_edges, RIGHT)
            + Self::number_of_contiguous_lines_in(&left_edges, DOWN)
    }

    fn edges_facing(&self, facing_offset: Point2d<i32>) -> HashSet<Point2d<i32>> {
        self.locations
            .iter()
            .copied()
            .filter(|&location| !self.locations.contains(&(location + facing_offset)))
            .collect()
    }

    fn number_of_contiguous_lines_in(
        locations: &HashSet<Point2d<i32>>,
        offset: Point2d<i32>,
    ) -> usize {
        let mut result = 0;

        let mut locations_clone = locations.clone();

        while let Some(&location) = locations_clone.iter().nth(0) {
            result += 1;

            locations_clone.remove(&location);

            let mut location_back = location - offset;
            let mut location_front = location + offset;

            while locations_clone.remove(&location_back) {
                location_back -= offset;
            }

            while locations_clone.remove(&location_front) {
                location_front += offset;
            }
        }

        result
    }

    fn add_locations(&mut self, locations: &HashSet<Point2d<i32>>) {
        self.locations.extend(locations);
    }
}

#[derive(Debug, PartialEq)]
pub struct Garden {
    regions: Vec<Region>,
}

impl Garden {
    pub fn total_price(&self, with_discount: bool) -> usize {
        self.regions
            .iter()
            .map(|region| region.price(with_discount))
            .sum()
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
        let mut result = Vec::new();

        let mut seen = HashSet::new();

        for (&start_point, &target) in map {
            if seen.contains(&start_point) {
                continue;
            }

            let mut region = Region::new(target);

            region.add_locations(&Self::valid_neighbors_for(
                start_point,
                target,
                &mut seen,
                map,
            ));

            result.push(region);
        }

        result
    }

    fn valid_neighbors_for(
        location: Point2d<i32>,
        target: Plant,
        seen: &mut HashSet<Point2d<i32>>,
        map: &HashMap<Point2d<i32>, Plant>,
    ) -> HashSet<Point2d<i32>> {
        let mut result = HashSet::new();

        match map.get(&location) {
            Some(&plant) if (plant == target) && !seen.contains(&location) => {
                seen.insert(location);
                result.insert(location);

                result.extend(Self::valid_neighbors_for(location + UP, target, seen, map));
                result.extend(Self::valid_neighbors_for(
                    location + RIGHT,
                    target,
                    seen,
                    map,
                ));
                result.extend(Self::valid_neighbors_for(
                    location + DOWN,
                    target,
                    seen,
                    map,
                ));
                result.extend(Self::valid_neighbors_for(
                    location + LEFT,
                    target,
                    seen,
                    map,
                ));
            }
            _ => {}
        }

        result
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
                .map(ToString::to_string)
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
            locations: HashSet::from([Point2d::new(3, 1)]),
        };

        let expected = Garden {
            regions: vec![a_region_1, a_region_2, b_region, c_region, d_region],
        };

        let result = Garden::from(["AAAA", "BBCD", "BBCC", "AAAC"]);

        assert_eq!(result.regions.len(), expected.regions.len());

        for expected_region in expected.regions {
            assert!(
                result.regions.contains(&expected_region),
                "{expected_region:?}"
            );
        }
    }

    #[test]
    fn test_garden_total_price_no_discount() {
        let garden = Garden::from(["AAAA", "BBCD", "BBCC", "AAAC"]);

        assert_eq!(garden.total_price(/*with_discount=*/ false), 140);
    }

    #[test]
    fn test_garden_total_price_with_discount() {
        let garden_1 = Garden::from(["AAAA", "BBCD", "BBCC", "AAAC"]);
        let garden_2 = Garden::from(["EEEEE", "EXXXX", "EEEEE", "EXXXX", "EEEEE"]);

        assert_eq!(garden_1.total_price(/*with_discount=*/ true), 80);
        assert_eq!(garden_2.total_price(/*with_discount=*/ true), 236);
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

    #[test]
    fn test_region_edges_facing_up() {
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

        let expected = HashSet::from([
            Point2d::new(0, 0),
            Point2d::new(1, 0),
            Point2d::new(2, 0),
            Point2d::new(3, 0),
            Point2d::new(4, 0),
            Point2d::new(1, 2),
            Point2d::new(3, 2),
            Point2d::new(1, 4),
            Point2d::new(3, 4),
        ]);

        let result = region.edges_facing(UP);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_region_edges_facing_right() {
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

        let expected = HashSet::from([
            Point2d::new(4, 0),
            Point2d::new(0, 1),
            Point2d::new(2, 1),
            Point2d::new(4, 1),
            Point2d::new(4, 2),
            Point2d::new(0, 3),
            Point2d::new(2, 3),
            Point2d::new(4, 3),
            Point2d::new(4, 4),
        ]);

        let result = region.edges_facing(RIGHT);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_region_edges_facing_down() {
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

        let expected = HashSet::from([
            Point2d::new(1, 0),
            Point2d::new(3, 0),
            Point2d::new(1, 2),
            Point2d::new(3, 2),
            Point2d::new(0, 4),
            Point2d::new(1, 4),
            Point2d::new(2, 4),
            Point2d::new(3, 4),
            Point2d::new(4, 4),
        ]);

        let result = region.edges_facing(DOWN);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_region_edges_facing_left() {
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

        let expected = HashSet::from([
            Point2d::new(0, 0),
            Point2d::new(0, 1),
            Point2d::new(2, 1),
            Point2d::new(4, 1),
            Point2d::new(0, 2),
            Point2d::new(0, 3),
            Point2d::new(2, 3),
            Point2d::new(4, 3),
            Point2d::new(0, 4),
        ]);

        let result = region.edges_facing(LEFT);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_region_number_of_contiguous_lines_in_horizontal() {
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

        let edges = region.edges_facing(UP);

        assert_eq!(Region::number_of_contiguous_lines_in(&edges, RIGHT), 5);
    }

    #[test]
    fn test_region_number_of_contiguous_lines_in_vertical() {
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

        let edges = region.edges_facing(RIGHT);

        assert_eq!(Region::number_of_contiguous_lines_in(&edges, DOWN), 5);
    }
}
