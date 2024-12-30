use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet};
use std::fmt;
use std::str::FromStr;

use crate::util::grid::{Direction, Grid};
use crate::util::point_2d::Point2d;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
enum Data {
    #[default]
    Empty,
    Corrupted,
}

impl From<char> for Data {
    fn from(input: char) -> Data {
        match input {
            '.' => Data::Empty,
            '#' => Data::Corrupted,
            _ => panic!("Cannot map `'{input}'` to a `Data` object."),
        }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_string = match self {
            Data::Empty => '.',
            Data::Corrupted => '#',
        };

        write!(f, "{as_string}")
    }
}

#[derive(Debug, Eq, Clone, Copy)]
struct SearchNode {
    payload: usize,
    point: Point2d<i32>,
}

impl SearchNode {
    fn new(payload: usize, point: Point2d<i32>) -> Self {
        SearchNode { payload, point }
    }

    #[must_use]
    fn move_to(self, direction: Direction) -> Self {
        SearchNode {
            payload: self.payload + 1,
            point: self.point + direction.as_offset(),
        }
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.payload.cmp(&other.payload)
    }
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.payload == other.payload
    }
}

#[derive(Debug, PartialEq)]
struct Computer {
    memory: Grid<Data>,
    falling_bytes: Vec<FallingByte>,
}

impl Computer {
    fn new(col_size: usize, row_size: usize, falling_bytes_input: &[String]) -> Computer {
        let mut memory = Grid::default();

        for row in 0..row_size {
            for col in 0..col_size {
                memory.insert_with_coords(
                    col.try_into().unwrap(),
                    row.try_into().unwrap(),
                    &Data::default(),
                );
            }
        }

        let falling_bytes = falling_bytes_input
            .iter()
            .map(|line| line.parse().unwrap())
            .collect();

        Computer {
            memory,
            falling_bytes,
        }
    }

    fn shortest_path_after(&self, seconds: usize) -> HashSet<Point2d<i32>> {
        let mut path = HashSet::new();
        let mut memory = self.memory.clone();
        let ending = self
            .memory
            .keys()
            .max()
            .cloned()
            .unwrap_or(Point2d::new(0, 0));

        let mut seconds_so_far = 0;

        for byte in &self.falling_bytes {
            if seconds_so_far == seconds {
                break;
            }

            memory.insert(byte.destination, &Data::Corrupted);

            seconds_so_far += 1;
        }

        path
    }
}

#[derive(Debug, PartialEq)]
struct FallingByte {
    destination: Point2d<i32>,
}

impl FallingByte {
    fn new(x: i32, y: i32) -> Self {
        FallingByte {
            destination: Point2d::new(x, y),
        }
    }
}

#[derive(Debug, PartialEq)]
struct FallingByteParseError;

impl FromStr for FallingByte {
    type Err = FallingByteParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input
            .split_once(',')
            .map(|(col_str, row_str)| {
                Ok(FallingByte::new(
                    col_str.parse().map_err(|_| FallingByteParseError)?,
                    row_str.parse().map_err(|_| FallingByteParseError)?,
                ))
            })
            .ok_or(FallingByteParseError)?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallingbyte_from_str() {
        let expected = FallingByte {
            destination: Point2d::new(18, 9),
        };

        let result = "18,9".parse();

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_computer_shortest_path_to_exit() {
        let falling_byte_input = [
            String::from("5,4"),
            String::from("4,2"),
            String::from("4,5"),
            String::from("3,0"),
            String::from("2,1"),
            String::from("6,3"),
            String::from("2,4"),
            String::from("1,5"),
            String::from("0,6"),
            String::from("3,3"),
            String::from("2,6"),
            String::from("5,1"),
            String::from("1,2"),
            String::from("5,5"),
            String::from("2,5"),
            String::from("6,5"),
            String::from("1,4"),
            String::from("0,4"),
            String::from("6,4"),
            String::from("1,1"),
            String::from("6,1"),
            String::from("1,0"),
            String::from("0,5"),
            String::from("1,6"),
            String::from("2,0"),
        ];

        let computer = Computer::new(7, 7, &falling_byte_input);

        assert_eq!(computer.shortest_path().len(), 22);
    }
}
