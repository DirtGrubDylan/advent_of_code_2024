use crate::util::file_reader::to_string_vector;

pub fn run() {
    let input = to_string_vector("inputs/day_5.txt").expect("Something went wrong with Day 5!");

    println!("Day 5 Part 1: {:?}", part_1(&input));
    println!("Day 5 Part 2: {:?}", part_2(&input));
}

fn part_1(puzzle: &[String]) -> usize {
    unimplemented!()
}

fn part_2(puzzle: &[String]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_5.txt").unwrap();

        assert_eq!(part_1(&input), 18);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_5.txt").unwrap();

        assert_eq!(part_2(&input), 9);
    }
}
