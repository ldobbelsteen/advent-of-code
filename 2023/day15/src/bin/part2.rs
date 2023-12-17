#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use regex::Regex;
use std::{fs, str::FromStr, vec};

#[derive(Debug)]
enum Operation {
    Insert(u64),
    Remove,
}

#[derive(Debug)]
struct Step {
    label: String,
    operation: Operation,
}

impl FromStr for Step {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let re = Regex::new(r"([ -~]+)(?:-|(?:=([0-9]+)))")?;
        let caps = re.captures(s).ok_or(anyhow!("invalid step: {}", s))?;

        let label = caps.get(1).unwrap().as_str().to_owned();
        if let Some(focal_length_raw) = caps.get(2) {
            let focal_length = focal_length_raw.as_str().parse()?;
            Ok(Self {
                label,
                operation: Operation::Insert(focal_length),
            })
        } else {
            Ok(Self {
                label,
                operation: Operation::Remove,
            })
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Lens {
    label: String,
    focal_length: u64,
}

#[derive(Clone, Debug)]
struct Box {
    lenses: Vec<Lens>,
}

impl Box {
    fn remove_lens(&mut self, lens_label: &str) {
        self.lenses.retain(|lens| lens.label != lens_label);
    }

    fn insert_lens(&mut self, lens: Lens) {
        if let Some(existing) = self.lenses.iter_mut().find(|ex| *ex.label == lens.label) {
            *existing = lens;
        } else {
            self.lenses.push(lens);
        }
    }
}

#[derive(Debug)]
struct Boxes {
    inner: Vec<Box>,
}

impl Boxes {
    fn new() -> Self {
        Self {
            inner: vec![Box { lenses: vec![] }; 256],
        }
    }

    fn apply(&mut self, step: &Step) {
        let hash = label_hash(&step.label);
        let b = &mut self.inner[hash];
        match step.operation {
            Operation::Remove => b.remove_lens(&step.label),
            Operation::Insert(focal_length) => b.insert_lens(Lens {
                label: step.label.clone(),
                focal_length,
            }),
        }
    }

    fn focusing_power(&self) -> u64 {
        let mut result = 0;
        for (i, b) in self.inner.iter().enumerate() {
            let box_number = (i + 1) as u64;
            for (j, l) in b.lenses.iter().enumerate() {
                let slot_number = (j + 1) as u64;
                result += box_number * slot_number * l.focal_length;
            }
        }
        result
    }
}

fn label_hash(s: &str) -> usize {
    let mut value = 0;
    for char in s.chars() {
        let ascii = char as u8;
        value += ascii as usize;
        value *= 17;
        value %= 256;
    }
    value
}

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let mut boxes = Boxes::new();

    for step in file.split(',').map(Step::from_str) {
        let step = step?;
        boxes.apply(&step);
    }

    let result = boxes.focusing_power();
    println!("{result}");
    Ok(())
}
