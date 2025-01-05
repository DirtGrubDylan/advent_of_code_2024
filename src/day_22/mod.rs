mod market;

use std::collections::HashMap;

use crate::util::file_reader::to_string_vector;

use market::SecretNumber;

pub fn run() {
    let input = to_string_vector("inputs/day_22.txt").expect("Something went wrong with Day 22!");

    println!("Day 22 Part 1: {:?}", part_1(&input));
    println!("Day 22 Part 2: {:?}", part_2(&input));
}

fn part_1(input: &[String]) -> isize {
    let secret_numbers = secret_numbers(input);

    secret_numbers
        .into_iter()
        .filter_map(|secret_number| secret_number.into_iter().nth(2_000))
        .sum()
}

fn part_2(input: &[String]) -> isize {
    let secret_numbers = secret_numbers(input);

    let mut windows_to_buy_values = HashMap::new();

    for secret_number in secret_numbers {
        for (window, buy_value) in windows_to_buy_value(&secret_number, 2_000) {
            windows_to_buy_values
                .entry(window)
                .and_modify(|sum| *sum += buy_value)
                .or_insert(buy_value);
        }
    }

    windows_to_buy_values.values().copied().max().unwrap_or(0)
}

fn secret_numbers(input: &[String]) -> Vec<SecretNumber> {
    input.iter().filter_map(|line| line.parse().ok()).collect()
}

fn windows_to_buy_value(
    secret: &SecretNumber,
    first_n_values: usize,
) -> HashMap<Vec<isize>, isize> {
    let mut result = HashMap::new();

    secret
        .delta_and_buy_values(first_n_values)
        .windows(4)
        .map(|window| {
            (
                window.iter().map(|item| item.0).collect(),
                window.last().unwrap().1,
            )
        })
        .for_each(|(window, buy_value)| {
            result.entry(window).or_insert(buy_value);
        });

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_22_part_1.txt").unwrap();

        assert_eq!(part_1(&input), 37_327_623);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_22_part_2.txt").unwrap();

        assert_eq!(part_2(&input), 23);
    }

    #[test]
    fn test_windows_and_buy_value() {
        let secret_number = SecretNumber::from(123);

        let expected = HashMap::from([
            (vec![-3, 6, -1, -1], 4),
            (vec![6, -1, -1, 0], 4),
            (vec![-1, -1, 0, 2], 6),
            (vec![-1, 0, 2, -2], 4),
            (vec![0, 2, -2, 0], 4),
            (vec![2, -2, 0, -2], 2),
        ]);

        let result = windows_to_buy_value(&secret_number, 10);

        assert_eq!(result, expected);
    }
}
