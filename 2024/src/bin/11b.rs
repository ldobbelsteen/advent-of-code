#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::collections::HashMap;

struct CachedStoneCounter {
    memo: HashMap<u64, HashMap<u64, u64>>,
}

impl CachedStoneCounter {
    fn new() -> Self {
        Self {
            memo: HashMap::new(),
        }
    }

    fn stone_count(&mut self, n: u64, blinks: u64) -> u64 {
        // Without blinks, no new stones can be created.
        if blinks == 0 {
            return 1;
        }

        // Check cache for result.
        if let Some(m) = self.memo.get(&n) {
            if let Some(c) = m.get(&blinks) {
                return *c;
            }
        }

        // Case distinction according to rules.
        let result = if n == 0 {
            self.stone_count(1, blinks - 1)
        } else {
            let digits = n.ilog10() + 1; // no. digits
            if digits % 2 == 0 {
                let half = digits / 2;
                let mask = 10u64.pow(half);
                let first_half = n / mask; // first half of digits
                let second_half = n % mask; // second half of digits
                self.stone_count(first_half, blinks - 1) + self.stone_count(second_half, blinks - 1)
            } else {
                self.stone_count(n * 2024, blinks - 1)
            }
        };

        // Store result in cache and return.
        self.memo.entry(n).or_default().insert(blinks, result);
        result
    }
}

fn main() -> Result<()> {
    let blinks = 75;
    let file = std::fs::read_to_string("inputs/11-input.txt")?;

    let start_stones = file
        .strip_suffix('\n')
        .ok_or(anyhow!("no newline at end of file"))?
        .split_whitespace()
        .map(|s| Ok(s.parse::<u64>()?))
        .collect::<Result<Vec<_>>>()?;

    let mut result = 0;
    let mut counter = CachedStoneCounter::new();
    for stone in start_stones {
        result += counter.stone_count(stone, blinks);
    }

    println!("result: {result}");

    Ok(())
}
