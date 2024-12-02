#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/1-input.txt")?;

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = file
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

    left.sort_unstable();
    right.sort_unstable();

    let mut result = 0;
    for (a, b) in left.iter().zip(right.iter()) {
        result += (a - b).abs();
    }

    println!("result: {result}");

    Ok(())
}
