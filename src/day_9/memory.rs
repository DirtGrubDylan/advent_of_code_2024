use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DiskData {
    Empty { block_length: u32 },
    File { id: usize, block_length: u32 },
}

impl DiskData {
    fn new_empty(block_length: u32) -> Self {
        DiskData::Empty { block_length }
    }

    fn new_file(id: usize, block_length: u32) -> Self {
        DiskData::File { id, block_length }
    }

    fn change_block_length(&self, block_length: u32) -> Self {
        match self {
            DiskData::Empty { .. } => Self::new_empty(block_length),
            DiskData::File { id, .. } => Self::new_file(*id, block_length),
        }
    }

    fn is_file(&self) -> bool {
        match self {
            DiskData::Empty { .. } => false,
            DiskData::File { .. } => true,
        }
    }

    fn to_empty(self) -> Self {
        match self {
            DiskData::Empty { .. } => self,
            DiskData::File {
                id: _,
                block_length,
            } => Self::new_empty(block_length),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DiskMap {
    data: Vec<DiskData>,
    raw_data: Vec<DiskData>,
}

impl DiskMap {
    pub fn compacted_raw_data(&self) -> Vec<DiskData> {
        let mut result = self.raw_data.clone();

        let mut empty_space_indices = Self::empty_space_indices(&self.raw_data);

        let mut back_index = result.len().saturating_sub(1);

        while 0 < back_index {
            let back_data = result.get(back_index).unwrap();

            let lowest_valid_empty_index = empty_space_indices
                .get_mut(&back_data.to_empty())
                .filter(|_| back_data.is_file())
                .and_then(VecDeque::pop_front);

            match lowest_valid_empty_index {
                Some(index) if index < back_index => result.swap(index, back_index),
                _ => {}
            }

            back_index -= 1;
        }

        result
    }

    pub fn checksum(data: &[DiskData]) -> usize {
        data.iter()
            .enumerate()
            .map(|(index, item)| match item {
                DiskData::File { id, .. } => id * index,
                DiskData::Empty { .. } => 0,
            })
            .sum()
    }

    fn empty_space_indices(data: &[DiskData]) -> HashMap<DiskData, VecDeque<usize>> {
        let mut result = HashMap::new();

        for (index, item) in data.iter().enumerate() {
            match item {
                DiskData::Empty { .. } => result
                    .entry(*item)
                    .or_insert(VecDeque::new())
                    .push_back(index),
                DiskData::File { .. } => continue,
            }
        }

        result
    }
}

#[derive(Debug, PartialEq)]
pub struct DiskMapParseError {}

impl FromStr for DiskMap {
    type Err = DiskMapParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::new();
        let mut raw_data = Vec::new();

        for (index, c) in input.char_indices() {
            let block_length = c.to_digit(10).expect("{c} is not a digit!");

            let block_data = if index % 2 == 0 {
                DiskData::new_file(index / 2, block_length)
            } else {
                DiskData::new_empty(block_length)
            };

            let raw_block_data = block_data.change_block_length(1);

            data.push(block_data);

            for _ in 0..block_length {
                raw_data.push(raw_block_data);
            }
        }

        Ok(DiskMap { data, raw_data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diskmap_from_str() {
        let input = "12345";

        let expected_data = vec![
            DiskData::File {
                id: 0,
                block_length: 1,
            },
            DiskData::Empty { block_length: 2 },
            DiskData::File {
                id: 1,
                block_length: 3,
            },
            DiskData::Empty { block_length: 4 },
            DiskData::File {
                id: 2,
                block_length: 5,
            },
        ];
        let expected_raw_data = vec![
            DiskData::File {
                id: 0,
                block_length: 1,
            },
            DiskData::Empty { block_length: 1 },
            DiskData::Empty { block_length: 1 },
            DiskData::File {
                id: 1,
                block_length: 1,
            },
            DiskData::File {
                id: 1,
                block_length: 1,
            },
            DiskData::File {
                id: 1,
                block_length: 1,
            },
            DiskData::Empty { block_length: 1 },
            DiskData::Empty { block_length: 1 },
            DiskData::Empty { block_length: 1 },
            DiskData::Empty { block_length: 1 },
            DiskData::File {
                id: 2,
                block_length: 1,
            },
            DiskData::File {
                id: 2,
                block_length: 1,
            },
            DiskData::File {
                id: 2,
                block_length: 1,
            },
            DiskData::File {
                id: 2,
                block_length: 1,
            },
            DiskData::File {
                id: 2,
                block_length: 1,
            },
        ];

        let expected = DiskMap {
            data: expected_data,
            raw_data: expected_raw_data,
        };

        let result = input.parse();

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_diskmap_empty_space_indices_data() {
        let diskmap: DiskMap = "12345".parse().unwrap();

        let expected = HashMap::from([
            (DiskData::Empty { block_length: 2 }, VecDeque::from([1])),
            (DiskData::Empty { block_length: 4 }, VecDeque::from([3])),
        ]);

        let result = DiskMap::empty_space_indices(&diskmap.data);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_diskmap_empty_space_indices_raw_data() {
        let diskmap: DiskMap = "12345".parse().unwrap();

        let expected = HashMap::from([(
            DiskData::Empty { block_length: 1 },
            VecDeque::from([1, 2, 6, 7, 8, 9]),
        )]);

        let result = DiskMap::empty_space_indices(&diskmap.raw_data);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_diskmap_compacted_raw_data() {
        let diskmap: DiskMap = "12345".parse().unwrap();

        let expected = vec![
            DiskData::File {
                id: 0,
                block_length: 1,
            },
            DiskData::File {
                id: 2,
                block_length: 1,
            },
            DiskData::File {
                id: 2,
                block_length: 1,
            },
            DiskData::File {
                id: 1,
                block_length: 1,
            },
            DiskData::File {
                id: 1,
                block_length: 1,
            },
            DiskData::File {
                id: 1,
                block_length: 1,
            },
            DiskData::File {
                id: 2,
                block_length: 1,
            },
            DiskData::File {
                id: 2,
                block_length: 1,
            },
            DiskData::File {
                id: 2,
                block_length: 1,
            },
            DiskData::Empty { block_length: 1 },
            DiskData::Empty { block_length: 1 },
            DiskData::Empty { block_length: 1 },
            DiskData::Empty { block_length: 1 },
            DiskData::Empty { block_length: 1 },
            DiskData::Empty { block_length: 1 },
        ];

        let result = diskmap.compacted_raw_data();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_diskmap_checksum_raw_data() {
        let diskmap: DiskMap = "2333133121414131402".parse().unwrap();

        let compacted_raw_data = diskmap.compacted_raw_data();

        assert_eq!(DiskMap::checksum(&compacted_raw_data), 1_928);
    }
}
