#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use regex::Regex;

#[derive(Debug)]
struct Instruction {
    clockwise: bool,
    count: i32,
}

impl Instruction {
    fn from_str(s: &str) -> Result<Self> {
        let re = Regex::new(r"^([RL])(\d+)$")?;
        let caps = re.captures(s).ok_or(anyhow!("invalid instruction: {s}"))?;
        let clockwise = match &caps[1] {
            "R" => true,
            "L" => false,
            _ => return Err(anyhow!("invalid direction: {}", &caps[1])),
        };
        let count = caps[2].parse()?;
        Ok(Self { clockwise, count })
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/1-input.txt")?;
    let instructions = file
        .lines()
        .map(Instruction::from_str)
        .collect::<Result<Vec<Instruction>>>()?;

    let dial_size = 100;
    let mut position: i32 = 50;
    let mut result = 0;
    for instruction in instructions {
        if instruction.clockwise {
            let distance_to_zero = dial_size - position;
            if instruction.count >= distance_to_zero {
                result += if distance_to_zero > 0 { 1 } else { 0 };
                result += (instruction.count - distance_to_zero) / dial_size;
            }
        } else {
            let distance_to_zero = position;
            if instruction.count >= distance_to_zero {
                result += if distance_to_zero > 0 { 1 } else { 0 };
                result += (instruction.count - distance_to_zero) / dial_size;
            }
        }
        
        let count = instruction.count % dial_size;
        if instruction.clockwise {
            position += count;
        } else {
            position -= count;
        }
        if position >= dial_size {
            position -= dial_size;
        } else if position < 0 {
            position += dial_size;
        }
    }

    println!("Result: {}", result);
    Ok(())
}
