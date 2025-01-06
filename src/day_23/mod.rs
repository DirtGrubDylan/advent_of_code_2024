mod network;

use crate::util::file_reader::to_string_vector;

use network::Network;

pub fn run() {
    let input = to_string_vector("inputs/day_23.txt").expect("Something went wrong with Day 23!");

    println!("Day 23 Part 1: {:?}", part_1(&input));
    println!("Day 23 Part 2: {}", part_2(&input));
}

fn part_1(input: &[String]) -> usize {
    let network = Network::from(input);

    network
        .all_subnetworks_of_size(3)
        .into_iter()
        .map(|ids| ids.iter().filter(|id| id.starts_with('t')).count())
        .filter(|count| *count > 0)
        .count()
}

fn part_2(input: &[String]) -> String {
    let network = Network::from(input);

    network.longest_sub_network().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_23.txt").unwrap();

        assert_eq!(part_1(&input), 7);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_23.txt").unwrap();

        assert_eq!(part_2(&input), "co,de,ka,ta");
    }
}
