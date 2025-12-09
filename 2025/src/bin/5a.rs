#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

fn in_range(range: &(u64, u64), ingredient: u64) -> bool {
    ingredient >= range.0 && ingredient <= range.1
}

fn in_any_range(ranges: &[(u64, u64)], ingredient: u64) -> bool {
    for range in ranges {
        if in_range(range, ingredient) {
            return true;
        }
    }
    false
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/5-input.txt")?;

    let (ranges_raw, ingredients_raw) = file
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("no double newline in input"))?;

    let ranges = ranges_raw
        .lines()
        .map(|line| {
            let (start_raw, end_raw) = line
                .split_once('-')
                .ok_or_else(|| anyhow!("no dash in range line: {line}"))?;

            let start: u64 = start_raw.parse()?;
            let end: u64 = end_raw.parse()?;

            Ok((start, end))
        })
        .collect::<Result<Vec<_>>>()?;

    let ingredients = ingredients_raw
        .lines()
        .map(|line| {
            let ingredient: u64 = line.parse()?;
            Ok(ingredient)
        })
        .collect::<Result<Vec<_>>>()?;

    let mut result = 0;
    for ingredient in ingredients {
        if in_any_range(&ranges, ingredient) {
            result += 1;
        }
    }

    println!("{result}");
    Ok(())
}
