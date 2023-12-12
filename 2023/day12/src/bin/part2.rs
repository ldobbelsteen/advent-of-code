#![warn(clippy::pedantic)]
#![allow(clippy::bool_to_int_with_if)]

use anyhow::Result;
use hashbrown::HashMap;
use itertools::Itertools;
use std::{fs, iter};

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
            _ => panic!("unexpected spring char: {s}"),
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
        #[derive(Debug, PartialEq, Eq, Hash)]
        struct Params<'a> {
            springs: &'a [Spring],
            group_sizes: &'a [u64],
            group_head_depth: u64,
        }

        fn count_recursive<'a>(p: Params<'a>, cache: &mut HashMap<Params<'a>, u64>) -> u64 {
            // assert!(p.group_sizes.len() > 0 || p.group_head_depth == 0);
            // assert!(p.group_sizes.len() == 0 || p.group_head_depth <= p.group_sizes[0]);

            // Check if result with current parameters is already in cache.
            if let Some(cached) = cache.get(&p) {
                return *cached;
            }

            if p.springs.is_empty() {
                return if p.group_head_depth > 0 {
                    if p.group_sizes[0] == p.group_head_depth {
                        if p.group_sizes.len() > 1 {
                            0 // there is still a group after this, so fail
                        } else {
                            1 // this was the last group, so success
                        }
                    } else {
                        0 // group is unfinished while at the end, so fail
                    }
                } else if !p.group_sizes.is_empty() {
                    0 // remaining groups with no remaining springs, so fail
                } else {
                    1 // success
                };
            }

            let mut result = 0;

            // If spring is operational or unknown, add result assuming this spring is operational.
            if p.springs[0] != Spring::Damaged {
                result += if p.group_head_depth > 0 {
                    if p.group_sizes[0] == p.group_head_depth {
                        count_recursive(
                            Params {
                                springs: &p.springs[1..],
                                group_sizes: &p.group_sizes[1..],
                                group_head_depth: 0,
                            },
                            cache,
                        ) // group finished, continue with next group
                    } else {
                        0 // group ends while not finished, so fail
                    }
                } else {
                    count_recursive(
                        Params {
                            springs: &p.springs[1..],
                            group_sizes: p.group_sizes,
                            group_head_depth: 0,
                        },
                        cache,
                    ) // simply continue
                };
            }

            // If spring is damaged or unknown, add result assuming this spring is damaged.
            if p.springs[0] != Spring::Operational {
                result += if p.group_sizes.is_empty() || p.group_sizes[0] == p.group_head_depth {
                    0 // no groups expected anymore or group finished while not ending, so fail
                } else {
                    count_recursive(
                        Params {
                            springs: &p.springs[1..],
                            group_sizes: p.group_sizes,
                            group_head_depth: p.group_head_depth + 1,
                        },
                        cache,
                    ) // continue with part of the group
                };
            }

            cache.insert(p, result);
            result
        }

        count_recursive(
            Params {
                springs: self.springs.as_slice(),
                group_sizes: self.group_sizes.as_slice(),
                group_head_depth: 0,
            },
            &mut HashMap::new(),
        )
    }
}

fn main() -> Result<()> {
    let copies = 5;
    let file = fs::read_to_string("input.txt")?;
    let result = file
        .lines()
        .map(|line| {
            let (springs_raw, group_sizes_raw) = line.split_once(' ').unwrap();
            let springs: Vec<Spring> = iter::repeat(springs_raw)
                .take(copies)
                .collect_vec()
                .join("?")
                .chars()
                .map(Spring::from_char)
                .collect();
            let group_sizes: Vec<u64> = iter::repeat(group_sizes_raw)
                .take(copies)
                .collect_vec()
                .join(",")
                .split(',')
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
