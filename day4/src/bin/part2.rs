use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
struct Card {
    numbers: HashSet<u32>,
    winning: HashSet<u32>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let re = Regex::new(r"Card\s+\d+:\s+([\d\s]+)\s+\|\s+([\d\s]+)")?;
        let caps = re.captures(s).ok_or(anyhow!("no matches found: {}", s))?;

        let winning_raw = caps
            .get(1)
            .ok_or(anyhow!("no card numbers found: {}", s))?
            .as_str();
        let numbers_raw = caps
            .get(2)
            .ok_or(anyhow!("no winning numbers found: {}", s))?
            .as_str();

        let winning = winning_raw
            .split_whitespace()
            .map(|s| {
                let n = s.parse::<u32>()?;
                Ok(n)
            })
            .collect::<Result<HashSet<u32>>>()?;

        let numbers = numbers_raw
            .split_whitespace()
            .map(|s| {
                let n = s.parse::<u32>()?;
                Ok(n)
            })
            .collect::<Result<HashSet<u32>>>()?;

        Ok(Card { winning, numbers })
    }
}

impl Card {
    fn matchings(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .count()
    }
}

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let cards = file
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<Card>>>()?;

    let mut copies: HashMap<usize, u32> = HashMap::new();
    for i in 0..cards.len() {
        copies.insert(i, 1);
    }

    for (i, card) in cards.iter().enumerate() {
        let count = copies
            .get(&i)
            .ok_or(anyhow!("card not in map while iterating: {}", i))?
            .clone();
        for j in i + 1..i + 1 + card.matchings() {
            let existing = copies
                .get(&j)
                .ok_or(anyhow!("card not in map while incrementing: {}", i))?;
            copies.insert(j, existing + count);
        }
    }

    let result = copies.values().sum::<u32>();
    println!("{}", result);
    Ok(())
}
