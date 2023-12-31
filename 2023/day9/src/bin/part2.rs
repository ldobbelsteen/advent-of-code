#![warn(clippy::pedantic)]

use anyhow::Result;
use std::fs;

fn extrapolate_history_backwards(values: &[i64]) -> Result<i64> {
    if values.iter().all(|v| *v == 0) {
        return Ok(0);
    }
    let mut child = Vec::new();
    for i in 1..values.len() {
        child.push(values[i] - values[i - 1]);
    }
    Ok(values.first().unwrap() - extrapolate_history_backwards(&child)?)
}

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let result = file
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| {
                    let v = s.parse()?;
                    Ok(v)
                })
                .collect::<Result<Vec<_>>>()
                .and_then(|vs| extrapolate_history_backwards(&vs))
        })
        .sum::<Result<i64>>()?;
    println!("{result}");
    Ok(())
}
