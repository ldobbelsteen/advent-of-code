#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_right(&mut self) {
        match self {
            Self::Up => *self = Self::Right,
            Self::Right => *self = Self::Down,
            Self::Down => *self = Self::Left,
            Self::Left => *self = Self::Up,
        };
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct DirectedPosition {
    p: Position,
    d: Direction,
}

impl DirectedPosition {
    fn turn_right(&mut self) {
        self.d.turn_right();
    }
}

#[derive(Debug, Clone)]
struct Grid {
    width: usize,
    height: usize,
    is_obstruction: Vec<Vec<bool>>,
}

impl Grid {
    fn from_str(s: &str) -> Result<(Self, DirectedPosition)> {
        let mut start: Option<DirectedPosition> = None;
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
                        start = Some(DirectedPosition {
                            p: Position { x, y },
                            d: Direction::Up,
                        });
                        row.push(false);
                    }
                    'v' => {
                        if start.is_some() {
                            return Err(anyhow!("multiple starting points found"));
                        }
                        start = Some(DirectedPosition {
                            p: Position { x, y },
                            d: Direction::Down,
                        });
                        row.push(false);
                    }
                    '<' => {
                        if start.is_some() {
                            return Err(anyhow!("multiple starting points found"));
                        }
                        start = Some(DirectedPosition {
                            p: Position { x, y },
                            d: Direction::Left,
                        });
                        row.push(false);
                    }
                    '>' => {
                        if start.is_some() {
                            return Err(anyhow!("multiple starting points found"));
                        }
                        start = Some(DirectedPosition {
                            p: Position { x, y },
                            d: Direction::Right,
                        });
                        row.push(false);
                    }
                    _ => return Err(anyhow!("invalid character in grid")),
                }
            }
            is_obstruction.push(row);
        }

        let height = is_obstruction.len();
        let width = if is_obstruction.is_empty() {
            0
        } else {
            is_obstruction[0].len()
        };

        Ok((
            Self {
                width,
                height,
                is_obstruction,
            },
            start.ok_or_else(|| anyhow!("no starting point found"))?,
        ))
    }

    fn with_extra_obstruction(&self, x: usize, y: usize) -> Self {
        let mut is_obstruction = self.is_obstruction.clone();
        is_obstruction[y][x] = true;
        Self {
            width: self.width,
            height: self.height,
            is_obstruction,
        }
    }
}

#[derive(Debug, Clone)]
struct State {
    grid: Grid,
    current: DirectedPosition,
}

impl State {
    /// Compute the position ahead of the current position. If the position ahead
    /// is out of bounds, return None.
    fn position_ahead(&self) -> Option<DirectedPosition> {
        let mut result = self.current;
        match self.current.d {
            Direction::Left => {
                if self.current.p.x == 0 {
                    return None;
                }
                result.p.x -= 1;
            }
            Direction::Right => {
                if self.current.p.x == self.grid.height - 1 {
                    return None;
                }
                result.p.x += 1;
            }
            Direction::Up => {
                if self.current.p.y == 0 {
                    return None;
                }
                result.p.y -= 1;
            }
            Direction::Down => {
                if self.current.p.y == self.grid.width - 1 {
                    return None;
                }
                result.p.y += 1;
            }
        };
        Some(result)
    }

    /// Check if walking from the current position results in a loop. If it does,
    /// return true. If we walk out of bounds, return false.
    fn results_in_loop(mut self) -> bool {
        let mut visited = HashSet::from([self.current]);
        loop {
            if let Some(ahead) = self.position_ahead() {
                if visited.contains(&ahead) {
                    return true;
                }

                if self.grid.is_obstruction[ahead.p.y][ahead.p.x] {
                    self.current.turn_right();
                } else {
                    self.current = ahead;
                }

                visited.insert(self.current);
            } else {
                return false;
            }
        }
    }

    fn count_extra_obstruction_loops(mut self, banned_positions: &HashSet<Position>) -> usize {
        assert!(!self.clone().results_in_loop());

        let mut loop_obstructions: HashSet<Position> = HashSet::new();
        let mut visited_directed = HashSet::from([self.current]);
        let mut visited_undirected = HashSet::from([self.current.p]);

        while let Some(ahead) = self.position_ahead() {
            if self.grid.is_obstruction[ahead.p.y][ahead.p.x] {
                self.current.turn_right();
            } else {
                // Try adding an obstruction at the position ahead and see if it results in a loop.
                // This is only valid if the position is not banned or previously visited. If a
                // previously visited position is obstructed, the current position is not necessarily
                // reachable, which would result in a false positive.
                if !banned_positions.contains(&ahead.p) && !visited_undirected.contains(&ahead.p) {
                    let with_extra = State {
                        grid: self.grid.with_extra_obstruction(ahead.p.x, ahead.p.y),
                        current: self.current,
                    };

                    if with_extra.results_in_loop() {
                        loop_obstructions.insert(ahead.p);
                    }
                }

                self.current = ahead;
            }

            visited_directed.insert(self.current);
            visited_undirected.insert(self.current.p);
        }

        loop_obstructions.len()
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/6-input.txt")?;
    let (grid, start) = Grid::from_str(&file)?;

    let state = State {
        grid,
        current: start,
    };

    let result = state.count_extra_obstruction_loops(&HashSet::from([start.p]));
    println!("result: {result}");

    Ok(())
}
