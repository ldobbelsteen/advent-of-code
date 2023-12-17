#![warn(clippy::pedantic)]

use anyhow::Result;
use std::fs;

fn hash(s: &str) -> u64 {
    let mut value = 0;
    for char in s.chars() {
        let ascii = char as u64;
        value += ascii;
        value *= 17;
        value %= 256;
    }
    value
}

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let result: u64 = file.split(',').map(hash).sum();
    println!("{result}");
    Ok(())
}
