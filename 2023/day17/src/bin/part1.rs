#![warn(clippy::pedantic)]

use anyhow::{anyhow, Result};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    fs,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn variants() -> impl Iterator<Item = Direction> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .copied()
    }

    fn reverse(self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    fn take(self, coord: &Coord, x_max: usize, y_max: usize) -> Option<Coord> {
        match self {
            Direction::Down => {
                if coord.y < y_max {
                    return Some(Coord {
                        x: coord.x,
                        y: coord.y + 1,
                    });
                }
            }
            Direction::Up => {
                if coord.y > 0 {
                    return Some(Coord {
                        x: coord.x,
                        y: coord.y - 1,
                    });
                }
            }
            Direction::Left => {
                if coord.x > 0 {
                    return Some(Coord {
                        x: coord.x - 1,
                        y: coord.y,
                    });
                }
            }
            Direction::Right => {
                if coord.x < x_max {
                    return Some(Coord {
                        x: coord.x + 1,
                        y: coord.y,
                    });
                }
            }
        }
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Chain {
    direction: Direction,
    length: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Vertex {
    coord: Coord,
    chain: Chain,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct State {
    vertex: Vertex,
    distance: u32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Grid {
    inner: Vec<Vec<u32>>,
    max_chain_length: u32,
    x_max: usize,
    y_max: usize,
}

impl Grid {
    fn new(inner: Vec<Vec<u32>>, max_chain_length: u32) -> Self {
        let x_max = inner[0].len() - 1;
        let y_max = inner.len() - 1;
        Self {
            inner,
            max_chain_length,
            x_max,
            y_max,
        }
    }

    fn next_states(&self, state: State) -> impl Iterator<Item = State> + '_ {
        Direction::variants()
            .filter(move |&d| {
                (d != state.vertex.chain.direction.reverse() || state.vertex.chain.length == 0)
                    && (d != state.vertex.chain.direction
                        || state.vertex.chain.length < self.max_chain_length)
            })
            .filter_map(move |d| {
                d.take(&state.vertex.coord, self.x_max, self.y_max)
                    .map(|c| State {
                        distance: state.distance + self.inner[c.y][c.x],
                        vertex: Vertex {
                            coord: c,
                            chain: if d == state.vertex.chain.direction {
                                Chain {
                                    direction: d,
                                    length: state.vertex.chain.length + 1,
                                }
                            } else {
                                Chain {
                                    direction: d,
                                    length: 1,
                                }
                            },
                        },
                    })
            })
    }

    fn dijkstra(&self, source: &Coord, target: &Coord) -> u32 {
        let mut distance: HashMap<Vertex, u32> = HashMap::new();
        let mut queue = BinaryHeap::new();

        for direction in Direction::variants() {
            distance.insert(
                Vertex {
                    coord: source.clone(),
                    chain: Chain {
                        direction,
                        length: 0,
                    },
                },
                0,
            );
        }

        for (starting_vertex, starting_dist) in &distance {
            queue.push(State {
                vertex: starting_vertex.clone(),
                distance: *starting_dist,
            });
        }

        while let Some(state) = queue.pop() {
            if state.vertex.coord == *target {
                return state.distance;
            }
            if state.distance > distance[&state.vertex] {
                continue;
            }
            for next in self.next_states(state) {
                if !distance.contains_key(&next.vertex) || next.distance < distance[&next.vertex] {
                    queue.push(next.clone());
                    distance.insert(next.vertex, next.distance);
                }
            }
        }

        u32::MAX
    }
}

fn main() -> Result<()> {
    let file = fs::read_to_string("input.txt")?;
    let grid = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).ok_or(anyhow!("invalid digit")))
                .collect::<Result<Vec<u32>>>()
        })
        .collect::<Result<Vec<Vec<u32>>>>()?;

    let grid = Grid::new(grid, 3);
    let result = grid.dijkstra(
        &Coord { x: 0, y: 0 },
        &Coord {
            x: grid.x_max,
            y: grid.y_max,
        },
    );

    println!("{result}");
    Ok(())
}
