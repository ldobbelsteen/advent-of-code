#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use regex::Regex;

fn outputs_self(mut a: u64, mut b: u64, mut c: u64, program: &[u64]) -> Result<bool> {
    let mut output_index = 0;
    let mut program_index = 0;
    while program_index < program.len() - 1 {
        let opcode = program[program_index];
        let literal_operand = program[program_index + 1];
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
                let denumerator = 2u64.pow(combo_operand.try_into()?);
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
                    program_index = literal_operand.try_into()?;
                    continue;
                }
            }
            4 => {
                b ^= c;
            }
            5 => {
                let output = combo_operand % 8;
                if output_index == program.len() || output != program[output_index] {
                    return Ok(false);
                }
                output_index += 1;
            }
            6 => {
                let numerator = a;
                let denumerator = 2u64.pow(combo_operand.try_into()?);
                b = numerator / denumerator;
            }
            7 => {
                let numerator = a;
                let denumerator = 2u64.pow(combo_operand.try_into()?);
                c = numerator / denumerator;
            }
            _ => return Err(anyhow!("invalid opcode")),
        }

        program_index += 2;
    }

    Ok(output_index == program.len())
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/17-input.txt")?;
    let re = Regex::new(
        r"Register A: ([0-9]+)\nRegister B: ([0-9]+)\nRegister C: ([0-9]+)\n\nProgram: ([0-9,]+)",
    )?;

    let caps = re.captures(&file).ok_or(anyhow!("invalid input"))?;
    let b: u64 = caps[2].parse()?;
    let c: u64 = caps[3].parse()?;
    let program = caps[4]
        .split(',')
        .map(str::parse)
        .collect::<Result<Vec<u64>, _>>()?;

    for a in 0.. {
        if a % 10_000_000 == 0 {
            println!("progress: {a}");
        }
        if outputs_self(a, b, c, &program)? {
            println!("result: {a}");
            break;
        }
    }

    Ok(())
}
