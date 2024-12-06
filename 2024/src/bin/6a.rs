#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

struct Walk {
    grid_width: usize,
    grid_height: usize,
    is_obstruction: Vec<Vec<bool>>,
    visited: HashSet<(usize, usize)>,
    location: (usize, usize),
    direction: Direction,
}

impl Walk {
    fn from_str(s: &str) -> Result<Self> {
        let mut start: Option<((usize, usize), Direction)> = None;
        let mut is_obstruction = Vec::new();

        for (y, line) in s.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => row.push(false),
                    '#' => row.push(true),
                    '^' => {
                        if start.is_some() {
                            return Err(anyhow!("multiple starting points found"));
                        }
                        start = Some(((x, y), Direction::Up));
                        row.push(false);
                    }
                    'v' => {
                        if start.is_some() {
                            return Err(anyhow!("multiple starting points found"));
                        }
                        start = Some(((x, y), Direction::Down));
                        row.push(false);
                    }
                    '<' => {
                        if start.is_some() {
                            return Err(anyhow!("multiple starting points found"));
                        }
                        start = Some(((x, y), Direction::Left));
                        row.push(false);
                    }
                    '>' => {
                        if start.is_some() {
                            return Err(anyhow!("multiple starting points found"));
                        }
                        start = Some(((x, y), Direction::Right));
                        row.push(false);
                    }
                    _ => return Err(anyhow!("invalid character in grid")),
                }
            }
            is_obstruction.push(row);
        }

        let grid_height = is_obstruction.len();
        let grid_width = is_obstruction[0].len();

        if let Some((start, direction)) = start {
            Ok(Self {
                grid_width,
                grid_height,
                is_obstruction,
                visited: HashSet::from([start]),
                location: start,
                direction,
            })
        } else {
            Err(anyhow!("no starting point found"))
        }
    }

    // Make the next move. Returns whether we walked out of bounds.
    fn next_move(&mut self) -> bool {
        let location_ahead = match self.direction {
            Direction::Left => {
                if self.location.0 == 0 {
                    return true;
                }
                (self.location.0 - 1, self.location.1)
            }
            Direction::Right => {
                if self.location.0 == self.grid_height - 1 {
                    return true;
                }
                (self.location.0 + 1, self.location.1)
            }
            Direction::Up => {
                if self.location.1 == 0 {
                    return true;
                }
                (self.location.0, self.location.1 - 1)
            }
            Direction::Down => {
                if self.location.1 == self.grid_width - 1 {
                    return true;
                }
                (self.location.0, self.location.1 + 1)
            }
        };

        if self.is_obstruction[location_ahead.1][location_ahead.0] {
            self.direction = self.direction.turn_right();
        } else {
            self.location = location_ahead;
            self.visited.insert(location_ahead);
        }

        false
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/6-input.txt")?;
    let mut walk = Walk::from_str(&file)?;

    loop {
        if walk.next_move() {
            break;
        }
    }

    let result = walk.visited.len();
    println!("result: {result}");

    Ok(())
}
