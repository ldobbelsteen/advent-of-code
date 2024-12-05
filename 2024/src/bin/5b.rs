#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/5-input.txt")?;
    let (ordering_rules_raw, updates_raw) =
        file.split_once("\n\n").ok_or(anyhow!("invalid input"))?;

    // Collect the ordering rules into a list of pairs.
    let ordering_rules = ordering_rules_raw
        .lines()
        .map(|line| {
            let (left_raw, right_raw) = line.split_once('|').ok_or(anyhow!("invalid input"))?;
            Ok((left_raw.parse()?, right_raw.parse()?))
        })
        .collect::<Result<Vec<(u32, u32)>>>()?;

    // Collect the updates into a vector of vectors.
    let updates = updates_raw
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| Ok(s.parse::<u32>()?))
                .collect::<Result<Vec<u32>>>()
        })
        .collect::<Result<Vec<_>>>()?;

    // Convert the ordering rules into mapped sets of banned values. For a key,
    // the value represents the set of values that are not allowed to come after
    // the key according to the rules.
    let banned_successors: HashMap<u32, HashSet<u32>> =
        ordering_rules
            .iter()
            .fold(HashMap::new(), |mut acc, (left, right)| {
                acc.entry(*right).or_default().insert(*left);
                acc
            });

    let mut result = 0;
    for mut update in updates {
        let mut change_made = false;

        loop {
            let mut to_swap: Option<(usize, usize)> = None;
            let mut banned_per_index: Vec<Option<&HashSet<u32>>> = Vec::new();

            for (vi, value) in update.iter().enumerate() {
                for (bi, banned) in banned_per_index.iter().enumerate() {
                    if let Some(banned) = banned {
                        if banned.contains(value) {
                            to_swap = Some((bi, vi));
                            break;
                        }
                    }
                }

                banned_per_index.push(banned_successors.get(value));
            }

            if let Some((left, right)) = to_swap {
                update.swap(left, right);
                change_made = true;
            } else {
                break;
            }
        }

        if change_made {
            let middle = update[update.len() / 2];
            result += middle;
        }
    }

    println!("Result: {result}");

    Ok(())
}
