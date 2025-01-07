use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Operation {
    And,
    Xor,
    Or,
}

impl From<&str> for Operation {
    fn from(input: &str) -> Self {
        match input {
            "AND" => Self::And,
            "XOR" => Self::Xor,
            "OR" => Self::Or,
            _ => panic!("Cannot map {input:?} to `Operation`"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Gate {
    input_wire_id_a: String,
    input_wire_id_b: String,
    output_wire_id: String,
    operation: Operation,
}

impl Gate {
    fn new(input_id_a: &str, input_id_b: &str, output_id: &str, operation: Operation) -> Self {
        Gate {
            input_wire_id_a: input_id_a.to_string(),
            input_wire_id_b: input_id_b.to_string(),
            output_wire_id: output_id.to_string(),
            operation,
        }
    }

    fn apply(&self, input_wire_state_a: bool, input_wire_state_b: bool) -> bool {
        match self.operation {
            Operation::And => input_wire_state_a & input_wire_state_b,
            Operation::Xor => input_wire_state_a ^ input_wire_state_b,
            Operation::Or => input_wire_state_a | input_wire_state_b,
        }
    }

    fn has_correct_output_id(&self, output_id: &str) -> bool {
        self.output_wire_id == output_id
    }
}

impl From<&String> for Gate {
    fn from(input: &String) -> Self {
        Self::from(input.as_str())
    }
}

impl From<&str> for Gate {
    fn from(input: &str) -> Self {
        let split_input: Vec<&str> = input.split(' ').collect();

        assert_eq!(split_input.len(), 5, "{input:?} is not valid for `Gate`!");

        Gate::new(
            split_input[0],
            split_input[2],
            split_input[4],
            Operation::from(split_input[1]),
        )
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Wire {
    id: String,
    state: bool,
}

impl Wire {
    fn new(id: &str, state: bool) -> Self {
        Wire {
            id: id.to_string(),
            state,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct CircuitBoard {
    wires: HashMap<String, bool>,
    gates: VecDeque<Gate>,
}

impl CircuitBoard {
    pub fn process(&mut self) {
        let mut processing = self.gates.clone();

        while let Some(gate) = processing.pop_front() {
            if let Some(wire) = self.process_gate(&gate) {
                self.wires.insert(wire.id, wire.state);
            } else {
                processing.push_back(gate);
            }
        }
    }

    pub fn number_from_z_wires(&self) -> u64 {
        self.z_wires_sorted().iter().rev().fold(0, |acc, wire| {
            if wire.state {
                (acc << 1) + 1
            } else {
                acc << 1
            }
        })
    }

    // Gate { input_wire_id_a: "ktr", input_wire_id_b: "ssd", output_wire_id: "cdr", operation: And }
    // Gate { input_wire_id_a: "cdr", input_wire_id_b: "hjh", output_wire_id: "vmr", operation: Or }
    // Gate { input_wire_id_a: "fph", input_wire_id_b: "vmr", output_wire_id: "z03", operation: Xor }
    // ---set z04---
    // Gate { input_wire_id_a: "fph", input_wire_id_b: "vmr", output_wire_id: "fnf", operation: And }
    // Gate { input_wire_id_a: "fnf", input_wire_id_b: "wpn", output_wire_id: "bbh", operation: Or }
    // Gate { input_wire_id_a: "jth", input_wire_id_b: "bbh", output_wire_id: "z04", operation: Xor }
    pub fn fix_gate_ouputs(&mut self) -> Vec<String> {
        // order gate outputs starting from highest Z
        let output_ids_to_gates = self.output_ids_to_gates();

        let mut z_ids: Vec<&String> = output_ids_to_gates
            .keys()
            .filter(|id| id.starts_with('z'))
            .collect();

        z_ids.sort();

        let mut processed = Vec::new();
        let mut processing: VecDeque<&String> = z_ids.into_iter().collect();

        while let Some(id) = processing.pop_front() {
            println!("ID: {id}");

            if let Some(gate) = output_ids_to_gates.get(id) {
                if let Some(wire) = self.process_gate(&gate) {
                    self.wires.insert(wire.id, wire.state);

                    if !gate.input_wire_id_a.starts_with('x')
                        && !gate.input_wire_id_a.starts_with('y')
                    {
                        processed.push(gate);
                    }
                } else {
                    processing.push_front(id);
                    processing.push_front(&gate.input_wire_id_b);
                    processing.push_front(&gate.input_wire_id_a);
                }
            }
        }

        println!("\nProcessed:\n");
        for (index, gate) in processed.into_iter().enumerate() {
            println!("{index}: {gate:?}");
        }

        unimplemented!()
    }

    fn process_gate(&self, gate: &Gate) -> Option<Wire> {
        let (input_id_a, input_id_b) = (&gate.input_wire_id_a, &gate.input_wire_id_b);

        match (self.wires.get(input_id_a), self.wires.get(input_id_b)) {
            (Some(wire_a_state), Some(wire_b_state)) => Some(Wire::new(
                &gate.output_wire_id,
                gate.apply(*wire_a_state, *wire_b_state),
            )),
            _ => None,
        }
    }

    fn switch_gate_outputs(&mut self, output_id_a: &str, output_id_b: &str) {
        unimplemented!()
    }

    fn z_wires_sorted(&self) -> Vec<Wire> {
        let mut z_wires: Vec<Wire> = self
            .wires
            .iter()
            .filter(|(id, _)| id.starts_with('z'))
            .map(|(id, state)| Wire::new(id, *state))
            .collect();

        z_wires.sort_by(|a, b| a.id.cmp(&b.id));

        z_wires
    }

    fn output_ids_to_gates(&self) -> HashMap<String, Gate> {
        self.gates
            .iter()
            .map(|gate| (gate.output_wire_id.clone(), gate.clone()))
            .collect()
    }

    fn parse_wires_input(input: &[String]) -> HashMap<String, bool> {
        input
            .iter()
            .filter_map(|line| line.split_once(": "))
            .map(|(wire_id, value)| (wire_id.to_string(), value == "1"))
            .collect()
    }

    fn parse_gates_input(input: &[String]) -> VecDeque<Gate> {
        input.iter().map(Gate::from).collect()
    }
}

impl<const N: usize> From<[&str; N]> for CircuitBoard {
    fn from(input: [&str; N]) -> Self {
        let strings: Vec<String> = input.iter().map(ToString::to_string).collect();

        Self::from(strings.as_slice())
    }
}

impl From<&[String]> for CircuitBoard {
    fn from(input: &[String]) -> Self {
        let split_input: Vec<&[String]> = input.split(String::is_empty).collect();

        assert_eq!(split_input.len(), 2, "{input:?} not a valid `CircuitBoard`");

        CircuitBoard {
            wires: CircuitBoard::parse_wires_input(split_input[0]),
            gates: CircuitBoard::parse_gates_input(split_input[1]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gate_from_str() {
        let inputs = [
            "tgd XOR rvg -> z01",
            "vdt OR tnw -> bfw",
            "bfw AND frj -> z10",
        ];

        let expected = vec![
            Gate::new("tgd", "rvg", "z01", Operation::Xor),
            Gate::new("vdt", "tnw", "bfw", Operation::Or),
            Gate::new("bfw", "frj", "z10", Operation::And),
        ];

        let result: Vec<Gate> = inputs.into_iter().map(Gate::from).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_gate_apply_and() {
        let gate = Gate::new("vdt", "tnw", "bfw", Operation::And);

        assert!(!gate.apply(false, false));
        assert!(!gate.apply(false, true));
        assert!(!gate.apply(true, false));
        assert!(gate.apply(true, true));
    }

    #[test]
    fn test_gate_apply_xor() {
        let gate = Gate::new("vdt", "tnw", "bfw", Operation::Xor);

        assert!(!gate.apply(false, false));
        assert!(!gate.apply(true, true));
        assert!(gate.apply(false, true));
        assert!(gate.apply(true, false));
    }

    #[test]
    fn test_gate_apply_or() {
        let gate = Gate::new("vdt", "tnw", "bfw", Operation::Or);

        assert!(!gate.apply(false, false));
        assert!(gate.apply(false, true));
        assert!(gate.apply(true, false));
        assert!(gate.apply(true, true));
    }

    #[test]
    fn test_circuit_board_from_str_array() {
        let expected_wires = HashMap::from([
            (String::from("x00"), true),
            (String::from("x01"), true),
            (String::from("x02"), true),
            (String::from("y00"), false),
            (String::from("y01"), true),
            (String::from("y02"), false),
        ]);

        let expected_gates = VecDeque::from([
            Gate::new("x00", "y00", "z00", Operation::And),
            Gate::new("x01", "y01", "z01", Operation::Xor),
            Gate::new("x02", "y02", "z02", Operation::Or),
        ]);

        let expected = CircuitBoard {
            wires: expected_wires,
            gates: expected_gates,
        };

        let result = CircuitBoard::from([
            "x00: 1",
            "x01: 1",
            "x02: 1",
            "y00: 0",
            "y01: 1",
            "y02: 0",
            "",
            "x00 AND y00 -> z00",
            "x01 XOR y01 -> z01",
            "x02 OR y02 -> z02",
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_circuit_board_z_wires_sorted() {
        let circuit_board = CircuitBoard::from([
            "z11: 1",
            "z00: 1",
            "z20: 0",
            "z22: 1",
            "z01: 1",
            "z10: 0",
            "z02: 1",
            "",
            "x00 AND y00 -> q00",
        ]);

        let expected = vec![
            Wire::new("z00", true),
            Wire::new("z01", true),
            Wire::new("z02", true),
            Wire::new("z10", false),
            Wire::new("z11", true),
            Wire::new("z20", false),
            Wire::new("z22", true),
        ];

        let result = circuit_board.z_wires_sorted();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_circuit_board_number_from_z_wires() {
        let circuit_board = CircuitBoard::from([
            "z11: 1",
            "z00: 1",
            "z20: 0",
            "z22: 1",
            "z01: 1",
            "z10: 0",
            "z02: 1",
            "",
            "x00 AND y00 -> q00",
        ]);

        assert_eq!(circuit_board.number_from_z_wires(), 87);
    }

    #[test]
    fn test_circuit_board_process_simple() {
        let mut circuit_board = CircuitBoard::from([
            "x00: 1",
            "x01: 1",
            "x02: 1",
            "y00: 0",
            "y01: 1",
            "y02: 0",
            "",
            "x00 AND y00 -> z00",
            "x01 XOR y01 -> z01",
            "x02 OR y02 -> z02",
        ]);

        circuit_board.process();

        assert_eq!(circuit_board.number_from_z_wires(), 4);
    }
}
