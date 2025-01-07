mod circuit_board;

use crate::util::file_reader::to_string_vector;

use circuit_board::CircuitBoard;

pub fn run() {
    let input = to_string_vector("inputs/day_24.txt").expect("Something went wrong with Day 24!");

    println!("Day 24 Part 1: {:?}", part_1(&input));
    println!("Day 24 Part 2: {:?}", part_2(&input));
}

// output_wire_id: "z\d{2}", operation: [^X]

fn part_1(input: &[String]) -> u64 {
    let mut circuit_board = CircuitBoard::from(input);

    circuit_board.fix_gate_ouputs();

    circuit_board.process();

    circuit_board.number_from_z_wires()
}

fn part_2(_input: &[String]) -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_24.txt").unwrap();

        assert_eq!(part_1(&input), 2_024);
    }

    #[test]
    #[should_panic(expected = "not implemented")]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_24.txt").unwrap();

        assert_eq!(part_2(&input), 666);
    }
}
