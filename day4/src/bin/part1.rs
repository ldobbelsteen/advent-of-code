use anyhow::{anyhow, Result};
use regex::Regex;
use std::collections::HashSet;
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
    fn compute_points(&self) -> u32 {
        let mut points = 0;
        for number in &self.numbers {
            if self.winning.contains(number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        points
    }
}

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt")?;

    let result = file
        .lines()
        .map(|line| {
            let card = Card::from_str(line)?;
            let points = card.compute_points();
            Ok(points)
        })
        .sum::<Result<u32>>()?;

    println!("{}", result);
    Ok(())
}
