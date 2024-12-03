mod records;

use crate::util::file_reader::to_string_vector;

use records::Record;

pub fn run() {
    let input = to_string_vector("inputs/day_2.txt").expect("Something went wrong with Day 2!");

    let records = get_records(&input);

    println!("Day 2 Part 1: {:?}", part_1(&records));
    println!("Day 2 Part 2: {:?}", part_2(&records));
}

fn part_1(records: &[Record]) -> usize {
    records.iter().filter(|record| record.is_safe()).count()
}

fn part_2(records: &[Record]) -> usize {
    records
        .iter()
        .filter(|record| record.is_safe_with_removal())
        .count()
}

fn get_records(input: &[String]) -> Vec<Record> {
    input.iter().map(|line| line.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_2.txt").unwrap();

        let records = get_records(&input);

        assert_eq!(part_1(&records), 2);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_2.txt").unwrap();

        let records = get_records(&input);

        assert_eq!(part_2(&records), 4);
    }
}
