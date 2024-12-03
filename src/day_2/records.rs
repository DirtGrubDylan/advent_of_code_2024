use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, PartialEq)]
pub struct Record {
    levels: Vec<u32>,
}

impl Record {
    pub fn is_safe(&self) -> bool {
        Self::_is_safe(&self.levels)
    }

    pub fn is_safe_with_removal(&self) -> bool {
        Self::_get_sub_one_level_combinations(&self.levels)
            .into_iter()
            .any(|levels| Self::_is_safe(&levels))
    }

    fn _is_safe(levels: &[u32]) -> bool {
        let is_decreasing = levels.first().unwrap() > levels.last().unwrap();

        for window in levels.windows(2) {
            let previous = window.first().unwrap();
            let current = window.last().unwrap();

            let has_stopped_decreasing = (previous < current) && is_decreasing;
            let has_stopped_increasing = (current < previous) && !is_decreasing;
            let has_invalid_diff =
                (3 < previous.abs_diff(*current)) || (previous.abs_diff(*current) == 0);

            if has_stopped_decreasing || has_stopped_increasing || has_invalid_diff {
                return false;
            }
        }

        true
    }

    fn _get_sub_one_level_combinations(levels: &[u32]) -> Vec<Vec<u32>> {
        let mut result = vec![levels.to_vec()];

        for index in 0..levels.len() {
            let mut temp = levels.to_vec();

            temp.remove(index);

            result.push(temp);
        }

        result
    }
}

impl FromStr for Record {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let levels = input
            .split(' ')
            .map(|str_val| str_val.parse().unwrap())
            .collect();

        Ok(Record { levels })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let input = "7 6 4 2 1";

        let expected = Record {
            levels: vec![7, 6, 4, 2, 1],
        };

        let result = input.parse();

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_is_safe() {
        let records: Vec<Record> = ["7 6 4 2 1", "1 3 6 7 9"]
            .into_iter()
            .map(|input| input.parse::<Record>().unwrap())
            .collect();

        assert!(records.iter().all(Record::is_safe));
    }

    #[test]
    fn test_is_not_safe() {
        let records: Vec<Record> = ["1 2 7 8 9", "9 7 6 2 1", "1 3 2 4 5", "8 6 4 4 1"]
            .into_iter()
            .map(|input| input.parse::<Record>().unwrap())
            .collect();

        assert!(records.iter().all(|record| !record.is_safe()));
    }

    #[test]
    fn test_is_safe_with_removal() {
        let records: Vec<Record> = ["1 2 7 8 9", "9 7 6 2 1", "1 3 2 4 5", "8 6 4 4 1"]
            .into_iter()
            .map(|input| input.parse::<Record>().unwrap())
            .collect();

        let expected = [false, false, true, true];

        let result: Vec<bool> = records.iter().map(Record::is_safe_with_removal).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_sub_one_level_combinations() {
        let input = [7, 6, 4, 2, 1];

        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![6, 4, 2, 1],
            vec![7, 4, 2, 1],
            vec![7, 6, 2, 1],
            vec![7, 6, 4, 1],
            vec![7, 6, 4, 2],
        ];

        let result = Record::_get_sub_one_level_combinations(&input);

        assert_eq!(result, expected);
    }
}
