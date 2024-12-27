use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fmt;

use crate::util::grid::{Direction, Grid};
use crate::util::point_2d::Point2d;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Item {
    Empty,
    Wall,
    Start,
    End,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_value = match self {
            Item::Empty => String::from("."),
            Item::Wall => String::from("#"),
            Item::Start => String::from("S"),
            Item::End => String::from("E"),
        };

        write!(f, "{string_value}")
    }
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '.' => Item::Empty,
            '#' => Item::Wall,
            'S' => Item::Start,
            'E' => Item::End,
            _ => panic!("Cannot parse {value} into an Item!"),
        }
    }
}

#[derive(Debug, Eq, Copy, Clone)]
struct SearchNode {
    score: u32,
    location: Point2d<i32>,
    facing: Direction,
}

impl SearchNode {
    fn new(score: u32, location: Point2d<i32>, facing: Direction) -> Self {
        SearchNode {
            score,
            location,
            facing,
        }
    }

    #[must_use]
    fn move_forward(self) -> Self {
        SearchNode::new(
            self.score + 1,
            self.location + self.facing.as_offset(),
            self.facing,
        )
    }

    #[must_use]
    fn turn_90_degrees_clockwise(self) -> Self {
        SearchNode::new(
            self.score + 1_000,
            self.location,
            self.facing.turn_90_degrees_clockwise(),
        )
    }

    #[must_use]
    fn turn_90_degrees_counter_clockwise(self) -> Self {
        SearchNode::new(
            self.score + 1_000,
            self.location,
            self.facing.turn_90_degrees_counter_clockwise(),
        )
    }

    fn location_facing(self) -> (Point2d<i32>, Direction) {
        (self.location, self.facing)
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

#[derive(Debug, PartialEq)]
pub struct Maze {
    map: Grid<Item>,
    starting_location: Point2d<i32>,
    ending_location: Point2d<i32>,
}

impl Maze {
    pub fn lowest_path_score(&self) -> u32 {
        self.path_backtrack()
            .get(&self.ending_location)
            .unwrap()
            .iter()
            .map(|node| node.score + 1)
            .min()
            .unwrap()
    }

    pub fn number_of_optimal_sitting_spots(&self) -> usize {
        let mut spots = HashSet::new();

        let backtrack = self.path_backtrack();

        let end_score = backtrack
            .get(&self.ending_location)
            .unwrap()
            .iter()
            .map(|node| node.score + 1)
            .min()
            .unwrap();

        let end_node = SearchNode::new(end_score, self.ending_location, Direction::Right);

        let mut backtrack_nodes = vec![&end_node];

        while let Some(current_node) = backtrack_nodes.pop() {
            spots.insert(current_node.location);

            for node in backtrack.get(&current_node.location).unwrap() {
                let score_diff = current_node.score.saturating_sub(node.score);

                if (score_diff == 1_001) || (score_diff == 1) {
                    backtrack_nodes.push(node);
                }
            }
        }

        spots.len()
    }

    fn path_backtrack(&self) -> HashMap<Point2d<i32>, Vec<SearchNode>> {
        let mut backtrack: HashMap<Point2d<i32>, Vec<SearchNode>> = HashMap::new();
        let mut seen_locations = HashSet::new();
        let mut heap = BinaryHeap::from([Reverse(SearchNode::new(
            0,
            self.starting_location,
            Direction::Right,
        ))]);

        while !heap.is_empty() {
            let Reverse(search_node) = heap.pop().unwrap();

            let location_facing = search_node.location_facing();

            let search_node_forward = search_node.move_forward();
            let search_node_clockwise = search_node.turn_90_degrees_clockwise();
            let search_node_counter_clockwise = search_node.turn_90_degrees_counter_clockwise();

            match self.map.get(search_node.location) {
                Some(Item::Empty | Item::Start) if seen_locations.insert(location_facing) => {
                    backtrack
                        .entry(search_node_forward.location)
                        .or_default()
                        .push(search_node);

                    heap.push(Reverse(search_node_forward));
                    heap.push(Reverse(search_node_clockwise));
                    heap.push(Reverse(search_node_counter_clockwise));
                }
                _ => {}
            }
        }

        backtrack
    }
}

impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.map)
    }
}

impl<const N: usize> From<&[&str; N]> for Maze {
    fn from(input: &[&str; N]) -> Self {
        Maze::from(input.as_slice())
    }
}

impl From<&[&str]> for Maze {
    fn from(input: &[&str]) -> Self {
        let input_strings: Vec<String> = input.iter().map(ToString::to_string).collect();

        Maze::from(input_strings.as_slice())
    }
}

impl From<&[String]> for Maze {
    fn from(input: &[String]) -> Self {
        let mut map = Grid::default();
        let mut starting_location = Point2d::default();
        let mut ending_location = Point2d::default();

        for (row, line) in input.iter().enumerate() {
            for (col, c) in line.char_indices() {
                let point = Point2d::new(i32::try_from(col).unwrap(), i32::try_from(row).unwrap());

                let item = Item::from(c);

                if let &Item::Start = &item {
                    starting_location = point;
                }

                if let &Item::End = &item {
                    ending_location = point;
                }

                map.insert(point, &item);
            }
        }

        Maze {
            map,
            starting_location,
            ending_location,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_from_str_array() {
        let expected_grid = Grid::from([
            (Point2d::new(0, 0), Item::Wall),
            (Point2d::new(1, 0), Item::Wall),
            (Point2d::new(2, 0), Item::Wall),
            (Point2d::new(0, 1), Item::Wall),
            (Point2d::new(1, 1), Item::End),
            (Point2d::new(2, 1), Item::Wall),
            (Point2d::new(0, 2), Item::Wall),
            (Point2d::new(1, 2), Item::Start),
            (Point2d::new(2, 2), Item::Wall),
            (Point2d::new(0, 3), Item::Wall),
            (Point2d::new(1, 3), Item::Wall),
            (Point2d::new(2, 3), Item::Empty),
        ]);

        let expected = Maze {
            map: expected_grid,
            starting_location: Point2d::new(1, 2),
            ending_location: Point2d::new(1, 1),
        };

        let result = Maze::from(&["###", "#E#", "#S#", "##."]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_maze_lowest_path_score() {
        let maze = Maze::from(&[
            "###############",
            "#.......#....E#",
            "#.#.###.#.###.#",
            "#.....#.#...#.#",
            "#.###.#####.#.#",
            "#.#.#.......#.#",
            "#.#.#####.###.#",
            "#...........#.#",
            "###.#.#####.#.#",
            "#...#.....#.#.#",
            "#.#.#.###.#.#.#",
            "#.....#...#.#.#",
            "#.###.#.#.#.#.#",
            "#S..#.....#...#",
            "###############",
        ]);

        assert_eq!(maze.lowest_path_score(), 7_036);
    }

    #[test]
    fn test_maze_lowest_path_score_complicated() {
        let maze = Maze::from(&[
            "####################",
            "####.##########.#.##",
            "##..............#..#",
            "##.#.##.#######.####",
            "#..#.##.....#...#..#",
            "##.#.######.#.#.#.##",
            "##....#...........E#",
            "##.####.#####.###.##",
            "#.........#.#.#...##",
            "##.####.#.#.#.#.####",
            "#S.#################",
            "####################",
        ]);

        assert_eq!(maze.lowest_path_score(), 4_021);
    }

    #[test]
    fn test_maze_number_of_optimal_sitting_spots() {
        let maze = Maze::from(&[
            "###############",
            "#.......#....E#",
            "#.#.###.#.###.#",
            "#.....#.#...#.#",
            "#.###.#####.#.#",
            "#.#.#.......#.#",
            "#.#.#####.###.#",
            "#...........#.#",
            "###.#.#####.#.#",
            "#...#.....#.#.#",
            "#.#.#.###.#.#.#",
            "#.....#...#.#.#",
            "#.###.#.#.#.#.#",
            "#S..#.....#...#",
            "###############",
        ]);

        assert_eq!(maze.number_of_optimal_sitting_spots(), 45);
    }

    #[test]
    fn test_maze_number_of_optimal_sitting_spots_complicated() {
        let maze = Maze::from(&[
            "####################",
            "####.##########.#.##",
            "##..............#..#",
            "##.#.##.#######.####",
            "#..#.##.....#...#..#",
            "##.#.######.#.#.#.##",
            "##....#...........E#",
            "##.####.#####.###.##",
            "#.........#.#.#...##",
            "##.####.#.#.#.#.####",
            "#S.#################",
            "####################",
        ]);

        assert_eq!(maze.number_of_optimal_sitting_spots(), 22);
    }

    #[test]
    fn test_search_node_ordering() {
        let node_1 = SearchNode::new(5, Point2d::new(0, 0), Direction::Up);
        let node_2 = SearchNode::new(4, Point2d::new(4, 4), Direction::Up);
        let node_3 = SearchNode::new(5, Point2d::new(4, 4), Direction::Down);
        let node_4 = SearchNode::new(6, Point2d::new(4, 4), Direction::Down);

        assert!(node_1 != node_2);
        assert!(node_1 > node_2);
        assert!(node_1 >= node_2);
        assert!(node_1 >= node_3);
        assert!(node_1 == node_3);
        assert!(node_1 <= node_3);
        assert!(node_1 <= node_4);
        assert!(node_1 < node_4);
        assert!(node_1 != node_4);
    }
}
