mod safety_manual;

use crate::util::file_reader::to_string_vector;
use std::collections::HashSet;

use safety_manual::{PageOrderingRuleLine, PageOrderingRules};

pub fn run() {
    let input = to_string_vector("inputs/day_5.txt").expect("Something went wrong with Day 5!");

    let (rules, pages_to_print) = parse_data(&input);

    println!("Day 5 Part 1: {:?}", part_1(&rules, &pages_to_print));
    println!("Day 5 Part 2: {:?}", part_2(&rules, &pages_to_print));
}

fn part_1(rules: &PageOrderingRules, pages_to_print: &[Vec<u32>]) -> u32 {
    pages_to_print
        .iter()
        .filter(|line| pages_to_print_line_is_valid(rules, line))
        .map(|line| line.get(line.len() / 2).unwrap())
        .sum()
}

fn part_2(rules: &PageOrderingRules, pages_to_print: &[Vec<u32>]) -> u32 {
    pages_to_print
        .iter()
        .filter(|line| !pages_to_print_line_is_valid(rules, line))
        .map(|line| fix_line(rules, line))
        .map(|line| *line.get(line.len() / 2).unwrap())
        .sum()
}

fn pages_to_print_line_is_valid(rules: &PageOrderingRules, line: &[u32]) -> bool {
    let mut valid_so_far = true;
    let mut pages_so_far = HashSet::new();

    for &page_number in line {
        if !valid_so_far {
            break;
        }

        valid_so_far = rules.pages_before_is_valid(page_number, &pages_so_far);

        pages_so_far.insert(page_number);
    }

    valid_so_far
}

fn fix_line(rules: &PageOrderingRules, line: &[u32]) -> Vec<u32> {
    let mut result = line.to_vec();

    loop {
        let mut current_index = 0;
        let mut conflicts = HashSet::new();
        let mut pages_so_far = HashSet::new();

        for &page_number in &result {
            conflicts = rules.conflicting_pages(page_number, &pages_so_far);

            if !conflicts.is_empty() {
                break;
            }

            current_index += 1;
            pages_so_far.insert(page_number);
        }

        if conflicts.is_empty() {
            break;
        }

        let conflict_with_lowest_index = result
            .iter()
            .position(|number| conflicts.contains(number))
            .unwrap();

        let problem_page_number = result.remove(current_index);

        result.insert(conflict_with_lowest_index, problem_page_number);
    }

    result
}

fn parse_data(input: &[String]) -> (PageOrderingRules, Vec<Vec<u32>>) {
    let mut split_data = input.split(String::is_empty);

    let rules_data = split_data.next().expect("No rules data!");
    let pages_data = split_data.next().expect("No rules data!");

    (parse_rules(rules_data), parse_pages_to_print(pages_data))
}

fn parse_rules(rules: &[String]) -> PageOrderingRules {
    let mut result = PageOrderingRules::new();

    for rule_line in parse_rule_lines(rules) {
        result.add_rule_from_line(rule_line);
    }

    result
}

fn parse_rule_lines(rules: &[String]) -> Vec<PageOrderingRuleLine> {
    rules.iter().map(|line| line.parse().unwrap()).collect()
}

fn parse_pages_to_print(lines: &[String]) -> Vec<Vec<u32>> {
    lines
        .iter()
        .map(|line| {
            line.split(',')
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = to_string_vector("test_inputs/day_5.txt").unwrap();

        let (rules, pages_to_print) = parse_data(&input);

        assert_eq!(part_1(&rules, &pages_to_print), 143);
    }

    #[test]
    fn test_part_2() {
        let input = to_string_vector("test_inputs/day_5.txt").unwrap();

        let (rules, pages_to_print) = parse_data(&input);

        assert_eq!(part_2(&rules, &pages_to_print), 123);
    }

    #[test]
    fn test_pages_to_print_line_is_valid_true() {
        let input_rule_lines = [
            String::from("47|53"),
            String::from("97|13"),
            String::from("97|61"),
            String::from("97|47"),
            String::from("75|29"),
            String::from("61|13"),
            String::from("75|53"),
            String::from("29|13"),
            String::from("97|29"),
            String::from("53|29"),
            String::from("61|53"),
            String::from("97|53"),
            String::from("61|29"),
            String::from("47|13"),
            String::from("75|47"),
            String::from("97|75"),
            String::from("47|61"),
            String::from("75|61"),
            String::from("47|29"),
            String::from("75|13"),
            String::from("53|13"),
        ];

        let input_rules = parse_rules(&input_rule_lines);
        let input_pages_to_print_line = [75, 47, 61, 53, 29];

        let result = pages_to_print_line_is_valid(&input_rules, &input_pages_to_print_line);

        assert!(result);
    }

    #[test]
    fn test_pages_to_print_line_is_valid_false() {
        let input_rule_lines = [
            String::from("47|53"),
            String::from("97|13"),
            String::from("97|61"),
            String::from("97|47"),
            String::from("75|29"),
            String::from("61|13"),
            String::from("75|53"),
            String::from("29|13"),
            String::from("97|29"),
            String::from("53|29"),
            String::from("61|53"),
            String::from("97|53"),
            String::from("61|29"),
            String::from("47|13"),
            String::from("75|47"),
            String::from("97|75"),
            String::from("47|61"),
            String::from("75|61"),
            String::from("47|29"),
            String::from("75|13"),
            String::from("53|13"),
        ];

        let input_rules = parse_rules(&input_rule_lines);
        let input_pages_to_print_line = [75, 97, 47, 61, 53];

        let result = pages_to_print_line_is_valid(&input_rules, &input_pages_to_print_line);

        assert!(!result);
    }

    #[test]
    fn test_fix_line() {
        let input_rule_lines = [
            String::from("47|53"),
            String::from("97|13"),
            String::from("97|61"),
            String::from("97|47"),
            String::from("75|29"),
            String::from("61|13"),
            String::from("75|53"),
            String::from("29|13"),
            String::from("97|29"),
            String::from("53|29"),
            String::from("61|53"),
            String::from("97|53"),
            String::from("61|29"),
            String::from("47|13"),
            String::from("75|47"),
            String::from("97|75"),
            String::from("47|61"),
            String::from("75|61"),
            String::from("47|29"),
            String::from("75|13"),
            String::from("53|13"),
        ];

        let input_rules = parse_rules(&input_rule_lines);
        let input_pages_to_print_line = [75, 61, 13, 97, 29];

        let expected = vec![97, 75, 61, 29, 13];

        let result = fix_line(&input_rules, &input_pages_to_print_line);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_rule_lines() {
        let input = [
            String::from("47|53"),
            String::from("97|13"),
            String::from("97|61"),
            String::from("97|47"),
        ];

        let expected = vec![
            PageOrderingRuleLine::new(47, 53),
            PageOrderingRuleLine::new(97, 13),
            PageOrderingRuleLine::new(97, 61),
            PageOrderingRuleLine::new(97, 47),
        ];

        let result = parse_rule_lines(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_rules() {
        let input = [
            String::from("47|53"),
            String::from("97|13"),
            String::from("97|61"),
            String::from("97|47"),
        ];

        let mut expected = PageOrderingRules::new();

        expected.add_rule_from_line(PageOrderingRuleLine::new(47, 53));
        expected.add_rule_from_line(PageOrderingRuleLine::new(97, 13));
        expected.add_rule_from_line(PageOrderingRuleLine::new(97, 61));
        expected.add_rule_from_line(PageOrderingRuleLine::new(97, 47));

        let result = parse_rules(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_pages_to_print() {
        let input = [
            String::from("75,47,61,53,29"),
            String::from("97,61,53,29,13"),
        ];

        let expected = vec![vec![75, 47, 61, 53, 29], vec![97, 61, 53, 29, 13]];

        let result = parse_pages_to_print(&input);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_data() {
        let input = [
            String::from("47|53"),
            String::from("97|13"),
            String::from("97|61"),
            String::from("97|47"),
            String::new(),
            String::from("75,47,61,53,29"),
            String::from("97,61,53,29,13"),
        ];

        let mut expected_rules = PageOrderingRules::new();

        expected_rules.add_rule_from_line(PageOrderingRuleLine::new(47, 53));
        expected_rules.add_rule_from_line(PageOrderingRuleLine::new(97, 13));
        expected_rules.add_rule_from_line(PageOrderingRuleLine::new(97, 61));
        expected_rules.add_rule_from_line(PageOrderingRuleLine::new(97, 47));

        let expected_pages = vec![vec![75, 47, 61, 53, 29], vec![97, 61, 53, 29, 13]];

        let result = parse_data(&input);

        assert_eq!(result, (expected_rules, expected_pages));
    }
}
