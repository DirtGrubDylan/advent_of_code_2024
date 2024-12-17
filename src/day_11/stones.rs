use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::hash::Hash;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Stone {
    engraving: u64,
}

impl Stone {
    fn apply_rules(&self) -> Vec<Self> {
        match self.to_string().as_str() {
            "0" => vec![Self::from(1)],
            s if s.len() % 2 == 0 => {
                let (first, second) = self.split();

                vec![first, second]
            }
            _ => vec![self.multiply()],
        }
    }

    fn multiply(&self) -> Self {
        Self::from(self.engraving * 2_024)
    }

    fn split(&self) -> (Self, Self) {
        let string_val = self.to_string();

        let (first, second) = string_val.split_at(string_val.len() / 2);

        (first.parse().unwrap(), second.parse().unwrap())
    }
}

#[derive(Debug, PartialEq)]
pub struct StoneParseError {}

impl FromStr for Stone {
    type Err = StoneParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u64>()
            .map(Stone::from)
            .map_err(|_| StoneParseError {})
    }
}

impl From<u64> for Stone {
    fn from(value: u64) -> Self {
        Stone { engraving: value }
    }
}

impl Display for Stone {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.engraving)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Arrangement {
    stones: HashMap<Stone, usize>,
}

impl Arrangement {
    pub fn number_of_stones(&self) -> usize {
        self.stones.values().sum()
    }
}

impl Iterator for Arrangement {
    type Item = Arrangement;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new_stones = HashMap::new();

        for (stone, &count) in &self.stones {
            for new_stone in stone.apply_rules() {
                new_stones
                    .entry(new_stone)
                    .and_modify(|new_count| *new_count += count)
                    .or_insert(count);
            }
        }

        self.stones = new_stones.clone();

        Some(Arrangement { stones: new_stones })
    }
}

#[derive(Debug, PartialEq)]
pub struct ArrangementParseError {}

impl FromStr for Arrangement {
    type Err = ArrangementParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stones = HashMap::new();

        for item in s.split(' ') {
            stones
                .entry(item.parse().map_err(|_| ArrangementParseError {})?)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        Ok(Arrangement { stones })
    }
}

impl<const N: usize> From<[u64; N]> for Arrangement {
    fn from(input: [u64; N]) -> Self {
        let mut stones = HashMap::new();

        for stone in input.into_iter().map(Stone::from) {
            stones
                .entry(stone)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        Arrangement { stones }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_rules() {
        let expected_1 = vec![Stone::from(1)];
        let expected_2 = vec![Stone::from(2_024)];
        let expected_3 = vec![Stone::from(20), Stone::from(24)];
        let expected_4 = vec![Stone::from(512_072)];
        let expected_5 = vec![Stone::from(512), Stone::from(72)];

        let result_1 = Stone::from(0).apply_rules();
        let result_2 = Stone::from(1).apply_rules();
        let result_3 = Stone::from(2_024).apply_rules();
        let result_4 = Stone::from(253).apply_rules();
        let result_5 = Stone::from(512_072).apply_rules();

        assert_eq!(result_1, expected_1);
        assert_eq!(result_2, expected_2);
        assert_eq!(result_3, expected_3);
        assert_eq!(result_4, expected_4);
        assert_eq!(result_5, expected_5);
    }

    #[test]
    fn test_arrangement_iter_1() {
        let mut stones: Arrangement = "125 17".parse().unwrap();

        let expected_1 = Arrangement::from([253_000, 1, 7]);
        let expected_2 = Arrangement::from([253, 0, 2_024, 14_168]);
        let expected_3 = Arrangement::from([512_072, 1, 20, 24, 28_676_032]);
        let expected_4 = Arrangement::from([512, 72, 2024, 2, 0, 2, 4, 2_867, 6_032]);
        let expected_5 = Arrangement::from([
            1_036_288, 7, 2, 20, 24, 4_048, 1, 4_048, 8_096, 28, 67, 60, 32,
        ]);
        let expected_6 = Arrangement::from([
            2_097_446_912,
            14_168,
            4_048,
            2,
            0,
            2,
            4,
            40,
            48,
            2024,
            40,
            48,
            80,
            96,
            2,
            8,
            6,
            7,
            6,
            0,
            3,
            2,
        ]);

        let result_1 = stones.next();
        let result_2 = stones.next();
        let result_3 = stones.next();
        let result_4 = stones.next();
        let result_5 = stones.next();
        let result_6 = stones.next();

        assert_eq!(result_1, Some(expected_1));
        assert_eq!(result_2, Some(expected_2));
        assert_eq!(result_3, Some(expected_3));
        assert_eq!(result_4, Some(expected_4));
        assert_eq!(result_5, Some(expected_5));
        assert_eq!(result_6, Some(expected_6));
    }
}
