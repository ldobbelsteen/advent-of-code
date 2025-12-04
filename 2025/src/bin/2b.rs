#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use regex::Regex;

/// Returns the number of digits in the number `n`. Assumes `n` is greater than 0.
fn number_of_digits(n: u64) -> u32 {
    debug_assert!(n > 0);
    n.ilog10() + 1
}

/// Splits the number `n` into two parts at the digit index `idx` (from the right).
/// Returns a tuple `(left_part, right_part)`. Assumes `n` has at least `idx + 1` digits.
fn split_digits_at(n: u64, idx: u32) -> (u64, u64) {
    debug_assert!(number_of_digits(n) > idx);
    let divisor = 10u64.pow(idx);
    (n / divisor, n % divisor)
}

/// Returns an iterator that yields chunks of digits of size `chunk_size` from the number `n`,
/// starting from the least significant digits. Assumes `n` has a number of digits that is
/// a multiple of `chunk_size`.
fn chunked_digit_iter<'a>(mut n: u64, chunk_size: u32) -> impl Iterator<Item = u64> + 'a {
    let mut digits = number_of_digits(n);
    debug_assert!(chunk_size > 0 && digits.is_multiple_of(chunk_size));
    std::iter::from_fn(move || {
        if digits == 0 {
            return None;
        }
        if digits == chunk_size {
            digits = 0;
            return Some(n);
        }
        let (left, right) = split_digits_at(n, chunk_size);
        n = left;
        digits -= chunk_size;
        Some(right)
    })
}

fn is_invalid(product_id: u64) -> bool {
    let digits = number_of_digits(product_id);

    for repetitions in 2..=digits {
        if !digits.is_multiple_of(repetitions) {
            continue;
        }

        let chunk_size = digits / repetitions;
        let mut chunk_iter = chunked_digit_iter(product_id, chunk_size);
        let first_chunk = chunk_iter.next().unwrap();
        if chunk_iter.all(|chunk| chunk == first_chunk) {
            return true;
        }
    }

    false
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
        assert_eq!(split_digits_at(10, 1), (1, 0));
    }

    #[test]
    fn test_is_invalid() {
        assert!(is_invalid(11));
        assert!(is_invalid(1212));
        assert!(is_invalid(123_123));
        assert!(is_invalid(9999));
    }
}
