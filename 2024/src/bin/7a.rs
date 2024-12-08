#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

fn is_possible_with_operators_rec(target: u64, current: u64, values: &[u64]) -> bool {
    // We can kill this branch early if we know we've gone over the target.
    if current > target {
        return false;
    }

    if values.is_empty() {
        return target == current;
    }

    let value = values[0];
    let rest = &values[1..];

    is_possible_with_operators_rec(target, current * value, rest)
        || is_possible_with_operators_rec(target, current + value, rest)
}

fn is_possible_with_operators(target: u64, values: &[u64]) -> bool {
    is_possible_with_operators_rec(target, 0, values)
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/7-input.txt")?;

    let equations = file
        .lines()
        .map(|line| {
            let (total_raw, values_raw) = line
                .split_once(": ")
                .ok_or_else(|| anyhow!("invalid equation line"))?;

            let total = total_raw.parse::<u64>()?;
            let values = values_raw
                .split_whitespace()
                .map(|v| Ok(v.parse::<u64>()?))
                .collect::<Result<Vec<_>>>()?;

            Ok((total, values))
        })
        .collect::<Result<Vec<_>>>()?;

    let mut result = 0;
    for (total, values) in equations {
        if is_possible_with_operators(total, &values) {
            result += total;
        }
    }

    println!("{result}");

    Ok(())
}
