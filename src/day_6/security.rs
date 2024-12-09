use crate::util::point_2d::Point2d;
use std::collections::{HashMap, HashSet};

const UP: Point2d<i32> = Point2d { x: 0, y: -1 };
const RIGHT: Point2d<i32> = Point2d { x: 1, y: 0 };
const DOWN: Point2d<i32> = Point2d { x: 0, y: 1 };
const LEFT: Point2d<i32> = Point2d { x: -1, y: 0 };

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn as_offset(&self) -> Point2d<i32> {
        match self {
            &Direction::Up => UP,
            &Direction::Right => RIGHT,
            &Direction::Down => DOWN,
            &Direction::Left => LEFT,
        }
    }

    fn turn_90_degrees_clockwise(&self) -> Self {
        match self {
            &Direction::Up => Direction::Right,
            &Direction::Right => Direction::Down,
            &Direction::Down => Direction::Left,
            &Direction::Left => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Guard {
    current_position: Point2d<i32>,
    direction_facing: Direction,
}

impl Guard {
    fn new(col: i32, row: i32) -> Self {
        let current_position = Point2d::new(col, row);
        let direction_facing = Direction::Up;

        Guard {
            current_position,
            direction_facing,
        }
    }

    pub fn number_of_unique_positions_to_walk(&self, map: &PatrolMap) -> usize {
        self.unique_positions_to_walk(map).len()
    }

    pub fn unique_positions_to_walk(&self, map: &PatrolMap) -> HashSet<Point2d<i32>> {
        let mut clone = self.clone();

        let mut positions_visited = HashSet::from([clone.current_position]);

        while let Some(obstacle_point) = map.first_obstacle_facing(&clone) {
            let point_before_obstacle = clone.point_just_before(obstacle_point);

            positions_visited.extend(clone.positions_needed_to_walk_to(point_before_obstacle));

            clone.current_position = point_before_obstacle;
            clone.direction_facing = clone.direction_facing.turn_90_degrees_clockwise();
        }

        let nearest_edge = map.nearest_map_edge_point_facing(&clone);

        positions_visited.extend(clone.positions_needed_to_walk_to(nearest_edge));

        positions_visited
    }

    fn positions_needed_to_walk_to(&self, dest: Point2d<i32>) -> HashSet<Point2d<i32>> {
        let mut result = HashSet::new();
        let mut current = self.current_position;

        while current != dest {
            current = current + self.direction_facing.as_offset();

            result.insert(current);
        }

        result
    }

    fn point_just_before(&self, dest: Point2d<i32>) -> Point2d<i32> {
        let mut previous = self.current_position;
        let mut current = self.current_position;

        match self.direction_facing {
            Direction::Up if (dest.x != current.x) || (current.y < dest.y) => {
                panic!(
                    "Destination {:?} cannot be reached going up from {:?}",
                    dest, current
                )
            }
            Direction::Right if (dest.y != current.y) || (dest.x < current.x) => {
                panic!(
                    "Destination {:?} cannot be reached going right from {:?}",
                    dest, current
                )
            }
            Direction::Down if (dest.x != current.x) || (dest.y < current.y) => {
                panic!(
                    "Destination {:?} cannot be reached going down from {:?}",
                    dest, current
                )
            }
            Direction::Left if (dest.y != current.y) || (current.x < dest.x) => {
                panic!(
                    "Destination {:?} cannot be reached going left from {:?}",
                    dest, current
                )
            }
            _ => {}
        }

        while current != dest {
            previous = current;
            current = current + self.direction_facing.as_offset();
        }

        previous
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum MapObject {
    Empty,
    Obstacle,
    GuardStartingPosition,
}

impl From<char> for MapObject {
    fn from(input: char) -> Self {
        match input {
            '.' => MapObject::Empty,
            '#' => MapObject::Obstacle,
            '^' => MapObject::GuardStartingPosition,
            _ => panic!("{input} is not a valid object!"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct PatrolMap {
    data: HashMap<Point2d<i32>, MapObject>,
    raw_data: Vec<String>,
}

impl PatrolMap {
    pub fn new(input: &[String]) -> Self {
        let mut data = HashMap::new();

        for (row, line) in input.iter().enumerate() {
            for (col, item) in line.char_indices() {
                let point = Point2d::new(i32::try_from(col).unwrap(), i32::try_from(row).unwrap());

                data.insert(point, item.into());
            }
        }

        PatrolMap {
            data,
            raw_data: input.to_vec(),
        }
    }

    pub fn guard(&self) -> Guard {
        self.data
            .iter()
            .filter(|(_, object)| **object == MapObject::GuardStartingPosition)
            .map(|(point, _)| Guard::new(point.x, point.y))
            .nth(0)
            .unwrap()
    }

    pub fn print_with_walked_positions(&self, walked_positions: &HashSet<Point2d<i32>>) {
        let mut data_to_print = self.raw_data.clone();

        for point in walked_positions {
            if let Some(line) = data_to_print.get_mut(usize::try_from(point.y).unwrap()) {
                let col = usize::try_from(point.x).unwrap();

                line.replace_range(col..=col, "X");
            }
        }

        println!("***************MAP***************");

        for row in data_to_print {
            println!("\t{row}");
        }

        println!("*********************************\n");
    }

    pub fn first_obstacle_facing(&self, guard: &Guard) -> Option<Point2d<i32>> {
        self.get_first_obstacle(guard.current_position, guard.direction_facing)
    }

    pub fn nearest_map_edge_point_facing(&self, guard: &Guard) -> Point2d<i32> {
        let mut previous_point = guard.current_position;
        let mut current_point = guard.current_position;

        while let Some(object) = self.data.get(&current_point) {
            previous_point = current_point;
            current_point = current_point + guard.direction_facing.as_offset();
        }

        previous_point
    }

    pub fn get_first_obstacle(
        &self,
        start: Point2d<i32>,
        direction: Direction,
    ) -> Option<Point2d<i32>> {
        self.traverse_find(
            start,
            |point| point + direction.as_offset(),
            |object| *object == MapObject::Obstacle,
        )
    }

    fn traverse_find<T, P>(
        &self,
        start: Point2d<i32>,
        transform: T,
        predicate: P,
    ) -> Option<Point2d<i32>>
    where
        T: Fn(Point2d<i32>) -> Point2d<i32>,
        P: Fn(&MapObject) -> bool,
    {
        let mut result = None;

        let mut current_point = start;

        while let Some(object) = self.data.get(&current_point) {
            if predicate(object) {
                result = Some(current_point);

                break;
            }

            current_point = transform(current_point);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_number_of_unique_positions_to_walk() {
        let input = [
            String::from("....#....."),
            String::from(".........#"),
            String::from(".........."),
            String::from("..#......."),
            String::from(".......#.."),
            String::from(".........."),
            String::from(".#..^....."),
            String::from("........#."),
            String::from("#........."),
            String::from("......#..."),
        ];

        let map = PatrolMap::new(&input);
        let guard = Guard::new(4, 6);

        assert_eq!(guard.number_of_unique_positions_to_walk(&map), 41);
    }

    #[test]
    fn test_guard_positions_needed_to_walk_to() {
        let guard = Guard::new(4, 6);

        let expected = HashSet::from([
            Point2d::new(4, 5),
            Point2d::new(4, 4),
            Point2d::new(4, 3),
            Point2d::new(4, 2),
            Point2d::new(4, 1),
        ]);

        let result = guard.positions_needed_to_walk_to(Point2d::new(4, 1));

        assert_eq!(result, expected);
    }

    #[test]
    fn test_patrol_map_traverse_find() {
        let input = [
            String::from("....#....."),
            String::from(".........#"),
            String::from(".........."),
            String::from("..#......."),
            String::from(".......#.."),
            String::from(".........."),
            String::from(".#..^....."),
            String::from("........#."),
            String::from("#........."),
            String::from("......#..."),
        ];

        let map = PatrolMap::new(&input);
        let start_point = Point2d::new(4, 6);

        let expected_some = Some(Point2d::new(4, 0));
        let expected_none = None;

        let result_some = map.traverse_find(
            start_point,
            |point| point + UP,
            |object| *object == MapObject::Obstacle,
        );
        let result_none = map.traverse_find(
            start_point,
            |point| point + RIGHT,
            |object| *object == MapObject::Obstacle,
        );

        assert_eq!(result_some, expected_some);
        assert_eq!(result_none, expected_none);
    }

    #[test]
    fn test_patrol_map_get_first_obstacle() {
        let input = [
            String::from("....#....."),
            String::from(".........#"),
            String::from(".........."),
            String::from("..#......."),
            String::from(".......#.."),
            String::from(".........."),
            String::from(".#..^....."),
            String::from("........#."),
            String::from("#........."),
            String::from("......#..."),
        ];

        let map = PatrolMap::new(&input);
        let start_point = Point2d::new(4, 6);

        let expected_up = Some(Point2d::new(4, 0));
        let expected_right = None;
        let expected_down = None;
        let expected_left = Some(Point2d::new(1, 6));

        let result_up = map.get_first_obstacle(start_point, Direction::Up);
        let result_right = map.get_first_obstacle(start_point, Direction::Right);
        let result_down = map.get_first_obstacle(start_point, Direction::Down);
        let result_left = map.get_first_obstacle(start_point, Direction::Left);

        assert_eq!(result_up, expected_up);
        assert_eq!(result_right, expected_right);
        assert_eq!(result_down, expected_down);
        assert_eq!(result_left, expected_left);
    }
}
