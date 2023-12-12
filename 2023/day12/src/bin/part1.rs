#![warn(clippy::pedantic)]

use anyhow::Result;
use std::fs;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn from_char(s: char) -> Self {
        match s {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("unexpected spring char: {}", s),
        }
    }
}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    group_sizes: Vec<u64>,
}

impl Row {
    fn count_arrangements(&self) -> u64 {
        fn count_rec(springs: &[Spring], group_sizes: &[u64], group_head_depth: u64) -> u64 {
            assert!(group_sizes.len() > 0 || group_head_depth == 0);
            assert!(group_sizes.len() == 0 || group_head_depth <= group_sizes[0]);

            if springs.len() == 0 {
                return if group_head_depth > 0 {
                    if group_sizes[0] == group_head_depth {
                        // group is finished while at the end, so remove group
                        count_rec(springs, &group_sizes[1..], 0)
                    } else {
                        0 // group is unfinished while at the end, so fail
                    }
                } else {
                    if group_sizes.len() > 0 {
                        0 // remaining groups with no remaining springs, so fail
                    } else {
                        1 // successfully reached the end
                    }
                };
            }

            let operational = if group_head_depth > 0 {
                if group_sizes[0] == group_head_depth {
                    count_rec(&springs[1..], &group_sizes[1..], 0) // group finished, continue with next group
                } else {
                    0 // group cannot be finished, so fail
                }
            } else {
                count_rec(&springs[1..], group_sizes, 0) // simply continue
            };

            let damaged = if group_sizes.len() > 0 {
                if group_sizes[0] == group_head_depth {
                    0 // end of group is not here, so fail
                } else {
                    count_rec(&springs[1..], group_sizes, group_head_depth + 1) // take as part of current group
                }
            } else {
                0 // no groups expected anymore
            };

            match springs[0] {
                Spring::Operational => operational,
                Spring::Damaged => damaged,
                Spring::Unknown => operational + damaged,
            }
        }
        count_rec(self.springs.as_slice(), self.group_sizes.as_slice(), 0)
    }
}

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let result = file
        .lines()
        .map(|line| {
            let (springs_raw, group_sizes_raw) = line.split_once(" ").unwrap();
            let springs: Vec<Spring> = springs_raw.chars().map(Spring::from_char).collect();
            let group_sizes: Vec<u64> = group_sizes_raw
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            Row {
                springs,
                group_sizes,
            }
        })
        .map(|r| r.count_arrangements())
        .sum::<u64>();
    println!("{result}");
    Ok(())
}
