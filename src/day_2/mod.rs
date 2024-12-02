use crate::util::file_reader::to_string_vector;

pub fn run() {
    let input = to_string_vector("inputs/day_2.txt").expect("Something went wrong with Day 2!");

    println!("Day 2 Part 1: {:?}", part_1(&input));
    println!("Day 2 Part 2: {:?}", part_2(&input));
}

fn part_1(_input: &[String]) {
    unimplemented!()
}

fn part_2(_input: &[String]) -> u32 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_2.txt").unwrap();

        part_1(&input);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_2.txt").unwrap();

        part_2(&input);
    }
}
