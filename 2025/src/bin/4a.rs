#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};

fn surrounding_count(grid: &[Vec<bool>], i: usize, j: usize) -> Result<usize> {
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut count = 0;
    for (di, dj) in directions {
        let ni = (isize::try_from(i)?) + di;
        let nj = (isize::try_from(j)?) + dj;

        if ni < 0 || nj < 0 {
            continue;
        }

        if let Some(row) = grid.get(usize::try_from(ni)?) {
            if let Some(&cell) = row.get(usize::try_from(nj)?) {
                if cell {
                    count += 1;
                }
            }
        }
    }

    Ok(count)
}

fn is_reachable(grid: &[Vec<bool>], i: usize, j: usize) -> Result<bool> {
    let count = surrounding_count(grid, i, j)?;
    Ok(count < 4)
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/4-input.txt")?;

    let grid = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    if c == '@' {
                        Ok(true)
                    } else if c == '.' {
                        Ok(false)
                    } else {
                        Err(anyhow!("invalid char: {c}"))
                    }
                })
                .collect::<Result<Vec<bool>>>()
        })
        .collect::<Result<Vec<Vec<bool>>>>()?;

    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] && is_reachable(&grid, i, j)? {
                result += 1;
            }
        }
    }
    println!("{result}");

    Ok(())
}
