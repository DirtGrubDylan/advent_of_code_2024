use crate::util::point_2d::Point2d;
use std::collections::HashMap;

const UP: Point2d<i32> = Point2d { x: 0, y: -1 };
const RIGHT: Point2d<i32> = Point2d { x: 1, y: 0 };
const DOWN: Point2d<i32> = Point2d { x: 0, y: 1 };
const LEFT: Point2d<i32> = Point2d { x: -1, y: 0 };

#[derive(Debug, PartialEq)]
pub struct WordSearch {
    data: HashMap<Point2d<i32>, char>,
}

impl WordSearch {
    pub fn new(input: &[String]) -> Self {
        let mut data = HashMap::new();

        for (row, line) in input.iter().enumerate() {
            for (col, item) in line.char_indices() {
                data.insert(
                    Point2d::new(i32::try_from(col).unwrap(), i32::try_from(row).unwrap()),
                    item,
                );
            }
        }

        WordSearch { data }
    }

    pub fn find_count(&self, word: &str) -> usize {
        let start_char = word.chars().nth(0).unwrap();

        self.data
            .iter()
            .filter(|(_, &item)| item == start_char)
            .map(|(&point, _)| self.find_count_around(point, word))
            .sum()
    }

    pub fn find_cross_count(&self, word: &str) -> usize {
        let middle_char = word.chars().nth(word.len() / 2).unwrap();

        self.data
            .iter()
            .filter(|(_, &item)| item == middle_char)
            .filter(|(&point, _)| self.has_cross_around(point, word))
            .count()
    }

    fn find_count_around(&self, point: Point2d<i32>, word: &str) -> usize {
        usize::from(self.word_is_up(point, word))
            + usize::from(self.word_is_right(point, word))
            + usize::from(self.word_is_down(point, word))
            + usize::from(self.word_is_left(point, word))
            + usize::from(self.word_is_up_right(point, word))
            + usize::from(self.word_is_down_right(point, word))
            + usize::from(self.word_is_down_left(point, word))
            + usize::from(self.word_is_up_left(point, word))
    }

    pub fn has_cross_around(&self, point: Point2d<i32>, word: &str) -> bool {
        let up_right_point = point + UP + RIGHT;
        let up_left_point = point + UP + LEFT;

        self.word_is_down_right(up_left_point, word) && self.word_is_down_left(up_right_point, word)
    }

    fn word_is_up(&self, initial_point: Point2d<i32>, word: &str) -> bool {
        self.word_is_in_direction(initial_point, word, |point| point + UP)
    }

    fn word_is_right(&self, initial_point: Point2d<i32>, word: &str) -> bool {
        self.word_is_in_direction(initial_point, word, |point| point + RIGHT)
    }

    fn word_is_down(&self, initial_point: Point2d<i32>, word: &str) -> bool {
        self.word_is_in_direction(initial_point, word, |point| point + DOWN)
    }

    fn word_is_left(&self, initial_point: Point2d<i32>, word: &str) -> bool {
        self.word_is_in_direction(initial_point, word, |point| point + LEFT)
    }

    fn word_is_up_right(&self, initial_point: Point2d<i32>, word: &str) -> bool {
        self.word_is_in_direction(initial_point, word, |point| point + UP + RIGHT)
    }

    fn word_is_down_right(&self, initial_point: Point2d<i32>, word: &str) -> bool {
        self.word_is_in_direction(initial_point, word, |point| point + DOWN + RIGHT)
    }

    fn word_is_down_left(&self, initial_point: Point2d<i32>, word: &str) -> bool {
        self.word_is_in_direction(initial_point, word, |point| point + DOWN + LEFT)
    }

    fn word_is_up_left(&self, initial_point: Point2d<i32>, word: &str) -> bool {
        self.word_is_in_direction(initial_point, word, |point| point + UP + LEFT)
    }

    fn word_is_in_direction<T>(&self, initial_point: Point2d<i32>, word: &str, transform: T) -> bool
    where
        T: Fn(Point2d<i32>) -> Point2d<i32>,
    {
        let reversed_word: String = word.chars().rev().collect();

        let mut temp_word = String::new();
        let mut current_point = initial_point;

        for _ in 0..word.len() {
            if let Some(&item) = self.data.get(&current_point) {
                temp_word.push(item);
            }

            current_point = transform(current_point);
        }

        (temp_word == word) || (temp_word == reversed_word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input = [
            String::from("XXM"),
            String::from("SAM"),
            String::from("..."),
        ];

        let expected = WordSearch {
            data: HashMap::from([
                (Point2d::new(0, 0), 'X'),
                (Point2d::new(1, 0), 'X'),
                (Point2d::new(2, 0), 'M'),
                (Point2d::new(0, 1), 'S'),
                (Point2d::new(1, 1), 'A'),
                (Point2d::new(2, 1), 'M'),
                (Point2d::new(0, 2), '.'),
                (Point2d::new(1, 2), '.'),
                (Point2d::new(2, 2), '.'),
            ]),
        };

        let result = WordSearch::new(&input);

        assert_eq!(result, expected);
    }
    #[test]
    fn test_find_count() {
        let input = [
            String::from("....XXMAS."),
            String::from(".SAMXMS..."),
            String::from("...S..A..."),
            String::from("..A.A.MS.X"),
            String::from("XMASAMX.MM"),
            String::from("X.....XA.A"),
            String::from("S.S.S.S.SS"),
            String::from(".A.A.A.A.A"),
            String::from("..M.M.M.MM"),
            String::from(".X.X.XMASX"),
        ];

        let puzzle = WordSearch::new(&input);

        assert_eq!(puzzle.find_count("XMAS"), 18);
    }

    #[test]
    fn test_find_count_around() {
        let input = [
            String::from("....XXMAS."),
            String::from(".SAMXMS..."),
            String::from("...S..A..."),
            String::from("..A.A.MS.X"),
            String::from("XMASAMX.MM"),
            String::from("X.....XA.A"),
            String::from("S.S.S.S.SS"),
            String::from(".A.A.A.A.A"),
            String::from("..M.M.M.MM"),
            String::from(".X.X.XMASX"),
        ];

        let puzzle = WordSearch::new(&input);

        assert_eq!(puzzle.find_count_around(Point2d::new(0, 0), "XMAS"), 0);
        assert_eq!(puzzle.find_count_around(Point2d::new(5, 9), "XMAS"), 3);
        assert_eq!(puzzle.find_count_around(Point2d::new(6, 5), "XMAS"), 1);
    }

    #[test]
    fn test_find_cross_count() {
        let input = [
            String::from(".M.S......"),
            String::from("..A..MSMS."),
            String::from(".M.S.MAA.."),
            String::from("..A.ASMSM."),
            String::from(".M.S.M...."),
            String::from(".........."),
            String::from("S.S.S.S.S."),
            String::from(".A.A.A.A.."),
            String::from("M.M.M.M.M."),
            String::from(".........."),
        ];

        let puzzle = WordSearch::new(&input);

        assert_eq!(puzzle.find_cross_count("MAS"), 9);
    }

    #[test]
    fn test_has_cross_around() {
        let input = [
            String::from(".M.S......"),
            String::from("..A..MSMS."),
            String::from(".M.S.MAA.."),
            String::from("..A.ASMSM."),
            String::from(".M.S.M...."),
            String::from(".........."),
            String::from("S.S.S.S.S."),
            String::from(".A.A.A.A.."),
            String::from("M.M.M.M.M."),
            String::from(".........."),
        ];

        let puzzle = WordSearch::new(&input);

        assert!(puzzle.has_cross_around(Point2d::new(2, 1), "MAS"));
        assert!(!puzzle.has_cross_around(Point2d::new(1, 1), "MAS"));
    }

    #[test]
    fn test_word_is_up() {
        let input = [
            String::from("....XXMAS."),
            String::from(".SAMXMS..."),
            String::from("...S..A..."),
            String::from("..A.A.MS.X"),
            String::from("XMASAMX.MM"),
            String::from("X.....XA.A"),
            String::from("S.S.S.S.SS"),
            String::from(".A.A.A.A.A"),
            String::from("..M.M.M.MM"),
            String::from(".X.X.XMASX"),
        ];

        let puzzle = WordSearch::new(&input);

        let result_false_1 = puzzle.word_is_up(Point2d::new(5, 0), "XMAS");
        let result_false_2 = puzzle.word_is_up(Point2d::new(3, 3), "XMAS");
        let result_false_3 = puzzle.word_is_up(Point2d::new(9, 8), "XMAS");
        let result_true_1 = puzzle.word_is_up(Point2d::new(6, 4), "XMAS");
        let result_true_2 = puzzle.word_is_up(Point2d::new(9, 9), "XMAS");

        assert!(!result_false_1);
        assert!(!result_false_2);
        assert!(!result_false_3);
        assert!(result_true_1);
        assert!(result_true_2);
    }

    #[test]
    fn test_word_is_right() {
        let input = [
            String::from("....XXMAS."),
            String::from(".SAMXMS..."),
            String::from("...S..A..."),
            String::from("..A.A.MS.X"),
            String::from("XMASAMX.MM"),
            String::from("X.....XA.A"),
            String::from("S.S.S.S.SS"),
            String::from(".A.A.A.A.A"),
            String::from("..M.M.M.MM"),
            String::from(".X.X.XMASX"),
        ];

        let puzzle = WordSearch::new(&input);

        let result_false_1 = puzzle.word_is_right(Point2d::new(7, 0), "XMAS");
        let result_false_2 = puzzle.word_is_right(Point2d::new(1, 4), "XMAS");
        let result_false_3 = puzzle.word_is_right(Point2d::new(9, 9), "XMAS");
        let result_true_1 = puzzle.word_is_right(Point2d::new(0, 4), "XMAS");
        let result_true_2 = puzzle.word_is_right(Point2d::new(1, 1), "XMAS");

        assert!(!result_false_1);
        assert!(!result_false_2);
        assert!(!result_false_3);
        assert!(result_true_1);
        assert!(result_true_2);
    }

    #[test]
    fn test_word_is_down() {
        let input = [
            String::from("....XXMAS."),
            String::from(".SAMXMS..."),
            String::from("...S..A..."),
            String::from("..A.A.MS.X"),
            String::from("XMASAMX.MM"),
            String::from("X.....XA.A"),
            String::from("S.S.S.S.SS"),
            String::from(".A.A.A.A.A"),
            String::from("..M.M.M.MM"),
            String::from(".X.X.XMASX"),
        ];

        let puzzle = WordSearch::new(&input);

        let result_false_1 = puzzle.word_is_down(Point2d::new(0, 3), "XMAS");
        let result_false_2 = puzzle.word_is_down(Point2d::new(6, 0), "XMAS");
        let result_false_3 = puzzle.word_is_down(Point2d::new(9, 9), "XMAS");
        let result_true_1 = puzzle.word_is_down(Point2d::new(6, 1), "XMAS");
        let result_true_2 = puzzle.word_is_down(Point2d::new(9, 3), "XMAS");

        assert!(!result_false_1);
        assert!(!result_false_2);
        assert!(!result_false_3);
        assert!(result_true_1);
        assert!(result_true_2);
    }

    #[test]
    fn test_word_is_left() {
        let input = [
            String::from("....XXMAS."),
            String::from(".SAMXMS..."),
            String::from("...S..A..."),
            String::from("..A.A.MS.X"),
            String::from("XMASAMX.MM"),
            String::from("X.....XA.A"),
            String::from("S.S.S.S.SS"),
            String::from(".A.A.A.A.A"),
            String::from("..M.M.M.MM"),
            String::from(".X.X.XMASX"),
        ];

        let puzzle = WordSearch::new(&input);

        let result_false_1 = puzzle.word_is_left(Point2d::new(0, 0), "XMAS");
        let result_false_2 = puzzle.word_is_left(Point2d::new(6, 1), "XMAS");
        let result_false_3 = puzzle.word_is_left(Point2d::new(9, 9), "XMAS");
        let result_true_1 = puzzle.word_is_left(Point2d::new(4, 1), "XMAS");
        let result_true_2 = puzzle.word_is_left(Point2d::new(3, 4), "XMAS");

        assert!(!result_false_1);
        assert!(!result_false_2);
        assert!(!result_false_3);
        assert!(result_true_1);
        assert!(result_true_2);
    }

    #[test]
    fn test_word_is_up_right() {
        let input = [
            String::from("....XXMAS."),
            String::from(".SAMXMS..."),
            String::from("...S..A..."),
            String::from("..A.A.MS.X"),
            String::from("XMASAMX.MM"),
            String::from("X.....XA.A"),
            String::from("S.S.S.S.SS"),
            String::from(".A.A.A.A.A"),
            String::from("..M.M.M.MM"),
            String::from(".X.X.XMASX"),
        ];

        let puzzle = WordSearch::new(&input);

        let result_false_1 = puzzle.word_is_up_right(Point2d::new(0, 9), "XMAS");
        let result_false_2 = puzzle.word_is_up_right(Point2d::new(2, 3), "XMAS");
        let result_false_3 = puzzle.word_is_up_right(Point2d::new(9, 9), "XMAS");
        let result_true_1 = puzzle.word_is_up_right(Point2d::new(5, 9), "XMAS");
        let result_true_2 = puzzle.word_is_up_right(Point2d::new(6, 6), "XMAS");

        assert!(!result_false_1);
        assert!(!result_false_2);
        assert!(!result_false_3);
        assert!(result_true_1);
        assert!(result_true_2);
    }

    #[test]
    fn test_word_is_down_right() {
        let input = [
            String::from("....XXMAS."),
            String::from(".SAMXMS..."),
            String::from("...S..A..."),
            String::from("..A.A.MS.X"),
            String::from("XMASAMX.MM"),
            String::from("X.....XA.A"),
            String::from("S.S.S.S.SS"),
            String::from(".A.A.A.A.A"),
            String::from("..M.M.M.MM"),
            String::from(".X.X.XMASX"),
        ];

        let puzzle = WordSearch::new(&input);

        let result_false_1 = puzzle.word_is_down_right(Point2d::new(9, 9), "XMAS");
        let result_false_2 = puzzle.word_is_down_right(Point2d::new(4, 1), "XMAS");
        let result_false_3 = puzzle.word_is_down_right(Point2d::new(9, 0), "XMAS");
        let result_true_1 = puzzle.word_is_down_right(Point2d::new(4, 0), "XMAS");
        let result_true_2 = puzzle.word_is_down_right(Point2d::new(3, 2), "XMAS");

        assert!(!result_false_1);
        assert!(!result_false_2);
        assert!(!result_false_3);
        assert!(result_true_1);
        assert!(result_true_2);
    }

    #[test]
    fn test_word_is_down_left() {
        let input = [
            String::from("....XXMAS."),
            String::from(".SAMXMS..."),
            String::from("...S..A..."),
            String::from("..A.A.MS.X"),
            String::from("XMASAMX.MM"),
            String::from("X.....XA.A"),
            String::from("S.S.S.S.SS"),
            String::from(".A.A.A.A.A"),
            String::from("..M.M.M.MM"),
            String::from(".X.X.XMASX"),
        ];

        let puzzle = WordSearch::new(&input);

        let result_false_1 = puzzle.word_is_down_left(Point2d::new(0, 0), "XMAS");
        let result_false_2 = puzzle.word_is_down_left(Point2d::new(4, 1), "XMAS");
        let result_false_3 = puzzle.word_is_down_left(Point2d::new(9, 7), "XMAS");
        let result_true_1 = puzzle.word_is_down_left(Point2d::new(8, 6), "XMAS");
        let result_true_2 = puzzle.word_is_down_left(Point2d::new(9, 3), "XMAS");

        assert!(!result_false_1);
        assert!(!result_false_2);
        assert!(!result_false_3);
        assert!(result_true_1);
        assert!(result_true_2);
    }

    #[test]
    fn test_word_is_up_left() {
        let input = [
            String::from("....XXMAS."),
            String::from(".SAMXMS..."),
            String::from("...S..A..."),
            String::from("..A.A.MS.X"),
            String::from("XMASAMX.MM"),
            String::from("X.....XA.A"),
            String::from("S.S.S.S.SS"),
            String::from(".A.A.A.A.A"),
            String::from("..M.M.M.MM"),
            String::from(".X.X.XMASX"),
        ];

        let puzzle = WordSearch::new(&input);

        let result_false_1 = puzzle.word_is_up_left(Point2d::new(0, 9), "XMAS");
        let result_false_2 = puzzle.word_is_up_left(Point2d::new(4, 4), "XMAS");
        let result_false_3 = puzzle.word_is_up_left(Point2d::new(9, 0), "XMAS");
        let result_true_1 = puzzle.word_is_up_left(Point2d::new(6, 5), "XMAS");
        let result_true_2 = puzzle.word_is_up_left(Point2d::new(7, 3), "XMAS");

        assert!(!result_false_1);
        assert!(!result_false_2);
        assert!(!result_false_3);
        assert!(result_true_1);
        assert!(result_true_2);
    }
}
