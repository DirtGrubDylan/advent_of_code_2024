use std::fmt;

use crate::util::grid::{Direction, Grid};
use crate::util::point_2d::Point2d;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Item {
    Empty,
    Wall,
    Box,
    Robot,
    WideBoxLeft,
    WideBoxRight,
}

impl Item {}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_value = match self {
            Item::Empty => String::from("."),
            Item::Wall => String::from("#"),
            Item::Box => String::from("O"),
            Item::Robot => String::from("@"),
            Item::WideBoxLeft => String::from("["),
            Item::WideBoxRight => String::from("]"),
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
            '@' => Item::Robot,
            '[' => Item::WideBoxLeft,
            ']' => Item::WideBoxRight,
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
            .filter(|(_, item)| **item == Item::Box || **item == Item::WideBoxLeft)
            .map(|(point, _)| point.x + 100 * point.y)
            .collect()
    }

    pub fn move_robot(&mut self, moves: &[Direction]) {
        for direction in moves {
            self.move_point(self.robot_location, *direction);
        }
    }

    fn move_point(&mut self, point: Point2d<i32>, direction: Direction) -> Option<Point2d<i32>> {
        let next_point = point + direction.as_offset();

        let item = self.map.get(point).copied().unwrap();
        let next_item = self.map.get(next_point).copied().unwrap();

        let result = match (next_item, direction) {
            (Item::Empty, _) => {
                self.map.insert(next_point, &item);

                self.map.insert(point, &Item::Empty);

                Some(next_point)
            }
            (Item::Box, _) => self
                .move_point(next_point, direction)
                .and_then(|_| self.move_point(point, direction)),
            (Item::WideBoxRight | Item::WideBoxLeft, Direction::Left | Direction::Right) => self
                .move_point(next_point, direction)
                .and_then(|_| self.move_point(point, direction)),
            (Item::WideBoxRight, _) if self.can_move_point(point, direction) => {
                let wide_box_left_point = next_point + Direction::Left.as_offset();

                self.move_point(next_point, direction);
                self.move_point(wide_box_left_point, direction);

                self.move_point(point, direction)
            }
            (Item::WideBoxLeft, _) if self.can_move_point(point, direction) => {
                let wide_box_right_point = next_point + Direction::Right.as_offset();

                self.move_point(next_point, direction);
                self.move_point(wide_box_right_point, direction);

                self.move_point(point, direction)
            }
            _ => None,
        };

        if let (Item::Robot, Some(point)) = (item, result) {
            self.robot_location = point;
        }

        result
    }

    fn can_move_point(&self, point: Point2d<i32>, direction: Direction) -> bool {
        let next_point = point + direction.as_offset();

        let result = match (self.map.get(next_point), direction) {
            (Some(Item::Empty), _) => true,
            (Some(Item::Box), _) => self.can_move_point(next_point, direction),
            (Some(Item::WideBoxRight), Direction::Left)
            | (Some(Item::WideBoxLeft), Direction::Right) => {
                self.can_move_point(next_point + direction.as_offset(), direction)
            }
            (Some(Item::WideBoxRight), _) => {
                let wide_box_left_point = next_point + Direction::Left.as_offset();

                self.can_move_point(next_point, direction)
                    && self.can_move_point(wide_box_left_point, direction)
            }
            (Some(Item::WideBoxLeft), _) => {
                let wide_box_right_point = next_point + Direction::Right.as_offset();

                self.can_move_point(next_point, direction)
                    && self.can_move_point(wide_box_right_point, direction)
            }
            _ => false,
        };

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

                if let &Item::Robot = &item {
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
        let expected_grid = Grid::from([
            (Point2d::new(0, 0), Item::Wall),
            (Point2d::new(1, 0), Item::Wall),
            (Point2d::new(2, 0), Item::Wall),
            (Point2d::new(0, 1), Item::Wall),
            (Point2d::new(1, 1), Item::Box),
            (Point2d::new(2, 1), Item::Wall),
            (Point2d::new(0, 2), Item::Wall),
            (Point2d::new(1, 2), Item::Robot),
            (Point2d::new(2, 2), Item::Wall),
            (Point2d::new(0, 3), Item::Wall),
            (Point2d::new(1, 3), Item::Wall),
            (Point2d::new(2, 3), Item::Wall),
        ]);

        let expected = Warehouse {
            map: expected_grid,
            robot_location: Point2d::new(1, 2),
        };

        let result = Warehouse::from(&["###", "#O#", "#@#", "###"]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_warehouse_move_point_robot_to_empty() {
        let mut warehouse = Warehouse::from(&[
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
        ]);

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

        let result = warehouse.move_point(Point2d::new(4, 4), Direction::Up);

        assert_eq!(result, Some(expected));
        assert_eq!(warehouse, expected_warehouse);
    }

    #[test]
    fn test_warehouse_move_point_robot_to_box_success() {
        let mut warehouse = Warehouse::from(&[
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
        ]);

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

        let result = warehouse.move_point(Point2d::new(4, 4), Direction::Left);

        assert_eq!(result, Some(expected));
        assert_eq!(warehouse, expected_warehouse);
    }

    #[test]
    fn test_warehouse_move_point_robot_to_box_failure() {
        let mut warehouse = Warehouse::from(&[
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

        let result = warehouse.move_point(Point2d::new(4, 4), Direction::Left);

        assert!(result.is_none());
        assert_eq!(warehouse, expected_warehouse);
    }

    #[test]
    fn test_warehouse_move_point_robot_to_wall() {
        let mut warehouse = Warehouse::from(&[
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

        let result = warehouse.move_point(Point2d::new(1, 4), Direction::Left);

        assert!(result.is_none());
        assert_eq!(warehouse, expected_warehouse);
    }

    #[test]
    fn test_warehouse_can_move_point_robot_to_empty_true() {
        let warehouse = Warehouse::from(&[
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
        ]);

        assert!(warehouse.can_move_point(Point2d::new(4, 4), Direction::Up));
    }

    #[test]
    fn test_warehouse_can_move_point_robot_to_box_true() {
        let warehouse = Warehouse::from(&[
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
        ]);

        assert!(warehouse.can_move_point(Point2d::new(4, 4), Direction::Left));
    }

    #[test]
    fn test_warehouse_can_move_point_robot_to_box_false() {
        let warehouse = Warehouse::from(&[
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

        assert!(!warehouse.can_move_point(Point2d::new(4, 4), Direction::Left));
    }

    #[test]
    fn test_warehouse_can_move_point_robot_to_wall_false() {
        let warehouse = Warehouse::from(&[
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

        assert!(!warehouse.can_move_point(Point2d::new(1, 4), Direction::Left));
    }

    #[test]
    fn test_warehouse_move_robot() {
        let moves: Vec<Direction> = "<^^>>>vv<v>>v<<".chars().map(Direction::from).collect();

        let mut warehouse = Warehouse::from(&[
            "########", "#..O.O.#", "##@.O..#", "#...O..#", "#.#.O..#", "#...O..#", "#......#",
            "########",
        ]);

        let expected = Warehouse::from(&[
            "########", "#....OO#", "##.....#", "#.....O#", "#.#O@..#", "#...O..#", "#...O..#",
            "########",
        ]);

        warehouse.move_robot(&moves);

        assert_eq!(warehouse, expected);
    }

    #[test]
    fn test_warehouse_box_gps_coordinates_sum() {
        let moves: Vec<Direction> = "<^^>>>vv<v>>v<<".chars().map(Direction::from).collect();

        let mut warehouse = Warehouse::from(&[
            "########", "#..O.O.#", "##@.O..#", "#...O..#", "#.#.O..#", "#...O..#", "#......#",
            "########",
        ]);

        warehouse.move_robot(&moves);

        assert_eq!(warehouse.box_gps_coordinates().iter().sum::<i32>(), 2_028);
    }

    #[test]
    fn test_warehouse_move_point_up_robot_to_wide_box_success() {
        let mut warehouse = Warehouse::from(&[
            "##############",
            "##......##..##",
            "##..........##",
            "##...[][]...##",
            "##....[]....##",
            "##.....@....##",
            "##############",
        ]);

        let expected_warehouse = Warehouse::from(&[
            "##############",
            "##......##..##",
            "##...[][]...##",
            "##....[]....##",
            "##.....@....##",
            "##..........##",
            "##############",
        ]);

        let expected = Point2d::new(7, 4);

        let result = warehouse.move_point(Point2d::new(7, 5), Direction::Up);

        assert_eq!(result, Some(expected));
        assert_eq!(warehouse, expected_warehouse);
    }

    #[test]
    fn test_warehouse_move_point_left_robot_to_wide_box_success() {
        let mut warehouse = Warehouse::from(&[
            "##############",
            "##......##..##",
            "##..........##",
            "##....[][]@.##",
            "##....[]....##",
            "##..........##",
            "##############",
        ]);

        let expected_warehouse = Warehouse::from(&[
            "##############",
            "##......##..##",
            "##..........##",
            "##...[][]@..##",
            "##....[]....##",
            "##..........##",
            "##############",
        ]);

        let expected = Point2d::new(9, 3);

        let result = warehouse.move_point(Point2d::new(10, 3), Direction::Left);

        assert_eq!(result, Some(expected));
        assert_eq!(warehouse, expected_warehouse);
    }

    #[test]
    fn test_warehouse_move_point_up_robot_to_wide_box_failure() {
        let mut warehouse = Warehouse::from(&[
            "##############",
            "##......##..##",
            "##...[][]...##",
            "##....[]....##",
            "##.....@....##",
            "##..........##",
            "##############",
        ]);

        let expected_warehouse = Warehouse::from(&[
            "##############",
            "##......##..##",
            "##...[][]...##",
            "##....[]....##",
            "##.....@....##",
            "##..........##",
            "##############",
        ]);

        let result = warehouse.move_point(Point2d::new(7, 5), Direction::Up);

        assert_eq!(result, None);
        assert_eq!(warehouse, expected_warehouse);
    }

    #[test]
    fn test_warehouse_can_move_point_up_robot_to_wide_box_true() {
        let warehouse = Warehouse::from(&[
            "##############",
            "##......##..##",
            "##..........##",
            "##...[][]...##",
            "##....[]....##",
            "##.....@....##",
            "##############",
        ]);

        assert!(warehouse.can_move_point(Point2d::new(7, 5), Direction::Up));
    }

    #[test]
    fn test_warehouse_can_move_point_left_robot_to_wide_box_true() {
        let warehouse = Warehouse::from(&[
            "##############",
            "##......##..##",
            "##..........##",
            "##....[][]@.##",
            "##....[]....##",
            "##..........##",
            "##############",
        ]);

        assert!(warehouse.can_move_point(Point2d::new(10, 3), Direction::Left));
    }

    #[test]
    fn test_warehouse_can_move_point_up_robot_to_wide_box_false() {
        let warehouse = Warehouse::from(&[
            "##############",
            "##......##..##",
            "##...[][]...##",
            "##....[]....##",
            "##.....@....##",
            "##..........##",
            "##############",
        ]);

        assert!(!warehouse.can_move_point(Point2d::new(7, 5), Direction::Up));
    }

    #[test]
    fn test_warehouse_move_robot_wide_boxes() {
        let moves: Vec<Direction> = "<vv<<^^<<^^".chars().map(Direction::from).collect();

        let mut warehouse = Warehouse::from(&[
            "##############",
            "##......##..##",
            "##..........##",
            "##....[][]@.##",
            "##....[]....##",
            "##..........##",
            "##############",
        ]);

        let expected_warehouse = Warehouse::from(&[
            "##############",
            "##...[].##..##",
            "##...@.[]...##",
            "##....[]....##",
            "##..........##",
            "##..........##",
            "##############",
        ]);

        warehouse.move_robot(&moves);

        assert_eq!(warehouse, expected_warehouse);
    }
}
