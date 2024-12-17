mod garden;

use crate::util::file_reader::to_string_vector;

use garden::Garden;

pub fn run() {
    let input = to_string_vector("inputs/day_12.txt").expect("Something went wrong with Day 12!");

    println!("Day 12 Part 1: {:?}", part_1(&input));
    println!("Day 12 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> usize {
    let garden = Garden::from(input);

    garden.total_price()
}

fn part_2(_input: &[String]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_12.txt").unwrap();

        assert_eq!(part_1(&input), 1_930);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_12.txt").unwrap();

        assert_eq!(part_2(&input), 666);
    }
}
