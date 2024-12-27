use std::collections::hash_map::{Entry, Iter, IterMut, Keys, Values};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::string::ToString;

use super::point_2d::Point2d;

#[allow(dead_code)]
pub const UP: Point2d<i32> = Point2d { x: 0, y: -1 };
#[allow(dead_code)]
pub const RIGHT: Point2d<i32> = Point2d { x: 1, y: 0 };
#[allow(dead_code)]
pub const DOWN: Point2d<i32> = Point2d { x: 0, y: 1 };
#[allow(dead_code)]
pub const LEFT: Point2d<i32> = Point2d { x: -1, y: 0 };

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    #[must_use]
    pub fn as_offset(self) -> Point2d<i32> {
        match self {
            Direction::Up => UP,
            Direction::Right => RIGHT,
            Direction::Down => DOWN,
            Direction::Left => LEFT,
        }
    }

    #[must_use]
    pub fn turn_90_degrees_clockwise(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    #[must_use]
    pub fn turn_90_degrees_counter_clockwise(self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' | 'V' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("Cannot parse {c} to Direction"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Grid<V>
where
    V: Display + Clone,
{
    data: HashMap<Point2d<i32>, V>,
    missing_data_string: String,
}

impl<V> Grid<V>
where
    V: Display + Clone,
{
    #[must_use]
    pub fn get(&self, point: Point2d<i32>) -> Option<&V> {
        self.data.get(&point)
    }

    #[must_use]
    pub fn get_from_coords(&self, x: i32, y: i32) -> Option<&V> {
        self.get(Point2d::new(x, y))
    }

    #[must_use]
    pub fn get_mut(&mut self, point: Point2d<i32>) -> Option<&mut V> {
        self.data.get_mut(&point)
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

    #[must_use]
    pub fn iter(&self) -> Iter<'_, Point2d<i32>, V> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Point2d<i32>, V> {
        self.data.iter_mut()
    }

    pub fn entry(&mut self, point: Point2d<i32>) -> Entry<'_, Point2d<i32>, V> {
        self.data.entry(point)
    }

    #[must_use]
    pub fn keys(&self) -> Keys<'_, Point2d<i32>, V> {
        self.data.keys()
    }

    #[must_use]
    pub fn values(&self) -> Values<'_, Point2d<i32>, V> {
        self.data.values()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[allow(dead_code)]
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

impl<V> Default for Grid<V>
where
    V: Display + Clone,
{
    fn default() -> Self {
        Grid {
            data: HashMap::new(),
            missing_data_string: String::from("."),
        }
    }
}

impl<V> fmt::Display for Grid<V>
where
    V: Display + Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_array = Vec::new();

        let max_point = self
            .data
            .keys()
            .max()
            .copied()
            .unwrap_or(Point2d::new(0, 0));

        for row in 0..=max_point.y {
            let mut temp_string = String::new();

            for col in 0..=max_point.x {
                temp_string += self
                    .get(Point2d::new(col, row))
                    .map(ToString::to_string)
                    .as_ref()
                    .unwrap_or(&self.missing_data_string);
            }

            string_array.push(temp_string);
        }

        write!(f, "{}", string_array.join("\n"))
    }
}

impl<V> From<HashMap<Point2d<i32>, V>> for Grid<V>
where
    V: Display + Clone,
{
    fn from(data: HashMap<Point2d<i32>, V>) -> Self {
        Grid {
            data,
            missing_data_string: String::from("."),
        }
    }
}

impl<const N: usize, V> From<[(Point2d<i32>, V); N]> for Grid<V>
where
    V: Display + Clone,
{
    fn from(data: [(Point2d<i32>, V); N]) -> Self {
        Grid {
            data: HashMap::from(data),
            missing_data_string: String::from("."),
        }
    }
}

impl<'a, V> IntoIterator for &'a Grid<V>
where
    V: Display + Clone,
{
    type Item = (&'a Point2d<i32>, &'a V);
    type IntoIter = Iter<'a, Point2d<i32>, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, V> IntoIterator for &'a mut Grid<V>
where
    V: Display + Clone,
{
    type Item = (&'a Point2d<i32>, &'a mut V);
    type IntoIter = IterMut<'a, Point2d<i32>, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
