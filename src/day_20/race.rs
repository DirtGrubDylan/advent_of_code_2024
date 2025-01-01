use std::collections::{HashMap, HashSet};
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Cheat {
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

#[derive(Debug, PartialEq)]
pub struct Race {
    map: Grid<Item>,
    start: Point2d<i32>,
    end: Point2d<i32>,
}

impl Race {
    pub fn cheats_to_save(&self, cheat_duration: usize) -> HashMap<usize, HashSet<Cheat>> {
        let path = self.path_to_end();

        let mut result = HashMap::new();

        for (current_time, point) in path.iter().enumerate() {
            for (time_to_next, next_point) in path.iter().enumerate().skip(current_time + 1) {
                let distance_between = Self::distance_between(*point, *next_point);

                let time_saved = time_to_next.saturating_sub(distance_between + current_time);
                let can_reach_by_cheating = distance_between <= cheat_duration;

                if can_reach_by_cheating && (time_saved != 0) {
                    result
                        .entry(time_saved)
                        .or_insert(HashSet::new())
                        .insert(Cheat::new(*point, *next_point));
                }
            }
        }

        result
    }

    fn path_to_end(&self) -> Vec<Point2d<i32>> {
        let mut path = Vec::new();
        let mut seen = HashSet::new();
        let mut processing = vec![self.start];

        while let Some(point) = processing.pop() {
            match self.map.get(point) {
                Some(Item::Empty | Item::Start | Item::End) if seen.insert(point) => {
                    path.push(point);

                    processing.push(point + Direction::Up.as_offset());
                    processing.push(point + Direction::Right.as_offset());
                    processing.push(point + Direction::Down.as_offset());
                    processing.push(point + Direction::Left.as_offset());
                }
                _ => {}
            }
        }

        path
    }

    fn distance_between(point_a: Point2d<i32>, point_b: Point2d<i32>) -> usize {
        let difference = point_b - point_a;

        usize::try_from(difference.x.abs() + difference.y.abs()).unwrap()
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

        let expected = vec![
            Point2d::new(1, 3),
            Point2d::new(1, 2),
            Point2d::new(1, 1),
            Point2d::new(2, 1),
            Point2d::new(3, 1),
            Point2d::new(3, 2),
            Point2d::new(3, 3),
            Point2d::new(4, 3),
        ];

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

        assert_eq!(race.path_to_end().len().saturating_sub(1), 84);
    }

    #[test]
    fn test_race_cheats_to_save_cheat_duration_2() {
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

        let result = race.cheats_to_save(2);

        assert_eq!(result.len(), 11);
        assert_eq!(result.values().map(HashSet::len).sum::<usize>(), 44);
        assert_eq!(result.get(&2).unwrap().len(), 14);
        assert_eq!(result.get(&4).unwrap().len(), 14);
        assert_eq!(result.get(&6).unwrap().len(), 2);
        assert_eq!(result.get(&8).unwrap().len(), 4);
        assert_eq!(result.get(&10).unwrap().len(), 2);
        assert_eq!(result.get(&12).unwrap().len(), 3);
        assert_eq!(result.get(&20).unwrap().len(), 1);
        assert_eq!(result.get(&36).unwrap().len(), 1);
        assert_eq!(result.get(&38).unwrap().len(), 1);
        assert_eq!(result.get(&40).unwrap().len(), 1);
        assert_eq!(result.get(&64).unwrap().len(), 1);
    }

    #[test]
    fn test_race_cheats_to_save_cheat_duration_20() {
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

        let result = race.cheats_to_save(20);

        assert_eq!(result.get(&50).unwrap().len(), 32);
        assert_eq!(result.get(&52).unwrap().len(), 31);
        assert_eq!(result.get(&54).unwrap().len(), 29);
        assert_eq!(result.get(&56).unwrap().len(), 39);
        assert_eq!(result.get(&58).unwrap().len(), 25);
        assert_eq!(result.get(&60).unwrap().len(), 23);
        assert_eq!(result.get(&62).unwrap().len(), 20);
        assert_eq!(result.get(&64).unwrap().len(), 19);
        assert_eq!(result.get(&66).unwrap().len(), 12);
        assert_eq!(result.get(&68).unwrap().len(), 14);
        assert_eq!(result.get(&70).unwrap().len(), 12);
        assert_eq!(result.get(&72).unwrap().len(), 22);
        assert_eq!(result.get(&74).unwrap().len(), 4);
        assert_eq!(result.get(&76).unwrap().len(), 3);
    }
}
