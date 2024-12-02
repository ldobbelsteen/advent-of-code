#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/1-input.txt")?;

    let (left, right): (Vec<i32>, Vec<i32>) = file
        .lines()
        .map(|line| {
            let (a, b) = line
                .split_once("   ")
                .ok_or(anyhow!("invalid line: {line}"))?;
            Ok((a.parse()?, b.parse()?))
        })
        .collect::<Result<Vec<(i32, i32)>>>()?
        .into_iter()
        .unzip();

    if left.len() != right.len() {
        return Err(anyhow!(
            "list size mismatch: left={} right={}",
            left.len(),
            right.len()
        ));
    }

    let right_occurrences = right
        .iter()
        .fold(std::collections::HashMap::new(), |mut acc, &x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });

    let mut result = 0;
    for v in &left {
        if let Some(&count) = right_occurrences.get(v) {
            result += v * count;
        }
    }

    println!("result: {result}");

    Ok(())
}
