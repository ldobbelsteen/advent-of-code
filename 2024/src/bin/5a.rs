#![warn(clippy::pedantic)]

use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};

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
    for update in updates {
        let mut is_correct = true;
        let mut current_banned: HashSet<u32> = HashSet::new();

        for value in &update {
            if current_banned.contains(value) {
                is_correct = false;
                break;
            }

            if let Some(banned) = banned_successors.get(value) {
                current_banned.extend(banned);
            }
        }

        if is_correct {
            let middle = update[update.len() / 2];
            result += middle;
        }
    }

    println!("Result: {result}");

    Ok(())
}
