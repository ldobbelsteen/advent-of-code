#![warn(clippy::pedantic)]

use anyhow::Result;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    button_a_x: i64,
    button_a_y: i64,
    button_b_x: i64,
    button_b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

impl Machine {
    fn min_prize_cost(&self, a_cost: i64, b_cost: i64) -> Option<i64> {
        let b_numerator = self.prize_y * self.button_a_x - self.prize_x * self.button_a_y;
        let b_denominator = self.button_b_y * self.button_a_x - self.button_b_x * self.button_a_y;
        if b_denominator == 0 || b_numerator % b_denominator != 0 {
            return None;
        }

        let b = b_numerator / b_denominator;
        if b < 0 {
            return None;
        }

        let a_numerator = self.prize_x - b * self.button_b_x;
        let a_denominator = self.button_a_x;
        if a_denominator == 0 || a_numerator % a_denominator != 0 {
            return None;
        }

        let a = a_numerator / a_denominator;
        if a < 0 {
            return None;
        }

        Some(a * a_cost + b * b_cost)
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/13-input.txt")?;

    let re = Regex::new(
        r"Button A: X\+([0-9]+), Y\+([0-9]+)\nButton B: X\+([0-9]+), Y+\+([0-9]+)\nPrize: X=([0-9]+), Y=([0-9]+)\n",
    )?;

    let machines = re
        .captures_iter(&file)
        .map(|cap| {
            Ok(Machine {
                button_a_x: cap[1].parse()?,
                button_a_y: cap[2].parse()?,
                button_b_x: cap[3].parse()?,
                button_b_y: cap[4].parse()?,
                prize_x: cap[5].parse::<i64>()? + 10_000_000_000_000,
                prize_y: cap[6].parse::<i64>()? + 10_000_000_000_000,
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let result: i64 = machines.iter().filter_map(|m| m.min_prize_cost(3, 1)).sum();
    println!("result: {result}");

    Ok(())
}
