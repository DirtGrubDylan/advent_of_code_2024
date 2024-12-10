use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DiskData {
    Empty { block_length: usize },
    File { id: usize, block_length: usize },
}

impl DiskData {
    pub fn new_empty(block_length: usize) -> Self {
        unimplemented!()
    }

    pub fn new_file(id: usize, block_length: usize) -> Self {
        unimplemented!()
    }

    pub fn empty_from_char(block_length: char) -> Self {
        unimplemented!()
    }

    pub fn file_from_char(id: usize, block_length: char) -> Self {
        unimplemented!()
    }
}

#[derive(Debug, PartialEq)]
pub struct DiskMap {
    data: Vec<DiskData>,
    raw_data: Vec<DiskData>,
}

#[derive(Debug, PartialEq)]
pub struct DiskMapParseError {}

impl FromStr for DiskMap {
    type Err = DiskMapParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        unimplemented!()
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
            raw_data: expected_raw_data,
        };

        let result = input.parse();

        assert_eq!(result, Ok(expected));
    }
}
