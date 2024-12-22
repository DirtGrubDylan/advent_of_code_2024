use std::fmt;

use crate::util::grid::{Direction, Grid};
use crate::util::point_2d::Point2d;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Item {
    Empty,
    Wall,
    Box,
    Robot { facing: Direction },
}

impl Item {}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_value = match self {
            Item::Empty => String::from("."),
            Item::Wall => String::from("#"),
            Item::Box => String::from("O"),
            Item::Robot { facing: _ } => String::from("@"),
        };

        write!(f, "{string_value}")
    }
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '.' => Item::Empty,
            '#' => Item::Wall,
            'O' => Item::Box,
            '@' => Item::Robot {
                facing: Direction::Up,
            },
            _ => panic!("Cannot parse {value} into an Item!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Warehouse {
    map: Grid<Item>,
    robot_location: Point2d<i32>,
}

impl Warehouse {
    pub fn box_gps_coordinates(&self) -> Vec<i32> {
        self.map
            .iter()
            .filter(|(_, item)| **item == Item::Box)
            .map(|(point, _)| point.x + 100 * point.y)
            .collect()
    }

    pub fn move_robot(&mut self, moves: &[Direction]) {
        for direction in moves {
            self.swap(
                self.robot_location,
                self.robot_location + direction.as_offset(),
                *direction,
            );
        }
    }

    fn swap(
        &mut self,
        a: Point2d<i32>,
        b: Point2d<i32>,
        direction: Direction,
    ) -> Option<Point2d<i32>> {
        let item_a = self.map.get(a).copied().unwrap();
        let item_b = self.map.get(b).copied().unwrap();

        let result = match item_b {
            Item::Box => self
                .swap(b, b + direction.as_offset(), direction)
                .and_then(|_| self.swap(a, b, direction)),
            Item::Empty => {
                self.map.insert(b, &item_a);

                self.map.insert(a, &item_b);

                Some(b)
            }
            _ => None,
        };

        if let (Item::Robot { facing: _ }, Some(point)) = (item_a, result) {
            self.robot_location = point;
        }

        result
    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.map)
    }
}

impl<const N: usize> From<&[&str; N]> for Warehouse {
    fn from(input: &[&str; N]) -> Self {
        Warehouse::from(input.as_slice())
    }
}

impl From<&[&str]> for Warehouse {
    fn from(input: &[&str]) -> Self {
        let input_strings: Vec<String> = input.iter().map(ToString::to_string).collect();

        Warehouse::from(input_strings.as_slice())
    }
}

impl From<&[String]> for Warehouse {
    fn from(input: &[String]) -> Self {
        let mut map = Grid::default();
        let mut robot_start = Point2d::default();

        for (row, line) in input.iter().enumerate() {
            for (col, c) in line.char_indices() {
                let point = Point2d::new(i32::try_from(col).unwrap(), i32::try_from(row).unwrap());

                let item = Item::from(c);

                if let &Item::Robot { facing: _ } = &item {
                    robot_start = point;
                }

                map.insert(point, &item);
            }
        }

        Warehouse {
            map,
            robot_location: robot_start,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warehouse_from_str_array() {
        let input = ["###", "#O#", "#@#", "###"];

        let expected_grid = Grid::from([
            (Point2d::new(0, 0), Item::Wall),
            (Point2d::new(1, 0), Item::Wall),
            (Point2d::new(2, 0), Item::Wall),
            (Point2d::new(0, 1), Item::Wall),
            (Point2d::new(1, 1), Item::Box),
            (Point2d::new(2, 1), Item::Wall),
            (Point2d::new(0, 2), Item::Wall),
            (
                Point2d::new(1, 2),
                Item::Robot {
                    facing: Direction::Up,
                },
            ),
            (Point2d::new(2, 2), Item::Wall),
            (Point2d::new(0, 3), Item::Wall),
            (Point2d::new(1, 3), Item::Wall),
            (Point2d::new(2, 3), Item::Wall),
        ]);

        let expected = Warehouse {
            map: expected_grid,
            robot_location: Point2d::new(1, 2),
        };

        let result = Warehouse::from(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_warehouse_swap_robot_empty() {
        let input = [
            "##########",
            "#..O..O.O#",
            "#......O.#",
            "#.OO..O.O#",
            "#..O@..O.#",
            "#O#..O...#",
            "#O..O..O.#",
            "#.OO.O.OO#",
            "#....O...#",
            "##########",
        ];

        let mut warehouse = Warehouse::from(&input);

        let expected_warehouse = Warehouse::from(&[
            "##########",
            "#..O..O.O#",
            "#......O.#",
            "#.OO@.O.O#",
            "#..O...O.#",
            "#O#..O...#",
            "#O..O..O.#",
            "#.OO.O.OO#",
            "#....O...#",
            "##########",
        ]);

        let expected = Point2d::new(4, 3);

        let result = warehouse.swap(Point2d::new(4, 4), Point2d::new(4, 3), Direction::Up);

        assert_eq!(result, Some(expected));
        assert_eq!(warehouse, expected_warehouse);
    }

    #[test]
    fn test_warehouse_swap_robot_box_does_swap() {
        let input = [
            "##########",
            "#..O..O.O#",
            "#......O.#",
            "#.OO..O.O#",
            "#.OO@..O.#",
            "#O#..O...#",
            "#O..O..O.#",
            "#.OO.O.OO#",
            "#....O...#",
            "##########",
        ];

        let mut warehouse = Warehouse::from(&input);

        let expected_warehouse = Warehouse::from(&[
            "##########",
            "#..O..O.O#",
            "#......O.#",
            "#.OO..O.O#",
            "#OO@...O.#",
            "#O#..O...#",
            "#O..O..O.#",
            "#.OO.O.OO#",
            "#....O...#",
            "##########",
        ]);

        let expected = Point2d::new(3, 4);

        let result = warehouse.swap(Point2d::new(4, 4), Point2d::new(3, 4), Direction::Left);

        assert_eq!(result, Some(expected));
        assert_eq!(warehouse, expected_warehouse);
    }

    #[test]
    fn test_warehouse_swap_robot_box_does_not_swap() {
        let input = [
            "##########",
            "#..O..O.O#",
            "#......O.#",
            "#.OO..O.O#",
            "#OOO@..O.#",
            "#O#..O...#",
            "#O..O..O.#",
            "#.OO.O.OO#",
            "#....O...#",
            "##########",
        ];

        let mut warehouse = Warehouse::from(&input);

        let expected_warehouse = Warehouse::from(&[
            "##########",
            "#..O..O.O#",
            "#......O.#",
            "#.OO..O.O#",
            "#OOO@..O.#",
            "#O#..O...#",
            "#O..O..O.#",
            "#.OO.O.OO#",
            "#....O...#",
            "##########",
        ]);

        let result = warehouse.swap(Point2d::new(4, 4), Point2d::new(3, 4), Direction::Left);

        assert!(result.is_none());
        assert_eq!(warehouse, expected_warehouse);
    }

    #[test]
    fn test_warehouse_swap_robot_wall() {
        let input = [
            "##########",
            "#..O..O.O#",
            "#......O.#",
            "#.OO..O.O#",
            "#@.....O.#",
            "#O#..O...#",
            "#O..O..O.#",
            "#.OO.O.OO#",
            "#....O...#",
            "##########",
        ];

        let mut warehouse = Warehouse::from(&input);

        let expected_warehouse = Warehouse::from(&[
            "##########",
            "#..O..O.O#",
            "#......O.#",
            "#.OO..O.O#",
            "#@.....O.#",
            "#O#..O...#",
            "#O..O..O.#",
            "#.OO.O.OO#",
            "#....O...#",
            "##########",
        ]);

        let result = warehouse.swap(Point2d::new(1, 4), Point2d::new(0, 4), Direction::Left);

        assert!(result.is_none());
        assert_eq!(warehouse, expected_warehouse);
    }

    #[test]
    fn test_warehouse_move_robot() {
        let input = [
            "########", "#..O.O.#", "##@.O..#", "#...O..#", "#.#.O..#", "#...O..#", "#......#",
            "########",
        ];

        let moves: Vec<Direction> = "<^^>>>vv<v>>v<<".chars().map(Direction::from).collect();

        let mut warehouse = Warehouse::from(&input);

        let expected = Warehouse::from(&[
            "########", "#....OO#", "##.....#", "#.....O#", "#.#O@..#", "#...O..#", "#...O..#",
            "########",
        ]);

        warehouse.move_robot(&moves);

        assert_eq!(warehouse, expected);
    }

    #[test]
    fn test_warehouse_box_gps_coordinates_sum() {
        let input = [
            "########", "#..O.O.#", "##@.O..#", "#...O..#", "#.#.O..#", "#...O..#", "#......#",
            "########",
        ];

        let moves: Vec<Direction> = "<^^>>>vv<v>>v<<".chars().map(Direction::from).collect();

        let mut warehouse = Warehouse::from(&input);

        warehouse.move_robot(&moves);

        assert_eq!(warehouse.box_gps_coordinates().iter().sum::<i32>(), 2_028);
    }
}
