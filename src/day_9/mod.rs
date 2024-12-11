mod memory;

use crate::util::file_reader::to_string_vector;

use memory::DiskMap;

pub fn run() {
    let input = to_string_vector("inputs/day_9.txt").expect("Something went wrong with Day 9!");

    println!("Day 9 Part 1: {:?}", part_1(&input));
    println!("Day 9 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> usize {
    let diskmap: DiskMap = input
        .first()
        .map(|line| line.parse().expect("Can't parse line"))
        .unwrap();

    let compacted_raw_data = diskmap.compacted_raw_data();

    DiskMap::checksum(&compacted_raw_data)
}

fn part_2(_input: &[String]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_9.txt").unwrap();

        assert_eq!(part_1(&input), 1_928);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_9.txt").unwrap();

        assert_eq!(part_2(&input), 2_858);
    }
}
