#![warn(clippy::pedantic)]

use anyhow::Result;

fn max_joltage(bank: &[char]) -> u64 {
    assert!(bank.len() >= 2);

    let (highest_first_digit_idx, highest_first_digit) = bank[..bank.len() - 1]
        .iter()
        .enumerate()
        .reduce(|a, b| if b.1 > a.1 { b } else { a })
        .map(|(i, d)| (i, *d))
        .unwrap();

    let highest_second_digit = bank[highest_first_digit_idx + 1..]
        .iter()
        .max()
        .copied()
        .unwrap();

    let result_str = format!("{highest_first_digit}{highest_second_digit}");
    result_str.parse().unwrap()
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/3-input.txt")?;

    let result = file
        .lines()
        .map(|line| {
            let bank = line.chars().collect::<Vec<_>>();
            max_joltage(&bank)
        })
        .sum::<u64>();
    println!("{result}");

    Ok(())
}
