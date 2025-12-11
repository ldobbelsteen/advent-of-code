#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/6-input.txt")?;
    let lines = file.lines().collect::<Vec<_>>();
    assert!(lines.len() >= 2);

    let operator_line = lines[lines.len() - 1];
    let operators = operator_line.split_whitespace().collect::<Vec<_>>();

    let operand_lines = &lines[..lines.len() - 1];
    let mut operands: Vec<Vec<u64>> = (0..operators.len()).map(|_| Vec::new()).collect();
    for line in operand_lines {
        let nums = line
            .split_whitespace()
            .map(|s| Ok(s.parse::<u64>()?))
            .collect::<Result<Vec<_>>>()?;
        if nums.len() != operators.len() {
            return Err(anyhow!(
                "expected {} operands per line, got {}",
                operators.len(),
                nums.len()
            ));
        }
        for (i, &num) in nums.iter().enumerate() {
            operands[i].push(num);
        }
    }

    let mut result = 0;
    for i in 0..operators.len() {
        let op = operators[i];
        let nums = &operands[i];
        let op_result: u64 = match op {
            "+" => Ok(nums.iter().sum()),
            "*" => Ok(nums.iter().product()),
            _ => Err(anyhow!("invalid operator: {op}")),
        }?;
        result += op_result;
    }

    println!("{result}");

    Ok(())
}
