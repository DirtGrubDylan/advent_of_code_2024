use std::collections::{HashMap, HashSet, VecDeque};

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

    fn is_connected_to_all(&self, ids: &HashSet<String>) -> bool {
        ids.is_subset(&self.ids_connected_to)
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
struct SearchNode {
    sub_network_ids: HashSet<String>,
    ids_to_process: HashSet<String>,
}

impl SearchNode {
    fn new(computer: &Computer) -> Self {
        SearchNode {
            sub_network_ids: HashSet::from([computer.id.clone()]),
            ids_to_process: computer.ids_connected_to.clone(),
        }
    }

    fn len(&self) -> usize {
        self.sub_network_ids.len()
    }

    fn maximum_possible_len(&self) -> usize {
        self.sub_network_ids.len() + self.ids_to_process.len()
    }

    fn clone_and_add_subnetwork_id_from(&self, computer: &Computer) -> Self {
        let mut clone = self.clone();

        clone.sub_network_ids.insert(computer.id.clone());

        clone.ids_to_process = computer
            .ids_connected_to
            .difference(&clone.sub_network_ids)
            .cloned()
            .collect();

        clone
    }

    fn sub_network_ids_to_sorted_vec(&self) -> Vec<String> {
        let mut as_vec: Vec<String> = self.sub_network_ids.iter().cloned().collect();

        as_vec.sort();

        as_vec
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Network {
    map: HashMap<String, Computer>,
}

impl Network {
    pub fn longest_sub_network(&self) -> Vec<String> {
        let mut seen_sub_networks: HashSet<Vec<String>> = HashSet::new();
        let mut processing: VecDeque<SearchNode> = self.map.values().map(SearchNode::new).collect();

        let mut max_length_sub_network = Vec::new();

        while let Some(node) = processing.pop_back() {
            let impossible_to_exceed_maximum =
                node.maximum_possible_len() <= max_length_sub_network.len();

            let already_seen = !seen_sub_networks.insert(node.sub_network_ids_to_sorted_vec());

            if already_seen || impossible_to_exceed_maximum {
                continue;
            }

            if node.len() > max_length_sub_network.len() {
                max_length_sub_network = node.sub_network_ids_to_sorted_vec();
            }

            node.ids_to_process
                .iter()
                .filter_map(|id| self.map.get(id))
                .filter(|computer| computer.is_connected_to_all(&node.sub_network_ids))
                .map(|computer| node.clone_and_add_subnetwork_id_from(computer))
                .for_each(|new_node| processing.push_back(new_node));
        }

        max_length_sub_network
    }

    pub fn all_subnetworks_of_size(&self, size: usize) -> HashSet<Vec<String>> {
        let mut result = HashSet::new();

        let mut processing: VecDeque<SearchNode> = self.map.values().map(SearchNode::new).collect();

        while let Some(node) = processing.pop_front() {
            if node.len() == size {
                result.insert(node.sub_network_ids_to_sorted_vec());

                continue;
            }

            node.ids_to_process
                .iter()
                .filter_map(|id| self.map.get(id))
                .filter(|computer| computer.is_connected_to_all(&node.sub_network_ids))
                .map(|computer| node.clone_and_add_subnetwork_id_from(computer))
                .for_each(|new_node| processing.push_back(new_node));
        }

        result
    }

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

    #[test]
    fn test_longest_sub_network() {
        let network = Network::from([
            "kh-tc", "qp-kh", "de-cg", "ka-co", "yn-aq", "qp-ub", "cg-tb", "vc-aq", "tb-ka",
            "wh-tc", "yn-cg", "kh-ub", "ta-co", "de-co", "tc-td", "tb-wq", "wh-td", "ta-ka",
            "td-qp", "aq-cg", "wq-ub", "ub-vc", "de-ta", "wq-aq", "wq-vc", "wh-yn", "ka-de",
            "kh-ta", "co-tc", "wh-qp", "tb-vc", "td-yn",
        ]);

        assert_eq!(network.longest_sub_network().join(","), "co,de,ka,ta");
    }

    #[test]
    fn test_all_subnetworks_of_size_3() {
        let network = Network::from([
            "kh-tc", "qp-kh", "de-cg", "ka-co", "yn-aq", "qp-ub", "cg-tb", "vc-aq", "tb-ka",
            "wh-tc", "yn-cg", "kh-ub", "ta-co", "de-co", "tc-td", "tb-wq", "wh-td", "ta-ka",
            "td-qp", "aq-cg", "wq-ub", "ub-vc", "de-ta", "wq-aq", "wq-vc", "wh-yn", "ka-de",
            "kh-ta", "co-tc", "wh-qp", "tb-vc", "td-yn",
        ]);

        let expected = HashSet::from([
            vec![String::from("aq"), String::from("cg"), String::from("yn")],
            vec![String::from("aq"), String::from("vc"), String::from("wq")],
            vec![String::from("co"), String::from("de"), String::from("ka")],
            vec![String::from("co"), String::from("de"), String::from("ta")],
            vec![String::from("co"), String::from("ka"), String::from("ta")],
            vec![String::from("de"), String::from("ka"), String::from("ta")],
            vec![String::from("kh"), String::from("qp"), String::from("ub")],
            vec![String::from("qp"), String::from("td"), String::from("wh")],
            vec![String::from("tb"), String::from("vc"), String::from("wq")],
            vec![String::from("tc"), String::from("td"), String::from("wh")],
            vec![String::from("td"), String::from("wh"), String::from("yn")],
            vec![String::from("ub"), String::from("vc"), String::from("wq")],
        ]);

        let result = network.all_subnetworks_of_size(3);

        assert_eq!(result.len(), expected.len());
        for expected_subnetwork in expected {
            assert!(
                result.contains(&expected_subnetwork),
                "Result does not contain {expected_subnetwork:?}"
            );
        }
    }
}
