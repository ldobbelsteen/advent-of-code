#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use regex::Regex;

fn number_of_digits(n: u64) -> u32 {
    n.ilog10() + 1
}

fn split_digits_at(n: u64, idx: u32) -> (u64, u64) {
    let divisor = 10u64.pow(idx);
    (n / divisor, n % divisor)
}

fn is_invalid(product_id: u64) -> bool {
    let digits = number_of_digits(product_id);
    if !digits.is_multiple_of(2) {
        return false;
    }

    let middle = digits / 2;
    let (first_half, second_half) = split_digits_at(product_id, middle);
    first_half == second_half
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/2-input.txt")?;

    let mut result = 0;
    let range_re = Regex::new(r"(\d+)-(\d+)")?;
    for raw_range in file.split(',') {
        let caps = range_re
            .captures(raw_range)
            .ok_or_else(|| anyhow!("invalid range: {raw_range}"))?;
        let start: u64 = caps[1].parse()?;
        let end: u64 = caps[2].parse()?;

        for product_id in start..=end {
            if is_invalid(product_id) {
                result += product_id;
            }
        }
    }

    println!("{result}");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_digits() {
        assert_eq!(number_of_digits(1), 1);
        assert_eq!(number_of_digits(5), 1);
        assert_eq!(number_of_digits(12), 2);
        assert_eq!(number_of_digits(100), 3);
        assert_eq!(number_of_digits(999), 3);
        assert_eq!(number_of_digits(1000), 4);
        assert_eq!(number_of_digits(1_234_567_890), 10);
    }

    #[test]
    fn test_split_digits_at() {
        assert_eq!(split_digits_at(123_456, 3), (123, 456));
        assert_eq!(split_digits_at(9_876_543_211, 5), (98765, 43211));
        assert_eq!(split_digits_at(1_188_511_885, 5), (11885, 11885));
        assert_eq!(split_digits_at(11, 1), (1, 1));
        assert_eq!(split_digits_at(12, 1), (1, 2));
    }
}
