use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn singularity_scores(input: &[String]) -> Vec<u32> {
    let mut left_list = Vec::new();
    let mut right_counter = HashMap::new();

    for (left, right) in input.iter().map(|input| to_ints(input)) {
        left_list.push(left);

        right_counter
            .entry(right)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    left_list
        .iter()
        .map(|left| left * right_counter.get(left).unwrap_or(&0))
        .collect()
}

pub fn minimum_differences(input: &[String]) -> Vec<u32> {
    let mut result = Vec::new();

    let (mut min_heap_1, mut min_heap_2) = to_min_heaps(input);

    while let (Some(Reverse(first)), Some(Reverse(second))) = (min_heap_1.pop(), min_heap_2.pop()) {
        result.push(first.abs_diff(second));
    }

    result
}

fn to_min_heaps(input: &[String]) -> (BinaryHeap<Reverse<u32>>, BinaryHeap<Reverse<u32>>) {
    let mut heap_1 = BinaryHeap::new();
    let mut heap_2 = BinaryHeap::new();

    for (first, second) in input.iter().map(|input| to_ints(input)) {
        heap_1.push(Reverse(first));
        heap_2.push(Reverse(second));
    }

    (heap_1, heap_2)
}

fn to_ints(input: &str) -> (u32, u32) {
    let (first_str, second_str) = input.split_once("   ").unwrap();

    (first_str.parse().unwrap(), second_str.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_ints() {
        let inputs = [
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];

        let expected = [(3, 4), (4, 3), (2, 5), (1, 3), (3, 9), (3, 3)];

        let result: Vec<(u32, u32)> = inputs.iter().map(|input| to_ints(input)).collect();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_to_min_heap() {
        let inputs = [
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];

        let expected_heap_1 = [
            Reverse(4),
            Reverse(3),
            Reverse(3),
            Reverse(3),
            Reverse(2),
            Reverse(1),
        ];
        let expected_heap_2 = [
            Reverse(9),
            Reverse(5),
            Reverse(4),
            Reverse(3),
            Reverse(3),
            Reverse(3),
        ];

        let (result_heap_1, result_heap_2) = to_min_heaps(&inputs);

        assert_eq!(result_heap_1.into_sorted_vec(), expected_heap_1);
        assert_eq!(result_heap_2.into_sorted_vec(), expected_heap_2);
    }

    #[test]
    fn test_minimum_differences() {
        let inputs = [
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];

        let expected = [2, 1, 0, 1, 2, 5];

        let result = minimum_differences(&inputs);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_get_singulatiry_scores() {
        let inputs = [
            "3   4".to_string(),
            "4   3".to_string(),
            "2   5".to_string(),
            "1   3".to_string(),
            "3   9".to_string(),
            "3   3".to_string(),
        ];

        let expected = [9, 4, 0, 0, 9, 9];

        let result = singularity_scores(&inputs);

        assert_eq!(result, expected);
    }
}
