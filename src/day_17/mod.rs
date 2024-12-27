use crate::util::file_reader::to_string_vector;

pub fn run() {
    let input = to_string_vector("inputs/day_17.txt").expect("Something went wrong with Day 17!");

    println!("Day 17 Part 1: {:?}", part_1(&input));
    println!("Day 17 Part 2: {:?}", part_2(&input));
}

fn part_1(_input: &[String]) -> usize {
    unimplemented!()
}

fn part_2(_input: &[String]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "File not found!")]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_17.txt").unwrap();

        assert_eq!(part_1(&input), 666);
    }

    #[test]
    #[should_panic(expected = "File not found!")]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_17.txt").unwrap();

        assert_eq!(part_2(&input), 666);
    }
}
