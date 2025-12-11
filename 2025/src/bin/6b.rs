#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/6-input.txt")?;
    let lines = file.lines().collect::<Vec<_>>();
    assert!(lines.len() >= 2);

    let operator_line = lines[lines.len() - 1];
    let operand_lines = &lines[..lines.len() - 1];
    let mut column_widths: Vec<usize> = Vec::from([1]);
    for char in operator_line.chars().skip(1) {
        let last = column_widths.last_mut().unwrap();
        if char.is_whitespace() {
            *last += 1;
        } else {
            *last -= 1;
            column_widths.push(1);
        }
    }

    let operators = operator_line.split_whitespace().collect::<Vec<_>>();
    let column_count = operators.len();
    let mut operand_chars = (0..column_count)
        .map(|i| {
            (0..column_widths[i])
                .map(|_| Vec::new())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>();

    for line in operand_lines {
        let mut chars = line.chars();
        for (column_idx, column_width) in column_widths.iter().enumerate() {
            for (subcolumn_idx, c) in chars.by_ref().take(*column_width).enumerate() {
                if !c.is_whitespace() {
                    operand_chars[column_idx][subcolumn_idx].push(c);
                }
            }
            let _ = chars.next();
        }
    }

    let operands = operand_chars
        .into_iter()
        .map(|subcolumns| {
            subcolumns
                .into_iter()
                .filter(|chars| !chars.is_empty())
                .map(|chars| chars.into_iter().collect::<String>())
                .map(|s| Ok(s.parse::<u64>()?))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<Vec<u64>>>>()?;

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
