use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct PageOrderingRules {
    rules: HashMap<u32, PageOrderingRule>,
}

impl PageOrderingRules {
    pub fn new() -> Self {
        PageOrderingRules {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule_from_line(&mut self, line: PageOrderingRuleLine) -> bool {
        self.rules
            .entry(line.before_page_number)
            .or_insert(PageOrderingRule::new(line.before_page_number))
            .add_page_after(line.after_page_number)
    }

    pub fn pages_before_is_valid(&self, page_number: u32, pages_before: &HashSet<u32>) -> bool {
        self.rules
            .get(&page_number)
            .is_none_or(|rule| !rule.contains_after_pages_in(pages_before))
    }

    pub fn conflicting_pages(&self, page_number: u32, pages_before: &HashSet<u32>) -> HashSet<u32> {
        self.rules.get(&page_number).map_or(HashSet::new(), |rule| {
            rule.after_pages_contained_in(pages_before)
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct PageOrderingRule {
    page_number: u32,
    pages_after: HashSet<u32>,
}

impl PageOrderingRule {
    pub fn new(page_number: u32) -> Self {
        PageOrderingRule {
            page_number,
            pages_after: HashSet::new(),
        }
    }

    pub fn add_page_after(&mut self, page_number: u32) -> bool {
        self.pages_after.insert(page_number)
    }

    pub fn contains_after_pages_in(&self, page_numbers: &HashSet<u32>) -> bool {
        !self.pages_after.is_disjoint(page_numbers)
    }

    pub fn after_pages_contained_in(&self, page_numbers: &HashSet<u32>) -> HashSet<u32> {
        self.pages_after
            .intersection(page_numbers)
            .copied()
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePageOrderingRuleError;

impl FromStr for PageOrderingRule {
    type Err = ParsePageOrderingRuleError;

    fn from_str(page_number_str: &str) -> Result<Self, Self::Err> {
        page_number_str
            .parse()
            .map(PageOrderingRule::new)
            .map_err(|_| ParsePageOrderingRuleError)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PageOrderingRuleLine {
    before_page_number: u32,
    after_page_number: u32,
}

impl PageOrderingRuleLine {
    pub fn new(before_page_number: u32, after_page_number: u32) -> Self {
        PageOrderingRuleLine {
            before_page_number,
            after_page_number,
        }
    }

    fn from_str_parts(before: &str, after: &str) -> Result<Self, ParsePageOrderingRuleLineError> {
        match (before.parse(), after.parse()) {
            (Ok(before_num), Ok(after_num)) => Ok(PageOrderingRuleLine::new(before_num, after_num)),
            _ => Err(ParsePageOrderingRuleLineError),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParsePageOrderingRuleLineError;

impl FromStr for PageOrderingRuleLine {
    type Err = ParsePageOrderingRuleLineError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        line.split_once('|')
            .ok_or(ParsePageOrderingRuleLineError)
            .and_then(|(before, after)| PageOrderingRuleLine::from_str_parts(before, after))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_ordering_rule_line_from_str() {
        let expected_ok = Ok(PageOrderingRuleLine::new(32, 12));
        let expected_err = Err(ParsePageOrderingRuleLineError);

        let result_ok = "32|12".parse();
        let result_err = "t2|12".parse::<PageOrderingRuleLine>();

        assert_eq!(result_ok, expected_ok);
        assert_eq!(result_err, expected_err);
    }
    #[test]
    fn test_page_ordering_rule_from_str() {
        let expected_ok = Ok(PageOrderingRule::new(32));
        let expected_err = Err(ParsePageOrderingRuleError);

        let result_ok = "32".parse();
        let result_err = "t2".parse::<PageOrderingRule>();

        assert_eq!(result_ok, expected_ok);
        assert_eq!(result_err, expected_err);
    }
}
