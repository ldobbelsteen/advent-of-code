#![warn(clippy::pedantic)]

use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Clone, Debug)]
struct RatingRange {
    start: u64,
    end: u64, // inclusive
}

impl RatingRange {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn size(&self) -> u64 {
        if self.end < self.start {
            0
        } else {
            self.end - self.start + 1
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn from_str(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("invalid category"),
        }
    }
}

#[derive(Debug)]
struct Condition {
    category: Category,
    greater_than: bool, // if false, smaller than
    value: u64,
}

impl Condition {
    fn from_str(s: &str) -> Self {
        if let Some((category_raw, value_raw)) = s.split_once('>') {
            Self {
                category: Category::from_str(category_raw),
                greater_than: true,
                value: value_raw.parse().unwrap(),
            }
        } else {
            let (category_raw, value_raw) = s.split_once('<').unwrap();
            Self {
                category: Category::from_str(category_raw),
                greater_than: false,
                value: value_raw.parse().unwrap(),
            }
        }
    }

    #[allow(clippy::range_plus_one)]
    fn split_ranges(&self, ranges: &RatingRanges) -> (RatingRanges, RatingRanges) {
        let range = ranges.inner.get(&self.category).unwrap();

        let (range_with_condition, range_without_condition) = if self.greater_than {
            (
                RatingRange::new(self.value + 1, range.end),
                RatingRange::new(range.start, self.value),
            )
        } else {
            (
                RatingRange::new(range.start, self.value - 1),
                RatingRange::new(self.value, range.end),
            )
        };

        let mut with_condition = ranges.clone();
        let mut without_condition = ranges.clone();

        with_condition
            .inner
            .insert(self.category.clone(), range_with_condition);
        without_condition
            .inner
            .insert(self.category.clone(), range_without_condition);

        (with_condition, without_condition)
    }
}

#[derive(Debug)]
struct Rule<'a> {
    condition: Option<Condition>,
    target: &'a str,
}

impl<'a> Rule<'a> {
    fn from_str(s: &'a str) -> Self {
        if s.contains(':') {
            let (condition_raw, target) = s.split_once(':').unwrap();
            Rule {
                condition: Some(Condition::from_str(condition_raw)),
                target,
            }
        } else {
            Rule {
                condition: None,
                target: s,
            }
        }
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
}

impl<'a> Workflow<'a> {
    fn from_line(s: &'a str) -> Self {
        let re = Regex::new(r"(.+)\{(.+)\}").unwrap();
        let captures = re.captures(s).unwrap();
        let name = captures.get(1).unwrap().as_str();

        let rules_raw = captures.get(2).unwrap().as_str();
        let rules = rules_raw.split(',').map(Rule::from_str).collect();

        Self { name, rules }
    }
}

#[derive(Debug)]
struct Workflows<'a> {
    inner: HashMap<&'a str, Workflow<'a>>,
}

impl<'a> Workflows<'a> {
    fn from_lines<I>(lines: I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        Self {
            inner: lines
                .map(Workflow::from_line)
                .map(|w| (w.name, w))
                .collect(),
        }
    }

    fn acceptance_combinations(&self, start: &str, mut ranges: RatingRanges) -> u64 {
        if start == "A" {
            return ranges.combinations();
        }
        if start == "R" {
            return 0;
        }

        let mut result = 0;
        for rule in &self.inner.get(start).unwrap().rules {
            if let Some(condition) = &rule.condition {
                let split = condition.split_ranges(&ranges);
                result += self.acceptance_combinations(rule.target, split.0);
                ranges = split.1;
            } else {
                return result + self.acceptance_combinations(rule.target, ranges);
            }
        }

        panic!("ruleset did not end with conditionless rule");
    }
}

#[derive(Clone, Debug)]
struct RatingRanges {
    inner: HashMap<Category, RatingRange>,
}

impl RatingRanges {
    fn full() -> Self {
        let mut inner = HashMap::new();
        inner.insert(Category::X, RatingRange::new(1, 4000));
        inner.insert(Category::M, RatingRange::new(1, 4000));
        inner.insert(Category::A, RatingRange::new(1, 4000));
        inner.insert(Category::S, RatingRange::new(1, 4000));
        Self { inner }
    }

    fn combinations(&self) -> u64 {
        self.inner.values().map(RatingRange::size).product()
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let workflows = Workflows::from_lines(file.lines().take_while(|s| !s.is_empty()));
    let result = workflows.acceptance_combinations("in", RatingRanges::full());
    println!("{result}");
}
