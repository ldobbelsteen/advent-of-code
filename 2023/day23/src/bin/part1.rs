#![warn(clippy::pedantic)]

use std::cmp::max;
use std::fs;

#[derive(Clone, Debug, PartialEq)]
struct Coord {
    x: usize,
    y: usize,
}

// Find maximum path length possible from this position given the already trodden path.
// Leaves the path in the same state when returning as it was when the function was called.
fn max_path_length(
    grid: &Vec<Vec<char>>,
    path: &mut Vec<Coord>,
    current: &Coord,
    end: &Coord,
) -> usize {
    let mut result = 0;

    // Go upwards
    if current.y > 0 {
        let next = Coord {
            x: current.x,
            y: current.y - 1,
        };
        if (grid[next.y][next.x] == '.' || grid[next.y][next.x] == '^') && !path.contains(&next) {
            path.push(next.clone());
            let len = max_path_length(grid, path, &next, end);
            if len > 0 || next == *end {
                result = max(result, 1 + len);
            }
            path.pop();
        }
    }

    // Go downwards
    if current.y < grid.len() - 1 {
        let next = Coord {
            x: current.x,
            y: current.y + 1,
        };
        if (grid[next.y][next.x] == '.' || grid[next.y][next.x] == 'v') && !path.contains(&next) {
            path.push(next.clone());
            let len = max_path_length(grid, path, &next, end);
            if len > 0 || next == *end {
                result = max(result, 1 + len);
            }
            path.pop();
        }
    }

    // Go to the left
    if current.x > 0 {
        let next = Coord {
            x: current.x - 1,
            y: current.y,
        };
        if (grid[next.y][next.x] == '.' || grid[next.y][next.x] == '<') && !path.contains(&next) {
            path.push(next.clone());
            let len = max_path_length(grid, path, &next, end);
            if len > 0 || next == *end {
                result = max(result, 1 + len);
            }
            path.pop();
        }
    }

    // Go to the right
    if current.x < grid[0].len() - 1 {
        let next = Coord {
            x: current.x + 1,
            y: current.y,
        };
        if (grid[next.y][next.x] == '.' || grid[next.y][next.x] == '>') && !path.contains(&next) {
            path.push(next.clone());
            let len = max_path_length(grid, path, &next, end);
            if len > 0 || next == *end {
                result = max(result, 1 + len);
            }
            path.pop();
        }
    }

    result
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<char>> = file.lines().map(|line| line.chars().collect()).collect();
    let start = Coord {
        x: grid[0].iter().position(|&c| c == '.').unwrap(),
        y: 0,
    };
    let end = Coord {
        x: grid[grid.len() - 1].iter().position(|&c| c == '.').unwrap(),
        y: grid.len() - 1,
    };

    let result = max_path_length(&grid, &mut Vec::from([start.clone()]), &start, &end);
    println!("{result}");
}
