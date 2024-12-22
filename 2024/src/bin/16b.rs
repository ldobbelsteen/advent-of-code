#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::collections::{BinaryHeap, HashMap, HashSet};

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

    fn compute_all_shortest_path_tiles(&self) -> Option<HashSet<Coordinate>> {
        assert!(!self.is_wall[self.start.coord.y][self.start.coord.x]);
        assert!(!self.is_wall[self.end.y][self.end.x]);

        let mut distances: HashMap<Position, u32> = HashMap::from([(self.start, 0)]);
        let mut predecessors: HashMap<Position, HashSet<Position>> = HashMap::new();
        let mut queue = BinaryHeap::from([State {
            position: self.start,
            distance: 0,
        }]);

        while let Some(state) = queue.pop() {
            if state.position.coord == self.end {
                let mut result = HashSet::from([state.position.coord]);
                let mut boundary = vec![state.position];
                while !boundary.is_empty() {
                    let mut next_boundary = Vec::new();
                    for pos in boundary {
                        for pred in predecessors.get(&pos).into_iter().flatten() {
                            result.insert(pred.coord);
                            next_boundary.push(*pred);
                        }
                    }
                    boundary = next_boundary;
                }
                return Some(result);
            }

            let next_states = [
                State {
                    position: state.position.move_forward(),
                    distance: state.distance + 1,
                },
                State {
                    position: Position {
                        coord: state.position.coord,
                        direction: state.position.direction.clockwise(),
                    },
                    distance: state.distance + 1_000,
                },
                State {
                    position: Position {
                        coord: state.position.coord,
                        direction: state.position.direction.counter_clockwise(),
                    },
                    distance: state.distance + 1_000,
                },
            ];

            for next_state in &next_states {
                if self.is_wall[next_state.position.coord.y][next_state.position.coord.x] {
                    continue;
                }

                let current = distances.get(&next_state.position).copied();

                if current.map_or(true, |d| next_state.distance <= d) {
                    distances.insert(next_state.position, next_state.distance);
                    queue.push(*next_state);

                    if next_state.distance < current.unwrap_or(u32::MAX) {
                        predecessors.insert(next_state.position, HashSet::from([state.position]));
                    } else {
                        predecessors
                            .get_mut(&next_state.position)
                            .unwrap()
                            .insert(state.position);
                    }
                }
            }
        }

        None
    }
}

fn main() -> Result<()> {
    let file = std::fs::read_to_string("inputs/16-input.txt")?;
    let grid = Grid::from_str(&file)?;

    let result = grid
        .compute_all_shortest_path_tiles()
        .ok_or(anyhow!("no path found"))?
        .len();
    println!("result: {result:?}");

    Ok(())
}
