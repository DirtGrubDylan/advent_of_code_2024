use std::collections::HashMap;

use super::point_2d::Point2d;

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

#[derive(Debug, PartialEq, Clone)]
pub struct Grid<V>
where
    V: Clone,
{
    data: HashMap<Point2d<i32>, V>,
}

impl<V> Grid<V>
where
    V: Clone,
{
    pub fn new() -> Self {
        Grid {
            data: HashMap::new(),
        }
    }

    pub fn get(&self, point: Point2d<i32>) -> Option<&V> {
        self.data.get(&point)
    }

    pub fn get_from_coords(&self, x: i32, y: i32) -> Option<&V> {
        self.get(Point2d::new(x, y))
    }

    pub fn insert(&mut self, point: Point2d<i32>, item: &V) -> Option<V> {
        self.data.insert(point, item.clone())
    }

    pub fn insert_with_coords(&mut self, x: i32, y: i32, item: &V) -> Option<V> {
        self.insert(Point2d::new(x, y), item)
    }

    pub fn remove(&mut self, point: Point2d<i32>) -> Option<V> {
        self.data.remove(&point)
    }

    pub fn remove_with_coords(&mut self, x: i32, y: i32) -> Option<V> {
        self.remove(Point2d::new(x, y))
    }

    fn traverse_find<T, P>(
        &self,
        start: Point2d<i32>,
        transform: T,
        predicate: P,
    ) -> Option<Point2d<i32>>
    where
        T: Fn(Point2d<i32>) -> Point2d<i32>,
        P: Fn(&V) -> bool,
    {
        let mut result = None;

        let mut current_point = start;

        while let Some(object) = self.get(current_point) {
            if predicate(object) {
                result = Some(current_point);

                break;
            }

            current_point = transform(current_point);
        }

        result
    }
}
