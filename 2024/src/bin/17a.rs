#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use regex::Regex;

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/17-input.txt")?;
    let re = Regex::new(
        r"Register A: ([0-9]+)\nRegister B: ([0-9]+)\nRegister C: ([0-9]+)\n\nProgram: ([0-9,]+)",
    )?;

    let caps = re.captures(&file).ok_or(anyhow!("invalid input"))?;
    let mut a: u32 = caps[1].parse()?;
    let mut b: u32 = caps[2].parse()?;
    let mut c: u32 = caps[3].parse()?;
    let program = caps[4]
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<u32>, _>>()?;

    let mut outputs = vec![];

    let mut pointer = 0;
    while pointer < program.len() - 1 {
        let opcode = program[pointer];
        let literal_operand = program[pointer + 1];
        let combo_operand = match literal_operand {
            0..=3 => literal_operand,
            4 => a,
            5 => b,
            6 => c,
            _ => return Err(anyhow!("invalid combo operand")),
        };

        match opcode {
            0 => {
                let numerator = a;
                let denumerator = 2u32.pow(combo_operand);
                a = numerator / denumerator;
            }
            1 => {
                b ^= literal_operand;
            }
            2 => {
                b = combo_operand % 8;
            }
            3 => {
                if a != 0 {
                    pointer = literal_operand.try_into()?;
                    continue;
                }
            }
            4 => {
                b ^= c;
            }
            5 => {
                let output = combo_operand % 8;
                outputs.push(output);
            }
            6 => {
                let numerator = a;
                let denumerator = 2u32.pow(combo_operand);
                b = numerator / denumerator;
            }
            7 => {
                let numerator = a;
                let denumerator = 2u32.pow(combo_operand);
                c = numerator / denumerator;
            }
            _ => return Err(anyhow!("invalid opcode")),
        }

        pointer += 2;
    }

    let result = outputs
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",");
    println!("result: {result}");

    Ok(())
}
