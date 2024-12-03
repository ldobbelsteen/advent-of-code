#![warn(clippy::pedantic)]

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/3-input.txt")?;

    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)")?;

    let mut result = 0;
    for m in re.captures_iter(&file) {
        let a = m[1].parse::<i32>()?;
        let b = m[2].parse::<i32>()?;
        result += a * b;
    }

    println!("Result: {result}");

    Ok(())
}
