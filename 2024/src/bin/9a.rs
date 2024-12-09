#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::iter;

fn compute_checksum(blocks: &[Option<usize>]) -> usize {
    blocks
        .iter()
        .enumerate()
        .fold(0, |checksum, (position, id)| {
            checksum + position * id.unwrap_or(0)
        })
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/9-input.txt")?;
    let input = file
        .strip_suffix('\n')
        .ok_or(anyhow::anyhow!("no newline at end of file"))?;

    let mut blocks = Vec::new();
    for (i, c) in input.chars().enumerate() {
        let id = if i % 2 == 0 { Some(i / 2) } else { None };
        let size = c.to_digit(10).ok_or(anyhow!("not a digit"))?;
        blocks.extend(iter::repeat(id).take(size.try_into()?));
    }

    let mut left_index = 0;
    let mut right_index = blocks.len() - 1;

    while left_index < right_index {
        if blocks[left_index].is_some() {
            left_index += 1;
        } else if blocks[right_index].is_none() {
            right_index -= 1;
        } else {
            blocks.swap(left_index, right_index);
            left_index += 1;
            right_index -= 1;
        }
    }

    let result = compute_checksum(&blocks);
    println!("result: {result}");

    Ok(())
}
