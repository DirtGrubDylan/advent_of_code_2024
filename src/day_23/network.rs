use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Clone)]
struct Computer {
    id: String,
    ids_connected_to: HashSet<String>,
}

impl Computer {
    fn new(id: &str) -> Self {
        Computer {
            id: id.to_string(),
            ids_connected_to: HashSet::new(),
        }
    }

    fn add_connection(&mut self, other_id: &str) -> bool {
        self.ids_connected_to.insert(other_id.to_string())
    }
}

#[derive(Debug, Default, PartialEq)]
struct Network {
    map: HashMap<String, Computer>,
}

impl Network {
    fn add_connection(&mut self, id_1: &str, id_2: &str) {
        self.map
            .entry(id_1.to_string())
            .or_insert(Computer::new(id_1))
            .add_connection(id_2);

        self.map
            .entry(id_2.to_string())
            .or_insert(Computer::new(id_2))
            .add_connection(id_1);
    }
}

impl<const N: usize> From<[&str; N]> for Network {
    fn from(input: [&str; N]) -> Self {
        let strings: Vec<String> = input.iter().map(ToString::to_string).collect();

        Self::from(strings.as_slice())
    }
}

impl From<&Vec<String>> for Network {
    fn from(input: &Vec<String>) -> Self {
        Self::from(input.as_slice())
    }
}

impl From<&[String]> for Network {
    fn from(input: &[String]) -> Self {
        let mut network = Network::default();

        input
            .iter()
            .filter_map(|line| line.split_once('-'))
            .for_each(|(lhs, rhs)| network.add_connection(lhs, rhs));

        network
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_from_str_array() {
        let mut kh_computer = Computer::new("kh");
        let mut tc_computer = Computer::new("tc");
        let mut qp_computer = Computer::new("qp");
        let mut de_computer = Computer::new("de");
        let mut cg_computer = Computer::new("cg");

        kh_computer.add_connection("tc");
        tc_computer.add_connection("kh");
        qp_computer.add_connection("kh");
        kh_computer.add_connection("qp");
        de_computer.add_connection("cg");
        cg_computer.add_connection("de");

        let expected_network_map = HashMap::from([
            (String::from("kh"), kh_computer),
            (String::from("tc"), tc_computer),
            (String::from("qp"), qp_computer),
            (String::from("de"), de_computer),
            (String::from("cg"), cg_computer),
        ]);

        let expected = Network {
            map: expected_network_map,
        };

        let result = Network::from(["kh-tc", "qp-kh", "de-cg"]);

        assert_eq!(result, expected);
    }
}
