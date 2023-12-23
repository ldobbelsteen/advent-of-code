#![warn(clippy::pedantic)]

use regex::Regex;
use std::{collections::HashMap, fs};

#[derive(Debug, PartialEq, Eq, Hash)]
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
    value: u32,
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

    fn holds(&self, rating: &Ratings) -> bool {
        if let Some(condition) = &self.condition {
            if let Some(value) = rating.0.get(&condition.category) {
                if condition.greater_than {
                    *value > condition.value
                } else {
                    *value < condition.value
                }
            } else {
                false
            }
        } else {
            true
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

    fn apply(&self, ratings: &Ratings) -> &str {
        for rule in &self.rules {
            if rule.holds(ratings) {
                return rule.target;
            }
        }
        panic!("no rules hold")
    }
}

#[derive(Debug)]
struct Ratings(HashMap<Category, u32>);

impl Ratings {
    fn from_line(s: &str) -> Self {
        let mut result = Self(HashMap::new());

        let re = Regex::new(r"\{x=([0-9]+),m=([0-9]+),a=([0-9]+),s=([0-9]+)\}").unwrap();
        let captures = re.captures(s).unwrap();

        result.0.insert(
            Category::X,
            captures.get(1).unwrap().as_str().parse().unwrap(),
        );
        result.0.insert(
            Category::M,
            captures.get(2).unwrap().as_str().parse().unwrap(),
        );
        result.0.insert(
            Category::A,
            captures.get(3).unwrap().as_str().parse().unwrap(),
        );
        result.0.insert(
            Category::S,
            captures.get(4).unwrap().as_str().parse().unwrap(),
        );

        result
    }

    fn sum(&self) -> u32 {
        self.0.values().sum()
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let mut lines = file.lines();

    let mut workflows = HashMap::new();
    for line in lines.by_ref().take_while(|s| !s.is_empty()) {
        let workflow = Workflow::from_line(line);
        workflows.insert(workflow.name, workflow);
    }

    let result: u32 = lines
        .map(Ratings::from_line)
        .filter(|ratings| {
            let mut workflow_name = "in";
            while workflow_name != "A" && workflow_name != "R" {
                workflow_name = workflows.get(workflow_name).unwrap().apply(ratings);
            }
            workflow_name == "A"
        })
        .map(|ratings| ratings.sum())
        .sum();

    println!("{result}");
}
