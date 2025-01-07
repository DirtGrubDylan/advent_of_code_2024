mod circuit_board;

use crate::util::file_reader::to_string_vector;

use circuit_board::CircuitBoard;

pub fn run() {
    let input = to_string_vector("inputs/day_24.txt").expect("Something went wrong with Day 24!");

    println!("Day 24 Part 1: {:?}", part_1(&input));
    println!("Day 24 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> u64 {
    let mut circuit_board = CircuitBoard::from(input);

    circuit_board.process();

    circuit_board.number_from_wires('z')
}

fn part_2(input: &[String]) -> String {
    let circuit_board = CircuitBoard::from(input);

    let mut outputs_to_swap = Vec::new();

    for (id_a, id_b) in circuit_board.gate_outputs_to_swap() {
        outputs_to_swap.push(id_a.clone());
        outputs_to_swap.push(id_b.clone());
    }

    outputs_to_swap.sort();

    outputs_to_swap.join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_24.txt").unwrap();

        assert_eq!(part_1(&input), 2_024);
    }
}
