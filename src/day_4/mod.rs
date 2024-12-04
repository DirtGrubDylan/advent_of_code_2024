mod word_search;

use crate::util::file_reader::to_string_vector;

use word_search::WordSearch;

pub fn run() {
    let input = to_string_vector("inputs/day_4.txt").expect("Something went wrong with Day 4!");

    let puzzle = WordSearch::new(&input);

    println!("Day 4 Part 1: {:?}", part_1(&puzzle));
    println!("Day 4 Part 2: {:?}", part_2(&puzzle));
}

fn part_1(puzzle: &WordSearch) -> usize {
    puzzle.find_count("XMAS")
}

fn part_2(puzzle: &WordSearch) -> usize {
    puzzle.find_cross_count("MAS")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_4.txt").unwrap();

        let puzzle = WordSearch::new(&input);

        assert_eq!(part_1(&puzzle), 18);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_4.txt").unwrap();

        let puzzle = WordSearch::new(&input);

        assert_eq!(part_2(&puzzle), 9);
    }
}
