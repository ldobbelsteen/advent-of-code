#![warn(clippy::pedantic)]

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/3-input.txt")?;

    let re = Regex::new(r"(?:don't\(\))|(?:do\(\))|(?:mul\(([0-9]{1,3}),([0-9]{1,3})\))")?;

    let mut result = 0;
    let mut future_enabled = true;
    for m in re.captures_iter(&file) {
        match &m[0] {
            "do()" => future_enabled = true,
            "don't()" => future_enabled = false,
            _ => {
                if future_enabled {
                    let a = m[1].parse::<i32>()?;
                    let b = m[2].parse::<i32>()?;
                    result += a * b;
                }
            }
        }
    }

    println!("result: {result}");

    Ok(())
}
