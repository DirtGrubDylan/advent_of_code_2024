use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::ops::Sub;
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

    fn block_length(&self) -> u32 {
        match self {
            DiskData::File {
                id: _,
                block_length,
            }
            | DiskData::Empty { block_length } => *block_length,
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

impl Sub for DiskData {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (
                DiskData::Empty {
                    block_length: empty_size,
                },
                DiskData::File {
                    id: _,
                    block_length: file_size,
                },
            ) if file_size <= empty_size => Self::new_empty(empty_size - file_size),
            _ => panic!("Cannot subtract {rhs:?} from {self:?}"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct DiskMap {
    data: Vec<DiskData>,
}

impl DiskMap {
    pub fn compacted_data(&self) -> Vec<DiskData> {
        let mut result = self.data.clone();

        let mut empty_space_indices = Self::empty_space_indices(&self.data);

        let mut back_index = result.len().saturating_sub(1);

        while 0 < back_index {
            let back_data = *result.get(back_index).unwrap();
            let back_data_length = usize::try_from(back_data.block_length()).unwrap();

            let mut empty_index = empty_space_indices
                .iter()
                .filter(|_| back_data.is_file())
                .filter(|(empty_space, _)| back_data.block_length() <= empty_space.block_length())
                .filter_map(|(_, indices)| indices.peek())
                .map(|Reverse(index)| *index)
                .min()
                .unwrap_or(back_index);
            let empty_block = *result.get(empty_index).unwrap();

            if back_index <= empty_index {
                back_index = back_index.saturating_sub(back_data_length);

                continue;
            }

            let remaining_empty_block_index = empty_index + back_data_length;
            let remaining_empty_block = empty_block - back_data;

            let newly_emptied_start_index = back_index - back_data_length + 1;

            for _ in 0..back_data_length {
                *result.get_mut(back_index).unwrap() = back_data.to_empty();
                *result.get_mut(empty_index).unwrap() = back_data;

                back_index = back_index.saturating_sub(1);
                empty_index += 1;
            }

            for _ in 0..remaining_empty_block.block_length() {
                *result.get_mut(empty_index).unwrap() = remaining_empty_block;
                empty_index += 1;
            }

            empty_space_indices.entry(empty_block).and_modify(|heap| {
                heap.pop();
            });

            if remaining_empty_block.block_length() != 0 {
                empty_space_indices
                    .entry(remaining_empty_block)
                    .or_default()
                    .push(Reverse(remaining_empty_block_index));
            }

            empty_space_indices
                .entry(back_data.to_empty())
                .or_default()
                .push(Reverse(newly_emptied_start_index));
        }

        result
    }

    pub fn compacted_data_single_length(&self) -> Vec<DiskData> {
        let mut result = self.data.clone();

        let raw_data_all_single_length: Vec<DiskData> = self
            .data
            .iter()
            .map(|item| item.change_block_length(1))
            .collect();

        let mut empty_space_indices = Self::empty_space_indices(&raw_data_all_single_length);

        let mut back_index = result.len().saturating_sub(1);

        while 0 < back_index {
            let back_data = result.get(back_index).unwrap();

            let lowest_valid_empty_index = empty_space_indices
                .get_mut(&back_data.change_block_length(1).to_empty())
                .filter(|_| back_data.is_file())
                .and_then(BinaryHeap::pop);

            match lowest_valid_empty_index {
                Some(Reverse(index)) if index < back_index => result.swap(index, back_index),
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

    fn empty_space_indices(data: &[DiskData]) -> HashMap<DiskData, BinaryHeap<Reverse<usize>>> {
        let mut result = HashMap::new();

        let mut index = 0;

        while let Some(item) = data.get(index).copied() {
            match item {
                DiskData::Empty { .. } => result
                    .entry(item)
                    .or_insert(BinaryHeap::new())
                    .push(Reverse(index)),
                DiskData::File { .. } => {}
            }

            index += usize::try_from(item.block_length()).unwrap();
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

        for (index, c) in input.char_indices() {
            let block_length = c.to_digit(10).expect("{c} is not a digit!");

            let block_data = if index % 2 == 0 {
                DiskData::new_file(index / 2, block_length)
            } else {
                DiskData::new_empty(block_length)
            };

            for _ in 0..block_length {
                data.push(block_data);
            }
        }

        Ok(DiskMap { data })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diskdata_sub_no_panic() {
        let empty = DiskData::Empty { block_length: 4 };
        let file = DiskData::File {
            id: 69,
            block_length: 3,
        };

        let expected = DiskData::Empty { block_length: 1 };

        let result = empty - file;

        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "Cannot subtract")]
    fn test_diskdata_sub_panic() {
        let empty = DiskData::Empty { block_length: 4 };
        let file = DiskData::File {
            id: 69,
            block_length: 5,
        };

        let _ = empty - file;
        let _ = empty - empty;
        let _ = file - empty;
        let _ = file - file;
    }

    #[test]
    fn test_diskmap_from_str() {
        let input = "12345";

        let expected_data = vec![
            DiskData::File {
                id: 0,
                block_length: 1,
            },
            DiskData::Empty { block_length: 2 },
            DiskData::Empty { block_length: 2 },
            DiskData::File {
                id: 1,
                block_length: 3,
            },
            DiskData::File {
                id: 1,
                block_length: 3,
            },
            DiskData::File {
                id: 1,
                block_length: 3,
            },
            DiskData::Empty { block_length: 4 },
            DiskData::Empty { block_length: 4 },
            DiskData::Empty { block_length: 4 },
            DiskData::Empty { block_length: 4 },
            DiskData::File {
                id: 2,
                block_length: 5,
            },
            DiskData::File {
                id: 2,
                block_length: 5,
            },
            DiskData::File {
                id: 2,
                block_length: 5,
            },
            DiskData::File {
                id: 2,
                block_length: 5,
            },
            DiskData::File {
                id: 2,
                block_length: 5,
            },
        ];

        let expected = DiskMap {
            data: expected_data,
        };

        let result = input.parse();

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_diskmap_empty_space_indices() {
        let diskmap: DiskMap = "12345".parse().unwrap();

        let expected = HashMap::from([
            (
                DiskData::Empty { block_length: 2 },
                BinaryHeap::from([Reverse(1)]),
            ),
            (
                DiskData::Empty { block_length: 4 },
                BinaryHeap::from([Reverse(6)]),
            ),
        ]);

        let mut result = DiskMap::empty_space_indices(&diskmap.data);

        assert_eq!(result.len(), expected.len());

        for (expected_data, mut expected_indices) in expected {
            let result_indices = result.get_mut(&expected_data).unwrap();

            assert_eq!(result_indices.len(), expected_indices.len());

            while !expected_indices.is_empty() {
                assert_eq!(result_indices.pop(), expected_indices.pop());
            }
        }
    }

    #[test]
    fn test_diskmap_compacted_data_single_length() {
        let diskmap: DiskMap = "12345".parse().unwrap();

        let expected = vec![
            DiskData::File {
                id: 0,
                block_length: 1,
            },
            DiskData::File {
                id: 2,
                block_length: 5,
            },
            DiskData::File {
                id: 2,
                block_length: 5,
            },
            DiskData::File {
                id: 1,
                block_length: 3,
            },
            DiskData::File {
                id: 1,
                block_length: 3,
            },
            DiskData::File {
                id: 1,
                block_length: 3,
            },
            DiskData::File {
                id: 2,
                block_length: 5,
            },
            DiskData::File {
                id: 2,
                block_length: 5,
            },
            DiskData::File {
                id: 2,
                block_length: 5,
            },
            DiskData::Empty { block_length: 4 },
            DiskData::Empty { block_length: 4 },
            DiskData::Empty { block_length: 4 },
            DiskData::Empty { block_length: 4 },
            DiskData::Empty { block_length: 2 },
            DiskData::Empty { block_length: 2 },
        ];

        let result = diskmap.compacted_data_single_length();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_diskmap_compacted_data() {
        let diskmap: DiskMap = "14322".parse().unwrap();

        let expected = vec![
            DiskData::File {
                id: 0,
                block_length: 1,
            },
            DiskData::File {
                id: 2,
                block_length: 2,
            },
            DiskData::File {
                id: 2,
                block_length: 2,
            },
            DiskData::Empty { block_length: 2 },
            DiskData::Empty { block_length: 2 },
            DiskData::File {
                id: 1,
                block_length: 3,
            },
            DiskData::File {
                id: 1,
                block_length: 3,
            },
            DiskData::File {
                id: 1,
                block_length: 3,
            },
            DiskData::Empty { block_length: 2 },
            DiskData::Empty { block_length: 2 },
            DiskData::Empty { block_length: 2 },
            DiskData::Empty { block_length: 2 },
        ];

        let result = diskmap.compacted_data();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_diskmap_checksum_data_single_length() {
        let diskmap: DiskMap = "2333133121414131402".parse().unwrap();

        let compacted_raw_data = diskmap.compacted_data_single_length();

        assert_eq!(DiskMap::checksum(&compacted_raw_data), 1_928);
    }

    #[test]
    fn test_diskmap_checksum_data() {
        let diskmap: DiskMap = "2333133121414131402".parse().unwrap();

        let compacted_data = diskmap.compacted_data();

        assert_eq!(DiskMap::checksum(&compacted_data), 2_858);
    }
}
