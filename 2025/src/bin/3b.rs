#![warn(clippy::pedantic)]

use anyhow::Result;

fn max_joltage_rec(bank: &[char], acc: &mut Vec<char>, budget: usize) {
    assert!(bank.len() >= budget);

    if budget == 0 {
        return;
    }

    let mut highest_first_digit_idx = 0;
    let mut highest_first_digit = bank[0];
    for (i, &d) in bank[..=(bank.len() - budget)].iter().enumerate() {
        if d > highest_first_digit {
            highest_first_digit = d;
            highest_first_digit_idx = i;
        }
    }

    acc.push(highest_first_digit);
    max_joltage_rec(&bank[(highest_first_digit_idx + 1)..], acc, budget - 1);
}

fn max_joltage(bank: &[char], budget: usize) -> u64 {
    let mut acc = vec![];
    max_joltage_rec(bank, &mut acc, budget);
    acc.iter().collect::<String>().parse().unwrap()
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/3-input.txt")?;

    let result = file
        .lines()
        .map(|line| {
            let bank = line.chars().collect::<Vec<_>>();
            max_joltage(&bank, 12)
        })
        .sum::<u64>();
    println!("{result}");

    Ok(())
}
