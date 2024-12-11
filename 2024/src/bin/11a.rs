#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let blinks = 25;
    let file = std::fs::read_to_string("inputs/11-input.txt")?;

    let mut stones = file
        .strip_suffix('\n')
        .ok_or(anyhow!("no newline at end of file"))?
        .split_whitespace()
        .map(|s| Ok(s.parse::<u64>()?))
        .collect::<Result<Vec<_>>>()?;

    for _ in 0..blinks {
        for stone in std::mem::take(&mut stones) {
            if stone == 0 {
                stones.push(1);
            } else {
                let digits = stone.ilog10() + 1;
                if digits % 2 == 0 {
                    let half = digits / 2;
                    let mask = 10u64.pow(half);
                    let first_half = stone / mask;
                    let second_half = stone % mask;
                    stones.push(first_half);
                    stones.push(second_half);
                } else {
                    stones.push(stone * 2024);
                }
            }
        }
    }

    let result = stones.len();
    println!("result: {result}");

    Ok(())
}
