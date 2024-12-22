#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn clockwise(self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    fn counter_clockwise(self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    fn move_in_direction(&self, d: Direction) -> Self {
        match d {
            Direction::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    coord: Coordinate,
    direction: Direction,
}

impl Position {
    fn move_forward(&self) -> Self {
        Self {
            coord: self.coord.move_in_direction(self.direction),
            direction: self.direction,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct State {
    position: Position,
    distance: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Grid {
    start: Position,
    end: Coordinate,
    is_wall: Vec<Vec<bool>>,
}

impl Grid {
    fn from_str(s: &str) -> Result<Self> {
        let mut start = None;
        let mut end = None;
        let is_wall = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Ok(true),
                        '.' => Ok(false),
                        'S' => {
                            if start.is_some() {
                                Err(anyhow!("multiple start positions"))
                            } else {
                                start = Some(Coordinate { x, y });
                                Ok(false)
                            }
                        }
                        'E' => {
                            if end.is_some() {
                                Err(anyhow!("multiple end positions"))
                            } else {
                                end = Some(Coordinate { x, y });
                                Ok(false)
                            }
                        }
                        _ => Err(anyhow!("invalid input character: {}", c)),
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<_>>>()?;

        let end = end.ok_or_else(|| anyhow!("no end position"))?;
        let start = Position {
            coord: start.ok_or_else(|| anyhow!("no start position"))?,
            direction: Direction::East,
        };

        Ok(Self {
            start,
            end,
            is_wall,
        })
    }

    fn compute_shortest_path_len(&self) -> Option<u32> {
        assert!(!self.is_wall[self.start.coord.y][self.start.coord.x]);
        assert!(!self.is_wall[self.end.y][self.end.x]);

        let mut distances: HashMap<Position, u32> = HashMap::from([(self.start, 0)]);
        let mut queue = BinaryHeap::from([State {
            position: self.start,
            distance: 0,
        }]);

        while let Some(State { position, distance }) = queue.pop() {
            if position.coord == self.end {
                return Some(distance);
            }

            let next_states = [
                State {
                    position: position.move_forward(),
                    distance: distance + 1,
                },
                State {
                    position: Position {
                        coord: position.coord,
                        direction: position.direction.clockwise(),
                    },
                    distance: distance + 1_000,
                },
                State {
                    position: Position {
                        coord: position.coord,
                        direction: position.direction.counter_clockwise(),
                    },
                    distance: distance + 1_000,
                },
            ];

            for state in &next_states {
                if self.is_wall[state.position.coord.y][state.position.coord.x] {
                    continue;
                }

                let current = distances.get(&state.position).copied();
                if current.map_or(true, |d| state.distance < d) {
                    distances.insert(state.position, state.distance);
                    queue.push(*state);
                }
            }
        }

        None
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/16-input.txt")?;
    let grid = Grid::from_str(&file)?;

    let result = grid.compute_shortest_path_len();
    println!("result: {result:?}");

    Ok(())
}
