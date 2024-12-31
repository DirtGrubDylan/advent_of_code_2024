use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;

use crate::util::grid::{Direction, Grid};
use crate::util::point_2d::Point2d;

#[derive(Default, Debug, PartialEq, Clone, Copy)]
enum Item {
    #[default]
    Empty,
    Wall,
    Start,
    End,
}

impl From<char> for Item {
    fn from(input: char) -> Item {
        match input {
            '.' => Item::Empty,
            '#' => Item::Wall,
            'S' => Item::Start,
            'E' => Item::End,
            _ => panic!("Cannot map `'{input}'` to an `Item`."),
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let as_string = match self {
            Item::Empty => '.',
            Item::Wall => '#',
            Item::Start => 'S',
            Item::End => 'E',
        };

        write!(f, "{as_string}")
    }
}

#[derive(Debug, Eq, Clone, Copy)]
struct SearchNode {
    payload: usize,
    from_point: Point2d<i32>,
    point: Point2d<i32>,
    can_cheat: bool,
}

impl SearchNode {
    fn new(payload: usize, point: Point2d<i32>) -> Self {
        SearchNode {
            payload,
            from_point: point,
            point,
            can_cheat: true,
        }
    }

    #[must_use]
    fn move_to(self, direction: Direction) -> Self {
        SearchNode {
            payload: self.payload + 1,
            from_point: self.point,
            point: self.point + direction.as_offset(),
            can_cheat: self.can_cheat,
        }
    }

    #[must_use]
    fn move_to_without_cheat(self, direction: Direction) -> Self {
        SearchNode {
            payload: self.payload + 1,
            from_point: self.point,
            point: self.point + direction.as_offset(),
            can_cheat: false,
        }
    }

    #[must_use]
    fn rev(self) -> Reverse<Self> {
        Reverse(self)
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

#[derive(Debug, PartialEq, Clone, Copy)]
struct Cheat {
    from_point: Point2d<i32>,
    to_point: Point2d<i32>,
}

impl Cheat {
    fn new(from_point: Point2d<i32>, to_point: Point2d<i32>) -> Self {
        Cheat {
            from_point,
            to_point,
        }
    }
}

impl From<SearchNode> for Cheat {
    fn from(node: SearchNode) -> Self {
        Cheat::new(node.from_point, node.point)
    }
}

#[derive(Debug, PartialEq)]
pub struct Race {
    map: Grid<Item>,
    start: Point2d<i32>,
    end: Point2d<i32>,
}

impl Race {
    pub fn number_of_cheats_to_save(&self, time_to_save: usize) -> usize {
        let path = self.path_to_end();
        let time_to_end = path.len().saturating_sub(1);
        let target_time = time_to_end.saturating_sub(time_to_save);

        let mut result = HashSet::new();
        let mut seen = HashSet::new();
        let mut heap = BinaryHeap::from([Reverse(SearchNode::new(0, self.start))]);

        while let Some(Reverse(node)) = heap.pop() {
            let saves_target_time = match path.get(&node.point) {
                Some(original_time) => *original_time == node.payload + time_to_save,
                None => false,
            };

            let inserted = seen.insert((node.point, node.can_cheat));

            match self.map.get(node.point) {
                _ if path.contains_key(&node.point) && !node.can_cheat => {
                    seen.remove(&(node.point, node.can_cheat));

                    if saves_target_time {
                        result.insert(node.from_point);
                    }
                }
                Some(Item::Wall) if inserted && node.can_cheat && node.payload < target_time => {
                    heap.push(node.move_to_without_cheat(Direction::Up).rev());
                    heap.push(node.move_to_without_cheat(Direction::Right).rev());
                    heap.push(node.move_to_without_cheat(Direction::Down).rev());
                    heap.push(node.move_to_without_cheat(Direction::Left).rev());
                }
                Some(Item::Empty | Item::Start) if inserted && node.payload < target_time => {
                    heap.push(node.move_to(Direction::Up).rev());
                    heap.push(node.move_to(Direction::Right).rev());
                    heap.push(node.move_to(Direction::Down).rev());
                    heap.push(node.move_to(Direction::Left).rev());
                }
                _ => {}
            }
        }

        result.len()
    }

    fn time_to_end(&self) -> usize {
        self.path_to_end().len().saturating_sub(1)
    }

    fn path_to_end(&self) -> HashMap<Point2d<i32>, usize> {
        let mut path = HashMap::new();
        let mut heap = BinaryHeap::from([Reverse(SearchNode::new(0, self.start))]);

        while let Some(Reverse(node)) = heap.pop() {
            match self.map.get(node.point) {
                Some(Item::Empty | Item::Start | Item::End) if !path.contains_key(&node.point) => {
                    path.insert(node.point, node.payload);

                    heap.push(node.move_to(Direction::Up).rev());
                    heap.push(node.move_to(Direction::Right).rev());
                    heap.push(node.move_to(Direction::Down).rev());
                    heap.push(node.move_to(Direction::Left).rev());
                }
                _ => {}
            }
        }

        path
    }
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.map)
    }
}

impl<const N: usize> From<[&str; N]> for Race {
    fn from(input: [&str; N]) -> Self {
        Race::from(input.as_slice())
    }
}

impl From<&[&str]> for Race {
    fn from(input: &[&str]) -> Self {
        let input_strings: Vec<String> = input.iter().map(ToString::to_string).collect();

        Race::from(input_strings.as_slice())
    }
}

impl From<&[String]> for Race {
    fn from(input: &[String]) -> Self {
        let mut map = Grid::default();
        let mut start = Point2d::default();
        let mut end = Point2d::default();

        for (row, line) in input.iter().enumerate() {
            for (col, c) in line.char_indices() {
                let point = Point2d::new(i32::try_from(col).unwrap(), i32::try_from(row).unwrap());

                let item = Item::from(c);

                if let &Item::Start = &item {
                    start = point;
                }

                if let &Item::End = &item {
                    end = point;
                }

                map.insert(point, &item);
            }
        }

        Race { map, start, end }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_race_path_to_end() {
        let race = Race::from(["#####", "#...#", "#.#.#", "#S#.E", "#####"]);

        let expected = HashMap::from([
            (Point2d::new(1, 3), 0),
            (Point2d::new(1, 2), 1),
            (Point2d::new(1, 1), 2),
            (Point2d::new(2, 1), 3),
            (Point2d::new(3, 1), 4),
            (Point2d::new(3, 2), 5),
            (Point2d::new(3, 3), 6),
            (Point2d::new(4, 3), 7),
        ]);

        let result = race.path_to_end();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_race_time_to_end() {
        let race = Race::from([
            "###############",
            "#...#...#.....#",
            "#.#.#.#.#.###.#",
            "#S#...#.#.#...#",
            "#######.#.#.###",
            "#######.#.#...#",
            "#######.#.###.#",
            "###..E#...#...#",
            "###.#######.###",
            "#...###...#...#",
            "#.#####.#.###.#",
            "#.#...#.#.#...#",
            "#.#.#.#.#.#.###",
            "#...#...#...###",
            "###############",
        ]);

        assert_eq!(race.time_to_end(), 84);
    }

    #[test]
    fn test_race_number_of_cheats_to_save() {
        let race = Race::from([
            "###############",
            "#...#...#.....#",
            "#.#.#.#.#.###.#",
            "#S#...#.#.#...#",
            "#######.#.#.###",
            "#######.#.#...#",
            "#######.#.###.#",
            "###..E#...#...#",
            "###.#######.###",
            "#...###...#...#",
            "#.#####.#.###.#",
            "#.#...#.#.#...#",
            "#.#.#.#.#.#.###",
            "#...#...#...###",
            "###############",
        ]);

        assert_eq!(race.number_of_cheats_to_save(2), 14);
        assert_eq!(race.number_of_cheats_to_save(4), 14);
        assert_eq!(race.number_of_cheats_to_save(6), 2);
        assert_eq!(race.number_of_cheats_to_save(8), 4);
        assert_eq!(race.number_of_cheats_to_save(10), 2);
        assert_eq!(race.number_of_cheats_to_save(12), 3);
        assert_eq!(race.number_of_cheats_to_save(20), 1);
        assert_eq!(race.number_of_cheats_to_save(36), 1);
        assert_eq!(race.number_of_cheats_to_save(38), 1);
        assert_eq!(race.number_of_cheats_to_save(40), 1);
        assert_eq!(race.number_of_cheats_to_save(64), 1);
    }
}
