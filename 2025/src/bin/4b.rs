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

fn remove_reachable(grid: &mut [Vec<bool>]) -> Result<usize> {
    let mut to_remove = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] && is_reachable(grid, i, j)? {
                to_remove.push((i, j));
            }
        }
    }

    let count = to_remove.len();
    for (i, j) in to_remove {
        grid[i][j] = false;
    }

    Ok(count)
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/4-input.txt")?;

    let mut grid = file
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
    loop {
        let removed = remove_reachable(&mut grid)?;
        if removed == 0 {
            break;
        }
        result += removed;
    }
    println!("{result}");

    Ok(())
}
