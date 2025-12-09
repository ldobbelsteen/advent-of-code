#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::cmp::{max, min};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Range {
    start: u64,
    end: u64, // inclusive
}

impl Range {
    fn merge(&self, other: &Self) -> Option<Self> {
        if self.end + 1 < other.start || other.end + 1 < self.start {
            return None;
        }

        let start = min(self.start, other.start);
        let end = max(self.end, other.end);
        Some(Range { start, end })
    }

    fn size(&self) -> u64 {
        self.end - self.start + 1
    }

    fn from_str(s: &str) -> Result<Self> {
        let (start_raw, end_raw) = s
            .split_once('-')
            .ok_or_else(|| anyhow!("no dash in range string: {s}"))?;

        let start: u64 = start_raw.parse()?;
        let end: u64 = end_raw.parse()?;

        Ok(Self { start, end })
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start).then(self.end.cmp(&other.end))
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/5-input.txt")?;

    let (ranges_raw, _) = file
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("no double newline in input"))?;

    let mut ranges = ranges_raw
        .lines()
        .map(Range::from_str)
        .collect::<Result<Vec<_>>>()?;
    ranges.sort_unstable();

    let mut ranges_merged = vec![];
    let mut current_range = ranges[0];
    for range in ranges {
        if let Some(merged) = current_range.merge(&range) {
            current_range = merged;
        } else {
            ranges_merged.push(current_range);
            current_range = range;
        }
    }
    ranges_merged.push(current_range);

    let result = ranges_merged.iter().map(Range::size).sum::<u64>();
    println!("{result}");
    Ok(())
}
